// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/framebuffer.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9d8f420ca2f5543478811cfdb585a3f321982e0dc05fd1c7c6145f1aa345b57f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: framebuffer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 703, 1680, 1971, 3140
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::framebuffer::FramebufferDesc::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12BB8
// 首条位置: {"module":1680,"inlinee":76728}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1680,"record_offset":1892,"parent_record_offset":1792,"ancestor_record_offset":1792,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1680,"record_offset":1836,"parent_record_offset":1792,"ancestor_record_offset":1792,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":1920,"parent_record_offset":1792,"ancestor_record_offset":1792,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1680,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1792,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::framebuffer::impl$0::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1537D
// 首条位置: {"module":1971,"inlinee":86909}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1971,"record_offset":2088,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/01971.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1971,"record_offset":1968,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/01971.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1971,"record_offset":2112,"parent_record_offset":2088,"ancestor_record_offset":1932,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/01971.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1971,"record_offset":1996,"parent_record_offset":1968,"ancestor_record_offset":1932,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01971.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1971,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":1932,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01971.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1971,"record_offset":2052,"parent_record_offset":1996,"ancestor_record_offset":1932,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01971.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_framebuffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":115340,"rva":12566256,"va":5381275376}
// 编译实例 RVAs: 0xbfbef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418ae7e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: framebuffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13314034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275376,"raw_hex":"e8f227cb00","size":5,"soft_fail":false,"target_va":5394589664,"va":5381275625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t557155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275376,"raw_hex":"e863800800","size":5,"soft_fail":false,"target_va":5381832720,"va":5381275560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30265973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275376,"raw_hex":"e875d2cd01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381275798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30205883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275376,"raw_hex":"e8bbe7cc01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381275734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*624(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381275376,"raw_hex":"ff9770020000","size":6,"soft_fail":false,"va":5381275544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::create_framebuffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":115700,"parent_record_offset":115340,"ancestor_record_offset":115340,"inlinee":39722,"source_line":2340,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::FramebufferCreateInfo::layers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":115672,"parent_record_offset":115340,"ancestor_record_offset":115340,"inlinee":39721,"source_line":7792,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":115636,"parent_record_offset":115340,"ancestor_record_offset":115340,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":115880,"parent_record_offset":115340,"ancestor_record_offset":115340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":115820,"parent_record_offset":115340,"ancestor_record_offset":115340,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResources::create_framebuffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/framebuffer.rs
//     evidence={"module":703,"record_offset":116104,"parent_record_offset":115340,"ancestor_record_offset":115340,"inlinee":39723,"source_line":29,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":115508,"parent_record_offset":115340,"ancestor_record_offset":115340,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":115964,"parent_record_offset":115936,"ancestor_record_offset":115340,"inlinee":6205,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":115936,"parent_record_offset":115908,"ancestor_record_offset":115340,"inlinee":6203,"source_line":49,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":116052,"parent_record_offset":116020,"ancestor_record_offset":115340,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":703,"record_offset":115992,"parent_record_offset":115964,"ancestor_record_offset":115340,"inlinee":6207,"source_line":72,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":703,"record_offset":116020,"parent_record_offset":115992,"ancestor_record_offset":115340,"inlinee":6208,"source_line":76,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":115732,"parent_record_offset":115700,"ancestor_record_offset":115340,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":115756,"parent_record_offset":115732,"ancestor_record_offset":115340,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":115780,"parent_record_offset":115756,"ancestor_record_offset":115340,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":115908,"parent_record_offset":115880,"ancestor_record_offset":115340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::error::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/error.rs
//     evidence={"module":703,"record_offset":115844,"parent_record_offset":115820,"ancestor_record_offset":115340,"inlinee":39272,"source_line":21,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":115552,"parent_record_offset":115508,"ancestor_record_offset":115340,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":115604,"parent_record_offset":115508,"ancestor_record_offset":115340,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":115580,"parent_record_offset":115508,"ancestor_record_offset":115340,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$12::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":116164,"parent_record_offset":116128,"ancestor_record_offset":115340,"inlinee":39725,"source_line":429,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":116128,"parent_record_offset":116104,"ancestor_record_offset":115340,"inlinee":15367,"source_line":298,"archive":"_pdb/B/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: framebuffer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::framebuffer::FramebufferDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: framebuffer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: framebuffer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_framebuffer
// 信息 ID: 2878f9cafdcc689a0a2673fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass @ 0x16f7424 -> 0xbfbef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::framebuffer::FramebufferDesc::new
// 信息 ID: 426ea866a8a4d3c47beef4d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1680, inline_record=1792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::framebuffer::impl$0::new::closure$0
// 信息 ID: e6148dce67d512a6bf65d14a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::framebuffer::impl$0::new", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::framebuffer::impl$0::new::closure_env$0 <- arrayvec::arrayvec::impl$25::from_iter<tuple$<ash::vk::enums::Format,ash::vk::bitflags::ImageUsageFlags,ash::vk::bitflags::ImageCreateFlags>,9,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >,rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<tuple$<ash::vk::enums::Format,ash::vk::bitflags::ImageUsageFlags,ash::vk::bitflags::ImageCreateFlags>,9,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>,core::option::IntoIter<ref$<rhapsody::raw::texture::TextureDesc> > > >,rhapsody::raw::framebuffer::impl$0::new::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::pass_api::RasterPassColorAttachment>,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0> > > >; module=1971, inline_record=1932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
