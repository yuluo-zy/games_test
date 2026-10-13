// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/onboarding/src/hint_unlink_windows.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1b69268d89ffb5aafddbdc514f441d8ea2edb3fd726be8411dea55057a740b01
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: hint_unlink_windows.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3962
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use system_onboarding; // namespace candidate
//   package identities: [{"name":"system-onboarding","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_unlink_windows::hint_unlink_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3962,"record_offset":272,"rva":42422416,"va":5411131536}
// 编译实例 RVAs: 0x2875090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-35453742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e8d204e3fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5411132425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-35454250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e8d602e3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411132949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: hint_unlink_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42410225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e80fdf78fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5411132412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108a8c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_unlink_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25079932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e8844f81fe","size":5,"soft_fail":false,"target_va":5386053824,"va":5411133751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108ac40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: hint_unlink_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25078406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e87a5581fe","size":5,"soft_fail":false,"target_va":5386054720,"va":5411133121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_unlink_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23882486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e80a9593fe","size":5,"soft_fail":false,"target_va":5387249456,"va":5411131937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420bc360
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: hint_unlink_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8097719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e8497084ff","size":5,"soft_fail":false,"target_va":5403034464,"va":5411132178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_unlink_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7940625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e8efd586ff","size":5,"soft_fail":false,"target_va":5403191360,"va":5411131980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_unlink_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5219126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e836a34f00","size":5,"soft_fail":false,"target_va":5416351056,"va":5411131925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_unlink_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5218598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e826a14f00","size":5,"soft_fail":false,"target_va":5416351392,"va":5411132789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: hint_unlink_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5924500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e894665a00","size":5,"soft_fail":false,"target_va":5417057056,"va":5411132551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5248872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e868175000","size":5,"soft_fail":false,"target_va":5416381904,"va":5411133027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5487481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e879bb5300","size":5,"soft_fail":false,"target_va":5416620794,"va":5411133308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<system_onboarding::hint_binding::BindingHintDisplay> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411131536,"raw_hex":"e8b1000000","size":5,"soft_fail":false,"target_va":5411133872,"va":5411133690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3962,"record_offset":1944,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3962,"record_offset":1524,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3962,"record_offset":448,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3962,"record_offset":1144,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3962,"record_offset":3848,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3962,"record_offset":2000,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":108016,"source_line":81,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_content::disable_snapping_hint
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_content.rs
//     evidence={"module":3962,"record_offset":1828,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":134570,"source_line":10,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_state::HintDisplayState::add_always_on
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_state.rs
//     evidence={"module":3962,"record_offset":3556,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":108042,"source_line":146,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_state::HintDisplayState::add_popup_candidate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_state.rs
//     evidence={"module":3962,"record_offset":6240,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":108043,"source_line":150,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3962,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":272,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3962,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":272,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3962,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3962,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3962,"record_offset":2804,"parent_record_offset":2780,"ancestor_record_offset":272,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3962,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3962,"record_offset":2732,"parent_record_offset":2576,"ancestor_record_offset":272,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3962,"record_offset":4112,"parent_record_offset":4056,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":3962,"record_offset":2264,"parent_record_offset":2204,"ancestor_record_offset":272,"inlinee":18893,"source_line":197,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":1360,"parent_record_offset":1328,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":2508,"parent_record_offset":2476,"ancestor_record_offset":272,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":4084,"parent_record_offset":4056,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":2576,"parent_record_offset":2540,"ancestor_record_offset":272,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":2540,"parent_record_offset":2508,"ancestor_record_offset":272,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3962,"record_offset":2620,"parent_record_offset":2576,"ancestor_record_offset":272,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3962,"record_offset":2368,"parent_record_offset":2336,"ancestor_record_offset":272,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3962,"record_offset":2400,"parent_record_offset":2368,"ancestor_record_offset":272,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3962,"record_offset":2432,"parent_record_offset":2400,"ancestor_record_offset":272,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3962,"record_offset":2336,"parent_record_offset":2304,"ancestor_record_offset":272,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":3962,"record_offset":2304,"parent_record_offset":2264,"ancestor_record_offset":272,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3962,"record_offset":1328,"parent_record_offset":1288,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3962,"record_offset":788,"parent_record_offset":764,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3962,"record_offset":1256,"parent_record_offset":1212,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3962,"record_offset":1288,"parent_record_offset":1256,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3962,"record_offset":2476,"parent_record_offset":2432,"ancestor_record_offset":272,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3962,"record_offset":2940,"parent_record_offset":2204,"ancestor_record_offset":272,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3962,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":272,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3962,"record_offset":764,"parent_record_offset":536,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3962,"record_offset":1488,"parent_record_offset":1172,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3962,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":272,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3962,"record_offset":1552,"parent_record_offset":1524,"ancestor_record_offset":272,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3962,"record_offset":512,"parent_record_offset":472,"ancestor_record_offset":272,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3962,"record_offset":1172,"parent_record_offset":1144,"ancestor_record_offset":272,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3962,"record_offset":1212,"parent_record_offset":1172,"ancestor_record_offset":272,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3962,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":272,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3962,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":272,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3962,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3962,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":272,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3962,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":272,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3962,"record_offset":1592,"parent_record_offset":1552,"ancestor_record_offset":272,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3962,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":272,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3962,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":272,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3962,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":272,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3962,"record_offset":3280,"parent_record_offset":3252,"ancestor_record_offset":272,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3962,"record_offset":580,"parent_record_offset":536,"ancestor_record_offset":272,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3962,"record_offset":4540,"parent_record_offset":4508,"ancestor_record_offset":272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3962,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":272,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3962,"record_offset":1676,"parent_record_offset":1592,"ancestor_record_offset":272,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3962,"record_offset":2860,"parent_record_offset":2432,"ancestor_record_offset":272,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3962,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":272,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3962,"record_offset":3808,"parent_record_offset":3616,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3962,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3962,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3962,"record_offset":1448,"parent_record_offset":1288,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3962,"record_offset":1912,"parent_record_offset":1828,"ancestor_record_offset":272,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3962,"record_offset":636,"parent_record_offset":536,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3962,"record_offset":988,"parent_record_offset":536,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3962,"record_offset":692,"parent_record_offset":664,"ancestor_record_offset":272,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3962,"record_offset":664,"parent_record_offset":636,"ancestor_record_offset":272,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3962,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3962,"record_offset":1752,"parent_record_offset":1700,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":3962,"record_offset":2132,"parent_record_offset":2000,"ancestor_record_offset":272,"inlinee":108020,"source_line":213,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3962,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":272,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::impl$10::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3962,"record_offset":2204,"parent_record_offset":2160,"ancestor_record_offset":272,"inlinee":108024,"source_line":21,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3962,"record_offset":2100,"parent_record_offset":2000,"ancestor_record_offset":272,"inlinee":108018,"source_line":31,"archive":"_pdb/C/modules/03962.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3962,"record_offset":2160,"parent_record_offset":2000,"ancestor_record_offset":272,"inlinee":108022,"source_line":14,"archive":"_pdb/C/modules/03962.jsonl.gz"}
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
// 完整结构及原始类型记录: hint_unlink_windows.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: hint_unlink_windows.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: hint_unlink_windows.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemWindow>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemWindow>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemWindow>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_unlink_windows::hint_unlink_windows
// 信息 ID: 4eaa1d7fa3614b23425fd9ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7deb0045f1e7e103cd7e9cd4: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemWindow>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemWindow>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemWindow>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>)> > @ 0x20f87c5 -> 0x2875090 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemWindow>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemWindow>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>)> @ 0x21070a6 -> 0x2875090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
