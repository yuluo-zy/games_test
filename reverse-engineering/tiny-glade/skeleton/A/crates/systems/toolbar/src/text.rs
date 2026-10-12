// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/text.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 07849a5aaf41d4b61e09ecd5d15db7b818ebc29796dc52afc38feff05d805475
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: text.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1019, 1055, 2489, 3695
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"PDB_registry_path_only","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: []
// use walkdir; // namespace candidate
//   package identities: [{"name":"walkdir","source":"PDB_registry_path_only","version":"2.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2489,"record_offset":3192,"rva":23394736,"va":5392103856}
// 编译实例 RVAs: 0x164f9b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ttf_parser..language..Language$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-400308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e84ce4f9ff","size":5,"soft_fail":false,"target_va":5391704832,"va":5392105135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$walkdir..IntoIter$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12469603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e89dba41ff","size":5,"soft_fail":false,"target_va":5379634672,"va":5392104270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19832328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e8f861d1fe","size":5,"soft_fail":false,"target_va":5372274416,"va":5392106739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19790745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e86704d2fe","size":5,"soft_fail":false,"target_va":5372314208,"va":5392104948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19354552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e848acd8fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5392104259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14699602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e8aeb31fff","size":5,"soft_fail":false,"target_va":5377405008,"va":5392104605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14667489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e81f3120ff","size":5,"soft_fail":false,"target_va":5377436496,"va":5392103980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5fc30
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12517850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e826fe40ff","size":5,"soft_fail":false,"target_va":5379587120,"va":5392104965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14576162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e8de9521ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5392106045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h3f153f63f443ab13 (.llvm.15245336942322826116)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11434406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e8a679ae00","size":5,"soft_fail":false,"target_va":5403540528,"va":5392106117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14742273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e8ff0c1fff","size":5,"soft_fail":false,"target_va":5377364336,"va":5392106604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10874269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e89deda500","size":5,"soft_fail":false,"target_va":5402980128,"va":5392105854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14751068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e8a4ea1eff","size":5,"soft_fail":false,"target_va":5377353776,"va":5392104839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] slotmap::KeyData::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7159034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e806c392ff","size":5,"soft_fail":false,"target_va":5384946272,"va":5392105301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14664923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e8253b20ff","size":5,"soft_fail":false,"target_va":5377439664,"va":5392104582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::font_faces
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16067853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e8f3d20aff","size":5,"soft_fail":false,"target_va":5376037136,"va":5392104984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::load_font_from_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16067814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e81ad30aff","size":5,"soft_fail":false,"target_va":5376037072,"va":5392104881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] walkdir::dent::DirEntry::path
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-12461171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392103856,"raw_hex":"e88ddb41ff","size":5,"soft_fail":false,"target_va":5379643408,"va":5392104574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2489,"record_offset":5428,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":10987,"source_line":58,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":5372,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2489,"record_offset":5288,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2489,"record_offset":5316,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2489,"record_offset":3692,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2489,"record_offset":5700,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":5070,"source_line":65,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":4852,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2489,"record_offset":3440,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2489,"record_offset":8436,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2489,"record_offset":4980,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2489,"record_offset":5344,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2489,"record_offset":5204,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] walkdir::WalkDir::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"walkdir","version":"2.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/walkdir-2.3.2/src/lib.rs
//     evidence={"module":2489,"record_offset":3364,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":17286,"source_line":286,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] walkdir::impl$2::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"walkdir","version":"2.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/walkdir-2.3.2/src/lib.rs
//     evidence={"module":2489,"record_offset":3412,"parent_record_offset":3192,"ancestor_record_offset":3192,"inlinee":17288,"source_line":517,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2489,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":3192,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2489,"record_offset":4220,"parent_record_offset":4164,"ancestor_record_offset":3192,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2489,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":3192,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2489,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":3192,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2489,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":3192,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2489,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":3192,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2489,"record_offset":6256,"parent_record_offset":6200,"ancestor_record_offset":3192,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2489,"record_offset":6200,"parent_record_offset":6168,"ancestor_record_offset":3192,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":2489,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":3192,"inlinee":10981,"source_line":316,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2489,"record_offset":5576,"parent_record_offset":5428,"ancestor_record_offset":3192,"inlinee":10980,"source_line":33,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":3192,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2489,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":3192,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2489,"record_offset":6284,"parent_record_offset":6200,"ancestor_record_offset":3192,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2489,"record_offset":6820,"parent_record_offset":6776,"ancestor_record_offset":3192,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2489,"record_offset":5812,"parent_record_offset":5740,"ancestor_record_offset":3192,"inlinee":5074,"source_line":839,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$35::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2489,"record_offset":5740,"parent_record_offset":5700,"ancestor_record_offset":3192,"inlinee":5072,"source_line":605,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":2489,"record_offset":6664,"parent_record_offset":6636,"ancestor_record_offset":3192,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":2489,"record_offset":6636,"parent_record_offset":5740,"ancestor_record_offset":3192,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2489,"record_offset":5936,"parent_record_offset":5904,"ancestor_record_offset":3192,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2489,"record_offset":3724,"parent_record_offset":3692,"ancestor_record_offset":3192,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2489,"record_offset":3792,"parent_record_offset":3756,"ancestor_record_offset":3192,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2489,"record_offset":3756,"parent_record_offset":3724,"ancestor_record_offset":3192,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2489,"record_offset":5904,"parent_record_offset":5812,"ancestor_record_offset":3192,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2489,"record_offset":3820,"parent_record_offset":3792,"ancestor_record_offset":3192,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2489,"record_offset":6132,"parent_record_offset":6096,"ancestor_record_offset":3192,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":5872,"parent_record_offset":5812,"ancestor_record_offset":3192,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":6576,"parent_record_offset":5812,"ancestor_record_offset":3192,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":6600,"parent_record_offset":6576,"ancestor_record_offset":3192,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":6096,"parent_record_offset":5936,"ancestor_record_offset":3192,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":3192,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2489,"record_offset":3464,"parent_record_offset":3440,"ancestor_record_offset":3192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2489,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":3192,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2489,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":3192,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2489,"record_offset":6008,"parent_record_offset":5972,"ancestor_record_offset":3192,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2489,"record_offset":6312,"parent_record_offset":6200,"ancestor_record_offset":3192,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2489,"record_offset":3880,"parent_record_offset":3852,"ancestor_record_offset":3192,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2489,"record_offset":5028,"parent_record_offset":5004,"ancestor_record_offset":3192,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2489,"record_offset":5052,"parent_record_offset":5028,"ancestor_record_offset":3192,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2489,"record_offset":7144,"parent_record_offset":7120,"ancestor_record_offset":3192,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2489,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":3192,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2489,"record_offset":6036,"parent_record_offset":5972,"ancestor_record_offset":3192,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2489,"record_offset":5972,"parent_record_offset":5936,"ancestor_record_offset":3192,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2489,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":3192,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2489,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":3192,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":2489,"record_offset":6980,"parent_record_offset":6776,"ancestor_record_offset":3192,"inlinee":25367,"source_line":52,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::occupied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":2489,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":3192,"inlinee":13413,"source_line":48,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$12::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":2489,"record_offset":6776,"parent_record_offset":6748,"ancestor_record_offset":3192,"inlinee":25366,"source_line":1008,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2489,"record_offset":4912,"parent_record_offset":4884,"ancestor_record_offset":3192,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::impl$2::font_faces::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/lib.rs
//     evidence={"module":2489,"record_offset":7120,"parent_record_offset":7096,"ancestor_record_offset":3192,"inlinee":50941,"source_line":141,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::impl$2::font_faces::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/lib.rs
//     evidence={"module":2489,"record_offset":6168,"parent_record_offset":6132,"ancestor_record_offset":3192,"inlinee":50939,"source_line":144,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::text::text_startup_system::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/text.rs
//     evidence={"module":2489,"record_offset":3852,"parent_record_offset":3820,"ancestor_record_offset":3192,"inlinee":51391,"source_line":18,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::text::text_startup_system::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/text.rs
//     evidence={"module":2489,"record_offset":4884,"parent_record_offset":4852,"ancestor_record_offset":3192,"inlinee":84124,"source_line":20,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::text::text_startup_system::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/text.rs
//     evidence={"module":2489,"record_offset":5464,"parent_record_offset":5428,"ancestor_record_offset":3192,"inlinee":84125,"source_line":24,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC8BF
// 首条位置: {"module":1055,"inlinee":51391}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2489,"record_offset":3880,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2489,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":3852,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2489,"record_offset":4220,"parent_record_offset":4164,"ancestor_record_offset":3852,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2489,"record_offset":4192,"parent_record_offset":4164,"ancestor_record_offset":3852,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2489,"record_offset":4164,"parent_record_offset":4136,"ancestor_record_offset":3852,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2489,"record_offset":3920,"parent_record_offset":3880,"ancestor_record_offset":3852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1489C
// 首条位置: {"module":2489,"inlinee":84124}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2489,"record_offset":4912,"parent_record_offset":4884,"ancestor_record_offset":4884,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2489,"record_offset":4940,"parent_record_offset":4912,"ancestor_record_offset":4884,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1489D
// 首条位置: {"module":2489,"inlinee":84125}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2489,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":5464,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2489,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":5464,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":5464,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::update_text_renderer_scale
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2489,"record_offset":9060,"rva":23397648,"va":5392106768}
// 编译实例 RVAs: 0x1650510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10871186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e892e1a500","size":5,"soft_fail":false,"target_va":5402978688,"va":5392107497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e86cecffff","size":5,"soft_fail":false,"target_va":5392102576,"va":5392107583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10872885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e835e8a500","size":5,"soft_fail":false,"target_va":5402980128,"va":5392107238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e827f0ffff","size":5,"soft_fail":false,"target_va":5392102768,"va":5392106820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10873754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e89aeba500","size":5,"soft_fail":false,"target_va":5402980656,"va":5392106897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15750495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e8a1aa0fff","size":5,"soft_fail":false,"target_va":5376356912,"va":5392107402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-15746614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e8cab90fff","size":5,"soft_fail":false,"target_va":5376360560,"va":5392107169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15750662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e8faa90fff","size":5,"soft_fail":false,"target_va":5376356144,"va":5392106801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h6a28a4a69af7475d (.llvm.7250520506814577795)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12485210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e8a67d41ff","size":5,"soft_fail":false,"target_va":5379621984,"va":5392107189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::set_pixels_per_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16070247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392106768,"raw_hex":"e899c90aff","size":5,"soft_fail":false,"target_va":5376037216,"va":5392107458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-4914","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5392106768,"raw_hex":"e9ceecffff","size":5,"soft_fail":false,"target_va":5392102576,"va":5392107485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":11820,"parent_record_offset":9060,"ancestor_record_offset":9060,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2489,"record_offset":10788,"parent_record_offset":9060,"ancestor_record_offset":9060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2489,"record_offset":9200,"parent_record_offset":9060,"ancestor_record_offset":9060,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2489,"record_offset":9776,"parent_record_offset":9060,"ancestor_record_offset":9060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":11848,"parent_record_offset":11820,"ancestor_record_offset":9060,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2489,"record_offset":11172,"parent_record_offset":11144,"ancestor_record_offset":9060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2489,"record_offset":11224,"parent_record_offset":11200,"ancestor_record_offset":9060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2489,"record_offset":11200,"parent_record_offset":11172,"ancestor_record_offset":9060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2489,"record_offset":11112,"parent_record_offset":11084,"ancestor_record_offset":9060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2489,"record_offset":11144,"parent_record_offset":11112,"ancestor_record_offset":9060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2489,"record_offset":11316,"parent_record_offset":11144,"ancestor_record_offset":9060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2489,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":9060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2489,"record_offset":11248,"parent_record_offset":11224,"ancestor_record_offset":9060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":2489,"record_offset":9412,"parent_record_offset":9316,"ancestor_record_offset":9060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":11008,"parent_record_offset":10980,"ancestor_record_offset":9060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":10372,"parent_record_offset":9912,"ancestor_record_offset":9060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":11056,"parent_record_offset":10884,"ancestor_record_offset":9060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2489,"record_offset":9724,"parent_record_offset":9696,"ancestor_record_offset":9060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2489,"record_offset":11380,"parent_record_offset":11084,"ancestor_record_offset":9060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2489,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":9060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2489,"record_offset":10124,"parent_record_offset":10100,"ancestor_record_offset":9060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2489,"record_offset":10332,"parent_record_offset":10300,"ancestor_record_offset":9060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2489,"record_offset":11272,"parent_record_offset":11248,"ancestor_record_offset":9060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2489,"record_offset":11780,"parent_record_offset":10852,"ancestor_record_offset":9060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2489,"record_offset":9348,"parent_record_offset":9316,"ancestor_record_offset":9060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2489,"record_offset":9376,"parent_record_offset":9348,"ancestor_record_offset":9060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2489,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":9060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2489,"record_offset":10428,"parent_record_offset":10404,"ancestor_record_offset":9060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2489,"record_offset":10076,"parent_record_offset":9968,"ancestor_record_offset":9060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2489,"record_offset":9316,"parent_record_offset":9288,"ancestor_record_offset":9060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2489,"record_offset":9808,"parent_record_offset":9776,"ancestor_record_offset":9060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2489,"record_offset":9940,"parent_record_offset":9912,"ancestor_record_offset":9060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2489,"record_offset":9912,"parent_record_offset":9880,"ancestor_record_offset":9060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2489,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":9060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2489,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":9060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2489,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":9060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2489,"record_offset":10404,"parent_record_offset":9880,"ancestor_record_offset":9060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2489,"record_offset":9512,"parent_record_offset":9480,"ancestor_record_offset":9060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2489,"record_offset":9672,"parent_record_offset":9512,"ancestor_record_offset":9060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2489,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":9060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2489,"record_offset":9232,"parent_record_offset":9200,"ancestor_record_offset":9060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2489,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":9060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2489,"record_offset":9288,"parent_record_offset":9256,"ancestor_record_offset":9060,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2489,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":9060,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2489,"record_offset":9968,"parent_record_offset":9940,"ancestor_record_offset":9060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2489,"record_offset":10216,"parent_record_offset":9968,"ancestor_record_offset":9060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2489,"record_offset":10188,"parent_record_offset":9968,"ancestor_record_offset":9060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2489,"record_offset":10020,"parent_record_offset":9968,"ancestor_record_offset":9060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2489,"record_offset":10820,"parent_record_offset":10788,"ancestor_record_offset":9060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2489,"record_offset":10920,"parent_record_offset":10884,"ancestor_record_offset":9060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2489,"record_offset":11084,"parent_record_offset":10884,"ancestor_record_offset":9060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2489,"record_offset":11352,"parent_record_offset":11084,"ancestor_record_offset":9060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2489,"record_offset":10948,"parent_record_offset":10920,"ancestor_record_offset":9060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2489,"record_offset":10980,"parent_record_offset":10948,"ancestor_record_offset":9060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2489,"record_offset":10884,"parent_record_offset":10852,"ancestor_record_offset":9060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2489,"record_offset":10852,"parent_record_offset":10820,"ancestor_record_offset":9060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::prepare_text_fragments
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2489,"record_offset":11884,"rva":23398496,"va":5392107616}
// 编译实例 RVAs: 0x1650860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::prepare
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16063104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392107616,"raw_hex":"e880e50aff","size":5,"soft_fail":false,"target_va":5376044704,"va":5392107803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":12968,"parent_record_offset":11884,"ancestor_record_offset":11884,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":12912,"parent_record_offset":11884,"ancestor_record_offset":11884,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2489,"record_offset":12216,"parent_record_offset":11884,"ancestor_record_offset":11884,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2489,"record_offset":12020,"parent_record_offset":11884,"ancestor_record_offset":11884,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":11884,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":12936,"parent_record_offset":12912,"ancestor_record_offset":11884,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2489,"record_offset":12576,"parent_record_offset":12244,"ancestor_record_offset":11884,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2489,"record_offset":12292,"parent_record_offset":12244,"ancestor_record_offset":11884,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2489,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":11884,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2489,"record_offset":12092,"parent_record_offset":12068,"ancestor_record_offset":11884,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2489,"record_offset":12244,"parent_record_offset":12216,"ancestor_record_offset":11884,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2489,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":11884,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2489,"record_offset":12540,"parent_record_offset":12516,"ancestor_record_offset":11884,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2489,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":11884,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2489,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":11884,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2489,"record_offset":12516,"parent_record_offset":12292,"ancestor_record_offset":11884,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2489,"record_offset":12876,"parent_record_offset":12244,"ancestor_record_offset":11884,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2489,"record_offset":12320,"parent_record_offset":12292,"ancestor_record_offset":11884,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2489,"record_offset":12600,"parent_record_offset":12576,"ancestor_record_offset":11884,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2489,"record_offset":12792,"parent_record_offset":12244,"ancestor_record_offset":11884,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":11884,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2489,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":11884,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2489,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":11884,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2489,"record_offset":12756,"parent_record_offset":12732,"ancestor_record_offset":11884,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2489,"record_offset":12704,"parent_record_offset":12668,"ancestor_record_offset":11884,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2489,"record_offset":12816,"parent_record_offset":12792,"ancestor_record_offset":11884,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2489,"record_offset":12416,"parent_record_offset":12392,"ancestor_record_offset":11884,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2489,"record_offset":12116,"parent_record_offset":12092,"ancestor_record_offset":11884,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2489,"record_offset":12840,"parent_record_offset":12816,"ancestor_record_offset":11884,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2489,"record_offset":12440,"parent_record_offset":12416,"ancestor_record_offset":11884,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2489,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":11884,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2489,"record_offset":12732,"parent_record_offset":12668,"ancestor_record_offset":11884,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2489,"record_offset":12668,"parent_record_offset":12244,"ancestor_record_offset":11884,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::render_text_fragments
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2489,"record_offset":13028,"rva":23398768,"va":5392107888}
// 编译实例 RVAs: 0x1650970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16062981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392107888,"raw_hex":"e8fbe50aff","size":5,"soft_fail":false,"target_va":5376045152,"va":5392108128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":14060,"parent_record_offset":13028,"ancestor_record_offset":13028,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":14004,"parent_record_offset":13028,"ancestor_record_offset":13028,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2489,"record_offset":13356,"parent_record_offset":13028,"ancestor_record_offset":13028,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2489,"record_offset":13160,"parent_record_offset":13028,"ancestor_record_offset":13028,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":13028,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":14028,"parent_record_offset":14004,"ancestor_record_offset":13028,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2489,"record_offset":13672,"parent_record_offset":13552,"ancestor_record_offset":13028,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2489,"record_offset":13464,"parent_record_offset":13440,"ancestor_record_offset":13028,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2489,"record_offset":13580,"parent_record_offset":13552,"ancestor_record_offset":13028,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2489,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":13028,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2489,"record_offset":13232,"parent_record_offset":13208,"ancestor_record_offset":13028,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2489,"record_offset":13388,"parent_record_offset":13356,"ancestor_record_offset":13028,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2489,"record_offset":13184,"parent_record_offset":13160,"ancestor_record_offset":13028,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2489,"record_offset":13552,"parent_record_offset":13388,"ancestor_record_offset":13028,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2489,"record_offset":13440,"parent_record_offset":13388,"ancestor_record_offset":13028,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2489,"record_offset":13512,"parent_record_offset":13488,"ancestor_record_offset":13028,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2489,"record_offset":13488,"parent_record_offset":13464,"ancestor_record_offset":13028,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2489,"record_offset":13968,"parent_record_offset":13388,"ancestor_record_offset":13028,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2489,"record_offset":13604,"parent_record_offset":13580,"ancestor_record_offset":13028,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2489,"record_offset":13884,"parent_record_offset":13388,"ancestor_record_offset":13028,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":2489,"record_offset":13696,"parent_record_offset":13672,"ancestor_record_offset":13028,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2489,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":13028,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2489,"record_offset":13848,"parent_record_offset":13824,"ancestor_record_offset":13028,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2489,"record_offset":13796,"parent_record_offset":13760,"ancestor_record_offset":13028,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2489,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":13028,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2489,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":13028,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2489,"record_offset":13932,"parent_record_offset":13908,"ancestor_record_offset":13028,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2489,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":13028,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2489,"record_offset":13824,"parent_record_offset":13760,"ancestor_record_offset":13028,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2489,"record_offset":13760,"parent_record_offset":13388,"ancestor_record_offset":13028,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::flush_text_render
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2489,"record_offset":14120,"rva":23399088,"va":5392108208}
// 编译实例 RVAs: 0x1650ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16735892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392108208,"raw_hex":"e86ca100ff","size":5,"soft_fail":false,"target_va":5375372576,"va":5392108463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12534659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392108208,"raw_hex":"e87dbc40ff","size":5,"soft_fail":false,"target_va":5379573840,"va":5392108494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9b310
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5548095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392108208,"raw_hex":"e83fa85400","size":5,"soft_fail":false,"target_va":5397656336,"va":5392108236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11485037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392108208,"raw_hex":"e86d3faf00","size":5,"soft_fail":false,"target_va":5403593344,"va":5392108302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10870313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392108208,"raw_hex":"e829dea500","size":5,"soft_fail":false,"target_va":5402978880,"va":5392108562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11348357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392108208,"raw_hex":"e88529ad00","size":5,"soft_fail":false,"target_va":5403456912,"va":5392108550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392108208,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5392108322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":14792,"parent_record_offset":14120,"ancestor_record_offset":14120,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2489,"record_offset":14480,"parent_record_offset":14120,"ancestor_record_offset":14120,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2489,"record_offset":14244,"parent_record_offset":14120,"ancestor_record_offset":14120,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2489,"record_offset":14820,"parent_record_offset":14792,"ancestor_record_offset":14120,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2489,"record_offset":14312,"parent_record_offset":14272,"ancestor_record_offset":14120,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2489,"record_offset":14396,"parent_record_offset":14368,"ancestor_record_offset":14120,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2489,"record_offset":14424,"parent_record_offset":14396,"ancestor_record_offset":14120,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2489,"record_offset":14340,"parent_record_offset":14312,"ancestor_record_offset":14120,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2489,"record_offset":14272,"parent_record_offset":14244,"ancestor_record_offset":14120,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/02489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::flush_text_render::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC704
// 首条位置: {"module":1019,"inlinee":50948}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1019,"record_offset":6964,"parent_record_offset":6564,"ancestor_record_offset":6564,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01019.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1019,"record_offset":6620,"parent_record_offset":6564,"ancestor_record_offset":6564,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01019.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1019,"record_offset":6748,"parent_record_offset":6648,"ancestor_record_offset":6564,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01019.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1019,"record_offset":6692,"parent_record_offset":6648,"ancestor_record_offset":6564,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01019.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1019,"record_offset":6648,"parent_record_offset":6620,"ancestor_record_offset":6564,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01019.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1019,"record_offset":6720,"parent_record_offset":6648,"ancestor_record_offset":6564,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01019.jsonl.gz"}
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
// 完整结构及原始类型记录: text.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: text.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: text.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 23; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>", "bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: toolbar::text::flush_text_render
// 信息 ID: dfc20adf9b91e2b071d70532
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8899861bcffb74c21d1ad7c3: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> @ 0xa8b1ff -> 0x1650ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x15d8e3a -> 0x1650ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::flush_text_render::closure$0
// 信息 ID: 63ed39df83c0d163ea3b7856
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::text::flush_text_render", "captures": "unknown"}
// [闭包环境类型引用] toolbar::text::flush_text_render::closure_env$0 <- tg_text::TextRenderer::consume<toolbar::text::flush_text_render::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] tg_text::TextRenderer::consume<toolbar::text::flush_text_render::closure_env$0>; module=1019, inline_record=6564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::prepare_text_fragments
// 信息 ID: 7bf7b0c7a33ca1669c458cf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8e3d842129617e04da1c3ac2: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)> @ 0xa8afa5 -> 0x1650860 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)> > @ 0x15e89ed -> 0x1650860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::render_text_fragments
// 信息 ID: 119b25d4bcef8dfd234b6cc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3e826ead3593b9199e32767d: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)> @ 0xa8adf5 -> 0x1650970 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)> > @ 0x15e1d1d -> 0x1650970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system
// 信息 ID: 27b218cbb5df454c66d938e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1a12d802edafed7f79fb1a9f: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)> @ 0xa8ac51 -> 0x164f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)> > @ 0x15e78f7 -> 0x164f9b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system::closure$0
// 信息 ID: ed618a3861a56077700a154e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::text::text_startup_system", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::text::text_startup_system; module=2489, inline_record=3852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system::closure$1
// 信息 ID: e460b5316010f23e437229b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::text::text_startup_system", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::text::text_startup_system; module=2489, inline_record=4884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system::closure$2
// 信息 ID: 6ce57679d8c43ab89d5b17b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::text::text_startup_system", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::text::text_startup_system; module=2489, inline_record=5464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::update_text_renderer_scale
// 信息 ID: 108bf7c2f7cf44a96f8c86e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b102c492741a390ab7ddbd1e: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> @ 0xa8b46f -> 0x1650510 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> > @ 0x15e6ab1 -> 0x1650510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
