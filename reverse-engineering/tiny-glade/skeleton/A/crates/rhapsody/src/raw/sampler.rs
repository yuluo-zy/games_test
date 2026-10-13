// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/sampler.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 726b8113960907d6c559567f01345771889dcf5b9fc899faff09fcca59300cfc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sampler.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 570, 3007
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
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_samplers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7908
// 首条位置: {"module":570,"inlinee":30984}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":43708,"parent_record_offset":43628,"ancestor_record_offset":43628,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":44252,"parent_record_offset":43628,"ancestor_record_offset":43628,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$287::default
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":44140,"parent_record_offset":43628,"ancestor_record_offset":43628,"inlinee":30988,"source_line":5362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":44364,"parent_record_offset":43628,"ancestor_record_offset":43628,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":44308,"parent_record_offset":43628,"ancestor_record_offset":43628,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::sampler::impl$0::create_samplers::closure$0
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/sampler.rs
//     evidence={"module":570,"record_offset":43860,"parent_record_offset":43628,"ancestor_record_offset":43628,"inlinee":30986,"source_line":25,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":43764,"parent_record_offset":43740,"ancestor_record_offset":43628,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":43788,"parent_record_offset":43764,"ancestor_record_offset":43628,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":43740,"parent_record_offset":43708,"ancestor_record_offset":43628,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$287::default
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":43884,"parent_record_offset":43860,"ancestor_record_offset":43628,"inlinee":30988,"source_line":5362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":570,"record_offset":44332,"parent_record_offset":44308,"ancestor_record_offset":43628,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":44276,"parent_record_offset":44252,"ancestor_record_offset":43628,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":43816,"parent_record_offset":43788,"ancestor_record_offset":43628,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::sampler::impl$0::create_samplers::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x790A
// 首条位置: {"module":570,"inlinee":30986}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::impl$287::default
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":43884,"parent_record_offset":43860,"ancestor_record_offset":43860,"inlinee":30988,"source_line":5362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::sampler::impl$0::create_samplers::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18181
// 首条位置: {"module":3007,"inlinee":98689}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::create_sampler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":3007,"record_offset":7112,"parent_record_offset":7084,"ancestor_record_offset":7084,"inlinee":31026,"source_line":1372,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3007,"record_offset":7228,"parent_record_offset":7084,"ancestor_record_offset":7084,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3007,"record_offset":7140,"parent_record_offset":7112,"ancestor_record_offset":7084,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3007,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":7084,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3007,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":7084,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::get_built_in_sampler
// 记录形式: compiled_procedure; instances=1
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":66824,"rva":7614272,"va":5376323392}
// 编译实例 RVAs: 0x742f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: sampler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6709260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323392,"raw_hex":"e8f49f99ff","size":5,"soft_fail":false,"target_va":5369614224,"va":5376323479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sampler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27269674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323392,"raw_hex":"e82a1aa001","size":5,"soft_fail":false,"target_va":5403593344,"va":5376323665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26655127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323392,"raw_hex":"e897b99601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376323812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":68068,"parent_record_offset":66824,"ancestor_record_offset":66824,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":570,"record_offset":66972,"parent_record_offset":66824,"ancestor_record_offset":66824,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":67868,"parent_record_offset":67840,"ancestor_record_offset":66824,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":570,"record_offset":67228,"parent_record_offset":67204,"ancestor_record_offset":66824,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":570,"record_offset":67312,"parent_record_offset":67288,"ancestor_record_offset":66824,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":68116,"parent_record_offset":68092,"ancestor_record_offset":66824,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":570,"record_offset":67252,"parent_record_offset":67228,"ancestor_record_offset":66824,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":570,"record_offset":67428,"parent_record_offset":67404,"ancestor_record_offset":66824,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":67772,"parent_record_offset":67748,"ancestor_record_offset":66824,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":67664,"parent_record_offset":67640,"ancestor_record_offset":66824,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":67952,"parent_record_offset":67924,"ancestor_record_offset":66824,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":67980,"parent_record_offset":67952,"ancestor_record_offset":66824,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":67896,"parent_record_offset":67868,"ancestor_record_offset":66824,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":570,"record_offset":67840,"parent_record_offset":67812,"ancestor_record_offset":66824,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":66996,"parent_record_offset":66972,"ancestor_record_offset":66824,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":67020,"parent_record_offset":66996,"ancestor_record_offset":66824,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":67812,"parent_record_offset":67592,"ancestor_record_offset":66824,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":67052,"parent_record_offset":67020,"ancestor_record_offset":66824,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67724,"parent_record_offset":67700,"ancestor_record_offset":66824,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67700,"parent_record_offset":67592,"ancestor_record_offset":66824,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67640,"parent_record_offset":67616,"ancestor_record_offset":66824,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67560,"parent_record_offset":67128,"ancestor_record_offset":66824,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67616,"parent_record_offset":67592,"ancestor_record_offset":66824,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67104,"parent_record_offset":67080,"ancestor_record_offset":66824,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67080,"parent_record_offset":67020,"ancestor_record_offset":66824,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67128,"parent_record_offset":67104,"ancestor_record_offset":66824,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":67372,"parent_record_offset":67344,"ancestor_record_offset":66824,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":67404,"parent_record_offset":67372,"ancestor_record_offset":66824,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":67344,"parent_record_offset":67128,"ancestor_record_offset":66824,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67176,"parent_record_offset":67128,"ancestor_record_offset":66824,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":67592,"parent_record_offset":67128,"ancestor_record_offset":66824,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":67204,"parent_record_offset":67128,"ancestor_record_offset":66824,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":67288,"parent_record_offset":67128,"ancestor_record_offset":66824,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":67468,"parent_record_offset":67128,"ancestor_record_offset":66824,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::sampler::impl$0::get_built_in_sampler::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/sampler.rs
//     evidence={"module":570,"record_offset":68092,"parent_record_offset":68068,"ancestor_record_offset":66824,"inlinee":31030,"source_line":159,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::sampler::impl$0::get_built_in_sampler::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7936
// 首条位置: {"module":570,"inlinee":31030}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":68116,"parent_record_offset":68092,"ancestor_record_offset":68092,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00570.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_samplers
// 信息 ID: 6de049aea6440c4ac015eae0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=43628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_built_in_sampler
// 信息 ID: 0af7d70de604e37dfed4d0d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::sampler::impl$0::create_samplers::closure$0
// 信息 ID: 1e8e5dba8b2f4d2fecf8b440
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::sampler::impl$0::create_samplers", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=43860; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=43916; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=43972; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=44028; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=44084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::sampler::impl$0::create_samplers::closure$1
// 信息 ID: ec4c90c1a1c8e0f8dd848464
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::sampler::impl$0::create_samplers", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1 <- std::collections::hash::map::impl$79::from_iter<ref$<str$>,ash::vk::definitions::Sampler,std::hash::random::RandomState,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,ash::vk::definitions::Sampler>,alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,ash::vk::definitions::Sampler>,hashbrown::map::impl$94::extend::closure_env$0<ref$<str$>,ash::vk::definitions::Sampler,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,ash::vk::definitions::Sampler>,alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,ash::vk::definitions::Sampler>,hashbrown::map::impl$94::extend::closure_env$0<ref$<str$>,ash::vk::definitions::Sampler,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,ash::vk::definitions::Sampler>,alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,ash::vk::definitions::Sampler>,hashbrown::map::impl$94::extend::closure_env$0<ref$<str$>,ash::vk::definitions::Sampler,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ash::vk::definitions::SamplerCreateInfo>,alloc::alloc::Global>,rhapsody::raw::sampler::impl$0::create_samplers::closure_env$1> > > >; module=3007, inline_record=7084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::sampler::impl$0::get_built_in_sampler::closure$0
// 信息 ID: 3a8c1f7382cc060002a3502a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::sampler::impl$0::get_built_in_sampler", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_built_in_sampler; module=570, inline_record=68092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
