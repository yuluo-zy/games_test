// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/audio/audio_update.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d26cbd2fc235f15fe74c13344d25986a21cc1ad51134b4acfdde9355b5c71af0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: audio_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 496
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::audio_update::audio_update_system
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":496,"record_offset":5244,"rva":7099296,"va":5375808416}
// 编译实例 RVAs: 0x6c53a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1132471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e849b8eeff","size":5,"soft_fail":false,"target_va":5374676720,"va":5375809186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7499249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e8f16d7200","size":5,"soft_fail":false,"target_va":5383308144,"va":5375808890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27169173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e895919e01","size":5,"soft_fail":false,"target_va":5402978688,"va":5375809510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e8e2f3ffff","size":5,"soft_fail":false,"target_va":5375806528,"va":5375809625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27171184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e870999e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375808939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e80af9ffff","size":5,"soft_fail":false,"target_va":5375806720,"va":5375808497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27172084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e8f49c9e01","size":5,"soft_fail":false,"target_va":5402980656,"va":5375808567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1136027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e865aaeeff","size":5,"soft_fail":false,"target_va":5374673440,"va":5375809462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t547799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e8d75b0800","size":5,"soft_fail":false,"target_va":5376356912,"va":5375809108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t551685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e8056b0800","size":5,"soft_fail":false,"target_va":5376360560,"va":5375808870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t547672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375808416,"raw_hex":"e8585b0800","size":5,"soft_fail":false,"target_va":5376356144,"va":5375808467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-2975","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375808416,"raw_hex":"e961f4ffff","size":5,"soft_fail":false,"target_va":5375806528,"va":5375809498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":8180,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":24661,"source_line":331,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":6012,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":24658,"source_line":313,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":8296,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":24659,"source_line":319,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":8412,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":24660,"source_line":325,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::update
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":8528,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":24657,"source_line":288,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":496,"record_offset":8624,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":23594,"source_line":1098,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_vector3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":496,"record_offset":8980,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":27860,"source_line":1119,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":7148,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":5428,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":6136,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":496,"record_offset":6076,"parent_record_offset":6012,"ancestor_record_offset":5244,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":5244,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":5244,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":5244,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":7472,"parent_record_offset":7444,"ancestor_record_offset":5244,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":7504,"parent_record_offset":7472,"ancestor_record_offset":5244,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":7676,"parent_record_offset":7504,"ancestor_record_offset":5244,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":7788,"parent_record_offset":7764,"ancestor_record_offset":5244,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":496,"record_offset":8772,"parent_record_offset":8716,"ancestor_record_offset":5244,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":496,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":5244,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":496,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":5244,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":496,"record_offset":5648,"parent_record_offset":5552,"ancestor_record_offset":5244,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":496,"record_offset":6044,"parent_record_offset":6012,"ancestor_record_offset":5244,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":496,"record_offset":6732,"parent_record_offset":6272,"ancestor_record_offset":5244,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":496,"record_offset":7416,"parent_record_offset":7244,"ancestor_record_offset":5244,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":496,"record_offset":5960,"parent_record_offset":5932,"ancestor_record_offset":5244,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":496,"record_offset":7740,"parent_record_offset":7444,"ancestor_record_offset":5244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":496,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":5244,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":496,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":5244,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":496,"record_offset":6692,"parent_record_offset":6660,"ancestor_record_offset":5244,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":496,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":5244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":496,"record_offset":8140,"parent_record_offset":7212,"ancestor_record_offset":5244,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":496,"record_offset":5584,"parent_record_offset":5552,"ancestor_record_offset":5244,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":496,"record_offset":5612,"parent_record_offset":5584,"ancestor_record_offset":5244,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":496,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":5244,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":496,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":5244,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":496,"record_offset":6436,"parent_record_offset":6328,"ancestor_record_offset":5244,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":496,"record_offset":5552,"parent_record_offset":5524,"ancestor_record_offset":5244,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":496,"record_offset":6168,"parent_record_offset":6136,"ancestor_record_offset":5244,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":496,"record_offset":6300,"parent_record_offset":6272,"ancestor_record_offset":5244,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":496,"record_offset":6272,"parent_record_offset":6240,"ancestor_record_offset":5244,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":496,"record_offset":6216,"parent_record_offset":6192,"ancestor_record_offset":5244,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":496,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":5244,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":496,"record_offset":6192,"parent_record_offset":6168,"ancestor_record_offset":5244,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":496,"record_offset":6764,"parent_record_offset":6240,"ancestor_record_offset":5244,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":5748,"parent_record_offset":5716,"ancestor_record_offset":5244,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":5908,"parent_record_offset":5748,"ancestor_record_offset":5244,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":5244,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":5464,"parent_record_offset":5428,"ancestor_record_offset":5244,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":5244,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":5524,"parent_record_offset":5492,"ancestor_record_offset":5244,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":5492,"parent_record_offset":5464,"ancestor_record_offset":5244,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":496,"record_offset":6328,"parent_record_offset":6300,"ancestor_record_offset":5244,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":496,"record_offset":6576,"parent_record_offset":6328,"ancestor_record_offset":5244,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":496,"record_offset":6548,"parent_record_offset":6328,"ancestor_record_offset":5244,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":496,"record_offset":6380,"parent_record_offset":6328,"ancestor_record_offset":5244,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":8264,"parent_record_offset":8180,"ancestor_record_offset":5244,"inlinee":24644,"source_line":234,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":6104,"parent_record_offset":6012,"ancestor_record_offset":5244,"inlinee":24641,"source_line":222,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":8380,"parent_record_offset":8296,"ancestor_record_offset":5244,"inlinee":24642,"source_line":226,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":8496,"parent_record_offset":8412,"ancestor_record_offset":5244,"inlinee":24643,"source_line":230,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":496,"record_offset":8660,"parent_record_offset":8624,"ancestor_record_offset":5244,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$13::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":496,"record_offset":8892,"parent_record_offset":8624,"ancestor_record_offset":5244,"inlinee":15701,"source_line":998,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":496,"record_offset":8948,"parent_record_offset":8624,"ancestor_record_offset":5244,"inlinee":21868,"source_line":223,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":7180,"parent_record_offset":7148,"ancestor_record_offset":5244,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":496,"record_offset":7280,"parent_record_offset":7244,"ancestor_record_offset":5244,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":7444,"parent_record_offset":7244,"ancestor_record_offset":5244,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":7712,"parent_record_offset":7444,"ancestor_record_offset":5244,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":496,"record_offset":7308,"parent_record_offset":7280,"ancestor_record_offset":5244,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":496,"record_offset":7340,"parent_record_offset":7308,"ancestor_record_offset":5244,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":496,"record_offset":7244,"parent_record_offset":7212,"ancestor_record_offset":5244,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":496,"record_offset":7212,"parent_record_offset":7180,"ancestor_record_offset":5244,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::audio_update::boot_audio_update_system
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":496,"record_offset":9360,"rva":7100544,"va":5375809664}
// 编译实例 RVAs: 0x6c5880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1133683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e88db3eeff","size":5,"soft_fail":false,"target_va":5374676720,"va":5375810398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7498022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e826697200","size":5,"soft_fail":false,"target_va":5383308144,"va":5375810117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27168048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e8308d9e01","size":5,"soft_fail":false,"target_va":5402978688,"va":5375810635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e889efffff","size":5,"soft_fail":false,"target_va":5375806528,"va":5375810738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27169957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e8a5949e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375810166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e837f4ffff","size":5,"soft_fail":false,"target_va":5375806720,"va":5375809732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27170849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e821989e01","size":5,"soft_fail":false,"target_va":5402980656,"va":5375809802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1137158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e8faa5eeff","size":5,"soft_fail":false,"target_va":5374673440,"va":5375810593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t546577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e811570800","size":5,"soft_fail":false,"target_va":5376356912,"va":5375810330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t550458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e83a660800","size":5,"soft_fail":false,"target_va":5376360560,"va":5375810097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t546436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375809664,"raw_hex":"e884560800","size":5,"soft_fail":false,"target_va":5376356144,"va":5375809703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-4100","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375809664,"raw_hex":"e9fcefffff","size":5,"soft_fail":false,"target_va":5375806528,"va":5375810623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":12292,"parent_record_offset":9360,"ancestor_record_offset":9360,"inlinee":24661,"source_line":331,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":10124,"parent_record_offset":9360,"ancestor_record_offset":9360,"inlinee":24658,"source_line":313,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":12408,"parent_record_offset":9360,"ancestor_record_offset":9360,"inlinee":24659,"source_line":319,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":12524,"parent_record_offset":9360,"ancestor_record_offset":9360,"inlinee":24660,"source_line":325,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::update
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":12640,"parent_record_offset":9360,"ancestor_record_offset":9360,"inlinee":24657,"source_line":288,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":11260,"parent_record_offset":9360,"ancestor_record_offset":9360,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":9540,"parent_record_offset":9360,"ancestor_record_offset":9360,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":10248,"parent_record_offset":9360,"ancestor_record_offset":9360,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":496,"record_offset":10188,"parent_record_offset":10124,"ancestor_record_offset":9360,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":11644,"parent_record_offset":11616,"ancestor_record_offset":9360,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":11696,"parent_record_offset":11672,"ancestor_record_offset":9360,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":11672,"parent_record_offset":11644,"ancestor_record_offset":9360,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":11584,"parent_record_offset":11556,"ancestor_record_offset":9360,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":11616,"parent_record_offset":11584,"ancestor_record_offset":9360,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":11788,"parent_record_offset":11616,"ancestor_record_offset":9360,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":496,"record_offset":11900,"parent_record_offset":11876,"ancestor_record_offset":9360,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":496,"record_offset":11720,"parent_record_offset":11696,"ancestor_record_offset":9360,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":496,"record_offset":9760,"parent_record_offset":9664,"ancestor_record_offset":9360,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":496,"record_offset":10156,"parent_record_offset":10124,"ancestor_record_offset":9360,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":496,"record_offset":10844,"parent_record_offset":10384,"ancestor_record_offset":9360,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":496,"record_offset":11528,"parent_record_offset":11356,"ancestor_record_offset":9360,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":496,"record_offset":10072,"parent_record_offset":10044,"ancestor_record_offset":9360,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":496,"record_offset":11852,"parent_record_offset":11556,"ancestor_record_offset":9360,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":496,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":9360,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":496,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":9360,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":496,"record_offset":10804,"parent_record_offset":10772,"ancestor_record_offset":9360,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":496,"record_offset":11744,"parent_record_offset":11720,"ancestor_record_offset":9360,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":496,"record_offset":12252,"parent_record_offset":11324,"ancestor_record_offset":9360,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":496,"record_offset":9696,"parent_record_offset":9664,"ancestor_record_offset":9360,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":496,"record_offset":9724,"parent_record_offset":9696,"ancestor_record_offset":9360,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":496,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":9360,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":496,"record_offset":10900,"parent_record_offset":10876,"ancestor_record_offset":9360,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":496,"record_offset":10548,"parent_record_offset":10440,"ancestor_record_offset":9360,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":496,"record_offset":9664,"parent_record_offset":9636,"ancestor_record_offset":9360,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":496,"record_offset":10280,"parent_record_offset":10248,"ancestor_record_offset":9360,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":496,"record_offset":10412,"parent_record_offset":10384,"ancestor_record_offset":9360,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":496,"record_offset":10384,"parent_record_offset":10352,"ancestor_record_offset":9360,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":496,"record_offset":10328,"parent_record_offset":10304,"ancestor_record_offset":9360,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":496,"record_offset":10352,"parent_record_offset":10328,"ancestor_record_offset":9360,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":496,"record_offset":10304,"parent_record_offset":10280,"ancestor_record_offset":9360,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":496,"record_offset":10876,"parent_record_offset":10352,"ancestor_record_offset":9360,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":9860,"parent_record_offset":9828,"ancestor_record_offset":9360,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":10020,"parent_record_offset":9860,"ancestor_record_offset":9360,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":9360,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":9576,"parent_record_offset":9540,"ancestor_record_offset":9360,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":9360,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":9636,"parent_record_offset":9604,"ancestor_record_offset":9360,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":496,"record_offset":9604,"parent_record_offset":9576,"ancestor_record_offset":9360,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":496,"record_offset":10440,"parent_record_offset":10412,"ancestor_record_offset":9360,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":496,"record_offset":10688,"parent_record_offset":10440,"ancestor_record_offset":9360,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":496,"record_offset":10660,"parent_record_offset":10440,"ancestor_record_offset":9360,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":496,"record_offset":10492,"parent_record_offset":10440,"ancestor_record_offset":9360,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":12376,"parent_record_offset":12292,"ancestor_record_offset":9360,"inlinee":24644,"source_line":234,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":10216,"parent_record_offset":10124,"ancestor_record_offset":9360,"inlinee":24641,"source_line":222,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":12492,"parent_record_offset":12408,"ancestor_record_offset":9360,"inlinee":24642,"source_line":226,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":496,"record_offset":12608,"parent_record_offset":12524,"ancestor_record_offset":9360,"inlinee":24643,"source_line":230,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":11292,"parent_record_offset":11260,"ancestor_record_offset":9360,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":496,"record_offset":11392,"parent_record_offset":11356,"ancestor_record_offset":9360,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":11556,"parent_record_offset":11356,"ancestor_record_offset":9360,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":496,"record_offset":11824,"parent_record_offset":11556,"ancestor_record_offset":9360,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":496,"record_offset":11420,"parent_record_offset":11392,"ancestor_record_offset":9360,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":496,"record_offset":11452,"parent_record_offset":11420,"ancestor_record_offset":9360,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":496,"record_offset":11356,"parent_record_offset":11324,"ancestor_record_offset":9360,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":496,"record_offset":11324,"parent_record_offset":11292,"ancestor_record_offset":9360,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00496.jsonl.gz"}
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
// 完整结构及原始类型记录: audio_update.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: audio_update.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: audio_update.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::audio_update::audio_update_system
// 信息 ID: 334da61f9099a98ae1a94979
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 29163daaac11eaf3cb3a8671: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>)> @ 0x5e7e4d -> 0x6c53a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>)> > @ 0xe3f91f -> 0x6c53a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::audio_update::boot_audio_update_system
// 信息 ID: c8dcff0b26f259b980f6d93a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cdfe10352a97cf17480ce5fb: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> @ 0x387f9 -> 0x6c5880 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > @ 0xd312b -> 0x6c5880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
