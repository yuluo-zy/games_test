// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: fdfd56fd738548e32a64cd97444d2c2bf02ad5229533dc46a59e06df2096e690
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rasterize_buildings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 566, 1643, 1674
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
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
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":566,"record_offset":39112,"rva":11759328,"va":5380468448}
// 编译实例 RVAs: 0xb36ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2994997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e835b32d00","size":5,"soft_fail":false,"target_va":5383464432,"va":5380469430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3002694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e846d12d00","size":5,"soft_fail":false,"target_va":5383472768,"va":5380470069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4792223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e861e0b6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380470922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=2
//     target_aliases=49 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8005848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e828d785ff","size":5,"soft_fail":false,"target_va":5372465520,"va":5380471363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2910984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8f894d3ff","size":5,"soft_fail":false,"target_va":5377560512,"va":5380471491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b20870
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-91818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e85699feff","size":5,"soft_fail":false,"target_va":5380376688,"va":5380468501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1108407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8b7e91000","size":5,"soft_fail":false,"target_va":5381577744,"va":5380469332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415384a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10490146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e82211a000","size":5,"soft_fail":false,"target_va":5390959776,"va":5380469625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141547800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10552197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e88503a100","size":5,"soft_fail":false,"target_va":5391022080,"va":5380469878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141608f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11345551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e88f1ead00","size":5,"soft_fail":false,"target_va":5391814416,"va":5380468860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176b770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12795742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e85e3fc300","size":5,"soft_fail":false,"target_va":5393266544,"va":5380470797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12819065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8799ac300","size":5,"soft_fail":false,"target_va":5393288768,"va":5380469698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177a260
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12856009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8c92ac400","size":5,"soft_fail":false,"target_va":5393326688,"va":5380470674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177ce30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12868108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e80c5ac400","size":5,"soft_fail":false,"target_va":5393337904,"va":5380469791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14178fcc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12945089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8c186c500","size":5,"soft_fail":false,"target_va":5393415360,"va":5380470266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36587234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8e2462e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5380469817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36152334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e80ea42702","size":5,"soft_fail":false,"target_va":5416621600,"va":5380469261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36151203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8a39f2702","size":5,"soft_fail":false,"target_va":5416622240,"va":5380471032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36150394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e87a9c2702","size":5,"soft_fail":false,"target_va":5416621632,"va":5380471233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1190287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e871d6edff","size":5,"soft_fail":false,"target_va":5379279280,"va":5380469562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36103950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e80ee72602","size":5,"soft_fail":false,"target_va":5416574944,"va":5380470989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3514542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8aea03500","size":5,"soft_fail":false,"target_va":5383985360,"va":5380470813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3250360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8b8983100","size":5,"soft_fail":false,"target_va":5383719888,"va":5380469523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3078148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e804f82e00","size":5,"soft_fail":false,"target_va":5383548144,"va":5380469991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3066801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8b1cb2e00","size":5,"soft_fail":false,"target_va":5383536672,"va":5380469866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3069415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380468448,"raw_hex":"e8e7d52e00","size":5,"soft_fail":false,"target_va":5383539152,"va":5380469732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5380468448,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5380469354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":566,"record_offset":41460,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":45632,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":566,"record_offset":46420,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":566,"record_offset":44748,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":566,"record_offset":45076,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":566,"record_offset":44808,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":27014,"source_line":366,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":566,"record_offset":39328,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":566,"record_offset":45344,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":566,"record_offset":45300,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":566,"record_offset":45412,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":41032,"parent_record_offset":39112,"ancestor_record_offset":39112,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":566,"record_offset":46316,"parent_record_offset":46292,"ancestor_record_offset":39112,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":566,"record_offset":46340,"parent_record_offset":46316,"ancestor_record_offset":39112,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":566,"record_offset":46364,"parent_record_offset":46340,"ancestor_record_offset":39112,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":566,"record_offset":46292,"parent_record_offset":46200,"ancestor_record_offset":39112,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":566,"record_offset":40676,"parent_record_offset":40652,"ancestor_record_offset":39112,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":566,"record_offset":46228,"parent_record_offset":46200,"ancestor_record_offset":39112,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":566,"record_offset":46200,"parent_record_offset":46176,"ancestor_record_offset":39112,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":566,"record_offset":40724,"parent_record_offset":40700,"ancestor_record_offset":39112,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":566,"record_offset":40700,"parent_record_offset":40676,"ancestor_record_offset":39112,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":566,"record_offset":46176,"parent_record_offset":46152,"ancestor_record_offset":39112,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":40652,"parent_record_offset":40628,"ancestor_record_offset":39112,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":40628,"parent_record_offset":40588,"ancestor_record_offset":39112,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":41484,"parent_record_offset":41460,"ancestor_record_offset":39112,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":40588,"parent_record_offset":40564,"ancestor_record_offset":39112,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":566,"record_offset":39384,"parent_record_offset":39356,"ancestor_record_offset":39112,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":566,"record_offset":40948,"parent_record_offset":40924,"ancestor_record_offset":39112,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":566,"record_offset":40924,"parent_record_offset":40900,"ancestor_record_offset":39112,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":566,"record_offset":40872,"parent_record_offset":40840,"ancestor_record_offset":39112,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":566,"record_offset":40816,"parent_record_offset":40308,"ancestor_record_offset":39112,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":566,"record_offset":40840,"parent_record_offset":40816,"ancestor_record_offset":39112,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":566,"record_offset":40336,"parent_record_offset":40308,"ancestor_record_offset":39112,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":566,"record_offset":40900,"parent_record_offset":40840,"ancestor_record_offset":39112,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":566,"record_offset":40384,"parent_record_offset":40360,"ancestor_record_offset":39112,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":566,"record_offset":40360,"parent_record_offset":40336,"ancestor_record_offset":39112,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":566,"record_offset":40564,"parent_record_offset":40540,"ancestor_record_offset":39112,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":566,"record_offset":39356,"parent_record_offset":39328,"ancestor_record_offset":39112,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":566,"record_offset":40308,"parent_record_offset":39356,"ancestor_record_offset":39112,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":566,"record_offset":39408,"parent_record_offset":39384,"ancestor_record_offset":39112,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":566,"record_offset":40056,"parent_record_offset":40032,"ancestor_record_offset":39112,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":566,"record_offset":40084,"parent_record_offset":40056,"ancestor_record_offset":39112,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":41320,"parent_record_offset":41292,"ancestor_record_offset":39112,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":39624,"parent_record_offset":39600,"ancestor_record_offset":39112,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":39708,"parent_record_offset":39684,"ancestor_record_offset":39112,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":566,"record_offset":46260,"parent_record_offset":46228,"ancestor_record_offset":39112,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":566,"record_offset":39856,"parent_record_offset":39828,"ancestor_record_offset":39112,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":566,"record_offset":39828,"parent_record_offset":39800,"ancestor_record_offset":39112,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":566,"record_offset":40476,"parent_record_offset":40412,"ancestor_record_offset":39112,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":566,"record_offset":40500,"parent_record_offset":40476,"ancestor_record_offset":39112,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":566,"record_offset":40540,"parent_record_offset":40360,"ancestor_record_offset":39112,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":566,"record_offset":41000,"parent_record_offset":39328,"ancestor_record_offset":39112,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":566,"record_offset":41264,"parent_record_offset":41232,"ancestor_record_offset":39112,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":39648,"parent_record_offset":39624,"ancestor_record_offset":39112,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":46152,"parent_record_offset":46128,"ancestor_record_offset":39112,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":39972,"parent_record_offset":39948,"ancestor_record_offset":39112,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":566,"record_offset":44972,"parent_record_offset":44904,"ancestor_record_offset":39112,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":566,"record_offset":40412,"parent_record_offset":40384,"ancestor_record_offset":39112,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":566,"record_offset":44848,"parent_record_offset":44808,"ancestor_record_offset":39112,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":566,"record_offset":40448,"parent_record_offset":40412,"ancestor_record_offset":39112,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":566,"record_offset":40768,"parent_record_offset":40588,"ancestor_record_offset":39112,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":45812,"parent_record_offset":45688,"ancestor_record_offset":39112,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":45016,"parent_record_offset":44808,"ancestor_record_offset":39112,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":45836,"parent_record_offset":45812,"ancestor_record_offset":39112,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":45040,"parent_record_offset":45016,"ancestor_record_offset":39112,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":566,"record_offset":44772,"parent_record_offset":44748,"ancestor_record_offset":39112,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":566,"record_offset":45108,"parent_record_offset":45076,"ancestor_record_offset":39112,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":566,"record_offset":45268,"parent_record_offset":45076,"ancestor_record_offset":39112,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":566,"record_offset":44876,"parent_record_offset":44808,"ancestor_record_offset":39112,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":566,"record_offset":44904,"parent_record_offset":44876,"ancestor_record_offset":39112,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::gauss_blur_building_mask::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":566,"record_offset":45380,"parent_record_offset":45344,"ancestor_record_offset":39112,"inlinee":35618,"source_line":8676,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::rasterize_buildings::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":566,"record_offset":45204,"parent_record_offset":45168,"ancestor_record_offset":39112,"inlinee":32102,"source_line":31433,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::rasterize_buildings_animate::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":566,"record_offset":45572,"parent_record_offset":45536,"ancestor_record_offset":39112,"inlinee":28770,"source_line":31526,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs
//     evidence={"module":566,"record_offset":45136,"parent_record_offset":45076,"ancestor_record_offset":39112,"inlinee":35616,"source_line":78,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":566,"record_offset":40032,"parent_record_offset":40008,"ancestor_record_offset":39112,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":566,"record_offset":39768,"parent_record_offset":39740,"ancestor_record_offset":39112,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":566,"record_offset":39800,"parent_record_offset":39768,"ancestor_record_offset":39112,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":566,"record_offset":39740,"parent_record_offset":39520,"ancestor_record_offset":39112,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":39600,"parent_record_offset":39520,"ancestor_record_offset":39112,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":41292,"parent_record_offset":41232,"ancestor_record_offset":39112,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":40160,"parent_record_offset":39520,"ancestor_record_offset":39112,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":41348,"parent_record_offset":41292,"ancestor_record_offset":39112,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":39684,"parent_record_offset":39520,"ancestor_record_offset":39112,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":566,"record_offset":39444,"parent_record_offset":39408,"ancestor_record_offset":39112,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":566,"record_offset":41084,"parent_record_offset":41056,"ancestor_record_offset":39112,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":566,"record_offset":40008,"parent_record_offset":39900,"ancestor_record_offset":39112,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":39948,"parent_record_offset":39924,"ancestor_record_offset":39112,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":40252,"parent_record_offset":39520,"ancestor_record_offset":39112,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":41232,"parent_record_offset":41140,"ancestor_record_offset":39112,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":39924,"parent_record_offset":39900,"ancestor_record_offset":39112,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":39496,"parent_record_offset":39472,"ancestor_record_offset":39112,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":39472,"parent_record_offset":39444,"ancestor_record_offset":39112,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":41112,"parent_record_offset":41084,"ancestor_record_offset":39112,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":41204,"parent_record_offset":41140,"ancestor_record_offset":39112,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":41176,"parent_record_offset":41140,"ancestor_record_offset":39112,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":39520,"parent_record_offset":39496,"ancestor_record_offset":39112,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":41140,"parent_record_offset":41112,"ancestor_record_offset":39112,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":39572,"parent_record_offset":39520,"ancestor_record_offset":39112,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":566,"record_offset":39900,"parent_record_offset":39520,"ancestor_record_offset":39112,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":566,"record_offset":40132,"parent_record_offset":39520,"ancestor_record_offset":39112,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":566,"record_offset":45660,"parent_record_offset":45632,"ancestor_record_offset":39112,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":566,"record_offset":44452,"parent_record_offset":44428,"ancestor_record_offset":39112,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":566,"record_offset":45724,"parent_record_offset":45688,"ancestor_record_offset":39112,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":566,"record_offset":45688,"parent_record_offset":45660,"ancestor_record_offset":39112,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":566,"record_offset":45752,"parent_record_offset":45724,"ancestor_record_offset":39112,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":566,"record_offset":44944,"parent_record_offset":44904,"ancestor_record_offset":39112,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":566,"record_offset":45168,"parent_record_offset":45136,"ancestor_record_offset":39112,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":566,"record_offset":45236,"parent_record_offset":45136,"ancestor_record_offset":39112,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":41056,"parent_record_offset":41032,"ancestor_record_offset":39112,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DB9
// 首条位置: {"module":1643,"inlinee":81337}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1643,"record_offset":530884,"parent_record_offset":530840,"ancestor_record_offset":530840,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":530908,"parent_record_offset":530884,"ancestor_record_offset":530840,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8B20
// 首条位置: {"module":566,"inlinee":35616}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":566,"record_offset":45168,"parent_record_offset":45136,"ancestor_record_offset":45136,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":566,"record_offset":45236,"parent_record_offset":45136,"ancestor_record_offset":45136,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::rasterize_buildings::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":566,"record_offset":45204,"parent_record_offset":45168,"ancestor_record_offset":45136,"inlinee":32102,"source_line":31433,"archive":"_pdb/C/modules/00566.jsonl.gz"}
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
// 完整结构及原始类型记录: rasterize_buildings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$7::new_storage_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::render::ssbo::impl$7::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rasterize_buildings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: rasterize_buildings.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
// [PDB Shader 类型/入口] {"shader_family":"country_core/rasterize_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rasterize_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":566,"record_offset":45204,"parent_record_offset":45168,"ancestor_record_offset":39112,"inlinee":32102,"source_line":31433,"archive":"_pdb/C/modules/00566.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/gauss_blur_building_mask","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_building_mask::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":566,"record_offset":45380,"parent_record_offset":45344,"ancestor_record_offset":39112,"inlinee":35618,"source_line":8676,"archive":"_pdb/C/modules/00566.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/rasterize_buildings_animate","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rasterize_buildings_animate::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":566,"record_offset":45572,"parent_record_offset":45536,"ancestor_record_offset":39112,"inlinee":28770,"source_line":31526,"archive":"_pdb/C/modules/00566.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$1
// [PDB Shader 类型/入口] {"shader_family":"country_core/rasterize_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rasterize_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":566,"record_offset":45204,"parent_record_offset":45168,"ancestor_record_offset":45136,"inlinee":32102,"source_line":31433,"archive":"_pdb/C/modules/00566.jsonl.gz"}}
// [Shader 映射] {"shader_id":"0ffd223dc45ac89e4d75e559","relative_path":"country-core/compiled/rasterize_buildings.precomp","sha256":"50a489319147fe69d52b56f33f3238d2243cfffb3ac9d7f48c10b1259d4f9d88","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings","hint":{"shader_family":"country_core/rasterize_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rasterize_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":566,"record_offset":45204,"parent_record_offset":45168,"ancestor_record_offset":39112,"inlinee":32102,"source_line":31433,"archive":"_pdb/C/modules/00566.jsonl.gz"}},"shader_uri":"country_core/rasterize_buildings:main"}
// [Shader 映射] {"shader_id":"10ba95054f7304095534bf14","relative_path":"country-core/compiled/gauss_blur_building_mask.precomp","sha256":"19211162cae5eeb2561f2ef751f303ff819e0a15a6f0b053f3a4f2156e63041c","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings","hint":{"shader_family":"country_core/gauss_blur_building_mask","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_building_mask::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":566,"record_offset":45380,"parent_record_offset":45344,"ancestor_record_offset":39112,"inlinee":35618,"source_line":8676,"archive":"_pdb/C/modules/00566.jsonl.gz"}},"shader_uri":"country_core/gauss_blur_building_mask:main"}
// [Shader 映射] {"shader_id":"ca1b1f0a7334e0fa96ca6044","relative_path":"country-core/compiled/rasterize_buildings_animate.precomp","sha256":"ab0c43dd9e267be3d0c36fdedcfa86d6832086b76b9fc23f28ea56b2fd464e53","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings","hint":{"shader_family":"country_core/rasterize_buildings_animate","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rasterize_buildings_animate::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":566,"record_offset":45572,"parent_record_offset":45536,"ancestor_record_offset":39112,"inlinee":28770,"source_line":31526,"archive":"_pdb/C/modules/00566.jsonl.gz"}},"shader_uri":"country_core/rasterize_buildings_animate:main"}
// [Shader 映射] {"shader_id":"0ffd223dc45ac89e4d75e559","relative_path":"country-core/compiled/rasterize_buildings.precomp","sha256":"50a489319147fe69d52b56f33f3238d2243cfffb3ac9d7f48c10b1259d4f9d88","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$1","hint":{"shader_family":"country_core/rasterize_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rasterize_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":566,"record_offset":45204,"parent_record_offset":45168,"ancestor_record_offset":45136,"inlinee":32102,"source_line":31433,"archive":"_pdb/C/modules/00566.jsonl.gz"}},"shader_uri":"country_core/rasterize_buildings:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/rasterize_buildings.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[55]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":45,"name":"globalParams","storage_class":"Uniform","type":{"id":44,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,41]},"decorations":[],"pointee":{"id":41,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[43,43,5]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"out_mask_dims","type_id":43,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"out_mask2_dims","type_id":43,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"buildings_count","type_id":5,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":104,"name":"buildings_buffer","storage_class":"Uniform","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,101]},"decorations":[],"pointee":{"id":101,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[102]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":102,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":353,"name":"out_mask","storage_class":"UniformConstant","type":{"id":352,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,350]},"decorations":[],"pointee":{"id":350,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":405,"name":"out_mask2","storage_class":"UniformConstant","type":{"id":404,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,402]},"decorations":[],"pointee":{"id":402,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,2,15]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":55,"name":"unknown","storage_class":"Input","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[42,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/gauss_blur_building_mask.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[58]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":65,"name":"globalParams","storage_class":"Uniform","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,63]},"decorations":[],"pointee":{"id":63,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[59,59,5]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"building_mask_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"out_building_mask_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"is_horizontal","type_id":5,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":95,"name":"building_mask","storage_class":"UniformConstant","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,92]},"decorations":[],"pointee":{"id":92,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":174,"name":"out_building_mask","storage_class":"UniformConstant","type":{"id":173,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,171]},"decorations":[],"pointee":{"id":171,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"unknown","storage_class":"Input","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,55]},"decorations":[],"pointee":{"id":55,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[54,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/rasterize_buildings_animate.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[19]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"out_mask","storage_class":"UniformConstant","type":{"id":29,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,27]},"decorations":[],"pointee":{"id":27,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":70,"name":"frame_constants","storage_class":"Uniform","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,67]},"decorations":[],"pointee":{"id":67,"name":"FrameConstants_natural","record":{"opcode":"OpTypeStruct","operands":[31,31,31,31,31,31,31,31,6,6,6,15,20,15,6,68,6,68,15,68,15,68,6,31,6,15,20,6,15,15,15,68,6,15,15,15,15,15,6,6,6,31,68,6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":31,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":31,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":31,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"time_since_startup_secs","type_id":6,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":6,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":15,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"world_render_resolution","type_id":20,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":15,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":6,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":6,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":68,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":15,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":68,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":15,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"dir_to_primary_directional_light","type_id":68,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"main_view_lod_scale","type_id":6,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":23,"name":"main_view_eye_position","type_id":31,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":24,"name":"fog_relative_extinction","type_id":6,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":25,"name":"highlighted_transient_stair_edge_id","type_id":15,"decorations":[{"decoration":"Offset","operands":[244]}]},{"index":26,"name":"swapchain_dims","type_id":20,"decorations":[{"decoration":"Offset","operands":[248]}]},{"index":27,"name":"swapchain_aspect_ratio","type_id":6,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":28,"name":"is_photomode","type_id":15,"decorations":[{"decoration":"Offset","operands":[260]}]},{"index":29,"name":"is_topdown_mode","type_id":15,"decorations":[{"decoration":"Offset","operands":[264]}]},{"index":30,"name":"is_firstperson_mode","type_id":15,"decorations":[{"decoration":"Offset","operands":[268]}]},{"index":31,"name":"brush_position","type_id":68,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":32,"name":"eraser_size_multiplier","type_id":6,"decorations":[{"decoration":"Offset","operands":[284]}]},{"index":33,"name":"roof_highlighted_slot_id","type_id":15,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":34,"name":"roof_highlight_mode","type_id":15,"decorations":[{"decoration":"Offset","operands":[292]}]},{"index":35,"name":"wall_highlighted_slot_id","type_id":15,"decorations":[{"decoration":"Offset","operands":[296]}]},{"index":36,"name":"wall_highlight_mode","type_id":15,"decorations":[{"decoration":"Offset","operands":[300]}]},{"index":37,"name":"highlight_bricks_to_delete","type_id":15,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":38,"name":"wind_strength","type_id":6,"decorations":[{"decoration":"Offset","operands":[308]}]},{"index":39,"name":"tree_canopy_cone_angle_mult","type_id":6,"decorations":[{"decoration":"Offset","operands":[312]}]},{"index":40,"name":"tree_canopy_max_cone_cull_dist_range","type_id":6,"decorations":[{"decoration":"Offset","operands":[316]}]},{"index":41,"name":"debug_tweakers","type_id":31,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":42,"name":"ground_fog","type_id":68,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":43,"name":"moonswap","type_id":6,"decorations":[{"decoration":"Offset","operands":[348]}]}]}},"descriptor_set":3,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[3]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"unknown","storage_class":"Input","type":{"id":18,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,16]},"decorations":[],"pointee":{"id":16,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[15,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679528,"position_among_mapped_render_targets":18,"mapped_render_target_count":74,"previous_callsite":5679517,"next_callsite":5679539,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rasterize_buildings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 31; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::NewReplayCommandEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::NewReplayCommandEvent>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::NewReplayCommandEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::DebugHistoryMsg>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::NewReplayCommandEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::DebugHistoryMsg>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::NewReplayCommandEvent>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::DebugHistoryMsg>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::NewReplayCommandEvent>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>", "bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
// 信息 ID: 5c31adaef41e6debcfd082af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a9a8 -> 0xb36ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$0
// 信息 ID: 3af0e3865bbd35c30f10efe2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >; module=1643, inline_record=530840; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >; module=1643, inline_record=532120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$1
// 信息 ID: 549803698f69daa7361c2936
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings; module=566, inline_record=45136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
