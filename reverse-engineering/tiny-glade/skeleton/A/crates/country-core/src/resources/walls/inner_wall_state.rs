// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/walls/inner_wall_state.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 788ee156d0744dfe3ec1a2f1851a4353f429f5404ef9d08b4f5492cf8baa348a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: inner_wall_state.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 55, 100, 114, 375, 376, 389, 392, 409, 423, 431, 435, 444, 452, 461, 464, 474, 480, 486, 492, 1253, 1258, 1342, 1389, 1432, 1440, 1459, 1461, 3467, 4255
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 19, 26, 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x616F
// 首条位置: {"module":431,"inlinee":24943}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":444088,"parent_record_offset":444064,"ancestor_record_offset":444064,"inlinee":6853,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":497324,"parent_record_offset":497300,"ancestor_record_offset":497300,"inlinee":6856,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":444112,"parent_record_offset":444088,"ancestor_record_offset":444064,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 19, 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10466
// 首条位置: {"module":1440,"inlinee":66662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":341960,"parent_record_offset":341936,"ancestor_record_offset":341936,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":341984,"parent_record_offset":341960,"ancestor_record_offset":341936,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":342008,"parent_record_offset":341984,"ancestor_record_offset":341936,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":342032,"parent_record_offset":342008,"ancestor_record_offset":341936,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 19, 26, 60, 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10432
// 首条位置: {"module":1440,"inlinee":66610}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":340792,"parent_record_offset":340764,"ancestor_record_offset":340764,"inlinee":66649,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":168204,"parent_record_offset":168176,"ancestor_record_offset":168176,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":168484,"parent_record_offset":168460,"ancestor_record_offset":168176,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":168460,"parent_record_offset":168404,"ancestor_record_offset":168176,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":168432,"parent_record_offset":168404,"ancestor_record_offset":168176,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":168404,"parent_record_offset":168380,"ancestor_record_offset":168176,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1440,"record_offset":340880,"parent_record_offset":340848,"ancestor_record_offset":340764,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":341984,"parent_record_offset":341960,"ancestor_record_offset":341784,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":168332,"parent_record_offset":168300,"ancestor_record_offset":168176,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":342008,"parent_record_offset":341984,"ancestor_record_offset":341784,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":342032,"parent_record_offset":342008,"ancestor_record_offset":341784,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":341960,"parent_record_offset":341936,"ancestor_record_offset":341784,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1440,"record_offset":341936,"parent_record_offset":341868,"ancestor_record_offset":341784,"inlinee":66662,"source_line":92,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":168300,"parent_record_offset":168268,"ancestor_record_offset":168176,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1440,"record_offset":340820,"parent_record_offset":340792,"ancestor_record_offset":340764,"inlinee":66650,"source_line":615,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":168236,"parent_record_offset":168204,"ancestor_record_offset":168176,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":168268,"parent_record_offset":168236,"ancestor_record_offset":168176,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 19, 26, 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1061C
// 首条位置: {"module":1459,"inlinee":67100}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1459,"record_offset":34104,"parent_record_offset":34048,"ancestor_record_offset":34048,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1459,"record_offset":34136,"parent_record_offset":34048,"ancestor_record_offset":34048,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1459,"record_offset":34076,"parent_record_offset":34048,"ancestor_record_offset":34048,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 19, 26, 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1459,"record_offset":33256,"rva":17968784,"va":5386677904}
// 编译实例 RVAs: 0x1122e90, 0x11236f0, 0x11244c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t16281896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e82871f800","size":5,"soft_fail":false,"target_va":5402960384,"va":5386678483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-10661429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e8cb515dff","size":5,"soft_fail":false,"target_va":5376016896,"va":5386678320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13929366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e86a742bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386679073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=9
//     target_aliases=7 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12437444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e83c3842ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5386679711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140547330
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12434440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e8f84342ff","size":5,"soft_fail":false,"target_va":5374243632,"va":5386678067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10786306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e8fe695bff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386678173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10748263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e899fe5bff","size":5,"soft_fail":false,"target_va":5375930160,"va":5386678418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4b80
//     owner=unresolved_namespace; count=6
//     target_aliases=4 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10741717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e82b185cff","size":5,"soft_fail":false,"target_va":5375937408,"va":5386679120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1360
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10690636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e8b4df5cff","size":5,"soft_fail":false,"target_va":5375988576,"va":5386679207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d8e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-305672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386680048,"raw_hex":"e8f855fbff","size":5,"soft_fail":false,"target_va":5386374736,"va":5386680403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d8f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-303208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e8985ffbff","size":5,"soft_fail":false,"target_va":5386375056,"va":5386678259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d9180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-308392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386683584,"raw_hex":"e8584bfbff","size":5,"soft_fail":false,"target_va":5386375552,"va":5386683939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9330
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16278437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e8a563f800","size":5,"soft_fail":false,"target_va":5402956592,"va":5386678150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t16281295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386677904,"raw_hex":"e8cf6ef800","size":5,"soft_fail":false,"target_va":5402959760,"va":5386678460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1459,"record_offset":33448,"parent_record_offset":33256,"ancestor_record_offset":33256,"inlinee":25381,"source_line":255,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1459,"record_offset":34436,"parent_record_offset":34412,"ancestor_record_offset":33256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1459,"record_offset":34412,"parent_record_offset":34320,"ancestor_record_offset":33256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1459,"record_offset":35480,"parent_record_offset":35452,"ancestor_record_offset":33256,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1459,"record_offset":33648,"parent_record_offset":33620,"ancestor_record_offset":33256,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1459,"record_offset":33712,"parent_record_offset":33680,"ancestor_record_offset":33256,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1459,"record_offset":33772,"parent_record_offset":33740,"ancestor_record_offset":33256,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1459,"record_offset":33800,"parent_record_offset":33772,"ancestor_record_offset":33256,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1459,"record_offset":34348,"parent_record_offset":34320,"ancestor_record_offset":33256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1459,"record_offset":34788,"parent_record_offset":34764,"ancestor_record_offset":33256,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1459,"record_offset":34320,"parent_record_offset":34296,"ancestor_record_offset":33256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1459,"record_offset":34764,"parent_record_offset":34736,"ancestor_record_offset":33256,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1459,"record_offset":34232,"parent_record_offset":34200,"ancestor_record_offset":33256,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1459,"record_offset":35572,"parent_record_offset":35480,"ancestor_record_offset":33256,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1459,"record_offset":34376,"parent_record_offset":34348,"ancestor_record_offset":33256,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1459,"record_offset":33740,"parent_record_offset":33712,"ancestor_record_offset":33256,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=114
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1459,"record_offset":34020,"parent_record_offset":33448,"ancestor_record_offset":33256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1459,"record_offset":33880,"parent_record_offset":33856,"ancestor_record_offset":33256,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1459,"record_offset":35508,"parent_record_offset":35480,"ancestor_record_offset":33256,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1459,"record_offset":34104,"parent_record_offset":34048,"ancestor_record_offset":33256,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1459,"record_offset":34136,"parent_record_offset":34048,"ancestor_record_offset":33256,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1459,"record_offset":33832,"parent_record_offset":33800,"ancestor_record_offset":33256,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1459,"record_offset":33856,"parent_record_offset":33832,"ancestor_record_offset":33256,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1459,"record_offset":34048,"parent_record_offset":33448,"ancestor_record_offset":33256,"inlinee":67100,"source_line":92,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1459,"record_offset":34076,"parent_record_offset":34048,"ancestor_record_offset":33256,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1459,"record_offset":33620,"parent_record_offset":33448,"ancestor_record_offset":33256,"inlinee":25383,"source_line":877,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1459,"record_offset":33680,"parent_record_offset":33448,"ancestor_record_offset":33256,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 19, 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EC3
// 首条位置: {"module":55,"inlinee":7875}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":100,"record_offset":7088,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":7826,"source_line":170,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":100,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":7060,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":100,"record_offset":7400,"parent_record_offset":7376,"ancestor_record_offset":7060,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":100,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":7060,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":100,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":7060,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":100,"record_offset":7304,"parent_record_offset":7280,"ancestor_record_offset":7060,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":100,"record_offset":7912,"parent_record_offset":7768,"ancestor_record_offset":7060,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":7256,"parent_record_offset":7232,"ancestor_record_offset":7060,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":7232,"parent_record_offset":7208,"ancestor_record_offset":7060,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":7060,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":7208,"parent_record_offset":7184,"ancestor_record_offset":7060,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":7060,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":100,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":7060,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":100,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":7060,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":100,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":7060,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":100,"record_offset":7948,"parent_record_offset":7704,"ancestor_record_offset":7060,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":100,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":7060,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":100,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":7060,"inlinee":7827,"source_line":150,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$6::fmt
// 记录形式: compiled_procedure; instances=3
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":375,"record_offset":4492,"rva":5204080,"va":5373913200}
// 编译实例 RVAs: 0x4f6870, 0x4f8f70, 0x5dabb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3670678","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373913200,"raw_hex":"e996023800","size":5,"soft_fail":false,"target_va":5377583920,"va":5373913237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 26, 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1459,"record_offset":43252,"rva":17973072,"va":5386682192}
// 编译实例 RVAs: 0x1123f50, 0x11242f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16274686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682192,"raw_hex":"e8fe54f800","size":5,"soft_fail":false,"target_va":5402957088,"va":5386682397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16909963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386683120,"raw_hex":"e88b060201","size":5,"soft_fail":false,"target_va":5403593344,"va":5386683376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1459,"record_offset":43456,"parent_record_offset":43252,"ancestor_record_offset":43252,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1459,"record_offset":43476,"parent_record_offset":43456,"ancestor_record_offset":43252,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1459,"record_offset":43496,"parent_record_offset":43476,"ancestor_record_offset":43252,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1459,"record_offset":43516,"parent_record_offset":43496,"ancestor_record_offset":43252,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=4
// 源码行: 26, 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":114,"record_offset":3328,"rva":1723872,"va":5370432992}
// 编译实例 RVAs: 0x1a4de0, 0x1a50e0, 0x1124030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140017e80
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1626704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e8b02de7ff","size":5,"soft_fail":false,"target_va":5368807040,"va":5370433739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401505d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-346691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e8bdb5faff","size":5,"soft_fail":false,"target_va":5370086864,"va":5370433550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140154360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-330479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e811f5faff","size":5,"soft_fail":false,"target_va":5370102624,"va":5370433098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401552b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-326637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e81304fbff","size":5,"soft_fail":false,"target_va":5370106544,"va":5370433176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401555d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-325950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e8c206fbff","size":5,"soft_fail":false,"target_va":5370107344,"va":5370433289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140155c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-324305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e82f0dfbff","size":5,"soft_fail":false,"target_va":5370109024,"va":5370433324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401565c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-321718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e84a17fbff","size":5,"soft_fail":false,"target_va":5370111424,"va":5370433137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401571e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-318839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e88922fbff","size":5,"soft_fail":false,"target_va":5370114528,"va":5370433362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1820344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e8b8c61b00","size":5,"soft_fail":false,"target_va":5372253808,"va":5370433459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2315888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370433760,"raw_hex":"e870562300","size":5,"soft_fail":false,"target_va":5372749696,"va":5370433803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2316219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e8bb572300","size":5,"soft_fail":false,"target_va":5372749712,"va":5370433488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405971e0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4137937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e8d1233f00","size":5,"soft_fail":false,"target_va":5374570976,"va":5370433034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cac50
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11900572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682416,"raw_hex":"e864694aff","size":5,"soft_fail":false,"target_va":5374782544,"va":5386683111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10790999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682416,"raw_hex":"e8a9575bff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386682866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406fa580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5592380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432992,"raw_hex":"e83c555500","size":5,"soft_fail":false,"target_va":5376025984,"va":5370433599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-581088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682416,"raw_hex":"e82022f7ff","size":5,"soft_fail":false,"target_va":5386101616,"va":5386682699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-578985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682416,"raw_hex":"e8572af7ff","size":5,"soft_fail":false,"target_va":5386103968,"va":5386682948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096f40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-577912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682416,"raw_hex":"e8882ef7ff","size":5,"soft_fail":false,"target_va":5386104640,"va":5386682547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410993d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-568591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682416,"raw_hex":"e8f152f7ff","size":5,"soft_fail":false,"target_va":5386114000,"va":5386682586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109aca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-562387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682416,"raw_hex":"e82d6bf7ff","size":5,"soft_fail":false,"target_va":5386120352,"va":5386682734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109e120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-548721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682416,"raw_hex":"e88fa0f7ff","size":5,"soft_fail":false,"target_va":5386133792,"va":5386682508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109eb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-546422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386682416,"raw_hex":"e88aa9f7ff","size":5,"soft_fail":false,"target_va":5386136352,"va":5386682769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32544176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370433760,"raw_hex":"e8b095f001","size":5,"soft_fail":false,"target_va":5402978144,"va":5370433963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":3552,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":114,"record_offset":3524,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1459,"record_offset":44916,"parent_record_offset":44052,"ancestor_record_offset":44052,"inlinee":24220,"source_line":132,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":114,"record_offset":4384,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":7826,"source_line":170,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":4652,"parent_record_offset":4632,"ancestor_record_offset":4192,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":4192,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":3328,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":4192,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":3912,"parent_record_offset":3856,"ancestor_record_offset":3328,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":114,"record_offset":3628,"parent_record_offset":3604,"ancestor_record_offset":3328,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":114,"record_offset":3684,"parent_record_offset":3628,"ancestor_record_offset":3328,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":3328,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":4608,"parent_record_offset":4588,"ancestor_record_offset":4192,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":4588,"parent_record_offset":4568,"ancestor_record_offset":4192,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":5184,"parent_record_offset":5040,"ancestor_record_offset":4192,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":3856,"parent_record_offset":3828,"ancestor_record_offset":3328,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":4548,"parent_record_offset":4528,"ancestor_record_offset":4192,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":4192,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":4480,"parent_record_offset":4460,"ancestor_record_offset":4192,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":4192,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":4460,"parent_record_offset":4440,"ancestor_record_offset":4192,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":114,"record_offset":4440,"parent_record_offset":4412,"ancestor_record_offset":4192,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":114,"record_offset":4568,"parent_record_offset":4548,"ancestor_record_offset":4192,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":114,"record_offset":5244,"parent_record_offset":5220,"ancestor_record_offset":4192,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":114,"record_offset":3716,"parent_record_offset":3628,"ancestor_record_offset":3328,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":114,"record_offset":5220,"parent_record_offset":4980,"ancestor_record_offset":4192,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":3580,"parent_record_offset":3552,"ancestor_record_offset":3328,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":3652,"parent_record_offset":3628,"ancestor_record_offset":3328,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":4192,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":114,"record_offset":4412,"parent_record_offset":4384,"ancestor_record_offset":4192,"inlinee":7827,"source_line":150,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$10::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":435,"record_offset":53964,"rva":6138848,"va":5374847968}
// 编译实例 RVAs: 0x5dabe0, 0x1dc8d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2735913","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374847968,"raw_hex":"e929bf2900","size":5,"soft_fail":false,"target_va":5377583920,"va":5374848002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$12::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A08D
// 首条位置: {"module":3467,"inlinee":106637}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x578B
// 首条位置: {"module":409,"inlinee":22411}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 45, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x578C
// 首条位置: {"module":392,"record_offset":5060,"rva":5351936,"va":5374061056}
// 编译实例 RVAs: 0x51aa00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57B1
// 首条位置: {"module":1459,"inlinee":22449}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10430
// 首条位置: {"module":1440,"inlinee":66608}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":169004,"parent_record_offset":166676,"ancestor_record_offset":166676,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":172432,"parent_record_offset":166676,"ancestor_record_offset":166676,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":169652,"parent_record_offset":166676,"ancestor_record_offset":166676,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":172256,"parent_record_offset":166676,"ancestor_record_offset":166676,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":166724,"parent_record_offset":166676,"ancestor_record_offset":166676,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":169032,"parent_record_offset":166676,"ancestor_record_offset":166676,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":167380,"parent_record_offset":167356,"ancestor_record_offset":166676,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":167356,"parent_record_offset":167300,"ancestor_record_offset":166676,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":168004,"parent_record_offset":167976,"ancestor_record_offset":166676,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":168064,"parent_record_offset":168004,"ancestor_record_offset":166676,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":166876,"parent_record_offset":166852,"ancestor_record_offset":166676,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":166928,"parent_record_offset":166900,"ancestor_record_offset":166676,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1440,"record_offset":166952,"parent_record_offset":166928,"ancestor_record_offset":166676,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":167328,"parent_record_offset":167300,"ancestor_record_offset":166676,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":167516,"parent_record_offset":167492,"ancestor_record_offset":166676,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":167300,"parent_record_offset":167272,"ancestor_record_offset":166676,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":167492,"parent_record_offset":167460,"ancestor_record_offset":166676,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":167460,"parent_record_offset":167428,"ancestor_record_offset":166676,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":168096,"parent_record_offset":168004,"ancestor_record_offset":166676,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":169156,"parent_record_offset":169132,"ancestor_record_offset":166676,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":167664,"parent_record_offset":167636,"ancestor_record_offset":166676,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":166900,"parent_record_offset":166876,"ancestor_record_offset":166676,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":169132,"parent_record_offset":169096,"ancestor_record_offset":166676,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":167168,"parent_record_offset":166796,"ancestor_record_offset":166676,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":167028,"parent_record_offset":167004,"ancestor_record_offset":166676,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":168032,"parent_record_offset":168004,"ancestor_record_offset":166676,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":169212,"parent_record_offset":169156,"ancestor_record_offset":166676,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":168756,"parent_record_offset":166796,"ancestor_record_offset":166676,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":169296,"parent_record_offset":169272,"ancestor_record_offset":166676,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1440,"record_offset":166980,"parent_record_offset":166952,"ancestor_record_offset":166676,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1440,"record_offset":167004,"parent_record_offset":166980,"ancestor_record_offset":166676,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1440,"record_offset":172152,"parent_record_offset":172128,"ancestor_record_offset":166676,"inlinee":24935,"source_line":313,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1440,"record_offset":168176,"parent_record_offset":168144,"ancestor_record_offset":166676,"inlinee":66610,"source_line":92,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1440,"record_offset":169272,"parent_record_offset":169248,"ancestor_record_offset":166676,"inlinee":66612,"source_line":60,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1440,"record_offset":171816,"parent_record_offset":171792,"ancestor_record_offset":166676,"inlinee":66614,"source_line":60,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":172392,"parent_record_offset":172368,"ancestor_record_offset":166676,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":172368,"parent_record_offset":172344,"ancestor_record_offset":166676,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":166760,"parent_record_offset":166724,"ancestor_record_offset":166676,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":169064,"parent_record_offset":169032,"ancestor_record_offset":166676,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":168300,"parent_record_offset":168268,"ancestor_record_offset":166676,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1440,"record_offset":169916,"parent_record_offset":169892,"ancestor_record_offset":166676,"inlinee":7237,"source_line":232,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":168144,"parent_record_offset":166796,"ancestor_record_offset":166676,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":172176,"parent_record_offset":172152,"ancestor_record_offset":166676,"inlinee":10276,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":169608,"parent_record_offset":169584,"ancestor_record_offset":166676,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1440,"record_offset":166852,"parent_record_offset":166796,"ancestor_record_offset":166676,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":168236,"parent_record_offset":168204,"ancestor_record_offset":166676,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":168204,"parent_record_offset":168176,"ancestor_record_offset":166676,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":168268,"parent_record_offset":168236,"ancestor_record_offset":166676,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":166796,"parent_record_offset":166760,"ancestor_record_offset":166676,"inlinee":50437,"source_line":610,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":169096,"parent_record_offset":169064,"ancestor_record_offset":166676,"inlinee":50438,"source_line":626,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10434
// 首条位置: {"module":1440,"inlinee":66612}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":169296,"parent_record_offset":169272,"ancestor_record_offset":169272,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10436
// 首条位置: {"module":1440,"inlinee":66614}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":171840,"parent_record_offset":171816,"ancestor_record_offset":171816,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1041F
// 首条位置: {"module":1440,"inlinee":66591}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":109644,"parent_record_offset":109060,"ancestor_record_offset":109060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":110816,"parent_record_offset":109060,"ancestor_record_offset":109060,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":109092,"parent_record_offset":109060,"ancestor_record_offset":109060,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":109876,"parent_record_offset":109852,"ancestor_record_offset":109060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":109852,"parent_record_offset":109740,"ancestor_record_offset":109060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":109776,"parent_record_offset":109740,"ancestor_record_offset":109060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":109740,"parent_record_offset":109708,"ancestor_record_offset":109060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1440,"record_offset":109176,"parent_record_offset":109148,"ancestor_record_offset":109060,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":109816,"parent_record_offset":109776,"ancestor_record_offset":109060,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":109676,"parent_record_offset":109644,"ancestor_record_offset":109060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":109236,"parent_record_offset":109176,"ancestor_record_offset":109060,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":109204,"parent_record_offset":109176,"ancestor_record_offset":109060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":109604,"parent_record_offset":109424,"ancestor_record_offset":109060,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":109316,"parent_record_offset":109292,"ancestor_record_offset":109060,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1440,"record_offset":109292,"parent_record_offset":109268,"ancestor_record_offset":109060,"inlinee":66593,"source_line":60,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":109120,"parent_record_offset":109092,"ancestor_record_offset":109060,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":109268,"parent_record_offset":109148,"ancestor_record_offset":109060,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":109572,"parent_record_offset":109548,"ancestor_record_offset":109060,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":109148,"parent_record_offset":109120,"ancestor_record_offset":109060,"inlinee":24451,"source_line":575,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10421
// 首条位置: {"module":1440,"inlinee":66593}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":109316,"parent_record_offset":109292,"ancestor_record_offset":109292,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10455
// 首条位置: {"module":1440,"inlinee":66645}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":320312,"parent_record_offset":320288,"ancestor_record_offset":320288,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25DD
// 首条位置: {"module":100,"inlinee":9693}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$14::clone
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=38
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FDD
// 首条位置: {"module":423,"inlinee":24541}
// 编译实例 RVAs: 0x6834d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2788782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375538384,"raw_hex":"e85272d5ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375538473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2789143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375538384,"raw_hex":"e8e970d5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375538850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28054838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375538384,"raw_hex":"e83615ac01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375538517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27439648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375538384,"raw_hex":"e820b2a201","size":5,"soft_fail":false,"target_va":5402978144,"va":5375538491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1988358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375538384,"raw_hex":"e806571e00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375538757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=25
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":423,"record_offset":18724,"parent_record_offset":17300,"ancestor_record_offset":17300,"inlinee":24553,"source_line":11,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=28
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":17368,"parent_record_offset":17300,"ancestor_record_offset":17300,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17796,"parent_record_offset":17772,"ancestor_record_offset":17300,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17820,"parent_record_offset":17796,"ancestor_record_offset":17300,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":18676,"parent_record_offset":18652,"ancestor_record_offset":17300,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17772,"parent_record_offset":17664,"ancestor_record_offset":17300,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":18652,"parent_record_offset":18560,"ancestor_record_offset":17300,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":18588,"parent_record_offset":18560,"ancestor_record_offset":17300,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":17664,"parent_record_offset":17628,"ancestor_record_offset":17300,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":17628,"parent_record_offset":17592,"ancestor_record_offset":17300,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":17892,"parent_record_offset":17628,"ancestor_record_offset":17300,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":18560,"parent_record_offset":18536,"ancestor_record_offset":17300,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":17548,"parent_record_offset":17508,"ancestor_record_offset":17300,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":17508,"parent_record_offset":17468,"ancestor_record_offset":17300,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":17468,"parent_record_offset":17428,"ancestor_record_offset":17300,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17592,"parent_record_offset":17548,"ancestor_record_offset":17300,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17428,"parent_record_offset":17368,"ancestor_record_offset":17300,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":17708,"parent_record_offset":17664,"ancestor_record_offset":17300,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":17736,"parent_record_offset":17708,"ancestor_record_offset":17300,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":423,"record_offset":17956,"parent_record_offset":17932,"ancestor_record_offset":17300,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":18616,"parent_record_offset":18588,"ancestor_record_offset":17300,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":17932,"parent_record_offset":17548,"ancestor_record_offset":17300,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=56
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":18512,"parent_record_offset":17368,"ancestor_record_offset":17300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":17848,"parent_record_offset":17820,"ancestor_record_offset":17300,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$21::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x562D
// 首条位置: {"module":389,"inlinee":22061}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::StructureType::is_fence
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x578E
// 首条位置: {"module":423,"inlinee":22414}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::new
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5791
// 首条位置: {"module":464,"inlinee":22417}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":464,"record_offset":44196,"parent_record_offset":44164,"ancestor_record_offset":44164,"inlinee":22333,"source_line":117,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":464,"record_offset":44384,"parent_record_offset":44344,"ancestor_record_offset":44344,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":464,"record_offset":44884,"parent_record_offset":44344,"ancestor_record_offset":44344,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":44636,"parent_record_offset":44612,"ancestor_record_offset":44344,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":44612,"parent_record_offset":44512,"ancestor_record_offset":44344,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":464,"record_offset":44544,"parent_record_offset":44512,"ancestor_record_offset":44344,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":464,"record_offset":44512,"parent_record_offset":44488,"ancestor_record_offset":44344,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":44576,"parent_record_offset":44544,"ancestor_record_offset":44344,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":44416,"parent_record_offset":44384,"ancestor_record_offset":44344,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":464,"record_offset":44908,"parent_record_offset":44884,"ancestor_record_offset":44344,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::height
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5793
// 首条位置: {"module":423,"inlinee":22419}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::is_premade_shape
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5795
// 首条位置: {"module":452,"inlinee":22421}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params
// 记录形式: compiled_procedure; instances=1
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":392,"record_offset":5212,"rva":5351952,"va":5374061072}
// 编译实例 RVAs: 0x51aa10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::is_enclosed
// 记录形式: inlinee_declaration; instances=1
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x573F
// 首条位置: {"module":392,"inlinee":22335}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5799
// 首条位置: {"module":474,"inlinee":22425}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1459,"record_offset":8116,"rva":17950384,"va":5386659504}
// 编译实例 RVAs: 0x111e6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16300461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386659504,"raw_hex":"e8adb9f800","size":5,"soft_fail":false,"target_va":5402960384,"va":5386659918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13910106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386659504,"raw_hex":"e8a6bf2bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386659813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16296830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386659504,"raw_hex":"e87eabf800","size":5,"soft_fail":false,"target_va":5402956592,"va":5386659757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1410b1fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-444260","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5386659504,"raw_hex":"e99c38f9ff","size":5,"soft_fail":false,"target_va":5386215392,"va":5386659647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1410c4ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-367749","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5386659504,"raw_hex":"e97b63faff","size":5,"soft_fail":false,"target_va":5386291888,"va":5386659632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1459,"record_offset":8308,"parent_record_offset":8116,"ancestor_record_offset":8116,"inlinee":16497,"source_line":212,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1459,"record_offset":8992,"parent_record_offset":8968,"ancestor_record_offset":8116,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1459,"record_offset":8968,"parent_record_offset":8904,"ancestor_record_offset":8116,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1459,"record_offset":8936,"parent_record_offset":8904,"ancestor_record_offset":8116,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1459,"record_offset":8904,"parent_record_offset":8876,"ancestor_record_offset":8116,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1459,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":8116,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1459,"record_offset":8820,"parent_record_offset":8388,"ancestor_record_offset":8116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1459,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":8116,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1459,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":8116,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1459,"record_offset":8476,"parent_record_offset":8432,"ancestor_record_offset":8116,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_bool
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1459,"record_offset":8360,"parent_record_offset":8308,"ancestor_record_offset":8116,"inlinee":67096,"source_line":200,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1459,"record_offset":8432,"parent_record_offset":8388,"ancestor_record_offset":8116,"inlinee":67097,"source_line":210,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1459,"record_offset":9360,"parent_record_offset":9272,"ancestor_record_offset":8116,"inlinee":9882,"source_line":1455,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1459,"record_offset":9304,"parent_record_offset":9272,"ancestor_record_offset":8116,"inlinee":9883,"source_line":1355,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1459,"record_offset":8388,"parent_record_offset":8308,"ancestor_record_offset":8116,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1459,"record_offset":9332,"parent_record_offset":9272,"ancestor_record_offset":8116,"inlinee":9885,"source_line":1417,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1459,"record_offset":9244,"parent_record_offset":8308,"ancestor_record_offset":8116,"inlinee":4320,"source_line":1619,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$8::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1459,"record_offset":9272,"parent_record_offset":8308,"ancestor_record_offset":8116,"inlinee":16500,"source_line":470,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x579B
// 首条位置: {"module":1459,"inlinee":22427}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_bool
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10618
// 首条位置: {"module":1459,"inlinee":67096}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10619
// 首条位置: {"module":1459,"inlinee":67097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1459,"record_offset":8476,"parent_record_offset":8432,"ancestor_record_offset":8432,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1459,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":8432,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1459,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":8432,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1459,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":8432,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01459.jsonl.gz"}
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
// 完整结构及原始类型记录: inner_wall_state.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::ArchType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$9::serialize_field<country_core::resources::walls::inner_wall_state::ArchType>", "constructor": "serde_json::value::ser::impl$9::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::ArchType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::ArchType>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::ArchType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::ArchType>", "constructor": "serde::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::ArchType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::StructureType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::StructureType>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::StructureType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::StructureType>", "constructor": "serde::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::StructureType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::WallState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$9::serialize_field<country_core::resources::walls::inner_wall_state::WallState>", "constructor": "serde_json::value::ser::impl$9::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::walls::inner_wall_state::WallState>", "constructor": "core::option::Option", "arguments": ["country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$9::serialize_field<enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde_json::value::ser::impl$9::serialize_field", "arguments": ["enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$79::from_iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "std::collections::hash::map::impl$79::from_iter", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "std::collections::hash::map::Drain", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::inner_wall_state::WallState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "std::collections::hash::map::Keys", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::WallId", "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::walls::inner_wall_state::WallState>", "constructor": "ref$", "arguments": ["country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "ref$<country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "std::collections::hash::map::Iter", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$0::all::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "country_core::resources::walls::inner_walls::impl$0::all::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$0::all::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$0::all::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIter::drop_elements<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "hashbrown::raw::RawIter::drop_elements", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "hashbrown::map::impl$94::extend", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::alloc::Global", "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::get_mut<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,country_core::resources::walls::WallId>", "constructor": "hashbrown::map::HashMap::get_mut", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::alloc::Global", "country_core::resources::walls::WallId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde_json::value::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "serde::de::impls::impl$188::deserialize::TupleVisitor", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_tuple<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "serde_json::value::de::impl$2::deserialize_tuple", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "ref$", "arguments": ["tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > > >", "constructor": "ref$", "arguments": ["ref$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<ref$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > > > >", "constructor": "ref$", "arguments": ["ref$<ref$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::to_value<ref$<ref$<ref$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > > > > >", "constructor": "serde_json::value::to_value", "arguments": ["ref$<ref$<ref$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >,alloc::alloc::Global> > >", "constructor": "serde::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::WallStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$9::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle>", "constructor": "serde_json::value::ser::impl$9::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::WallStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::WallStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>", "constructor": "serde_json::value::de::visit_array", "arguments": ["country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>", "constructor": "serde_json::value::de::visit_object", "arguments": ["country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>", "constructor": "serde::de::impl$1::fmt", "arguments": ["country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: inner_wall_state.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: inner_wall_state.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 48; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::resources::steam::SteamInput>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::resources::steam::SteamInput>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::steam::SteamInput>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::StructureType::is_fence
// 信息 ID: ed2960b3855df6fc8997c579
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=20576; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=68256; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=71540; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_edit_freehand_walls::ui_edit_freehand_walls; module=492, inline_record=41916; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute_quads; module=1253, inline_record=41956; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::signifier_audio::signifier_audio_system; module=1258, inline_record=40560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
// 信息 ID: e0fd394ed24b07355ddb79d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=68280; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=68372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params
// 信息 ID: ed2baeadaee467ab49d74971
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x502c6b -> 0x51aa10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::height
// 信息 ID: b86d86d374af4f8b97e4cbb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=22596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::is_enclosed
// 信息 ID: e7bbe352b262af8bde465bb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::is_premade_shape
// 信息 ID: 57a9f68780286c3c29add29b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl; module=452, inline_record=50640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=480, inline_record=110240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::new
// 信息 ID: de927d0fff06acbb238bac61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_walls::InnerWalls::new_wall_from_curve; module=464, inline_record=44164; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_walls::InnerWalls::new_wall_from_curve; module=464, inline_record=44256; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_walls::InnerWalls::new_wall_from_curve; module=464, inline_record=44344; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_walls::InnerWalls::new_wall_w_params; module=464, inline_record=45116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
// 信息 ID: 9005d4039f509276722c26a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze; module=409, inline_record=37152; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=22480; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=32432; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=38660; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=69552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered
// 信息 ID: f8535a5ffe36921722ec218e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 34 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea844 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea8d8 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4eac38 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea593 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x4fb955 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x4fbb22 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x4fbc0e -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501c8f -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501d60 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x502019 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x502160 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::should_snap_to_corner @ 0x502a25 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0xc9e4a4 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9f3f2 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca024d -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca0b42 -> 0x51aa00 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=452, inline_record=12772; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
// 信息 ID: e1adfd2e31e14f3a7b0049b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >; module=1440, inline_record=39904; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=378276; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=444064; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=444288; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=449832; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=450056; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=455088; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=495324; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 6a0815554a5c2c86a44af9a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1440, inline_record=341936; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1440, inline_record=343276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: f78eeb98c536e3e3a18c0015
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> > @ 0x10d8e92 -> 0x1123f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor> @ 0x10c3ab5 -> 0x11242f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 73a1c96b9d707afd00f0b71b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=168176; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1440, inline_record=340764; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1440, inline_record=341784; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1440, inline_record=343124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 93c6f01c3451451d65d80414
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1459, inline_record=440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: ec4b62719e5653c17dabdce5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1459, inline_record=34048; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1459, inline_record=40128; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1459, inline_record=47212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 0e9e636d25b715dadc212d23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=166676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 信息 ID: 125c3ac51c04225e6f5735d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=169272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 信息 ID: 00a55e27dfe918c65e579a6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=171816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 0cc60f94b392fcf706d36389
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=109060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 信息 ID: fbebf6d8394da9fc25a45840
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=109292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 信息 ID: 420d490064f5b4451b04f447
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::__DeserializeWith>; module=1440, inline_record=320288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 1745ca581c07d7ef08585977
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::ArchType> @ 0x10d720d -> 0x1122e90 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e74c2 -> 0x1122e90 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e756a -> 0x1122e90 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e6822 -> 0x11236f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e68ca -> 0x11236f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::StructureType> @ 0x10d728d -> 0x11244c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize
// 信息 ID: 47e1feb888391278b8d9f378
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$9::serialize_field<country_core::resources::walls::inner_wall_state::ArchType>; module=100, inline_record=7060; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::ArchType>; module=100, inline_record=190136; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::StructureType>; module=100, inline_record=216824; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$9::serialize_field<country_core::resources::walls::inner_wall_state::ArchType>; module=1432, inline_record=26168; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::ArchType>; module=1432, inline_record=132684; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::StructureType>; module=1432, inline_record=260680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize::impl$0::serialize
// 信息 ID: 88bdf5a97dc1f55aabedf273
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::_::impl$0::serialize::__SerializeWith>; module=100, inline_record=172116; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::_::impl$0::serialize::__SerializeWith>; module=1432, inline_record=251480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 53144cae8b2ad65e647fcf73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$9::serialize_field<country_core::resources::walls::inner_wall_state::WallState> @ 0x147e87 -> 0x1a4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$9::serialize_field<enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > > @ 0x14a1ec -> 0x1a4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$9::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle> @ 0x1496c7 -> 0x1a50e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle> @ 0x155d1b -> 0x1a50e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$9::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle> @ 0x108def7 -> 0x1a50e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle> @ 0x109ad5b -> 0x1a50e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::to_value<ref$<ref$<ref$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > > > > > @ 0x1019df1 -> 0x1124030 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$9::serialize_field<country_core::resources::walls::inner_wall_state::WallState> @ 0x108c407 -> 0x1124030 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$9::serialize_field<enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > > @ 0x108c70c -> 0x1124030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::expecting
// 信息 ID: f9104d04f343d0e8a0fc6fb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>; module=1459, inline_record=888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_bool
// 信息 ID: eb0f18c6a9acf2314c9e10c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >; module=1459, inline_record=8360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_str
// 信息 ID: 3a01e7df3d7185dc47e0d07a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >; module=1459, inline_record=8432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >
// 信息 ID: 9a04dbf345729eb3b13b4801
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::inner_wall_state::deserialize_wall_style -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::__DeserializeWith> @ 0x10d7310 -> 0x111e6b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$10::fmt
// 信息 ID: c22727484a9cf93ee71f92c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$12::eq
// 信息 ID: cc313d2d46e372254cfbecb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3467, inline_record=7432; 不是独立调用边。
// [PDB 内联宿主] system_color::radial_menu::ColorRadialMenu::create; module=4255, inline_record=5224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$14::clone
// 信息 ID: 1334449e29bb8d86c76eaaeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 29 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0x681bcb -> 0x6834d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=17300; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=29708; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::wall_deleted; module=435, inline_record=9336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall; module=435, inline_record=15764; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=32516; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=35960; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=38692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=43088; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$21::eq
// 信息 ID: 5c585144bdc77afe51610711
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=389, inline_record=41252; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=389, inline_record=45328; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall::closure$0; module=480, inline_record=112300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$6::fmt
// 信息 ID: 81ec9b92e3418b012bea116b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
