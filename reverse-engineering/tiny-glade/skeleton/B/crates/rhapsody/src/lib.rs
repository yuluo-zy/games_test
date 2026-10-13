// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: adde8e8ba61f39a6a516987d96df099592346a1cb7aa64ece3f43b75d33a0d8c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 683, 694, 699, 703, 707, 709, 1741, 1962, 1970, 1978, 1987, 1988, 3273
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use append_only_vec; // namespace candidate
//   package identities: [{"name":"append-only-vec","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8","version":"0.1.2"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use gpu_allocator; // namespace candidate
//   package identities: [{"name":"gpu-allocator","source":"local_or_path","version":"0.26.0"}]
// use leekspin; // namespace candidate
//   package identities: [{"name":"leekspin","source":"local_or_path","version":"0.1.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 记录形式: compiled_procedure; instances=8
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":699,"record_offset":3164,"rva":12491008,"va":5381200128}
// 编译实例 RVAs: 0xbe9900, 0xbeda20, 0xc030a0, 0xc0d740, 0x18c06a0, 0x18c30c0, 0x18c5370, 0x18c9ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=32
//     evidence={"assembly":"\tcallq\t30341664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381200128,"raw_hex":"e820face01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381200627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":699,"record_offset":3440,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":699,"record_offset":3568,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":699,"record_offset":3348,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":699,"record_offset":3720,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":5012,"source_line":1736,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":699,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":3164,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":699,"record_offset":3656,"parent_record_offset":3596,"ancestor_record_offset":3164,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":699,"record_offset":3596,"parent_record_offset":3568,"ancestor_record_offset":3164,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":699,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":3164,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":699,"record_offset":3468,"parent_record_offset":3440,"ancestor_record_offset":3164,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":699,"record_offset":3680,"parent_record_offset":3656,"ancestor_record_offset":3164,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":699,"record_offset":3400,"parent_record_offset":3348,"ancestor_record_offset":3164,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":5488,"rva":12596320,"va":5381305440}
// 编译实例 RVAs: 0xc03460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-135852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381305440,"raw_hex":"e854edfdff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381306199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-86814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381305440,"raw_hex":"e8e2acfeff","size":5,"soft_fail":false,"target_va":5381218816,"va":5381305625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c04840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381305440,"raw_hex":"e8f7100000","size":5,"soft_fail":false,"target_va":5381310528,"va":5381306180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30541397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381305440,"raw_hex":"e85506d201","size":5,"soft_fail":false,"target_va":5411847168,"va":5381305766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30235392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381305440,"raw_hex":"e8005bcd01","size":5,"soft_fail":false,"target_va":5411541808,"va":5381306411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30235708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381305440,"raw_hex":"e83c5ccd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381306351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30236731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381305440,"raw_hex":"e83b60cd01","size":5,"soft_fail":false,"target_va":5411543104,"va":5381306368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":707,"record_offset":5864,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$286::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":707,"record_offset":5780,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":15804,"source_line":45,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":6284,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":6508,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":5892,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":6108,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":707,"record_offset":5928,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":6136,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":38998,"source_line":108,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$4::byte_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":5724,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":39017,"source_line":152,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$4::copy_to_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":5996,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":39018,"source_line":156,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":707,"record_offset":5808,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::into_owner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":6080,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":13553,"source_line":310,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":707,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":5488,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":5488,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":5748,"parent_record_offset":5724,"ancestor_record_offset":5488,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":5488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":707,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5488,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":707,"record_offset":5956,"parent_record_offset":5928,"ancestor_record_offset":5488,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":41668,"rva":12613968,"va":5381323088}
// 编译实例 RVAs: 0xc07950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11168024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8e89655ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381326147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6242233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e847c0a0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381323716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6241962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e856c1a0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381323461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406868b0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5772396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e894eba7ff","size":5,"soft_fail":false,"target_va":5375551664,"va":5381324055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-167682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8fe70fdff","size":5,"soft_fail":false,"target_va":5381158416,"va":5381326093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-154819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e83da3fdff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381325166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfe270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-40511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8c161ffff","size":5,"soft_fail":false,"target_va":5381284464,"va":5381324970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d797d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1514887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8871d1700","size":5,"soft_fail":false,"target_va":5382838224,"va":5381323332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30522779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e89bbdd101","size":5,"soft_fail":false,"target_va":5411847168,"va":5381324384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30215319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8970ccd01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381325639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::AppendOnlyVec$LT$T$GT$::push
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     evidence={"assembly":"\tcallq\t13319483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e83b3dcb00","size":5,"soft_fail":false,"target_va":5394644272,"va":5381324784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     evidence={"assembly":"\tcallq\t37276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e89c910000","size":5,"soft_fail":false,"target_va":5381362096,"va":5381324815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30216846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e88e12cd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381325213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30216626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8b211cd01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381325665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30217464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8f814cd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381325411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocation::memory
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5468246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8aa8facff","size":5,"soft_fail":false,"target_va":5375855792,"va":5381324033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30167064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e81850cc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381325299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30170116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8045ccc01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381324679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t500066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e862a10700","size":5,"soft_fail":false,"target_va":5381823600,"va":5381323529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t500020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e834a10700","size":5,"soft_fail":false,"target_va":5381824016,"va":5381323991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t24272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8d05e0000","size":5,"soft_fail":false,"target_va":5381347904,"va":5381323627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::texture::ManagedTexture::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-178957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8f344fdff","size":5,"soft_fail":false,"target_va":5381145296,"va":5381324248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::get_image_create_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-203524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381323088,"raw_hex":"e8fce4fcff","size":5,"soft_fail":false,"target_va":5381119696,"va":5381323215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*416(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381323088,"raw_hex":"ff93a0010000","size":6,"soft_fail":false,"va":5381323280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":707,"record_offset":42280,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":707,"record_offset":42932,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::bind_image_memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":44288,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":15829,"source_line":2714,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_image
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":42108,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":15817,"source_line":2590,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_image_memory_requirements
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":42904,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":15826,"source_line":2624,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::set_debug_utils_object_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":707,"record_offset":42612,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":15825,"source_line":11,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$354::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":707,"record_offset":42052,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":15813,"source_line":45,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsObjectNameInfoEXT::object_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":707,"record_offset":42584,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":15823,"source_line":19604,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":707,"record_offset":41996,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":707,"record_offset":41940,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":42732,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":42696,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":43172,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":42308,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":44060,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":45020,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":42960,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":43116,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":707,"record_offset":42224,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::into_owner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":45260,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":13553,"source_line":310,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":707,"record_offset":44444,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":15367,"source_line":298,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":43632,"parent_record_offset":43608,"ancestor_record_offset":41668,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":43656,"parent_record_offset":43632,"ancestor_record_offset":41668,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":42528,"parent_record_offset":42504,"ancestor_record_offset":41668,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":43608,"parent_record_offset":43580,"ancestor_record_offset":41668,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":42504,"parent_record_offset":42448,"ancestor_record_offset":41668,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":707,"record_offset":42808,"parent_record_offset":42784,"ancestor_record_offset":41668,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":707,"record_offset":42756,"parent_record_offset":42732,"ancestor_record_offset":41668,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":43520,"parent_record_offset":43492,"ancestor_record_offset":41668,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":42476,"parent_record_offset":42448,"ancestor_record_offset":41668,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":42448,"parent_record_offset":42424,"ancestor_record_offset":41668,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":43580,"parent_record_offset":43548,"ancestor_record_offset":41668,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":43548,"parent_record_offset":43520,"ancestor_record_offset":41668,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":42424,"parent_record_offset":42400,"ancestor_record_offset":41668,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":707,"record_offset":43456,"parent_record_offset":43428,"ancestor_record_offset":41668,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":707,"record_offset":43428,"parent_record_offset":43400,"ancestor_record_offset":41668,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":707,"record_offset":43372,"parent_record_offset":43344,"ancestor_record_offset":41668,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":707,"record_offset":43400,"parent_record_offset":43372,"ancestor_record_offset":41668,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":707,"record_offset":43344,"parent_record_offset":43316,"ancestor_record_offset":41668,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":707,"record_offset":43316,"parent_record_offset":43288,"ancestor_record_offset":41668,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":707,"record_offset":43288,"parent_record_offset":43260,"ancestor_record_offset":41668,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":43492,"parent_record_offset":43456,"ancestor_record_offset":41668,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":707,"record_offset":44860,"parent_record_offset":44812,"ancestor_record_offset":41668,"inlinee":16141,"source_line":104,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":707,"record_offset":44812,"parent_record_offset":44536,"ancestor_record_offset":41668,"inlinee":39243,"source_line":225,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$346::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":707,"record_offset":42020,"parent_record_offset":41996,"ancestor_record_offset":41668,"inlinee":14690,"source_line":183,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$353::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":707,"record_offset":42076,"parent_record_offset":42052,"ancestor_record_offset":41668,"inlinee":15815,"source_line":39,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":707,"record_offset":42136,"parent_record_offset":42108,"ancestor_record_offset":41668,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":707,"record_offset":42160,"parent_record_offset":42136,"ancestor_record_offset":41668,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":707,"record_offset":42184,"parent_record_offset":42160,"ancestor_record_offset":41668,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":43140,"parent_record_offset":43116,"ancestor_record_offset":41668,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":707,"record_offset":43260,"parent_record_offset":43220,"ancestor_record_offset":41668,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$4::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":707,"record_offset":41964,"parent_record_offset":41940,"ancestor_record_offset":41668,"inlinee":10038,"source_line":770,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":707,"record_offset":43732,"parent_record_offset":43708,"ancestor_record_offset":41668,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":42248,"parent_record_offset":42224,"ancestor_record_offset":41668,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":707,"record_offset":43708,"parent_record_offset":43456,"ancestor_record_offset":41668,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":707,"record_offset":44688,"parent_record_offset":44664,"ancestor_record_offset":41668,"inlinee":27245,"source_line":39,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":707,"record_offset":44716,"parent_record_offset":44688,"ancestor_record_offset":41668,"inlinee":27247,"source_line":3874,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":42352,"parent_record_offset":42308,"ancestor_record_offset":41668,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":707,"record_offset":44944,"parent_record_offset":44812,"ancestor_record_offset":41668,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":707,"record_offset":44664,"parent_record_offset":44536,"ancestor_record_offset":41668,"inlinee":32600,"source_line":38,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":44756,"parent_record_offset":44536,"ancestor_record_offset":41668,"inlinee":39241,"source_line":1779,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":44608,"parent_record_offset":44536,"ancestor_record_offset":41668,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":44220,"parent_record_offset":44156,"ancestor_record_offset":41668,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":43024,"parent_record_offset":42988,"ancestor_record_offset":41668,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":44884,"parent_record_offset":44860,"ancestor_record_offset":41668,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":44244,"parent_record_offset":44220,"ancestor_record_offset":41668,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":43048,"parent_record_offset":43024,"ancestor_record_offset":41668,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":44632,"parent_record_offset":44608,"ancestor_record_offset":41668,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":44128,"parent_record_offset":44100,"ancestor_record_offset":41668,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":42988,"parent_record_offset":42960,"ancestor_record_offset":41668,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":44156,"parent_record_offset":44128,"ancestor_record_offset":41668,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":43080,"parent_record_offset":42988,"ancestor_record_offset":41668,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":44192,"parent_record_offset":44156,"ancestor_record_offset":41668,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedRef::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":44984,"parent_record_offset":44444,"ancestor_record_offset":41668,"inlinee":13554,"source_line":302,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":707,"record_offset":44536,"parent_record_offset":44496,"ancestor_record_offset":41668,"inlinee":39240,"source_line":71,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$6::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":707,"record_offset":44496,"parent_record_offset":44444,"ancestor_record_offset":41668,"inlinee":39274,"source_line":369,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$9::create_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":707,"record_offset":43220,"parent_record_offset":43172,"ancestor_record_offset":41668,"inlinee":15828,"source_line":608,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::update_buffer_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":13880,"rva":12513184,"va":5381222304}
// 编译实例 RVAs: 0xbeefa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-52761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8e731ffff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381223108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8d9f0ffff","size":5,"soft_fail":false,"target_va":5381218816,"va":5381222690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c04280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t86076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e83c500100","size":5,"soft_fail":false,"target_va":5381309056,"va":5381222975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30624295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8274ad301","size":5,"soft_fail":false,"target_va":5411847168,"va":5381222868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30318342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8069fce01","size":5,"soft_fail":false,"target_va":5411541808,"va":5381223461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30318334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8fe9ece01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381223437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30258340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8a4b4cd01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381223277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30319506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e892a3ce01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381222553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30318498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8a29fce01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381223401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30319938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e842a5ce01","size":5,"soft_fail":false,"target_va":5411543104,"va":5381223161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30319972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e864a5ce01","size":5,"soft_fail":false,"target_va":5411543120,"va":5381223143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":14644,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::BufferUsageFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":703,"record_offset":14520,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":39594,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":15484,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":14708,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":14212,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":15372,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":14672,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":15316,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":703,"record_offset":15056,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14420,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14988,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":14360,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":703,"record_offset":14248,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":703,"record_offset":14548,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":38998,"source_line":108,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":14584,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":14084,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":15080,"parent_record_offset":15056,"ancestor_record_offset":13880,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":14484,"parent_record_offset":14448,"ancestor_record_offset":13880,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":14612,"parent_record_offset":14584,"ancestor_record_offset":13880,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":13880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":13880,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":14388,"parent_record_offset":14360,"ancestor_record_offset":13880,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":703,"record_offset":14272,"parent_record_offset":14248,"ancestor_record_offset":13880,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15232,"parent_record_offset":15208,"ancestor_record_offset":13880,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":13880,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15208,"parent_record_offset":15184,"ancestor_record_offset":13880,"inlinee":5242,"source_line":112,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14924,"parent_record_offset":14852,"ancestor_record_offset":13880,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15184,"parent_record_offset":15160,"ancestor_record_offset":13880,"inlinee":5241,"source_line":413,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14448,"parent_record_offset":14420,"ancestor_record_offset":13880,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15016,"parent_record_offset":14988,"ancestor_record_offset":13880,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15160,"parent_record_offset":15136,"ancestor_record_offset":13880,"inlinee":5240,"source_line":553,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15136,"parent_record_offset":15112,"ancestor_record_offset":13880,"inlinee":5239,"source_line":568,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":703,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":13880,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":14128,"parent_record_offset":14084,"ancestor_record_offset":13880,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":14180,"parent_record_offset":14084,"ancestor_record_offset":13880,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":14156,"parent_record_offset":14084,"ancestor_record_offset":13880,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::driver_support
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":33732,"rva":12429712,"va":5381138832}
// 编译实例 RVAs: 0xbda990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7250
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138832,"raw_hex":"e891c8ffff","size":5,"soft_fail":false,"target_va":5381124688,"va":5381138874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::is_old_intel
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138832,"raw_hex":"e80b030000","size":5,"soft_fail":false,"target_va":5381140256,"va":5381139472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$17::lt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":683,"record_offset":33904,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":39198,"source_line":387,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":33880,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":39195,"source_line":651,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":33952,"parent_record_offset":33928,"ancestor_record_offset":33732,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":33928,"parent_record_offset":33904,"ancestor_record_offset":33732,"inlinee":9733,"source_line":1939,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":34112,"parent_record_offset":34084,"ancestor_record_offset":33732,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$65::cmp
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":34204,"parent_record_offset":34024,"ancestor_record_offset":33732,"inlinee":10268,"source_line":1836,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":34140,"parent_record_offset":34084,"ancestor_record_offset":33732,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":34084,"parent_record_offset":34056,"ancestor_record_offset":33732,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":34056,"parent_record_offset":34024,"ancestor_record_offset":33732,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":34164,"parent_record_offset":34140,"ancestor_record_offset":33732,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$3::partial_cmp
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":33976,"parent_record_offset":33952,"ancestor_record_offset":33732,"inlinee":8340,"source_line":37,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$7::partial_compare
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":34000,"parent_record_offset":33976,"ancestor_record_offset":33732,"inlinee":8342,"source_line":136,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$8::compare
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":34024,"parent_record_offset":34000,"ancestor_record_offset":33732,"inlinee":18974,"source_line":167,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1988,"record_offset":1408,"rva":26006736,"va":5394715856}
// 编译实例 RVAs: 0x18cd4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19634459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394715856,"raw_hex":"e8e566d4fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5394715942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19634559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394715856,"raw_hex":"e88166d4fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394716058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16824865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394715856,"raw_hex":"e821ba0001","size":5,"soft_fail":false,"target_va":5411540963,"va":5394716093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*72(%r14)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5394715856,"raw_hex":"41ff5648","size":4,"soft_fail":false,"va":5394715896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1988,"record_offset":1704,"parent_record_offset":1408,"ancestor_record_offset":1408,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1988,"record_offset":2196,"parent_record_offset":1408,"ancestor_record_offset":1408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1988,"record_offset":2112,"parent_record_offset":1408,"ancestor_record_offset":1408,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::enumerate_physical_devices::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1988,"record_offset":1672,"parent_record_offset":1408,"ancestor_record_offset":1408,"inlinee":87502,"source_line":336,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1408,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1408,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":1408,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2008,"parent_record_offset":1832,"ancestor_record_offset":1408,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2324,"parent_record_offset":2268,"ancestor_record_offset":1408,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1768,"parent_record_offset":1736,"ancestor_record_offset":1408,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":2296,"parent_record_offset":2268,"ancestor_record_offset":1408,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":1408,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1832,"parent_record_offset":1800,"ancestor_record_offset":1408,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1800,"parent_record_offset":1768,"ancestor_record_offset":1408,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1408,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1868,"parent_record_offset":1832,"ancestor_record_offset":1408,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1988,"record_offset":1736,"parent_record_offset":1704,"ancestor_record_offset":1408,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1988,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":1408,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1988,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":1408,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1988,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":1408,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1988,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":1408,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1988,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::init_logging
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":694,"record_offset":256,"rva":12479120,"va":5381188240}
// 编译实例 RVAs: 0xbe6a90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6106902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188240,"raw_hex":"e8ead0a2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381188401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30353713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188240,"raw_hex":"e83129cf01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381188346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13477984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188240,"raw_hex":"e860a8cd00","size":5,"soft_fail":false,"target_va":5394666256,"va":5381188267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::leek::leekspin_init
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":694,"record_offset":364,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":39326,"source_line":16,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":694,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":694,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":694,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":256,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":694,"record_offset":428,"parent_record_offset":404,"ancestor_record_offset":256,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":694,"record_offset":516,"parent_record_offset":364,"ancestor_record_offset":256,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":694,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":256,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":694,"record_offset":404,"parent_record_offset":364,"ancestor_record_offset":256,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":694,"record_offset":572,"parent_record_offset":364,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":694,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":256,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":694,"record_offset":480,"parent_record_offset":428,"ancestor_record_offset":256,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::print_log_dump
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":694,"record_offset":752,"rva":12479296,"va":5381188416}
// 编译实例 RVAs: 0xbe6b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] rhapsody::leek::leekspin_print_dump
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-16389","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381188416,"raw_hex":"e9fbbfffff","size":5,"soft_fail":false,"target_va":5381172032,"va":5381188416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::dump_log
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":694,"record_offset":852,"rva":12479312,"va":5381188432}
// 编译实例 RVAs: 0xbe6b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30353293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188432,"raw_hex":"e88d27cf01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381188478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::to_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13478082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188432,"raw_hex":"e8c2a8cd00","size":5,"soft_fail":false,"target_va":5394666544,"va":5381188457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::leek::leekspin_dump
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":694,"record_offset":956,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":39373,"source_line":63,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":694,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":852,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00694.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":694,"record_offset":1004,"parent_record_offset":956,"ancestor_record_offset":852,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00694.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 935fd15b7fefbfb868c4a7b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next_back -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0xbe98b7 -> 0xbe9900 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0xbed9d7 -> 0xbeda20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0xc03057 -> 0xc030a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0xc0d6f7 -> 0xc0d740 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x18c0657 -> 0x18c06a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x18c3077 -> 0x18c30c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x18c5327 -> 0x18c5370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x18c9b57 -> 0x18c9ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::dump_log
// 信息 ID: e21c0904d523f83fb53cfc9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_rhapsody_log @ 0x3ae7ab -> 0xbe6b50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::init_logging
// 信息 ID: 025d230e8db6c2128630cd1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dd63b -> 0xbe6a90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::print_log_dump
// 信息 ID: 18bea2df788081552835ca5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3db66a -> 0xbe6b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// 信息 ID: f995ff9afb109c2f6dc8f999
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_buffer -> ["enum2$<core::option::Option<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources @ 0xbe84bc -> 0xc03460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::new @ 0xbea3b1 -> 0xc03460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> >
// 信息 ID: cb89daa2e50bdf9450ba0569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::raw::texture::TextureInitialData"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_texture -> ["enum2$<rhapsody::raw::texture::TextureInitialData>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources @ 0xbe841f -> 0xc07950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_buffer_bytes
// 信息 ID: e2e46da22ae17b7565857c2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 36 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x2b247e -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x2b266d -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x2b2852 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::maintain_lantern_lights @ 0x790f05 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::dormer_holes::upload_dormer_holes_to_gpu @ 0x7b3732 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::dormer_holes::upload_dormer_holes_to_gpu @ 0x7b38b8 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x8bd4b5 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x8bd5b1 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x8bd6b7 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf @ 0x8c1a1b -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf @ 0x8c1c6e -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::color_library::update_color_library @ 0x93b89c -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings @ 0xa32a3f -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::reload_nani @ 0xa39c64 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xa79723 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa7b618 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::driver_support
// 信息 ID: f5a0976496154eae61b878c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5964c8 -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x598dbd -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::init::AvailableGraphicsDevice::is_supported @ 0xbda852 -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible @ 0xbda902 -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_gpu_selector::gpu_selection::impl$2::view @ 0x21bc6d7 -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_gpu_selector::gpu_selection::device_driver_advice @ 0x21be62d -> 0xbda990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>
// 信息 ID: 995607e5685e99e5aa6ca567
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::physical_device::enumerate_vk_data -> ["u32", "ash::vk::definitions::PhysicalDevice", "rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::physical_device::enumerate_physical_devices @ 0xc01e11 -> 0x18cd4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
