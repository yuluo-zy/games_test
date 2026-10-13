// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/asset/src/texture/blob.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 483b4b06f9907f09d6fe060320b7e6a2b3cfd5c4b0409214c7f5995e911e4b82
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: blob.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2828, 3923
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use asset; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rkyv; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::texture::blob::_::impl$0::resolve
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 5, 196, 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1607E
// 首条位置: {"module":2828,"inlinee":90238}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::blob::_::impl$0::resolve
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":2828,"record_offset":5760,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":90238,"source_line":196,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2828,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":5760,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$4::resolve
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/impls/alloc/vec.rs
//     evidence={"module":2828,"record_offset":5928,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":90244,"source_line":45,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::core::impl$4::resolve
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/impls/core/mod.rs
//     evidence={"module":2828,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":90240,"source_line":133,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::core::primitive::_::impl$0::resolve
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/impls/core/primitive.rs
//     evidence={"module":2828,"record_offset":5676,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":90242,"source_line":60,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":6164,"parent_record_offset":6124,"ancestor_record_offset":5536,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2828,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":5536,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":5536,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":6296,"parent_record_offset":6020,"ancestor_record_offset":5536,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":6204,"parent_record_offset":6072,"ancestor_record_offset":5536,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::primitive::_::impl$0::resolve
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/impls/core/primitive.rs
//     evidence={"module":2828,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":5536,"inlinee":90242,"source_line":60,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::primitive::impl$6::resolve
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/impls/core/primitive.rs
//     evidence={"module":2828,"record_offset":6328,"parent_record_offset":5984,"ancestor_record_offset":5536,"inlinee":90254,"source_line":238,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::try_emplace
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":5536,"inlinee":90249,"source_line":167,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::emplace
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":6020,"parent_record_offset":5984,"ancestor_record_offset":5536,"inlinee":90247,"source_line":299,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::try_emplace
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":6048,"parent_record_offset":6020,"ancestor_record_offset":5536,"inlinee":90248,"source_line":281,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::impl$18::between
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":6100,"parent_record_offset":6072,"ancestor_record_offset":5536,"inlinee":90250,"source_line":100,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::signed_offset
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":5536,"inlinee":90252,"source_line":58,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::resolve_from_len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":5984,"parent_record_offset":5956,"ancestor_record_offset":5536,"inlinee":90246,"source_line":97,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::resolve_from_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":5956,"parent_record_offset":5928,"ancestor_record_offset":5536,"inlinee":90245,"source_line":81,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::blob::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160A8
// 首条位置: {"module":2828,"inlinee":90280}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::blob::impl$5::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160D7
// 首条位置: {"module":2828,"record_offset":9344,"rva":25225552,"va":5393934672}
// 编译实例 RVAs: 0x180e950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=185
//     target_aliases=2 candidates; complete list: blob.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-16350795","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393934672,"raw_hex":"e9b58106ff","size":5,"soft_fail":false,"target_va":5377583920,"va":5393934710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::blob::impl$11::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160D5
// 首条位置: {"module":2828,"record_offset":9232,"rva":25225504,"va":5393934624}
// 编译实例 RVAs: 0x180e920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: blob.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-16350743","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393934624,"raw_hex":"e9e98106ff","size":5,"soft_fail":false,"target_va":5377583920,"va":5393934658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::blob::impl$9::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160AA
// 首条位置: {"module":2828,"inlinee":90282}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::blob::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1606C
// 首条位置: {"module":2828,"inlinee":90220}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$5::serialize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/impls/alloc/vec.rs
//     evidence={"module":2828,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":4116,"inlinee":90222,"source_line":52,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":4200,"parent_record_offset":4172,"ancestor_record_offset":4116,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::serialize_from_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":4172,"parent_record_offset":4144,"ancestor_record_offset":4116,"inlinee":90223,"source_line":106,"archive":"_pdb/A/modules/02828.jsonl.gz"}
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
// 完整结构及原始类型记录: blob.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::blob::TextureAssetBlob
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "rkyv::util::to_bytes<asset::texture::blob::TextureAssetBlob,256>", "constructor": "rkyv::util::to_bytes", "arguments": ["asset::texture::blob::TextureAssetBlob", "256"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::blob::TextureAssetFormat
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$52::fmt<asset::texture::blob::TextureAssetFormat>", "constructor": "core::fmt::impl$52::fmt", "arguments": ["asset::texture::blob::TextureAssetFormat"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::blob::TextureAssetType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$52::fmt<asset::texture::blob::TextureAssetType>", "constructor": "core::fmt::impl$52::fmt", "arguments": ["asset::texture::blob::TextureAssetType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: blob.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: blob.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::texture::blob::_::impl$0::resolve
// 信息 ID: e6a9be62163ead78a19d5671
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::util::to_bytes<asset::texture::blob::TextureAssetBlob,256>; module=2828, inline_record=5536; 不是独立调用边。
// [PDB 内联宿主] rkyv::util::to_bytes<asset::texture::blob::TextureAssetBlob,256>; module=2828, inline_record=5760; 不是独立调用边。
// [PDB 内联宿主] rkyv::util::to_bytes<asset::texture::blob::TextureAssetBlob,256>; module=2828, inline_record=5844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::blob::_::impl$0::serialize
// 信息 ID: 5a8b9a1334c6ac2c82a447b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::util::to_bytes<asset::texture::blob::TextureAssetBlob,256>; module=2828, inline_record=4116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::blob::impl$11::fmt
// 信息 ID: 3825e6b1829d67c1af04e93b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$52::fmt<asset::texture::blob::TextureAssetType>; module=3923, inline_record=396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::blob::impl$3::eq
// 信息 ID: ab6c47ff7787d4165bb69ec5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=28240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::blob::impl$5::fmt
// 信息 ID: e7c539d0ee5e2e419881d68f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$52::fmt<asset::texture::blob::TextureAssetFormat>; module=3923, inline_record=576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::blob::impl$9::eq
// 信息 ID: 9baf3c4f9f0a393c29e4fa3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=28296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
