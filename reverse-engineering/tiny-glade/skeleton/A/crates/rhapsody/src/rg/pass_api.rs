// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/rg/pass_api.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ce8b3e8c79a669042e62a0f1254b3c2049a7fd39fc66b6eb06771fc46ef28cbd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: pass_api.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 398, 550, 1318, 1385, 1464, 3025
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"PDB_registry_path_only","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::begin_render_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":776,"rva":18047792,"va":5386756912}
// 编译实例 RVAs: 0x1136330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10518658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e87e7f5fff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386758573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140742850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10436678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e8babf60ff","size":5,"soft_fail":false,"target_va":5376321616,"va":5386758289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t267427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e8a3140400","size":5,"soft_fail":false,"target_va":5387024432,"va":5386757000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141188890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t336860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e8dc230500","size":5,"soft_fail":false,"target_va":5387094160,"va":5386757295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420aa900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16203152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e8903df700","size":5,"soft_fail":false,"target_va":5402962176,"va":5386759019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16219852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e8cc7ef700","size":5,"soft_fail":false,"target_va":5402978912,"va":5386759055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16219844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e8c47ef700","size":5,"soft_fail":false,"target_va":5402978880,"va":5386759031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16467151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e8cf44fb00","size":5,"soft_fail":false,"target_va":5403226160,"va":5386759004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16219882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e8ea7ef700","size":5,"soft_fail":false,"target_va":5402979152,"va":5386759265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16220517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e86581f700","size":5,"soft_fail":false,"target_va":5402979344,"va":5386758822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16219806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e89e7ef700","size":5,"soft_fail":false,"target_va":5402978944,"va":5386759133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16221227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e82b84f700","size":5,"soft_fail":false,"target_va":5402980128,"va":5386758896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::image_view
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10422162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386756912,"raw_hex":"e86ef860ff","size":5,"soft_fail":false,"target_va":5376335392,"va":5386757549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1040(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386756912,"raw_hex":"ff9710040000","size":6,"soft_fail":false,"va":5386758546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1464,"record_offset":1796,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":23754,"source_line":82,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::push
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1464,"record_offset":1968,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":15307,"source_line":178,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1464,"record_offset":3432,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::cmd_begin_render_pass
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":3928,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":67170,"source_line":1829,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::RenderPassBeginInfo::clear_values
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1464,"record_offset":3900,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":67169,"source_line":5787,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1464,"record_offset":2348,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":1024,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":1056,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":2404,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":988,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":3192,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1464,"record_offset":3956,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1464,"record_offset":1932,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1464,"record_offset":2692,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1464,"record_offset":1852,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::framebuffer::FramebufferDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/framebuffer.rs
//     evidence={"module":1464,"record_offset":1628,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":67166,"source_line":22,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::get_tracked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":3608,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":30923,"source_line":327,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1464,"record_offset":2320,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":31217,"source_line":245,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::resource
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":1464,"record_offset":2180,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":31155,"source_line":42,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1464,"record_offset":1264,"parent_record_offset":1216,"ancestor_record_offset":776,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1464,"record_offset":3456,"parent_record_offset":3432,"ancestor_record_offset":776,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1464,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":776,"inlinee":15311,"source_line":113,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1464,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":776,"inlinee":15310,"source_line":713,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1464,"record_offset":2108,"parent_record_offset":2052,"ancestor_record_offset":776,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1464,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":776,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1464,"record_offset":1996,"parent_record_offset":1968,"ancestor_record_offset":776,"inlinee":15308,"source_line":39,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1464,"record_offset":2052,"parent_record_offset":2024,"ancestor_record_offset":776,"inlinee":15312,"source_line":54,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1464,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":776,"inlinee":15309,"source_line":43,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1464,"record_offset":1160,"parent_record_offset":1096,"ancestor_record_offset":776,"inlinee":30812,"source_line":27,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1464,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":776,"inlinee":30813,"source_line":23,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1464,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":776,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1464,"record_offset":1416,"parent_record_offset":1372,"ancestor_record_offset":776,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1464,"record_offset":3856,"parent_record_offset":3764,"ancestor_record_offset":776,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1464,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":776,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1464,"record_offset":1728,"parent_record_offset":1628,"ancestor_record_offset":776,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1464,"record_offset":1672,"parent_record_offset":1628,"ancestor_record_offset":776,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":1756,"parent_record_offset":1628,"ancestor_record_offset":776,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":1584,"parent_record_offset":1096,"ancestor_record_offset":776,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1464,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1464,"record_offset":1896,"parent_record_offset":1852,"ancestor_record_offset":776,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1464,"record_offset":2080,"parent_record_offset":2052,"ancestor_record_offset":776,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1464,"record_offset":2144,"parent_record_offset":1996,"ancestor_record_offset":776,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1464,"record_offset":1296,"parent_record_offset":1264,"ancestor_record_offset":776,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1464,"record_offset":1328,"parent_record_offset":1296,"ancestor_record_offset":776,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1464,"record_offset":3800,"parent_record_offset":3764,"ancestor_record_offset":776,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1464,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":776,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1464,"record_offset":3764,"parent_record_offset":3672,"ancestor_record_offset":776,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":3644,"parent_record_offset":3608,"ancestor_record_offset":776,"inlinee":7654,"source_line":250,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":1496,"parent_record_offset":1452,"ancestor_record_offset":776,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":1552,"parent_record_offset":1452,"ancestor_record_offset":776,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":1524,"parent_record_offset":1452,"ancestor_record_offset":776,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$4::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":3672,"parent_record_offset":3644,"ancestor_record_offset":776,"inlinee":30867,"source_line":94,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1464,"record_offset":3708,"parent_record_offset":3672,"ancestor_record_offset":776,"inlinee":30448,"source_line":43,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1464,"record_offset":1452,"parent_record_offset":1096,"ancestor_record_offset":776,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1464,"record_offset":1096,"parent_record_offset":1056,"ancestor_record_offset":776,"inlinee":67164,"source_line":85,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1464,"record_offset":2436,"parent_record_offset":2404,"ancestor_record_offset":776,"inlinee":67167,"source_line":125,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1464,"record_offset":1372,"parent_record_offset":1216,"ancestor_record_offset":776,"inlinee":30469,"source_line":215,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":1464,"record_offset":1216,"parent_record_offset":1096,"ancestor_record_offset":776,"inlinee":31157,"source_line":68,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1834A
// 首条位置: {"module":3025,"inlinee":99146}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::ImageAspectFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":3025,"record_offset":556,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":30812,"source_line":27,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3025,"record_offset":960,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":3025,"record_offset":836,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":3025,"record_offset":612,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":31157,"source_line":68,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3025,"record_offset":652,"parent_record_offset":612,"ancestor_record_offset":508,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":3025,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":508,"inlinee":30813,"source_line":23,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3025,"record_offset":800,"parent_record_offset":760,"ancestor_record_offset":508,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3025,"record_offset":684,"parent_record_offset":652,"ancestor_record_offset":508,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3025,"record_offset":716,"parent_record_offset":684,"ancestor_record_offset":508,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":3025,"record_offset":876,"parent_record_offset":836,"ancestor_record_offset":508,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":3025,"record_offset":928,"parent_record_offset":836,"ancestor_record_offset":508,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":3025,"record_offset":904,"parent_record_offset":836,"ancestor_record_offset":508,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":3025,"record_offset":760,"parent_record_offset":612,"ancestor_record_offset":508,"inlinee":30469,"source_line":215,"archive":"_pdb/A/modules/03025.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1065C
// 首条位置: {"module":1464,"inlinee":67164}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::ImageAspectFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1464,"record_offset":1160,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":30812,"source_line":27,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":1584,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1464,"record_offset":1452,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":1464,"record_offset":1216,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":31157,"source_line":68,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1464,"record_offset":1264,"parent_record_offset":1216,"ancestor_record_offset":1096,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1464,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":1096,"inlinee":30813,"source_line":23,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1464,"record_offset":1416,"parent_record_offset":1372,"ancestor_record_offset":1096,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1464,"record_offset":1296,"parent_record_offset":1264,"ancestor_record_offset":1096,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1464,"record_offset":1328,"parent_record_offset":1296,"ancestor_record_offset":1096,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":1496,"parent_record_offset":1452,"ancestor_record_offset":1096,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":1552,"parent_record_offset":1452,"ancestor_record_offset":1096,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":1524,"parent_record_offset":1452,"ancestor_record_offset":1096,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1464,"record_offset":1372,"parent_record_offset":1216,"ancestor_record_offset":1096,"inlinee":30469,"source_line":215,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1065F
// 首条位置: {"module":1464,"inlinee":67167}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::end_render_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":4340,"rva":18050448,"va":5386759568}
// 编译实例 RVAs: 0x1136d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_end_render_pass
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":4480,"parent_record_offset":4340,"ancestor_record_offset":4340,"inlinee":67171,"source_line":1902,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::set_default_view_and_scissor
// 记录形式: compiled_procedure; instances=1
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":4508,"rva":18050480,"va":5386759600}
// 编译实例 RVAs: 0x1136db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*728(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386759600,"raw_hex":"ff93d8020000","size":6,"soft_fail":false,"va":5386759708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_set_scissor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":4692,"parent_record_offset":4508,"ancestor_record_offset":4508,"inlinee":67173,"source_line":1861,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::cmd_set_viewport
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":4664,"parent_record_offset":4508,"ancestor_record_offset":4508,"inlinee":67172,"source_line":1968,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::set_stencil_ref
// 记录形式: compiled_procedure; instances=1
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":4724,"rva":18050656,"va":5386759776}
// 编译实例 RVAs: 0x1136e60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*792(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386759776,"raw_hex":"ff9018030000","size":6,"soft_fail":false,"va":5386759809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::set_stencil_reference
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1464,"record_offset":4864,"parent_record_offset":4724,"ancestor_record_offset":4724,"inlinee":67174,"source_line":241,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_set_stencil_reference
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":4896,"parent_record_offset":4864,"ancestor_record_offset":4724,"inlinee":31128,"source_line":2048,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::set_line_width
// 记录形式: compiled_procedure; instances=1
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":4932,"rva":18050720,"va":5386759840}
// 编译实例 RVAs: 0x1136ea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_set_line_width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":5072,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":67175,"source_line":1877,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCCB
// 首条位置: {"module":1385,"inlinee":64715}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":38348,"parent_record_offset":38316,"ancestor_record_offset":38316,"inlinee":64716,"source_line":1908,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indexed
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 254
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCCD
// 首条位置: {"module":1385,"inlinee":64717}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indexed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":38576,"parent_record_offset":38544,"ancestor_record_offset":38544,"inlinee":64718,"source_line":1707,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indirect
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCD1
// 首条位置: {"module":1385,"inlinee":64721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indirect
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":39376,"parent_record_offset":39296,"ancestor_record_offset":39296,"inlinee":64722,"source_line":1927,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":39340,"parent_record_offset":39296,"ancestor_record_offset":39296,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indirect_count
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCD3
// 首条位置: {"module":1385,"inlinee":64723}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indirect_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":39536,"parent_record_offset":39408,"ancestor_record_offset":39408,"inlinee":64724,"source_line":565,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":39464,"parent_record_offset":39408,"ancestor_record_offset":39408,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCCF
// 首条位置: {"module":1385,"inlinee":64719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indexed_indirect
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":39144,"parent_record_offset":39064,"ancestor_record_offset":39064,"inlinee":64720,"source_line":1728,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":39108,"parent_record_offset":39064,"ancestor_record_offset":39064,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect_count
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCD5
// 首条位置: {"module":1385,"inlinee":64725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indexed_indirect_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":39696,"parent_record_offset":39568,"ancestor_record_offset":39568,"inlinee":64726,"source_line":588,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":39624,"parent_record_offset":39568,"ancestor_record_offset":39568,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::dispatch_groups_3d
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 374
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5943
// 首条位置: {"module":398,"inlinee":22851}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_dispatch
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1318,"record_offset":944,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":22852,"source_line":1940,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::dispatch_threads_3d
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 383
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5942
// 首条位置: {"module":398,"inlinee":22850}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::dispatch_groups_3d
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1318,"record_offset":912,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":22851,"source_line":374,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_dispatch
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1318,"record_offset":944,"parent_record_offset":912,"ancestor_record_offset":852,"inlinee":22852,"source_line":1940,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::dispatch_indirect
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5945
// 首条位置: {"module":398,"inlinee":22853}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_dispatch_indirect
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1318,"record_offset":3440,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":22854,"source_line":1957,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":3404,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::bind_vertex_buffer_by_binding_point
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 419
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10668
// 首条位置: {"module":1464,"inlinee":67176}
// 编译实例 RVAs: 0x1136ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141177710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t264238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386759872,"raw_hex":"e82e080400","size":5,"soft_fail":false,"target_va":5387024144,"va":5386759901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16218904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386759872,"raw_hex":"e8187bf700","size":5,"soft_fail":false,"target_va":5402978880,"va":5386759971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*816(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386759872,"raw_hex":"ff9330030000","size":6,"soft_fail":false,"va":5386759949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_vertex_buffers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":5300,"parent_record_offset":5104,"ancestor_record_offset":5104,"inlinee":67177,"source_line":1883,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":5268,"parent_record_offset":5104,"ancestor_record_offset":5104,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::bind_index_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 433
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":5332,"rva":18050864,"va":5386759984}
// 编译实例 RVAs: 0x1136f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141177710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t264128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386759984,"raw_hex":"e8c0070400","size":5,"soft_fail":false,"target_va":5387024144,"va":5386760011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16218816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386759984,"raw_hex":"e8c07af700","size":5,"soft_fail":false,"target_va":5402978880,"va":5386760059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_index_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":5508,"parent_record_offset":5332,"ancestor_record_offset":5332,"inlinee":67178,"source_line":1638,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":5476,"parent_record_offset":5332,"ancestor_record_offset":5332,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::push_constants
// 记录形式: compiled_procedure; instances=1
// 源码行: 444
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":5540,"rva":18050960,"va":5386760080}
// 编译实例 RVAs: 0x1136f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16218972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386760080,"raw_hex":"e85c7bf700","size":5,"soft_fail":false,"target_va":5402979152,"va":5386760175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1032(%r10)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386760080,"raw_hex":"41ff9208040000","size":7,"soft_fail":false,"va":5386760143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_push_constants
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":5680,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":67179,"source_line":1809,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::copy_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 467
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":5712,"rva":18051072,"va":5386760192}
// 编译实例 RVAs: 0x1137000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16218444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386760192,"raw_hex":"e84c79f700","size":5,"soft_fail":false,"target_va":5402978880,"va":5386760431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16466100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386760192,"raw_hex":"e8b440fb00","size":5,"soft_fail":false,"target_va":5403226480,"va":5386760375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*872(%r10)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386760192,"raw_hex":"41ff9268030000","size":7,"soft_fail":false,"va":5386760319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":6176,"parent_record_offset":5712,"ancestor_record_offset":5712,"inlinee":67180,"source_line":1466,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":5980,"parent_record_offset":5712,"ancestor_record_offset":5712,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1464,"record_offset":5856,"parent_record_offset":5712,"ancestor_record_offset":5712,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":5896,"parent_record_offset":5856,"ancestor_record_offset":5712,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":5948,"parent_record_offset":5856,"ancestor_record_offset":5712,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":5924,"parent_record_offset":5856,"ancestor_record_offset":5712,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::fill_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 478
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":6208,"rva":18051440,"va":5386760560}
// 编译实例 RVAs: 0x1137170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16218044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386760560,"raw_hex":"e8bc77f700","size":5,"soft_fail":false,"target_va":5402978880,"va":5386760831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16465700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386760560,"raw_hex":"e8243ffb00","size":5,"soft_fail":false,"target_va":5403226480,"va":5386760775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*920(%r10)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386760560,"raw_hex":"41ff9298030000","size":7,"soft_fail":false,"va":5386760721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_fill_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":6512,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":30924,"source_line":1435,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":6476,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1464,"record_offset":6352,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":6392,"parent_record_offset":6352,"ancestor_record_offset":6208,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":6444,"parent_record_offset":6352,"ancestor_record_offset":6208,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":6420,"parent_record_offset":6352,"ancestor_record_offset":6208,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::clear_color_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1464,"record_offset":6544,"rva":18051776,"va":5386760896}
// 编译实例 RVAs: 0x11372c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16217730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386760896,"raw_hex":"e88276f700","size":5,"soft_fail":false,"target_va":5402978880,"va":5386761145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16465386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386760896,"raw_hex":"e8ea3dfb00","size":5,"soft_fail":false,"target_va":5403226480,"va":5386761089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*928(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386760896,"raw_hex":"ff90a0030000","size":6,"soft_fail":false,"va":5386761038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_clear_color_image
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":6964,"parent_record_offset":6544,"ancestor_record_offset":6544,"inlinee":67183,"source_line":1650,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$316::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1464,"record_offset":6852,"parent_record_offset":6544,"ancestor_record_offset":6544,"inlinee":67182,"source_line":5817,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1464,"record_offset":6816,"parent_record_offset":6544,"ancestor_record_offset":6544,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1464,"record_offset":6696,"parent_record_offset":6544,"ancestor_record_offset":6544,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::into_inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1464,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":6544,"inlinee":25246,"source_line":88,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1464,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":6544,"inlinee":17599,"source_line":627,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1464,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":6544,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":6732,"parent_record_offset":6696,"ancestor_record_offset":6544,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":6784,"parent_record_offset":6696,"ancestor_record_offset":6544,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1464,"record_offset":6760,"parent_record_offset":6696,"ancestor_record_offset":6544,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource::Ref::bind_view
// 记录形式: compiled_procedure, inlinee_declaration; instances=4
// 源码行: 661, 677
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76F8
// 首条位置: {"module":550,"inlinee":30456}
// 编译实例 RVAs: 0x728f70, 0x728fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::impl$5::bind
// 记录形式: compiled_procedure; instances=1
// 源码行: 687
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":550,"record_offset":480,"rva":7507920,"va":5376217040}
// 编译实例 RVAs: 0x728fd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource::Ref::bind_uniform_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 695
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":550,"record_offset":592,"rva":7507936,"va":5376217056}
// 编译实例 RVAs: 0x728fe0
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
// 完整结构及原始类型记录: pass_api.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::pass_api::RasterPassColorAttachment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >", "constructor": "arrayvec::arrayvec::impl$25::from_iter", "arguments": ["rhapsody::rg::pass_api::RasterPassColorAttachment", "5", "core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::rg::pass_api::RasterPassColorAttachment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>", "rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>", "core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> >", "constructor": "rhapsody::raw::framebuffer::impl$0::new::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >,rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >", "rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arrayvec::arrayvec::impl$25::from_iter<tuple$<ash::vk::enums::Format,ash::vk::bitflags::ImageUsageFlags,ash::vk::bitflags::ImageCreateFlags>,9,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >,rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> > > >", "constructor": "arrayvec::arrayvec::impl$25::from_iter", "arguments": ["tuple$<ash::vk::enums::Format,ash::vk::bitflags::ImageUsageFlags,ash::vk::bitflags::ImageCreateFlags>", "9", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >,rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::rg::pass_api::RasterPassColorAttachment>", "constructor": "ref$", "arguments": ["rhapsody::rg::pass_api::RasterPassColorAttachment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<rhapsody::rg::pass_api::RasterPassColorAttachment> >", "constructor": "tuple$", "arguments": ["ref$<rhapsody::rg::pass_api::RasterPassColorAttachment>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$4::call_once<tuple$<ref$<rhapsody::rg::pass_api::RasterPassColorAttachment> >,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>", "constructor": "core::ops::function::impls::impl$4::call_once", "arguments": ["tuple$<ref$<rhapsody::rg::pass_api::RasterPassColorAttachment> >", "rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::pass_api::RenderPassApi
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 23; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rhapsody::rg::pass_api::RenderPassApi>", "constructor": "ref_mut$", "arguments": ["rhapsody::rg::pass_api::RenderPassApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "tuple$", "arguments": ["ref_mut$<rhapsody::rg::pass_api::RenderPassApi>", "ref_mut$<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$47::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$47::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$47::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$47::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::pass_api::RenderPassBinding
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::rg::pass_api::RenderPassBinding>", "constructor": "enum2$", "arguments": ["rhapsody::rg::pass_api::RenderPassBinding"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<rhapsody::rg::pass_api::RenderPassBinding>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<rhapsody::rg::pass_api::RenderPassBinding>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >", "constructor": "core::slice::iter::Iter", "arguments": ["enum2$<rhapsody::rg::pass_api::RenderPassBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: pass_api.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: pass_api.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::begin_render_pass
// 信息 ID: a6f9df5677444dd235adb504
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x100224e -> 0x1136330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::bind_index_buffer
// 信息 ID: 184b2e816e27ca0ab35126fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1002bfd -> 0x1136f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::bind_vertex_buffer_by_binding_point
// 信息 ID: b7f8f9b9d99af182d15969c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1002bb8 -> 0x1136ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::clear_color_image
// 信息 ID: 50b8d1d48c8cf2ac3c7419fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources> @ 0x6aa86f -> 0x11372c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::copy_buffer
// 信息 ID: a888b6846b7cc7152821bfbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global> @ 0x1000c09 -> 0x1137000 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x1001b3e -> 0x1137000 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x1001bc7 -> 0x1137000 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x1001c4c -> 0x1137000 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x1001cd1 -> 0x1137000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::dispatch_groups_3d
// 信息 ID: 5880375a83e30bcde313a426
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=912; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure_env$0<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=2052; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2::closure_env$2,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=4288; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2::closure_env$2,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=4992; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=6744; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=9724; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=12100; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=14056; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::dispatch_indirect
// 信息 ID: 39e7c51805b337cf309811c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_unsorted_instance_indirection::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1318, inline_record=3360; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::allocate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1318, inline_record=8044; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1318, inline_record=8860; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_sorted_instance_list::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1318, inline_record=10848; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1318, inline_record=14760; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$1<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1318, inline_record=15576; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=20616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::dispatch_threads_3d
// 信息 ID: 329df7aca6a615f06d54b250
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=852; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure_env$0<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=2004; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2::closure_env$2,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=4240; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2::closure_env$2,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=4944; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=6696; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=9664; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=12052; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1318, inline_record=14004; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw
// 信息 ID: 592f4309cbda163323d300f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=38316; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=38900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indexed
// 信息 ID: 6d8653b9c8fcc223aa99d004
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=38544; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=39000; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=39176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect
// 信息 ID: cd6fd01059b2defd46d8ea40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=39064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect_count
// 信息 ID: 67d8aca5810d0e3c8dc9dc61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=39568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indirect
// 信息 ID: 84d0e637259c068c490b50bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=39296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indirect_count
// 信息 ID: 7fddfb631e3ac1b137f93c20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=39408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::end_render_pass
// 信息 ID: 685671ffe60123fbaf973728
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x100393a -> 0x1136d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::fill_buffer
// 信息 ID: fadbbb51551bf2193478ca05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_buffer::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources> @ 0x6aa90e -> 0x1137170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::push_constants
// 信息 ID: 925a231db987117da67f4325
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc8e32 -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc8fd5 -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc8f74 -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc8f14 -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure_env$0<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc9092 -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_unsorted_instance_indirection::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdc9289 -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2::closure_env$2,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc93e7 -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2::closure_env$2,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc94ab -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc9767 -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::allocate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdc996a -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdc9b4a -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc9d5a -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc9edb -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc9e8c -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0xdc9e3c -> 0x1136f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_sorted_instance_list::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdca1cb -> 0x1136f90 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::set_default_view_and_scissor
// 信息 ID: 878d78e3743e980057354d02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1002279 -> 0x1136db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::set_line_width
// 信息 ID: 079f6f7d59cb1aa8b986a31d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x10031a8 -> 0x1136ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x10031dd -> 0x1136ea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::set_stencil_ref
// 信息 ID: 7281fad177b95352ae4a82f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x100316a -> 0x1136e60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$0
// 信息 ID: c319fe995076577ed0ff7963
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::pass_api::impl$0::begin_render_pass", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0 <- arrayvec::arrayvec::impl$25::from_iter<tuple$<ash::vk::enums::Format,ash::vk::bitflags::ImageUsageFlags,ash::vk::bitflags::ImageCreateFlags>,9,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >,rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0 <- arrayvec::arrayvec::impl$25::from_iter<tuple$<ash::vk::enums::Format,ash::vk::bitflags::ImageUsageFlags,ash::vk::bitflags::ImageCreateFlags>,9,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >,rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ref$<rhapsody::rg::pass_api::RasterPassColorAttachment> >,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<rhapsody::rg::pass_api::RasterPassColorAttachment> >,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>; module=3025, inline_record=508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$1
// 信息 ID: c2fd2236c6cab803fe1572c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::rg::pass_api::impl$0::begin_render_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1464, inline_record=1096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$2
// 信息 ID: f4d6c1aa47a1391e80a35230
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::rg::pass_api::impl$0::begin_render_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1464, inline_record=2436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::impl$5::bind
// 信息 ID: 2eb939763bfb5cba6f6fafae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 101 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x65391 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x65491 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x65591 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer @ 0x53edaf -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer @ 0x53f04c -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer_concurrent @ 0x53f2ee -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::read_rg_storage_buffer @ 0x6ab575 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::write_rg_storage_buffer @ 0x6abbd2 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::walls::dormers::DormerHoleCountPerCellSsbo> @ 0xddcbf1 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::rt::RtTriangleMetaSsbo> @ 0xddccf1 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::render::lantern::LanternSsbo> @ 0xddcdf1 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo> @ 0xddcef1 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo> @ 0xddcff1 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::color_id::HalftimberColorIdSsbo> @ 0xddd0f1 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo> @ 0xddd1f1 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo> @ 0xddd2f1 -> 0x728fd0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::Ref::bind_uniform_buffer
// 信息 ID: 5eb6236bfc9b38c7659f0776
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x65291 -> 0x728fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults> @ 0xddd3f1 -> 0x728fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo> @ 0xdde9f1 -> 0x728fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0xddedf1 -> 0x728fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::Ref::bind_view
// 信息 ID: 6311c8e1aed4b17caa1c40ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 26 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64daa -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cccda -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd63a -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x338e8a -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x358775 -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x540871 -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset> @ 0x54155e -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > > @ 0x5acec4 -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > > @ 0x5ad6e8 -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x643074 -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x643604 -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::raw::texture::Texture> > @ 0x643cda -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x644728 -> 0x728f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d45 -> 0x728fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1ccc75 -> 0x728fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd5d5 -> 0x728fa0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
