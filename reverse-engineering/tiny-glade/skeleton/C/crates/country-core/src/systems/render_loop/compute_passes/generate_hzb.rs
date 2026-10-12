// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d917b64c709cf41bfb06ccebac42919accb22f3ecd9c3045b267c0da2bba92c6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: generate_hzb.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 517, 1740
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.6"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use typed_index_collections; // namespace candidate
//   package identities: [{"name":"typed-index-collections","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":41440,"rva":10169408,"va":5378878528}
// 编译实例 RVAs: 0x9b2c40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4593439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e81f174600","size":5,"soft_fail":false,"target_va":5383472768,"va":5378879324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409d8900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t151564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e80c500200","size":5,"soft_fail":false,"target_va":5379033344,"va":5378881775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14406952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e828d5db00","size":5,"soft_fail":false,"target_va":5393288768,"va":5378881811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417969d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14561384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e86830de00","size":5,"soft_fail":false,"target_va":5393443280,"va":5378881891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38175182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e8ce814602","size":5,"soft_fail":false,"target_va":5417057056,"va":5378881869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37492247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e817163c02","size":5,"soft_fail":false,"target_va":5416370864,"va":5378878612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arrayvec::arrayvec::extend_panic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37731251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e8b3bb3f02","size":5,"soft_fail":false,"target_va":5416612592,"va":5378881336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37739326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e83edb3f02","size":5,"soft_fail":false,"target_va":5416621632,"va":5378882301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::clear_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4662337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e841244700","size":5,"soft_fail":false,"target_va":5383541136,"va":5378878794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4669401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e8d93f4700","size":5,"soft_fail":false,"target_va":5383548144,"va":5378878738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4654717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e87d064700","size":5,"soft_fail":false,"target_va":5383536672,"va":5378881950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4657303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e897104700","size":5,"soft_fail":false,"target_va":5383539152,"va":5378881844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::BufferDesc::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4645489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e871e24600","size":5,"soft_fail":false,"target_va":5383524160,"va":5378878666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::TexView::with_layer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4644676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e844df4600","size":5,"soft_fail":false,"target_va":5383524560,"va":5378879879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::TexView::with_ty_override
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4644763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378878528,"raw_hex":"e89bdf4600","size":5,"soft_fail":false,"target_va":5383524656,"va":5378879888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":41604,"parent_record_offset":41440,"ancestor_record_offset":41440,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":41764,"parent_record_offset":41732,"ancestor_record_offset":41440,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":41812,"parent_record_offset":41788,"ancestor_record_offset":41440,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":41788,"parent_record_offset":41764,"ancestor_record_offset":41440,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":41732,"parent_record_offset":41696,"ancestor_record_offset":41440,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":41672,"parent_record_offset":41648,"ancestor_record_offset":41440,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":41696,"parent_record_offset":41672,"ancestor_record_offset":41440,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_mut_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42856,"parent_record_offset":42832,"ancestor_record_offset":41440,"inlinee":30192,"source_line":695,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::extend_from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42732,"parent_record_offset":42708,"ancestor_record_offset":41440,"inlinee":13951,"source_line":1080,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::into_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":43252,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":30195,"source_line":657,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$24::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42708,"parent_record_offset":42676,"ancestor_record_offset":41440,"inlinee":13950,"source_line":1058,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$26::from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42676,"parent_record_offset":42652,"ancestor_record_offset":41440,"inlinee":13948,"source_line":1145,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$4::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42832,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":30191,"source_line":740,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_mut_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":517,"record_offset":42880,"parent_record_offset":42856,"ancestor_record_offset":41440,"inlinee":30193,"source_line":25,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":517,"record_offset":42480,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":517,"record_offset":42612,"parent_record_offset":42584,"ancestor_record_offset":41440,"inlinee":22737,"source_line":1916,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":517,"record_offset":43028,"parent_record_offset":42992,"ancestor_record_offset":41440,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":517,"record_offset":42584,"parent_record_offset":42556,"ancestor_record_offset":41440,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":517,"record_offset":42556,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":517,"record_offset":42652,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":42412,"parent_record_offset":42380,"ancestor_record_offset":41440,"inlinee":7461,"source_line":153,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":42088,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":30180,"source_line":3806,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":42112,"parent_record_offset":42088,"ancestor_record_offset":41440,"inlinee":30181,"source_line":3775,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":42760,"parent_record_offset":42732,"ancestor_record_offset":41440,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":43084,"parent_record_offset":43056,"ancestor_record_offset":41440,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":41852,"parent_record_offset":41696,"ancestor_record_offset":41440,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":517,"record_offset":43436,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":43056,"parent_record_offset":43028,"ancestor_record_offset":41440,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs
//     evidence={"module":517,"record_offset":41892,"parent_record_offset":41604,"ancestor_record_offset":41440,"inlinee":30179,"source_line":27,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::TextureType::is_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":517,"record_offset":42244,"parent_record_offset":42220,"ancestor_record_offset":41440,"inlinee":30184,"source_line":313,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::mip_count_1d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":517,"record_offset":42380,"parent_record_offset":42356,"ancestor_record_offset":41440,"inlinee":30189,"source_line":443,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":43368,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":42028,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_groups
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":43408,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":30197,"source_line":228,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":43280,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":15344,"source_line":680,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::into_view_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":43336,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":30196,"source_line":732,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::with_view
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":43192,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":30194,"source_line":696,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42060,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::full_mip_chain_length
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42356,"parent_record_offset":42332,"ancestor_record_offset":41440,"inlinee":30187,"source_line":349,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::is_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42220,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":30182,"source_line":419,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42300,"parent_record_offset":42276,"ancestor_record_offset":41440,"inlinee":12369,"source_line":287,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::new_2d_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42276,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":30185,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::with_all_mip_levels
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42332,"parent_record_offset":41892,"ancestor_record_offset":41440,"inlinee":30186,"source_line":354,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::impl$59::clone
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":43220,"parent_record_offset":43192,"ancestor_record_offset":41440,"inlinee":15346,"source_line":182,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":517,"record_offset":41648,"parent_record_offset":41604,"ancestor_record_offset":41440,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75E3
// 首条位置: {"module":517,"inlinee":30179}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::into_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":43252,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":30195,"source_line":657,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$4::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42832,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":30191,"source_line":740,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":517,"record_offset":42480,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":517,"record_offset":42556,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":517,"record_offset":42652,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$7::next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":42088,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":30180,"source_line":3806,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":517,"record_offset":43436,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":43368,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":42028,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_groups
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":43408,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":30197,"source_line":228,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":43280,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":15344,"source_line":680,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::into_view_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":43336,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":30196,"source_line":732,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::with_view
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":43192,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":30194,"source_line":696,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42060,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::is_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42220,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":30182,"source_line":419,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::new_2d_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42276,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":30185,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::with_all_mip_levels
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42332,"parent_record_offset":41892,"ancestor_record_offset":41892,"inlinee":30186,"source_line":354,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_mut_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42856,"parent_record_offset":42832,"ancestor_record_offset":41892,"inlinee":30192,"source_line":695,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::extend_from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42732,"parent_record_offset":42708,"ancestor_record_offset":41892,"inlinee":13951,"source_line":1080,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$24::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42708,"parent_record_offset":42676,"ancestor_record_offset":41892,"inlinee":13950,"source_line":1058,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$26::from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":517,"record_offset":42676,"parent_record_offset":42652,"ancestor_record_offset":41892,"inlinee":13948,"source_line":1145,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_mut_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":517,"record_offset":42880,"parent_record_offset":42856,"ancestor_record_offset":41892,"inlinee":30193,"source_line":25,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":517,"record_offset":42612,"parent_record_offset":42584,"ancestor_record_offset":41892,"inlinee":22737,"source_line":1916,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":517,"record_offset":43028,"parent_record_offset":42992,"ancestor_record_offset":41892,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":517,"record_offset":42584,"parent_record_offset":42556,"ancestor_record_offset":41892,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":42412,"parent_record_offset":42380,"ancestor_record_offset":41892,"inlinee":7461,"source_line":153,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":42112,"parent_record_offset":42088,"ancestor_record_offset":41892,"inlinee":30181,"source_line":3775,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":42760,"parent_record_offset":42732,"ancestor_record_offset":41892,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":43084,"parent_record_offset":43056,"ancestor_record_offset":41892,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":42784,"parent_record_offset":42760,"ancestor_record_offset":41892,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":43056,"parent_record_offset":43028,"ancestor_record_offset":41892,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::TextureType::is_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":517,"record_offset":42244,"parent_record_offset":42220,"ancestor_record_offset":41892,"inlinee":30184,"source_line":313,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::mip_count_1d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":517,"record_offset":42380,"parent_record_offset":42356,"ancestor_record_offset":41892,"inlinee":30189,"source_line":443,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::full_mip_chain_length
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42356,"parent_record_offset":42332,"ancestor_record_offset":41892,"inlinee":30187,"source_line":349,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":42300,"parent_record_offset":42276,"ancestor_record_offset":41892,"inlinee":12369,"source_line":287,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::impl$59::clone
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":517,"record_offset":43220,"parent_record_offset":43192,"ancestor_record_offset":41892,"inlinee":15346,"source_line":182,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb::closure$0::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x146FE
// 首条位置: {"module":1740,"inlinee":83710}
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
// 完整结构及原始类型记录: generate_hzb.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: generate_hzb.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: generate_hzb.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5682156,"position_among_mapped_render_targets":33,"mapped_render_target_count":74,"previous_callsite":5682125,"next_callsite":5682350,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5684613,"position_among_mapped_render_targets":42,"mapped_render_target_count":74,"previous_callsite":5683501,"next_callsite":5684723,"previous_target_files":["rhapsody2/render-std/rhapsody-shadow/src/lib.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: generate_hzb.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
// 信息 ID: 47c1ee79aeb453b57c69d9b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b3ec -> 0x9b2c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56bd85 -> 0x9b2c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb::closure$0
// 信息 ID: 4a3238749b04adc0f89ef160
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb; module=517, inline_record=41892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb::closure$0::closure$0
// 信息 ID: 2ccbc3c363443932e57ef08b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
