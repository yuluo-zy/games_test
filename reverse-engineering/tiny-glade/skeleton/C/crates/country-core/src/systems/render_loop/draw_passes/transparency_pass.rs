// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 52e6660c750ed57f8647c30e199ff7cb112c72ad547789506e43295c35345b3b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: transparency_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 556, 1655
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":47924,"rva":11447952,"va":5380157072}
// 编译实例 RVAs: 0xaeae90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3315532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157072,"raw_hex":"e84c973200","size":5,"soft_fail":false,"target_va":5383472768,"va":5380157231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141762f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13074361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157072,"raw_hex":"e8b97fc700","size":5,"soft_fail":false,"target_va":5393231760,"va":5380157394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3390956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157072,"raw_hex":"e8ecbd3300","size":5,"soft_fail":false,"target_va":5383548144,"va":5380157183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":48156,"parent_record_offset":47924,"ancestor_record_offset":47924,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":48100,"parent_record_offset":47924,"ancestor_record_offset":47924,"inlinee":13021,"source_line":791,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":48124,"parent_record_offset":48100,"ancestor_record_offset":47924,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14063
// 首条位置: {"module":1655,"inlinee":82019}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1655,"record_offset":123692,"parent_record_offset":123560,"ancestor_record_offset":123560,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":125564,"parent_record_offset":123560,"ancestor_record_offset":123560,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":125708,"parent_record_offset":123560,"ancestor_record_offset":123560,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":125364,"parent_record_offset":123560,"ancestor_record_offset":123560,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1655,"record_offset":125472,"parent_record_offset":123560,"ancestor_record_offset":123560,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1655,"record_offset":127408,"parent_record_offset":123560,"ancestor_record_offset":123560,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":1655,"record_offset":128404,"parent_record_offset":123560,"ancestor_record_offset":123560,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::ShaderPipelineCommon::debug_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":1655,"record_offset":125536,"parent_record_offset":123560,"ancestor_record_offset":123560,"inlinee":10388,"source_line":150,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::rg
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":125336,"parent_record_offset":123560,"ancestor_record_offset":123560,"inlinee":34679,"source_line":1266,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":125012,"parent_record_offset":124988,"ancestor_record_offset":123560,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":125060,"parent_record_offset":125036,"ancestor_record_offset":123560,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":125036,"parent_record_offset":125012,"ancestor_record_offset":123560,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":124988,"parent_record_offset":124964,"ancestor_record_offset":123560,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":124964,"parent_record_offset":124924,"ancestor_record_offset":123560,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":124924,"parent_record_offset":124900,"ancestor_record_offset":123560,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":1655,"record_offset":123720,"parent_record_offset":123692,"ancestor_record_offset":123560,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1655,"record_offset":125284,"parent_record_offset":125260,"ancestor_record_offset":123560,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1655,"record_offset":125260,"parent_record_offset":125236,"ancestor_record_offset":123560,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1655,"record_offset":125208,"parent_record_offset":125176,"ancestor_record_offset":123560,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":125152,"parent_record_offset":124644,"ancestor_record_offset":123560,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":125176,"parent_record_offset":125152,"ancestor_record_offset":123560,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":124672,"parent_record_offset":124644,"ancestor_record_offset":123560,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":125236,"parent_record_offset":125176,"ancestor_record_offset":123560,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1655,"record_offset":124720,"parent_record_offset":124696,"ancestor_record_offset":123560,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1655,"record_offset":124696,"parent_record_offset":124672,"ancestor_record_offset":123560,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1655,"record_offset":124900,"parent_record_offset":124876,"ancestor_record_offset":123560,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1655,"record_offset":124644,"parent_record_offset":123692,"ancestor_record_offset":123560,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":1655,"record_offset":123744,"parent_record_offset":123720,"ancestor_record_offset":123560,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1655,"record_offset":124392,"parent_record_offset":124368,"ancestor_record_offset":123560,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":124420,"parent_record_offset":124392,"ancestor_record_offset":123560,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":125612,"parent_record_offset":125588,"ancestor_record_offset":123560,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":123960,"parent_record_offset":123936,"ancestor_record_offset":123560,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":124044,"parent_record_offset":124020,"ancestor_record_offset":123560,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":128228,"parent_record_offset":127564,"ancestor_record_offset":123560,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":124192,"parent_record_offset":124164,"ancestor_record_offset":123560,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":124164,"parent_record_offset":124136,"ancestor_record_offset":123560,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1655,"record_offset":124812,"parent_record_offset":124748,"ancestor_record_offset":123560,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":124836,"parent_record_offset":124812,"ancestor_record_offset":123560,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":125732,"parent_record_offset":125708,"ancestor_record_offset":123560,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":124876,"parent_record_offset":124696,"ancestor_record_offset":123560,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":123984,"parent_record_offset":123960,"ancestor_record_offset":123560,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1655,"record_offset":124308,"parent_record_offset":124284,"ancestor_record_offset":123560,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1655,"record_offset":125500,"parent_record_offset":125472,"ancestor_record_offset":123560,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1655,"record_offset":125636,"parent_record_offset":125612,"ancestor_record_offset":123560,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1655,"record_offset":125664,"parent_record_offset":125636,"ancestor_record_offset":123560,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1655,"record_offset":124748,"parent_record_offset":124720,"ancestor_record_offset":123560,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1655,"record_offset":124784,"parent_record_offset":124748,"ancestor_record_offset":123560,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1655,"record_offset":125104,"parent_record_offset":124924,"ancestor_record_offset":123560,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1655,"record_offset":125588,"parent_record_offset":125564,"ancestor_record_offset":123560,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1655,"record_offset":128180,"parent_record_offset":128156,"ancestor_record_offset":123560,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1655,"record_offset":128156,"parent_record_offset":128132,"ancestor_record_offset":123560,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":1655,"record_offset":124368,"parent_record_offset":124344,"ancestor_record_offset":123560,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":124104,"parent_record_offset":124076,"ancestor_record_offset":123560,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":124136,"parent_record_offset":124104,"ancestor_record_offset":123560,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":124076,"parent_record_offset":123856,"ancestor_record_offset":123560,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":123936,"parent_record_offset":123856,"ancestor_record_offset":123560,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":124496,"parent_record_offset":123856,"ancestor_record_offset":123560,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":124020,"parent_record_offset":123856,"ancestor_record_offset":123560,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/tag.rs
//     evidence={"module":1655,"record_offset":127620,"parent_record_offset":127564,"ancestor_record_offset":123560,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1655,"record_offset":128108,"parent_record_offset":128084,"ancestor_record_offset":123560,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1655,"record_offset":123780,"parent_record_offset":123744,"ancestor_record_offset":123560,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1655,"record_offset":124344,"parent_record_offset":124236,"ancestor_record_offset":123560,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1655,"record_offset":127488,"parent_record_offset":127456,"ancestor_record_offset":123560,"inlinee":4403,"source_line":240,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":124284,"parent_record_offset":124260,"ancestor_record_offset":123560,"inlinee":4324,"source_line":261,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":124588,"parent_record_offset":123856,"ancestor_record_offset":123560,"inlinee":4339,"source_line":81,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":124260,"parent_record_offset":124236,"ancestor_record_offset":123560,"inlinee":4322,"source_line":734,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":123832,"parent_record_offset":123808,"ancestor_record_offset":123560,"inlinee":4302,"source_line":1156,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":123808,"parent_record_offset":123780,"ancestor_record_offset":123560,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":123856,"parent_record_offset":123832,"ancestor_record_offset":123560,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":123908,"parent_record_offset":123856,"ancestor_record_offset":123560,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":124236,"parent_record_offset":123856,"ancestor_record_offset":123560,"inlinee":4321,"source_line":1166,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":1655,"record_offset":124468,"parent_record_offset":123856,"ancestor_record_offset":123560,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":128600,"parent_record_offset":128404,"ancestor_record_offset":123560,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":128440,"parent_record_offset":128404,"ancestor_record_offset":123560,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1655,"record_offset":128524,"parent_record_offset":128496,"ancestor_record_offset":123560,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":128628,"parent_record_offset":128600,"ancestor_record_offset":123560,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":128468,"parent_record_offset":128440,"ancestor_record_offset":123560,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1655,"record_offset":128552,"parent_record_offset":128524,"ancestor_record_offset":123560,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":1655,"record_offset":128496,"parent_record_offset":128468,"ancestor_record_offset":123560,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":127432,"parent_record_offset":127408,"ancestor_record_offset":123560,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":43716,"rva":11446224,"va":5380155344}
// 编译实例 RVAs: 0xaea7d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141538b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10805602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e862e1a400","size":5,"soft_fail":false,"target_va":5390961552,"va":5380155945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141546ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10862132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e834bea500","size":5,"soft_fail":false,"target_va":5391018720,"va":5380156583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14175f650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13060527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e8af49c700","size":5,"soft_fail":false,"target_va":5393217104,"va":5380156572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13132108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e84c61c800","size":5,"soft_fail":false,"target_va":5393288768,"va":5380156655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177da70
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13184325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e8452dc900","size":5,"soft_fail":false,"target_va":5393341040,"va":5380156710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141780430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13194698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e8ca55c900","size":5,"soft_fail":false,"target_va":5393351728,"va":5380157025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36465309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e89d6a2c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380156286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::clear_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3385585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e8f1a83300","size":5,"soft_fail":false,"target_va":5383541136,"va":5380155546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3392634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e87ac43300","size":5,"soft_fail":false,"target_va":5383548144,"va":5380155505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3379926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e8d6923300","size":5,"soft_fail":false,"target_va":5383536672,"va":5380156741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3381724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e8dc993300","size":5,"soft_fail":false,"target_va":5383538768,"va":5380157039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3382459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e8bb9c3300","size":5,"soft_fail":false,"target_va":5383539152,"va":5380156688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::BufferDesc::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3368721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380155344,"raw_hex":"e811673300","size":5,"soft_fail":false,"target_va":5383524160,"va":5380155434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":44008,"parent_record_offset":43716,"ancestor_record_offset":43716,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":47684,"parent_record_offset":43716,"ancestor_record_offset":43716,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":45652,"parent_record_offset":43716,"ancestor_record_offset":43716,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::apply_volumetrics::Volumetrics::reflection_fog
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs
//     evidence={"module":556,"record_offset":47808,"parent_record_offset":43716,"ancestor_record_offset":43716,"inlinee":34749,"source_line":35,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":556,"record_offset":47712,"parent_record_offset":43716,"ancestor_record_offset":43716,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":43916,"parent_record_offset":43716,"ancestor_record_offset":43716,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":556,"record_offset":47780,"parent_record_offset":43716,"ancestor_record_offset":43716,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::pixel_count
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":43948,"parent_record_offset":43716,"ancestor_record_offset":43716,"inlinee":34747,"source_line":415,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":45328,"parent_record_offset":45304,"ancestor_record_offset":43716,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":45376,"parent_record_offset":45352,"ancestor_record_offset":43716,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":45352,"parent_record_offset":45328,"ancestor_record_offset":43716,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":45304,"parent_record_offset":45280,"ancestor_record_offset":43716,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":45280,"parent_record_offset":45240,"ancestor_record_offset":43716,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":45240,"parent_record_offset":45216,"ancestor_record_offset":43716,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":556,"record_offset":44036,"parent_record_offset":44008,"ancestor_record_offset":43716,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":45600,"parent_record_offset":45576,"ancestor_record_offset":43716,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":45576,"parent_record_offset":45552,"ancestor_record_offset":43716,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":45524,"parent_record_offset":45492,"ancestor_record_offset":43716,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":45468,"parent_record_offset":44960,"ancestor_record_offset":43716,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":45492,"parent_record_offset":45468,"ancestor_record_offset":43716,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":44988,"parent_record_offset":44960,"ancestor_record_offset":43716,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":45552,"parent_record_offset":45492,"ancestor_record_offset":43716,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":45036,"parent_record_offset":45012,"ancestor_record_offset":43716,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":45012,"parent_record_offset":44988,"ancestor_record_offset":43716,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":45216,"parent_record_offset":45192,"ancestor_record_offset":43716,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":45796,"parent_record_offset":45768,"ancestor_record_offset":43716,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":44960,"parent_record_offset":44008,"ancestor_record_offset":43716,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":556,"record_offset":44060,"parent_record_offset":44036,"ancestor_record_offset":43716,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":556,"record_offset":44708,"parent_record_offset":44684,"ancestor_record_offset":43716,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":556,"record_offset":44736,"parent_record_offset":44708,"ancestor_record_offset":43716,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":44276,"parent_record_offset":44252,"ancestor_record_offset":43716,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":44360,"parent_record_offset":44336,"ancestor_record_offset":43716,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":44508,"parent_record_offset":44480,"ancestor_record_offset":43716,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":44480,"parent_record_offset":44452,"ancestor_record_offset":43716,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":45128,"parent_record_offset":45064,"ancestor_record_offset":43716,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":45152,"parent_record_offset":45128,"ancestor_record_offset":43716,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":45192,"parent_record_offset":45012,"ancestor_record_offset":43716,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":47440,"parent_record_offset":45768,"ancestor_record_offset":43716,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":44300,"parent_record_offset":44276,"ancestor_record_offset":43716,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":556,"record_offset":44624,"parent_record_offset":44600,"ancestor_record_offset":43716,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":47552,"parent_record_offset":47528,"ancestor_record_offset":43716,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":45064,"parent_record_offset":45036,"ancestor_record_offset":43716,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":45100,"parent_record_offset":45064,"ancestor_record_offset":43716,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":45420,"parent_record_offset":45240,"ancestor_record_offset":43716,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":47500,"parent_record_offset":47472,"ancestor_record_offset":43716,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":47528,"parent_record_offset":47500,"ancestor_record_offset":43716,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":45688,"parent_record_offset":45652,"ancestor_record_offset":43716,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":47652,"parent_record_offset":45652,"ancestor_record_offset":43716,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::is_halloween2024
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":556,"record_offset":47472,"parent_record_offset":45716,"ancestor_record_offset":43716,"inlinee":34758,"source_line":239,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$3::is_halloween2024::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":556,"record_offset":47584,"parent_record_offset":47500,"ancestor_record_offset":43716,"inlinee":34760,"source_line":242,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::calc_deferred_rt_args::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":556,"record_offset":47748,"parent_record_offset":47712,"ancestor_record_offset":43716,"inlinee":30814,"source_line":40766,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::deferred_rt::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":556,"record_offset":47888,"parent_record_offset":47848,"ancestor_record_offset":43716,"inlinee":30655,"source_line":40864,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs
//     evidence={"module":556,"record_offset":45716,"parent_record_offset":45652,"ancestor_record_offset":43716,"inlinee":34757,"source_line":151,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":556,"record_offset":45768,"parent_record_offset":45716,"ancestor_record_offset":43716,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":556,"record_offset":44684,"parent_record_offset":44660,"ancestor_record_offset":43716,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":556,"record_offset":47620,"parent_record_offset":45716,"ancestor_record_offset":43716,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":44420,"parent_record_offset":44392,"ancestor_record_offset":43716,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":44452,"parent_record_offset":44420,"ancestor_record_offset":43716,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":44392,"parent_record_offset":44172,"ancestor_record_offset":43716,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":44252,"parent_record_offset":44172,"ancestor_record_offset":43716,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":44812,"parent_record_offset":44172,"ancestor_record_offset":43716,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":44336,"parent_record_offset":44172,"ancestor_record_offset":43716,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":44096,"parent_record_offset":44060,"ancestor_record_offset":43716,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":44660,"parent_record_offset":44552,"ancestor_record_offset":43716,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":44600,"parent_record_offset":44576,"ancestor_record_offset":43716,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":44904,"parent_record_offset":44172,"ancestor_record_offset":43716,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":44576,"parent_record_offset":44552,"ancestor_record_offset":43716,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":44148,"parent_record_offset":44124,"ancestor_record_offset":43716,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":44124,"parent_record_offset":44096,"ancestor_record_offset":43716,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":44172,"parent_record_offset":44148,"ancestor_record_offset":43716,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":44224,"parent_record_offset":44172,"ancestor_record_offset":43716,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":44552,"parent_record_offset":44172,"ancestor_record_offset":43716,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":556,"record_offset":44784,"parent_record_offset":44172,"ancestor_record_offset":43716,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x87C5
// 首条位置: {"module":556,"inlinee":34757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::session::SessionMeta::is_halloween2024
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":556,"record_offset":47472,"parent_record_offset":45716,"ancestor_record_offset":45716,"inlinee":34758,"source_line":239,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":556,"record_offset":45768,"parent_record_offset":45716,"ancestor_record_offset":45716,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":556,"record_offset":47620,"parent_record_offset":45716,"ancestor_record_offset":45716,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":47116,"parent_record_offset":47092,"ancestor_record_offset":45716,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":47164,"parent_record_offset":47140,"ancestor_record_offset":45716,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":47140,"parent_record_offset":47116,"ancestor_record_offset":45716,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":47092,"parent_record_offset":47068,"ancestor_record_offset":45716,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":47068,"parent_record_offset":47028,"ancestor_record_offset":45716,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":47028,"parent_record_offset":47004,"ancestor_record_offset":45716,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":556,"record_offset":45824,"parent_record_offset":45796,"ancestor_record_offset":45716,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":47388,"parent_record_offset":47364,"ancestor_record_offset":45716,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":47364,"parent_record_offset":47340,"ancestor_record_offset":45716,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":47312,"parent_record_offset":47280,"ancestor_record_offset":45716,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":47256,"parent_record_offset":46748,"ancestor_record_offset":45716,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":47280,"parent_record_offset":47256,"ancestor_record_offset":45716,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":46776,"parent_record_offset":46748,"ancestor_record_offset":45716,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":47340,"parent_record_offset":47280,"ancestor_record_offset":45716,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":46824,"parent_record_offset":46800,"ancestor_record_offset":45716,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":46800,"parent_record_offset":46776,"ancestor_record_offset":45716,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":47004,"parent_record_offset":46980,"ancestor_record_offset":45716,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":45796,"parent_record_offset":45768,"ancestor_record_offset":45716,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":46748,"parent_record_offset":45796,"ancestor_record_offset":45716,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":556,"record_offset":45848,"parent_record_offset":45824,"ancestor_record_offset":45716,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":556,"record_offset":46496,"parent_record_offset":46472,"ancestor_record_offset":45716,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":556,"record_offset":46524,"parent_record_offset":46496,"ancestor_record_offset":45716,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":46064,"parent_record_offset":46040,"ancestor_record_offset":45716,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":46148,"parent_record_offset":46124,"ancestor_record_offset":45716,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":46296,"parent_record_offset":46268,"ancestor_record_offset":45716,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":46268,"parent_record_offset":46240,"ancestor_record_offset":45716,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":46916,"parent_record_offset":46852,"ancestor_record_offset":45716,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":46940,"parent_record_offset":46916,"ancestor_record_offset":45716,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":46980,"parent_record_offset":46800,"ancestor_record_offset":45716,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":47440,"parent_record_offset":45768,"ancestor_record_offset":45716,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":46088,"parent_record_offset":46064,"ancestor_record_offset":45716,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":556,"record_offset":46412,"parent_record_offset":46388,"ancestor_record_offset":45716,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":47552,"parent_record_offset":47528,"ancestor_record_offset":45716,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":46852,"parent_record_offset":46824,"ancestor_record_offset":45716,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":46888,"parent_record_offset":46852,"ancestor_record_offset":45716,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":47208,"parent_record_offset":47028,"ancestor_record_offset":45716,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":47500,"parent_record_offset":47472,"ancestor_record_offset":45716,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":47528,"parent_record_offset":47500,"ancestor_record_offset":45716,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$3::is_halloween2024::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":556,"record_offset":47584,"parent_record_offset":47500,"ancestor_record_offset":45716,"inlinee":34760,"source_line":242,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":556,"record_offset":46472,"parent_record_offset":46448,"ancestor_record_offset":45716,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":46208,"parent_record_offset":46180,"ancestor_record_offset":45716,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":46240,"parent_record_offset":46208,"ancestor_record_offset":45716,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":46180,"parent_record_offset":45960,"ancestor_record_offset":45716,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":46040,"parent_record_offset":45960,"ancestor_record_offset":45716,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":46600,"parent_record_offset":45960,"ancestor_record_offset":45716,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":46124,"parent_record_offset":45960,"ancestor_record_offset":45716,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":45884,"parent_record_offset":45848,"ancestor_record_offset":45716,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":46448,"parent_record_offset":46340,"ancestor_record_offset":45716,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":46388,"parent_record_offset":46364,"ancestor_record_offset":45716,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":46692,"parent_record_offset":45960,"ancestor_record_offset":45716,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":46364,"parent_record_offset":46340,"ancestor_record_offset":45716,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":45936,"parent_record_offset":45912,"ancestor_record_offset":45716,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":45912,"parent_record_offset":45884,"ancestor_record_offset":45716,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":45960,"parent_record_offset":45936,"ancestor_record_offset":45716,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":46012,"parent_record_offset":45960,"ancestor_record_offset":45716,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":46340,"parent_record_offset":45960,"ancestor_record_offset":45716,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":556,"record_offset":46572,"parent_record_offset":45960,"ancestor_record_offset":45716,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14059
// 首条位置: {"module":1655,"inlinee":82009}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1655,"record_offset":103616,"parent_record_offset":103464,"ancestor_record_offset":103464,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":105612,"parent_record_offset":103464,"ancestor_record_offset":103464,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":105288,"parent_record_offset":103464,"ancestor_record_offset":103464,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1655,"record_offset":105396,"parent_record_offset":103464,"ancestor_record_offset":103464,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1655,"record_offset":105460,"parent_record_offset":103464,"ancestor_record_offset":103464,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1655,"record_offset":107312,"parent_record_offset":103464,"ancestor_record_offset":103464,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":1655,"record_offset":108308,"parent_record_offset":103464,"ancestor_record_offset":103464,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::ShaderPipelineCommon::debug_name
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":1655,"record_offset":105584,"parent_record_offset":103464,"ancestor_record_offset":103464,"inlinee":10388,"source_line":150,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::rg
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":105260,"parent_record_offset":103464,"ancestor_record_offset":103464,"inlinee":34679,"source_line":1266,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":104936,"parent_record_offset":104912,"ancestor_record_offset":103464,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":104984,"parent_record_offset":104960,"ancestor_record_offset":103464,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":104960,"parent_record_offset":104936,"ancestor_record_offset":103464,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":104912,"parent_record_offset":104888,"ancestor_record_offset":103464,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":104888,"parent_record_offset":104848,"ancestor_record_offset":103464,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":104848,"parent_record_offset":104824,"ancestor_record_offset":103464,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":1655,"record_offset":103644,"parent_record_offset":103616,"ancestor_record_offset":103464,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1655,"record_offset":105208,"parent_record_offset":105184,"ancestor_record_offset":103464,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1655,"record_offset":105184,"parent_record_offset":105160,"ancestor_record_offset":103464,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1655,"record_offset":105132,"parent_record_offset":105100,"ancestor_record_offset":103464,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":105076,"parent_record_offset":104568,"ancestor_record_offset":103464,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":105100,"parent_record_offset":105076,"ancestor_record_offset":103464,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":104596,"parent_record_offset":104568,"ancestor_record_offset":103464,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":105160,"parent_record_offset":105100,"ancestor_record_offset":103464,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1655,"record_offset":104644,"parent_record_offset":104620,"ancestor_record_offset":103464,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1655,"record_offset":104620,"parent_record_offset":104596,"ancestor_record_offset":103464,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1655,"record_offset":104824,"parent_record_offset":104800,"ancestor_record_offset":103464,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1655,"record_offset":104568,"parent_record_offset":103616,"ancestor_record_offset":103464,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":1655,"record_offset":103668,"parent_record_offset":103644,"ancestor_record_offset":103464,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1655,"record_offset":104316,"parent_record_offset":104292,"ancestor_record_offset":103464,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":104344,"parent_record_offset":104316,"ancestor_record_offset":103464,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":105488,"parent_record_offset":105460,"ancestor_record_offset":103464,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":103884,"parent_record_offset":103860,"ancestor_record_offset":103464,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":103968,"parent_record_offset":103944,"ancestor_record_offset":103464,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":108132,"parent_record_offset":107468,"ancestor_record_offset":103464,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":104116,"parent_record_offset":104088,"ancestor_record_offset":103464,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":104088,"parent_record_offset":104060,"ancestor_record_offset":103464,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1655,"record_offset":104736,"parent_record_offset":104672,"ancestor_record_offset":103464,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":104760,"parent_record_offset":104736,"ancestor_record_offset":103464,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":105636,"parent_record_offset":105612,"ancestor_record_offset":103464,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":104800,"parent_record_offset":104620,"ancestor_record_offset":103464,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":103908,"parent_record_offset":103884,"ancestor_record_offset":103464,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1655,"record_offset":104232,"parent_record_offset":104208,"ancestor_record_offset":103464,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1655,"record_offset":105424,"parent_record_offset":105396,"ancestor_record_offset":103464,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1655,"record_offset":105516,"parent_record_offset":105488,"ancestor_record_offset":103464,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1655,"record_offset":105544,"parent_record_offset":105516,"ancestor_record_offset":103464,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1655,"record_offset":104672,"parent_record_offset":104644,"ancestor_record_offset":103464,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1655,"record_offset":104708,"parent_record_offset":104672,"ancestor_record_offset":103464,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1655,"record_offset":105028,"parent_record_offset":104848,"ancestor_record_offset":103464,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1655,"record_offset":108084,"parent_record_offset":108060,"ancestor_record_offset":103464,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1655,"record_offset":108060,"parent_record_offset":108036,"ancestor_record_offset":103464,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":1655,"record_offset":104292,"parent_record_offset":104268,"ancestor_record_offset":103464,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":104028,"parent_record_offset":104000,"ancestor_record_offset":103464,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":104060,"parent_record_offset":104028,"ancestor_record_offset":103464,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":104000,"parent_record_offset":103780,"ancestor_record_offset":103464,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":103860,"parent_record_offset":103780,"ancestor_record_offset":103464,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":104420,"parent_record_offset":103780,"ancestor_record_offset":103464,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":103944,"parent_record_offset":103780,"ancestor_record_offset":103464,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/tag.rs
//     evidence={"module":1655,"record_offset":107524,"parent_record_offset":107468,"ancestor_record_offset":103464,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1655,"record_offset":108012,"parent_record_offset":107988,"ancestor_record_offset":103464,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1655,"record_offset":103704,"parent_record_offset":103668,"ancestor_record_offset":103464,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1655,"record_offset":104268,"parent_record_offset":104160,"ancestor_record_offset":103464,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1655,"record_offset":107392,"parent_record_offset":107360,"ancestor_record_offset":103464,"inlinee":4403,"source_line":240,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":104208,"parent_record_offset":104184,"ancestor_record_offset":103464,"inlinee":4324,"source_line":261,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":104512,"parent_record_offset":103780,"ancestor_record_offset":103464,"inlinee":4339,"source_line":81,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":104184,"parent_record_offset":104160,"ancestor_record_offset":103464,"inlinee":4322,"source_line":734,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":103756,"parent_record_offset":103732,"ancestor_record_offset":103464,"inlinee":4302,"source_line":1156,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":103732,"parent_record_offset":103704,"ancestor_record_offset":103464,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":103780,"parent_record_offset":103756,"ancestor_record_offset":103464,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":103832,"parent_record_offset":103780,"ancestor_record_offset":103464,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":104160,"parent_record_offset":103780,"ancestor_record_offset":103464,"inlinee":4321,"source_line":1166,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":1655,"record_offset":104392,"parent_record_offset":103780,"ancestor_record_offset":103464,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":108504,"parent_record_offset":108308,"ancestor_record_offset":103464,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":108344,"parent_record_offset":108308,"ancestor_record_offset":103464,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1655,"record_offset":108428,"parent_record_offset":108400,"ancestor_record_offset":103464,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":108532,"parent_record_offset":108504,"ancestor_record_offset":103464,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":108372,"parent_record_offset":108344,"ancestor_record_offset":103464,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1655,"record_offset":108456,"parent_record_offset":108428,"ancestor_record_offset":103464,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":1655,"record_offset":108400,"parent_record_offset":108372,"ancestor_record_offset":103464,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":107336,"parent_record_offset":107312,"ancestor_record_offset":103464,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":40088,"rva":11441424,"va":5380150544}
// 编译实例 RVAs: 0xae9510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t3321970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e872b03200","size":5,"soft_fail":false,"target_va":5383472768,"va":5380150793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141751070
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13006389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e83576c600","size":5,"soft_fail":false,"target_va":5393158256,"va":5380151862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141767a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13098345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e869ddc700","size":5,"soft_fail":false,"target_va":5393250944,"va":5380152594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176a9e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13111553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e80111c800","size":5,"soft_fail":false,"target_va":5393263072,"va":5380151514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13135498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e88a6ec800","size":5,"soft_fail":false,"target_va":5393288768,"va":5380153265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417863d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13222683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e81bc3c900","size":5,"soft_fail":false,"target_va":5393376208,"va":5380153520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141787050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13224584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e888cac900","size":5,"soft_fail":false,"target_va":5393379408,"va":5380154819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14178ea90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13256871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e8a748ca00","size":5,"soft_fail":false,"target_va":5393410704,"va":5380153828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141795de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13285786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e89ab9ca00","size":5,"soft_fail":false,"target_va":5393440224,"va":5380154433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4468931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e8c3304400","size":5,"soft_fail":false,"target_va":5384624080,"va":5380155144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36466688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e800702c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380154907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36466663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e8e76f2c02","size":5,"soft_fail":false,"target_va":5416621632,"va":5380154964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::clear_color_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3394674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e872cc3300","size":5,"soft_fail":false,"target_va":5383547808,"va":5380153129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3397406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e81ed73300","size":5,"soft_fail":false,"target_va":5383548144,"va":5380150733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3382171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e89b9b3300","size":5,"soft_fail":false,"target_va":5383536672,"va":5380154496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_indirect
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3385229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e88da73300","size":5,"soft_fail":false,"target_va":5383538768,"va":5380153534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3385853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e8fda93300","size":5,"soft_fail":false,"target_va":5383539152,"va":5380153294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::BufferDesc::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3372880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380150544,"raw_hex":"e850773300","size":5,"soft_fail":false,"target_va":5383524160,"va":5380151275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":40800,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":43684,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":556,"record_offset":40724,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::apply_volumetrics::Volumetrics::reflection_fog
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs
//     evidence={"module":556,"record_offset":42808,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":34749,"source_line":35,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::sampled
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":556,"record_offset":40696,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":9289,"source_line":512,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":556,"record_offset":42848,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":40364,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":556,"record_offset":43496,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::dims_2d
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":40304,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":13021,"source_line":791,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::with_view
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":43364,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":30194,"source_line":696,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::half_res
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":42444,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":14910,"source_line":395,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::pixel_count
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":40604,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":34747,"source_line":415,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::Temporal::into_inner
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/temporal.rs
//     evidence={"module":556,"record_offset":43308,"parent_record_offset":40088,"ancestor_record_offset":40088,"inlinee":20515,"source_line":133,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":42120,"parent_record_offset":42096,"ancestor_record_offset":40088,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":42168,"parent_record_offset":42144,"ancestor_record_offset":40088,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":42144,"parent_record_offset":42120,"ancestor_record_offset":40088,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":42096,"parent_record_offset":42072,"ancestor_record_offset":40088,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":42072,"parent_record_offset":42032,"ancestor_record_offset":40088,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":42032,"parent_record_offset":42008,"ancestor_record_offset":40088,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":556,"record_offset":40828,"parent_record_offset":40800,"ancestor_record_offset":40088,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":42392,"parent_record_offset":42368,"ancestor_record_offset":40088,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":42368,"parent_record_offset":42344,"ancestor_record_offset":40088,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":42316,"parent_record_offset":42284,"ancestor_record_offset":40088,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":42260,"parent_record_offset":41752,"ancestor_record_offset":40088,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":42284,"parent_record_offset":42260,"ancestor_record_offset":40088,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":41780,"parent_record_offset":41752,"ancestor_record_offset":40088,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":42344,"parent_record_offset":42284,"ancestor_record_offset":40088,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":41828,"parent_record_offset":41804,"ancestor_record_offset":40088,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":41804,"parent_record_offset":41780,"ancestor_record_offset":40088,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":42008,"parent_record_offset":41984,"ancestor_record_offset":40088,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":41752,"parent_record_offset":40800,"ancestor_record_offset":40088,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":556,"record_offset":40852,"parent_record_offset":40828,"ancestor_record_offset":40088,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":556,"record_offset":41500,"parent_record_offset":41476,"ancestor_record_offset":40088,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":556,"record_offset":41528,"parent_record_offset":41500,"ancestor_record_offset":40088,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":556,"record_offset":42536,"parent_record_offset":42468,"ancestor_record_offset":40088,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":41068,"parent_record_offset":41044,"ancestor_record_offset":40088,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":41152,"parent_record_offset":41128,"ancestor_record_offset":40088,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":556,"record_offset":42496,"parent_record_offset":42468,"ancestor_record_offset":40088,"inlinee":14912,"source_line":3657,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":41300,"parent_record_offset":41272,"ancestor_record_offset":40088,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":41272,"parent_record_offset":41244,"ancestor_record_offset":40088,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":41920,"parent_record_offset":41856,"ancestor_record_offset":40088,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":41944,"parent_record_offset":41920,"ancestor_record_offset":40088,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":41984,"parent_record_offset":41804,"ancestor_record_offset":40088,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":41092,"parent_record_offset":41068,"ancestor_record_offset":40088,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":40768,"parent_record_offset":40724,"ancestor_record_offset":40088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":556,"record_offset":41416,"parent_record_offset":41392,"ancestor_record_offset":40088,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":41856,"parent_record_offset":41828,"ancestor_record_offset":40088,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":41892,"parent_record_offset":41856,"ancestor_record_offset":40088,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":42212,"parent_record_offset":42032,"ancestor_record_offset":40088,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::ice_deferred_rt::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":556,"record_offset":42956,"parent_record_offset":42920,"ancestor_record_offset":40088,"inlinee":34753,"source_line":41198,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::ice_refl_march::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":556,"record_offset":42888,"parent_record_offset":42848,"ancestor_record_offset":40088,"inlinee":34751,"source_line":41424,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::ice_refl_resolve::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":556,"record_offset":43624,"parent_record_offset":43588,"ancestor_record_offset":40088,"inlinee":34624,"source_line":41680,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::ice_refr_resolve::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":556,"record_offset":43464,"parent_record_offset":43428,"ancestor_record_offset":40088,"inlinee":34755,"source_line":41754,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":556,"record_offset":41476,"parent_record_offset":41452,"ancestor_record_offset":40088,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":41212,"parent_record_offset":41184,"ancestor_record_offset":40088,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":41244,"parent_record_offset":41212,"ancestor_record_offset":40088,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":41184,"parent_record_offset":40964,"ancestor_record_offset":40088,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":41044,"parent_record_offset":40964,"ancestor_record_offset":40088,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":41604,"parent_record_offset":40964,"ancestor_record_offset":40088,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":41128,"parent_record_offset":40964,"ancestor_record_offset":40088,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":40888,"parent_record_offset":40852,"ancestor_record_offset":40088,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":41452,"parent_record_offset":41344,"ancestor_record_offset":40088,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":41392,"parent_record_offset":41368,"ancestor_record_offset":40088,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":41696,"parent_record_offset":40964,"ancestor_record_offset":40088,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":41368,"parent_record_offset":41344,"ancestor_record_offset":40088,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":40940,"parent_record_offset":40916,"ancestor_record_offset":40088,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":40916,"parent_record_offset":40888,"ancestor_record_offset":40088,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":40964,"parent_record_offset":40940,"ancestor_record_offset":40088,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":41016,"parent_record_offset":40964,"ancestor_record_offset":40088,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":41344,"parent_record_offset":40964,"ancestor_record_offset":40088,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":556,"record_offset":41576,"parent_record_offset":40964,"ancestor_record_offset":40088,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":40332,"parent_record_offset":40304,"ancestor_record_offset":40088,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::div_ceil_dims
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":42468,"parent_record_offset":42444,"ancestor_record_offset":40088,"inlinee":14911,"source_line":379,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::impl$59::clone
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":43396,"parent_record_offset":43364,"ancestor_record_offset":40088,"inlinee":15346,"source_line":182,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 348
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14076
// 首条位置: {"module":1655,"inlinee":82038}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1655,"record_offset":155420,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":158412,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":157092,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1655,"record_offset":157256,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1655,"record_offset":157320,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_bindless_handle_by_name_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":1655,"record_offset":157472,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":30266,"source_line":429,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1655,"record_offset":160112,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":1655,"record_offset":161108,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::ShaderPipelineCommon::debug_name
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":1655,"record_offset":157444,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":10388,"source_line":150,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::rg
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":157064,"parent_record_offset":155252,"ancestor_record_offset":155252,"inlinee":34679,"source_line":1266,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":156740,"parent_record_offset":156716,"ancestor_record_offset":155252,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":156788,"parent_record_offset":156764,"ancestor_record_offset":155252,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":156764,"parent_record_offset":156740,"ancestor_record_offset":155252,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":156716,"parent_record_offset":156692,"ancestor_record_offset":155252,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":156692,"parent_record_offset":156652,"ancestor_record_offset":155252,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":156652,"parent_record_offset":156628,"ancestor_record_offset":155252,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":1655,"record_offset":155448,"parent_record_offset":155420,"ancestor_record_offset":155252,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1655,"record_offset":157012,"parent_record_offset":156988,"ancestor_record_offset":155252,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1655,"record_offset":156988,"parent_record_offset":156964,"ancestor_record_offset":155252,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1655,"record_offset":156936,"parent_record_offset":156904,"ancestor_record_offset":155252,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":156880,"parent_record_offset":156372,"ancestor_record_offset":155252,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":156904,"parent_record_offset":156880,"ancestor_record_offset":155252,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":156400,"parent_record_offset":156372,"ancestor_record_offset":155252,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1655,"record_offset":156964,"parent_record_offset":156904,"ancestor_record_offset":155252,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1655,"record_offset":156448,"parent_record_offset":156424,"ancestor_record_offset":155252,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1655,"record_offset":156424,"parent_record_offset":156400,"ancestor_record_offset":155252,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1655,"record_offset":156628,"parent_record_offset":156604,"ancestor_record_offset":155252,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1655,"record_offset":156372,"parent_record_offset":155420,"ancestor_record_offset":155252,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":1655,"record_offset":155472,"parent_record_offset":155448,"ancestor_record_offset":155252,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1655,"record_offset":156120,"parent_record_offset":156096,"ancestor_record_offset":155252,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":156148,"parent_record_offset":156120,"ancestor_record_offset":155252,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":157348,"parent_record_offset":157320,"ancestor_record_offset":155252,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":155688,"parent_record_offset":155664,"ancestor_record_offset":155252,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":155772,"parent_record_offset":155748,"ancestor_record_offset":155252,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":158236,"parent_record_offset":157600,"ancestor_record_offset":155252,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":155920,"parent_record_offset":155892,"ancestor_record_offset":155252,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":155892,"parent_record_offset":155864,"ancestor_record_offset":155252,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1655,"record_offset":156540,"parent_record_offset":156476,"ancestor_record_offset":155252,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":156564,"parent_record_offset":156540,"ancestor_record_offset":155252,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":158436,"parent_record_offset":158412,"ancestor_record_offset":155252,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":156604,"parent_record_offset":156424,"ancestor_record_offset":155252,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":155712,"parent_record_offset":155688,"ancestor_record_offset":155252,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1655,"record_offset":156036,"parent_record_offset":156012,"ancestor_record_offset":155252,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1655,"record_offset":157284,"parent_record_offset":157256,"ancestor_record_offset":155252,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1655,"record_offset":157376,"parent_record_offset":157348,"ancestor_record_offset":155252,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1655,"record_offset":157404,"parent_record_offset":157376,"ancestor_record_offset":155252,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1655,"record_offset":156476,"parent_record_offset":156448,"ancestor_record_offset":155252,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1655,"record_offset":156512,"parent_record_offset":156476,"ancestor_record_offset":155252,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1655,"record_offset":156832,"parent_record_offset":156652,"ancestor_record_offset":155252,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1655,"record_offset":160884,"parent_record_offset":160860,"ancestor_record_offset":155252,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1655,"record_offset":160860,"parent_record_offset":160836,"ancestor_record_offset":155252,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":1655,"record_offset":156096,"parent_record_offset":156072,"ancestor_record_offset":155252,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":155832,"parent_record_offset":155804,"ancestor_record_offset":155252,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":155864,"parent_record_offset":155832,"ancestor_record_offset":155252,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":155804,"parent_record_offset":155584,"ancestor_record_offset":155252,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":155664,"parent_record_offset":155584,"ancestor_record_offset":155252,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":156224,"parent_record_offset":155584,"ancestor_record_offset":155252,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":155748,"parent_record_offset":155584,"ancestor_record_offset":155252,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/tag.rs
//     evidence={"module":1655,"record_offset":160324,"parent_record_offset":160268,"ancestor_record_offset":155252,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1655,"record_offset":158172,"parent_record_offset":158148,"ancestor_record_offset":155252,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1655,"record_offset":155508,"parent_record_offset":155472,"ancestor_record_offset":155252,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1655,"record_offset":156072,"parent_record_offset":155964,"ancestor_record_offset":155252,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1655,"record_offset":160192,"parent_record_offset":160160,"ancestor_record_offset":155252,"inlinee":4403,"source_line":240,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":158088,"parent_record_offset":158004,"ancestor_record_offset":155252,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":156012,"parent_record_offset":155988,"ancestor_record_offset":155252,"inlinee":4324,"source_line":261,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":156316,"parent_record_offset":155584,"ancestor_record_offset":155252,"inlinee":4339,"source_line":81,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":155988,"parent_record_offset":155964,"ancestor_record_offset":155252,"inlinee":4322,"source_line":734,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":155560,"parent_record_offset":155536,"ancestor_record_offset":155252,"inlinee":4302,"source_line":1156,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":155536,"parent_record_offset":155508,"ancestor_record_offset":155252,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":155584,"parent_record_offset":155560,"ancestor_record_offset":155252,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":155636,"parent_record_offset":155584,"ancestor_record_offset":155252,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":155964,"parent_record_offset":155584,"ancestor_record_offset":155252,"inlinee":4321,"source_line":1166,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":1655,"record_offset":156196,"parent_record_offset":155584,"ancestor_record_offset":155252,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":161304,"parent_record_offset":161108,"ancestor_record_offset":155252,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":161144,"parent_record_offset":161108,"ancestor_record_offset":155252,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1655,"record_offset":161228,"parent_record_offset":161200,"ancestor_record_offset":155252,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":161332,"parent_record_offset":161304,"ancestor_record_offset":155252,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":161172,"parent_record_offset":161144,"ancestor_record_offset":155252,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1655,"record_offset":161256,"parent_record_offset":161228,"ancestor_record_offset":155252,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":1655,"record_offset":161200,"parent_record_offset":161172,"ancestor_record_offset":155252,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":157500,"parent_record_offset":157472,"ancestor_record_offset":155252,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 511
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":48200,"rva":11448320,"va":5380157440}
// 编译实例 RVAs: 0xaeb000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141764810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13080522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157440,"raw_hex":"e8ca97c700","size":5,"soft_fail":false,"target_va":5393238032,"va":5380157505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":48380,"parent_record_offset":48200,"ancestor_record_offset":48200,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14069
// 首条位置: {"module":1655,"inlinee":82025}
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
// 完整结构及原始类型记录: transparency_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: transparency_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: transparency_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/calc_deferred_rt_args","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::calc_deferred_rt_args::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":47748,"parent_record_offset":47712,"ancestor_record_offset":43716,"inlinee":30814,"source_line":40766,"archive":"_pdb/C/modules/00556.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/deferred_rt","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::deferred_rt::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":47888,"parent_record_offset":47848,"ancestor_record_offset":43716,"inlinee":30655,"source_line":40864,"archive":"_pdb/C/modules/00556.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/ice_refl_march","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::ice_refl_march::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":42888,"parent_record_offset":42848,"ancestor_record_offset":40088,"inlinee":34751,"source_line":41424,"archive":"_pdb/C/modules/00556.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/ice_deferred_rt","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::ice_deferred_rt::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":42956,"parent_record_offset":42920,"ancestor_record_offset":40088,"inlinee":34753,"source_line":41198,"archive":"_pdb/C/modules/00556.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/ice_refr_resolve","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::ice_refr_resolve::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":43464,"parent_record_offset":43428,"ancestor_record_offset":40088,"inlinee":34755,"source_line":41754,"archive":"_pdb/C/modules/00556.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/ice_refl_resolve","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::ice_refl_resolve::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":43624,"parent_record_offset":43588,"ancestor_record_offset":40088,"inlinee":34624,"source_line":41680,"archive":"_pdb/C/modules/00556.jsonl.gz"}}
// [Shader 映射] {"shader_id":"5abc196b877ce7c2af34c226","relative_path":"country-core/compiled/water/calc_deferred_rt_args.precomp","sha256":"729b62ab6ffa778918a9582a8e51bae593f169ef908c008a33e9b467c78210c9","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass","hint":{"shader_family":"country_core/water/calc_deferred_rt_args","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::calc_deferred_rt_args::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":47748,"parent_record_offset":47712,"ancestor_record_offset":43716,"inlinee":30814,"source_line":40766,"archive":"_pdb/C/modules/00556.jsonl.gz"}},"shader_uri":"country_core/water/calc_deferred_rt_args:main"}
// [Shader 映射] {"shader_id":"a1cbfe89c5e78fd0ddaf7673","relative_path":"country-core/compiled/water/deferred_rt.precomp","sha256":"51fee1d60913c25131e672c6494fcf9d2338667941ab690234b31a9ce7c22783","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass","hint":{"shader_family":"country_core/water/deferred_rt","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::deferred_rt::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":47888,"parent_record_offset":47848,"ancestor_record_offset":43716,"inlinee":30655,"source_line":40864,"archive":"_pdb/C/modules/00556.jsonl.gz"}},"shader_uri":"country_core/water/deferred_rt:main"}
// [Shader 映射] {"shader_id":"4e6a9deacf80da0a3576e04a","relative_path":"country-core/compiled/water/ice_refl_march.precomp","sha256":"6caf3a751c48a542e40c1a5f96a7470c4a841a2df80037ff5eef8e11acca2a83","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass","hint":{"shader_family":"country_core/water/ice_refl_march","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::ice_refl_march::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":42888,"parent_record_offset":42848,"ancestor_record_offset":40088,"inlinee":34751,"source_line":41424,"archive":"_pdb/C/modules/00556.jsonl.gz"}},"shader_uri":"country_core/water/ice_refl_march:main"}
// [Shader 映射] {"shader_id":"3a4bf0cc09389b677370a643","relative_path":"country-core/compiled/water/ice_deferred_rt.precomp","sha256":"97a7d40ed72e6fda38781e9f6ac265f4bfe5d0ee6997a36eff16dcb9b650e41e","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass","hint":{"shader_family":"country_core/water/ice_deferred_rt","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::ice_deferred_rt::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":42956,"parent_record_offset":42920,"ancestor_record_offset":40088,"inlinee":34753,"source_line":41198,"archive":"_pdb/C/modules/00556.jsonl.gz"}},"shader_uri":"country_core/water/ice_deferred_rt:main"}
// [Shader 映射] {"shader_id":"cfe37d8ad934757238f4dc2d","relative_path":"country-core/compiled/water/ice_refr_resolve.precomp","sha256":"bead2a8b85de55a7334ee6c4b724d244c566766b456be7dcba98ae7c040fe605","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass","hint":{"shader_family":"country_core/water/ice_refr_resolve","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::ice_refr_resolve::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":43464,"parent_record_offset":43428,"ancestor_record_offset":40088,"inlinee":34755,"source_line":41754,"archive":"_pdb/C/modules/00556.jsonl.gz"}},"shader_uri":"country_core/water/ice_refr_resolve:main"}
// [Shader 映射] {"shader_id":"ec4ac8935accf6c94b285079","relative_path":"country-core/compiled/water/ice_refl_resolve.precomp","sha256":"6507cdb23df01140730aa945bdac91cec7e1d0f9994054c8cab40664b226694b","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass","hint":{"shader_family":"country_core/water/ice_refl_resolve","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::ice_refl_resolve::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":43624,"parent_record_offset":43588,"ancestor_record_offset":40088,"inlinee":34624,"source_line":41680,"archive":"_pdb/C/modules/00556.jsonl.gz"}},"shader_uri":"country_core/water/ice_refl_resolve:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/water/ice_deferred_rt.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[76]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":70,"name":"ice_rt_count_buffer","storage_class":"Uniform","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,67]},"decorations":[],"pointee":{"id":67,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[68]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":68,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":84,"name":"ice_rt_ray_buffer","storage_class":"Uniform","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,67]},"decorations":[],"pointee":{"id":67,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[68]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":68,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":103,"name":"current_view","storage_class":"Uniform","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,99]},"decorations":[],"pointee":{"id":99,"name":"ViewConstants_natural","record":{"opcode":"OpTypeStruct","operands":[100,100,100,100,100,100,100,100,100,100,100,100,100,101,101,101,101,100,100,15,6,6,6,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"view_to_clip","type_id":100,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":1,"name":"clip_to_view","type_id":100,"decorations":[{"decoration":"Offset","operands":[64]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":2,"name":"view_to_sample","type_id":100,"decorations":[{"decoration":"Offset","operands":[128]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":3,"name":"sample_to_view","type_id":100,"decorations":[{"decoration":"Offset","operands":[192]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":4,"name":"world_to_view","type_id":100,"decorations":[{"decoration":"Offset","operands":[256]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":5,"name":"view_to_world","type_id":100,"decorations":[{"decoration":"Offset","operands":[320]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":6,"name":"world_to_sample","type_id":100,"decorations":[{"decoration":"Offset","operands":[384]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":7,"name":"clip_to_prev_clip","type_id":100,"decorations":[{"decoration":"Offset","operands":[448]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":8,"name":"prev_clip_to_clip","type_id":100,"decorations":[{"decoration":"Offset","operands":[512]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":9,"name":"prev_view_to_prev_clip","type_id":100,"decorations":[{"decoration":"Offset","operands":[576]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":10,"name":"prev_clip_to_prev_view","type_id":100,"decorations":[{"decoration":"Offset","operands":[640]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":11,"name":"prev_world_to_prev_view","type_id":100,"decorations":[{"decoration":"Offset","operands":[704]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":12,"name":"prev_view_to_prev_world","type_id":100,"decorations":[{"decoration":"Offset","operands":[768]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":13,"name":"sample_offset_pixels","type_id":101,"decorations":[{"decoration":"Offset","operands":[832]}]},{"index":14,"name":"sample_offset_clip","type_id":101,"decorations":[{"decoration":"Offset","operands":[840]}]},{"index":15,"name":"prev_sample_offset_pixels","type_id":101,"decorations":[{"decoration":"Offset","operands":[848]}]},{"index":16,"name":"prev_sample_offset_clip","type_id":101,"decorations":[{"decoration":"Offset","operands":[856]}]},{"index":17,"name":"pad0","type_id":100,"decorations":[{"decoration":"Offset","operands":[864]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":18,"name":"pad1","type_id":100,"decorations":[{"decoration":"Offset","operands":[928]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":19,"name":"pad2","type_id":15,"decorations":[{"decoration":"Offset","operands":[992]}]},{"index":20,"name":"pad3","type_id":6,"decorations":[{"decoration":"Offset","operands":[1008]}]},{"index":21,"name":"pad4","type_id":6,"decorations":[{"decoration":"Offset","operands":[1012]}]},{"index":22,"name":"pad5","type_id":6,"decorations":[{"decoration":"Offset","operands":[1016]}]},{"index":23,"name":"mip_bias","type_id":6,"decorations":[{"decoration":"Offset","operands":[1020]}]}]}},"descriptor_set":5,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[5]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":247,"name":"frame_counter","storage_class":"Uniform","type":{"id":246,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,245]},"decorations":[],"pointee":{"id":245,"name":"FrameCounter_natural","record":{"opcode":"OpTypeStruct","operands":[63]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"index","type_id":63,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":3,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[3]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":288,"name":"blue_noise_textures.uniform01","storage_class":"UniformConstant","type":{"id":287,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,285]},"decorations":[],"pointee":{"id":285,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":530,"name":"rt_scene","storage_class":"Uniform","type":{"id":529,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,528]},"decorations":[],"pointee":{"id":528,"name":"RtScene_natural","record":{"opcode":"OpTypeStruct","operands":[63]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"rt_triangle_count","type_id":63,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":24,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[24]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":553,"name":"rt_scene.rt_bl_bvh","storage_class":"Uniform","type":{"id":552,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,550]},"decorations":[],"pointee":{"id":550,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[551]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":551,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":24,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[24]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1255,"name":"rt_scene.rt_triangles","storage_class":"Uniform","type":{"id":1254,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1252]},"decorations":[],"pointee":{"id":1252,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[1253]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":1253,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":24,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[24]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1612,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":1611,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1609]},"decorations":[],"pointee":{"id":1609,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[92,1610,1610,101,15]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":1610,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":1610,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":101,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":15,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":22,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[22]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1984,"name":"resource_handles","storage_class":"UniformConstant","type":{"id":1983,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1982]},"decorations":[],"pointee":{"id":1982,"name":"unknown","record":{"opcode":"OpTypeRuntimeArray","operands":[285]},"decorations":[],"element":{"id":285,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1997,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":1996,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1994]},"decorations":[],"pointee":{"id":1994,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2418,"name":"visual_terrain_heightmap","storage_class":"Uniform","type":{"id":2417,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,2416]},"decorations":[],"pointee":{"id":2416,"name":"VisualTerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[92]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"visual_terrain_heightmap_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":23,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[23]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2434,"name":"samplers.lnc","storage_class":"UniformConstant","type":{"id":1996,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1994]},"decorations":[],"pointee":{"id":1994,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2436,"name":"visual_terrain_heightmap.visual_terrain_heightmap","storage_class":"UniformConstant","type":{"id":287,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,285]},"decorations":[],"pointee":{"id":285,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":23,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[23]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2667,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":287,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,285]},"decorations":[],"pointee":{"id":285,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":22,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[22]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2763,"name":"frame_constants","storage_class":"Uniform","type":{"id":2762,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,2761]},"decorations":[],"pointee":{"id":2761,"name":"FrameConstants_natural","record":{"opcode":"OpTypeStruct","operands":[15,15,15,15,15,15,15,15,6,6,6,63,92,63,6,7,6,7,63,7,63,7,6,15,6,63,92,6,63,63,63,7,6,63,63,63,63,63,6,6,6,15,7,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":15,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":15,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":15,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":15,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":15,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":15,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":15,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":15,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"time_since_startup_secs","type_id":6,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":6,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":63,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"world_render_resolution","type_id":92,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":63,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":6,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":6,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":7,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":63,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":7,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":63,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"dir_to_primary_directional_light","type_id":7,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"main_view_lod_scale","type_id":6,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":23,"name":"main_view_eye_position","type_id":15,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":24,"name":"fog_relative_extinction","type_id":6,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":25,"name":"highlighted_transient_stair_edge_id","type_id":63,"decorations":[{"decoration":"Offset","operands":[244]}]},{"index":26,"name":"swapchain_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[248]}]},{"index":27,"name":"swapchain_aspect_ratio","type_id":6,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":28,"name":"is_photomode","type_id":63,"decorations":[{"decoration":"Offset","operands":[260]}]},{"index":29,"name":"is_topdown_mode","type_id":63,"decorations":[{"decoration":"Offset","operands":[264]}]},{"index":30,"name":"is_firstperson_mode","type_id":63,"decorations":[{"decoration":"Offset","operands":[268]}]},{"index":31,"name":"brush_position","type_id":7,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":32,"name":"eraser_size_multiplier","type_id":6,"decorations":[{"decoration":"Offset","operands":[284]}]},{"index":33,"name":"roof_highlighted_slot_id","type_id":63,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":34,"name":"roof_highlight_mode","type_id":63,"decorations":[{"decoration":"Offset","operands":[292]}]},{"index":35,"name":"wall_highlighted_slot_id","type_id":63,"decorations":[{"decoration":"Offset","operands":[296]}]},{"index":36,"name":"wall_highlight_mode","type_id":63,"decorations":[{"decoration":"Offset","operands":[300]}]},{"index":37,"name":"highlight_bricks_to_delete","type_id":63,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":38,"name":"wind_strength","type_id":6,"decorations":[{"decoration":"Offset","operands":[308]}]},{"index":39,"name":"tree_canopy_cone_angle_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[312]}]},{"index":40,"name":"tree_canopy_max_cone_cull_dist_range","type_id":6,"decorations":[{"decoration":"Offset","operands":[316]}]},{"index":41,"name":"debug_tweakers","type_id":15,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":42,"name":"ground_fog","type_id":7,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":43,"name":"moonswap","type_id":6,"decorations":[{"decoration":"Offset","operands":[348]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2860,"name":"color_library.data","storage_class":"Uniform","type":{"id":2859,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,2858]},"decorations":[],"pointee":{"id":2858,"name":"ColorLibraryData_natural","record":{"opcode":"OpTypeStruct","operands":[15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":15,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":15,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":15,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":15,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":15,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":15,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":15,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":15,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":15,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":15,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":15,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":15,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":15,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":15,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":15,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":15,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":15,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":15,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":15,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":21,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[21]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3480,"name":"csm","storage_class":"Uniform","type":{"id":3479,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,3478]},"decorations":[],"pointee":{"id":3478,"name":"CascadedShadows_natural","record":{"opcode":"OpTypeStruct","operands":[92]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_shadow_map_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":26,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[26]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3500,"name":"csm_setup.sun_view","storage_class":"Uniform","type":{"id":3499,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,3497]},"decorations":[],"pointee":{"id":3497,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[3498]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":3498,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":25,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[25]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3533,"name":"csm_setup.cascade_splits_buf","storage_class":"Uniform","type":{"id":3532,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,3530]},"decorations":[],"pointee":{"id":3530,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[3531]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":3531,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":25,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[25]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3726,"name":"csm.sun_shadow_map","storage_class":"UniformConstant","type":{"id":3725,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,3723]},"decorations":[],"pointee":{"id":3723,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":26,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[26]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3745,"name":"csm.sun_shadow_stencil_map","storage_class":"UniformConstant","type":{"id":3744,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,3742]},"decorations":[],"pointee":{"id":3742,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[63,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":26,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[26]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3873,"name":"samplers.shadow_linear","storage_class":"UniformConstant","type":{"id":1996,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1994]},"decorations":[],"pointee":{"id":1994,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[11]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4493,"name":"rt_scene.rt_triangle_meta","storage_class":"Uniform","type":{"id":4492,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4490]},"decorations":[],"pointee":{"id":4490,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[4491]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":4491,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":24,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[24]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4501,"name":"wall_color_data.wall_color_ids","storage_class":"Uniform","type":{"id":4492,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4490]},"decorations":[],"pointee":{"id":4490,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[4491]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":4491,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":28,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[28]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4585,"name":"roof_colors.ids","storage_class":"Uniform","type":{"id":4492,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4490]},"decorations":[],"pointee":{"id":4490,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[4491]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":4491,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":29,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[29]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4808,"name":"samplers.nnc","storage_class":"UniformConstant","type":{"id":1996,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1994]},"decorations":[],"pointee":{"id":1994,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4810,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":287,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,285]},"decorations":[],"pointee":{"id":285,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4839,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":287,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,285]},"decorations":[],"pointee":{"id":285,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4856,"name":"globalParams","storage_class":"Uniform","type":{"id":4855,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4852]},"decorations":[],"pointee":{"id":4852,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[63,92,92,92,92,92,4853]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"history_valid","type_id":63,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"half_depth_tex_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"taa_history_tex_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"reprojection_tex_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":4,"name":"refl_hit_tex_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[28]}]},{"index":5,"name":"refl_hit_radiance_tex_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[36]}]},{"index":6,"name":"reflection_fog","type_id":4853,"decorations":[{"decoration":"Offset","operands":[44]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4980,"name":"refl_hit_tex","storage_class":"UniformConstant","type":{"id":4979,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4977]},"decorations":[],"pointee":{"id":4977,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":6,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[6]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5029,"name":"taa_history_tex","storage_class":"UniformConstant","type":{"id":287,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,285]},"decorations":[],"pointee":{"id":285,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5114,"name":"globalParams.reflection_fog.froxel_scattering_tex","storage_class":"UniformConstant","type":{"id":5113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5111]},"decorations":[],"pointee":{"id":5111,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[11]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5133,"name":"globalParams.reflection_fog.froxel_extinction_tex","storage_class":"UniformConstant","type":{"id":5113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5111]},"decorations":[],"pointee":{"id":5111,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[12]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5468,"name":"samplers.llr","storage_class":"UniformConstant","type":{"id":1996,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1994]},"decorations":[],"pointee":{"id":1994,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5470,"name":"fog_noise_texture.tex","storage_class":"UniformConstant","type":{"id":5113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5111]},"decorations":[],"pointee":{"id":5111,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":31,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[31]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5596,"name":"globalParams.reflection_fog.volumetric_shadows.volumetric_shadow_rendering_views","storage_class":"Uniform","type":{"id":5595,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5593]},"decorations":[],"pointee":{"id":5593,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[5594]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":5594,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[9]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5661,"name":"globalParams.reflection_fog.volumetric_shadows.volumetric_shadow_map_tex","storage_class":"UniformConstant","type":{"id":3725,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,3723]},"decorations":[],"pointee":{"id":3723,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":8,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[8]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5794,"name":"globalParams.reflection_fog.sky_visibility_volume_tex","storage_class":"UniformConstant","type":{"id":5113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5111]},"decorations":[],"pointee":{"id":5111,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[10]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6069,"name":"refl_hit_radiance_tex","storage_class":"UniformConstant","type":{"id":6068,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6066]},"decorations":[],"pointee":{"id":6066,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[63,1,2,0,0,2,33]},"decorations":[]}},"descriptor_set":0,"binding":7,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[7]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":76,"name":"unknown","storage_class":"Input","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,73]},"decorations":[],"pointee":{"id":73,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[63,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/water/ice_refl_march.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[97]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":91,"name":"ice_refl_count_buffer","storage_class":"Uniform","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,88]},"decorations":[],"pointee":{"id":88,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[89]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":89,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":109,"name":"ice_refl_px_buffer","storage_class":"Uniform","type":{"id":108,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,106]},"decorations":[],"pointee":{"id":106,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[107]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":107,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":127,"name":"globalParams","storage_class":"Uniform","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,123]},"decorations":[],"pointee":{"id":123,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[17,116,116,116,116,116,116,124]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"history_valid","type_id":17,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"half_depth_tex_dims","type_id":116,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"taa_history_tex_dims","type_id":116,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"reprojection_tex_dims","type_id":116,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":4,"name":"ice_roughness_tex_dims","type_id":116,"decorations":[{"decoration":"Offset","operands":[28]}]},{"index":5,"name":"refl_hit_tex_dims","type_id":116,"decorations":[{"decoration":"Offset","operands":[36]}]},{"index":6,"name":"refl_hit_radiance_tex_dims","type_id":116,"decorations":[{"decoration":"Offset","operands":[44]}]},{"index":7,"name":"reflection_fog","type_id":124,"decorations":[{"decoration":"Offset","operands":[52]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":173,"name":"current_view","storage_class":"Uniform","type":{"id":172,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,170]},"decorations":[],"pointee":{"id":170,"name":"ViewConstants_natural","record":{"opcode":"OpTypeStruct","operands":[171,171,171,171,171,171,171,171,171,171,171,171,171,22,22,22,22,171,171,7,6,6,6,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"view_to_clip","type_id":171,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":1,"name":"clip_to_view","type_id":171,"decorations":[{"decoration":"Offset","operands":[64]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":2,"name":"view_to_sample","type_id":171,"decorations":[{"decoration":"Offset","operands":[128]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":3,"name":"sample_to_view","type_id":171,"decorations":[{"decoration":"Offset","operands":[192]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":4,"name":"world_to_view","type_id":171,"decorations":[{"decoration":"Offset","operands":[256]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":5,"name":"view_to_world","type_id":171,"decorations":[{"decoration":"Offset","operands":[320]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":6,"name":"world_to_sample","type_id":171,"decorations":[{"decoration":"Offset","operands":[384]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":7,"name":"clip_to_prev_clip","type_id":171,"decorations":[{"decoration":"Offset","operands":[448]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":8,"name":"prev_clip_to_clip","type_id":171,"decorations":[{"decoration":"Offset","operands":[512]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":9,"name":"prev_view_to_prev_clip","type_id":171,"decorations":[{"decoration":"Offset","operands":[576]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":10,"name":"prev_clip_to_prev_view","type_id":171,"decorations":[{"decoration":"Offset","operands":[640]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":11,"name":"prev_world_to_prev_view","type_id":171,"decorations":[{"decoration":"Offset","operands":[704]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":12,"name":"prev_view_to_prev_world","type_id":171,"decorations":[{"decoration":"Offset","operands":[768]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":13,"name":"sample_offset_pixels","type_id":22,"decorations":[{"decoration":"Offset","operands":[832]}]},{"index":14,"name":"sample_offset_clip","type_id":22,"decorations":[{"decoration":"Offset","operands":[840]}]},{"index":15,"name":"prev_sample_offset_pixels","type_id":22,"decorations":[{"decoration":"Offset","operands":[848]}]},{"index":16,"name":"prev_sample_offset_clip","type_id":22,"decorations":[{"decoration":"Offset","operands":[856]}]},{"index":17,"name":"pad0","type_id":171,"decorations":[{"decoration":"Offset","operands":[864]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":18,"name":"pad1","type_id":171,"decorations":[{"decoration":"Offset","operands":[928]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":19,"name":"pad2","type_id":7,"decorations":[{"decoration":"Offset","operands":[992]}]},{"index":20,"name":"pad3","type_id":6,"decorations":[{"decoration":"Offset","operands":[1008]}]},{"index":21,"name":"pad4","type_id":6,"decorations":[{"decoration":"Offset","operands":[1012]}]},{"index":22,"name":"pad5","type_id":6,"decorations":[{"decoration":"Offset","operands":[1016]}]},{"index":23,"name":"mip_bias","type_id":6,"decorations":[{"decoration":"Offset","operands":[1020]}]}]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":343,"name":"frame_counter","storage_class":"Uniform","type":{"id":342,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,341]},"decorations":[],"pointee":{"id":341,"name":"FrameCounter_natural","record":{"opcode":"OpTypeStruct","operands":[17]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"index","type_id":17,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":3,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[3]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":447,"name":"ice_roughness_tex","storage_class":"UniformConstant","type":{"id":446,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,444]},"decorations":[],"pointee":{"id":444,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":6,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[6]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1564,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":446,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,444]},"decorations":[],"pointee":{"id":444,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1568,"name":"samplers.llc","storage_class":"UniformConstant","type":{"id":1567,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1565]},"decorations":[],"pointee":{"id":1565,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1577,"name":"samplers.nnc","storage_class":"UniformConstant","type":{"id":1567,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1565]},"decorations":[],"pointee":{"id":1565,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1943,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":446,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,444]},"decorations":[],"pointee":{"id":444,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1987,"name":"samplers.lnc","storage_class":"UniformConstant","type":{"id":1567,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1565]},"decorations":[],"pointee":{"id":1565,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1989,"name":"taa_history_tex","storage_class":"UniformConstant","type":{"id":446,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,444]},"decorations":[],"pointee":{"id":444,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2011,"name":"blue_noise_textures.uniform01","storage_class":"UniformConstant","type":{"id":446,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,444]},"decorations":[],"pointee":{"id":444,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":5,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[5]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2026,"name":"frame_constants","storage_class":"Uniform","type":{"id":2025,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,2024]},"decorations":[],"pointee":{"id":2024,"name":"FrameConstants_natural","record":{"opcode":"OpTypeStruct","operands":[7,7,7,7,7,7,7,7,6,6,6,17,116,17,6,11,6,11,17,11,17,11,6,7,6,17,116,6,17,17,17,11,6,17,17,17,17,17,6,6,6,7,11,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":7,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":7,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":7,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"time_since_startup_secs","type_id":6,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":6,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":17,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"world_render_resolution","type_id":116,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":17,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":6,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":11,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":6,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":11,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":17,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":11,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":17,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"dir_to_primary_directional_light","type_id":11,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"main_view_lod_scale","type_id":6,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":23,"name":"main_view_eye_position","type_id":7,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":24,"name":"fog_relative_extinction","type_id":6,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":25,"name":"highlighted_transient_stair_edge_id","type_id":17,"decorations":[{"decoration":"Offset","operands":[244]}]},{"index":26,"name":"swapchain_dims","type_id":116,"decorations":[{"decoration":"Offset","operands":[248]}]},{"index":27,"name":"swapchain_aspect_ratio","type_id":6,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":28,"name":"is_photomode","type_id":17,"decorations":[{"decoration":"Offset","operands":[260]}]},{"index":29,"name":"is_topdown_mode","type_id":17,"decorations":[{"decoration":"Offset","operands":[264]}]},{"index":30,"name":"is_firstperson_mode","type_id":17,"decorations":[{"decoration":"Offset","operands":[268]}]},{"index":31,"name":"brush_position","type_id":11,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":32,"name":"eraser_size_multiplier","type_id":6,"decorations":[{"decoration":"Offset","operands":[284]}]},{"index":33,"name":"roof_highlighted_slot_id","type_id":17,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":34,"name":"roof_highlight_mode","type_id":17,"decorations":[{"decoration":"Offset","operands":[292]}]},{"index":35,"name":"wall_highlighted_slot_id","type_id":17,"decorations":[{"decoration":"Offset","operands":[296]}]},{"index":36,"name":"wall_highlight_mode","type_id":17,"decorations":[{"decoration":"Offset","operands":[300]}]},{"index":37,"name":"highlight_bricks_to_delete","type_id":17,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":38,"name":"wind_strength","type_id":6,"decorations":[{"decoration":"Offset","operands":[308]}]},{"index":39,"name":"tree_canopy_cone_angle_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[312]}]},{"index":40,"name":"tree_canopy_max_cone_cull_dist_range","type_id":6,"decorations":[{"decoration":"Offset","operands":[316]}]},{"index":41,"name":"debug_tweakers","type_id":7,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":42,"name":"ground_fog","type_id":11,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":43,"name":"moonswap","type_id":6,"decorations":[{"decoration":"Offset","operands":[348]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2100,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":1567,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1565]},"decorations":[],"pointee":{"id":1565,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2104,"name":"globalParams.reflection_fog.froxel_scattering_tex","storage_class":"UniformConstant","type":{"id":2103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2101]},"decorations":[],"pointee":{"id":2101,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[14]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2124,"name":"globalParams.reflection_fog.froxel_extinction_tex","storage_class":"UniformConstant","type":{"id":2103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2101]},"decorations":[],"pointee":{"id":2101,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[15]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2172,"name":"ice_rt_count_buffer","storage_class":"Uniform","type":{"id":2171,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,2170]},"decorations":[],"pointee":{"id":2170,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[107]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":107,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[9]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2255,"name":"ice_rt_ray_buffer","storage_class":"Uniform","type":{"id":2254,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,2253]},"decorations":[],"pointee":{"id":2253,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[89]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":89,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[10]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2294,"name":"refl_hit_tex","storage_class":"UniformConstant","type":{"id":2293,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2291]},"decorations":[],"pointee":{"id":2291,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":7,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[7]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2347,"name":"refl_hit_radiance_tex","storage_class":"UniformConstant","type":{"id":2346,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2344]},"decorations":[],"pointee":{"id":2344,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[17,1,2,0,0,2,33]},"decorations":[]}},"descriptor_set":0,"binding":8,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[8]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":97,"name":"unknown","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,94]},"decorations":[],"pointee":{"id":94,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[17,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/water/calc_deferred_rt_args.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[1,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"water_rt_count_buffer","storage_class":"Uniform","type":{"id":13,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[12]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":12,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/water/deferred_rt.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[149]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":143,"name":"water_rt_count_buffer","storage_class":"Uniform","type":{"id":142,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,140]},"decorations":[],"pointee":{"id":140,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[141]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":141,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":157,"name":"water_rt_ray_buffer","storage_class":"Uniform","type":{"id":142,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,140]},"decorations":[],"pointee":{"id":140,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[141]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":141,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":173,"name":"globalParams","storage_class":"Uniform","type":{"id":172,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,169]},"decorations":[],"pointee":{"id":169,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[136,165,165,165,165,170]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"history_valid","type_id":136,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"half_depth_tex_dims","type_id":165,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"taa_history_tex_dims","type_id":165,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"reprojection_tex_dims","type_id":165,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":4,"name":"color_tex_dims","type_id":165,"decorations":[{"decoration":"Offset","operands":[28]}]},{"index":5,"name":"reflection_fog","type_id":170,"decorations":[{"decoration":"Offset","operands":[36]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":223,"name":"current_view","storage_class":"Uniform","type":{"id":222,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,220]},"decorations":[],"pointee":{"id":220,"name":"ViewConstants_natural","record":{"opcode":"OpTypeStruct","operands":[221,221,221,221,221,221,221,221,221,221,221,221,221,178,178,178,178,221,221,7,6,6,6,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"view_to_clip","type_id":221,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":1,"name":"clip_to_view","type_id":221,"decorations":[{"decoration":"Offset","operands":[64]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":2,"name":"view_to_sample","type_id":221,"decorations":[{"decoration":"Offset","operands":[128]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":3,"name":"sample_to_view","type_id":221,"decorations":[{"decoration":"Offset","operands":[192]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":4,"name":"world_to_view","type_id":221,"decorations":[{"decoration":"Offset","operands":[256]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":5,"name":"view_to_world","type_id":221,"decorations":[{"decoration":"Offset","operands":[320]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":6,"name":"world_to_sample","type_id":221,"decorations":[{"decoration":"Offset","operands":[384]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":7,"name":"clip_to_prev_clip","type_id":221,"decorations":[{"decoration":"Offset","operands":[448]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":8,"name":"prev_clip_to_clip","type_id":221,"decorations":[{"decoration":"Offset","operands":[512]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":9,"name":"prev_view_to_prev_clip","type_id":221,"decorations":[{"decoration":"Offset","operands":[576]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":10,"name":"prev_clip_to_prev_view","type_id":221,"decorations":[{"decoration":"Offset","operands":[640]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":11,"name":"prev_world_to_prev_view","type_id":221,"decorations":[{"decoration":"Offset","operands":[704]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":12,"name":"prev_view_to_prev_world","type_id":221,"decorations":[{"decoration":"Offset","operands":[768]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":13,"name":"sample_offset_pixels","type_id":178,"decorations":[{"decoration":"Offset","operands":[832]}]},{"index":14,"name":"sample_offset_clip","type_id":178,"decorations":[{"decoration":"Offset","operands":[840]}]},{"index":15,"name":"prev_sample_offset_pixels","type_id":178,"decorations":[{"decoration":"Offset","operands":[848]}]},{"index":16,"name":"prev_sample_offset_clip","type_id":178,"decorations":[{"decoration":"Offset","operands":[856]}]},{"index":17,"name":"pad0","type_id":221,"decorations":[{"decoration":"Offset","operands":[864]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":18,"name":"pad1","type_id":221,"decorations":[{"decoration":"Offset","operands":[928]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":19,"name":"pad2","type_id":7,"decorations":[{"decoration":"Offset","operands":[992]}]},{"index":20,"name":"pad3","type_id":6,"decorations":[{"decoration":"Offset","operands":[1008]}]},{"index":21,"name":"pad4","type_id":6,"decorations":[{"decoration":"Offset","operands":[1012]}]},{"index":22,"name":"pad5","type_id":6,"decorations":[{"decoration":"Offset","operands":[1016]}]},{"index":23,"name":"mip_bias","type_id":6,"decorations":[{"decoration":"Offset","operands":[1020]}]}]}},"descriptor_set":5,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[5]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":455,"name":"frame_counter","storage_class":"Uniform","type":{"id":454,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,453]},"decorations":[],"pointee":{"id":453,"name":"FrameCounter_natural","record":{"opcode":"OpTypeStruct","operands":[136]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"index","type_id":136,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":3,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[3]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":495,"name":"blue_noise_textures.uniform01","storage_class":"UniformConstant","type":{"id":494,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,492]},"decorations":[],"pointee":{"id":492,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":733,"name":"rt_scene","storage_class":"Uniform","type":{"id":732,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,731]},"decorations":[],"pointee":{"id":731,"name":"RtScene_natural","record":{"opcode":"OpTypeStruct","operands":[136]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"rt_triangle_count","type_id":136,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":24,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[24]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":756,"name":"rt_scene.rt_bl_bvh","storage_class":"Uniform","type":{"id":755,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,753]},"decorations":[],"pointee":{"id":753,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[754]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":754,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":24,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[24]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1458,"name":"rt_scene.rt_triangles","storage_class":"Uniform","type":{"id":1457,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1455]},"decorations":[],"pointee":{"id":1455,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[1456]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":1456,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":24,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[24]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1815,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":1814,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1812]},"decorations":[],"pointee":{"id":1812,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[165,1813,1813,178,7]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":165,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":1813,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":1813,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":178,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":7,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":22,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[22]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2184,"name":"resource_handles","storage_class":"UniformConstant","type":{"id":2183,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2182]},"decorations":[],"pointee":{"id":2182,"name":"unknown","record":{"opcode":"OpTypeRuntimeArray","operands":[492]},"decorations":[],"element":{"id":492,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2197,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":2196,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2194]},"decorations":[],"pointee":{"id":2194,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2618,"name":"visual_terrain_heightmap","storage_class":"Uniform","type":{"id":2617,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,2616]},"decorations":[],"pointee":{"id":2616,"name":"VisualTerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[165]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"visual_terrain_heightmap_dims","type_id":165,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":23,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[23]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2634,"name":"samplers.lnc","storage_class":"UniformConstant","type":{"id":2196,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2194]},"decorations":[],"pointee":{"id":2194,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2636,"name":"visual_terrain_heightmap.visual_terrain_heightmap","storage_class":"UniformConstant","type":{"id":494,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,492]},"decorations":[],"pointee":{"id":492,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":23,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[23]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2867,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":494,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,492]},"decorations":[],"pointee":{"id":492,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":22,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[22]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2963,"name":"frame_constants","storage_class":"Uniform","type":{"id":2962,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,2961]},"decorations":[],"pointee":{"id":2961,"name":"FrameConstants_natural","record":{"opcode":"OpTypeStruct","operands":[7,7,7,7,7,7,7,7,6,6,6,136,165,136,6,11,6,11,136,11,136,11,6,7,6,136,165,6,136,136,136,11,6,136,136,136,136,136,6,6,6,7,11,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":7,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":7,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":7,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"time_since_startup_secs","type_id":6,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":6,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":136,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"world_render_resolution","type_id":165,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":136,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":6,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":11,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":6,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":11,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":136,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":11,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":136,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"dir_to_primary_directional_light","type_id":11,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"main_view_lod_scale","type_id":6,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":23,"name":"main_view_eye_position","type_id":7,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":24,"name":"fog_relative_extinction","type_id":6,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":25,"name":"highlighted_transient_stair_edge_id","type_id":136,"decorations":[{"decoration":"Offset","operands":[244]}]},{"index":26,"name":"swapchain_dims","type_id":165,"decorations":[{"decoration":"Offset","operands":[248]}]},{"index":27,"name":"swapchain_aspect_ratio","type_id":6,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":28,"name":"is_photomode","type_id":136,"decorations":[{"decoration":"Offset","operands":[260]}]},{"index":29,"name":"is_topdown_mode","type_id":136,"decorations":[{"decoration":"Offset","operands":[264]}]},{"index":30,"name":"is_firstperson_mode","type_id":136,"decorations":[{"decoration":"Offset","operands":[268]}]},{"index":31,"name":"brush_position","type_id":11,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":32,"name":"eraser_size_multiplier","type_id":6,"decorations":[{"decoration":"Offset","operands":[284]}]},{"index":33,"name":"roof_highlighted_slot_id","type_id":136,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":34,"name":"roof_highlight_mode","type_id":136,"decorations":[{"decoration":"Offset","operands":[292]}]},{"index":35,"name":"wall_highlighted_slot_id","type_id":136,"decorations":[{"decoration":"Offset","operands":[296]}]},{"index":36,"name":"wall_highlight_mode","type_id":136,"decorations":[{"decoration":"Offset","operands":[300]}]},{"index":37,"name":"highlight_bricks_to_delete","type_id":136,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":38,"name":"wind_strength","type_id":6,"decorations":[{"decoration":"Offset","operands":[308]}]},{"index":39,"name":"tree_canopy_cone_angle_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[312]}]},{"index":40,"name":"tree_canopy_max_cone_cull_dist_range","type_id":6,"decorations":[{"decoration":"Offset","operands":[316]}]},{"index":41,"name":"debug_tweakers","type_id":7,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":42,"name":"ground_fog","type_id":11,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":43,"name":"moonswap","type_id":6,"decorations":[{"decoration":"Offset","operands":[348]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3060,"name":"color_library.data","storage_class":"Uniform","type":{"id":3059,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,3058]},"decorations":[],"pointee":{"id":3058,"name":"ColorLibraryData_natural","record":{"opcode":"OpTypeStruct","operands":[7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":7,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":7,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":7,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":7,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":7,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":7,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":7,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":7,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":7,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":7,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":7,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":7,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":7,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":7,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":7,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":7,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":7,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":7,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":7,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":21,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[21]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3681,"name":"csm","storage_class":"Uniform","type":{"id":3680,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,3679]},"decorations":[],"pointee":{"id":3679,"name":"CascadedShadows_natural","record":{"opcode":"OpTypeStruct","operands":[165]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_shadow_map_dims","type_id":165,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":26,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[26]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3701,"name":"csm_setup.sun_view","storage_class":"Uniform","type":{"id":3700,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,3698]},"decorations":[],"pointee":{"id":3698,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[3699]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":3699,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":25,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[25]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3734,"name":"csm_setup.cascade_splits_buf","storage_class":"Uniform","type":{"id":3733,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,3731]},"decorations":[],"pointee":{"id":3731,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[3732]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":3732,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":25,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[25]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3924,"name":"csm.sun_shadow_map","storage_class":"UniformConstant","type":{"id":3923,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,3921]},"decorations":[],"pointee":{"id":3921,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":26,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[26]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3943,"name":"csm.sun_shadow_stencil_map","storage_class":"UniformConstant","type":{"id":3942,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,3940]},"decorations":[],"pointee":{"id":3940,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[136,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":26,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[26]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4071,"name":"samplers.shadow_linear","storage_class":"UniformConstant","type":{"id":2196,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2194]},"decorations":[],"pointee":{"id":2194,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[11]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4690,"name":"rt_scene.rt_triangle_meta","storage_class":"Uniform","type":{"id":4689,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4687]},"decorations":[],"pointee":{"id":4687,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[4688]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":4688,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":24,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[24]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4698,"name":"wall_color_data.wall_color_ids","storage_class":"Uniform","type":{"id":4689,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4687]},"decorations":[],"pointee":{"id":4687,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[4688]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":4688,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":28,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[28]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4782,"name":"roof_colors.ids","storage_class":"Uniform","type":{"id":4689,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4687]},"decorations":[],"pointee":{"id":4687,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[4688]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":4688,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":29,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[29]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5005,"name":"samplers.nnc","storage_class":"UniformConstant","type":{"id":2196,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2194]},"decorations":[],"pointee":{"id":2194,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5007,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":494,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,492]},"decorations":[],"pointee":{"id":492,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5036,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":494,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,492]},"decorations":[],"pointee":{"id":492,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5075,"name":"taa_history_tex","storage_class":"UniformConstant","type":{"id":494,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,492]},"decorations":[],"pointee":{"id":492,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5170,"name":"globalParams.reflection_fog.froxel_scattering_tex","storage_class":"UniformConstant","type":{"id":5169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5167]},"decorations":[],"pointee":{"id":5167,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[10]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5189,"name":"globalParams.reflection_fog.froxel_extinction_tex","storage_class":"UniformConstant","type":{"id":5169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5167]},"decorations":[],"pointee":{"id":5167,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[11]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5525,"name":"samplers.llr","storage_class":"UniformConstant","type":{"id":2196,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,2194]},"decorations":[],"pointee":{"id":2194,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5527,"name":"fog_noise_texture.tex","storage_class":"UniformConstant","type":{"id":5169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5167]},"decorations":[],"pointee":{"id":5167,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":31,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[31]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5653,"name":"globalParams.reflection_fog.volumetric_shadows.volumetric_shadow_rendering_views","storage_class":"Uniform","type":{"id":5652,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5650]},"decorations":[],"pointee":{"id":5650,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[5651]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":5651,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":8,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[8]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5718,"name":"globalParams.reflection_fog.volumetric_shadows.volumetric_shadow_map_tex","storage_class":"UniformConstant","type":{"id":3923,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,3921]},"decorations":[],"pointee":{"id":3921,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":7,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[7]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5851,"name":"globalParams.reflection_fog.sky_visibility_volume_tex","storage_class":"UniformConstant","type":{"id":5169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5167]},"decorations":[],"pointee":{"id":5167,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[9]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6212,"name":"color_tex","storage_class":"UniformConstant","type":{"id":6211,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6209]},"decorations":[],"pointee":{"id":6209,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":6,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[6]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":149,"name":"unknown","storage_class":"Input","type":{"id":148,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,146]},"decorations":[],"pointee":{"id":146,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[136,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/water/ice_refr_resolve.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[49]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":55,"name":"refr_hit_tex","storage_class":"UniformConstant","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[45,1,2,0,0,1,33]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":138,"name":"globalParams","storage_class":"Uniform","type":{"id":137,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,136]},"decorations":[],"pointee":{"id":136,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[50,50,50,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"refr_hit_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"refr_hit_radiance_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"refl_refr_radiance_out_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":160,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":159,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,157]},"decorations":[],"pointee":{"id":157,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":190,"name":"current_view","storage_class":"Uniform","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,187]},"decorations":[],"pointee":{"id":187,"name":"ViewConstants_natural","record":{"opcode":"OpTypeStruct","operands":[188,188,188,188,188,188,188,188,188,188,188,188,188,73,73,73,73,188,188,7,6,6,6,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"view_to_clip","type_id":188,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":1,"name":"clip_to_view","type_id":188,"decorations":[{"decoration":"Offset","operands":[64]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":2,"name":"view_to_sample","type_id":188,"decorations":[{"decoration":"Offset","operands":[128]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":3,"name":"sample_to_view","type_id":188,"decorations":[{"decoration":"Offset","operands":[192]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":4,"name":"world_to_view","type_id":188,"decorations":[{"decoration":"Offset","operands":[256]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":5,"name":"view_to_world","type_id":188,"decorations":[{"decoration":"Offset","operands":[320]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":6,"name":"world_to_sample","type_id":188,"decorations":[{"decoration":"Offset","operands":[384]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":7,"name":"clip_to_prev_clip","type_id":188,"decorations":[{"decoration":"Offset","operands":[448]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":8,"name":"prev_clip_to_clip","type_id":188,"decorations":[{"decoration":"Offset","operands":[512]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":9,"name":"prev_view_to_prev_clip","type_id":188,"decorations":[{"decoration":"Offset","operands":[576]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":10,"name":"prev_clip_to_prev_view","type_id":188,"decorations":[{"decoration":"Offset","operands":[640]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":11,"name":"prev_world_to_prev_view","type_id":188,"decorations":[{"decoration":"Offset","operands":[704]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":12,"name":"prev_view_to_prev_world","type_id":188,"decorations":[{"decoration":"Offset","operands":[768]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":13,"name":"sample_offset_pixels","type_id":73,"decorations":[{"decoration":"Offset","operands":[832]}]},{"index":14,"name":"sample_offset_clip","type_id":73,"decorations":[{"decoration":"Offset","operands":[840]}]},{"index":15,"name":"prev_sample_offset_pixels","type_id":73,"decorations":[{"decoration":"Offset","operands":[848]}]},{"index":16,"name":"prev_sample_offset_clip","type_id":73,"decorations":[{"decoration":"Offset","operands":[856]}]},{"index":17,"name":"pad0","type_id":188,"decorations":[{"decoration":"Offset","operands":[864]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":18,"name":"pad1","type_id":188,"decorations":[{"decoration":"Offset","operands":[928]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":19,"name":"pad2","type_id":7,"decorations":[{"decoration":"Offset","operands":[992]}]},{"index":20,"name":"pad3","type_id":6,"decorations":[{"decoration":"Offset","operands":[1008]}]},{"index":21,"name":"pad4","type_id":6,"decorations":[{"decoration":"Offset","operands":[1012]}]},{"index":22,"name":"pad5","type_id":6,"decorations":[{"decoration":"Offset","operands":[1016]}]},{"index":23,"name":"mip_bias","type_id":6,"decorations":[{"decoration":"Offset","operands":[1020]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":460,"name":"frame_counter","storage_class":"Uniform","type":{"id":459,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,458]},"decorations":[],"pointee":{"id":458,"name":"FrameCounter_natural","record":{"opcode":"OpTypeStruct","operands":[45]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"index","type_id":45,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":565,"name":"refr_hit_radiance_tex","storage_class":"UniformConstant","type":{"id":159,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,157]},"decorations":[],"pointee":{"id":157,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":623,"name":"refl_refr_radiance_out_tex","storage_class":"UniformConstant","type":{"id":622,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,620]},"decorations":[],"pointee":{"id":620,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":49,"name":"unknown","storage_class":"Input","type":{"id":48,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,46]},"decorations":[],"pointee":{"id":46,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[45,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/water/ice_refl_resolve.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[67]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":73,"name":"ice_roughness_tex","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":94,"name":"refl_refr_radiance_out_tex","storage_class":"UniformConstant","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,91]},"decorations":[],"pointee":{"id":91,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":7,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[7]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":113,"name":"globalParams","storage_class":"Uniform","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,111]},"decorations":[],"pointee":{"id":111,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[63,68,68,68,68,68,68,68,68,68,68]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"history_valid","type_id":63,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"depth_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"ice_roughness_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"refl_hit_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":4,"name":"refl_hit_radiance_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[28]}]},{"index":5,"name":"color_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[36]}]},{"index":6,"name":"refl_refr_radiance_history_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[44]}]},{"index":7,"name":"refl_refr_radiance_out_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[52]}]},{"index":8,"name":"ray_len_history_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[60]}]},{"index":9,"name":"ray_len_out_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[68]}]},{"index":10,"name":"reprojection_tex_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[76]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":135,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":143,"name":"frame_counter","storage_class":"Uniform","type":{"id":142,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,141]},"decorations":[],"pointee":{"id":141,"name":"FrameCounter_natural","record":{"opcode":"OpTypeStruct","operands":[63]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"index","type_id":63,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":3,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[3]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":179,"name":"blue_noise_textures.uniform01","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":5,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[5]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":217,"name":"current_view","storage_class":"Uniform","type":{"id":216,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,214]},"decorations":[],"pointee":{"id":214,"name":"ViewConstants_natural","record":{"opcode":"OpTypeStruct","operands":[215,215,215,215,215,215,215,215,215,215,215,215,215,118,118,118,118,215,215,7,6,6,6,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"view_to_clip","type_id":215,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":1,"name":"clip_to_view","type_id":215,"decorations":[{"decoration":"Offset","operands":[64]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":2,"name":"view_to_sample","type_id":215,"decorations":[{"decoration":"Offset","operands":[128]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":3,"name":"sample_to_view","type_id":215,"decorations":[{"decoration":"Offset","operands":[192]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":4,"name":"world_to_view","type_id":215,"decorations":[{"decoration":"Offset","operands":[256]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":5,"name":"view_to_world","type_id":215,"decorations":[{"decoration":"Offset","operands":[320]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":6,"name":"world_to_sample","type_id":215,"decorations":[{"decoration":"Offset","operands":[384]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":7,"name":"clip_to_prev_clip","type_id":215,"decorations":[{"decoration":"Offset","operands":[448]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":8,"name":"prev_clip_to_clip","type_id":215,"decorations":[{"decoration":"Offset","operands":[512]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":9,"name":"prev_view_to_prev_clip","type_id":215,"decorations":[{"decoration":"Offset","operands":[576]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":10,"name":"prev_clip_to_prev_view","type_id":215,"decorations":[{"decoration":"Offset","operands":[640]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":11,"name":"prev_world_to_prev_view","type_id":215,"decorations":[{"decoration":"Offset","operands":[704]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":12,"name":"prev_view_to_prev_world","type_id":215,"decorations":[{"decoration":"Offset","operands":[768]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":13,"name":"sample_offset_pixels","type_id":118,"decorations":[{"decoration":"Offset","operands":[832]}]},{"index":14,"name":"sample_offset_clip","type_id":118,"decorations":[{"decoration":"Offset","operands":[840]}]},{"index":15,"name":"prev_sample_offset_pixels","type_id":118,"decorations":[{"decoration":"Offset","operands":[848]}]},{"index":16,"name":"prev_sample_offset_clip","type_id":118,"decorations":[{"decoration":"Offset","operands":[856]}]},{"index":17,"name":"pad0","type_id":215,"decorations":[{"decoration":"Offset","operands":[864]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":18,"name":"pad1","type_id":215,"decorations":[{"decoration":"Offset","operands":[928]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":19,"name":"pad2","type_id":7,"decorations":[{"decoration":"Offset","operands":[992]}]},{"index":20,"name":"pad3","type_id":6,"decorations":[{"decoration":"Offset","operands":[1008]}]},{"index":21,"name":"pad4","type_id":6,"decorations":[{"decoration":"Offset","operands":[1012]}]},{"index":22,"name":"pad5","type_id":6,"decorations":[{"decoration":"Offset","operands":[1016]}]},{"index":23,"name":"mip_bias","type_id":6,"decorations":[{"decoration":"Offset","operands":[1020]}]}]}},"descriptor_set":7,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[7]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":280,"name":"refl_hit_tex","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":503,"name":"samplers.lnc","storage_class":"UniformConstant","type":{"id":502,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,500]},"decorations":[],"pointee":{"id":500,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":505,"name":"ray_len_history_tex","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":8,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[8]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":704,"name":"refl_hit_radiance_tex","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":824,"name":"refl_refr_radiance_history_tex","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":6,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[6]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":948,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":502,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,500]},"decorations":[],"pointee":{"id":500,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":952,"name":"froxel_scattering_tex","storage_class":"UniformConstant","type":{"id":951,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,949]},"decorations":[],"pointee":{"id":949,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[11]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":965,"name":"frame_constants","storage_class":"Uniform","type":{"id":964,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,963]},"decorations":[],"pointee":{"id":963,"name":"FrameConstants_natural","record":{"opcode":"OpTypeStruct","operands":[7,7,7,7,7,7,7,7,6,6,6,63,68,63,6,10,6,10,63,10,63,10,6,7,6,63,68,6,63,63,63,10,6,63,63,63,63,63,6,6,6,7,10,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":7,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":7,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":7,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":7,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"time_since_startup_secs","type_id":6,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":6,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":63,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"world_render_resolution","type_id":68,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":63,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":6,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":10,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":6,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":10,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":63,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":10,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":63,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"dir_to_primary_directional_light","type_id":10,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"main_view_lod_scale","type_id":6,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":23,"name":"main_view_eye_position","type_id":7,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":24,"name":"fog_relative_extinction","type_id":6,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":25,"name":"highlighted_transient_stair_edge_id","type_id":63,"decorations":[{"decoration":"Offset","operands":[244]}]},{"index":26,"name":"swapchain_dims","type_id":68,"decorations":[{"decoration":"Offset","operands":[248]}]},{"index":27,"name":"swapchain_aspect_ratio","type_id":6,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":28,"name":"is_photomode","type_id":63,"decorations":[{"decoration":"Offset","operands":[260]}]},{"index":29,"name":"is_topdown_mode","type_id":63,"decorations":[{"decoration":"Offset","operands":[264]}]},{"index":30,"name":"is_firstperson_mode","type_id":63,"decorations":[{"decoration":"Offset","operands":[268]}]},{"index":31,"name":"brush_position","type_id":10,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":32,"name":"eraser_size_multiplier","type_id":6,"decorations":[{"decoration":"Offset","operands":[284]}]},{"index":33,"name":"roof_highlighted_slot_id","type_id":63,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":34,"name":"roof_highlight_mode","type_id":63,"decorations":[{"decoration":"Offset","operands":[292]}]},{"index":35,"name":"wall_highlighted_slot_id","type_id":63,"decorations":[{"decoration":"Offset","operands":[296]}]},{"index":36,"name":"wall_highlight_mode","type_id":63,"decorations":[{"decoration":"Offset","operands":[300]}]},{"index":37,"name":"highlight_bricks_to_delete","type_id":63,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":38,"name":"wind_strength","type_id":6,"decorations":[{"decoration":"Offset","operands":[308]}]},{"index":39,"name":"tree_canopy_cone_angle_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[312]}]},{"index":40,"name":"tree_canopy_max_cone_cull_dist_range","type_id":6,"decorations":[{"decoration":"Offset","operands":[316]}]},{"index":41,"name":"debug_tweakers","type_id":7,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":42,"name":"ground_fog","type_id":10,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":43,"name":"moonswap","type_id":6,"decorations":[{"decoration":"Offset","operands":[348]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":975,"name":"froxel_extinction_tex","storage_class":"UniformConstant","type":{"id":951,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,949]},"decorations":[],"pointee":{"id":949,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[12]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":989,"name":"color_tex","storage_class":"UniformConstant","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,91]},"decorations":[],"pointee":{"id":91,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1011,"name":"ray_len_out_tex","storage_class":"UniformConstant","type":{"id":1010,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1008]},"decorations":[],"pointee":{"id":1008,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[9]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":67,"name":"unknown","storage_class":"Input","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[63,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5688164,"position_among_mapped_render_targets":63,"mapped_render_target_count":74,"previous_callsite":5688084,"next_callsite":5688200,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/deferred_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5690999,"position_among_mapped_render_targets":74,"mapped_render_target_count":74,"previous_callsite":5689962,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: transparency_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<country_core::systems::render_loop::find_crash::State>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<country_core::systems::render_loop::find_crash::State>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::render_loop::find_crash::State>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass
// 信息 ID: b413ac6dd3e014cda93aabd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure$0
// 信息 ID: 2c182cdc47544bd804286fc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0>; module=1655, inline_record=155252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass
// 信息 ID: 1c27d2cd5fd5d83ada4eede8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56d677 -> 0xaeb000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass::closure$0
// 信息 ID: c243aca6e8678ffc24a2879f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass::closure_env$0>; module=1655, inline_record=134976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass
// 信息 ID: b78e2b221af8b76d39179fe5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56cb64 -> 0xaeae90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure$0
// 信息 ID: c5d1ad89a4d09546e5586491
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0>; module=1655, inline_record=123560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass
// 信息 ID: 254b5c55480cbea658800c72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0
// 信息 ID: d8675454ad41a1183a9c63f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass; module=556, inline_record=45716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure$0
// 信息 ID: b2ceddb6b8266d6371e55394
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0>; module=1655, inline_record=103464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
