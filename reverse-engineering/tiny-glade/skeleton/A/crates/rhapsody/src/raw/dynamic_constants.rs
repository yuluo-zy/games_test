// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/dynamic_constants.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5ea705f8fac16ad4d72ab5711732b1a02135f4dd8be152422c755567bbbbcd04
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dynamic_constants.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 47, 197, 400, 402, 456, 490, 565, 1202, 1617
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"PDB_registry_path_only","version":"1.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D53
// 首条位置: {"module":47,"inlinee":7507}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":11308,"parent_record_offset":11276,"ancestor_record_offset":11276,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":197,"record_offset":11344,"parent_record_offset":11308,"ancestor_record_offset":11276,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::with_data
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x664B
// 首条位置: {"module":456,"inlinee":26187}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs
//     evidence={"module":456,"record_offset":45084,"parent_record_offset":45052,"ancestor_record_offset":45052,"inlinee":26188,"source_line":77,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":45052,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":456,"record_offset":45140,"parent_record_offset":45084,"ancestor_record_offset":45052,"inlinee":15183,"source_line":364,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":456,"record_offset":45172,"parent_record_offset":45140,"ancestor_record_offset":45052,"inlinee":7723,"source_line":266,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":456,"record_offset":45208,"parent_record_offset":45172,"ancestor_record_offset":45052,"inlinee":7724,"source_line":379,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":456,"record_offset":45352,"parent_record_offset":45328,"ancestor_record_offset":45052,"inlinee":6059,"source_line":594,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":456,"record_offset":45304,"parent_record_offset":45252,"ancestor_record_offset":45052,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":456,"record_offset":45476,"parent_record_offset":45084,"ancestor_record_offset":45052,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":456,"record_offset":45328,"parent_record_offset":45304,"ancestor_record_offset":45052,"inlinee":11986,"source_line":300,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":456,"record_offset":45508,"parent_record_offset":45476,"ancestor_record_offset":45052,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":456,"record_offset":45252,"parent_record_offset":45084,"ancestor_record_offset":45052,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":456,"record_offset":45424,"parent_record_offset":45400,"ancestor_record_offset":45052,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":45584,"parent_record_offset":45560,"ancestor_record_offset":45052,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":456,"record_offset":45400,"parent_record_offset":45376,"ancestor_record_offset":45052,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":456,"record_offset":45560,"parent_record_offset":45508,"ancestor_record_offset":45052,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":456,"record_offset":45376,"parent_record_offset":45352,"ancestor_record_offset":45052,"inlinee":6146,"source_line":171,"archive":"_pdb/A/modules/00456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":1936,"rva":7552032,"va":5376261152}
// 编译实例 RVAs: 0x733c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_gpu_lifetime_tracker
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":2084,"rva":7552048,"va":5376261168}
// 编译实例 RVAs: 0x733c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":2244,"rva":7552080,"va":5376261200}
// 编译实例 RVAs: 0x733c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261200,"raw_hex":"e826acffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376261381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10762191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261200,"raw_hex":"e8cf37a400","size":5,"soft_fail":false,"target_va":5387023456,"va":5376261260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26717522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261200,"raw_hex":"e852ad9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376261353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::map_tracker
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":2412,"parent_record_offset":2244,"ancestor_record_offset":2244,"inlinee":30768,"source_line":507,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":2520,"parent_record_offset":2444,"ancestor_record_offset":2244,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":2548,"parent_record_offset":2444,"ancestor_record_offset":2244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":565,"record_offset":2488,"parent_record_offset":2444,"ancestor_record_offset":2244,"inlinee":30771,"source_line":450,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":565,"record_offset":2444,"parent_record_offset":2412,"ancestor_record_offset":2244,"inlinee":30770,"source_line":53,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7832
// 首条位置: {"module":565,"inlinee":30770}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":2520,"parent_record_offset":2444,"ancestor_record_offset":2444,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":2548,"parent_record_offset":2444,"ancestor_record_offset":2444,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":565,"record_offset":2488,"parent_record_offset":2444,"ancestor_record_offset":2444,"inlinee":30771,"source_line":450,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":2444,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":2616,"rva":7552288,"va":5376261408}
// 编译实例 RVAs: 0x733d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261408,"raw_hex":"e836abffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376261621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10761983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261408,"raw_hex":"e8ff36a400","size":5,"soft_fail":false,"target_va":5387023456,"va":5376261468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26717295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261408,"raw_hex":"e86fac9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376261580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::map_tracker
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":2796,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":30768,"source_line":507,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":2904,"parent_record_offset":2828,"ancestor_record_offset":2616,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":2932,"parent_record_offset":2828,"ancestor_record_offset":2616,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":565,"record_offset":2872,"parent_record_offset":2828,"ancestor_record_offset":2616,"inlinee":30771,"source_line":450,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::impl$0::into_uniform_and_storage_buffer_binding::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":565,"record_offset":2828,"parent_record_offset":2796,"ancestor_record_offset":2616,"inlinee":30773,"source_line":75,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::impl$0::into_uniform_and_storage_buffer_binding::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7835
// 首条位置: {"module":565,"inlinee":30773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":2904,"parent_record_offset":2828,"ancestor_record_offset":2828,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":2932,"parent_record_offset":2828,"ancestor_record_offset":2828,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":565,"record_offset":2872,"parent_record_offset":2828,"ancestor_record_offset":2828,"inlinee":30771,"source_line":450,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":2828,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":3000,"rva":7552528,"va":5376261648}
// 编译实例 RVAs: 0x733e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261648,"raw_hex":"e812aaffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376261913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10761739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261648,"raw_hex":"e80b36a400","size":5,"soft_fail":false,"target_va":5387023456,"va":5376261712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26717009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261648,"raw_hex":"e851ab9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376261866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::map_tracker
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":3184,"parent_record_offset":3000,"ancestor_record_offset":3000,"inlinee":30768,"source_line":507,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":3380,"parent_record_offset":3000,"ancestor_record_offset":3000,"inlinee":30605,"source_line":107,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":3288,"parent_record_offset":3212,"ancestor_record_offset":3000,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":3316,"parent_record_offset":3212,"ancestor_record_offset":3000,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":3000,"inlinee":30606,"source_line":2665,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":3000,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":565,"record_offset":3256,"parent_record_offset":3212,"ancestor_record_offset":3000,"inlinee":30771,"source_line":450,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding_and_data::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":565,"record_offset":3212,"parent_record_offset":3184,"ancestor_record_offset":3000,"inlinee":30775,"source_line":101,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding_and_data::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7837
// 首条位置: {"module":565,"inlinee":30775}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":3288,"parent_record_offset":3212,"ancestor_record_offset":3212,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":3316,"parent_record_offset":3212,"ancestor_record_offset":3212,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":565,"record_offset":3256,"parent_record_offset":3212,"ancestor_record_offset":3212,"inlinee":30771,"source_line":450,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":3340,"parent_record_offset":3316,"ancestor_record_offset":3212,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":3468,"rva":7552816,"va":5376261936}
// 编译实例 RVAs: 0x733f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261936,"raw_hex":"e83fa8ffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376262380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26716643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261936,"raw_hex":"e8e3a99701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376262232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26964199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261936,"raw_hex":"e8e7709b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376262276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26716929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261936,"raw_hex":"e801ab9701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376262218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::Buffer::mapped_slice_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261936,"raw_hex":"e8f5abffff","size":5,"soft_fail":false,"target_va":5376240512,"va":5376262022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376261936,"raw_hex":"e852040000","size":5,"soft_fail":false,"target_va":5376263088,"va":5376261977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1280(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376261936,"raw_hex":"ff9700050000","size":6,"soft_fail":false,"va":5376262127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::is_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":3652,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":30777,"source_line":2689,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":3680,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":3984,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::Buffer::device_address
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":565,"record_offset":3716,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":30611,"source_line":38,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::limits
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":565,"record_offset":3624,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":30776,"source_line":608,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":565,"record_offset":3952,"parent_record_offset":3716,"ancestor_record_offset":3468,"inlinee":30596,"source_line":710,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":565,"record_offset":3924,"parent_record_offset":3716,"ancestor_record_offset":3468,"inlinee":30595,"source_line":27762,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":3888,"parent_record_offset":3716,"ancestor_record_offset":3468,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3468,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":3800,"parent_record_offset":3756,"ancestor_record_offset":3468,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":3856,"parent_record_offset":3756,"ancestor_record_offset":3468,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":3828,"parent_record_offset":3756,"ancestor_record_offset":3468,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":565,"record_offset":3756,"parent_record_offset":3716,"ancestor_record_offset":3468,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::push
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D62
// 首条位置: {"module":197,"inlinee":7522}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":11380,"parent_record_offset":11228,"ancestor_record_offset":11228,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":197,"record_offset":11276,"parent_record_offset":11228,"ancestor_record_offset":11228,"inlinee":7507,"source_line":26,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":197,"record_offset":11344,"parent_record_offset":11308,"ancestor_record_offset":11228,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":197,"record_offset":11460,"parent_record_offset":11428,"ancestor_record_offset":11228,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":11404,"parent_record_offset":11380,"ancestor_record_offset":11228,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":11488,"parent_record_offset":11460,"ancestor_record_offset":11228,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":11308,"parent_record_offset":11276,"ancestor_record_offset":11228,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":197,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":11228,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x59E0
// 首条位置: {"module":400,"inlinee":23008}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":54204,"parent_record_offset":54060,"ancestor_record_offset":54060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":400,"record_offset":54136,"parent_record_offset":54060,"ancestor_record_offset":54060,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":400,"record_offset":54108,"parent_record_offset":54060,"ancestor_record_offset":54060,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":54172,"parent_record_offset":54136,"ancestor_record_offset":54060,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":400,"record_offset":54280,"parent_record_offset":54252,"ancestor_record_offset":54060,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":54228,"parent_record_offset":54204,"ancestor_record_offset":54060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":54304,"parent_record_offset":54280,"ancestor_record_offset":54060,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":400,"record_offset":54252,"parent_record_offset":54228,"ancestor_record_offset":54060,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::allocate
// 记录形式: compiled_procedure; instances=1
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":4072,"rva":7553280,"va":5376262400}
// 编译实例 RVAs: 0x734100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376262400,"raw_hex":"e89ba7ffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376262544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26715878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376262400,"raw_hex":"e8e6a69701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376262997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26963535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376262400,"raw_hex":"e84f6e9b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376262940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26716282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376262400,"raw_hex":"e87aa89701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376262865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26717404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376262400,"raw_hex":"e8dcac9701","size":5,"soft_fail":false,"target_va":5402980304,"va":5376262895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26717438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376262400,"raw_hex":"e8feac9701","size":5,"soft_fail":false,"target_va":5402980320,"va":5376262877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::Buffer::mapped_slice_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376262400,"raw_hex":"e85ba9ffff","size":5,"soft_fail":false,"target_va":5376240512,"va":5376262688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376262400,"raw_hex":"e828020000","size":5,"soft_fail":false,"target_va":5376263088,"va":5376262531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1280(%r14)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376262400,"raw_hex":"41ff9600050000","size":7,"soft_fail":false,"va":5376262671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":4616,"parent_record_offset":4072,"ancestor_record_offset":4072,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":4236,"parent_record_offset":4072,"ancestor_record_offset":4072,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":4264,"parent_record_offset":4072,"ancestor_record_offset":4072,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":565,"record_offset":4652,"parent_record_offset":4072,"ancestor_record_offset":4072,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::Buffer::device_address
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":565,"record_offset":4348,"parent_record_offset":4072,"ancestor_record_offset":4072,"inlinee":30611,"source_line":38,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":4788,"parent_record_offset":4072,"ancestor_record_offset":4072,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":565,"record_offset":4584,"parent_record_offset":4348,"ancestor_record_offset":4072,"inlinee":30596,"source_line":710,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":565,"record_offset":4556,"parent_record_offset":4348,"ancestor_record_offset":4072,"inlinee":30595,"source_line":27762,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":565,"record_offset":4520,"parent_record_offset":4348,"ancestor_record_offset":4072,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":4288,"parent_record_offset":4264,"ancestor_record_offset":4072,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":4072,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":565,"record_offset":4724,"parent_record_offset":4684,"ancestor_record_offset":4072,"inlinee":5946,"source_line":395,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":565,"record_offset":4684,"parent_record_offset":4652,"ancestor_record_offset":4072,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":4856,"parent_record_offset":4816,"ancestor_record_offset":4072,"inlinee":30606,"source_line":2665,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":4072,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":4432,"parent_record_offset":4388,"ancestor_record_offset":4072,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":4488,"parent_record_offset":4388,"ancestor_record_offset":4072,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":4460,"parent_record_offset":4388,"ancestor_record_offset":4072,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":565,"record_offset":4816,"parent_record_offset":4788,"ancestor_record_offset":4072,"inlinee":30605,"source_line":107,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":565,"record_offset":4388,"parent_record_offset":4348,"ancestor_record_offset":4072,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":4924,"rva":7553968,"va":5376263088}
// 编译实例 RVAs: 0x7343b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7180060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376263088,"raw_hex":"e8e47092ff","size":5,"soft_fail":false,"target_va":5369083440,"va":5376263495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376263088,"raw_hex":"e8bf510000","size":5,"soft_fail":false,"target_va":5376284224,"va":5376263292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26716751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376263088,"raw_hex":"e84faa9701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376263372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1090567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376263088,"raw_hex":"e807a41000","size":5,"soft_fail":false,"target_va":5377353776,"va":5376263204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":5184,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":565,"record_offset":5212,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":565,"record_offset":5424,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":565,"record_offset":5240,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":565,"record_offset":5100,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::alignment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":565,"record_offset":5296,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":30619,"source_line":127,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::limits
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":565,"record_offset":5268,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":30776,"source_line":608,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":565,"record_offset":5328,"parent_record_offset":5296,"ancestor_record_offset":4924,"inlinee":4103,"source_line":797,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$1::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":565,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":4924,"inlinee":30603,"source_line":248,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":565,"record_offset":5384,"parent_record_offset":5296,"ancestor_record_offset":4924,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":4924,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":4924,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00565.jsonl.gz"}
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
// 完整结构及原始类型记录: dynamic_constants.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::dynamic_constants::DynamicConstants
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 3
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::dynamic_constants::DynamicConstantsChunk
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 5
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dynamic_constants.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dynamic_constants.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>", "bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::allocate
// 信息 ID: 386c6f787a469e516ef8e851
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fd52e -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x302878 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::uniform_binder::UniformBinder::new @ 0x50e4d7 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x553514 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x5535ad -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x553635 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x5536b9 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x625a11 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x625f1e -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::new @ 0x6ab0a4 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::rg_buffer_copy_from_slice<u32> @ 0x6af699 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketFlowers> @ 0xdcbf72 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank> @ 0xdcc892 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> @ 0xdcd1b2 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > > @ 0xdcdab1 -> 0x734100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0xdce3d2 -> 0x734100 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer
// 信息 ID: d73ad53eb5f034f680d03232
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::new @ 0x733f59 -> 0x7343b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::allocate @ 0x734183 -> 0x7343b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::new
// 信息 ID: 97e952613b8815016031c6d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3cf188 -> 0x733f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3d4a30 -> 0x733f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::push
// 信息 ID: 6db152f76b27274151bee09a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=11228; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=56036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
// 信息 ID: b58646395595956615f05cb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=400, inline_record=54060; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args; module=402, inline_record=25000; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args; module=402, inline_record=25400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args; module=402, inline_record=25804; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args; module=402, inline_record=26208; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke; module=456, inline_record=48228; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>; module=490, inline_record=65828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address
// 信息 ID: 0f73e08be62aca464c4c43df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x625bbc -> 0x733c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x625f8f -> 0x733c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::buffer::Buffer::mapped_slice_mut @ 0x72ebd6 -> 0x733c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_gpu_lifetime_tracker
// 信息 ID: 7dae944ce662c7998ec675dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x625c89 -> 0x733c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x625ffc -> 0x733c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
// 信息 ID: b722f5fdd69fac77ab040b3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fd5ca -> 0x733d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x302914 -> 0x733d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding
// 信息 ID: af01bb79e721ea05e90ffbc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data
// 信息 ID: ab082e25f9aebc6d7b66b17e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::uniform_binder::UniformBinder::new @ 0x50e4e7 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::new @ 0x6ab0ba -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketFlowers> @ 0xdcbf88 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank> @ 0xdcc8a8 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> @ 0xdcd1c8 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > > @ 0xdcdac7 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0xdce3e8 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0xdced08 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0xdcf628 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile> @ 0xdcff48 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketMain> @ 0xdd0868 -> 0x733e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0xdd1188 -> 0x733e10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::with_data
// 信息 ID: 8652d471b79f6b42ba16ad5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke; module=456, inline_record=45052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
// 信息 ID: 69edb0dcbc8b7138416996bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=11276; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=56084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::impl$0::into_uniform_and_storage_buffer_binding::closure$0
// 信息 ID: 165b5a047be4a5bc08ed3ba7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::dynamic_constants::impl$0::into_uniform_and_storage_buffer_binding", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding; module=565, inline_record=2828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding::closure$0
// 信息 ID: dfa5ab77e26bf83e51124e1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding; module=565, inline_record=2444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding_and_data::closure$0
// 信息 ID: 08ffae40b093be23e11986cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding_and_data", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data; module=565, inline_record=3212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
