// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/render_loop.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4b5f8c62ff6a7e1ac8fdba529a487da5e71778f478e25577903fde7c682a827c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: render_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 107, 112, 143, 202
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.6"}]
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
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rhapsody_dof; // namespace candidate
//   package identities: [{"name":"rhapsody-dof","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rhapsody_gi; // namespace candidate
//   package identities: [{"name":"rhapsody-gi","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rhapsody_shadow; // namespace candidate
//   package identities: [{"name":"rhapsody-shadow","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rhapsody_taa; // namespace candidate
//   package identities: [{"name":"rhapsody-taa","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use typed_index_collections; // namespace candidate
//   package identities: [{"name":"typed-index-collections","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.0"}]
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F3C
// 首条位置: {"module":202,"inlinee":16188}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3040
// 首条位置: {"module":107,"inlinee":12352}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":48392,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":40516,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":46596,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":107,"record_offset":64172,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":107,"record_offset":64088,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":63976,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":64792,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":46152,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":64144,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":61348,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":70336,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":107,"record_offset":61276,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":107,"record_offset":46416,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":107,"record_offset":46296,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":107,"record_offset":43620,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$9::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":107,"record_offset":46512,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12186,"source_line":407,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":107,"record_offset":38808,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":107,"record_offset":48448,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":107,"record_offset":64004,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::depth_stencil_attachment
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":107,"record_offset":61248,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12366,"source_line":532,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::with_binding_set
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":107,"record_offset":52020,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12362,"source_line":448,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphResourceApi::import
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":107,"record_offset":43648,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12355,"source_line":79,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":107,"record_offset":56356,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::handle_screenshot_error
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":107,"record_offset":64436,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12374,"source_line":1083,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::next_requested_screenshot_stage
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":107,"record_offset":61592,"parent_record_offset":37976,"ancestor_record_offset":37976,"inlinee":12372,"source_line":505,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":70536,"parent_record_offset":70512,"ancestor_record_offset":37976,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":70560,"parent_record_offset":70536,"ancestor_record_offset":37976,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":70584,"parent_record_offset":70560,"ancestor_record_offset":37976,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":70512,"parent_record_offset":70456,"ancestor_record_offset":37976,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":40160,"parent_record_offset":40136,"ancestor_record_offset":37976,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":52240,"parent_record_offset":52216,"ancestor_record_offset":37976,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":70484,"parent_record_offset":70456,"ancestor_record_offset":37976,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":70456,"parent_record_offset":70432,"ancestor_record_offset":37976,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":40208,"parent_record_offset":40184,"ancestor_record_offset":37976,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":40184,"parent_record_offset":40160,"ancestor_record_offset":37976,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":52264,"parent_record_offset":52240,"ancestor_record_offset":37976,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":70432,"parent_record_offset":70408,"ancestor_record_offset":37976,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":52300,"parent_record_offset":52180,"ancestor_record_offset":37976,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":40136,"parent_record_offset":40112,"ancestor_record_offset":37976,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":40112,"parent_record_offset":40072,"ancestor_record_offset":37976,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":52180,"parent_record_offset":52156,"ancestor_record_offset":37976,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":52104,"parent_record_offset":52076,"ancestor_record_offset":37976,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":52216,"parent_record_offset":52180,"ancestor_record_offset":37976,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":52420,"parent_record_offset":52180,"ancestor_record_offset":37976,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":56288,"parent_record_offset":56264,"ancestor_record_offset":37976,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":52132,"parent_record_offset":52076,"ancestor_record_offset":37976,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":40072,"parent_record_offset":40048,"ancestor_record_offset":37976,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":107,"record_offset":52156,"parent_record_offset":52132,"ancestor_record_offset":37976,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":48416,"parent_record_offset":48392,"ancestor_record_offset":37976,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":54284,"parent_record_offset":52464,"ancestor_record_offset":37976,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":107,"record_offset":38868,"parent_record_offset":38840,"ancestor_record_offset":37976,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":40432,"parent_record_offset":40408,"ancestor_record_offset":37976,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":40408,"parent_record_offset":40384,"ancestor_record_offset":37976,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":40356,"parent_record_offset":40324,"ancestor_record_offset":37976,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":40300,"parent_record_offset":39792,"ancestor_record_offset":37976,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":48068,"parent_record_offset":47556,"ancestor_record_offset":37976,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":40324,"parent_record_offset":40300,"ancestor_record_offset":37976,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":39820,"parent_record_offset":39792,"ancestor_record_offset":37976,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":40384,"parent_record_offset":40324,"ancestor_record_offset":37976,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":48152,"parent_record_offset":48092,"ancestor_record_offset":37976,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":39868,"parent_record_offset":39844,"ancestor_record_offset":37976,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":39844,"parent_record_offset":39820,"ancestor_record_offset":37976,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":40048,"parent_record_offset":40024,"ancestor_record_offset":37976,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":107,"record_offset":48268,"parent_record_offset":48244,"ancestor_record_offset":37976,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":48244,"parent_record_offset":47556,"ancestor_record_offset":37976,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":38840,"parent_record_offset":38808,"ancestor_record_offset":37976,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":39792,"parent_record_offset":38840,"ancestor_record_offset":37976,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":48484,"parent_record_offset":48448,"ancestor_record_offset":37976,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":47556,"parent_record_offset":46596,"ancestor_record_offset":37976,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":107,"record_offset":38892,"parent_record_offset":38868,"ancestor_record_offset":37976,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":107,"record_offset":39540,"parent_record_offset":39516,"ancestor_record_offset":37976,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":107,"record_offset":39568,"parent_record_offset":39540,"ancestor_record_offset":37976,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":107,"record_offset":64196,"parent_record_offset":64172,"ancestor_record_offset":37976,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":107,"record_offset":64312,"parent_record_offset":64220,"ancestor_record_offset":37976,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":107,"record_offset":64564,"parent_record_offset":64436,"ancestor_record_offset":37976,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":107,"record_offset":39108,"parent_record_offset":39084,"ancestor_record_offset":37976,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":107,"record_offset":39192,"parent_record_offset":39168,"ancestor_record_offset":37976,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":107,"record_offset":64336,"parent_record_offset":64312,"ancestor_record_offset":37976,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":107,"record_offset":64816,"parent_record_offset":64792,"ancestor_record_offset":37976,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":107,"record_offset":64280,"parent_record_offset":64256,"ancestor_record_offset":37976,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":107,"record_offset":39340,"parent_record_offset":39312,"ancestor_record_offset":37976,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":107,"record_offset":39312,"parent_record_offset":39284,"ancestor_record_offset":37976,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":107,"record_offset":39960,"parent_record_offset":39896,"ancestor_record_offset":37976,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":107,"record_offset":64256,"parent_record_offset":64220,"ancestor_record_offset":37976,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":107,"record_offset":64220,"parent_record_offset":64196,"ancestor_record_offset":37976,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":39984,"parent_record_offset":39960,"ancestor_record_offset":37976,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":64112,"parent_record_offset":64088,"ancestor_record_offset":37976,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":40024,"parent_record_offset":39844,"ancestor_record_offset":37976,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":40484,"parent_record_offset":38808,"ancestor_record_offset":37976,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":107,"record_offset":64360,"parent_record_offset":64336,"ancestor_record_offset":37976,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":39132,"parent_record_offset":39108,"ancestor_record_offset":37976,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":61316,"parent_record_offset":61276,"ancestor_record_offset":37976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":39456,"parent_record_offset":39432,"ancestor_record_offset":37976,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":107,"record_offset":61864,"parent_record_offset":61804,"ancestor_record_offset":37976,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":107,"record_offset":61836,"parent_record_offset":61804,"ancestor_record_offset":37976,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":64384,"parent_record_offset":64360,"ancestor_record_offset":37976,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":107,"record_offset":39896,"parent_record_offset":39868,"ancestor_record_offset":37976,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":107,"record_offset":39932,"parent_record_offset":39896,"ancestor_record_offset":37976,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":107,"record_offset":40252,"parent_record_offset":40072,"ancestor_record_offset":37976,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":107,"record_offset":61804,"parent_record_offset":61592,"ancestor_record_offset":37976,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":48292,"parent_record_offset":48268,"ancestor_record_offset":37976,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":62000,"parent_record_offset":61944,"ancestor_record_offset":37976,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":61712,"parent_record_offset":61676,"ancestor_record_offset":37976,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":46536,"parent_record_offset":46512,"ancestor_record_offset":37976,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":62024,"parent_record_offset":62000,"ancestor_record_offset":37976,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":61736,"parent_record_offset":61712,"ancestor_record_offset":37976,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":46560,"parent_record_offset":46536,"ancestor_record_offset":37976,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":107,"record_offset":46440,"parent_record_offset":46416,"ancestor_record_offset":37976,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":107,"record_offset":46328,"parent_record_offset":46296,"ancestor_record_offset":37976,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":107,"record_offset":46384,"parent_record_offset":46296,"ancestor_record_offset":37976,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":107,"record_offset":54340,"parent_record_offset":52464,"ancestor_record_offset":37976,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":107,"record_offset":39516,"parent_record_offset":39492,"ancestor_record_offset":37976,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":39252,"parent_record_offset":39224,"ancestor_record_offset":37976,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":39284,"parent_record_offset":39252,"ancestor_record_offset":37976,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":39224,"parent_record_offset":39004,"ancestor_record_offset":37976,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":39084,"parent_record_offset":39004,"ancestor_record_offset":37976,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":39644,"parent_record_offset":39004,"ancestor_record_offset":37976,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":39168,"parent_record_offset":39004,"ancestor_record_offset":37976,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":107,"record_offset":38928,"parent_record_offset":38892,"ancestor_record_offset":37976,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":107,"record_offset":39492,"parent_record_offset":39384,"ancestor_record_offset":37976,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":39432,"parent_record_offset":39408,"ancestor_record_offset":37976,"inlinee":4324,"source_line":261,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":39736,"parent_record_offset":39004,"ancestor_record_offset":37976,"inlinee":4339,"source_line":81,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":39408,"parent_record_offset":39384,"ancestor_record_offset":37976,"inlinee":4322,"source_line":734,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":38980,"parent_record_offset":38956,"ancestor_record_offset":37976,"inlinee":4302,"source_line":1156,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":38956,"parent_record_offset":38928,"ancestor_record_offset":37976,"inlinee":4301,"source_line":1179,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":39004,"parent_record_offset":38980,"ancestor_record_offset":37976,"inlinee":4304,"source_line":1862,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":39056,"parent_record_offset":39004,"ancestor_record_offset":37976,"inlinee":4305,"source_line":2301,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":39384,"parent_record_offset":39004,"ancestor_record_offset":37976,"inlinee":4321,"source_line":1166,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":107,"record_offset":39616,"parent_record_offset":39004,"ancestor_record_offset":37976,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":107,"record_offset":61648,"parent_record_offset":61592,"ancestor_record_offset":37976,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":107,"record_offset":61920,"parent_record_offset":61896,"ancestor_record_offset":37976,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":107,"record_offset":64668,"parent_record_offset":64644,"ancestor_record_offset":37976,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":107,"record_offset":64620,"parent_record_offset":64436,"ancestor_record_offset":37976,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":107,"record_offset":64644,"parent_record_offset":64620,"ancestor_record_offset":37976,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":107,"record_offset":64692,"parent_record_offset":64668,"ancestor_record_offset":37976,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":107,"record_offset":64480,"parent_record_offset":64436,"ancestor_record_offset":37976,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":107,"record_offset":43356,"parent_record_offset":43332,"ancestor_record_offset":37976,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":107,"record_offset":61676,"parent_record_offset":61648,"ancestor_record_offset":37976,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":107,"record_offset":61944,"parent_record_offset":61920,"ancestor_record_offset":37976,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":107,"record_offset":61768,"parent_record_offset":61676,"ancestor_record_offset":37976,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":107,"record_offset":61972,"parent_record_offset":61944,"ancestor_record_offset":37976,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::push_binding_sets
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":107,"record_offset":52076,"parent_record_offset":52020,"ancestor_record_offset":37976,"inlinee":12363,"source_line":473,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::trim_binding_set_stack
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":107,"record_offset":56264,"parent_record_offset":52020,"ancestor_record_offset":37976,"inlinee":12365,"source_line":496,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$0::into_graph_borrow
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":107,"record_offset":43700,"parent_record_offset":43672,"ancestor_record_offset":37976,"inlinee":8323,"source_line":186,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::import_with_access_type
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":107,"record_offset":43672,"parent_record_offset":43648,"ancestor_record_offset":37976,"inlinee":12356,"source_line":1308,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":107,"record_offset":61432,"parent_record_offset":61408,"ancestor_record_offset":37976,"inlinee":12369,"source_line":287,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::new_2d
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":107,"record_offset":61408,"parent_record_offset":61380,"ancestor_record_offset":37976,"inlinee":12368,"source_line":305,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":107,"record_offset":46356,"parent_record_offset":46296,"ancestor_record_offset":37976,"inlinee":12360,"source_line":274,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":107,"record_offset":52464,"parent_record_offset":52020,"ancestor_record_offset":37976,"inlinee":12364,"source_line":324,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":107,"record_offset":61380,"parent_record_offset":61348,"ancestor_record_offset":37976,"inlinee":12367,"source_line":448,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":107,"record_offset":61504,"parent_record_offset":61472,"ancestor_record_offset":37976,"inlinee":12370,"source_line":456,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 274
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3048
// 首条位置: {"module":107,"inlinee":12360}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 324
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x304C
// 首条位置: {"module":107,"inlinee":12364}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":54284,"parent_record_offset":52464,"ancestor_record_offset":52464,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":52516,"parent_record_offset":52464,"ancestor_record_offset":52464,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":56232,"parent_record_offset":52464,"ancestor_record_offset":52464,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":107,"record_offset":54340,"parent_record_offset":52464,"ancestor_record_offset":52464,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":53848,"parent_record_offset":53824,"ancestor_record_offset":52464,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":53896,"parent_record_offset":53872,"ancestor_record_offset":52464,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":53872,"parent_record_offset":53848,"ancestor_record_offset":52464,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":53824,"parent_record_offset":53800,"ancestor_record_offset":52464,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":53800,"parent_record_offset":53760,"ancestor_record_offset":52464,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":53760,"parent_record_offset":53736,"ancestor_record_offset":52464,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":54308,"parent_record_offset":54284,"ancestor_record_offset":52464,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":107,"record_offset":52548,"parent_record_offset":52516,"ancestor_record_offset":52464,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":54120,"parent_record_offset":54096,"ancestor_record_offset":52464,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":54096,"parent_record_offset":54072,"ancestor_record_offset":52464,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":54044,"parent_record_offset":54012,"ancestor_record_offset":52464,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":53988,"parent_record_offset":53476,"ancestor_record_offset":52464,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":54012,"parent_record_offset":53988,"ancestor_record_offset":52464,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":53508,"parent_record_offset":53476,"ancestor_record_offset":52464,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":54072,"parent_record_offset":54012,"ancestor_record_offset":52464,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":53556,"parent_record_offset":53532,"ancestor_record_offset":52464,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":53532,"parent_record_offset":53508,"ancestor_record_offset":52464,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":53736,"parent_record_offset":53712,"ancestor_record_offset":52464,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":107,"record_offset":54188,"parent_record_offset":54164,"ancestor_record_offset":52464,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":54164,"parent_record_offset":53476,"ancestor_record_offset":52464,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":54376,"parent_record_offset":54340,"ancestor_record_offset":52464,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":53476,"parent_record_offset":52516,"ancestor_record_offset":52464,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":107,"record_offset":52576,"parent_record_offset":52548,"ancestor_record_offset":52464,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":107,"record_offset":53224,"parent_record_offset":53200,"ancestor_record_offset":52464,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":107,"record_offset":53252,"parent_record_offset":53224,"ancestor_record_offset":52464,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":107,"record_offset":52792,"parent_record_offset":52768,"ancestor_record_offset":52464,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":107,"record_offset":52876,"parent_record_offset":52852,"ancestor_record_offset":52464,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":107,"record_offset":53024,"parent_record_offset":52996,"ancestor_record_offset":52464,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":107,"record_offset":52996,"parent_record_offset":52968,"ancestor_record_offset":52464,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":107,"record_offset":53648,"parent_record_offset":53584,"ancestor_record_offset":52464,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":53672,"parent_record_offset":53648,"ancestor_record_offset":52464,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":53712,"parent_record_offset":53532,"ancestor_record_offset":52464,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":56144,"parent_record_offset":54340,"ancestor_record_offset":52464,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":52816,"parent_record_offset":52792,"ancestor_record_offset":52464,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":53140,"parent_record_offset":53116,"ancestor_record_offset":52464,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":107,"record_offset":53584,"parent_record_offset":53556,"ancestor_record_offset":52464,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":107,"record_offset":53620,"parent_record_offset":53584,"ancestor_record_offset":52464,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":107,"record_offset":53940,"parent_record_offset":53760,"ancestor_record_offset":52464,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":54212,"parent_record_offset":54188,"ancestor_record_offset":52464,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":54236,"parent_record_offset":54212,"ancestor_record_offset":52464,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":107,"record_offset":53200,"parent_record_offset":53176,"ancestor_record_offset":52464,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":52936,"parent_record_offset":52908,"ancestor_record_offset":52464,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":52968,"parent_record_offset":52936,"ancestor_record_offset":52464,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":52908,"parent_record_offset":52688,"ancestor_record_offset":52464,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":52768,"parent_record_offset":52688,"ancestor_record_offset":52464,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":53328,"parent_record_offset":52688,"ancestor_record_offset":52464,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":52852,"parent_record_offset":52688,"ancestor_record_offset":52464,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":107,"record_offset":52612,"parent_record_offset":52576,"ancestor_record_offset":52464,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":107,"record_offset":53176,"parent_record_offset":53068,"ancestor_record_offset":52464,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":53116,"parent_record_offset":53092,"ancestor_record_offset":52464,"inlinee":4324,"source_line":261,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":53420,"parent_record_offset":52688,"ancestor_record_offset":52464,"inlinee":4339,"source_line":81,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":53092,"parent_record_offset":53068,"ancestor_record_offset":52464,"inlinee":4322,"source_line":734,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":52664,"parent_record_offset":52640,"ancestor_record_offset":52464,"inlinee":4302,"source_line":1156,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":52640,"parent_record_offset":52612,"ancestor_record_offset":52464,"inlinee":4301,"source_line":1179,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":52688,"parent_record_offset":52664,"ancestor_record_offset":52464,"inlinee":4304,"source_line":1862,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":52740,"parent_record_offset":52688,"ancestor_record_offset":52464,"inlinee":4305,"source_line":2301,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":53068,"parent_record_offset":52688,"ancestor_record_offset":52464,"inlinee":4321,"source_line":1166,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":107,"record_offset":53300,"parent_record_offset":52688,"ancestor_record_offset":52464,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0::explain
// 记录形式: compiled_procedure; instances=1
// 源码行: 361
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":202,"record_offset":36136,"rva":5692560,"va":5374401680}
// 编译实例 RVAs: 0x56dc90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10207503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401680,"raw_hex":"e80fc19b00","size":5,"soft_fail":false,"target_va":5384609408,"va":5374401900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":202,"record_offset":36384,"parent_record_offset":36136,"ancestor_record_offset":36136,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":202,"record_offset":36440,"parent_record_offset":36136,"ancestor_record_offset":36136,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":202,"record_offset":36300,"parent_record_offset":36136,"ancestor_record_offset":36136,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":36408,"parent_record_offset":36384,"ancestor_record_offset":36136,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":36324,"parent_record_offset":36300,"ancestor_record_offset":36136,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":36348,"parent_record_offset":36324,"ancestor_record_offset":36136,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":202,"record_offset":36488,"parent_record_offset":36464,"ancestor_record_offset":36136,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":202,"record_offset":36464,"parent_record_offset":36440,"ancestor_record_offset":36136,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":202,"record_offset":36512,"parent_record_offset":36488,"ancestor_record_offset":36136,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 448
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x304F
// 首条位置: {"module":107,"inlinee":12367}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::new_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":107,"record_offset":61408,"parent_record_offset":61380,"ancestor_record_offset":61380,"inlinee":12368,"source_line":305,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":107,"record_offset":61432,"parent_record_offset":61408,"ancestor_record_offset":61380,"inlinee":12369,"source_line":287,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 456
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3052
// 首条位置: {"module":107,"inlinee":12370}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::new_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":107,"record_offset":61528,"parent_record_offset":61504,"ancestor_record_offset":61504,"inlinee":12368,"source_line":305,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":107,"record_offset":61552,"parent_record_offset":61528,"ancestor_record_offset":61504,"inlinee":12369,"source_line":287,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 664
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x369B
// 首条位置: {"module":143,"inlinee":13979}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_loading_screen_render_graph
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 707
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F3D
// 首条位置: {"module":202,"inlinee":16189}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 716
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x301F
// 首条位置: {"module":107,"inlinee":12319}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":19464,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":7012,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":17896,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":107,"record_offset":17868,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":19520,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":8656,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":16088,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":107,"record_offset":16160,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":107,"record_offset":16116,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":10876,"parent_record_offset":10852,"ancestor_record_offset":6832,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":10900,"parent_record_offset":10876,"ancestor_record_offset":6832,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":10924,"parent_record_offset":10900,"ancestor_record_offset":6832,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":10852,"parent_record_offset":10796,"ancestor_record_offset":6832,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":6832,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":10824,"parent_record_offset":10796,"ancestor_record_offset":6832,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":10796,"parent_record_offset":10772,"ancestor_record_offset":6832,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":8380,"parent_record_offset":8356,"ancestor_record_offset":6832,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":6832,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":10772,"parent_record_offset":10748,"ancestor_record_offset":6832,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":6832,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":8284,"parent_record_offset":8244,"ancestor_record_offset":6832,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":8244,"parent_record_offset":8220,"ancestor_record_offset":6832,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":14820,"parent_record_offset":14796,"ancestor_record_offset":6832,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":14796,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":107,"record_offset":7040,"parent_record_offset":7012,"ancestor_record_offset":6832,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":8604,"parent_record_offset":8580,"ancestor_record_offset":6832,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":6832,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":8528,"parent_record_offset":8496,"ancestor_record_offset":6832,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":8472,"parent_record_offset":7964,"ancestor_record_offset":6832,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":14500,"parent_record_offset":13988,"ancestor_record_offset":6832,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":6832,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":7992,"parent_record_offset":7964,"ancestor_record_offset":6832,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":8556,"parent_record_offset":8496,"ancestor_record_offset":6832,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":14584,"parent_record_offset":14524,"ancestor_record_offset":6832,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":8040,"parent_record_offset":8016,"ancestor_record_offset":6832,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":8016,"parent_record_offset":7992,"ancestor_record_offset":6832,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":8220,"parent_record_offset":8196,"ancestor_record_offset":6832,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":107,"record_offset":14700,"parent_record_offset":14676,"ancestor_record_offset":6832,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":14676,"parent_record_offset":13988,"ancestor_record_offset":6832,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":8880,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":7964,"parent_record_offset":7012,"ancestor_record_offset":6832,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":13020,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":13988,"parent_record_offset":13020,"ancestor_record_offset":6832,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":107,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":6832,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":107,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":6832,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":107,"record_offset":7740,"parent_record_offset":7712,"ancestor_record_offset":6832,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":107,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":6832,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":107,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":6832,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":107,"record_offset":15004,"parent_record_offset":14976,"ancestor_record_offset":6832,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":107,"record_offset":7512,"parent_record_offset":7484,"ancestor_record_offset":6832,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":107,"record_offset":7484,"parent_record_offset":7456,"ancestor_record_offset":6832,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":107,"record_offset":8132,"parent_record_offset":8068,"ancestor_record_offset":6832,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":6832,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":8196,"parent_record_offset":8016,"ancestor_record_offset":6832,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":15196,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":11560,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":7304,"parent_record_offset":7280,"ancestor_record_offset":6832,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":10628,"parent_record_offset":10596,"ancestor_record_offset":6832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":6832,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":14976,"parent_record_offset":14952,"ancestor_record_offset":6832,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":15032,"parent_record_offset":14976,"ancestor_record_offset":6832,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":15056,"parent_record_offset":15032,"ancestor_record_offset":6832,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":107,"record_offset":14928,"parent_record_offset":14904,"ancestor_record_offset":6832,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":107,"record_offset":14952,"parent_record_offset":14928,"ancestor_record_offset":6832,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":107,"record_offset":11436,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":107,"record_offset":11668,"parent_record_offset":11628,"ancestor_record_offset":6832,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":107,"record_offset":11368,"parent_record_offset":10560,"ancestor_record_offset":6832,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":107,"record_offset":11404,"parent_record_offset":10560,"ancestor_record_offset":6832,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":107,"record_offset":10532,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":107,"record_offset":8068,"parent_record_offset":8040,"ancestor_record_offset":6832,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":107,"record_offset":8104,"parent_record_offset":8068,"ancestor_record_offset":6832,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":107,"record_offset":8424,"parent_record_offset":8244,"ancestor_record_offset":6832,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":15104,"parent_record_offset":14876,"ancestor_record_offset":6832,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":6832,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":15128,"parent_record_offset":15104,"ancestor_record_offset":6832,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":6832,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":107,"record_offset":7688,"parent_record_offset":7664,"ancestor_record_offset":6832,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":7424,"parent_record_offset":7396,"ancestor_record_offset":6832,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":7456,"parent_record_offset":7424,"ancestor_record_offset":6832,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":7396,"parent_record_offset":7176,"ancestor_record_offset":6832,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":7256,"parent_record_offset":7176,"ancestor_record_offset":6832,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":7816,"parent_record_offset":7176,"ancestor_record_offset":6832,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":7340,"parent_record_offset":7176,"ancestor_record_offset":6832,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":107,"record_offset":7100,"parent_record_offset":7064,"ancestor_record_offset":6832,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":107,"record_offset":7664,"parent_record_offset":7556,"ancestor_record_offset":6832,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":6832,"inlinee":4324,"source_line":261,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":7908,"parent_record_offset":7176,"ancestor_record_offset":6832,"inlinee":4339,"source_line":81,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":7580,"parent_record_offset":7556,"ancestor_record_offset":6832,"inlinee":4322,"source_line":734,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":6832,"inlinee":4302,"source_line":1156,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":7128,"parent_record_offset":7100,"ancestor_record_offset":6832,"inlinee":4301,"source_line":1179,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":6832,"inlinee":4304,"source_line":1862,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":7228,"parent_record_offset":7176,"ancestor_record_offset":6832,"inlinee":4305,"source_line":2301,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":7556,"parent_record_offset":7176,"ancestor_record_offset":6832,"inlinee":4321,"source_line":1166,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":107,"record_offset":7788,"parent_record_offset":7176,"ancestor_record_offset":6832,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":107,"record_offset":11748,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":12341,"source_line":688,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":107,"record_offset":10560,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":12322,"source_line":1083,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/io/reader.rs
//     evidence={"module":107,"record_offset":10596,"parent_record_offset":10560,"ancestor_record_offset":6832,"inlinee":11158,"source_line":293,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":107,"record_offset":19088,"parent_record_offset":19064,"ancestor_record_offset":6832,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":107,"record_offset":14904,"parent_record_offset":14876,"ancestor_record_offset":6832,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":107,"record_offset":14876,"parent_record_offset":14852,"ancestor_record_offset":6832,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":107,"record_offset":11592,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":8327,"source_line":469,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::TextureDesc::sampled
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":107,"record_offset":11720,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":9289,"source_line":512,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$16::into_tex_dim
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":107,"record_offset":11628,"parent_record_offset":11592,"ancestor_record_offset":6832,"inlinee":8319,"source_line":1349,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$47::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":107,"record_offset":14852,"parent_record_offset":8720,"ancestor_record_offset":6832,"inlinee":12343,"source_line":78,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":107,"record_offset":8720,"parent_record_offset":8656,"ancestor_record_offset":6832,"inlinee":12321,"source_line":731,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 731
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3021
// 首条位置: {"module":107,"inlinee":12321}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":14796,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":8880,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":13020,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":15196,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":11560,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":12764,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":107,"record_offset":11436,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":107,"record_offset":10532,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":107,"record_offset":11748,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":12341,"source_line":688,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":107,"record_offset":10560,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":12322,"source_line":1083,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":107,"record_offset":11592,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":8327,"source_line":469,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::sampled
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":107,"record_offset":11720,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":9289,"source_line":512,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$47::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":107,"record_offset":14852,"parent_record_offset":8720,"ancestor_record_offset":8720,"inlinee":12343,"source_line":78,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":10876,"parent_record_offset":10852,"ancestor_record_offset":8720,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":10900,"parent_record_offset":10876,"ancestor_record_offset":8720,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":10924,"parent_record_offset":10900,"ancestor_record_offset":8720,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":10852,"parent_record_offset":10796,"ancestor_record_offset":8720,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":8720,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":10824,"parent_record_offset":10796,"ancestor_record_offset":8720,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":10796,"parent_record_offset":10772,"ancestor_record_offset":8720,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":10256,"parent_record_offset":10232,"ancestor_record_offset":8720,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":8720,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":107,"record_offset":10772,"parent_record_offset":10748,"ancestor_record_offset":8720,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":10184,"parent_record_offset":10160,"ancestor_record_offset":8720,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":10160,"parent_record_offset":10120,"ancestor_record_offset":8720,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":10120,"parent_record_offset":10096,"ancestor_record_offset":8720,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":107,"record_offset":14820,"parent_record_offset":14796,"ancestor_record_offset":8720,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":107,"record_offset":8912,"parent_record_offset":8880,"ancestor_record_offset":8720,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":10480,"parent_record_offset":10456,"ancestor_record_offset":8720,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":10456,"parent_record_offset":10432,"ancestor_record_offset":8720,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":107,"record_offset":10404,"parent_record_offset":10372,"ancestor_record_offset":8720,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":10348,"parent_record_offset":9840,"ancestor_record_offset":8720,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":14500,"parent_record_offset":13988,"ancestor_record_offset":8720,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":8720,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":9868,"parent_record_offset":9840,"ancestor_record_offset":8720,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":10432,"parent_record_offset":10372,"ancestor_record_offset":8720,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":107,"record_offset":14584,"parent_record_offset":14524,"ancestor_record_offset":8720,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":9916,"parent_record_offset":9892,"ancestor_record_offset":8720,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":9892,"parent_record_offset":9868,"ancestor_record_offset":8720,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":107,"record_offset":10096,"parent_record_offset":10072,"ancestor_record_offset":8720,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":107,"record_offset":14700,"parent_record_offset":14676,"ancestor_record_offset":8720,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":14676,"parent_record_offset":13988,"ancestor_record_offset":8720,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":9840,"parent_record_offset":8880,"ancestor_record_offset":8720,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":107,"record_offset":13988,"parent_record_offset":13020,"ancestor_record_offset":8720,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":107,"record_offset":8940,"parent_record_offset":8912,"ancestor_record_offset":8720,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":107,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":8720,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":107,"record_offset":9616,"parent_record_offset":9588,"ancestor_record_offset":8720,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":107,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":8720,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":107,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":8720,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":107,"record_offset":15004,"parent_record_offset":14976,"ancestor_record_offset":8720,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":107,"record_offset":9388,"parent_record_offset":9360,"ancestor_record_offset":8720,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":107,"record_offset":9360,"parent_record_offset":9332,"ancestor_record_offset":8720,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":107,"record_offset":10008,"parent_record_offset":9944,"ancestor_record_offset":8720,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":8720,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":10072,"parent_record_offset":9892,"ancestor_record_offset":8720,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":8720,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":10628,"parent_record_offset":10596,"ancestor_record_offset":8720,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":8720,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":14976,"parent_record_offset":14952,"ancestor_record_offset":8720,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":15032,"parent_record_offset":14976,"ancestor_record_offset":8720,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":107,"record_offset":15056,"parent_record_offset":15032,"ancestor_record_offset":8720,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":107,"record_offset":14928,"parent_record_offset":14904,"ancestor_record_offset":8720,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":107,"record_offset":14952,"parent_record_offset":14928,"ancestor_record_offset":8720,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":107,"record_offset":11668,"parent_record_offset":11628,"ancestor_record_offset":8720,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":107,"record_offset":11368,"parent_record_offset":10560,"ancestor_record_offset":8720,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":107,"record_offset":11404,"parent_record_offset":10560,"ancestor_record_offset":8720,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":107,"record_offset":9944,"parent_record_offset":9916,"ancestor_record_offset":8720,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":107,"record_offset":9980,"parent_record_offset":9944,"ancestor_record_offset":8720,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":107,"record_offset":10300,"parent_record_offset":10120,"ancestor_record_offset":8720,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":15104,"parent_record_offset":14876,"ancestor_record_offset":8720,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":8720,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":15128,"parent_record_offset":15104,"ancestor_record_offset":8720,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":8720,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":107,"record_offset":9564,"parent_record_offset":9540,"ancestor_record_offset":8720,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":9300,"parent_record_offset":9272,"ancestor_record_offset":8720,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":9332,"parent_record_offset":9300,"ancestor_record_offset":8720,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":107,"record_offset":9272,"parent_record_offset":9052,"ancestor_record_offset":8720,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":9132,"parent_record_offset":9052,"ancestor_record_offset":8720,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":9692,"parent_record_offset":9052,"ancestor_record_offset":8720,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":107,"record_offset":9216,"parent_record_offset":9052,"ancestor_record_offset":8720,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":107,"record_offset":8976,"parent_record_offset":8940,"ancestor_record_offset":8720,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":107,"record_offset":9540,"parent_record_offset":9432,"ancestor_record_offset":8720,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":8720,"inlinee":4324,"source_line":261,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":9784,"parent_record_offset":9052,"ancestor_record_offset":8720,"inlinee":4339,"source_line":81,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":8720,"inlinee":4322,"source_line":734,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":9028,"parent_record_offset":9004,"ancestor_record_offset":8720,"inlinee":4302,"source_line":1156,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":9004,"parent_record_offset":8976,"ancestor_record_offset":8720,"inlinee":4301,"source_line":1179,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":8720,"inlinee":4304,"source_line":1862,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":9104,"parent_record_offset":9052,"ancestor_record_offset":8720,"inlinee":4305,"source_line":2301,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":107,"record_offset":9432,"parent_record_offset":9052,"ancestor_record_offset":8720,"inlinee":4321,"source_line":1166,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":107,"record_offset":9664,"parent_record_offset":9052,"ancestor_record_offset":8720,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/io/reader.rs
//     evidence={"module":107,"record_offset":10596,"parent_record_offset":10560,"ancestor_record_offset":8720,"inlinee":11158,"source_line":293,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":107,"record_offset":14904,"parent_record_offset":14876,"ancestor_record_offset":8720,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":107,"record_offset":14876,"parent_record_offset":14852,"ancestor_record_offset":8720,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$16::into_tex_dim
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":107,"record_offset":11628,"parent_record_offset":11592,"ancestor_record_offset":8720,"inlinee":8319,"source_line":1349,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 814
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F3B
// 首条位置: {"module":202,"inlinee":16187}
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
// 源码行: 824
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":202,"record_offset":36556,"rva":5692800,"va":5374401920}
// 编译实例 RVAs: 0x56dd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10392836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e804959e00","size":5,"soft_fail":false,"target_va":5384795984,"va":5374403143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant as core::ops::arith::Sub>::sub
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10355069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e87d019e00","size":5,"soft_fail":false,"target_va":5384762272,"va":5374407198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10239549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e83d3e9c00","size":5,"soft_fail":false,"target_va":5384648448,"va":5374408894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10240851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e853439c00","size":5,"soft_fail":false,"target_va":5384648032,"va":5374407176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10203210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e84ab09b00","size":5,"soft_fail":false,"target_va":5384609408,"va":5374406193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t1269527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8175f1300","size":5,"soft_fail":false,"target_va":5375678704,"va":5374409172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5179808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e860f6b0ff","size":5,"soft_fail":false,"target_va":5369231648,"va":5374411451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5033366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e86a32b3ff","size":5,"soft_fail":false,"target_va":5369376720,"va":5374410081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a7f60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5014903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8897ab3ff","size":5,"soft_fail":false,"target_va":5369397088,"va":5374411986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b6390
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4954533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e85b66b4ff","size":5,"soft_fail":false,"target_va":5369455504,"va":5374410032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c1f40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4906467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e81d22b5ff","size":5,"soft_fail":false,"target_va":5369503552,"va":5374410014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140139520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4417683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e86d97bcff","size":5,"soft_fail":false,"target_va":5369992480,"va":5374410158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fbfa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2565109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e80bdcd8ff","size":5,"soft_fail":false,"target_va":5371838368,"va":5374403472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fc370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2564021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e84be0d8ff","size":5,"soft_fail":false,"target_va":5371839344,"va":5374403360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1940040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8b865e2ff","size":5,"soft_fail":false,"target_va":5372465696,"va":5374405731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140397c30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1928677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e81b92e2ff","size":5,"soft_fail":false,"target_va":5372476464,"va":5374405136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140584570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t86014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8fe4f0100","size":5,"soft_fail":false,"target_va":5374494064,"va":5374408045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140584af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t85526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8164e0100","size":5,"soft_fail":false,"target_va":5374495472,"va":5374409941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140586b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t95906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8a2760100","size":5,"soft_fail":false,"target_va":5374503760,"va":5374407849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d65fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41910810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e81a827f02","size":5,"soft_fail":false,"target_va":5416312752,"va":5374401937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41941749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8f5fa7f02","size":5,"soft_fail":false,"target_va":5416350384,"va":5374408630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41946361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8f90c8002","size":5,"soft_fail":false,"target_va":5416356240,"va":5374409874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42653081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e899d58a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374403970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9227520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e800cd8c00","size":5,"soft_fail":false,"target_va":5383637632,"va":5374410107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42211496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8a8188402","size":5,"soft_fail":false,"target_va":5416621408,"va":5374409907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::Option::map<ref$<ref_mut$<rhapsody::raw::swapchain::Swapchain> >,tuple$<ash::vk::enums::Format,array$<u16,2> >,tiny_glade::render_loop::render::closure_env$1>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e871120000","size":5,"soft_fail":false,"target_va":5374415216,"va":5374410490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42218308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e844338402","size":5,"soft_fail":false,"target_va":5416621536,"va":5374403223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t42218402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8a2338402","size":5,"soft_fail":false,"target_va":5416621600,"va":5374403193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42211697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e871198402","size":5,"soft_fail":false,"target_va":5416622240,"va":5374410538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42211330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e802188402","size":5,"soft_fail":false,"target_va":5416621632,"va":5374410297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8392f0000","size":5,"soft_fail":false,"target_va":5374414160,"va":5374402066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42210984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8a8168402","size":5,"soft_fail":false,"target_va":5416621376,"va":5374410387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::maintenance::nani_frame_boundary_maintenance
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4168568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8789b3f00","size":5,"soft_fail":false,"target_va":5378571312,"va":5374402739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::image_baker::finish_pending_image_bakes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5820566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e896d05800","size":5,"soft_fail":false,"target_va":5380229360,"va":5374408789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5729541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8056d5700","size":5,"soft_fail":false,"target_va":5380133648,"va":5374404102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::sort
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5729893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8656e5700","size":5,"soft_fail":false,"target_va":5380134032,"va":5374404134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5722421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e835515700","size":5,"soft_fail":false,"target_va":5380126544,"va":5374404118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9561372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e81ce59100","size":5,"soft_fail":false,"target_va":5383963792,"va":5374402415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9569879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e857069200","size":5,"soft_fail":false,"target_va":5383972224,"va":5374402340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9554000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e850c89100","size":5,"soft_fail":false,"target_va":5383964912,"va":5374410907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9560768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8c0e29100","size":5,"soft_fail":false,"target_va":5383962816,"va":5374402043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9320619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8ab388e00","size":5,"soft_fail":false,"target_va":5383729488,"va":5374408864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9324527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8ef478e00","size":5,"soft_fail":false,"target_va":5383730768,"va":5374406236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::gpu_scope::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_gpu_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9397220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8e4638f00","size":5,"soft_fail":false,"target_va":5383799984,"va":5374402759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::profiler::TimedFrame::send_to_puffin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9048817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8f1128a00","size":5,"soft_fail":false,"target_va":5383457168,"va":5374408346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::FallibleSwapchain::dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9485709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e88dbd9000","size":5,"soft_fail":false,"target_va":5383890336,"va":5374404622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::rcache::RenderResourceCache::clear_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9447004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e85c269000","size":5,"soft_fail":false,"target_va":5383853232,"va":5374406223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42209460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8b4108402","size":5,"soft_fail":false,"target_va":5416619360,"va":5374409895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::finish_screenshot_capture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-743170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8fea8f4ff","size":5,"soft_fail":false,"target_va":5373665584,"va":5374408749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::tgsr_capture::drain_tgsr_capture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t271338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401920,"raw_hex":"e8ea230400","size":5,"soft_fail":false,"target_va":5374680112,"va":5374408769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":40692,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$11::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":202,"record_offset":59688,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":16195,"source_line":898,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$9::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":202,"record_offset":58940,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":16193,"source_line":846,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":202,"record_offset":48620,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":202,"record_offset":58968,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":202,"record_offset":39020,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":46852,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":202,"record_offset":46788,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":202,"record_offset":52540,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::f64::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":202,"record_offset":62868,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":16202,"source_line":1009,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":46672,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":43420,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":46700,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":55184,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":43392,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":61716,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":202,"record_offset":52360,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":202,"record_offset":62556,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f64
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":202,"record_offset":55244,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":16191,"source_line":866,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_secs_f64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":202,"record_offset":62896,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":16203,"source_line":959,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_config::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":202,"record_offset":42452,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":13776,"source_line":125,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":202,"record_offset":43448,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":202,"record_offset":52596,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":202,"record_offset":52456,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":202,"record_offset":38192,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":202,"record_offset":36792,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":202,"record_offset":37180,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::stats::RenderStats::snapshot_and_reset
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/stats.rs
//     evidence={"module":202,"record_offset":62188,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":16198,"source_line":11,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::SmolRingBuffer::push
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":62420,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":12483,"source_line":1105,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::compile_loading_screen_render_graph
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":52332,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":16189,"source_line":707,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::compile_main_render_graph
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":48676,"parent_record_offset":36556,"ancestor_record_offset":36556,"inlinee":16188,"source_line":123,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":43144,"parent_record_offset":43120,"ancestor_record_offset":36556,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":43168,"parent_record_offset":43144,"ancestor_record_offset":36556,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":43192,"parent_record_offset":43168,"ancestor_record_offset":36556,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":43120,"parent_record_offset":43096,"ancestor_record_offset":36556,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":40364,"parent_record_offset":40340,"ancestor_record_offset":36556,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":59472,"parent_record_offset":59444,"ancestor_record_offset":36556,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":43096,"parent_record_offset":43072,"ancestor_record_offset":36556,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":40412,"parent_record_offset":40388,"ancestor_record_offset":36556,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":40388,"parent_record_offset":40364,"ancestor_record_offset":36556,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":43072,"parent_record_offset":43048,"ancestor_record_offset":36556,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":63860,"parent_record_offset":63824,"ancestor_record_offset":36556,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":63824,"parent_record_offset":63800,"ancestor_record_offset":36556,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":59152,"parent_record_offset":59096,"ancestor_record_offset":36556,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":40340,"parent_record_offset":40316,"ancestor_record_offset":36556,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":40316,"parent_record_offset":40276,"ancestor_record_offset":36556,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":59052,"parent_record_offset":59004,"ancestor_record_offset":36556,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":59096,"parent_record_offset":59052,"ancestor_record_offset":36556,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":40276,"parent_record_offset":40252,"ancestor_record_offset":36556,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::get_unchecked_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":202,"record_offset":59748,"parent_record_offset":59688,"ancestor_record_offset":36556,"inlinee":16196,"source_line":260,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":202,"record_offset":59720,"parent_record_offset":59688,"ancestor_record_offset":36556,"inlinee":4756,"source_line":114,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":202,"record_offset":61776,"parent_record_offset":61744,"ancestor_record_offset":36556,"inlinee":4757,"source_line":545,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$14::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":202,"record_offset":61744,"parent_record_offset":61716,"ancestor_record_offset":36556,"inlinee":4755,"source_line":933,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":202,"record_offset":48644,"parent_record_offset":48620,"ancestor_record_offset":36556,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":202,"record_offset":39072,"parent_record_offset":39044,"ancestor_record_offset":36556,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":202,"record_offset":59004,"parent_record_offset":58968,"ancestor_record_offset":36556,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":40636,"parent_record_offset":40612,"ancestor_record_offset":36556,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":40612,"parent_record_offset":40588,"ancestor_record_offset":36556,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":40560,"parent_record_offset":40528,"ancestor_record_offset":36556,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":40504,"parent_record_offset":39996,"ancestor_record_offset":36556,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":48324,"parent_record_offset":47812,"ancestor_record_offset":36556,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":40528,"parent_record_offset":40504,"ancestor_record_offset":36556,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":40024,"parent_record_offset":39996,"ancestor_record_offset":36556,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":40588,"parent_record_offset":40528,"ancestor_record_offset":36556,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":48408,"parent_record_offset":48348,"ancestor_record_offset":36556,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":40072,"parent_record_offset":40048,"ancestor_record_offset":36556,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":40048,"parent_record_offset":40024,"ancestor_record_offset":36556,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":40252,"parent_record_offset":40228,"ancestor_record_offset":36556,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":202,"record_offset":48524,"parent_record_offset":48500,"ancestor_record_offset":36556,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":48500,"parent_record_offset":47812,"ancestor_record_offset":36556,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":39044,"parent_record_offset":39020,"ancestor_record_offset":36556,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":39996,"parent_record_offset":39044,"ancestor_record_offset":36556,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":47812,"parent_record_offset":46852,"ancestor_record_offset":36556,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":202,"record_offset":39096,"parent_record_offset":39072,"ancestor_record_offset":36556,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":202,"record_offset":39744,"parent_record_offset":39720,"ancestor_record_offset":36556,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":202,"record_offset":39772,"parent_record_offset":39744,"ancestor_record_offset":36556,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":202,"record_offset":38540,"parent_record_offset":38512,"ancestor_record_offset":36556,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":202,"record_offset":38568,"parent_record_offset":38540,"ancestor_record_offset":36556,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":202,"record_offset":38480,"parent_record_offset":38448,"ancestor_record_offset":36556,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":202,"record_offset":38512,"parent_record_offset":38480,"ancestor_record_offset":36556,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":202,"record_offset":38628,"parent_record_offset":38512,"ancestor_record_offset":36556,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":202,"record_offset":38740,"parent_record_offset":38716,"ancestor_record_offset":36556,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":39312,"parent_record_offset":39288,"ancestor_record_offset":36556,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":39396,"parent_record_offset":39372,"ancestor_record_offset":36556,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":202,"record_offset":62820,"parent_record_offset":62784,"ancestor_record_offset":36556,"inlinee":16202,"source_line":1009,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":202,"record_offset":62648,"parent_record_offset":62612,"ancestor_record_offset":36556,"inlinee":16200,"source_line":1036,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":202,"record_offset":63044,"parent_record_offset":62932,"ancestor_record_offset":36556,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":202,"record_offset":38592,"parent_record_offset":38568,"ancestor_record_offset":36556,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":202,"record_offset":59500,"parent_record_offset":59472,"ancestor_record_offset":36556,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":202,"record_offset":62456,"parent_record_offset":62420,"ancestor_record_offset":36556,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":39544,"parent_record_offset":39516,"ancestor_record_offset":36556,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":39516,"parent_record_offset":39488,"ancestor_record_offset":36556,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":202,"record_offset":38348,"parent_record_offset":38304,"ancestor_record_offset":36556,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":40188,"parent_record_offset":40164,"ancestor_record_offset":36556,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":52564,"parent_record_offset":52540,"ancestor_record_offset":36556,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":40228,"parent_record_offset":40048,"ancestor_record_offset":36556,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":45124,"parent_record_offset":43448,"ancestor_record_offset":36556,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":42708,"parent_record_offset":42608,"ancestor_record_offset":36556,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":202,"record_offset":37128,"parent_record_offset":37100,"ancestor_record_offset":36556,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":202,"record_offset":38976,"parent_record_offset":38304,"ancestor_record_offset":36556,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":39336,"parent_record_offset":39312,"ancestor_record_offset":36556,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":38692,"parent_record_offset":38448,"ancestor_record_offset":36556,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":202,"record_offset":59780,"parent_record_offset":59748,"ancestor_record_offset":36556,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":202,"record_offset":39660,"parent_record_offset":39636,"ancestor_record_offset":36556,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":202,"record_offset":63892,"parent_record_offset":63860,"ancestor_record_offset":36556,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":202,"record_offset":37600,"parent_record_offset":37576,"ancestor_record_offset":36556,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":202,"record_offset":37576,"parent_record_offset":37552,"ancestor_record_offset":36556,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":202,"record_offset":37784,"parent_record_offset":37756,"ancestor_record_offset":36556,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":59272,"parent_record_offset":59096,"ancestor_record_offset":36556,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":202,"record_offset":37884,"parent_record_offset":37856,"ancestor_record_offset":36556,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":202,"record_offset":37856,"parent_record_offset":37828,"ancestor_record_offset":36556,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":202,"record_offset":40100,"parent_record_offset":40072,"ancestor_record_offset":36556,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":40136,"parent_record_offset":40100,"ancestor_record_offset":36556,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":40456,"parent_record_offset":40276,"ancestor_record_offset":36556,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":202,"record_offset":37528,"parent_record_offset":37504,"ancestor_record_offset":36556,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":202,"record_offset":37552,"parent_record_offset":37528,"ancestor_record_offset":36556,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":202,"record_offset":37912,"parent_record_offset":37884,"ancestor_record_offset":36556,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":202,"record_offset":37504,"parent_record_offset":37388,"ancestor_record_offset":36556,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":202,"record_offset":36920,"parent_record_offset":36896,"ancestor_record_offset":36556,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":202,"record_offset":37216,"parent_record_offset":37180,"ancestor_record_offset":36556,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":202,"record_offset":37356,"parent_record_offset":37324,"ancestor_record_offset":36556,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":202,"record_offset":37324,"parent_record_offset":37288,"ancestor_record_offset":36556,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":202,"record_offset":37264,"parent_record_offset":37240,"ancestor_record_offset":36556,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":202,"record_offset":37288,"parent_record_offset":37264,"ancestor_record_offset":36556,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":202,"record_offset":37240,"parent_record_offset":37216,"ancestor_record_offset":36556,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":202,"record_offset":37828,"parent_record_offset":37288,"ancestor_record_offset":36556,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":202,"record_offset":36896,"parent_record_offset":36864,"ancestor_record_offset":36556,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":202,"record_offset":36864,"parent_record_offset":36836,"ancestor_record_offset":36556,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":202,"record_offset":37020,"parent_record_offset":36988,"ancestor_record_offset":36556,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":202,"record_offset":37076,"parent_record_offset":37020,"ancestor_record_offset":36556,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":202,"record_offset":36988,"parent_record_offset":36960,"ancestor_record_offset":36556,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":202,"record_offset":36836,"parent_record_offset":36792,"ancestor_record_offset":36556,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":202,"record_offset":37100,"parent_record_offset":37076,"ancestor_record_offset":36556,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":202,"record_offset":37388,"parent_record_offset":37356,"ancestor_record_offset":36556,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":202,"record_offset":37672,"parent_record_offset":37388,"ancestor_record_offset":36556,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":202,"record_offset":37644,"parent_record_offset":37388,"ancestor_record_offset":36556,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":202,"record_offset":37448,"parent_record_offset":37388,"ancestor_record_offset":36556,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":48548,"parent_record_offset":48524,"ancestor_record_offset":36556,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":40720,"parent_record_offset":40692,"ancestor_record_offset":36556,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":52480,"parent_record_offset":52456,"ancestor_record_offset":36556,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::swap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":62220,"parent_record_offset":62188,"ancestor_record_offset":36556,"inlinee":7009,"source_line":2899,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":40744,"parent_record_offset":40720,"ancestor_record_offset":36556,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":48572,"parent_record_offset":48548,"ancestor_record_offset":36556,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":63920,"parent_record_offset":63860,"ancestor_record_offset":36556,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":62244,"parent_record_offset":62220,"ancestor_record_offset":36556,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":63952,"parent_record_offset":63824,"ancestor_record_offset":36556,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::try_from_secs_f64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":202,"record_offset":62932,"parent_record_offset":62896,"ancestor_record_offset":36556,"inlinee":16204,"source_line":1772,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":202,"record_offset":43352,"parent_record_offset":42452,"ancestor_record_offset":36556,"inlinee":13756,"source_line":68,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":202,"record_offset":42676,"parent_record_offset":42648,"ancestor_record_offset":36556,"inlinee":11523,"source_line":10,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":202,"record_offset":42648,"parent_record_offset":42608,"ancestor_record_offset":36556,"inlinee":11521,"source_line":19,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":202,"record_offset":42740,"parent_record_offset":42708,"ancestor_record_offset":36556,"inlinee":11479,"source_line":36,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":202,"record_offset":42608,"parent_record_offset":42520,"ancestor_record_offset":36556,"inlinee":11519,"source_line":54,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":202,"record_offset":42520,"parent_record_offset":42452,"ancestor_record_offset":36556,"inlinee":11517,"source_line":126,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":202,"record_offset":43272,"parent_record_offset":42452,"ancestor_record_offset":36556,"inlinee":13754,"source_line":5,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":202,"record_offset":39720,"parent_record_offset":39696,"ancestor_record_offset":36556,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":39456,"parent_record_offset":39428,"ancestor_record_offset":36556,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":39488,"parent_record_offset":39456,"ancestor_record_offset":36556,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":39428,"parent_record_offset":39208,"ancestor_record_offset":36556,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":39288,"parent_record_offset":39208,"ancestor_record_offset":36556,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":39848,"parent_record_offset":39208,"ancestor_record_offset":36556,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":39372,"parent_record_offset":39208,"ancestor_record_offset":36556,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":39132,"parent_record_offset":39096,"ancestor_record_offset":36556,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":39696,"parent_record_offset":39588,"ancestor_record_offset":36556,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":39636,"parent_record_offset":39612,"ancestor_record_offset":36556,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":39940,"parent_record_offset":39208,"ancestor_record_offset":36556,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":39612,"parent_record_offset":39588,"ancestor_record_offset":36556,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":39184,"parent_record_offset":39160,"ancestor_record_offset":36556,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":39160,"parent_record_offset":39132,"ancestor_record_offset":36556,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":39208,"parent_record_offset":39184,"ancestor_record_offset":36556,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":39260,"parent_record_offset":39208,"ancestor_record_offset":36556,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":39588,"parent_record_offset":39208,"ancestor_record_offset":36556,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":202,"record_offset":39820,"parent_record_offset":39208,"ancestor_record_offset":36556,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":202,"record_offset":52644,"parent_record_offset":52620,"ancestor_record_offset":36556,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":202,"record_offset":52620,"parent_record_offset":52596,"ancestor_record_offset":36556,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":202,"record_offset":52668,"parent_record_offset":52644,"ancestor_record_offset":36556,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":202,"record_offset":46376,"parent_record_offset":46352,"ancestor_record_offset":36556,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":202,"record_offset":38228,"parent_record_offset":38192,"ancestor_record_offset":36556,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":202,"record_offset":64036,"parent_record_offset":64012,"ancestor_record_offset":36556,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":202,"record_offset":64624,"parent_record_offset":64412,"ancestor_record_offset":36556,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":202,"record_offset":38376,"parent_record_offset":38348,"ancestor_record_offset":36556,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":202,"record_offset":38448,"parent_record_offset":38304,"ancestor_record_offset":36556,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":202,"record_offset":38664,"parent_record_offset":38448,"ancestor_record_offset":36556,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$24::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/init.rs
//     evidence={"module":202,"record_offset":42872,"parent_record_offset":42520,"ancestor_record_offset":36556,"inlinee":11527,"source_line":93,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::swapchain::impl$2::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/swapchain.rs
//     evidence={"module":202,"record_offset":46752,"parent_record_offset":46728,"ancestor_record_offset":36556,"inlinee":16185,"source_line":136,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":202,"record_offset":38404,"parent_record_offset":38376,"ancestor_record_offset":36556,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":202,"record_offset":38304,"parent_record_offset":38264,"ancestor_record_offset":36556,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":202,"record_offset":38264,"parent_record_offset":38228,"ancestor_record_offset":36556,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":46816,"parent_record_offset":46788,"ancestor_record_offset":36556,"inlinee":16187,"source_line":814,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":46728,"parent_record_offset":46700,"ancestor_record_offset":36556,"inlinee":16183,"source_line":876,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":55212,"parent_record_offset":55184,"ancestor_record_offset":36556,"inlinee":16190,"source_line":981,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":62612,"parent_record_offset":62556,"ancestor_record_offset":36556,"inlinee":16199,"source_line":1057,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":62784,"parent_record_offset":62696,"ancestor_record_offset":36556,"inlinee":16201,"source_line":1062,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$19::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":202,"record_offset":42840,"parent_record_offset":42812,"ancestor_record_offset":36556,"inlinee":11525,"source_line":437,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 876
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F37
// 首条位置: {"module":202,"inlinee":16183}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::swapchain::impl$2::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/swapchain.rs
//     evidence={"module":202,"record_offset":46752,"parent_record_offset":46728,"ancestor_record_offset":46728,"inlinee":16185,"source_line":136,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 918
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F4D
// 首条位置: {"module":202,"inlinee":16205}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::swapchain::impl$2::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/swapchain.rs
//     evidence={"module":202,"record_offset":68408,"parent_record_offset":68388,"ancestor_record_offset":68388,"inlinee":16185,"source_line":136,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 981
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F3E
// 首条位置: {"module":202,"inlinee":16190}
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
// 源码行: 1003
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3065
// 首条位置: {"module":107,"inlinee":12389}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1057
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F47
// 首条位置: {"module":202,"inlinee":16199}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f64::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":202,"record_offset":62648,"parent_record_offset":62612,"ancestor_record_offset":62612,"inlinee":16200,"source_line":1036,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1062
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F49
// 首条位置: {"module":202,"inlinee":16201}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f64::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":202,"record_offset":62820,"parent_record_offset":62784,"ancestor_record_offset":62784,"inlinee":16202,"source_line":1009,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::handle_screenshot_error
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 1083
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3056
// 首条位置: {"module":107,"inlinee":12374}
// 编译实例 RVAs: 0x56db80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10207805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401408,"raw_hex":"e83dc29b00","size":5,"soft_fail":false,"target_va":5384609408,"va":5374401598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10222467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401408,"raw_hex":"e883fb9b00","size":5,"soft_fail":false,"target_va":5384624080,"va":5374401608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":107,"record_offset":64564,"parent_record_offset":64436,"ancestor_record_offset":64436,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":64732,"parent_record_offset":64436,"ancestor_record_offset":64436,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":107,"record_offset":64620,"parent_record_offset":64436,"ancestor_record_offset":64436,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":107,"record_offset":64480,"parent_record_offset":64436,"ancestor_record_offset":64436,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":107,"record_offset":64588,"parent_record_offset":64564,"ancestor_record_offset":64436,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":64504,"parent_record_offset":64480,"ancestor_record_offset":64436,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":64528,"parent_record_offset":64504,"ancestor_record_offset":64436,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":107,"record_offset":64668,"parent_record_offset":64644,"ancestor_record_offset":64436,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":107,"record_offset":64644,"parent_record_offset":64620,"ancestor_record_offset":64436,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":107,"record_offset":64692,"parent_record_offset":64668,"ancestor_record_offset":64436,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::SmolRingBuffer::push
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 1105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30C3
// 首条位置: {"module":202,"inlinee":12483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":202,"record_offset":62456,"parent_record_offset":62420,"ancestor_record_offset":62420,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world
// 记录形式: compiled_procedure; instances=1
// 源码行: 1121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":202,"record_offset":248,"rva":5678848,"va":5374387968}
// 编译实例 RVAs: 0x56a700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t9082313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8c9958a00","size":5,"soft_fail":false,"target_va":5383472768,"va":5374390450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1285490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8729d1300","size":5,"soft_fail":false,"target_va":5375678688,"va":5374393193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1284563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8d3991300","size":5,"soft_fail":false,"target_va":5375678704,"va":5374394136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5670991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8b177a9ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374393178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b1790
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4953289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8376bb4ff","size":5,"soft_fail":false,"target_va":5369436048,"va":5374389332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140452f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1150462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e80272eeff","size":5,"soft_fail":false,"target_va":5373243232,"va":5374393689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140453ae0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1143148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8948eeeff","size":5,"soft_fail":false,"target_va":5373246176,"va":5374389319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404541d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1145411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8bd85eeff","size":5,"soft_fail":false,"target_va":5373247952,"va":5374393358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140454dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1141832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8b893eeff","size":5,"soft_fail":false,"target_va":5373251024,"va":5374392851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140456ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1130470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e81ac0eeff","size":5,"soft_fail":false,"target_va":5373258912,"va":5374389377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c3130
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t362041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e839860500","size":5,"soft_fail":false,"target_va":5374751024,"va":5374388978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069f380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1264218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e85a4a1300","size":5,"soft_fail":false,"target_va":5375652736,"va":5374388513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069f910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1265656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8f84f1300","size":5,"soft_fail":false,"target_va":5375654160,"va":5374388499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069fea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1267136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8c0551300","size":5,"soft_fail":false,"target_va":5375655584,"va":5374388443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a0430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1268546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8425b1300","size":5,"soft_fail":false,"target_va":5375657008,"va":5374388457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a09c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1269942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8b6601300","size":5,"soft_fail":false,"target_va":5375658432,"va":5374388485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a0f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1271324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e81c661300","size":5,"soft_fail":false,"target_va":5375659856,"va":5374388527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a14e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1272804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8e46b1300","size":5,"soft_fail":false,"target_va":5375661280,"va":5374388471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b05d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5878950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8a6b45900","size":5,"soft_fail":false,"target_va":5380267296,"va":5374388341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7486868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8943d7200","size":5,"soft_fail":false,"target_va":5381887120,"va":5374400247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41963230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8de4e8002","size":5,"soft_fail":false,"target_va":5416353040,"va":5374389805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42667762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8f20e8b02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374389289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42220367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e84f3b8402","size":5,"soft_fail":false,"target_va":5416620769,"va":5374400397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=26
//     evidence={"assembly":"\tcallq\t41982481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8119a8002","size":5,"soft_fail":false,"target_va":5416370864,"va":5374388378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10222878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e81efd9b00","size":5,"soft_fail":false,"target_va":5384624080,"va":5374401197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t42231985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8b1688402","size":5,"soft_fail":false,"target_va":5416621600,"va":5374389610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42221828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e804418402","size":5,"soft_fail":false,"target_va":5416622208,"va":5374400375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42221170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8723e8402","size":5,"soft_fail":false,"target_va":5416621632,"va":5374400457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::RenderSublist::iter_main
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4668140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8ec3a4700","size":5,"soft_fail":false,"target_va":5379058288,"va":5374390143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::RenderSublist::iter_shadow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4664269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8cd2b4700","size":5,"soft_fail":false,"target_va":5379058256,"va":5374393982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5766677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e815fe5700","size":5,"soft_fail":false,"target_va":5380163776,"va":5374397094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::apply_screen_shadows::compute_shadow_mask_mean
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5767443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e813015800","size":5,"soft_fail":false,"target_va":5380164768,"va":5374397320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5970863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8af1b5b00","size":5,"soft_fail":false,"target_va":5380368384,"va":5374397516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5376168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8a8085200","size":5,"soft_fail":false,"target_va":5379772976,"va":5374396803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4495112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e808974400","size":5,"soft_fail":false,"target_va":5378883776,"va":5374388659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4493478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8a6904400","size":5,"soft_fail":false,"target_va":5378882320,"va":5374388837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4501392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e890af4400","size":5,"soft_fail":false,"target_va":5378889968,"va":5374388571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5896406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8d6f85900","size":5,"soft_fail":false,"target_va":5380284960,"va":5374388549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5894759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e867f25900","size":5,"soft_fail":false,"target_va":5380283568,"va":5374388804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4507221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e855c64400","size":5,"soft_fail":false,"target_va":5378895808,"va":5374388582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::dof::dof_prepare
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5767443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e813015800","size":5,"soft_fail":false,"target_va":5380165296,"va":5374397848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5916352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8c0465a00","size":5,"soft_fail":false,"target_va":5380311776,"va":5374395419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4487247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e84f784400","size":5,"soft_fail":false,"target_va":5378878528,"va":5374391276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gather_wall_draw_batches
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4536009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8c9364500","size":5,"soft_fail":false,"target_va":5378925760,"va":5374389746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4537680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8503d4500","size":5,"soft_fail":false,"target_va":5378927696,"va":5374390011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4539917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e80d464500","size":5,"soft_fail":false,"target_va":5378931392,"va":5374391470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_shadow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4539000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e878424500","size":5,"soft_fail":false,"target_va":5378932848,"va":5374393843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4526474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e88a114500","size":5,"soft_fail":false,"target_va":5378916656,"va":5374390177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::grass2::compute_grass2_removal_mask
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5397901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e88d5d5200","size":5,"soft_fail":false,"target_va":5379786576,"va":5374388670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::grass2::grass2_card_splat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5383597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8ad255200","size":5,"soft_fail":false,"target_va":5379775840,"va":5374392238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::grass2::grass2_compute_main
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5387530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e80a355200","size":5,"soft_fail":false,"target_va":5379779440,"va":5374391905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::grass2::grass2_compute_shadow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5387014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e806335200","size":5,"soft_fail":false,"target_va":5379781120,"va":5374394101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::grass2::grass2_debug_overlays
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5385320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8682c5200","size":5,"soft_fail":false,"target_va":5379782752,"va":5374397427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5770927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8af0e5800","size":5,"soft_fail":false,"target_va":5380166464,"va":5374395532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::highlight_active_terrain_stroke
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3729673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e809e93800","size":5,"soft_fail":false,"target_va":5378118304,"va":5374388626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::image_baker::bake_grass2_world_albedo_tint_from_env
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5839440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8501a5900","size":5,"soft_fail":false,"target_va":5380231328,"va":5374391883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5958080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8c0e95a00","size":5,"soft_fail":false,"target_va":5380346944,"va":5374388859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5984595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e853515b00","size":5,"soft_fail":false,"target_va":5380380048,"va":5374395448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4760911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e84fa54800","size":5,"soft_fail":false,"target_va":5379149520,"va":5374388604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4527227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e87b144500","size":5,"soft_fail":false,"target_va":5378916080,"va":5374388848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5762849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e821ef5700","size":5,"soft_fail":false,"target_va":5380161936,"va":5374399082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6079795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e833c55c00","size":5,"soft_fail":false,"target_va":5380468448,"va":5374388648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4508123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8dbc94400","size":5,"soft_fail":false,"target_va":5378896688,"va":5374388560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5763540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8d4f15700","size":5,"soft_fail":false,"target_va":5380160160,"va":5374396615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4545073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8315a4500","size":5,"soft_fail":false,"target_va":5378933904,"va":5374388826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4496746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e86a9d4400","size":5,"soft_fail":false,"target_va":5378885344,"va":5374388593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5770350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e86e0c5800","size":5,"soft_fail":false,"target_va":5380167216,"va":5374396861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5773541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8e5185800","size":5,"soft_fail":false,"target_va":5380162416,"va":5374388870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6103660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e86c225d00","size":5,"soft_fail":false,"target_va":5380492480,"va":5374388815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::stairs::stairs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4516234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e88ae94400","size":5,"soft_fail":false,"target_va":5378905120,"va":5374388881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5379651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e843165200","size":5,"soft_fail":false,"target_va":5379774976,"va":5374395320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4759233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8c19e4800","size":5,"soft_fail":false,"target_va":5379147776,"va":5374388538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::visibility_queries::run_visibility_queries
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6072493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8ada85c00","size":5,"soft_fail":false,"target_va":5380467888,"va":5374395390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4747399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e887704800","size":5,"soft_fail":false,"target_va":5379144608,"va":5374397204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4483140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e844684400","size":5,"soft_fail":false,"target_va":5378871760,"va":5374388615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::grass_velocity_reconstruct::reconstruct_grass_velocity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4477974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e816544400","size":5,"soft_fail":false,"target_va":5378873600,"va":5374395621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4479724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8ec5a4400","size":5,"soft_fail":false,"target_va":5378875232,"va":5374395503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::main_pass::main_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4485198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e84e704400","size":5,"soft_fail":false,"target_va":5378876448,"va":5374391245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4481484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8cc614400","size":5,"soft_fail":false,"target_va":5378873696,"va":5374392207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5811246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e82eac5800","size":5,"soft_fail":false,"target_va":5380199888,"va":5374388637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5751944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e888c45700","size":5,"soft_fail":false,"target_va":5380147056,"va":5374395107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5757316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e884d95700","size":5,"soft_fail":false,"target_va":5380157440,"va":5374400119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5759783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e827e35700","size":5,"soft_fail":false,"target_va":5380157072,"va":5374397284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_and_push_binding_set_from_stack
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9161085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e87dc98b00","size":5,"soft_fail":false,"target_va":5383555520,"va":5374394430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_binding_set
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9155651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e843b48b00","size":5,"soft_fail":false,"target_va":5383544656,"va":5374389000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_binding_set_from_stack
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9162156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8accd8b00","size":5,"soft_fail":false,"target_va":5383551872,"va":5374389711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9157751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e877bc8b00","size":5,"soft_fail":false,"target_va":5383548144,"va":5374390388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::init_binding_set_from_stack
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9150619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e89ba08b00","size":5,"soft_fail":false,"target_va":5383550672,"va":5374400048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::push_binding_set
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9149298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8729b8b00","size":5,"soft_fail":false,"target_va":5383541472,"va":5374392169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::TexView::with_layer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9131055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e82f548b00","size":5,"soft_fail":false,"target_va":5383524560,"va":5374393500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::reduction::DepthMinMax::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9023638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e896b08900","size":5,"soft_fail":false,"target_va":5383415904,"va":5374392261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::reduction::generate_mips
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9018568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8c89c8900","size":5,"soft_fail":false,"target_va":5383416720,"va":5374398147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::reprojection::render_reprojection_map
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9012450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8e2848900","size":5,"soft_fail":false,"target_va":5383408480,"va":5374396025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_dof::render_dof_with_coc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-dof","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7737126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e8260f7600","size":5,"soft_fail":false,"target_va":5382135328,"va":5374398197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_gi::calculate_gtao
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-gi","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7731028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e854f77500","size":5,"soft_fail":false,"target_va":5382127120,"va":5374396087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_gi::eval_gi_screen_radiance
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-gi","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7732351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e87ffc7500","size":5,"soft_fail":false,"target_va":5382129040,"va":5374396684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_shadow::setup_directional_light_cascades
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-shadow","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7593038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e84edc7300","size":5,"soft_fail":false,"target_va":5381985664,"va":5374392621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_taa::create_temporal_history
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-taa","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t1596046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e88e5a1800","size":5,"soft_fail":false,"target_va":5375992448,"va":5374396397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_taa::render_taa
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-taa","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t1590333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e83d441800","size":5,"soft_fail":false,"target_va":5375989136,"va":5374398798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_taa::render_tgsr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-taa","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t1591076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e824471800","size":5,"soft_fail":false,"target_va":5375989680,"va":5374398599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_taa::render_tgsr2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-taa","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t1590819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e823461800","size":5,"soft_fail":false,"target_va":5375989776,"va":5374398952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::tgsr_capture::capture_tgsr_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t279142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374387968,"raw_hex":"e866420400","size":5,"soft_fail":false,"target_va":5374677456,"va":5374398309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r10","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374387968,"raw_hex":"41ffd2","size":3,"soft_fail":false,"va":5374397742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":35104,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":202,"record_offset":2020,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":16149,"source_line":296,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":202,"record_offset":400,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::device
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":3216,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":10362,"source_line":426,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":2048,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::with_binding_set
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":4952,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":12362,"source_line":448,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::with_binding_sets
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":11300,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":16161,"source_line":456,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":9492,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18480,"parent_record_offset":18456,"ancestor_record_offset":248,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18504,"parent_record_offset":18480,"ancestor_record_offset":248,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":20308,"parent_record_offset":20284,"ancestor_record_offset":248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":20332,"parent_record_offset":20308,"ancestor_record_offset":248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":248,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18456,"parent_record_offset":18424,"ancestor_record_offset":248,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":20284,"parent_record_offset":20192,"ancestor_record_offset":248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":18580,"parent_record_offset":18364,"ancestor_record_offset":248,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":18424,"parent_record_offset":18396,"ancestor_record_offset":248,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":18396,"parent_record_offset":18364,"ancestor_record_offset":248,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":11504,"parent_record_offset":11476,"ancestor_record_offset":248,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":248,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":11532,"parent_record_offset":11504,"ancestor_record_offset":248,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":20220,"parent_record_offset":20192,"ancestor_record_offset":248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":20192,"parent_record_offset":20168,"ancestor_record_offset":248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":5356,"parent_record_offset":5332,"ancestor_record_offset":248,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":20168,"parent_record_offset":20144,"ancestor_record_offset":248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":202,"record_offset":18604,"parent_record_offset":18580,"ancestor_record_offset":248,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":35164,"parent_record_offset":35128,"ancestor_record_offset":248,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":248,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":35128,"parent_record_offset":35104,"ancestor_record_offset":248,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":2164,"parent_record_offset":2128,"ancestor_record_offset":248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":1724,"parent_record_offset":1700,"ancestor_record_offset":248,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":1700,"parent_record_offset":1660,"ancestor_record_offset":248,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":11476,"parent_record_offset":11420,"ancestor_record_offset":248,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":5120,"parent_record_offset":5092,"ancestor_record_offset":248,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":18628,"parent_record_offset":18604,"ancestor_record_offset":248,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":5036,"parent_record_offset":5004,"ancestor_record_offset":248,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":248,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":248,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":5308,"parent_record_offset":5120,"ancestor_record_offset":248,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":5280,"parent_record_offset":5120,"ancestor_record_offset":248,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":248,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":5064,"parent_record_offset":5004,"ancestor_record_offset":248,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":248,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":202,"record_offset":5092,"parent_record_offset":5064,"ancestor_record_offset":248,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":202,"record_offset":35552,"parent_record_offset":35528,"ancestor_record_offset":248,"inlinee":4748,"source_line":254,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":202,"record_offset":35528,"parent_record_offset":35504,"ancestor_record_offset":248,"inlinee":4746,"source_line":50,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":202,"record_offset":35628,"parent_record_offset":35600,"ancestor_record_offset":248,"inlinee":4753,"source_line":716,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":202,"record_offset":35576,"parent_record_offset":35552,"ancestor_record_offset":248,"inlinee":4750,"source_line":72,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":202,"record_offset":35600,"parent_record_offset":35576,"ancestor_record_offset":248,"inlinee":4751,"source_line":76,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":202,"record_offset":456,"parent_record_offset":428,"ancestor_record_offset":248,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":248,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":248,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":1944,"parent_record_offset":1912,"ancestor_record_offset":248,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":1888,"parent_record_offset":1380,"ancestor_record_offset":248,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":248,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":248,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":4820,"parent_record_offset":4760,"ancestor_record_offset":248,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":248,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":248,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":248,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":428,"parent_record_offset":400,"ancestor_record_offset":248,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":1380,"parent_record_offset":428,"ancestor_record_offset":248,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":202,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":248,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":202,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":248,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":202,"record_offset":1156,"parent_record_offset":1128,"ancestor_record_offset":248,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":202,"record_offset":18812,"parent_record_offset":18784,"ancestor_record_offset":248,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":248,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":248,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":202,"record_offset":18784,"parent_record_offset":18756,"ancestor_record_offset":248,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":202,"record_offset":18756,"parent_record_offset":18252,"ancestor_record_offset":248,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":202,"record_offset":20252,"parent_record_offset":20220,"ancestor_record_offset":248,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":202,"record_offset":18724,"parent_record_offset":18252,"ancestor_record_offset":248,"inlinee":14912,"source_line":3657,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":928,"parent_record_offset":900,"ancestor_record_offset":248,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":900,"parent_record_offset":872,"ancestor_record_offset":248,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":202,"record_offset":17644,"parent_record_offset":17616,"ancestor_record_offset":248,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":202,"record_offset":1548,"parent_record_offset":1484,"ancestor_record_offset":248,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":248,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":1612,"parent_record_offset":1432,"ancestor_record_offset":248,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":1988,"parent_record_offset":400,"ancestor_record_offset":248,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":248,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":20120,"parent_record_offset":18252,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":202,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":248,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":202,"record_offset":35196,"parent_record_offset":35164,"ancestor_record_offset":248,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":2284,"parent_record_offset":2128,"ancestor_record_offset":248,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":202,"record_offset":27428,"parent_record_offset":22932,"ancestor_record_offset":248,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":202,"record_offset":1484,"parent_record_offset":1456,"ancestor_record_offset":248,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":1520,"parent_record_offset":1484,"ancestor_record_offset":248,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":1840,"parent_record_offset":1660,"ancestor_record_offset":248,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8312,"parent_record_offset":8284,"ancestor_record_offset":248,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":248,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":35224,"parent_record_offset":35164,"ancestor_record_offset":248,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":35256,"parent_record_offset":35128,"ancestor_record_offset":248,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":202,"record_offset":5444,"parent_record_offset":5408,"ancestor_record_offset":248,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":202,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":248,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":202,"record_offset":17616,"parent_record_offset":14160,"ancestor_record_offset":248,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":840,"parent_record_offset":812,"ancestor_record_offset":248,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":872,"parent_record_offset":840,"ancestor_record_offset":248,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":812,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":672,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":1232,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":756,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":516,"parent_record_offset":480,"ancestor_record_offset":248,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":1080,"parent_record_offset":972,"ancestor_record_offset":248,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":248,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":1324,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":248,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":248,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":544,"parent_record_offset":516,"ancestor_record_offset":248,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":248,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":644,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":972,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":202,"record_offset":1204,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::create_instant_binding_set
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":34988,"parent_record_offset":34952,"ancestor_record_offset":248,"inlinee":14012,"source_line":682,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":7676,"parent_record_offset":7652,"ancestor_record_offset":248,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::push_binding_sets
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":5004,"parent_record_offset":4952,"ancestor_record_offset":248,"inlinee":12363,"source_line":473,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::push_instant_binding_set
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":34952,"parent_record_offset":33280,"ancestor_record_offset":248,"inlinee":14100,"source_line":699,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::trim_binding_set_stack
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":7168,"parent_record_offset":4952,"ancestor_record_offset":248,"inlinee":12365,"source_line":496,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::with_binding_sets
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":23232,"parent_record_offset":22932,"ancestor_record_offset":248,"inlinee":16161,"source_line":456,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::dims_2d
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":18668,"parent_record_offset":18252,"ancestor_record_offset":248,"inlinee":13021,"source_line":791,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::with_view_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":18852,"parent_record_offset":18252,"ancestor_record_offset":248,"inlinee":15701,"source_line":707,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":18692,"parent_record_offset":18668,"ancestor_record_offset":248,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::impl$59::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":18884,"parent_record_offset":18852,"ancestor_record_offset":248,"inlinee":15346,"source_line":182,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::temporal::Temporal::map
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/temporal.rs
//     evidence={"module":202,"record_offset":30736,"parent_record_offset":30660,"ancestor_record_offset":248,"inlinee":15703,"source_line":148,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::temporal::Temporal::split_pair
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/temporal.rs
//     evidence={"module":202,"record_offset":27500,"parent_record_offset":22932,"ancestor_record_offset":248,"inlinee":16178,"source_line":157,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::create_cube_mesh_index_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":18364,"parent_record_offset":18252,"ancestor_record_offset":248,"inlinee":16170,"source_line":1989,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":2324,"parent_record_offset":2048,"ancestor_record_offset":248,"inlinee":16152,"source_line":1135,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":2924,"parent_record_offset":2648,"ancestor_record_offset":248,"inlinee":16153,"source_line":1188,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":5408,"parent_record_offset":4952,"ancestor_record_offset":248,"inlinee":16154,"source_line":1228,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":7652,"parent_record_offset":7232,"ancestor_record_offset":248,"inlinee":16155,"source_line":1234,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":7952,"parent_record_offset":7676,"ancestor_record_offset":248,"inlinee":16157,"source_line":1235,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$3::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":8260,"parent_record_offset":7984,"ancestor_record_offset":248,"inlinee":16159,"source_line":1236,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$3::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":8900,"parent_record_offset":8624,"ancestor_record_offset":248,"inlinee":16160,"source_line":1240,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":11896,"parent_record_offset":11300,"ancestor_record_offset":248,"inlinee":16162,"source_line":1267,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$4::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":12208,"parent_record_offset":11932,"ancestor_record_offset":248,"inlinee":16164,"source_line":1287,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$4::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":12516,"parent_record_offset":12240,"ancestor_record_offset":248,"inlinee":16166,"source_line":1288,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$4::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":13152,"parent_record_offset":12876,"ancestor_record_offset":248,"inlinee":16167,"source_line":1292,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":14160,"parent_record_offset":13748,"ancestor_record_offset":248,"inlinee":16168,"source_line":1308,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":18252,"parent_record_offset":17816,"ancestor_record_offset":248,"inlinee":16169,"source_line":1370,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$6::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":19192,"parent_record_offset":18916,"ancestor_record_offset":248,"inlinee":16171,"source_line":1439,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$6::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":19840,"parent_record_offset":19564,"ancestor_record_offset":248,"inlinee":16172,"source_line":1448,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":22932,"parent_record_offset":22492,"ancestor_record_offset":248,"inlinee":16173,"source_line":1469,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$7::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":23824,"parent_record_offset":23232,"ancestor_record_offset":248,"inlinee":16175,"source_line":1478,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$7::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":27812,"parent_record_offset":27532,"ancestor_record_offset":248,"inlinee":16179,"source_line":1585,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$7::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":28380,"parent_record_offset":28100,"ancestor_record_offset":248,"inlinee":16180,"source_line":1661,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$7::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":30660,"parent_record_offset":30368,"ancestor_record_offset":248,"inlinee":16181,"source_line":1718,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$8
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":33280,"parent_record_offset":32848,"ancestor_record_offset":248,"inlinee":16182,"source_line":1966,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::tgsr_capture::CaptureMode::is_off
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/tgsr_capture.rs
//     evidence={"module":202,"record_offset":25636,"parent_record_offset":22932,"ancestor_record_offset":248,"inlinee":16177,"source_line":243,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":202,"record_offset":2080,"parent_record_offset":2048,"ancestor_record_offset":248,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F18
// 首条位置: {"module":202,"inlinee":16152}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F19
// 首条位置: {"module":202,"inlinee":16153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1228
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F1A
// 首条位置: {"module":202,"inlinee":16154}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":202,"record_offset":5444,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":5408,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":5408,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":5408,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":5408,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":6760,"parent_record_offset":6720,"ancestor_record_offset":5408,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":5408,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":202,"record_offset":5512,"parent_record_offset":5480,"ancestor_record_offset":5408,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":5408,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":5408,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":7004,"parent_record_offset":6972,"ancestor_record_offset":5408,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":6948,"parent_record_offset":6440,"ancestor_record_offset":5408,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":6972,"parent_record_offset":6948,"ancestor_record_offset":5408,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":6468,"parent_record_offset":6440,"ancestor_record_offset":5408,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":7032,"parent_record_offset":6972,"ancestor_record_offset":5408,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":5408,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":5408,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":5408,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":5480,"parent_record_offset":5444,"ancestor_record_offset":5408,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":6440,"parent_record_offset":5480,"ancestor_record_offset":5408,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":202,"record_offset":5540,"parent_record_offset":5512,"ancestor_record_offset":5408,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":202,"record_offset":6188,"parent_record_offset":6164,"ancestor_record_offset":5408,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":202,"record_offset":6216,"parent_record_offset":6188,"ancestor_record_offset":5408,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":5408,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":5408,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":5988,"parent_record_offset":5960,"ancestor_record_offset":5408,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":5960,"parent_record_offset":5932,"ancestor_record_offset":5408,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":202,"record_offset":6608,"parent_record_offset":6544,"ancestor_record_offset":5408,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":5408,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":6672,"parent_record_offset":6492,"ancestor_record_offset":5408,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":7132,"parent_record_offset":5444,"ancestor_record_offset":5408,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":5780,"parent_record_offset":5756,"ancestor_record_offset":5408,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":202,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":5408,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":202,"record_offset":6544,"parent_record_offset":6516,"ancestor_record_offset":5408,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":6580,"parent_record_offset":6544,"ancestor_record_offset":5408,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":6900,"parent_record_offset":6720,"ancestor_record_offset":5408,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":202,"record_offset":6164,"parent_record_offset":6140,"ancestor_record_offset":5408,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":5900,"parent_record_offset":5872,"ancestor_record_offset":5408,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":5932,"parent_record_offset":5900,"ancestor_record_offset":5408,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":5872,"parent_record_offset":5652,"ancestor_record_offset":5408,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":5732,"parent_record_offset":5652,"ancestor_record_offset":5408,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":6292,"parent_record_offset":5652,"ancestor_record_offset":5408,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":5816,"parent_record_offset":5652,"ancestor_record_offset":5408,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":5576,"parent_record_offset":5540,"ancestor_record_offset":5408,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":6140,"parent_record_offset":6032,"ancestor_record_offset":5408,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":5408,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":6384,"parent_record_offset":5652,"ancestor_record_offset":5408,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":5408,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":5408,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":5604,"parent_record_offset":5576,"ancestor_record_offset":5408,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":5652,"parent_record_offset":5628,"ancestor_record_offset":5408,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":5704,"parent_record_offset":5652,"ancestor_record_offset":5408,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":6032,"parent_record_offset":5652,"ancestor_record_offset":5408,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":202,"record_offset":6264,"parent_record_offset":5652,"ancestor_record_offset":5408,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F1B
// 首条位置: {"module":202,"inlinee":16155}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":7676,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":7824,"parent_record_offset":7792,"ancestor_record_offset":7652,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":7872,"parent_record_offset":7848,"ancestor_record_offset":7652,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":7848,"parent_record_offset":7824,"ancestor_record_offset":7652,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":7652,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":7792,"parent_record_offset":7756,"ancestor_record_offset":7652,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":7732,"parent_record_offset":7708,"ancestor_record_offset":7652,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":7756,"parent_record_offset":7732,"ancestor_record_offset":7652,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":7912,"parent_record_offset":7756,"ancestor_record_offset":7652,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8312,"parent_record_offset":8284,"ancestor_record_offset":7652,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":7652,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":7984,"parent_record_offset":7952,"ancestor_record_offset":7652,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":7952,"parent_record_offset":7676,"ancestor_record_offset":7652,"inlinee":16157,"source_line":1235,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$3::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":8260,"parent_record_offset":7984,"ancestor_record_offset":7652,"inlinee":16159,"source_line":1236,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$3::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":8900,"parent_record_offset":8624,"ancestor_record_offset":7652,"inlinee":16160,"source_line":1240,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":202,"record_offset":7708,"parent_record_offset":7676,"ancestor_record_offset":7652,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$3::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F1D
// 首条位置: {"module":202,"inlinee":16157}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":7984,"parent_record_offset":7952,"ancestor_record_offset":7952,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":8132,"parent_record_offset":8100,"ancestor_record_offset":7952,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":7952,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":7952,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":7952,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":8100,"parent_record_offset":8064,"ancestor_record_offset":7952,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":8040,"parent_record_offset":8016,"ancestor_record_offset":7952,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":8064,"parent_record_offset":8040,"ancestor_record_offset":7952,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":8220,"parent_record_offset":8064,"ancestor_record_offset":7952,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8312,"parent_record_offset":8284,"ancestor_record_offset":7952,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":7952,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$3::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":8260,"parent_record_offset":7984,"ancestor_record_offset":7952,"inlinee":16159,"source_line":1236,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$3::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":8900,"parent_record_offset":8624,"ancestor_record_offset":7952,"inlinee":16160,"source_line":1240,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":202,"record_offset":8016,"parent_record_offset":7984,"ancestor_record_offset":7952,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$3::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1236
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F1F
// 首条位置: {"module":202,"inlinee":16159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":8260,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8312,"parent_record_offset":8284,"ancestor_record_offset":8260,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":8260,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$3::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F20
// 首条位置: {"module":202,"inlinee":16160}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F22
// 首条位置: {"module":202,"inlinee":16162}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":11932,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":12080,"parent_record_offset":12048,"ancestor_record_offset":11896,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":12128,"parent_record_offset":12104,"ancestor_record_offset":11896,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":12104,"parent_record_offset":12080,"ancestor_record_offset":11896,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":12540,"parent_record_offset":12516,"ancestor_record_offset":11896,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":12048,"parent_record_offset":12012,"ancestor_record_offset":11896,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":11896,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":12012,"parent_record_offset":11988,"ancestor_record_offset":11896,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":12168,"parent_record_offset":12012,"ancestor_record_offset":11896,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":11896,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":12588,"parent_record_offset":12564,"ancestor_record_offset":11896,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":12240,"parent_record_offset":12208,"ancestor_record_offset":11896,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$4::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":12208,"parent_record_offset":11932,"ancestor_record_offset":11896,"inlinee":16164,"source_line":1287,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$4::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":12516,"parent_record_offset":12240,"ancestor_record_offset":11896,"inlinee":16166,"source_line":1288,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$4::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":13152,"parent_record_offset":12876,"ancestor_record_offset":11896,"inlinee":16167,"source_line":1292,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":202,"record_offset":11964,"parent_record_offset":11932,"ancestor_record_offset":11896,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$4::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F24
// 首条位置: {"module":202,"inlinee":16164}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":12240,"parent_record_offset":12208,"ancestor_record_offset":12208,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":12388,"parent_record_offset":12356,"ancestor_record_offset":12208,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":12436,"parent_record_offset":12412,"ancestor_record_offset":12208,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":12412,"parent_record_offset":12388,"ancestor_record_offset":12208,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":12540,"parent_record_offset":12516,"ancestor_record_offset":12208,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":12356,"parent_record_offset":12320,"ancestor_record_offset":12208,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":12208,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":12208,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":12476,"parent_record_offset":12320,"ancestor_record_offset":12208,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":12208,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":12588,"parent_record_offset":12564,"ancestor_record_offset":12208,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$4::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":12516,"parent_record_offset":12240,"ancestor_record_offset":12208,"inlinee":16166,"source_line":1288,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$4::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":13152,"parent_record_offset":12876,"ancestor_record_offset":12208,"inlinee":16167,"source_line":1292,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":202,"record_offset":12272,"parent_record_offset":12240,"ancestor_record_offset":12208,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$4::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1288
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F26
// 首条位置: {"module":202,"inlinee":16166}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":12540,"parent_record_offset":12516,"ancestor_record_offset":12516,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":12516,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":12588,"parent_record_offset":12564,"ancestor_record_offset":12516,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$4::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F27
// 首条位置: {"module":202,"inlinee":16167}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F28
// 首条位置: {"module":202,"inlinee":16168}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":15964,"parent_record_offset":14160,"ancestor_record_offset":14160,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":17676,"parent_record_offset":14160,"ancestor_record_offset":14160,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":202,"record_offset":14244,"parent_record_offset":14160,"ancestor_record_offset":14160,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":202,"record_offset":17616,"parent_record_offset":14160,"ancestor_record_offset":14160,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":15608,"parent_record_offset":15584,"ancestor_record_offset":14160,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":15656,"parent_record_offset":15632,"ancestor_record_offset":14160,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":15632,"parent_record_offset":15608,"ancestor_record_offset":14160,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":15584,"parent_record_offset":15560,"ancestor_record_offset":14160,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":15560,"parent_record_offset":15520,"ancestor_record_offset":14160,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":15520,"parent_record_offset":15496,"ancestor_record_offset":14160,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":202,"record_offset":14312,"parent_record_offset":14280,"ancestor_record_offset":14160,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":15880,"parent_record_offset":15856,"ancestor_record_offset":14160,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":15856,"parent_record_offset":15832,"ancestor_record_offset":14160,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":15804,"parent_record_offset":15772,"ancestor_record_offset":14160,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":15748,"parent_record_offset":15240,"ancestor_record_offset":14160,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":15772,"parent_record_offset":15748,"ancestor_record_offset":14160,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":15268,"parent_record_offset":15240,"ancestor_record_offset":14160,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":15832,"parent_record_offset":15772,"ancestor_record_offset":14160,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":14160,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":15292,"parent_record_offset":15268,"ancestor_record_offset":14160,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":15496,"parent_record_offset":15472,"ancestor_record_offset":14160,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":14280,"parent_record_offset":14244,"ancestor_record_offset":14160,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":15240,"parent_record_offset":14280,"ancestor_record_offset":14160,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":202,"record_offset":14340,"parent_record_offset":14312,"ancestor_record_offset":14160,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":202,"record_offset":14988,"parent_record_offset":14964,"ancestor_record_offset":14160,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":202,"record_offset":15016,"parent_record_offset":14988,"ancestor_record_offset":14160,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":14160,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":14640,"parent_record_offset":14616,"ancestor_record_offset":14160,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":14788,"parent_record_offset":14760,"ancestor_record_offset":14160,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":14760,"parent_record_offset":14732,"ancestor_record_offset":14160,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":202,"record_offset":17644,"parent_record_offset":17616,"ancestor_record_offset":14160,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":202,"record_offset":15408,"parent_record_offset":15344,"ancestor_record_offset":14160,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":15432,"parent_record_offset":15408,"ancestor_record_offset":14160,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":15472,"parent_record_offset":15292,"ancestor_record_offset":14160,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":15932,"parent_record_offset":14244,"ancestor_record_offset":14160,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":14580,"parent_record_offset":14556,"ancestor_record_offset":14160,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":202,"record_offset":14904,"parent_record_offset":14880,"ancestor_record_offset":14160,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":202,"record_offset":15344,"parent_record_offset":15316,"ancestor_record_offset":14160,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":15380,"parent_record_offset":15344,"ancestor_record_offset":14160,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":15700,"parent_record_offset":15520,"ancestor_record_offset":14160,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":202,"record_offset":14964,"parent_record_offset":14940,"ancestor_record_offset":14160,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":14700,"parent_record_offset":14672,"ancestor_record_offset":14160,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":14732,"parent_record_offset":14700,"ancestor_record_offset":14160,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":14672,"parent_record_offset":14452,"ancestor_record_offset":14160,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":14532,"parent_record_offset":14452,"ancestor_record_offset":14160,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":15092,"parent_record_offset":14452,"ancestor_record_offset":14160,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":14616,"parent_record_offset":14452,"ancestor_record_offset":14160,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":14376,"parent_record_offset":14340,"ancestor_record_offset":14160,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":14940,"parent_record_offset":14832,"ancestor_record_offset":14160,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":14880,"parent_record_offset":14856,"ancestor_record_offset":14160,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":15184,"parent_record_offset":14452,"ancestor_record_offset":14160,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":14856,"parent_record_offset":14832,"ancestor_record_offset":14160,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":14428,"parent_record_offset":14404,"ancestor_record_offset":14160,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":14404,"parent_record_offset":14376,"ancestor_record_offset":14160,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":14452,"parent_record_offset":14428,"ancestor_record_offset":14160,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":14504,"parent_record_offset":14452,"ancestor_record_offset":14160,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":14832,"parent_record_offset":14452,"ancestor_record_offset":14160,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":202,"record_offset":15064,"parent_record_offset":14452,"ancestor_record_offset":14160,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1370
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F29
// 首条位置: {"module":202,"inlinee":16169}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":202,"record_offset":18756,"parent_record_offset":18252,"ancestor_record_offset":18252,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$7::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":202,"record_offset":18724,"parent_record_offset":18252,"ancestor_record_offset":18252,"inlinee":14912,"source_line":3657,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":20120,"parent_record_offset":18252,"ancestor_record_offset":18252,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":18916,"parent_record_offset":18252,"ancestor_record_offset":18252,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":18668,"parent_record_offset":18252,"ancestor_record_offset":18252,"inlinee":13021,"source_line":791,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::with_view_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":18852,"parent_record_offset":18252,"ancestor_record_offset":18252,"inlinee":15701,"source_line":707,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::create_cube_mesh_index_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":18364,"parent_record_offset":18252,"ancestor_record_offset":18252,"inlinee":16170,"source_line":1989,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18480,"parent_record_offset":18456,"ancestor_record_offset":18252,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18504,"parent_record_offset":18480,"ancestor_record_offset":18252,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":20308,"parent_record_offset":20284,"ancestor_record_offset":18252,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":20332,"parent_record_offset":20308,"ancestor_record_offset":18252,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":18252,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":18252,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18456,"parent_record_offset":18424,"ancestor_record_offset":18252,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":20284,"parent_record_offset":20192,"ancestor_record_offset":18252,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":18580,"parent_record_offset":18364,"ancestor_record_offset":18252,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":18424,"parent_record_offset":18396,"ancestor_record_offset":18252,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":18396,"parent_record_offset":18364,"ancestor_record_offset":18252,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":19064,"parent_record_offset":19032,"ancestor_record_offset":18252,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":20220,"parent_record_offset":20192,"ancestor_record_offset":18252,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":20192,"parent_record_offset":20168,"ancestor_record_offset":18252,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":19112,"parent_record_offset":19088,"ancestor_record_offset":18252,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":19088,"parent_record_offset":19064,"ancestor_record_offset":18252,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":20168,"parent_record_offset":20144,"ancestor_record_offset":18252,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":202,"record_offset":18604,"parent_record_offset":18580,"ancestor_record_offset":18252,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":19224,"parent_record_offset":19192,"ancestor_record_offset":18252,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":19032,"parent_record_offset":18996,"ancestor_record_offset":18252,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":18628,"parent_record_offset":18604,"ancestor_record_offset":18252,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":18972,"parent_record_offset":18948,"ancestor_record_offset":18252,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":18252,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":202,"record_offset":18812,"parent_record_offset":18784,"ancestor_record_offset":18252,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":202,"record_offset":18784,"parent_record_offset":18756,"ancestor_record_offset":18252,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":202,"record_offset":20252,"parent_record_offset":20220,"ancestor_record_offset":18252,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":20144,"parent_record_offset":20120,"ancestor_record_offset":18252,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":19152,"parent_record_offset":18996,"ancestor_record_offset":18252,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":19252,"parent_record_offset":19224,"ancestor_record_offset":18252,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":19276,"parent_record_offset":19252,"ancestor_record_offset":18252,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":18692,"parent_record_offset":18668,"ancestor_record_offset":18252,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::impl$59::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":18884,"parent_record_offset":18852,"ancestor_record_offset":18252,"inlinee":15346,"source_line":182,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$6::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":19192,"parent_record_offset":18916,"ancestor_record_offset":18252,"inlinee":16171,"source_line":1439,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$6::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":19840,"parent_record_offset":19564,"ancestor_record_offset":18252,"inlinee":16172,"source_line":1448,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":202,"record_offset":18948,"parent_record_offset":18916,"ancestor_record_offset":18252,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$6::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1397
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3697
// 首条位置: {"module":143,"inlinee":13975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":143,"record_offset":18656,"parent_record_offset":18552,"ancestor_record_offset":18552,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::dims_uvec2
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":143,"record_offset":18628,"parent_record_offset":18552,"ancestor_record_offset":18552,"inlinee":13978,"source_line":805,"archive":"_pdb/C/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] rhapsody_core::reduction::DepthMinMax::tex_div_16x16
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/reduction.rs
//     evidence={"module":143,"record_offset":18600,"parent_record_offset":18552,"ancestor_record_offset":18552,"inlinee":13977,"source_line":75,"archive":"_pdb/C/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":143,"record_offset":18740,"parent_record_offset":18712,"ancestor_record_offset":18552,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":143,"record_offset":18684,"parent_record_offset":18656,"ancestor_record_offset":18552,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":143,"record_offset":18768,"parent_record_offset":18740,"ancestor_record_offset":18552,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":143,"record_offset":18712,"parent_record_offset":18684,"ancestor_record_offset":18552,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/00143.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$6::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1439
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F2B
// 首条位置: {"module":202,"inlinee":16171}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":19224,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":19252,"parent_record_offset":19224,"ancestor_record_offset":19192,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":19276,"parent_record_offset":19252,"ancestor_record_offset":19192,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$6::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1448
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F2C
// 首条位置: {"module":202,"inlinee":16172}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1469
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F2D
// 首条位置: {"module":202,"inlinee":16173}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":202,"record_offset":27428,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":202,"record_offset":23916,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":27532,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::with_binding_sets
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":23232,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":16161,"source_line":456,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":27372,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":13021,"source_line":791,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::Temporal::split_pair
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/temporal.rs
//     evidence={"module":202,"record_offset":27500,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":16178,"source_line":157,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::tgsr_capture::CaptureMode::is_off
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/tgsr_capture.rs
//     evidence={"module":202,"record_offset":25636,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":16177,"source_line":243,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":23432,"parent_record_offset":23404,"ancestor_record_offset":22932,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":23648,"parent_record_offset":23596,"ancestor_record_offset":22932,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":23528,"parent_record_offset":23500,"ancestor_record_offset":22932,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":23460,"parent_record_offset":23432,"ancestor_record_offset":22932,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":23704,"parent_record_offset":23676,"ancestor_record_offset":22932,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":23676,"parent_record_offset":23648,"ancestor_record_offset":22932,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":23556,"parent_record_offset":23528,"ancestor_record_offset":22932,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":23596,"parent_record_offset":23348,"ancestor_record_offset":22932,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":25256,"parent_record_offset":25232,"ancestor_record_offset":22932,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":25232,"parent_record_offset":25192,"ancestor_record_offset":22932,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":23404,"parent_record_offset":23348,"ancestor_record_offset":22932,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":23348,"parent_record_offset":23320,"ancestor_record_offset":22932,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":23792,"parent_record_offset":23264,"ancestor_record_offset":22932,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":27588,"parent_record_offset":27564,"ancestor_record_offset":22932,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":27612,"parent_record_offset":27588,"ancestor_record_offset":22932,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":23500,"parent_record_offset":23348,"ancestor_record_offset":22932,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":23748,"parent_record_offset":23348,"ancestor_record_offset":22932,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":23876,"parent_record_offset":23852,"ancestor_record_offset":22932,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":23292,"parent_record_offset":23264,"ancestor_record_offset":22932,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":25192,"parent_record_offset":25168,"ancestor_record_offset":22932,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":202,"record_offset":23320,"parent_record_offset":23292,"ancestor_record_offset":22932,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":202,"record_offset":23984,"parent_record_offset":23952,"ancestor_record_offset":22932,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":25552,"parent_record_offset":25528,"ancestor_record_offset":22932,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":25528,"parent_record_offset":25504,"ancestor_record_offset":22932,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":25476,"parent_record_offset":25444,"ancestor_record_offset":22932,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":25420,"parent_record_offset":24912,"ancestor_record_offset":22932,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":25444,"parent_record_offset":25420,"ancestor_record_offset":22932,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":24940,"parent_record_offset":24912,"ancestor_record_offset":22932,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":25504,"parent_record_offset":25444,"ancestor_record_offset":22932,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":24988,"parent_record_offset":24964,"ancestor_record_offset":22932,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":24964,"parent_record_offset":24940,"ancestor_record_offset":22932,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":25168,"parent_record_offset":25144,"ancestor_record_offset":22932,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":23952,"parent_record_offset":23916,"ancestor_record_offset":22932,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":24912,"parent_record_offset":23952,"ancestor_record_offset":22932,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":202,"record_offset":24012,"parent_record_offset":23984,"ancestor_record_offset":22932,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":202,"record_offset":24660,"parent_record_offset":24636,"ancestor_record_offset":22932,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":202,"record_offset":24688,"parent_record_offset":24660,"ancestor_record_offset":22932,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":24228,"parent_record_offset":24204,"ancestor_record_offset":22932,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":24312,"parent_record_offset":24288,"ancestor_record_offset":22932,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":24460,"parent_record_offset":24432,"ancestor_record_offset":22932,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":24432,"parent_record_offset":24404,"ancestor_record_offset":22932,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":202,"record_offset":25080,"parent_record_offset":25016,"ancestor_record_offset":22932,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":25104,"parent_record_offset":25080,"ancestor_record_offset":22932,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":25144,"parent_record_offset":24964,"ancestor_record_offset":22932,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":25604,"parent_record_offset":23916,"ancestor_record_offset":22932,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":24252,"parent_record_offset":24228,"ancestor_record_offset":22932,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":27468,"parent_record_offset":27428,"ancestor_record_offset":22932,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":202,"record_offset":24576,"parent_record_offset":24552,"ancestor_record_offset":22932,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":27772,"parent_record_offset":27612,"ancestor_record_offset":22932,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":202,"record_offset":25016,"parent_record_offset":24988,"ancestor_record_offset":22932,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":25052,"parent_record_offset":25016,"ancestor_record_offset":22932,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":25372,"parent_record_offset":25192,"ancestor_record_offset":22932,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":202,"record_offset":24636,"parent_record_offset":24612,"ancestor_record_offset":22932,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":24372,"parent_record_offset":24344,"ancestor_record_offset":22932,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":24404,"parent_record_offset":24372,"ancestor_record_offset":22932,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":24344,"parent_record_offset":24124,"ancestor_record_offset":22932,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":24204,"parent_record_offset":24124,"ancestor_record_offset":22932,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":24764,"parent_record_offset":24124,"ancestor_record_offset":22932,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":24288,"parent_record_offset":24124,"ancestor_record_offset":22932,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":24048,"parent_record_offset":24012,"ancestor_record_offset":22932,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":24612,"parent_record_offset":24504,"ancestor_record_offset":22932,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":24552,"parent_record_offset":24528,"ancestor_record_offset":22932,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":24856,"parent_record_offset":24124,"ancestor_record_offset":22932,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":24528,"parent_record_offset":24504,"ancestor_record_offset":22932,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":24100,"parent_record_offset":24076,"ancestor_record_offset":22932,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":24076,"parent_record_offset":24048,"ancestor_record_offset":22932,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":24124,"parent_record_offset":24100,"ancestor_record_offset":22932,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":24176,"parent_record_offset":24124,"ancestor_record_offset":22932,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":24504,"parent_record_offset":24124,"ancestor_record_offset":22932,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":202,"record_offset":24736,"parent_record_offset":24124,"ancestor_record_offset":22932,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::push_binding_sets
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":23264,"parent_record_offset":23232,"ancestor_record_offset":22932,"inlinee":12363,"source_line":473,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::trim_binding_set_stack
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":23852,"parent_record_offset":23232,"ancestor_record_offset":22932,"inlinee":12365,"source_line":496,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":202,"record_offset":27396,"parent_record_offset":27372,"ancestor_record_offset":22932,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::temporal::Temporal::map
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/temporal.rs
//     evidence={"module":202,"record_offset":30736,"parent_record_offset":30660,"ancestor_record_offset":22932,"inlinee":15703,"source_line":148,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$7::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":23824,"parent_record_offset":23232,"ancestor_record_offset":22932,"inlinee":16175,"source_line":1478,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$7::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":27812,"parent_record_offset":27532,"ancestor_record_offset":22932,"inlinee":16179,"source_line":1585,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$7::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":28380,"parent_record_offset":28100,"ancestor_record_offset":22932,"inlinee":16180,"source_line":1661,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render_world::closure$7::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":202,"record_offset":30660,"parent_record_offset":30368,"ancestor_record_offset":22932,"inlinee":16181,"source_line":1718,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":202,"record_offset":27564,"parent_record_offset":27532,"ancestor_record_offset":22932,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$7::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1478
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F2F
// 首条位置: {"module":202,"inlinee":16175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$7::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1585
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F33
// 首条位置: {"module":202,"inlinee":16179}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$7::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1661
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F34
// 首条位置: {"module":202,"inlinee":16180}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$7::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1718
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F35
// 首条位置: {"module":202,"inlinee":16181}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::temporal::Temporal::map
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/temporal.rs
//     evidence={"module":202,"record_offset":30736,"parent_record_offset":30660,"ancestor_record_offset":30660,"inlinee":15703,"source_line":148,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1966
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F36
// 首条位置: {"module":202,"inlinee":16182}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":202,"record_offset":33328,"parent_record_offset":33280,"ancestor_record_offset":33280,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::push_instant_binding_set
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":34952,"parent_record_offset":33280,"ancestor_record_offset":33280,"inlinee":14100,"source_line":699,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":34680,"parent_record_offset":34656,"ancestor_record_offset":33280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":34728,"parent_record_offset":34704,"ancestor_record_offset":33280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":202,"record_offset":34704,"parent_record_offset":34680,"ancestor_record_offset":33280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":34656,"parent_record_offset":34632,"ancestor_record_offset":33280,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":34632,"parent_record_offset":34592,"ancestor_record_offset":33280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":34592,"parent_record_offset":34568,"ancestor_record_offset":33280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":202,"record_offset":33388,"parent_record_offset":33360,"ancestor_record_offset":33280,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":202,"record_offset":34876,"parent_record_offset":34844,"ancestor_record_offset":33280,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":34820,"parent_record_offset":34312,"ancestor_record_offset":33280,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":34844,"parent_record_offset":34820,"ancestor_record_offset":33280,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":202,"record_offset":34340,"parent_record_offset":34312,"ancestor_record_offset":33280,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":34388,"parent_record_offset":34364,"ancestor_record_offset":33280,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":34364,"parent_record_offset":34340,"ancestor_record_offset":33280,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":202,"record_offset":34568,"parent_record_offset":34544,"ancestor_record_offset":33280,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":33360,"parent_record_offset":33328,"ancestor_record_offset":33280,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":202,"record_offset":34312,"parent_record_offset":33360,"ancestor_record_offset":33280,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":202,"record_offset":33412,"parent_record_offset":33388,"ancestor_record_offset":33280,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":202,"record_offset":34060,"parent_record_offset":34036,"ancestor_record_offset":33280,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":202,"record_offset":34088,"parent_record_offset":34060,"ancestor_record_offset":33280,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":33628,"parent_record_offset":33604,"ancestor_record_offset":33280,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":202,"record_offset":33712,"parent_record_offset":33688,"ancestor_record_offset":33280,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":33860,"parent_record_offset":33832,"ancestor_record_offset":33280,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":202,"record_offset":33832,"parent_record_offset":33804,"ancestor_record_offset":33280,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":202,"record_offset":34480,"parent_record_offset":34416,"ancestor_record_offset":33280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":34504,"parent_record_offset":34480,"ancestor_record_offset":33280,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":34544,"parent_record_offset":34364,"ancestor_record_offset":33280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":202,"record_offset":34920,"parent_record_offset":33328,"ancestor_record_offset":33280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":33652,"parent_record_offset":33628,"ancestor_record_offset":33280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":202,"record_offset":33976,"parent_record_offset":33952,"ancestor_record_offset":33280,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":202,"record_offset":34416,"parent_record_offset":34388,"ancestor_record_offset":33280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":34452,"parent_record_offset":34416,"ancestor_record_offset":33280,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":202,"record_offset":34772,"parent_record_offset":34592,"ancestor_record_offset":33280,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":202,"record_offset":34036,"parent_record_offset":34012,"ancestor_record_offset":33280,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":33772,"parent_record_offset":33744,"ancestor_record_offset":33280,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":33804,"parent_record_offset":33772,"ancestor_record_offset":33280,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":202,"record_offset":33744,"parent_record_offset":33524,"ancestor_record_offset":33280,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":33604,"parent_record_offset":33524,"ancestor_record_offset":33280,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":34164,"parent_record_offset":33524,"ancestor_record_offset":33280,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":202,"record_offset":33688,"parent_record_offset":33524,"ancestor_record_offset":33280,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":33448,"parent_record_offset":33412,"ancestor_record_offset":33280,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":202,"record_offset":34012,"parent_record_offset":33904,"ancestor_record_offset":33280,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":33952,"parent_record_offset":33928,"ancestor_record_offset":33280,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":34256,"parent_record_offset":33524,"ancestor_record_offset":33280,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":33928,"parent_record_offset":33904,"ancestor_record_offset":33280,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":33500,"parent_record_offset":33476,"ancestor_record_offset":33280,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":33476,"parent_record_offset":33448,"ancestor_record_offset":33280,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":33524,"parent_record_offset":33500,"ancestor_record_offset":33280,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":33576,"parent_record_offset":33524,"ancestor_record_offset":33280,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":202,"record_offset":33904,"parent_record_offset":33524,"ancestor_record_offset":33280,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":202,"record_offset":34136,"parent_record_offset":33524,"ancestor_record_offset":33280,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::create_instant_binding_set
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":202,"record_offset":34988,"parent_record_offset":34952,"ancestor_record_offset":33280,"inlinee":14012,"source_line":682,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::create_cube_mesh_index_buffer
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1989
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F2A
// 首条位置: {"module":202,"inlinee":16170}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":18580,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":18396,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18480,"parent_record_offset":18456,"ancestor_record_offset":18364,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18504,"parent_record_offset":18480,"ancestor_record_offset":18364,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":18364,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":18456,"parent_record_offset":18424,"ancestor_record_offset":18364,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":18424,"parent_record_offset":18396,"ancestor_record_offset":18364,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":202,"record_offset":18604,"parent_record_offset":18580,"ancestor_record_offset":18364,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":18628,"parent_record_offset":18604,"ancestor_record_offset":18364,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00202.jsonl.gz"}
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
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3775233,"position_among_mapped_render_targets":3,"mapped_render_target_count":24,"previous_callsite":3774563,"next_callsite":3775310,"previous_target_files":["crates/country-core/src/systems/render_loop/painterly/mod.rs"],"next_target_files":["crates/tiny-glade/src/render_loop.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3775310,"position_among_mapped_render_targets":4,"mapped_render_target_count":24,"previous_callsite":3775233,"next_callsite":3775367,"previous_target_files":["crates/tiny-glade/src/render_loop.rs"],"next_target_files":["crates/tiny-glade/src/render_loop.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3775367,"position_among_mapped_render_targets":5,"mapped_render_target_count":24,"previous_callsite":3775310,"next_callsite":3775709,"previous_target_files":["crates/tiny-glade/src/render_loop.rs"],"next_target_files":["crates/tiny-glade/src/render_loop.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3775709,"position_among_mapped_render_targets":6,"mapped_render_target_count":24,"previous_callsite":3775367,"next_callsite":3777034,"previous_target_files":["crates/tiny-glade/src/render_loop.rs"],"next_target_files":["rhapsody2/render-std/rhapsody-post/src/blur_pyramid.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3777624,"position_among_mapped_render_targets":9,"mapped_render_target_count":24,"previous_callsite":3777243,"next_callsite":3777654,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/final_post.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/menu_background.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3778020,"position_among_mapped_render_targets":14,"mapped_render_target_count":24,"previous_callsite":3777975,"next_callsite":3778059,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/session_selection/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3778368,"position_among_mapped_render_targets":22,"mapped_render_target_count":24,"previous_callsite":3778323,"next_callsite":3778408,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3778652,"position_among_mapped_render_targets":24,"mapped_render_target_count":24,"previous_callsite":3778408,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5741680,"caller_aliases":{"alias_group_rva":5741680,"count":1,"display_examples":["tiny_glade::game_loop::GameLoop::main_loop"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5766718,"position_among_mapped_render_targets":2,"mapped_render_target_count":2,"previous_callsite":5745731,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/vermeer.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6188640,"caller_aliases":{"alias_group_rva":6188640,"count":1,"display_examples":["std::sys::backtrace::__rust_begin_short_backtrace"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6189658,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: render_loop.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >)
// 候选参数类型: ["ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >", "ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<winit::monitor::VideoMode>,ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>", "ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::render::render_config::LastKnownGraphicsDevice>,ref$<country_core::render::render_config::LastKnownGraphicsDevice>)
// 候选参数类型: ["ref$<country_core::render::render_config::LastKnownGraphicsDevice>", "ref$<country_core::render::render_config::LastKnownGraphicsDevice>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)
// 候选参数类型: ["ref$<tiny_glade::gui::settings::RefreshRateMillihertz>", "ref$<tiny_glade::gui::settings::RefreshRateMillihertz>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::SmolRingBuffer::push
// 信息 ID: 14504f9377cce42ee27e71e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=62420; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=62492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_loading_screen_render_graph
// 信息 ID: 5916239851bba2ffdd45aa0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=52332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0
// 信息 ID: d3300dadb2a79b574c1ff0e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_loading_screen_render_graph", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0 <- rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; module=107, inline_record=6832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0::closure$0
// 信息 ID: a7a41628801e2ab8c95e4027
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_loading_screen_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; module=107, inline_record=8720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph
// 信息 ID: 5f6be48add8bb80ef9aa204b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=48676; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=54868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0
// 信息 ID: f4abb5dfa15d3d7be38f5282
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::render_loop::compile_main_render_graph::closure_env$0 <- alloc::vec::spec_extend::impl$1::spec_extend<rhapsody::rg::rcache::NodeDebugInfo,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::recorder::RecordedNode>,rhapsody::rg::compile::compile::closure_env$1<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> >,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::render_loop::compile_main_render_graph::closure_env$0 <- rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=107, inline_record=37976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$0
// 信息 ID: 04ae6bb83a1449a6c6603401
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=107, inline_record=46356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$1
// 信息 ID: 1d2db2b8b05e4b1537182096
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=107, inline_record=52464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$3
// 信息 ID: 82d90ec52127d4d7b40e9994
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=107, inline_record=61380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$4
// 信息 ID: c52d3420e464c0008b022496
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=107, inline_record=61504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$6
// 信息 ID: ceb5ee9f3e2b1030cc48df9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6 <- rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6 <- rhapsody::rg::recorder::impl$8::new_in::drop_impl<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6 <- rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6>; module=143, inline_record=20312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0::explain
// 信息 ID: 558fae61f2af0417e77b2cde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x399b01 -> 0x56dc90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x399b4e -> 0x56dc90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x399b87 -> 0x56dc90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::create_cube_mesh_index_buffer
// 信息 ID: b3914f02cf59e8c014eaebfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=18364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::handle_screenshot_error
// 信息 ID: 2019f16884479920aac3f707
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a458 -> 0x56db80 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a5e4 -> 0x56db80 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a740 -> 0x56db80 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a85c -> 0x56db80 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=107, inline_record=64436; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::impl$3::eq
// 信息 ID: 4eb0afe5f65a17a71bd3e899
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=46816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render
// 信息 ID: a98fe4a7c84ba34a59a2cbd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x57fe3e -> 0x56dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0x5e725a -> 0x56dd80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$0
// 信息 ID: 5b4d71ca2fc0b270796a6fc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=46728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$1
// 信息 ID: 22c05d0e518bec45fd24c26e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::render_loop::render::closure_env$1 <- core::option::Option::map<ref$<ref_mut$<rhapsody::raw::swapchain::Swapchain> >,tuple$<ash::vk::enums::Format,array$<u16,2> >,tiny_glade::render_loop::render::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::option::Option::map<ref$<ref_mut$<rhapsody::raw::swapchain::Swapchain> >,tuple$<ash::vk::enums::Format,array$<u16,2> >,tiny_glade::render_loop::render::closure_env$1>; module=202, inline_record=68388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$2
// 信息 ID: 7f30174a8b234e059019c3ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=55212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$3
// 信息 ID: 155f3c88d146ce3df1f54717
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::render_loop::render::closure_env$3 <- rhapsody::rg::compile::compile<tuple$<>,rhapsody::rg::executor::impl$12::present::closure$0::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,rhapsody::rg::executor::impl$9::execute_and_present::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,tiny_glade::render_loop::render::closure_env$3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::render_loop::render::closure_env$3 <- rhapsody::raw::swapchain::Swapchain::with_next_image<rhapsody::rg::executor::impl$12::present::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,rhapsody::rg::executor::impl$9::execute_and_present::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,tiny_glade::render_loop::render::closure_env$3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::render_loop::render::closure_env$3 <- rhapsody::rg::executor::GraphExecutor::execute_and_present<tiny_glade::run_game::GraphExecutorBevy,tiny_glade::render_loop::render::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<tuple$<>,rhapsody::rg::executor::impl$12::present::closure$0::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,rhapsody::rg::executor::impl$9::execute_and_present::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,tiny_glade::render_loop::render::closure_env$3> > >; module=107, inline_record=82456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$4
// 信息 ID: 91e17a88976e8d46de03ae73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=62612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$5
// 信息 ID: e778856fd9a1cec5d9902083
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=62784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world
// 信息 ID: c73faa955e6915683a75ffd1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x399cdd -> 0x56a700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$0
// 信息 ID: 58c1b93ccedf675297773148
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=2324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$1
// 信息 ID: 3c599951be52425e92ed5474
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=2924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$2
// 信息 ID: 5bac25c38b16a5c4d90ffe40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=5408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$3
// 信息 ID: 2f62a1fab2a61de67dcaca27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=7652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$3::closure$0
// 信息 ID: 175a0e40a219c7268a3be631
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=7952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$3::closure$0::closure$0
// 信息 ID: 223e6de9a1805f3a65adfd56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=8260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$3::closure$0::closure$1
// 信息 ID: 4206d75d5d18746a2353d460
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=8900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$4
// 信息 ID: 7dab9276d0ca86ba6a596998
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=11896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$4::closure$0
// 信息 ID: 6eb4617f20d365ab377772a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=12208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$4::closure$0::closure$0
// 信息 ID: a771d703a894f9d6467a67a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=12516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$4::closure$0::closure$1
// 信息 ID: b58206f50c59a9d53ad0f115
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=13152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$5
// 信息 ID: 0ff105183df53c65e22960b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=14160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$6
// 信息 ID: e8d90c1710b63ce620094367
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=18252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$6::closure$0
// 信息 ID: 0c2feb304ce889ee6606880d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::render_loop::render_world::closure$6::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::render_loop::render_world::closure$6::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0>; module=143, inline_record=18552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$6::closure$1
// 信息 ID: f6935569fdcf94ef1009d6a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=19192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$6::closure$2
// 信息 ID: 2de36f563f69f85572b684ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=19840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$7
// 信息 ID: c6eef46a600dda4a661517d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=22932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$7::closure$0
// 信息 ID: 7d56be479744d7b81f6f40b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=23824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$7::closure$1
// 信息 ID: f88f098e6c0d1e537c414a49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=27812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$7::closure$2
// 信息 ID: 6a035f3a8d5f70ae7905eb75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=28380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$7::closure$3
// 信息 ID: d00757f99bd2a6297c5c4a57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=30660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world::closure$8
// 信息 ID: 58cd01e5286252501a7ddf39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "tiny_glade::render_loop::render_world", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=202, inline_record=33280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
