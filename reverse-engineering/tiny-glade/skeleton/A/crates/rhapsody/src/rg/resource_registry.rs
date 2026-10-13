// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/rg/resource_registry.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 8475240f5aa83c90ec5fef8d5c22c62e48adde8f11d843d60a4a11db06c51b4b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: resource_registry.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 42, 197, 490, 572, 1328, 1385, 1464, 2824, 3025
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":572,"record_offset":4156,"rva":7626624,"va":5376335744}
// 编译实例 RVAs: 0x745f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7254760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e8184d91ff","size":5,"soft_fail":false,"target_va":5369083440,"va":5376338195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4682945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e83f8bb8ff","size":5,"soft_fail":false,"target_va":5371653024,"va":5376335964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3588085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e80b40c9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376337792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072d070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-104524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e8b467feff","size":5,"soft_fail":false,"target_va":5376233584,"va":5376338103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-95940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e83c89feff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376335855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-52228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e8fc33ffff","size":5,"soft_fail":false,"target_va":5376284224,"va":5376336447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-51603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e86d36ffff","size":5,"soft_fail":false,"target_va":5376285408,"va":5376337006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140744090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e83adbffff","size":5,"soft_fail":false,"target_va":5376327824,"va":5376337233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141187030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10751340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e86c0da400","size":5,"soft_fail":false,"target_va":5387087920,"va":5376336575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118cff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10775724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e8ac6ca400","size":5,"soft_fail":false,"target_va":5387112432,"va":5376336703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26879412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e8b4259a01","size":5,"soft_fail":false,"target_va":5403215504,"va":5376336087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5029237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e88b42b3ff","size":5,"soft_fail":false,"target_va":5371308240,"va":5376337472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26641270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e876839601","size":5,"soft_fail":false,"target_va":5402979152,"va":5376337877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26641498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e85a849601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376337841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26641208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e838839601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376337731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_pool::ResourcePool::get_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335744,"raw_hex":"e8d9240000","size":5,"soft_fail":false,"target_va":5376345808,"va":5376336370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":572,"record_offset":5980,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":4360,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":6068,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":5004,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":5440,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::BufferUsageFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":572,"record_offset":5536,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":31151,"source_line":23,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::ImageUsageFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":572,"record_offset":5660,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":31152,"source_line":23,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":572,"record_offset":6208,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":572,"record_offset":5152,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":572,"record_offset":6236,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":572,"record_offset":5856,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":572,"record_offset":5284,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":12247,"source_line":23,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_pool::ResourcePool::get_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_pool.rs
//     evidence={"module":572,"record_offset":5688,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":31154,"source_line":14,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":572,"record_offset":6440,"parent_record_offset":6416,"ancestor_record_offset":4156,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":572,"record_offset":6416,"parent_record_offset":6360,"ancestor_record_offset":4156,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":5100,"parent_record_offset":5072,"ancestor_record_offset":4156,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":6388,"parent_record_offset":6360,"ancestor_record_offset":4156,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":5072,"parent_record_offset":5036,"ancestor_record_offset":4156,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":5816,"parent_record_offset":5784,"ancestor_record_offset":4156,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":5036,"parent_record_offset":5004,"ancestor_record_offset":4156,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":6360,"parent_record_offset":6332,"ancestor_record_offset":4156,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":572,"record_offset":4720,"parent_record_offset":4684,"ancestor_record_offset":4156,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":572,"record_offset":4684,"parent_record_offset":4656,"ancestor_record_offset":4156,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":6112,"parent_record_offset":6068,"ancestor_record_offset":4156,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":4396,"parent_record_offset":4360,"ancestor_record_offset":4156,"inlinee":4126,"source_line":1244,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":5784,"parent_record_offset":5748,"ancestor_record_offset":4156,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":5748,"parent_record_offset":5688,"ancestor_record_offset":4156,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":572,"record_offset":4424,"parent_record_offset":4360,"ancestor_record_offset":4156,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":572,"record_offset":4752,"parent_record_offset":4720,"ancestor_record_offset":4156,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":572,"record_offset":5240,"parent_record_offset":5184,"ancestor_record_offset":4156,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":572,"record_offset":6176,"parent_record_offset":6068,"ancestor_record_offset":4156,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":5468,"parent_record_offset":5440,"ancestor_record_offset":4156,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":5496,"parent_record_offset":5468,"ancestor_record_offset":4156,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":572,"record_offset":5184,"parent_record_offset":5152,"ancestor_record_offset":4156,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":572,"record_offset":5372,"parent_record_offset":5336,"ancestor_record_offset":4156,"inlinee":30606,"source_line":2665,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":572,"record_offset":6008,"parent_record_offset":5980,"ancestor_record_offset":4156,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":572,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":4156,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":572,"record_offset":4780,"parent_record_offset":4720,"ancestor_record_offset":4156,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":572,"record_offset":4812,"parent_record_offset":4684,"ancestor_record_offset":4156,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":572,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":4156,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":572,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":4156,"inlinee":30605,"source_line":107,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":572,"record_offset":5720,"parent_record_offset":5688,"ancestor_record_offset":4156,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::impl$1::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":197,"record_offset":3516,"rva":3133088,"va":5371842208}
// 编译实例 RVAs: 0x2fcea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t5741691","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371842208,"raw_hex":"e97b9c5700","size":5,"soft_fail":false,"target_va":5377583920,"va":5371842224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::impl$4::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D18
// 首条位置: {"module":42,"inlinee":7448}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::resource
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79B3
// 首条位置: {"module":572,"record_offset":2316,"rva":7625648,"va":5376334768}
// 编译实例 RVAs: 0x745bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26644529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376334768,"raw_hex":"e831909601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376334810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":2460,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":2316,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":2316,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::provide_lazy_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":572,"record_offset":2548,"rva":7625696,"va":5376334816}
// 编译实例 RVAs: 0x745be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-95111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376334816,"raw_hex":"e8798cfeff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376335026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26643943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376334816,"raw_hex":"e8e78d9601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376334996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":572,"record_offset":2864,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":572,"record_offset":2716,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":572,"record_offset":2892,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":572,"record_offset":2836,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":7666,"source_line":196,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":572,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2548,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":572,"record_offset":2792,"parent_record_offset":2748,"ancestor_record_offset":2548,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":572,"record_offset":2748,"parent_record_offset":2716,"ancestor_record_offset":2548,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::image
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B90
// 首条位置: {"module":490,"inlinee":27536}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79B5
// 首条位置: {"module":572,"record_offset":2980,"rva":7625920,"va":5376335040}
// 编译实例 RVAs: 0x745cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420aa900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26627060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335040,"raw_hex":"e8f44b9601","size":5,"soft_fail":false,"target_va":5402962176,"va":5376335111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26643949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335040,"raw_hex":"e8ed8d9601","size":5,"soft_fail":false,"target_va":5402979152,"va":5376335198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26644243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335040,"raw_hex":"e8138f9601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376335096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26643765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335040,"raw_hex":"e8358d9601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376335174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":3140,"parent_record_offset":2980,"ancestor_record_offset":2980,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":572,"record_offset":3236,"parent_record_offset":2980,"ancestor_record_offset":2980,"inlinee":30469,"source_line":215,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":572,"record_offset":3280,"parent_record_offset":3236,"ancestor_record_offset":2980,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":3168,"parent_record_offset":3140,"ancestor_record_offset":2980,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":3196,"parent_record_offset":3168,"ancestor_record_offset":2980,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::buffer
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79B6
// 首条位置: {"module":572,"record_offset":3316,"rva":7626096,"va":5376335216}
// 编译实例 RVAs: 0x745d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420aa920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26626906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335216,"raw_hex":"e85a4b9601","size":5,"soft_fail":false,"target_va":5402962208,"va":5376335297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26643826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335216,"raw_hex":"e8728d9601","size":5,"soft_fail":false,"target_va":5402979152,"va":5376335321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26644063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335216,"raw_hex":"e85f8e9601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376335276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26643560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335216,"raw_hex":"e8688c9601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376335379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":3464,"parent_record_offset":3316,"ancestor_record_offset":3316,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":572,"record_offset":3560,"parent_record_offset":3316,"ancestor_record_offset":3316,"inlinee":30469,"source_line":215,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":572,"record_offset":3596,"parent_record_offset":3560,"ancestor_record_offset":3316,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":3492,"parent_record_offset":3464,"ancestor_record_offset":3316,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":3520,"parent_record_offset":3492,"ancestor_record_offset":3316,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::image_view
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":572,"record_offset":3660,"rva":7626272,"va":5376335392}
// 编译实例 RVAs: 0x745e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420aa940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26626690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335392,"raw_hex":"e8824a9601","size":5,"soft_fail":false,"target_va":5402962240,"va":5376335545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26643161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335392,"raw_hex":"e8d98a9601","size":5,"soft_fail":false,"target_va":5402978880,"va":5376335714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26890817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335392,"raw_hex":"e841529a01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376335658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26643409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335392,"raw_hex":"e8d18b9601","size":5,"soft_fail":false,"target_va":5402979152,"va":5376335738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26643809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335392,"raw_hex":"e8618d9601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376335530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26643327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335392,"raw_hex":"e87f8b9601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376335612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::texture::ManagedTexture::view
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-69207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376335392,"raw_hex":"e8a9f1feff","size":5,"soft_fail":false,"target_va":5376266304,"va":5376335506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":3820,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":572,"record_offset":4116,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":572,"record_offset":3988,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":572,"record_offset":3916,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":30469,"source_line":215,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":572,"record_offset":3956,"parent_record_offset":3916,"ancestor_record_offset":3660,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":3660,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":572,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3660,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":572,"record_offset":4032,"parent_record_offset":3988,"ancestor_record_offset":3660,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":572,"record_offset":4084,"parent_record_offset":3988,"ancestor_record_offset":3660,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":572,"record_offset":4060,"parent_record_offset":3988,"ancestor_record_offset":3660,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::release
// 记录形式: compiled_procedure; instances=1
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":572,"record_offset":6956,"rva":7629360,"va":5376338480}
// 编译实例 RVAs: 0x746a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4685927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376338480,"raw_hex":"e8997fb8ff","size":5,"soft_fail":false,"target_va":5371653024,"va":5376338946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403809b0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3957368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376338480,"raw_hex":"e8889dc3ff","size":5,"soft_fail":false,"target_va":5372381616,"va":5376338979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b1e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-113208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376338480,"raw_hex":"e8c845feff","size":5,"soft_fail":false,"target_va":5376225760,"va":5376338963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-98680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376338480,"raw_hex":"e8887efeff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376338595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26639890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376338480,"raw_hex":"e8127e9601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376339049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::rg::resource::RuntimeResource> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376338480,"raw_hex":"e8a1edffff","size":5,"soft_fail":false,"target_va":5376334400,"va":5376339098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_pool::ResourcePool::insert_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376338480,"raw_hex":"e88f1c0000","size":5,"soft_fail":false,"target_va":5376346128,"va":5376338812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_pool::ResourcePool::insert_texture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376338480,"raw_hex":"e866180000","size":5,"soft_fail":false,"target_va":5376345120,"va":5376338869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":7112,"parent_record_offset":6956,"ancestor_record_offset":6956,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":572,"record_offset":7520,"parent_record_offset":6956,"ancestor_record_offset":6956,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":572,"record_offset":7616,"parent_record_offset":6956,"ancestor_record_offset":6956,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":572,"record_offset":7168,"parent_record_offset":6956,"ancestor_record_offset":6956,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":572,"record_offset":7388,"parent_record_offset":7352,"ancestor_record_offset":6956,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":572,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":6956,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":572,"record_offset":7196,"parent_record_offset":7168,"ancestor_record_offset":6956,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":572,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":6956,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":572,"record_offset":7584,"parent_record_offset":7520,"ancestor_record_offset":6956,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":572,"record_offset":7420,"parent_record_offset":7388,"ancestor_record_offset":6956,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":572,"record_offset":7552,"parent_record_offset":7520,"ancestor_record_offset":6956,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":572,"record_offset":7448,"parent_record_offset":7388,"ancestor_record_offset":6956,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":572,"record_offset":7480,"parent_record_offset":7352,"ancestor_record_offset":6956,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00572.jsonl.gz"}
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
// 完整结构及原始类型记录: resource_registry.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_registry::RegistryResource
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::resource_registry::RegistryResource>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::resource_registry::RegistryResource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["rhapsody::rg::resource_registry::RegistryResource", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["rhapsody::rg::resource_registry::RegistryResource", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_registry::ResourceRegistry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::resource_registry::ResourceRegistry>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::resource_registry::ResourceRegistry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: resource_registry.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: resource_registry.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)
// 候选参数类型: ["country_core::resources::world_raster_defs::AllWorldRasterWrite", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::buffer
// 信息 ID: a28073665439c94c2590ba12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_buffer::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources> @ 0x6aa8f4 -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_unsorted_instance_indirection::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdc929f -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::allocate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdc9980 -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdc9b60 -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_sorted_instance_list::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdca1e1 -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdca816 -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$1<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdcaa00 -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global> @ 0x1000ba0 -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x1001a6d -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x1001a8f -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x1001ab1 -> 0x745d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x1001ad3 -> 0x745d70 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::binding::rg_binding_to_descriptor_binding; module=2824, inline_record=2988; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::binding::rg_binding_to_descriptor_binding; module=2824, inline_record=3304; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::binding::rg_binding_to_descriptor_binding; module=2824, inline_record=3752; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::image
// 信息 ID: 5a3545904893d1dccea5308b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; module=490, inline_record=22948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
// 信息 ID: cf6fde1fe6bc4fb9dbe768a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources> @ 0x6aa864 -> 0x745cc0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1464, inline_record=1216; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::binding::rg_binding_to_descriptor_binding; module=2824, inline_record=2436; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<rhapsody::rg::pass_api::RasterPassColorAttachment> >,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>; module=3025, inline_record=612; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::image_view
// 信息 ID: 1c07696d25078fd597953ab3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass @ 0x11365ad -> 0x745e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass @ 0x1136749 -> 0x745e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::binding::rg_binding_to_descriptor_binding @ 0x18020f3 -> 0x745e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::DescriptorImageInfo,core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2> > > > @ 0x191540e -> 0x745e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
// 信息 ID: 11c45d902f3323d0c88423b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::graph::RenderGraph::compile<country_core::render::pass_builder::pass_resources::PassResources> @ 0x669f5 -> 0x745f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::provide_lazy_texture
// 信息 ID: 60992cb98d86b2406b07730b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2df304 -> 0x745be0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2df6b3 -> 0x745be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::release
// 信息 ID: bf1f3a13b5ed2790cab80c61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::retire<country_core::render::pass_builder::pass_resources::PassResources> @ 0x75e6c -> 0x746a30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::resource
// 信息 ID: a08e37d13910f0671e9b0987
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x747c4 -> 0x745bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x749bd -> 0x745bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x74bf5 -> 0x745bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x74fd2 -> 0x745bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x7507b -> 0x745bb0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1464, inline_record=2180; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1464, inline_record=2968; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::impl$1::fmt
// 信息 ID: b3d35dba633f525cabf84061
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::impl$4::default
// 信息 ID: b0f3f7ca1b1b2f59e52236af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::graph::RenderGraph::compile<country_core::render::pass_builder::pass_resources::PassResources>; module=42, inline_record=24468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
