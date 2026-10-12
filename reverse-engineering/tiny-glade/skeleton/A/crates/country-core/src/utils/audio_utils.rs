// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/utils/audio_utils.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 890687e438028b92bf28c85e24c16b5a3abe9bb4c37bf918dea305c09d576d03
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: audio_utils.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 392, 395, 409, 418, 420, 425, 432, 452, 459, 472, 495, 496, 499, 1207, 1216, 1321, 1325, 1368, 1374, 1378
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::start_if_not_playing
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 0, 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x653F
// 首条位置: {"module":452,"inlinee":25919}
// 编译实例 RVAs: 0xf29190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405af4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9936413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384606096,"raw_hex":"e8e36168ff","size":5,"soft_fail":false,"target_va":5374669984,"va":5384606392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b2850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9922980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384606096,"raw_hex":"e85c9668ff","size":5,"soft_fail":false,"target_va":5374683216,"va":5384606191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18373068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384606096,"raw_hex":"e8cc591801","size":5,"soft_fail":false,"target_va":5402979344,"va":5384606271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18373762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384606096,"raw_hex":"e8825c1801","size":5,"soft_fail":false,"target_va":5402980128,"va":5384606361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::start
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2003948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384606096,"raw_hex":"e8146ce1ff","size":5,"soft_fail":false,"target_va":5382602288,"va":5384606231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":452,"record_offset":15432,"parent_record_offset":15364,"ancestor_record_offset":15364,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":452,"record_offset":15636,"parent_record_offset":15364,"ancestor_record_offset":15364,"inlinee":21884,"source_line":387,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":452,"record_offset":15680,"parent_record_offset":15636,"ancestor_record_offset":15364,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":452,"record_offset":15712,"parent_record_offset":15636,"ancestor_record_offset":15364,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":452,"record_offset":15460,"parent_record_offset":15432,"ancestor_record_offset":15364,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":452,"record_offset":15488,"parent_record_offset":15460,"ancestor_record_offset":15364,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::stop
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1368,"record_offset":119264,"rva":15896544,"va":5384605664}
// 编译实例 RVAs: 0xf28fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405af4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9936093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384605664,"raw_hex":"e8236368ff","size":5,"soft_fail":false,"target_va":5374669984,"va":5384606072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b2850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9922611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384605664,"raw_hex":"e8cd9768ff","size":5,"soft_fail":false,"target_va":5374683216,"va":5384605822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18373390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384605664,"raw_hex":"e80e5b1801","size":5,"soft_fail":false,"target_va":5402979344,"va":5384605949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18374084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384605664,"raw_hex":"e8c45d1801","size":5,"soft_fail":false,"target_va":5402980128,"va":5384606039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2003232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384605664,"raw_hex":"e8e06ee1ff","size":5,"soft_fail":false,"target_va":5382602640,"va":5384605867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7161981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384605664,"raw_hex":"e883b792ff","size":5,"soft_fail":false,"target_va":5377443760,"va":5384605736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7162765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384605664,"raw_hex":"e873b492ff","size":5,"soft_fail":false,"target_va":5377442944,"va":5384605704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1368,"record_offset":119436,"parent_record_offset":119264,"ancestor_record_offset":119264,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01368.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_millis
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":1368,"record_offset":119408,"parent_record_offset":119264,"ancestor_record_offset":119264,"inlinee":15667,"source_line":554,"archive":"_pdb/A/modules/01368.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":1368,"record_offset":119760,"parent_record_offset":119264,"ancestor_record_offset":119264,"inlinee":24665,"source_line":414,"archive":"_pdb/A/modules/01368.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1368,"record_offset":119796,"parent_record_offset":119760,"ancestor_record_offset":119264,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01368.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1368,"record_offset":119828,"parent_record_offset":119760,"ancestor_record_offset":119264,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01368.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1368,"record_offset":119472,"parent_record_offset":119436,"ancestor_record_offset":119264,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01368.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1368,"record_offset":119508,"parent_record_offset":119472,"ancestor_record_offset":119264,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01368.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoopDesc::new
// 记录形式: inline_instance, inlinee_declaration; instances=72
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5741
// 首条位置: {"module":392,"inlinee":22337}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::audio_resources::AudioCore::instantiate_loop
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5E32
// 首条位置: {"module":418,"inlinee":24114}
// 编译实例 RVAs: 0x5b3660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405af4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374686816,"raw_hex":"e84dbdffff","size":5,"soft_fail":false,"target_va":5374669984,"va":5374687054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1ae0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1303549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374686816,"raw_hex":"e8fde31300","size":5,"soft_fail":false,"target_va":5375990496,"va":5374686942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7de40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8169341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374686816,"raw_hex":"e87da77c00","size":5,"soft_fail":false,"target_va":5382856256,"va":5374686910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2677156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374686816,"raw_hex":"e8a4d92800","size":5,"soft_fail":false,"target_va":5377364336,"va":5374687175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28292211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374686816,"raw_hex":"e873b4af01","size":5,"soft_fail":false,"target_va":5402979344,"va":5374687128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::get_parameter_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7916782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374686816,"raw_hex":"e8eecc7800","size":5,"soft_fail":false,"target_va":5382603824,"va":5374687037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":418,"record_offset":15376,"parent_record_offset":13540,"ancestor_record_offset":13540,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":418,"record_offset":13596,"parent_record_offset":13540,"ancestor_record_offset":13540,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":418,"record_offset":15348,"parent_record_offset":13540,"ancestor_record_offset":13540,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":425,"record_offset":38064,"parent_record_offset":37556,"ancestor_record_offset":37556,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00425.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::impl$10::has_parameter
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":418,"record_offset":15472,"parent_record_offset":13540,"ancestor_record_offset":13540,"inlinee":24117,"source_line":493,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":418,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":13540,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":418,"record_offset":14396,"parent_record_offset":14340,"ancestor_record_offset":13540,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":418,"record_offset":14368,"parent_record_offset":14340,"ancestor_record_offset":13540,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":418,"record_offset":14340,"parent_record_offset":14312,"ancestor_record_offset":13540,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":418,"record_offset":13776,"parent_record_offset":13752,"ancestor_record_offset":13540,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":418,"record_offset":13908,"parent_record_offset":13868,"ancestor_record_offset":13540,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::try_process
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\mod.rs
//     evidence={"module":418,"record_offset":13676,"parent_record_offset":13636,"ancestor_record_offset":13540,"inlinee":22001,"source_line":162,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":418,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":13540,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":418,"record_offset":13752,"parent_record_offset":13728,"ancestor_record_offset":13540,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":418,"record_offset":14100,"parent_record_offset":14072,"ancestor_record_offset":13540,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":418,"record_offset":14996,"parent_record_offset":14968,"ancestor_record_offset":13540,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":418,"record_offset":15504,"parent_record_offset":15472,"ancestor_record_offset":13540,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":418,"record_offset":14072,"parent_record_offset":14024,"ancestor_record_offset":13540,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":418,"record_offset":13868,"parent_record_offset":13676,"ancestor_record_offset":13540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":418,"record_offset":14128,"parent_record_offset":14100,"ancestor_record_offset":13540,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":418,"record_offset":14156,"parent_record_offset":14100,"ancestor_record_offset":13540,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":418,"record_offset":15536,"parent_record_offset":15472,"ancestor_record_offset":13540,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":418,"record_offset":14220,"parent_record_offset":14024,"ancestor_record_offset":13540,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":418,"record_offset":13636,"parent_record_offset":13596,"ancestor_record_offset":13540,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":418,"record_offset":13728,"parent_record_offset":13676,"ancestor_record_offset":13540,"inlinee":21966,"source_line":1935,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::from_output
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":418,"record_offset":13840,"parent_record_offset":13676,"ancestor_record_offset":13540,"inlinee":21968,"source_line":1945,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":418,"record_offset":15404,"parent_record_offset":15376,"ancestor_record_offset":13540,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":418,"record_offset":15432,"parent_record_offset":15404,"ancestor_record_offset":13540,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::impl$13::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":418,"record_offset":14024,"parent_record_offset":13988,"ancestor_record_offset":13540,"inlinee":24119,"source_line":540,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::impl$1::instantiate_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF180
// 首条位置: {"module":1321,"inlinee":61824}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":1321,"record_offset":101824,"parent_record_offset":101780,"ancestor_record_offset":101780,"inlinee":21881,"source_line":301,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":102212,"parent_record_offset":102188,"ancestor_record_offset":101780,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":102236,"parent_record_offset":102212,"ancestor_record_offset":101780,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":102188,"parent_record_offset":102148,"ancestor_record_offset":101780,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":102148,"parent_record_offset":102116,"ancestor_record_offset":101780,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":102116,"parent_record_offset":102084,"ancestor_record_offset":101780,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":102308,"parent_record_offset":102116,"ancestor_record_offset":101780,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1321,"record_offset":102044,"parent_record_offset":102012,"ancestor_record_offset":101780,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1321,"record_offset":102012,"parent_record_offset":101980,"ancestor_record_offset":101780,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1321,"record_offset":101948,"parent_record_offset":101916,"ancestor_record_offset":101780,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1321,"record_offset":101980,"parent_record_offset":101948,"ancestor_record_offset":101780,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1321,"record_offset":101916,"parent_record_offset":101884,"ancestor_record_offset":101780,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1321,"record_offset":101884,"parent_record_offset":101824,"ancestor_record_offset":101780,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":102084,"parent_record_offset":102044,"ancestor_record_offset":101780,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":1321,"record_offset":102428,"parent_record_offset":101824,"ancestor_record_offset":101780,"inlinee":21883,"source_line":647,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":102372,"parent_record_offset":102348,"ancestor_record_offset":101780,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":102348,"parent_record_offset":102044,"ancestor_record_offset":101780,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":102264,"parent_record_offset":102236,"ancestor_record_offset":101780,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::current
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=84
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x586E
// 首条位置: {"module":395,"inlinee":22638}
// 编译实例 RVAs: 0xf28fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18373695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384605600,"raw_hex":"e83f5c1801","size":5,"soft_fail":false,"target_va":5402979344,"va":5384605644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=74
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":62948,"parent_record_offset":62920,"ancestor_record_offset":62920,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=74
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":395,"record_offset":62980,"parent_record_offset":62948,"ancestor_record_offset":62920,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=74
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":395,"record_offset":63012,"parent_record_offset":62980,"ancestor_record_offset":62920,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::is_playing
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EE0
// 首条位置: {"module":420,"inlinee":24288}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":420,"record_offset":32132,"parent_record_offset":32100,"ancestor_record_offset":32100,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::is_playing
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":420,"record_offset":32240,"parent_record_offset":32100,"ancestor_record_offset":32100,"inlinee":24289,"source_line":473,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":420,"record_offset":32280,"parent_record_offset":32240,"ancestor_record_offset":32100,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":420,"record_offset":32312,"parent_record_offset":32240,"ancestor_record_offset":32100,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":420,"record_offset":32164,"parent_record_offset":32132,"ancestor_record_offset":32100,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":420,"record_offset":32196,"parent_record_offset":32164,"ancestor_record_offset":32100,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00420.jsonl.gz"}
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
// 完整结构及原始类型记录: audio_utils.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::audio_utils::AudioLoop
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "array$<country_core::utils::audio_utils::AudioLoop,3>", "constructor": "array$", "arguments": ["country_core::utils::audio_utils::AudioLoop", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<array$<country_core::utils::audio_utils::AudioLoop,3> >", "constructor": "core::ptr::drop_in_place", "arguments": ["array$<country_core::utils::audio_utils::AudioLoop,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::utils::audio_utils::AudioLoop", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<usize,country_core::utils::audio_utils::AudioLoop>", "constructor": "tuple$", "arguments": ["usize", "country_core::utils::audio_utils::AudioLoop"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<usize,country_core::utils::audio_utils::AudioLoop>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<usize,country_core::utils::audio_utils::AudioLoop>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<usize,country_core::utils::audio_utils::AudioLoop,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["usize", "country_core::utils::audio_utils::AudioLoop", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<usize,country_core::utils::audio_utils::AudioLoop>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<usize,country_core::utils::audio_utils::AudioLoop,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<usize,country_core::utils::audio_utils::AudioLoop>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<usize,country_core::utils::audio_utils::AudioLoop,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<usize,country_core::utils::audio_utils::AudioLoop,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["usize", "country_core::utils::audio_utils::AudioLoop", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::utils::audio_utils::AudioLoop>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::utils::audio_utils::AudioLoop"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::audio_utils::AudioLoopDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: audio_utils.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: audio_utils.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 208; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::audio_resources::AudioCore::instantiate_loop
// 信息 ID: 089c05cf8455e3f91c38594a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 91ceb1718f806fece44659eb: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 57 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::garden_audio::garden_audio_init_system @ 0x51aa80 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::garden_audio::garden_audio_init_system @ 0x51aafe -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::garden_audio::garden_audio_init_system @ 0x51ab72 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::garden_audio::garden_audio_init_system @ 0x51abe0 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567855 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x5678e8 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567971 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x5679fa -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567a80 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567b02 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567b84 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567c7d -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567cfa -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567d77 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567df4 -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x567e7d -> 0x5b3660 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::water_ambience_audio::water_ambience_audio_init_system; module=418, inline_record=13540; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::current
// 信息 ID: aca4b96382a518cefaf5db19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 73 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x1b3323a -> 0xf28fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x1b33263 -> 0xf28fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x1b33284 -> 0xf28fa0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play; module=395, inline_record=62920; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_system; module=409, inline_record=17164; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_system; module=409, inline_record=18040; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_system; module=409, inline_record=18168; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_system; module=409, inline_record=18768; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_system; module=409, inline_record=18896; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_system; module=409, inline_record=19496; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_system; module=409, inline_record=19624; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::is_playing
// 信息 ID: 1cf14a79d3fa1f9abc19f015
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::stairs_audio::stairs_audio_system; module=420, inline_record=32100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::start_if_not_playing
// 信息 ID: 9f1d2445660b3fd3406bb4cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 40 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56adf7 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56ae77 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56aea0 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56aec9 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56aef2 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56af1b -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56af44 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56b009 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56ae59 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56ae2d -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_ambience_audio::water_ambience_audio_system @ 0x595a69 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0x59dd6b -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0x59ddec -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0x59dc5e -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::stairs_audio::stairs_audio_system @ 0x59e6b0 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::timberframe_audio::timberframe_audio @ 0x59f0e8 -> 0xf29190 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=452, inline_record=15364; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=452, inline_record=16796; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::stop
// 信息 ID: b6e6823c25fd215ae92f0ada
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56ae22 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56ae4e -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56af93 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56af4f -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56af26 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56aefd -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56aed4 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56aeab -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56ae82 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0x56ae01 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_ambience_audio::water_ambience_audio_system @ 0x595980 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0x59dc30 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0x59de40 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0x59ddb9 -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::stairs_audio::stairs_audio_system @ 0x59e49a -> 0xf28fe0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::timberframe_audio::timberframe_audio @ 0x59f19d -> 0xf28fe0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoopDesc::new
// 信息 ID: 69c6b981cb15b2f5c961d0a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 91ceb1718f806fece44659eb: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 57 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::garden_audio::garden_audio_init_system; module=392, inline_record=5532; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::garden_audio::garden_audio_init_system; module=392, inline_record=5632; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::garden_audio::garden_audio_init_system; module=392, inline_record=5732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::garden_audio::garden_audio_init_system; module=392, inline_record=5832; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_init_system; module=409, inline_record=10580; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_init_system; module=409, inline_record=10680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_init_system; module=409, inline_record=10780; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_init_system; module=409, inline_record=10880; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::impl$1::instantiate_loop::closure$0
// 信息 ID: 2da17c37fee9ca7ccbe31e47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8021970ebabb368c6d099be2: enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::audio_utils::impl$1::instantiate_loop", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::audio_resources::AudioInstance,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<country_core::resources::audio_resources::AudioInstance,anyhow::Error> >,core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::resources::audio_resources::AudioInstance,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<country_core::resources::audio_resources::AudioInstance,anyhow::Error> >,core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::resources::audio_resources::AudioInstance,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<country_core::resources::audio_resources::AudioInstance,anyhow::Error> >,core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::resources::audio_resources::AudioInstance,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,tuple$<> > > >; module=1321, inline_record=101780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
