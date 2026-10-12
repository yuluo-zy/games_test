// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/sampler.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: efed2d8a2df4cd878912effa29dd81c2dc7ead0695dcbb596fc68d0bd9d6e31b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sampler.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 703, 3273
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_samplers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B01
// 首条位置: {"module":703,"inlinee":39681}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":58228,"parent_record_offset":58132,"ancestor_record_offset":58132,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":58996,"parent_record_offset":58132,"ancestor_record_offset":58132,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$287::default
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":58884,"parent_record_offset":58132,"ancestor_record_offset":58132,"inlinee":39685,"source_line":5362,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":59108,"parent_record_offset":58132,"ancestor_record_offset":58132,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":59052,"parent_record_offset":58132,"ancestor_record_offset":58132,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::sampler::impl$0::create_samplers::closure$0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/sampler.rs
//     evidence={"module":703,"record_offset":58380,"parent_record_offset":58132,"ancestor_record_offset":58132,"inlinee":39683,"source_line":25,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":58284,"parent_record_offset":58260,"ancestor_record_offset":58132,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":58308,"parent_record_offset":58284,"ancestor_record_offset":58132,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":58260,"parent_record_offset":58228,"ancestor_record_offset":58132,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$287::default
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":58404,"parent_record_offset":58380,"ancestor_record_offset":58132,"inlinee":39685,"source_line":5362,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":703,"record_offset":59076,"parent_record_offset":59052,"ancestor_record_offset":58132,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":59020,"parent_record_offset":58996,"ancestor_record_offset":58132,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":58336,"parent_record_offset":58308,"ancestor_record_offset":58132,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::sampler::impl$0::create_samplers::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B03
// 首条位置: {"module":703,"inlinee":39683}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::impl$287::default
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":58404,"parent_record_offset":58380,"ancestor_record_offset":58380,"inlinee":39685,"source_line":5362,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::sampler::impl$0::create_samplers::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BBBF
// 首条位置: {"module":3273,"inlinee":113599}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::create_sampler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":3273,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":4040,"inlinee":39739,"source_line":1372,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3273,"record_offset":4184,"parent_record_offset":4040,"ancestor_record_offset":4040,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3273,"record_offset":4096,"parent_record_offset":4068,"ancestor_record_offset":4040,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3273,"record_offset":4120,"parent_record_offset":4096,"ancestor_record_offset":4040,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3273,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":4040,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::get_built_in_sampler
// 记录形式: compiled_procedure; instances=1
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":119832,"rva":12567984,"va":5381277104}
// 编译实例 RVAs: 0xbfc5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sampler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6597532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277104,"raw_hex":"e864549bff","size":5,"soft_fail":false,"target_va":5374679664,"va":5381277191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sampler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30569834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277104,"raw_hex":"e86a75d201","size":5,"soft_fail":false,"target_va":5411847152,"va":5381277313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30264389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277104,"raw_hex":"e845cccd01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381277510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":121120,"parent_record_offset":119832,"ancestor_record_offset":119832,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":703,"record_offset":119980,"parent_record_offset":119832,"ancestor_record_offset":119832,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":120768,"parent_record_offset":120740,"ancestor_record_offset":119832,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":703,"record_offset":120268,"parent_record_offset":120244,"ancestor_record_offset":119832,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":703,"record_offset":120352,"parent_record_offset":120328,"ancestor_record_offset":119832,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":121168,"parent_record_offset":121144,"ancestor_record_offset":119832,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":120292,"parent_record_offset":120268,"ancestor_record_offset":119832,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":120944,"parent_record_offset":120136,"ancestor_record_offset":119832,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":703,"record_offset":120500,"parent_record_offset":120472,"ancestor_record_offset":119832,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":703,"record_offset":120472,"parent_record_offset":120444,"ancestor_record_offset":119832,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":120620,"parent_record_offset":120596,"ancestor_record_offset":119832,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":120852,"parent_record_offset":120824,"ancestor_record_offset":119832,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":120880,"parent_record_offset":120852,"ancestor_record_offset":119832,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":120796,"parent_record_offset":120768,"ancestor_record_offset":119832,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":120412,"parent_record_offset":120384,"ancestor_record_offset":119832,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":120444,"parent_record_offset":120412,"ancestor_record_offset":119832,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":120384,"parent_record_offset":120136,"ancestor_record_offset":119832,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":120244,"parent_record_offset":120136,"ancestor_record_offset":119832,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":120972,"parent_record_offset":120136,"ancestor_record_offset":119832,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":120328,"parent_record_offset":120136,"ancestor_record_offset":119832,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":703,"record_offset":120216,"parent_record_offset":120136,"ancestor_record_offset":119832,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":703,"record_offset":120740,"parent_record_offset":120712,"ancestor_record_offset":119832,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":120004,"parent_record_offset":119980,"ancestor_record_offset":119832,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":120028,"parent_record_offset":120004,"ancestor_record_offset":119832,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":120712,"parent_record_offset":120544,"ancestor_record_offset":119832,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":120060,"parent_record_offset":120028,"ancestor_record_offset":119832,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":120652,"parent_record_offset":120568,"ancestor_record_offset":119832,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":120596,"parent_record_offset":120568,"ancestor_record_offset":119832,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":121064,"parent_record_offset":120136,"ancestor_record_offset":119832,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":120568,"parent_record_offset":120544,"ancestor_record_offset":119832,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":120112,"parent_record_offset":120088,"ancestor_record_offset":119832,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":120088,"parent_record_offset":120028,"ancestor_record_offset":119832,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":120136,"parent_record_offset":120112,"ancestor_record_offset":119832,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":120188,"parent_record_offset":120136,"ancestor_record_offset":119832,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":120544,"parent_record_offset":120136,"ancestor_record_offset":119832,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::sampler::impl$0::get_built_in_sampler::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/sampler.rs
//     evidence={"module":703,"record_offset":121144,"parent_record_offset":121120,"ancestor_record_offset":119832,"inlinee":39744,"source_line":201,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::sampler::impl$0::get_built_in_sampler::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B40
// 首条位置: {"module":703,"inlinee":39744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":121168,"parent_record_offset":121144,"ancestor_record_offset":121144,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: sampler.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sampler.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sampler.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_samplers
// 信息 ID: 2efcc8d9280c1cda39243691
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=58132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_built_in_sampler
// 信息 ID: bb7bcb42b2373721ea4a6c1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::sampler::impl$0::create_samplers::closure$0
// 信息 ID: ecc3a83593364a7e7404c266
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::sampler::impl$0::create_samplers", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=58380; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=58436; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=58492; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=58548; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=58604; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=58660; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=58716; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=58772; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::sampler::impl$0::create_samplers::closure$1
// 信息 ID: 283c0bc7e706d2cfe86c3171
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::sampler::impl$0::create_samplers", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1 <- std::collections::hash::map::impl$87::from_iter<ref$<str$>,ash::vk::definitions::Sampler,std::hash::random::RandomState,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1 <- alloc::vec::into_iter::impl$5::fold<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,tuple$<ref$<str$>,ash::vk::definitions::Sampler>,tuple$<>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,ash::vk::definitions::Sampler>,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,ash::vk::definitions::Sampler,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1 <- alloc::vec::into_iter::impl$5::fold<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,tuple$<ref$<str$>,ash::vk::definitions::Sampler>,tuple$<>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,ash::vk::definitions::Sampler>,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,ash::vk::definitions::Sampler,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::fold<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,tuple$<ref$<str$>,ash::vk::definitions::Sampler>,tuple$<>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,ash::vk::definitions::Sampler>,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,ash::vk::definitions::Sampler,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1> > > > >; module=3273, inline_record=4040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::sampler::impl$0::get_built_in_sampler::closure$0
// 信息 ID: 3f312656c2f6567f78a77e5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::sampler::impl$0::get_built_in_sampler", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_built_in_sampler; module=703, inline_record=121144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
