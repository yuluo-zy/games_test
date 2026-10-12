// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/image_baker.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c9d9e1d2abcf8a727236a15282ff4f29dcb2a10f97590d73216c7a66e924173d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: image_baker.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 559, 1253
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::image_baker::PENDING_IMAGE_BAKES::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x88F7
// 首条位置: {"module":559,"inlinee":35063}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":559,"record_offset":117892,"parent_record_offset":117872,"ancestor_record_offset":117872,"inlinee":8490,"source_line":149,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::image_baker::bake_grass2_world_albedo_tint_from_env
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":8732,"rva":11522208,"va":5380231328}
// 编译实例 RVAs: 0xafd0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4561727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e83f9b4500","size":5,"soft_fail":false,"target_va":5384793264,"va":5380231532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::export
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3260016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e870be3100","size":5,"soft_fail":false,"target_va":5383492416,"va":5380232395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::import_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3260154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8fabe3100","size":5,"soft_fail":false,"target_va":5383492272,"va":5380232113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::import
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3262211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e803c73100","size":5,"soft_fail":false,"target_va":5383494288,"va":5380232072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4552866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e85e87baff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380231565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10927766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e86a4159ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5380233345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f50
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10459373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8136760ff","size":5,"soft_fail":false,"target_va":5369773904,"va":5380233272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1639862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e84afae6ff","size":5,"soft_fail":false,"target_va":5378593552,"va":5380233409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098ee20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1501407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e82117e9ff","size":5,"soft_fail":false,"target_va":5378731552,"va":5380232954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13056589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e84d3ac700","size":5,"soft_fail":false,"target_va":5393288768,"va":5380232174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141797620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13214140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8bca1c900","size":5,"soft_fail":false,"target_va":5393446432,"va":5380232287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36118961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8b1212702","size":5,"soft_fail":false,"target_va":5416351504,"va":5380232538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36150208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8c09b2702","size":5,"soft_fail":false,"target_va":5416383056,"va":5380232843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36824738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8a2e63102","size":5,"soft_fail":false,"target_va":5417057056,"va":5380232313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36825337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8f9e83102","size":5,"soft_fail":false,"target_va":5417057088,"va":5380231746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36388886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e816402b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380232709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36388809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8c93f2b02","size":5,"soft_fail":false,"target_va":5416621632,"va":5380232818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-953108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8ec74f1ff","size":5,"soft_fail":false,"target_va":5379278928,"va":5380232031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36343953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e891902a02","size":5,"soft_fail":false,"target_va":5416576832,"va":5380232874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36343238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8c68d2a02","size":5,"soft_fail":false,"target_va":5416576144,"va":5380232901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3752393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8c9413900","size":5,"soft_fail":false,"target_va":5383984848,"va":5380232450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3752726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e816433900","size":5,"soft_fail":false,"target_va":5383985360,"va":5380232629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3256154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e85aaf3100","size":5,"soft_fail":false,"target_va":5383487824,"va":5380231665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_mapped_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3490340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e824423500","size":5,"soft_fail":false,"target_va":5383722048,"va":5380231703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3304318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e87e6b3200","size":5,"soft_fail":false,"target_va":5383536672,"va":5380232349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3306932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8b4753200","size":5,"soft_fail":false,"target_va":5383539152,"va":5380232215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4480749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8ed5e4400","size":5,"soft_fail":false,"target_va":5384712144,"va":5380231390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3092719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380231328,"raw_hex":"e8ef302f00","size":5,"soft_fail":false,"target_va":5383324736,"va":5380232012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":12620,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":9244,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":559,"record_offset":10116,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":10160,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":9076,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":9020,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":7925,"source_line":918,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":559,"record_offset":11924,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":559,"record_offset":10216,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":559,"record_offset":12464,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":12220,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":559,"record_offset":12068,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::device
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":559,"record_offset":10088,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":10362,"source_line":426,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphResourceApi::export
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":559,"record_offset":12164,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":13028,"source_line":105,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphResourceApi::import
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":559,"record_offset":11952,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":12355,"source_line":79,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::import_mut_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":559,"record_offset":12040,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":12379,"source_line":1321,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":559,"record_offset":12136,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":559,"record_offset":8992,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":10542,"source_line":222,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":559,"record_offset":9160,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$40::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":559,"record_offset":10032,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":32372,"source_line":1992,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":9516,"parent_record_offset":9492,"ancestor_record_offset":8732,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":8732,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":9564,"parent_record_offset":9540,"ancestor_record_offset":8732,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":9492,"parent_record_offset":9436,"ancestor_record_offset":8732,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":8732,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":9464,"parent_record_offset":9436,"ancestor_record_offset":8732,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":9436,"parent_record_offset":9412,"ancestor_record_offset":8732,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":11616,"parent_record_offset":11592,"ancestor_record_offset":8732,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":8732,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":9412,"parent_record_offset":9388,"ancestor_record_offset":8732,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":12692,"parent_record_offset":12648,"ancestor_record_offset":8732,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":8732,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":11520,"parent_record_offset":11480,"ancestor_record_offset":8732,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":12648,"parent_record_offset":12620,"ancestor_record_offset":8732,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":11480,"parent_record_offset":11456,"ancestor_record_offset":8732,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":559,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":8732,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":559,"record_offset":11840,"parent_record_offset":11816,"ancestor_record_offset":8732,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":559,"record_offset":11816,"parent_record_offset":11792,"ancestor_record_offset":8732,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":559,"record_offset":11764,"parent_record_offset":11732,"ancestor_record_offset":8732,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":559,"record_offset":11708,"parent_record_offset":11200,"ancestor_record_offset":8732,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":559,"record_offset":11732,"parent_record_offset":11708,"ancestor_record_offset":8732,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":559,"record_offset":11228,"parent_record_offset":11200,"ancestor_record_offset":8732,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":559,"record_offset":11792,"parent_record_offset":11732,"ancestor_record_offset":8732,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":559,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":8732,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":559,"record_offset":11252,"parent_record_offset":11228,"ancestor_record_offset":8732,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":559,"record_offset":11456,"parent_record_offset":11432,"ancestor_record_offset":8732,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":559,"record_offset":10248,"parent_record_offset":10216,"ancestor_record_offset":8732,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":559,"record_offset":11200,"parent_record_offset":10248,"ancestor_record_offset":8732,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":559,"record_offset":10300,"parent_record_offset":10276,"ancestor_record_offset":8732,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":559,"record_offset":10948,"parent_record_offset":10924,"ancestor_record_offset":8732,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":559,"record_offset":10976,"parent_record_offset":10948,"ancestor_record_offset":8732,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":8732,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":10516,"parent_record_offset":10492,"ancestor_record_offset":8732,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":10600,"parent_record_offset":10576,"ancestor_record_offset":8732,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":559,"record_offset":10748,"parent_record_offset":10720,"ancestor_record_offset":8732,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":559,"record_offset":10720,"parent_record_offset":10692,"ancestor_record_offset":8732,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":559,"record_offset":11368,"parent_record_offset":11304,"ancestor_record_offset":8732,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":11392,"parent_record_offset":11368,"ancestor_record_offset":8732,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":11432,"parent_record_offset":11252,"ancestor_record_offset":8732,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":11892,"parent_record_offset":10216,"ancestor_record_offset":8732,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":10540,"parent_record_offset":10516,"ancestor_record_offset":8732,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":9268,"parent_record_offset":9244,"ancestor_record_offset":8732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":8732,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":12812,"parent_record_offset":12648,"ancestor_record_offset":8732,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":8732,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":8732,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":11304,"parent_record_offset":11276,"ancestor_record_offset":8732,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":559,"record_offset":11340,"parent_record_offset":11304,"ancestor_record_offset":8732,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":559,"record_offset":11660,"parent_record_offset":11480,"ancestor_record_offset":8732,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$2::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":559,"record_offset":10184,"parent_record_offset":10160,"ancestor_record_offset":8732,"inlinee":19477,"source_line":31,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12384,"parent_record_offset":12360,"ancestor_record_offset":8732,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12968,"parent_record_offset":12904,"ancestor_record_offset":8732,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12528,"parent_record_offset":12492,"ancestor_record_offset":8732,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":9044,"parent_record_offset":9020,"ancestor_record_offset":8732,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12552,"parent_record_offset":12528,"ancestor_record_offset":8732,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12408,"parent_record_offset":12384,"ancestor_record_offset":8732,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::grass2::bake_world_albedo_tint::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":559,"record_offset":12104,"parent_record_offset":12068,"ancestor_record_offset":8732,"inlinee":34991,"source_line":9330,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":559,"record_offset":10924,"parent_record_offset":10900,"ancestor_record_offset":8732,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":559,"record_offset":10660,"parent_record_offset":10632,"ancestor_record_offset":8732,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":559,"record_offset":10692,"parent_record_offset":10660,"ancestor_record_offset":8732,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":559,"record_offset":10632,"parent_record_offset":10412,"ancestor_record_offset":8732,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":559,"record_offset":10492,"parent_record_offset":10412,"ancestor_record_offset":8732,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":559,"record_offset":11052,"parent_record_offset":10412,"ancestor_record_offset":8732,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":559,"record_offset":10576,"parent_record_offset":10412,"ancestor_record_offset":8732,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":559,"record_offset":10336,"parent_record_offset":10300,"ancestor_record_offset":8732,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":559,"record_offset":10900,"parent_record_offset":10792,"ancestor_record_offset":8732,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":8732,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":11144,"parent_record_offset":10412,"ancestor_record_offset":8732,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":8732,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":8732,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":10364,"parent_record_offset":10336,"ancestor_record_offset":8732,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":8732,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":10464,"parent_record_offset":10412,"ancestor_record_offset":8732,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":10792,"parent_record_offset":10412,"ancestor_record_offset":8732,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":559,"record_offset":11024,"parent_record_offset":10412,"ancestor_record_offset":8732,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":559,"record_offset":12876,"parent_record_offset":12848,"ancestor_record_offset":8732,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":559,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":8732,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":12248,"parent_record_offset":12220,"ancestor_record_offset":8732,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":12336,"parent_record_offset":12304,"ancestor_record_offset":8732,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":12276,"parent_record_offset":12248,"ancestor_record_offset":8732,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":12304,"parent_record_offset":12276,"ancestor_record_offset":8732,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":559,"record_offset":12492,"parent_record_offset":12464,"ancestor_record_offset":8732,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":559,"record_offset":12904,"parent_record_offset":12876,"ancestor_record_offset":8732,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":559,"record_offset":12584,"parent_record_offset":12492,"ancestor_record_offset":8732,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":559,"record_offset":12940,"parent_record_offset":12904,"ancestor_record_offset":8732,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$0::into_graph_borrow
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":559,"record_offset":12004,"parent_record_offset":11976,"ancestor_record_offset":8732,"inlinee":8323,"source_line":186,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::export_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":559,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":8732,"inlinee":12390,"source_line":1341,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::import_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":559,"record_offset":11976,"parent_record_offset":11952,"ancestor_record_offset":8732,"inlinee":12356,"source_line":1308,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":559,"record_offset":9184,"parent_record_offset":9160,"ancestor_record_offset":8732,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":559,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":8732,"inlinee":22267,"source_line":1970,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":559,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":8732,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::image_baker::finish_pending_image_bakes
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":5412,"rva":11520240,"va":5380229360}
// 编译实例 RVAs: 0xafc8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4459781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8050d4400","size":5,"soft_fail":false,"target_va":5384689616,"va":5380229830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4473592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8f8424400","size":5,"soft_fail":false,"target_va":5384703536,"va":5380229939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4426712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8d88b4300","size":5,"soft_fail":false,"target_va":5384656512,"va":5380229795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4378987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e86bd14200","size":5,"soft_fail":false,"target_va":5384609408,"va":5380230416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4551316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e86c8dbaff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380229999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4551769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8a78bbaff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380230468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11507797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8ab6750ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380229984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1637333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e82b04e7ff","size":5,"soft_fail":false,"target_va":5378593552,"va":5380230880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140978080
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1592765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e843b2e7ff","size":5,"soft_fail":false,"target_va":5378637952,"va":5380230712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ba50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1577950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e822ece7ff","size":5,"soft_fail":false,"target_va":5378652752,"va":5380230697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098ee20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1498892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8f420e9ff","size":5,"soft_fail":false,"target_va":5378731552,"va":5380230439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a0770
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1426603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8553beaff","size":5,"soft_fail":false,"target_va":5378803568,"va":5380230166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a9d110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-391918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e81205faff","size":5,"soft_fail":false,"target_va":5379838224,"va":5380230137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7022603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e80b286b00","size":5,"soft_fail":false,"target_va":5387252560,"va":5380229952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b02e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7026389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8d5366b00","size":5,"soft_fail":false,"target_va":5387256544,"va":5380230150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b06f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7027736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8183c6b00","size":5,"soft_fail":false,"target_va":5387257584,"va":5380229843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: image_baker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36152303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8efa32702","size":5,"soft_fail":false,"target_va":5416383056,"va":5380230748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36389958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e846442b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380230831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4393646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8ae0a4300","size":5,"soft_fail":false,"target_va":5384624080,"va":5380230429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36346043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8bb982a02","size":5,"soft_fail":false,"target_va":5416576832,"va":5380230784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36345329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8f1952a02","size":5,"soft_fail":false,"target_va":5416576144,"va":5380230810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3755397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8854d3900","size":5,"soft_fail":false,"target_va":5383984848,"va":5380229446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3755833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8394f3900","size":5,"soft_fail":false,"target_va":5383985360,"va":5380229522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3524041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380229360,"raw_hex":"e8c9c53500","size":5,"soft_fail":false,"target_va":5383753616,"va":5380229570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*45930223(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5380229360,"raw_hex":"ff15efd6bc02","size":6,"soft_fail":false,"va":5380230947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":6292,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":6320,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":6376,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":7484,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":6044,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":6104,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/image_baker.rs
//     evidence={"module":559,"record_offset":6536,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":34989,"source_line":111,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":559,"record_offset":5872,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":559,"record_offset":7540,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":559,"record_offset":7400,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":5628,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":5412,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":5412,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":5412,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":5412,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7132,"parent_record_offset":7108,"ancestor_record_offset":5412,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":5412,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7060,"parent_record_offset":7032,"ancestor_record_offset":5412,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7932,"parent_record_offset":7876,"ancestor_record_offset":5412,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":6972,"parent_record_offset":6944,"ancestor_record_offset":5412,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":7904,"parent_record_offset":7876,"ancestor_record_offset":5412,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":5412,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":7032,"parent_record_offset":7000,"ancestor_record_offset":5412,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":7000,"parent_record_offset":6972,"ancestor_record_offset":5412,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":5412,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":5412,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":5412,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$69::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":559,"record_offset":7368,"parent_record_offset":6536,"ancestor_record_offset":5412,"inlinee":10407,"source_line":79,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":5412,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":7508,"parent_record_offset":7484,"ancestor_record_offset":5412,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":7192,"parent_record_offset":6884,"ancestor_record_offset":5412,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":5412,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":6448,"parent_record_offset":6376,"ancestor_record_offset":5412,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":6476,"parent_record_offset":6376,"ancestor_record_offset":5412,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":6416,"parent_record_offset":6376,"ancestor_record_offset":5412,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":5412,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":5412,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":6224,"parent_record_offset":6160,"ancestor_record_offset":5412,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":5944,"parent_record_offset":5900,"ancestor_record_offset":5412,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":5412,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":5412,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":5412,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":5412,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] image::codecs::png::PngEncoder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/codecs/png.rs
//     evidence={"module":559,"record_offset":7340,"parent_record_offset":6536,"ancestor_record_offset":5412,"inlinee":6981,"source_line":507,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":559,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":5412,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":559,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":5412,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":559,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":5412,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":559,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":5412,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":559,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":5412,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":5656,"parent_record_offset":5628,"ancestor_record_offset":5412,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":5744,"parent_record_offset":5712,"ancestor_record_offset":5412,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":5684,"parent_record_offset":5656,"ancestor_record_offset":5412,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":559,"record_offset":5712,"parent_record_offset":5684,"ancestor_record_offset":5412,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":559,"record_offset":5900,"parent_record_offset":5872,"ancestor_record_offset":5412,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":559,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":5412,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":559,"record_offset":6008,"parent_record_offset":5900,"ancestor_record_offset":5412,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":559,"record_offset":6196,"parent_record_offset":6160,"ancestor_record_offset":5412,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6672,"parent_record_offset":6620,"ancestor_record_offset":5412,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6644,"parent_record_offset":6620,"ancestor_record_offset":5412,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6704,"parent_record_offset":6536,"ancestor_record_offset":5412,"inlinee":7543,"source_line":638,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":5412,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6820,"parent_record_offset":6704,"ancestor_record_offset":5412,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6620,"parent_record_offset":6536,"ancestor_record_offset":5412,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufwriter::BufWriter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":559,"record_offset":6852,"parent_record_offset":6536,"ancestor_record_offset":5412,"inlinee":16720,"source_line":93,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufwriter::BufWriter::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":559,"record_offset":6884,"parent_record_offset":6852,"ancestor_record_offset":5412,"inlinee":16694,"source_line":121,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":559,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":5412,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":559,"record_offset":6788,"parent_record_offset":6728,"ancestor_record_offset":5412,"inlinee":7546,"source_line":224,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":559,"record_offset":6760,"parent_record_offset":6728,"ancestor_record_offset":5412,"inlinee":7544,"source_line":218,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x88AD
// 首条位置: {"module":559,"inlinee":34989}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::num::impl$69::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":559,"record_offset":7368,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":10407,"source_line":79,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] image::codecs::png::PngEncoder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/codecs/png.rs
//     evidence={"module":559,"record_offset":7340,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":6981,"source_line":507,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6704,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":7543,"source_line":638,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6620,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufwriter::BufWriter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":559,"record_offset":6852,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":16720,"source_line":93,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":6536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":6536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7132,"parent_record_offset":7108,"ancestor_record_offset":6536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":7060,"parent_record_offset":7032,"ancestor_record_offset":6536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":6972,"parent_record_offset":6944,"ancestor_record_offset":6536,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":7032,"parent_record_offset":7000,"ancestor_record_offset":6536,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":7000,"parent_record_offset":6972,"ancestor_record_offset":6536,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":6536,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":6536,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":7192,"parent_record_offset":6884,"ancestor_record_offset":6536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6672,"parent_record_offset":6620,"ancestor_record_offset":6536,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6644,"parent_record_offset":6620,"ancestor_record_offset":6536,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":6536,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":559,"record_offset":6820,"parent_record_offset":6704,"ancestor_record_offset":6536,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufwriter::BufWriter::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":559,"record_offset":6884,"parent_record_offset":6852,"ancestor_record_offset":6536,"inlinee":16694,"source_line":121,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":559,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":6536,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":559,"record_offset":6788,"parent_record_offset":6728,"ancestor_record_offset":6536,"inlinee":7546,"source_line":224,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":559,"record_offset":6760,"parent_record_offset":6728,"ancestor_record_offset":6536,"inlinee":7544,"source_line":218,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112E0
// 首条位置: {"module":1253,"inlinee":70368}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":41080,"parent_record_offset":41060,"ancestor_record_offset":41060,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":41128,"parent_record_offset":41104,"ancestor_record_offset":41060,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":41104,"parent_record_offset":41080,"ancestor_record_offset":41060,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112C7
// 首条位置: {"module":1253,"inlinee":70343}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":30536,"parent_record_offset":30516,"ancestor_record_offset":30516,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":30584,"parent_record_offset":30560,"ancestor_record_offset":30516,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":30560,"parent_record_offset":30536,"ancestor_record_offset":30516,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112DA
// 首条位置: {"module":1253,"inlinee":70362}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":38804,"parent_record_offset":38784,"ancestor_record_offset":38784,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":38852,"parent_record_offset":38828,"ancestor_record_offset":38784,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":38828,"parent_record_offset":38804,"ancestor_record_offset":38784,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
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
// 完整结构及原始类型记录: image_baker.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::image_baker::PENDING_IMAGE_BAKES
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::image_baker::PENDING_IMAGE_BAKES::closure_env$0,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::systems::render_loop::compute_passes::image_baker::PENDING_IMAGE_BAKES::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()>", "constructor": "once_cell::sync::impl$11::force::closure_env$0", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "once_cell::sync::impl$6::get_or_init::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()> >", "constructor": "once_cell::sync::impl$6::get_or_init::closure_env$0", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >", "once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "once_cell::imp::impl$4::initialize::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$6::get_or_init::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()> >,enum2$<once_cell::sync::impl$6::get_or_init::Void> >", "constructor": "once_cell::imp::impl$4::initialize::closure_env$0", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >", "once_cell::sync::impl$6::get_or_init::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()> >", "enum2$<once_cell::sync::impl$6::get_or_init::Void>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<once_cell::imp::impl$4::initialize::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$6::get_or_init::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()> >,enum2$<once_cell::sync::impl$6::get_or_init::Void> >,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["once_cell::imp::impl$4::initialize::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$6::get_or_init::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()> >,enum2$<once_cell::sync::impl$6::get_or_init::Void> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "once_cell::imp::OnceCell::initialize<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$6::get_or_init::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()> >,enum2$<once_cell::sync::impl$6::get_or_init::Void> >", "constructor": "once_cell::imp::OnceCell::initialize", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >", "once_cell::sync::impl$6::get_or_init::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()> >", "enum2$<once_cell::sync::impl$6::get_or_init::Void>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "once_cell::imp::impl$4::initialize::closure$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$6::get_or_init::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()> >,enum2$<once_cell::sync::impl$6::get_or_init::Void> >", "constructor": "once_cell::imp::impl$4::initialize::closure$0", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >", "once_cell::sync::impl$6::get_or_init::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,once_cell::sync::impl$11::force::closure_env$0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()> >", "enum2$<once_cell::sync::impl$6::get_or_init::Void>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: image_baker.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/image_baker.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: image_baker.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::image_baker::bake_grass2_world_albedo_tint_from_env
// [PDB Shader 类型/入口] {"shader_family":"country_core/grass2/bake_world_albedo_tint","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::grass2::bake_world_albedo_tint::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":559,"record_offset":12104,"parent_record_offset":12068,"ancestor_record_offset":8732,"inlinee":34991,"source_line":9330,"archive":"_pdb/C/modules/00559.jsonl.gz"}}
// [Shader 映射] {"shader_id":"8c8a0a095bf81bf6d6902cf3","relative_path":"country-core/compiled/grass2/bake_world_albedo_tint.precomp","sha256":"a48ff3e85c8819512b8f39664281791be841ff18e1d431cd5ac6162fd61f7750","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::image_baker::bake_grass2_world_albedo_tint_from_env","hint":{"shader_family":"country_core/grass2/bake_world_albedo_tint","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::grass2::bake_world_albedo_tint::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":559,"record_offset":12104,"parent_record_offset":12068,"ancestor_record_offset":8732,"inlinee":34991,"source_line":9330,"archive":"_pdb/C/modules/00559.jsonl.gz"}},"shader_uri":"country_core/grass2/bake_world_albedo_tint:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/grass2/bake_world_albedo_tint.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[11]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"globalParams","storage_class":"Uniform","type":{"id":16,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[12,15]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"output_dims","type_id":12,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"texels_per_meter","type_id":15,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":76,"name":"samplers.llr_iso","storage_class":"UniformConstant","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,73]},"decorations":[],"pointee":{"id":73,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":6,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[6]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":80,"name":"tint_texture","storage_class":"UniformConstant","type":{"id":79,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,77]},"decorations":[],"pointee":{"id":77,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[15,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":195,"name":"output_buf","storage_class":"Uniform","type":{"id":194,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,192]},"decorations":[],"pointee":{"id":192,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[193]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":193,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"unknown","storage_class":"Input","type":{"id":10,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,8]},"decorations":[],"pointee":{"id":8,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[7,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5682763,"position_among_mapped_render_targets":36,"mapped_render_target_count":74,"previous_callsite":5682530,"next_callsite":5682785,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/grass2.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5692800,"caller_aliases":{"alias_group_rva":5692800,"count":1,"display_examples":["tiny_glade::render_loop::render"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5699669,"position_among_mapped_render_targets":4,"mapped_render_target_count":4,"previous_callsite":5695014,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: image_baker.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::image_baker::PENDING_IMAGE_BAKES::closure$0
// 信息 ID: cd96d8e11dc5e6ca9d0fb2ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::image_baker::PENDING_IMAGE_BAKES", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::image_baker::PENDING_IMAGE_BAKES::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::image_baker::PENDING_IMAGE_BAKES::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::image_baker::PENDING_IMAGE_BAKES::closure_env$0,tuple$<> >; module=559, inline_record=117872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::image_baker::bake_grass2_world_albedo_tint_from_env
// 信息 ID: 8f93832e706386c790f6ff62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b64b -> 0xafd0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::image_baker::finish_pending_image_bakes
// 信息 ID: 65dd6723c0cf800f0302c480
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x56f855 -> 0xafc8f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png
// 信息 ID: bc5e4b0200ac6f9939c30c2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::image_baker::finish_pending_image_bakes; module=559, inline_record=6536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure$0
// 信息 ID: 3d36186ce1dc931efa0d8f28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure_env$0 <- anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure_env$0>; module=1253, inline_record=41060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure$1
// 信息 ID: 746eaaaa769aecc6cbed4ea1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure_env$1 <- anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure_env$1>; module=1253, inline_record=30516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure$2
// 信息 ID: 069d14ef6ad4f66a7203158e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure_env$2 <- anyhow::context::impl$0::with_context<tuple$<>,enum2$<image::error::ImageError>,alloc::string::String,country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,enum2$<image::error::ImageError>,alloc::string::String,country_core::systems::render_loop::compute_passes::image_baker::write_rgba8_png::closure_env$2>; module=1253, inline_record=38784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
