// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c9b318c537c7c904e377b59697a4528eec3dfc36a2dd11f6a357a1ba022e4f24
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: util_pillar_construction.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3176, 4377
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3176,"record_offset":1880,"rva":38717680,"va":5407426800}
// 编译实例 RVAs: 0x24ec8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-31751725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8d3811bfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407430424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008f730
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38134634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8961cbafd","size":5,"soft_fail":false,"target_va":5369296688,"va":5407431317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=3
//     target_aliases=17 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37885919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e821e8bdfd","size":5,"soft_fail":false,"target_va":5369542000,"va":5407427914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=3
//     target_aliases=20 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29300997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8fbe640fe","size":5,"soft_fail":false,"target_va":5378126944,"va":5407427936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25701384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8f8d377fe","size":5,"soft_fail":false,"target_va":5381726576,"va":5407427955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24154299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8456f8ffe","size":5,"soft_fail":false,"target_va":5383273968,"va":5407428262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24102365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8233a90fe","size":5,"soft_fail":false,"target_va":5383325056,"va":5407427416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f11b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17807319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e82948f0fe","size":5,"soft_fail":false,"target_va":5389619632,"va":5407426946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403450
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17734051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e85d66f1fe","size":5,"soft_fail":false,"target_va":5389694032,"va":5407428078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142a252d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5470806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8567a5300","size":5,"soft_fail":false,"target_va":5412901584,"va":5407430773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9628249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e859ea9200","size":5,"soft_fail":false,"target_va":5417057696,"va":5407429442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9628598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8b6eb9200","size":5,"soft_fail":false,"target_va":5417058032,"va":5407429429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8937365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8955f8800","size":5,"soft_fail":false,"target_va":5416364848,"va":5407427478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t9190590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8be3c8c00","size":5,"soft_fail":false,"target_va":5416621600,"va":5407431005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9145808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8d08d8b00","size":5,"soft_fail":false,"target_va":5416576832,"va":5407431019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9145102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e80e8b8b00","size":5,"soft_fail":false,"target_va":5416576144,"va":5407431037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23442175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8014d9afe","size":5,"soft_fail":false,"target_va":5383984848,"va":5407427018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23445491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e80d409afe","size":5,"soft_fail":false,"target_va":5383985360,"va":5407430846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-24019643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8457d91fe","size":5,"soft_fail":false,"target_va":5383408256,"va":5407427894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24151145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8977b8ffe","size":5,"soft_fail":false,"target_va":5383277472,"va":5407428612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24151896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426800,"raw_hex":"e8a8788ffe","size":5,"soft_fail":false,"target_va":5383275712,"va":5407427603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5407426800,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5407426967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3176,"record_offset":8852,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3176,"record_offset":2724,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3176,"record_offset":2920,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3176,"record_offset":9712,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":3176,"record_offset":3904,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3176,"record_offset":3820,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3176,"record_offset":4224,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3176,"record_offset":4368,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3176,"record_offset":3792,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3176,"record_offset":2836,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3176,"record_offset":6036,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3176,"record_offset":2260,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3176,"record_offset":9416,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3176,"record_offset":2752,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3176,"record_offset":2200,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3176,"record_offset":2296,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3176,"record_offset":6752,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":11736,"source_line":178,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$27::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3176,"record_offset":6132,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":11714,"source_line":1123,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$33::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3176,"record_offset":6900,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":11718,"source_line":1174,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3176,"record_offset":4424,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":79708,"source_line":1359,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3176,"record_offset":4480,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3176,"record_offset":5560,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3176,"record_offset":4656,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3176,"record_offset":5776,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3176,"record_offset":4536,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3176,"record_offset":5840,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3176,"record_offset":4256,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3176,"record_offset":9860,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs
//     evidence={"module":3176,"record_offset":3876,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":117874,"source_line":44,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3176,"record_offset":3140,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3176,"record_offset":9604,"parent_record_offset":9580,"ancestor_record_offset":1880,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3176,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":1880,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3176,"record_offset":9656,"parent_record_offset":9628,"ancestor_record_offset":1880,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3176,"record_offset":9580,"parent_record_offset":9488,"ancestor_record_offset":1880,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3176,"record_offset":3016,"parent_record_offset":2984,"ancestor_record_offset":1880,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3176,"record_offset":9516,"parent_record_offset":9488,"ancestor_record_offset":1880,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3176,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":1880,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3176,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":1880,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3176,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":1880,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3176,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":1880,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3176,"record_offset":2984,"parent_record_offset":2944,"ancestor_record_offset":1880,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3176,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":1880,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3176,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":1880,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3176,"record_offset":10016,"parent_record_offset":9992,"ancestor_record_offset":1880,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":3176,"record_offset":3932,"parent_record_offset":3904,"ancestor_record_offset":1880,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":3176,"record_offset":4064,"parent_record_offset":3960,"ancestor_record_offset":1880,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3176,"record_offset":6092,"parent_record_offset":6064,"ancestor_record_offset":1880,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3176,"record_offset":3700,"parent_record_offset":3180,"ancestor_record_offset":1880,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":3176,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":1880,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3176,"record_offset":6420,"parent_record_offset":6160,"ancestor_record_offset":1880,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3176,"record_offset":7276,"parent_record_offset":7220,"ancestor_record_offset":1880,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3176,"record_offset":3500,"parent_record_offset":3420,"ancestor_record_offset":1880,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3176,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":1880,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3176,"record_offset":6240,"parent_record_offset":6160,"ancestor_record_offset":1880,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3176,"record_offset":7140,"parent_record_offset":6948,"ancestor_record_offset":1880,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3176,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":1880,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3176,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":1880,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3176,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":1880,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3176,"record_offset":2648,"parent_record_offset":2620,"ancestor_record_offset":1880,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3176,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":1880,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3176,"record_offset":9548,"parent_record_offset":9516,"ancestor_record_offset":1880,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3176,"record_offset":4336,"parent_record_offset":4256,"ancestor_record_offset":1880,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3176,"record_offset":4308,"parent_record_offset":4256,"ancestor_record_offset":1880,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3176,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":1880,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3176,"record_offset":4280,"parent_record_offset":4256,"ancestor_record_offset":1880,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":3176,"record_offset":4008,"parent_record_offset":3960,"ancestor_record_offset":1880,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":3176,"record_offset":3960,"parent_record_offset":3932,"ancestor_record_offset":1880,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3176,"record_offset":2556,"parent_record_offset":2296,"ancestor_record_offset":1880,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3176,"record_offset":2368,"parent_record_offset":2296,"ancestor_record_offset":1880,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3176,"record_offset":2588,"parent_record_offset":2296,"ancestor_record_offset":1880,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3176,"record_offset":4148,"parent_record_offset":4088,"ancestor_record_offset":1880,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3176,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":1880,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3176,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":1880,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3176,"record_offset":9772,"parent_record_offset":9712,"ancestor_record_offset":1880,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3176,"record_offset":9800,"parent_record_offset":9712,"ancestor_record_offset":1880,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3176,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":1880,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3176,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":1880,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3176,"record_offset":3104,"parent_record_offset":2944,"ancestor_record_offset":1880,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3176,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":1880,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3176,"record_offset":2528,"parent_record_offset":2296,"ancestor_record_offset":1880,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3176,"record_offset":4120,"parent_record_offset":4088,"ancestor_record_offset":1880,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3176,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1880,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3176,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":1880,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3176,"record_offset":10436,"parent_record_offset":10372,"ancestor_record_offset":1880,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3176,"record_offset":2440,"parent_record_offset":2404,"ancestor_record_offset":1880,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3176,"record_offset":10460,"parent_record_offset":10436,"ancestor_record_offset":1880,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3176,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":1880,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3176,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":1880,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3176,"record_offset":2620,"parent_record_offset":2588,"ancestor_record_offset":1880,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3176,"record_offset":10344,"parent_record_offset":10316,"ancestor_record_offset":1880,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":3176,"record_offset":3180,"parent_record_offset":3140,"ancestor_record_offset":1880,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":3176,"record_offset":6780,"parent_record_offset":6752,"ancestor_record_offset":1880,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":3176,"record_offset":3236,"parent_record_offset":3180,"ancestor_record_offset":1880,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3176,"record_offset":6868,"parent_record_offset":6752,"ancestor_record_offset":1880,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3176,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":1880,"inlinee":11715,"source_line":804,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3176,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":1880,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3176,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":1880,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3176,"record_offset":7396,"parent_record_offset":6948,"ancestor_record_offset":1880,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3176,"record_offset":3320,"parent_record_offset":3236,"ancestor_record_offset":1880,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3176,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":1880,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3176,"record_offset":3420,"parent_record_offset":3180,"ancestor_record_offset":1880,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3176,"record_offset":4772,"parent_record_offset":4732,"ancestor_record_offset":1880,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3176,"record_offset":4732,"parent_record_offset":4684,"ancestor_record_offset":1880,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3176,"record_offset":4684,"parent_record_offset":4656,"ancestor_record_offset":1880,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3176,"record_offset":5588,"parent_record_offset":5560,"ancestor_record_offset":1880,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":3176,"record_offset":7020,"parent_record_offset":6992,"ancestor_record_offset":1880,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":3176,"record_offset":6992,"parent_record_offset":6948,"ancestor_record_offset":1880,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":3176,"record_offset":2404,"parent_record_offset":2296,"ancestor_record_offset":1880,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":3176,"record_offset":10372,"parent_record_offset":10344,"ancestor_record_offset":1880,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3176,"record_offset":2496,"parent_record_offset":2404,"ancestor_record_offset":1880,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3176,"record_offset":10408,"parent_record_offset":10372,"ancestor_record_offset":1880,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3176,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":1880,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3176,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":1880,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3176,"record_offset":4812,"parent_record_offset":4772,"ancestor_record_offset":1880,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC72
// 首条位置: {"module":3176,"inlinee":117874}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns
// 记录形式: compiled_procedure; instances=1
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3176,"record_offset":280,"rva":38716896,"va":5407426016}
// 编译实例 RVAs: 0x24ec5e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-31747816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426016,"raw_hex":"e818911bfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407426515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008f730
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38130090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426016,"raw_hex":"e8562ebafd","size":5,"soft_fail":false,"target_va":5369296688,"va":5407426773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f11b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17806450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426016,"raw_hex":"e88e4bf0fe","size":5,"soft_fail":false,"target_va":5389619632,"va":5407426077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142a252d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5475141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426016,"raw_hex":"e8458b5300","size":5,"soft_fail":false,"target_va":5412901584,"va":5407426438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9195041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426016,"raw_hex":"e8214e8c00","size":5,"soft_fail":false,"target_va":5416621600,"va":5407426554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9150256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426016,"raw_hex":"e8309f8b00","size":5,"soft_fail":false,"target_va":5416576832,"va":5407426571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9149550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426016,"raw_hex":"e86e9c8b00","size":5,"soft_fail":false,"target_va":5416576144,"va":5407426589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23441315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426016,"raw_hex":"e85d509afe","size":5,"soft_fail":false,"target_va":5383984848,"va":5407426158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23441119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407426016,"raw_hex":"e821519afe","size":5,"soft_fail":false,"target_va":5383985360,"va":5407426474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5407426016,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5407426099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3176,"record_offset":544,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3176,"record_offset":1108,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3176,"record_offset":1008,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3176,"record_offset":484,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3176,"record_offset":580,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3176,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3176,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3176,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3176,"record_offset":1456,"parent_record_offset":1368,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3176,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3176,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3176,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3176,"record_offset":932,"parent_record_offset":904,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3176,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3176,"record_offset":840,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3176,"record_offset":652,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3176,"record_offset":872,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3176,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3176,"record_offset":1076,"parent_record_offset":1008,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3176,"record_offset":1044,"parent_record_offset":1008,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3176,"record_offset":812,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3176,"record_offset":1228,"parent_record_offset":1164,"ancestor_record_offset":280,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3176,"record_offset":724,"parent_record_offset":688,"ancestor_record_offset":280,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3176,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":280,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3176,"record_offset":748,"parent_record_offset":724,"ancestor_record_offset":280,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3176,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":280,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3176,"record_offset":904,"parent_record_offset":872,"ancestor_record_offset":280,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3176,"record_offset":1136,"parent_record_offset":1108,"ancestor_record_offset":280,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":3176,"record_offset":688,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":3176,"record_offset":1164,"parent_record_offset":1136,"ancestor_record_offset":280,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3176,"record_offset":780,"parent_record_offset":688,"ancestor_record_offset":280,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3176,"record_offset":1200,"parent_record_offset":1164,"ancestor_record_offset":280,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/03176.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21EDC
// 首条位置: {"module":4377,"inlinee":138972}
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
// 完整结构及原始类型记录: util_pillar_construction.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: util_pillar_construction.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: util_pillar_construction.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns
// 信息 ID: d1136dd95fa8a56ae1e082d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x1482e0b -> 0x24ec5e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x1487180 -> 0x24ec5e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns::closure$0
// 信息 ID: 9cb5743232e261b8afe9a632
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns::closure_env$0 <- system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns::closure_env$0> >; module=4377, inline_record=4320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base
// 信息 ID: 6f52ef33f94e580dd5c1bb2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x1482d96 -> 0x24ec8f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x1487108 -> 0x24ec8f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure$0
// 信息 ID: f2db6b6e8c07b445df8d2ce4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base; module=3176, inline_record=3876; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base; module=3176, inline_record=5952; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base; module=3176, inline_record=8824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
