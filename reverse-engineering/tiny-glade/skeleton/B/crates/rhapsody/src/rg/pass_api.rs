// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/pass_api.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ae3e025f04f4dcfcb11eaed3a5b32a24819cee198a2756b5155261eaccde3203
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: pass_api.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 518, 682, 1632, 1655, 1680, 3314
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::begin_render_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":880,"rva":24080016,"va":5392789136}
// 编译实例 RVAs: 0x16f6e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11620495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e871af4eff","size":5,"soft_fail":false,"target_va":5381170352,"va":5392790842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfbef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11515193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e8c74a50ff","size":5,"soft_fail":false,"target_va":5381275376,"va":5392790564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418aef70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1802371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e883801b00","size":5,"soft_fail":false,"target_va":5394591600,"va":5392789224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418c36e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1885878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e8b6c61c00","size":5,"soft_fail":false,"target_va":5394675424,"va":5392789541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428caa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18690764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e8cc321d01","size":5,"soft_fail":false,"target_va":5411482176,"va":5392791407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18750372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e8a41b1e01","size":5,"soft_fail":false,"target_va":5411541808,"va":5392791431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18750379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e8ab1b1e01","size":5,"soft_fail":false,"target_va":5411541776,"va":5392791392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18690804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e8f4321d01","size":5,"soft_fail":false,"target_va":5411481799,"va":5392790990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18750961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e8f11d1e01","size":5,"soft_fail":false,"target_va":5411542064,"va":5392791098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t18751045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e8451e1e01","size":5,"soft_fail":false,"target_va":5411542296,"va":5392791246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18750707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e8f31c1e01","size":5,"soft_fail":false,"target_va":5411541904,"va":5392791192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18751552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e840201e01","size":5,"soft_fail":false,"target_va":5411542880,"va":5392791323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::image_view
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11597490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392789136,"raw_hex":"e84e094fff","size":5,"soft_fail":false,"target_va":5381192288,"va":5392789773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1048(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392789136,"raw_hex":"ff9718040000","size":6,"soft_fail":false,"va":5392790815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1680,"record_offset":1960,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":39495,"source_line":82,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::push
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1680,"record_offset":2120,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":39496,"source_line":178,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1680,"record_offset":4232,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::cmd_begin_render_pass
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":4728,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":76731,"source_line":1829,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::RenderPassBeginInfo::clear_values
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1680,"record_offset":4700,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":76730,"source_line":5787,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1680,"record_offset":2664,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":1124,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":1156,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":2720,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":1088,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1680,"record_offset":4756,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1680,"record_offset":2092,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1680,"record_offset":4840,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1680,"record_offset":2016,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::framebuffer::FramebufferDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/framebuffer.rs
//     evidence={"module":1680,"record_offset":1792,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":76728,"source_line":22,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::get_tracked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":4408,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":39610,"source_line":327,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1680,"record_offset":2636,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":39059,"source_line":245,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::resource
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":1680,"record_offset":2332,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":39455,"source_line":73,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1680,"record_offset":1368,"parent_record_offset":1316,"ancestor_record_offset":880,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1680,"record_offset":4256,"parent_record_offset":4232,"ancestor_record_offset":880,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1680,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":880,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1680,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":880,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1680,"record_offset":2260,"parent_record_offset":2204,"ancestor_record_offset":880,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1680,"record_offset":4280,"parent_record_offset":4256,"ancestor_record_offset":880,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1680,"record_offset":2148,"parent_record_offset":2120,"ancestor_record_offset":880,"inlinee":39497,"source_line":39,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1680,"record_offset":2204,"parent_record_offset":2176,"ancestor_record_offset":880,"inlinee":28488,"source_line":54,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1680,"record_offset":2176,"parent_record_offset":2148,"ancestor_record_offset":880,"inlinee":28487,"source_line":43,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1680,"record_offset":1260,"parent_record_offset":1196,"ancestor_record_offset":880,"inlinee":39259,"source_line":27,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1680,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":880,"inlinee":39260,"source_line":23,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1680,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":880,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1680,"record_offset":4656,"parent_record_offset":4564,"ancestor_record_offset":880,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1680,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":880,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1680,"record_offset":1892,"parent_record_offset":1792,"ancestor_record_offset":880,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1680,"record_offset":1836,"parent_record_offset":1792,"ancestor_record_offset":880,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":1920,"parent_record_offset":1792,"ancestor_record_offset":880,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":1748,"parent_record_offset":1196,"ancestor_record_offset":880,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1680,"record_offset":4780,"parent_record_offset":4756,"ancestor_record_offset":880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1680,"record_offset":2056,"parent_record_offset":2016,"ancestor_record_offset":880,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1680,"record_offset":2232,"parent_record_offset":2204,"ancestor_record_offset":880,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1680,"record_offset":2296,"parent_record_offset":2148,"ancestor_record_offset":880,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1680,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":880,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1680,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":880,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1680,"record_offset":4600,"parent_record_offset":4564,"ancestor_record_offset":880,"inlinee":39124,"source_line":3344,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1680,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":880,"inlinee":29644,"source_line":3962,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1680,"record_offset":4564,"parent_record_offset":4472,"ancestor_record_offset":880,"inlinee":39123,"source_line":54,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":4444,"parent_record_offset":4408,"ancestor_record_offset":880,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":1660,"parent_record_offset":1616,"ancestor_record_offset":880,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":1716,"parent_record_offset":1616,"ancestor_record_offset":880,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":1688,"parent_record_offset":1616,"ancestor_record_offset":880,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$4::track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":4472,"parent_record_offset":4444,"ancestor_record_offset":880,"inlinee":39132,"source_line":96,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1680,"record_offset":4508,"parent_record_offset":4472,"ancestor_record_offset":880,"inlinee":39121,"source_line":43,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1680,"record_offset":1616,"parent_record_offset":1196,"ancestor_record_offset":880,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1680,"record_offset":1196,"parent_record_offset":1156,"ancestor_record_offset":880,"inlinee":76726,"source_line":87,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1680,"record_offset":1572,"parent_record_offset":1316,"ancestor_record_offset":880,"inlinee":39156,"source_line":207,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":1680,"record_offset":1316,"parent_record_offset":1196,"ancestor_record_offset":880,"inlinee":39456,"source_line":86,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C040
// 首条位置: {"module":3314,"inlinee":114752}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::ImageAspectFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":3314,"record_offset":568,"parent_record_offset":516,"ancestor_record_offset":516,"inlinee":39259,"source_line":27,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3314,"record_offset":1048,"parent_record_offset":516,"ancestor_record_offset":516,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":3314,"record_offset":924,"parent_record_offset":516,"ancestor_record_offset":516,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":3314,"record_offset":624,"parent_record_offset":516,"ancestor_record_offset":516,"inlinee":39456,"source_line":86,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3314,"record_offset":676,"parent_record_offset":624,"ancestor_record_offset":516,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":3314,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":516,"inlinee":39260,"source_line":23,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3314,"record_offset":704,"parent_record_offset":676,"ancestor_record_offset":516,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3314,"record_offset":732,"parent_record_offset":704,"ancestor_record_offset":516,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":3314,"record_offset":964,"parent_record_offset":924,"ancestor_record_offset":516,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":3314,"record_offset":1016,"parent_record_offset":924,"ancestor_record_offset":516,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":3314,"record_offset":992,"parent_record_offset":924,"ancestor_record_offset":516,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":3314,"record_offset":880,"parent_record_offset":624,"ancestor_record_offset":516,"inlinee":39156,"source_line":207,"archive":"_pdb/B/modules/03314.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12BB6
// 首条位置: {"module":1680,"inlinee":76726}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::ImageAspectFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1680,"record_offset":1260,"parent_record_offset":1196,"ancestor_record_offset":1196,"inlinee":39259,"source_line":27,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":1748,"parent_record_offset":1196,"ancestor_record_offset":1196,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1680,"record_offset":1616,"parent_record_offset":1196,"ancestor_record_offset":1196,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":1680,"record_offset":1316,"parent_record_offset":1196,"ancestor_record_offset":1196,"inlinee":39456,"source_line":86,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1680,"record_offset":1368,"parent_record_offset":1316,"ancestor_record_offset":1196,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1680,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":1196,"inlinee":39260,"source_line":23,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1680,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":1196,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1680,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":1196,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":1660,"parent_record_offset":1616,"ancestor_record_offset":1196,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":1716,"parent_record_offset":1616,"ancestor_record_offset":1196,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":1688,"parent_record_offset":1616,"ancestor_record_offset":1196,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1680,"record_offset":1572,"parent_record_offset":1316,"ancestor_record_offset":1196,"inlinee":39156,"source_line":207,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::end_render_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":4956,"rva":24082512,"va":5392791632}
// 编译实例 RVAs: 0x16f7850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_end_render_pass
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":5096,"parent_record_offset":4956,"ancestor_record_offset":4956,"inlinee":76732,"source_line":1902,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::set_default_view_and_scissor
// 记录形式: compiled_procedure; instances=1
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":5124,"rva":24082544,"va":5392791664}
// 编译实例 RVAs: 0x16f7870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*736(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392791664,"raw_hex":"ff93e0020000","size":6,"soft_fail":false,"va":5392791772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_set_scissor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":5308,"parent_record_offset":5124,"ancestor_record_offset":5124,"inlinee":76734,"source_line":1861,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::cmd_set_viewport
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":5280,"parent_record_offset":5124,"ancestor_record_offset":5124,"inlinee":76733,"source_line":1968,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::set_stencil_ref
// 记录形式: compiled_procedure; instances=1
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":5340,"rva":24082720,"va":5392791840}
// 编译实例 RVAs: 0x16f7920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*800(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392791840,"raw_hex":"ff9020030000","size":6,"soft_fail":false,"va":5392791873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::set_stencil_reference
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1680,"record_offset":5480,"parent_record_offset":5340,"ancestor_record_offset":5340,"inlinee":76735,"source_line":251,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_set_stencil_reference
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":5512,"parent_record_offset":5480,"ancestor_record_offset":5340,"inlinee":39551,"source_line":2048,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::set_line_width
// 记录形式: compiled_procedure; instances=1
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":5548,"rva":24082784,"va":5392791904}
// 编译实例 RVAs: 0x16f7960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_set_line_width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":5688,"parent_record_offset":5548,"ancestor_record_offset":5548,"inlinee":76736,"source_line":1877,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12571
// 首条位置: {"module":1655,"inlinee":75121}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":32040,"parent_record_offset":31940,"ancestor_record_offset":31940,"inlinee":75122,"source_line":1908,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":31984,"parent_record_offset":31940,"ancestor_record_offset":31940,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":31940,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indexed
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12573
// 首条位置: {"module":1655,"inlinee":75123}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indexed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":35552,"parent_record_offset":35456,"ancestor_record_offset":35456,"inlinee":75124,"source_line":1707,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":35496,"parent_record_offset":35456,"ancestor_record_offset":35456,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":35520,"parent_record_offset":35496,"ancestor_record_offset":35456,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indirect
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 297
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12577
// 首条位置: {"module":1655,"inlinee":75127}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indirect
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":36436,"parent_record_offset":36288,"ancestor_record_offset":36288,"inlinee":75128,"source_line":1927,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1655,"record_offset":36400,"parent_record_offset":36288,"ancestor_record_offset":36288,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":36344,"parent_record_offset":36288,"ancestor_record_offset":36288,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":36368,"parent_record_offset":36344,"ancestor_record_offset":36288,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indirect_count
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12579
// 首条位置: {"module":1655,"inlinee":75129}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indirect_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":36688,"parent_record_offset":36468,"ancestor_record_offset":36468,"inlinee":75131,"source_line":565,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1655,"record_offset":36616,"parent_record_offset":36468,"ancestor_record_offset":36468,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":36560,"parent_record_offset":36468,"ancestor_record_offset":36468,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":36584,"parent_record_offset":36560,"ancestor_record_offset":36468,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 366
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12575
// 首条位置: {"module":1655,"inlinee":75125}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indexed_indirect
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":36076,"parent_record_offset":35928,"ancestor_record_offset":35928,"inlinee":75126,"source_line":1728,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1655,"record_offset":36040,"parent_record_offset":35928,"ancestor_record_offset":35928,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":35984,"parent_record_offset":35928,"ancestor_record_offset":35928,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":36008,"parent_record_offset":35984,"ancestor_record_offset":35928,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect_count
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 397
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1257A
// 首条位置: {"module":1655,"inlinee":75130}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_draw_indexed_indirect_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":36940,"parent_record_offset":36720,"ancestor_record_offset":36720,"inlinee":75132,"source_line":588,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1655,"record_offset":36868,"parent_record_offset":36720,"ancestor_record_offset":36720,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":36812,"parent_record_offset":36720,"ancestor_record_offset":36720,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":36836,"parent_record_offset":36812,"ancestor_record_offset":36720,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::dispatch_groups_3d
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 435
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73D3
// 首条位置: {"module":518,"inlinee":29651}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_dispatch
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":518,"record_offset":26412,"parent_record_offset":26316,"ancestor_record_offset":26316,"inlinee":29652,"source_line":1940,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":518,"record_offset":26356,"parent_record_offset":26316,"ancestor_record_offset":26316,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":518,"record_offset":26380,"parent_record_offset":26356,"ancestor_record_offset":26316,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::dispatch_threads_3d
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 453
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73D2
// 首条位置: {"module":518,"inlinee":29650}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":518,"record_offset":26196,"parent_record_offset":26140,"ancestor_record_offset":26140,"inlinee":8810,"source_line":3197,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::dispatch_groups_3d
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":518,"record_offset":26316,"parent_record_offset":26140,"ancestor_record_offset":26140,"inlinee":29651,"source_line":435,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_dispatch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":518,"record_offset":26412,"parent_record_offset":26316,"ancestor_record_offset":26140,"inlinee":29652,"source_line":1940,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":518,"record_offset":26356,"parent_record_offset":26316,"ancestor_record_offset":26140,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":518,"record_offset":26380,"parent_record_offset":26356,"ancestor_record_offset":26140,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::dispatch_indirect
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 464
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73D5
// 首条位置: {"module":518,"inlinee":29653}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_dispatch_indirect
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1632,"record_offset":1216,"parent_record_offset":1072,"ancestor_record_offset":1072,"inlinee":29654,"source_line":1957,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":1180,"parent_record_offset":1072,"ancestor_record_offset":1072,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":1124,"parent_record_offset":1072,"ancestor_record_offset":1072,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":1072,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::bind_vertex_buffer_by_binding_point
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 498
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12BC1
// 首条位置: {"module":1680,"inlinee":76737}
// 编译实例 RVAs: 0x16f7980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418aed30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1799054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392791936,"raw_hex":"e88e731b00","size":5,"soft_fail":false,"target_va":5394591024,"va":5392791965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18749736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392791936,"raw_hex":"e828191e01","size":5,"soft_fail":false,"target_va":5411541776,"va":5392792035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*824(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392791936,"raw_hex":"ff9338030000","size":6,"soft_fail":false,"va":5392792013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_vertex_buffers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":5916,"parent_record_offset":5720,"ancestor_record_offset":5720,"inlinee":76738,"source_line":1883,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":5884,"parent_record_offset":5720,"ancestor_record_offset":5720,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::bind_index_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 512
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":5948,"rva":24082928,"va":5392792048}
// 编译实例 RVAs: 0x16f79f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418aed30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_api.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1798944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392792048,"raw_hex":"e820731b00","size":5,"soft_fail":false,"target_va":5394591024,"va":5392792075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18749648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392792048,"raw_hex":"e8d0181e01","size":5,"soft_fail":false,"target_va":5411541776,"va":5392792123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_index_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":6124,"parent_record_offset":5948,"ancestor_record_offset":5948,"inlinee":76739,"source_line":1638,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":6092,"parent_record_offset":5948,"ancestor_record_offset":5948,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::push_constants
// 记录形式: compiled_procedure; instances=1
// 源码行: 523
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":6156,"rva":24083024,"va":5392792144}
// 编译实例 RVAs: 0x16f7a50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18749820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392792144,"raw_hex":"e87c191e01","size":5,"soft_fail":false,"target_va":5411542064,"va":5392792239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1040(%r10)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392792144,"raw_hex":"41ff9210040000","size":7,"soft_fail":false,"va":5392792207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_push_constants
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":6296,"parent_record_offset":6156,"ancestor_record_offset":6156,"inlinee":76740,"source_line":1809,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::copy_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 548
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":6328,"rva":24083136,"va":5392792256}
// 编译实例 RVAs: 0x16f7ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18749177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392792256,"raw_hex":"e8f9161e01","size":5,"soft_fail":false,"target_va":5411541776,"va":5392792594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18689178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392792256,"raw_hex":"e89a2c1d01","size":5,"soft_fail":false,"target_va":5411481622,"va":5392792439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*880(%r10)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392792256,"raw_hex":"41ff9270030000","size":7,"soft_fail":false,"va":5392792383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":6792,"parent_record_offset":6328,"ancestor_record_offset":6328,"inlinee":39940,"source_line":1466,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":6596,"parent_record_offset":6328,"ancestor_record_offset":6328,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1680,"record_offset":6472,"parent_record_offset":6328,"ancestor_record_offset":6328,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":6512,"parent_record_offset":6472,"ancestor_record_offset":6328,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":6564,"parent_record_offset":6472,"ancestor_record_offset":6328,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":6540,"parent_record_offset":6472,"ancestor_record_offset":6328,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::fill_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 559
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":6824,"rva":24083504,"va":5392792624}
// 编译实例 RVAs: 0x16f7c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18748831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392792624,"raw_hex":"e89f151e01","size":5,"soft_fail":false,"target_va":5411541776,"va":5392792940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18688778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392792624,"raw_hex":"e80a2b1d01","size":5,"soft_fail":false,"target_va":5411481622,"va":5392792839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*928(%r10)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392792624,"raw_hex":"41ff92a0030000","size":7,"soft_fail":false,"va":5392792785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_fill_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":7128,"parent_record_offset":6824,"ancestor_record_offset":6824,"inlinee":39611,"source_line":1435,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":7092,"parent_record_offset":6824,"ancestor_record_offset":6824,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1680,"record_offset":6968,"parent_record_offset":6824,"ancestor_record_offset":6824,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":7008,"parent_record_offset":6968,"ancestor_record_offset":6824,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":7060,"parent_record_offset":6968,"ancestor_record_offset":6824,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":7036,"parent_record_offset":6968,"ancestor_record_offset":6824,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::clear_color_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 578
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":7160,"rva":24083840,"va":5392792960}
// 编译实例 RVAs: 0x16f7d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18748566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392792960,"raw_hex":"e896141e01","size":5,"soft_fail":false,"target_va":5411541776,"va":5392793205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18688468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392792960,"raw_hex":"e8d4291d01","size":5,"soft_fail":false,"target_va":5411481622,"va":5392793149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*936(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392792960,"raw_hex":"ff90a8030000","size":6,"soft_fail":false,"va":5392793098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_clear_color_image
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1680,"record_offset":7640,"parent_record_offset":7160,"ancestor_record_offset":7160,"inlinee":39968,"source_line":1650,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ImageSubresourceRange::layer_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1680,"record_offset":7612,"parent_record_offset":7160,"ancestor_record_offset":7160,"inlinee":39964,"source_line":2384,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$316::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1680,"record_offset":7472,"parent_record_offset":7160,"ancestor_record_offset":7160,"inlinee":39967,"source_line":5817,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":7436,"parent_record_offset":7160,"ancestor_record_offset":7160,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1680,"record_offset":7316,"parent_record_offset":7160,"ancestor_record_offset":7160,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1680,"record_offset":7568,"parent_record_offset":7544,"ancestor_record_offset":7160,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1680,"record_offset":7520,"parent_record_offset":7496,"ancestor_record_offset":7160,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1680,"record_offset":7496,"parent_record_offset":7472,"ancestor_record_offset":7160,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1680,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":7160,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":7352,"parent_record_offset":7316,"ancestor_record_offset":7160,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":7404,"parent_record_offset":7316,"ancestor_record_offset":7160,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1680,"record_offset":7380,"parent_record_offset":7316,"ancestor_record_offset":7160,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource::Ref::bind_view
// 记录形式: compiled_procedure, inlinee_declaration; instances=4
// 源码行: 742, 758
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x98E7
// 首条位置: {"module":682,"inlinee":39143}
// 编译实例 RVAs: 0xbd6c00, 0xbd6c30
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
// 源码行: 768
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":682,"record_offset":488,"rva":12414048,"va":5381123168}
// 编译实例 RVAs: 0xbd6c60
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
// 源码行: 776
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":682,"record_offset":600,"rva":12414064,"va":5381123184}
// 编译实例 RVAs: 0xbd6c70
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
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "tuple$", "arguments": ["ref_mut$<rhapsody::rg::pass_api::RenderPassApi>", "ref$<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$28::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$28::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$28::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$28::call_once", "arguments": ["tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::pass_api::RenderPassBinding
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::rg::pass_api::RenderPassBinding>", "constructor": "enum2$", "arguments": ["rhapsody::rg::pass_api::RenderPassBinding"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<rhapsody::rg::pass_api::RenderPassBinding>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<rhapsody::rg::pass_api::RenderPassBinding>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::begin_render_pass
// 信息 ID: 3aafe2972ba202b45794cbdf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1697b97 -> 0x16f6e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::bind_index_buffer
// 信息 ID: 2d04b5789c788095cbb42e90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1698b50 -> 0x16f79f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::bind_vertex_buffer_by_binding_point
// 信息 ID: 3882e96eb47d54d819b733cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x16988f8 -> 0x16f7980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::clear_color_image
// 信息 ID: d1a92d9db7092b9fb263ba1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources> @ 0x904a5f -> 0x16f7d80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::copy_buffer
// 信息 ID: 8db5339acd2bc125fe6076a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global> @ 0x169a6e9 -> 0x16f7ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x169a94e -> 0x16f7ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x169a9d7 -> 0x16f7ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x169aa5c -> 0x16f7ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x169aae1 -> 0x16f7ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::dispatch_groups_3d
// 信息 ID: f00ea2fa007d3f6e1ef73780
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi::dispatch_3d; module=518, inline_record=26316; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1632, inline_record=8728; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1632, inline_record=14024; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=50372; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=51204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::dispatch_indirect
// 信息 ID: dec39754b44d5bee7b4bc27a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::allocate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1632, inline_record=1072; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1632, inline_record=7440; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_unsorted_instance_indirection::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1632, inline_record=11436; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1632, inline_record=12760; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$1<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1632, inline_record=16644; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_sorted_instance_list::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1632, inline_record=18004; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=50672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::dispatch_threads_3d
// 信息 ID: dc10a0481973c616ce43ec61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi::dispatch_3d; module=518, inline_record=26140; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1632, inline_record=8640; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1632, inline_record=13936; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=51028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw
// 信息 ID: 499e860f393f67fa6ff90926
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=31940; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=35584; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=35708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indexed
// 信息 ID: 4405408fb31e436940a9afc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=35456; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=35800; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=36108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect
// 信息 ID: acb6b0bfe577126a85e2afbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=35928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect_count
// 信息 ID: 469d0cc5cdabde5021325303
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=36720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indirect
// 信息 ID: 637b37c18fef045993994da9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=36288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::draw_indirect_count
// 信息 ID: d6201e9d033507f3895ddb72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=36468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::end_render_pass
// 信息 ID: 7cc8409aca71a0f8647cd17c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x169941a -> 0x16f7850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::fill_buffer
// 信息 ID: 806fc840a719a712918438ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_buffer::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources> @ 0x9049ce -> 0x16f7c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::push_constants
// 信息 ID: e23ffe1739870e3fb6522161
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::allocate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15f8fc1 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15f92a9 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure_env$2,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15f93cf -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure_env$2,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15f9477 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure_env$0<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15f9657 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15f9a47 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15fa036 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15fa2cc -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15fa28b -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15fa24a -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15fa3f2 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_unsorted_instance_indirection::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15fa76a -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15fac97 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15fb27c -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15fb525 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x15fb4e0 -> 0x16f7a50 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::set_default_view_and_scissor
// 信息 ID: 3208a555239f7c5a9170bf18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1697bbe -> 0x16f7870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::set_line_width
// 信息 ID: 3280304b061541285179981c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1698709 -> 0x16f7960 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x169873d -> 0x16f7960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::set_stencil_ref
// 信息 ID: 2fde4b5efa81fa761080a021
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x16986cb -> 0x16f7920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$0
// 信息 ID: 17d7196402415d0b75b12ffb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::pass_api::impl$0::begin_render_pass", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0 <- arrayvec::arrayvec::impl$25::from_iter<tuple$<ash::vk::enums::Format,ash::vk::bitflags::ImageUsageFlags,ash::vk::bitflags::ImageCreateFlags>,9,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >,rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0 <- arrayvec::arrayvec::impl$25::from_iter<tuple$<ash::vk::enums::Format,ash::vk::bitflags::ImageUsageFlags,ash::vk::bitflags::ImageCreateFlags>,9,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >,rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ref$<rhapsody::rg::pass_api::RasterPassColorAttachment> >,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<rhapsody::rg::pass_api::RasterPassColorAttachment> >,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>; module=3314, inline_record=516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::impl$0::begin_render_pass::closure$1
// 信息 ID: ad1bb218fb136f74fa18c431
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::rg::pass_api::impl$0::begin_render_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1680, inline_record=1196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::impl$5::bind
// 信息 ID: aa5923126d7766857d20a154
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 111 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo> @ 0x2b0241 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo> @ 0x2b0341 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x2b0441 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x2b0541 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo> @ 0x2b0641 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x2b0741 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::read_rg_storage_buffer @ 0x906287 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::write_rg_storage_buffer @ 0x906914 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer @ 0x96dc02 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer @ 0x96de9f -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer_concurrent @ 0x96e141 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::spawn_boats::BoatSsbo> @ 0x96fe87 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo> @ 0x96ff87 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::sparkle::SparkleSsbo> @ 0x970087 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::rt::RtBvhSsbo> @ 0x970187 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::resources::critters::ButterflySsbo> @ 0x970287 -> 0xbd6c60 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::Ref::bind_uniform_buffer
// 信息 ID: c7c3f438f8b4499a1c2f1e25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x2b0141 -> 0xbd6c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults> @ 0x971587 -> 0xbd6c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x972887 -> 0xbd6c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo> @ 0x973287 -> 0xbd6c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::Ref::bind_view
// 信息 ID: 88c9baae11eaf854a10c3117
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 26 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1558d6 -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1562be -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2afc8e -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x3642ee -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x373350 -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x8edffb -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x8ee54b -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x8eec64 -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::raw::texture::Texture> > @ 0x8fd215 -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x9054d0 -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset> @ 0x9090e6 -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > > @ 0x96f39b -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > > @ 0x96fab4 -> 0xbd6c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x155836 -> 0xbd6c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x15621a -> 0xbd6c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2afbea -> 0xbd6c30 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
