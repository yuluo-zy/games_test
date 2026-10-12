// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/rg/temporal.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 443cc3a26e7abc205d99ad99531ec18f743e9defa67eb82882acafc9cce77a23
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: temporal.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 14, 41, 70, 127, 134, 197, 256, 387, 424, 441, 449, 462, 561, 1320
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"PDB_registry_path_only","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":70,"record_offset":3044,"rva":895744,"va":5369604864}
// 编译实例 RVAs: 0xdab00, 0x5ab500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ustr..Ustr$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6743323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e81be56600","size":5,"soft_fail":false,"target_va":5376348240,"va":5369604912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=2
//     target_aliases=1889 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-885223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e8197ef2ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5369605890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e8a6280000","size":5,"soft_fail":false,"target_va":5369615360,"va":5369604949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140264050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1610308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e844921800","size":5,"soft_fail":false,"target_va":5371215952,"va":5369605639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14026c110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1643283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e813131900","size":5,"soft_fail":false,"target_va":5371248912,"va":5369605624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403779e0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2738469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e825c92900","size":5,"soft_fail":false,"target_va":5372344800,"va":5369606326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140504d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-682636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374653696,"raw_hex":"e87495f5ff","size":5,"soft_fail":false,"target_va":5373971840,"va":5374654471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6634009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e8193a6500","size":5,"soft_fail":false,"target_va":5376239920,"va":5369605906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407480f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6739016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e848d46600","size":5,"soft_fail":false,"target_va":5376344304,"va":5369605283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e93930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9339862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374653696,"raw_hex":"e8d6838e00","size":5,"soft_fail":false,"target_va":5383993648,"va":5374653781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e9f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9387011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374653696,"raw_hex":"e8033c8f00","size":5,"soft_fail":false,"target_va":5384041472,"va":5374654456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f338e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9994225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374653696,"raw_hex":"e8f17f9800","size":5,"soft_fail":false,"target_va":5384648928,"va":5374654698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209cd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33299651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e8c31cfc01","size":5,"soft_fail":false,"target_va":5402905856,"va":5369606200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28303299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374653696,"raw_hex":"e8c3dfaf01","size":5,"soft_fail":false,"target_va":5402958336,"va":5374655032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28436686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374653696,"raw_hex":"e8cee8b101","size":5,"soft_fail":false,"target_va":5403091664,"va":5374654973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7923750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e826e87800","size":5,"soft_fail":false,"target_va":5377529888,"va":5369606133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t33339630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e8eeb8fc01","size":5,"soft_fail":false,"target_va":5402945776,"va":5369606141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2018337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e821cc1e00","size":5,"soft_fail":false,"target_va":5371624208,"va":5369605866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7748241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e8913a7600","size":5,"soft_fail":false,"target_va":5377353776,"va":5369605530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::as_str
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6742657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604864,"raw_hex":"e881e26600","size":5,"soft_fail":false,"target_va":5376348208,"va":5369605546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":5176,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":70,"record_offset":4384,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":70,"record_offset":4760,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":70,"record_offset":3328,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":70,"record_offset":4788,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":4992,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":70,"record_offset":4844,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":70,"record_offset":4876,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":70,"record_offset":4816,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":70,"record_offset":4676,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$31::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":70,"record_offset":4440,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":8311,"source_line":67,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":70,"record_offset":4620,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":8307,"source_line":61,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":70,"record_offset":4648,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":8308,"source_line":338,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":3356,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":4964,"parent_record_offset":3044,"ancestor_record_offset":3044,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":5224,"parent_record_offset":5200,"ancestor_record_offset":3044,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$346::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":70,"record_offset":4504,"parent_record_offset":4440,"ancestor_record_offset":3044,"inlinee":8315,"source_line":183,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$363::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":70,"record_offset":4532,"parent_record_offset":4440,"ancestor_record_offset":3044,"inlinee":8317,"source_line":206,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":70,"record_offset":4560,"parent_record_offset":4440,"ancestor_record_offset":3044,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":70,"record_offset":4408,"parent_record_offset":4384,"ancestor_record_offset":3044,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":70,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":3044,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":70,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":3044,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":70,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":3044,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":70,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":3044,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":70,"record_offset":3804,"parent_record_offset":3780,"ancestor_record_offset":3044,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":70,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":3044,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":5020,"parent_record_offset":4992,"ancestor_record_offset":3044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":70,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":3044,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":70,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":3044,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":70,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":3044,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":70,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":3044,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":70,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":3044,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":70,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":3044,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":3044,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":3044,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":4092,"parent_record_offset":3872,"ancestor_record_offset":3044,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":3436,"parent_record_offset":3404,"ancestor_record_offset":3044,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":3044,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":3980,"parent_record_offset":3872,"ancestor_record_offset":3044,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":3044,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":4328,"parent_record_offset":3512,"ancestor_record_offset":3044,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":3044,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":3488,"parent_record_offset":3464,"ancestor_record_offset":3044,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":3464,"parent_record_offset":3404,"ancestor_record_offset":3044,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":3044,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":3752,"parent_record_offset":3724,"ancestor_record_offset":3044,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":3044,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":3840,"parent_record_offset":3724,"ancestor_record_offset":3044,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":3724,"parent_record_offset":3512,"ancestor_record_offset":3044,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":3556,"parent_record_offset":3512,"ancestor_record_offset":3044,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":3872,"parent_record_offset":3512,"ancestor_record_offset":3044,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":3584,"parent_record_offset":3512,"ancestor_record_offset":3044,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":3668,"parent_record_offset":3512,"ancestor_record_offset":3044,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":4236,"parent_record_offset":3512,"ancestor_record_offset":3044,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":70,"record_offset":4476,"parent_record_offset":4440,"ancestor_record_offset":3044,"inlinee":8313,"source_line":38,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":70,"record_offset":4588,"parent_record_offset":4440,"ancestor_record_offset":3044,"inlinee":8321,"source_line":49,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$21::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ustr-1.0.0/src/lib.rs
//     evidence={"module":70,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3044,"inlinee":8306,"source_line":166,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":70,"record_offset":5432,"rva":897216,"va":5369606336}
// 编译实例 RVAs: 0xdb0c0, 0x5abac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ustr..Ustr$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6741828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e844df6600","size":5,"soft_fail":false,"target_va":5376348224,"va":5369606391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=2
//     target_aliases=1889 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-886695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e85978f2ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5369607362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e8df220000","size":5,"soft_fail":false,"target_va":5369615360,"va":5369606428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140264050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1608836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e8848c1800","size":5,"soft_fail":false,"target_va":5371215952,"va":5369607111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14026c110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1641811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e8530d1900","size":5,"soft_fail":false,"target_va":5371248912,"va":5369607096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403779e0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2736981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e855c32900","size":5,"soft_fail":false,"target_va":5372344800,"va":5369607814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140504d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-684108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374655168,"raw_hex":"e8b48ff5ff","size":5,"soft_fail":false,"target_va":5373971840,"va":5374655943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6632537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e859346500","size":5,"soft_fail":false,"target_va":5376239920,"va":5369607378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407480f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6737544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e888ce6600","size":5,"soft_fail":false,"target_va":5376344304,"va":5369606755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e93930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9338383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374655168,"raw_hex":"e80f7e8e00","size":5,"soft_fail":false,"target_va":5383993648,"va":5374655260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e9f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9385539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374655168,"raw_hex":"e843368f00","size":5,"soft_fail":false,"target_va":5384041472,"va":5374655928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f338e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9992753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374655168,"raw_hex":"e8317a9800","size":5,"soft_fail":false,"target_va":5384648928,"va":5374656170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209cd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33298176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e80017fc01","size":5,"soft_fail":false,"target_va":5402905856,"va":5369607675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28301824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374655168,"raw_hex":"e800daaf01","size":5,"soft_fail":false,"target_va":5402958336,"va":5374656507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28435211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374655168,"raw_hex":"e80be3b101","size":5,"soft_fail":false,"target_va":5403091664,"va":5374656448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7922275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e863e27800","size":5,"soft_fail":false,"target_va":5377529888,"va":5369607608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t33338155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e82bb3fc01","size":5,"soft_fail":false,"target_va":5402945776,"va":5369607616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2016865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e861c61e00","size":5,"soft_fail":false,"target_va":5371624208,"va":5369607338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7746769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e8d1347600","size":5,"soft_fail":false,"target_va":5377353776,"va":5369607002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::as_str
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6741185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369606336,"raw_hex":"e8c1dc6600","size":5,"soft_fail":false,"target_va":5376348208,"va":5369607018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":7556,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":70,"record_offset":6764,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":70,"record_offset":7140,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":70,"record_offset":5708,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":70,"record_offset":7168,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":7372,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":70,"record_offset":7224,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":70,"record_offset":7256,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":70,"record_offset":7196,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":70,"record_offset":7056,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$31::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":70,"record_offset":6820,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":8311,"source_line":67,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":70,"record_offset":7000,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":8307,"source_line":61,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":70,"record_offset":7028,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":8308,"source_line":338,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":5736,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":7344,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":5432,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$346::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":70,"record_offset":6884,"parent_record_offset":6820,"ancestor_record_offset":5432,"inlinee":8315,"source_line":183,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$363::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":70,"record_offset":6912,"parent_record_offset":6820,"ancestor_record_offset":5432,"inlinee":8317,"source_line":206,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":70,"record_offset":6940,"parent_record_offset":6820,"ancestor_record_offset":5432,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":70,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":5432,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":70,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":5432,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":70,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":5432,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":70,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":5432,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":70,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":5432,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":70,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":5432,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":70,"record_offset":7580,"parent_record_offset":7556,"ancestor_record_offset":5432,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":7400,"parent_record_offset":7372,"ancestor_record_offset":5432,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":70,"record_offset":6432,"parent_record_offset":6408,"ancestor_record_offset":5432,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":70,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":5432,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":70,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":5432,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":70,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":5432,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":70,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":5432,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":70,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":5432,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":5432,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":5432,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":6472,"parent_record_offset":6252,"ancestor_record_offset":5432,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":5816,"parent_record_offset":5784,"ancestor_record_offset":5432,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":5432,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":6360,"parent_record_offset":6252,"ancestor_record_offset":5432,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":5432,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":6708,"parent_record_offset":5892,"ancestor_record_offset":5432,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":5432,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":5868,"parent_record_offset":5844,"ancestor_record_offset":5432,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":5844,"parent_record_offset":5784,"ancestor_record_offset":5432,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":5432,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":5432,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":5432,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":6220,"parent_record_offset":6104,"ancestor_record_offset":5432,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":6104,"parent_record_offset":5892,"ancestor_record_offset":5432,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":5936,"parent_record_offset":5892,"ancestor_record_offset":5432,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":6252,"parent_record_offset":5892,"ancestor_record_offset":5432,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":5964,"parent_record_offset":5892,"ancestor_record_offset":5432,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":6048,"parent_record_offset":5892,"ancestor_record_offset":5432,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":6616,"parent_record_offset":5892,"ancestor_record_offset":5432,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":70,"record_offset":6856,"parent_record_offset":6820,"ancestor_record_offset":5432,"inlinee":8313,"source_line":38,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":70,"record_offset":6968,"parent_record_offset":6820,"ancestor_record_offset":5432,"inlinee":8321,"source_line":49,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$21::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ustr-1.0.0/src/lib.rs
//     evidence={"module":70,"record_offset":6544,"parent_record_offset":6520,"ancestor_record_offset":5432,"inlinee":8306,"source_line":166,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":70,"record_offset":232,"rva":894368,"va":5369603488}
// 编译实例 RVAs: 0xda5a0, 0x5aafa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ustr..Ustr$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6744681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e869ea6600","size":5,"soft_fail":false,"target_va":5376348224,"va":5369603538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=2
//     target_aliases=1889 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-883685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e81b84f2ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5369604352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e8042e0000","size":5,"soft_fail":false,"target_va":5369615360,"va":5369603575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140264530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1613164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e86c9d1800","size":5,"soft_fail":false,"target_va":5371217200,"va":5369604031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3145623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e897ff2f00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369604084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3145807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e84f003000","size":5,"soft_fail":false,"target_va":5372749744,"va":5369603932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405050d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-680180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374652320,"raw_hex":"e80c9ff5ff","size":5,"soft_fail":false,"target_va":5373972688,"va":5374652863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6635547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e81b406500","size":5,"soft_fail":false,"target_va":5376239920,"va":5369604368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072edf0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6637342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e81e476500","size":5,"soft_fail":false,"target_va":5376241136,"va":5369603789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6680207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e88fee6500","size":5,"soft_fail":false,"target_va":5376284224,"va":5369604012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e93930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9341236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374652320,"raw_hex":"e834898e00","size":5,"soft_fail":false,"target_va":5383993648,"va":5374652407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f338e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9995760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374652320,"raw_hex":"e8f0859800","size":5,"soft_fail":false,"target_va":5384648928,"va":5374653163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28438248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374652320,"raw_hex":"e8e8eeb101","size":5,"soft_fail":false,"target_va":5403091664,"va":5374653411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33374189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e8ed3ffd01","size":5,"soft_fail":false,"target_va":5402978144,"va":5369603950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7925312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e840ee7800","size":5,"soft_fail":false,"target_va":5377529888,"va":5369604571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7922501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e845e37800","size":5,"soft_fail":false,"target_va":5377527120,"va":5369604614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t33341192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e808bffc01","size":5,"soft_fail":false,"target_va":5402945776,"va":5369604579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2019872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e820d21e00","size":5,"soft_fail":false,"target_va":5371624208,"va":5369604331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::as_str
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6744316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369603488,"raw_hex":"e8fce86600","size":5,"soft_fail":false,"target_va":5376348208,"va":5369603887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":2532,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":70,"record_offset":1600,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":70,"record_offset":488,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":1920,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":70,"record_offset":1856,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":70,"record_offset":1892,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":70,"record_offset":1544,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8307,"source_line":61,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":70,"record_offset":1572,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8308,"source_line":338,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":516,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":2320,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":232,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":232,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":1720,"parent_record_offset":1684,"ancestor_record_offset":232,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":2024,"parent_record_offset":1968,"ancestor_record_offset":232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":232,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":1996,"parent_record_offset":1968,"ancestor_record_offset":232,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":1684,"parent_record_offset":1656,"ancestor_record_offset":232,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":1656,"parent_record_offset":1628,"ancestor_record_offset":232,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":1808,"parent_record_offset":1656,"ancestor_record_offset":232,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":232,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":70,"record_offset":1628,"parent_record_offset":1600,"ancestor_record_offset":232,"inlinee":6032,"source_line":50,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":70,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":232,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":70,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":232,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":70,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":232,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":70,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":70,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":232,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":70,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":232,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":70,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":232,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":70,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":232,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":70,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":232,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":70,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":232,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":232,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":232,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":1252,"parent_record_offset":1032,"ancestor_record_offset":232,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":596,"parent_record_offset":564,"ancestor_record_offset":232,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":232,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":1140,"parent_record_offset":1032,"ancestor_record_offset":232,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":232,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":1488,"parent_record_offset":672,"ancestor_record_offset":232,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":232,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":232,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":624,"parent_record_offset":564,"ancestor_record_offset":232,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":232,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":912,"parent_record_offset":884,"ancestor_record_offset":232,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":940,"parent_record_offset":912,"ancestor_record_offset":232,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":1000,"parent_record_offset":884,"ancestor_record_offset":232,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":884,"parent_record_offset":672,"ancestor_record_offset":232,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":716,"parent_record_offset":672,"ancestor_record_offset":232,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":1032,"parent_record_offset":672,"ancestor_record_offset":232,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":744,"parent_record_offset":672,"ancestor_record_offset":232,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":828,"parent_record_offset":672,"ancestor_record_offset":232,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":1396,"parent_record_offset":672,"ancestor_record_offset":232,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$21::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ustr-1.0.0/src/lib.rs
//     evidence={"module":70,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":232,"inlinee":8306,"source_line":166,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$14::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6007
// 首条位置: {"module":424,"inlinee":24583}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$3::from
// 记录形式: inline_instance, inlinee_declaration; instances=60
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D04
// 首条位置: {"module":41,"inlinee":7428}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::Parity::flip
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7801
// 首条位置: {"module":561,"inlinee":30721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$18::default
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15B3
// 首条位置: {"module":14,"inlinee":5555}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":8204,"parent_record_offset":7264,"ancestor_record_offset":7264,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":14,"record_offset":7304,"parent_record_offset":7264,"ancestor_record_offset":7264,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":7576,"parent_record_offset":7548,"ancestor_record_offset":7264,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":7264,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":7604,"parent_record_offset":7548,"ancestor_record_offset":7264,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":14,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":7264,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":102424,"parent_record_offset":102364,"ancestor_record_offset":101536,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":7264,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":7264,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":7676,"parent_record_offset":7652,"ancestor_record_offset":7264,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":14,"record_offset":8276,"parent_record_offset":8252,"ancestor_record_offset":7264,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":14,"record_offset":7736,"parent_record_offset":7304,"ancestor_record_offset":7264,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":7264,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":14,"record_offset":7428,"parent_record_offset":7400,"ancestor_record_offset":7264,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":7548,"parent_record_offset":7400,"ancestor_record_offset":7264,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":7264,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":14,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":7264,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":14,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":7264,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":14,"record_offset":7400,"parent_record_offset":7376,"ancestor_record_offset":7264,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":14,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":7264,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::TemporalRenderGraphState::get_and_flip_parity
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":248,"rva":7549712,"va":5376258832}
// 编译实例 RVAs: 0x733310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6643517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376258832,"raw_hex":"e8c3a09aff","size":5,"soft_fail":false,"target_va":5369615360,"va":5376258872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411865f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10826233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376258832,"raw_hex":"e8f931a500","size":5,"soft_fail":false,"target_va":5387085296,"va":5376259058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::temporal::Parity::flip
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":561,"record_offset":1460,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":30721,"source_line":73,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":561,"record_offset":412,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":561,"record_offset":1488,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":248,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":248,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":748,"parent_record_offset":724,"ancestor_record_offset":248,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":561,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":248,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":561,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":248,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":248,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":248,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":561,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":248,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":561,"record_offset":460,"parent_record_offset":436,"ancestor_record_offset":248,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":561,"record_offset":436,"parent_record_offset":412,"ancestor_record_offset":248,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":561,"record_offset":1056,"parent_record_offset":920,"ancestor_record_offset":248,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":561,"record_offset":488,"parent_record_offset":460,"ancestor_record_offset":248,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":248,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":248,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":248,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":1292,"parent_record_offset":568,"ancestor_record_offset":248,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":1328,"parent_record_offset":540,"ancestor_record_offset":248,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":248,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":516,"parent_record_offset":460,"ancestor_record_offset":248,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":568,"parent_record_offset":540,"ancestor_record_offset":248,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":561,"record_offset":804,"parent_record_offset":780,"ancestor_record_offset":248,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":561,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":248,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":561,"record_offset":888,"parent_record_offset":780,"ancestor_record_offset":248,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":561,"record_offset":780,"parent_record_offset":568,"ancestor_record_offset":248,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":612,"parent_record_offset":568,"ancestor_record_offset":248,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":561,"record_offset":920,"parent_record_offset":568,"ancestor_record_offset":248,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":561,"record_offset":640,"parent_record_offset":568,"ancestor_record_offset":248,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":561,"record_offset":724,"parent_record_offset":568,"ancestor_record_offset":248,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":561,"record_offset":1200,"parent_record_offset":568,"ancestor_record_offset":248,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$21::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ustr-1.0.0/src/lib.rs
//     evidence={"module":561,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":248,"inlinee":8306,"source_line":166,"archive":"_pdb/A/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::TemporalRenderGraphRecorder::new
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C0
// 首条位置: {"module":197,"inlinee":8384}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::Temporal::into_inner
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x601D
// 首条位置: {"module":441,"inlinee":24605}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::Temporal::as_ref
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BC
// 首条位置: {"module":197,"inlinee":8380}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::Temporal::split_pair
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BD
// 首条位置: {"module":197,"inlinee":8381}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ADE
// 首条位置: {"module":134,"inlinee":10974}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":13528,"parent_record_offset":13508,"ancestor_record_offset":13508,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":13576,"parent_record_offset":13552,"ancestor_record_offset":13508,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":134,"record_offset":13552,"parent_record_offset":13528,"ancestor_record_offset":13508,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 299
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AE2
// 首条位置: {"module":134,"inlinee":10978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":15352,"parent_record_offset":15332,"ancestor_record_offset":15332,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":15400,"parent_record_offset":15376,"ancestor_record_offset":15332,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":134,"record_offset":15376,"parent_record_offset":15352,"ancestor_record_offset":15332,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 320
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":70,"record_offset":10788,"rva":899552,"va":5369608672}
// 编译实例 RVAs: 0xdb9e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140367730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2669410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369608672,"raw_hex":"e862bb2800","size":5,"soft_fail":false,"target_va":5372278576,"va":5369609161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140385b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2793403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369608672,"raw_hex":"e8bb9f2a00","size":5,"soft_fail":false,"target_va":5372402560,"va":5369609152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072ef10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6632412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369608672,"raw_hex":"e8dc336500","size":5,"soft_fail":false,"target_va":5376241424,"va":5369609007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140748250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6735801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369608672,"raw_hex":"e8b9c76600","size":5,"soft_fail":false,"target_va":5376344656,"va":5369608850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":12156,"parent_record_offset":10788,"ancestor_record_offset":10788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":70,"record_offset":12068,"parent_record_offset":10788,"ancestor_record_offset":10788,"inlinee":8356,"source_line":69,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":70,"record_offset":12036,"parent_record_offset":10788,"ancestor_record_offset":10788,"inlinee":8308,"source_line":338,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":11024,"parent_record_offset":10788,"ancestor_record_offset":10788,"inlinee":6439,"source_line":456,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$56::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":11396,"parent_record_offset":10788,"ancestor_record_offset":10788,"inlinee":8349,"source_line":2427,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":70,"record_offset":11276,"parent_record_offset":11248,"ancestor_record_offset":10788,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":70,"record_offset":11336,"parent_record_offset":11304,"ancestor_record_offset":10788,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":70,"record_offset":11648,"parent_record_offset":11624,"ancestor_record_offset":10788,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":70,"record_offset":11220,"parent_record_offset":11196,"ancestor_record_offset":10788,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":10788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":70,"record_offset":11868,"parent_record_offset":11844,"ancestor_record_offset":10788,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":70,"record_offset":11844,"parent_record_offset":11820,"ancestor_record_offset":10788,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":11080,"parent_record_offset":11052,"ancestor_record_offset":10788,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":70,"record_offset":11452,"parent_record_offset":11424,"ancestor_record_offset":10788,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":11820,"parent_record_offset":11512,"ancestor_record_offset":10788,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":11196,"parent_record_offset":11136,"ancestor_record_offset":10788,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":11512,"parent_record_offset":11480,"ancestor_record_offset":10788,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":11108,"parent_record_offset":11080,"ancestor_record_offset":10788,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":11168,"parent_record_offset":11136,"ancestor_record_offset":10788,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":11136,"parent_record_offset":11108,"ancestor_record_offset":10788,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":11592,"parent_record_offset":11560,"ancestor_record_offset":10788,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":11624,"parent_record_offset":11592,"ancestor_record_offset":10788,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":70,"record_offset":11560,"parent_record_offset":11512,"ancestor_record_offset":10788,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":70,"record_offset":11480,"parent_record_offset":11452,"ancestor_record_offset":10788,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":11248,"parent_record_offset":11196,"ancestor_record_offset":10788,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":70,"record_offset":11304,"parent_record_offset":11248,"ancestor_record_offset":10788,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":11052,"parent_record_offset":11024,"ancestor_record_offset":10788,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":70,"record_offset":11424,"parent_record_offset":11396,"ancestor_record_offset":10788,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 369
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":70,"record_offset":7812,"rva":898704,"va":5369607824}
// 编译实例 RVAs: 0xdb690, 0x5ac090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ustr..Ustr$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6740356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607824,"raw_hex":"e884d96600","size":5,"soft_fail":false,"target_va":5376348224,"va":5369607863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7921923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607824,"raw_hex":"e803e17800","size":5,"soft_fail":false,"target_va":5377529888,"va":5369607960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::temporal::TemporalRenderGraphState::get_and_flip_parity
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6650586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607824,"raw_hex":"e8da7a6500","size":5,"soft_fail":false,"target_va":5376258832,"va":5369608241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607824,"raw_hex":"e8bbf3ffff","size":5,"soft_fail":false,"target_va":5369604864,"va":5369608000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":8076,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":70,"record_offset":8048,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":70,"record_offset":8156,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":70,"record_offset":8304,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":7812,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":70,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7812,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00070.jsonl.gz"}
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
// 完整结构及原始类型记录: temporal.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::Parity
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ustr::Ustr,rhapsody::rg::temporal::Parity>", "constructor": "tuple$", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::Parity"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::Parity,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::Parity", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<ustr::Ustr,rhapsody::rg::temporal::Parity>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::Parity,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<ustr::Ustr,rhapsody::rg::temporal::Parity>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::Parity,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<ustr::Ustr,rhapsody::rg::temporal::Parity,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::Parity", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::ReadOnlyHandle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture>", "constructor": "rhapsody::rg::temporal::ReadOnlyHandle", "arguments": ["rhapsody::raw::texture::Texture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> >", "constructor": "ref$", "arguments": ["rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass", "arguments": ["ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >", "constructor": "country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder", "arguments": ["ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::Temporal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::TemporalRenderGraphRecorder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::temporal::TemporalRenderGraphRecorder", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::TemporalRenderGraphState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphState>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::temporal::TemporalRenderGraphState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState>", "constructor": "core::option::Option", "arguments": ["rhapsody::rg::temporal::TemporalRenderGraphState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState> >", "constructor": "enum2$", "arguments": ["core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::TemporalResourceValue
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>", "constructor": "tuple$", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::TemporalResourceValue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::TemporalResourceValue", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::TemporalResourceValue", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalResourceValue>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::temporal::TemporalResourceValue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: temporal.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: temporal.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::Parity::flip
// 信息 ID: 40d338854e2367393c4cf349
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::temporal::TemporalRenderGraphState::get_and_flip_parity; module=561, inline_record=1460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::Temporal::as_ref
// 信息 ID: cb5cdc231f94a461db120d43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=22612; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=22640; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=22768; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=22796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::Temporal::into_inner
// 信息 ID: f662132046ff6635f3ecb7df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi; module=441, inline_record=11628; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi; module=441, inline_record=11656; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi; module=441, inline_record=11684; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=449, inline_record=21236; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=449, inline_record=21264; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::vermeer_pass; module=462, inline_record=40608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::Temporal::split_pair
// 信息 ID: 0950d0379cedf57c509b4ecf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=22584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: 25ef82a2746996f680349960
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x300047 -> 0xdb9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x3037ac -> 0xdb9e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::TemporalRenderGraphRecorder::new
// 信息 ID: 4aedd8f194a633acf84baebe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=4148; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=50756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::TemporalRenderGraphState::get_and_flip_parity
// 信息 ID: d6d02f56c6a573a8983a1150
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0xdb831 -> 0x733310 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x5ac231 -> 0x733310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure$0
// 信息 ID: cbaf426717e92e4f09f3d42c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::temporal::impl$11::get_or_create_temporal", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0 <- anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0 <- anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> >; module=134, inline_record=13508; 不是独立调用边。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> >; module=387, inline_record=2784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>
// 信息 ID: 12e553f932e9b5f0011c24cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::temporal::impl$11::get_or_create_temporal -> ["country_core::render::pass_builder::pass_resources::PassResources", "alloc::string::String"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0xdb740 -> 0xdab00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0xdb7da -> 0xdab00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x5ac140 -> 0x5ab500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x5ac1da -> 0x5ab500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 信息 ID: b44374a77dce12b807b807e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::temporal::impl$11::get_or_create_temporal -> ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe146 -> 0xdb0c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring @ 0x578c54 -> 0x5abac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring @ 0x578cf8 -> 0x5abac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x5ee5d7 -> 0x5abac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x5ee65f -> 0x5abac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_pass @ 0x63ccf0 -> 0x5abac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure$0
// 信息 ID: ee948eb1c73250f8c78148a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::temporal::impl$12::get_or_create_temporal", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0 <- anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0 <- anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; module=134, inline_record=15332; 不是独立调用边。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; module=387, inline_record=4176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 信息 ID: 8b438db0a133db99cb515f1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::temporal::impl$12::get_or_create_temporal -> ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0xdc586 -> 0xda5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset> @ 0x5ada24 -> 0x5aafa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x5eed78 -> 0x5aafa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x63d8dd -> 0x5aafa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb @ 0x656202 -> 0x5aafa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass::grass_compute @ 0x691a71 -> 0x5aafa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$14::fmt
// 信息 ID: 70a3fa178d654548fad7ac36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >; module=424, inline_record=8776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$18::default
// 信息 ID: 327e8741e0d838e34049333d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> >; module=14, inline_record=7264; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=101536; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=57016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 信息 ID: 2a0654cb7bea27f324d6f673
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair -> ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe865 -> 0xdb690 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows @ 0x5529ae -> 0x5ac090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0x5eaea6 -> 0x5ac090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0x5eaf8e -> 0x5ac090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0x5eb070 -> 0x5ac090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60a025 -> 0x5ac090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60b08d -> 0x5ac090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60b176 -> 0x5ac090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi @ 0x63b18c -> 0x5ac090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance @ 0x63fd4f -> 0x5ac090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance @ 0x63fe2a -> 0x5ac090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$3::from
// 信息 ID: e1da2ec26af0e46ede490d66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 58 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo>; module=41, inline_record=5892; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo>; module=41, inline_record=6504; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo>; module=41, inline_record=7116; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>; module=41, inline_record=7732; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::walls::dormers::DormerHoleCountPerCellSsbo>; module=1320, inline_record=16616; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::rt::RtTriangleMetaSsbo>; module=1320, inline_record=17212; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::render::lantern::LanternSsbo>; module=1320, inline_record=17812; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>; module=1320, inline_record=18424; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
