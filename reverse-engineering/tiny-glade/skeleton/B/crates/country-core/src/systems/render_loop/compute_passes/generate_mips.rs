// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6e3c7e32ab5b84f2d47ed43c7c0298078a8d1d2a7b9b8baead7d428f121bf7c9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: generate_mips.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 44, 544
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":44,"record_offset":240,"rva":1030288,"va":5369739408}
// 编译实例 RVAs: 0xfb890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402af750
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1784885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8353c1b00","size":5,"soft_fail":false,"target_va":5371524944,"va":5369740054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404f62d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4172076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e82ca93f00","size":5,"soft_fail":false,"target_va":5373911760,"va":5369739679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140587eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4768832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e840c44800","size":5,"soft_fail":false,"target_va":5374508720,"va":5369739883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5297678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e80ed65000","size":5,"soft_fail":false,"target_va":5375038752,"va":5369741069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5340870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8c67e5100","size":5,"soft_fail":false,"target_va":5375081488,"va":5369740613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5341737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e829825100","size":5,"soft_fail":false,"target_va":5375081504,"va":5369739762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9914
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41605865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8e9da7a02","size":5,"soft_fail":false,"target_va":5411346708,"va":5369740838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42107080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8c8808202","size":5,"soft_fail":false,"target_va":5411847168,"va":5369740083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13100606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e83ee6c700","size":5,"soft_fail":false,"target_va":5382840208,"va":5369739597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41800045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e86dd17d02","size":5,"soft_fail":false,"target_va":5411540963,"va":5369740913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12913739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e84b0cc500","size":5,"soft_fail":false,"target_va":5382655024,"va":5369741280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41801272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e838d67d02","size":5,"soft_fail":false,"target_va":5411542064,"va":5369740787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41801986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e802d97d02","size":5,"soft_fail":false,"target_va":5411542880,"va":5369740889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_groups
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8859717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e845308700","size":5,"soft_fail":false,"target_va":5378600464,"va":5369740742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8855596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e82c208700","size":5,"soft_fail":false,"target_va":5378596192,"va":5369740591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11375766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e89694ad00","size":5,"soft_fail":false,"target_va":5381115424,"va":5369739653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11378065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8919dad00","size":5,"soft_fail":false,"target_va":5381117840,"va":5369739770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_mip_level
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11378246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8469ead00","size":5,"soft_fail":false,"target_va":5381118176,"va":5369739925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::bind_individual_mips
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11378312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8889ead00","size":5,"soft_fail":false,"target_va":5381118448,"va":5369740131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11378272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8609ead00","size":5,"soft_fail":false,"target_va":5381118240,"va":5369739963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11377952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8209dad00","size":5,"soft_fail":false,"target_va":5381117856,"va":5369739899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::view_type
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11377720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8389cad00","size":5,"soft_fail":false,"target_va":5381117920,"va":5369740195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11614852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369739408,"raw_hex":"e8843ab100","size":5,"soft_fail":false,"target_va":5381354480,"va":5369739623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":44,"record_offset":552,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":44,"record_offset":524,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":8804,"source_line":649,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":44,"record_offset":1160,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":44,"record_offset":1016,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":8810,"source_line":3197,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":44,"record_offset":788,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":44,"record_offset":716,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":44,"record_offset":1228,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":8813,"source_line":415,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":44,"record_offset":1080,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_direct] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":44,"record_offset":660,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":8808,"source_line":239,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":44,"record_offset":632,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":8806,"source_line":469,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":44,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":240,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":44,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":240,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":44,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":240,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":44,"record_offset":1528,"parent_record_offset":1500,"ancestor_record_offset":240,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":44,"record_offset":940,"parent_record_offset":884,"ancestor_record_offset":240,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":44,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":240,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":44,"record_offset":1440,"parent_record_offset":1412,"ancestor_record_offset":240,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":44,"record_offset":912,"parent_record_offset":884,"ancestor_record_offset":240,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":44,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":240,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":44,"record_offset":1500,"parent_record_offset":1468,"ancestor_record_offset":240,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":44,"record_offset":1468,"parent_record_offset":1440,"ancestor_record_offset":240,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":44,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":240,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":44,"record_offset":1376,"parent_record_offset":1344,"ancestor_record_offset":240,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":44,"record_offset":1344,"parent_record_offset":1312,"ancestor_record_offset":240,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":44,"record_offset":1280,"parent_record_offset":1228,"ancestor_record_offset":240,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":44,"record_offset":1312,"parent_record_offset":1280,"ancestor_record_offset":240,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":44,"record_offset":1412,"parent_record_offset":1376,"ancestor_record_offset":240,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":44,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":240,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":44,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":240,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":44,"record_offset":1628,"parent_record_offset":1376,"ancestor_record_offset":240,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":44,"record_offset":756,"parent_record_offset":716,"ancestor_record_offset":240,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":44,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":240,"inlinee":8809,"source_line":308,"archive":"_pdb/B/modules/00044.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset>
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":76868,"rva":10725616,"va":5379434736}
// 编译实例 RVAs: 0xa3a8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4354416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8908ebdff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379435899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4353577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8d791bdff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379435076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1720610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8debee5ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379436749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1267965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e803a7ecff","size":5,"soft_fail":false,"target_va":5378167392,"va":5379435352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096c310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-845548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e81419f3ff","size":5,"soft_fail":false,"target_va":5378589456,"va":5379434999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-383577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8a725faff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379436324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1734002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e872751a00","size":5,"soft_fail":false,"target_va":5381170352,"va":5379436345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12355573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8f587bc00","size":5,"soft_fail":false,"target_va":5391790752,"va":5379435174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9914
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31910262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e876e9e601","size":5,"soft_fail":false,"target_va":5411346708,"va":5379436441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32411789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e88d90ee01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379435374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3405286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8e6f53300","size":5,"soft_fail":false,"target_va":5382840208,"va":5379434917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32104442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8fadfe901","size":5,"soft_fail":false,"target_va":5411540963,"va":5379436516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3218059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e88b1a3100","size":5,"soft_fail":false,"target_va":5382655024,"va":5379436960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32105669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8c5e4e901","size":5,"soft_fail":false,"target_va":5411542064,"va":5379436390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32106383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e88fe7e901","size":5,"soft_fail":false,"target_va":5411542880,"va":5379436492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-839690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8f62ff3ff","size":5,"soft_fail":false,"target_va":5378596192,"va":5379435877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1680446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e83ea41900","size":5,"soft_fail":false,"target_va":5381115424,"va":5379434973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1682751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e83fad1900","size":5,"soft_fail":false,"target_va":5381117840,"va":5379435084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_mip_level
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1682955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e80bae1900","size":5,"soft_fail":false,"target_va":5381118176,"va":5379435216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::bind_individual_mips
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1683021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e84dae1900","size":5,"soft_fail":false,"target_va":5381118448,"va":5379435422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1682981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e825ae1900","size":5,"soft_fail":false,"target_va":5381118240,"va":5379435254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1682661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8e5ac1900","size":5,"soft_fail":false,"target_va":5381117856,"va":5379435190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::view_type
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1682429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e8fdab1900","size":5,"soft_fail":false,"target_va":5381117920,"va":5379435486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1919532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434736,"raw_hex":"e82c4a1d00","size":5,"soft_fail":false,"target_va":5381354480,"va":5379434943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":544,"record_offset":77160,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":544,"record_offset":77132,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":8804,"source_line":649,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":544,"record_offset":77768,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":544,"record_offset":77624,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":8810,"source_line":3197,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":77396,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":544,"record_offset":77324,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_groups
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":78512,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":28848,"source_line":464,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":77836,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":8813,"source_line":415,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":77688,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":544,"record_offset":77268,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":8808,"source_line":239,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":544,"record_offset":77240,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":8806,"source_line":469,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":78160,"parent_record_offset":78136,"ancestor_record_offset":76868,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":78184,"parent_record_offset":78160,"ancestor_record_offset":76868,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":77572,"parent_record_offset":77548,"ancestor_record_offset":76868,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":78136,"parent_record_offset":78108,"ancestor_record_offset":76868,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":77548,"parent_record_offset":77492,"ancestor_record_offset":76868,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":544,"record_offset":77208,"parent_record_offset":77184,"ancestor_record_offset":76868,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":78048,"parent_record_offset":78020,"ancestor_record_offset":76868,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":77520,"parent_record_offset":77492,"ancestor_record_offset":76868,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":77492,"parent_record_offset":77468,"ancestor_record_offset":76868,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":78108,"parent_record_offset":78076,"ancestor_record_offset":76868,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":78076,"parent_record_offset":78048,"ancestor_record_offset":76868,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":77468,"parent_record_offset":77444,"ancestor_record_offset":76868,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":77984,"parent_record_offset":77952,"ancestor_record_offset":76868,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":77952,"parent_record_offset":77920,"ancestor_record_offset":76868,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":77888,"parent_record_offset":77836,"ancestor_record_offset":76868,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":77920,"parent_record_offset":77888,"ancestor_record_offset":76868,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":78020,"parent_record_offset":77984,"ancestor_record_offset":76868,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":544,"record_offset":78260,"parent_record_offset":78236,"ancestor_record_offset":76868,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":77184,"parent_record_offset":77160,"ancestor_record_offset":76868,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":544,"record_offset":78236,"parent_record_offset":77984,"ancestor_record_offset":76868,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":77364,"parent_record_offset":77324,"ancestor_record_offset":76868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":78544,"parent_record_offset":78512,"ancestor_record_offset":76868,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":544,"record_offset":77292,"parent_record_offset":77268,"ancestor_record_offset":76868,"inlinee":8809,"source_line":308,"archive":"_pdb/B/modules/00544.jsonl.gz"}
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
// 完整结构及原始类型记录: generate_mips.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: generate_mips.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: generate_mips.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":44,"record_offset":1080,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00044.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":544,"record_offset":77688,"parent_record_offset":76868,"ancestor_record_offset":76868,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00544.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"159394549e0732695b4d8236","relative_path":"_generate_mips.cs.hlsl_cs_main_0.bin","sha256":"c08d0c1cd1c4e844961b5b9159462e82f1542dc7f55ce9dbf119d5d596e9b860","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"generate_mips","prefix_candidates":["generate_mips"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _generate_mips.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"cs_main","interface_ids":[2,3,4]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[256,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"spdConstants","storage_class":"PushConstant","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,9]},"decorations":[],"pointee":{"id":9,"name":"type.ConstantBuffer.SpdConstants","record":{"opcode":"OpTypeStruct","operands":[30,30,37]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"mips","type_id":30,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"numWorkGroups","type_id":30,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"workGroupOffset","type_id":37,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"imgSrc","storage_class":"UniformConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[53,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"imgDst","storage_class":"UniformConstant","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,59]},"decorations":[],"pointee":{"id":59,"name":"unknown","record":{"opcode":"OpTypeArray","operands":[13,58]},"decorations":[],"element":{"id":13,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[53,1,2,1,0,2,8]},"decorations":[]}}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"imgDst6","storage_class":"UniformConstant","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[53,1,2,1,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]},{"decoration":"Coherent","operands":[]}],"access_qualifiers":["Binding","Coherent","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"spdGlobalAtomic","storage_class":"Uniform","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,16]},"decorations":[],"pointee":{"id":16,"name":"type.RWStructuredBuffer.SpdGlobalAtomicBuffer","record":{"opcode":"OpTypeStruct","operands":[25]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":25,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]},{"decoration":"Coherent","operands":[]}],"access_qualifiers":["Binding","BufferBlock","Coherent","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,67]},"decorations":[],"pointee":{"id":67,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[30,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5164258,"position_among_mapped_render_targets":54,"mapped_render_target_count":59,"previous_callsite":5164070,"next_callsite":5164297,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/dof.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/dof.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10818624,"caller_aliases":{"alias_group_rva":10818624,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10820254,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: generate_mips.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>
// 信息 ID: 7524af6b5ffbde29fc1721ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips -> ["country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecce2 -> 0xfb890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset>
// 信息 ID: e41a0882dff2d906462b38da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips -> ["country_core::resources::texture_presets::TexturePreset"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture @ 0xa51a9e -> 0xa3a8f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
