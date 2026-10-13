// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/render/nani/cull_shaders.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: fc29aafbe55b340650342c53e54d8ce34aa6b82157a009d00c0dd3d29e850a29
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cull_shaders.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 518, 540, 597
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$0::nani_cull_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":34928,"rva":10956656,"va":5379665776}
// 编译实例 RVAs: 0xa72f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ae90
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1278306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665776,"raw_hex":"e89e7eecff","size":5,"soft_fail":false,"target_va":5378387600,"va":5379665901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13622831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665776,"raw_hex":"e82fdecf00","size":5,"soft_fail":false,"target_va":5393288768,"va":5379665932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177bb90
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13667034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665776,"raw_hex":"e8da8ad000","size":5,"soft_fail":false,"target_va":5393333136,"va":5379666097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3873182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665776,"raw_hex":"e89e193b00","size":5,"soft_fail":false,"target_va":5383539152,"va":5379665965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":540,"record_offset":35080,"parent_record_offset":34928,"ancestor_record_offset":34928,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$1::nani_cull_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":34144,"rva":10955248,"va":5379664368}
// 编译实例 RVAs: 0xa729f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ae90
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1276898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664368,"raw_hex":"e81e84ecff","size":5,"soft_fail":false,"target_va":5378387600,"va":5379664493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13624239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664368,"raw_hex":"e8afe3cf00","size":5,"soft_fail":false,"target_va":5393288768,"va":5379664524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177bb90
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13668442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664368,"raw_hex":"e85a90d000","size":5,"soft_fail":false,"target_va":5393333136,"va":5379664689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3874590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664368,"raw_hex":"e81e1f3b00","size":5,"soft_fail":false,"target_va":5383539152,"va":5379664557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":540,"record_offset":34296,"parent_record_offset":34144,"ancestor_record_offset":34144,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$2::nani_cull_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":35124,"rva":10957008,"va":5379666128}
// 编译实例 RVAs: 0xa730d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ae90
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1278658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666128,"raw_hex":"e83e7decff","size":5,"soft_fail":false,"target_va":5378387600,"va":5379666253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13622479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666128,"raw_hex":"e8cfdccf00","size":5,"soft_fail":false,"target_va":5393288768,"va":5379666284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177bb90
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13666682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666128,"raw_hex":"e87a89d000","size":5,"soft_fail":false,"target_va":5393333136,"va":5379666449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3872830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666128,"raw_hex":"e83e183b00","size":5,"soft_fail":false,"target_va":5383539152,"va":5379666317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":540,"record_offset":35276,"parent_record_offset":35124,"ancestor_record_offset":35124,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$3::nani_cull_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":34732,"rva":10956304,"va":5379665424}
// 编译实例 RVAs: 0xa72e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ae90
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1277954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665424,"raw_hex":"e8fe7fecff","size":5,"soft_fail":false,"target_va":5378387600,"va":5379665549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13623183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665424,"raw_hex":"e88fdfcf00","size":5,"soft_fail":false,"target_va":5393288768,"va":5379665580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177bb90
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13667386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665424,"raw_hex":"e83a8cd000","size":5,"soft_fail":false,"target_va":5393333136,"va":5379665745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3873534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665424,"raw_hex":"e8fe1a3b00","size":5,"soft_fail":false,"target_va":5383539152,"va":5379665613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":540,"record_offset":34884,"parent_record_offset":34732,"ancestor_record_offset":34732,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$4::nani_cull_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":34340,"rva":10955600,"va":5379664720}
// 编译实例 RVAs: 0xa72b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ae90
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1277250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664720,"raw_hex":"e8be82ecff","size":5,"soft_fail":false,"target_va":5378387600,"va":5379664845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13623887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664720,"raw_hex":"e84fe2cf00","size":5,"soft_fail":false,"target_va":5393288768,"va":5379664876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177bb90
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13668090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664720,"raw_hex":"e8fa8ed000","size":5,"soft_fail":false,"target_va":5393333136,"va":5379665041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3874238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664720,"raw_hex":"e8be1d3b00","size":5,"soft_fail":false,"target_va":5383539152,"va":5379664909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":540,"record_offset":34492,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$5::nani_cull_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":33948,"rva":10954896,"va":5379664016}
// 编译实例 RVAs: 0xa72890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ae90
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1276546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664016,"raw_hex":"e87e85ecff","size":5,"soft_fail":false,"target_va":5378387600,"va":5379664141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13624591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664016,"raw_hex":"e80fe5cf00","size":5,"soft_fail":false,"target_va":5393288768,"va":5379664172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177bb90
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13668794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664016,"raw_hex":"e8ba91d000","size":5,"soft_fail":false,"target_va":5393333136,"va":5379664337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3874942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379664016,"raw_hex":"e87e203b00","size":5,"soft_fail":false,"target_va":5383539152,"va":5379664205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":540,"record_offset":34100,"parent_record_offset":33948,"ancestor_record_offset":33948,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$6::nani_cull_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":34536,"rva":10955952,"va":5379665072}
// 编译实例 RVAs: 0xa72cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ae90
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1277602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665072,"raw_hex":"e85e81ecff","size":5,"soft_fail":false,"target_va":5378387600,"va":5379665197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13623535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665072,"raw_hex":"e8efe0cf00","size":5,"soft_fail":false,"target_va":5393288768,"va":5379665228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177bb90
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: cull_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13667738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665072,"raw_hex":"e89a8dd000","size":5,"soft_fail":false,"target_va":5393333136,"va":5379665393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3873886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379665072,"raw_hex":"e85e1c3b00","size":5,"soft_fail":false,"target_va":5383539152,"va":5379665261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":540,"record_offset":34688,"parent_record_offset":34536,"ancestor_record_offset":34536,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$10::nani_cull_shader
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76A3
// 首条位置: {"module":518,"inlinee":30371}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::shader::nani::cull_and_bucket_grass2_lod0::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":518,"record_offset":30800,"parent_record_offset":30764,"ancestor_record_offset":30764,"inlinee":30373,"source_line":21335,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":518,"record_offset":30828,"parent_record_offset":30764,"ancestor_record_offset":30764,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$7::nani_cull_shader
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x823F
// 首条位置: {"module":597,"inlinee":33343}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::shader::nani::cull_and_bucket_roof_tile::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":597,"record_offset":6940,"parent_record_offset":6868,"ancestor_record_offset":6868,"inlinee":33291,"source_line":21775,"archive":"_pdb/C/modules/00597.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":597,"record_offset":6968,"parent_record_offset":6868,"ancestor_record_offset":6868,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00597.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$8::nani_cull_shader
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8245
// 首条位置: {"module":597,"inlinee":33349}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::shader::nani::cull_and_bucket_wooden_plank::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":597,"record_offset":5640,"parent_record_offset":5568,"ancestor_record_offset":5568,"inlinee":33293,"source_line":23240,"archive":"_pdb/C/modules/00597.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":597,"record_offset":5668,"parent_record_offset":5568,"ancestor_record_offset":5568,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00597.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$9::nani_cull_shader
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x824B
// 首条位置: {"module":597,"inlinee":33355}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::shader::nani::cull_and_bucket_wooden_gable_plank::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":597,"record_offset":8264,"parent_record_offset":8192,"ancestor_record_offset":8192,"inlinee":33295,"source_line":22995,"archive":"_pdb/C/modules/00597.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":597,"record_offset":8292,"parent_record_offset":8192,"ancestor_record_offset":8192,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00597.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$11::nani_cull_shader
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76B3
// 首条位置: {"module":518,"inlinee":30387}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":518,"record_offset":31908,"parent_record_offset":31876,"ancestor_record_offset":31876,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$12::nani_cull_shader
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76B0
// 首条位置: {"module":518,"inlinee":30384}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":518,"record_offset":31536,"parent_record_offset":31504,"ancestor_record_offset":31504,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00518.jsonl.gz"}
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
// 完整结构及原始类型记录: cull_shaders.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketCurvedHalftimber
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketCurvedHalftimber>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0", "arguments": ["country_core::render::nani::instance_data::CurvedHalfTimberInstanceData", "country_core::render::nani::cull_shaders::NaniCullAndBucketCurvedHalftimber"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketCurvedHalftimber>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketCurvedHalftimber>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketPlaster
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::PlasterInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketPlaster>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0", "arguments": ["country_core::render::nani::instance_data::PlasterInstanceData", "country_core::render::nani::cull_shaders::NaniCullAndBucketPlaster"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::PlasterInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketPlaster>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::PlasterInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketPlaster>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile>", "constructor": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external", "arguments": ["country_core::resources::roof_tile_ssbo::RoofTileSsbo", "country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile>", "constructor": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0", "arguments": ["country_core::resources::roof_tile_ssbo::RoofTileSsbo", "country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_with<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile> >", "constructor": "rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_with", "arguments": ["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketSmoke
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::SmokeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketSmoke>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0", "arguments": ["country_core::render::nani::instance_data::SmokeInstanceData", "country_core::render::nani::cull_shaders::NaniCullAndBucketSmoke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::SmokeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketSmoke>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::SmokeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketSmoke>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketStairRope
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::StairRopeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStairRope>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0", "arguments": ["country_core::render::nani::instance_data::StairRopeInstanceData", "country_core::render::nani::cull_shaders::NaniCullAndBucketStairRope"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::StairRopeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStairRope>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::StairRopeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStairRope>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketStairsFloor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::StairsFloorInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStairsFloor>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0", "arguments": ["country_core::render::nani::instance_data::StairsFloorInstanceData", "country_core::render::nani::cull_shaders::NaniCullAndBucketStairsFloor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::StairsFloorInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStairsFloor>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::StairsFloorInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStairsFloor>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketStoneFoundationSurface
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::shader::inc::stone_foundation_surface::StoneFoundationSurfaceInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStoneFoundationSurface>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0", "arguments": ["country_core::shader::inc::stone_foundation_surface::StoneFoundationSurfaceInstanceData", "country_core::render::nani::cull_shaders::NaniCullAndBucketStoneFoundationSurface"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::shader::inc::stone_foundation_surface::StoneFoundationSurfaceInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStoneFoundationSurface>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::shader::inc::stone_foundation_surface::StoneFoundationSurfaceInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStoneFoundationSurface>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenArchFrame
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::WoodenArchFrameInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenArchFrame>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0", "arguments": ["country_core::render::nani::instance_data::WoodenArchFrameInstanceData", "country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenArchFrame"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::WoodenArchFrameInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenArchFrame>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::WoodenArchFrameInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenArchFrame>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank>", "constructor": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external", "arguments": ["country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo", "country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank>", "constructor": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0", "arguments": ["country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo", "country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank>", "tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_with<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank> >", "constructor": "rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_with", "arguments": ["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank>", "constructor": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external", "arguments": ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo", "country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank>", "constructor": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0", "arguments": ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo", "country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_with<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank> >", "constructor": "rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_with", "arguments": ["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external::closure$0::closure_env$0<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cull_shaders.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cull_shaders.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$0::nani_cull_shader
// 信息 ID: a75d92bc389be7dccb70ec2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$10::nani_cull_shader
// 信息 ID: 162306cc259fdace8cf6c7dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=518, inline_record=30764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$11::nani_cull_shader
// 信息 ID: cebc2b5cf499618c234d4042
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=518, inline_record=31876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$12::nani_cull_shader
// 信息 ID: 28e29c4a92c7df786f6fe326
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=518, inline_record=31504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$1::nani_cull_shader
// 信息 ID: f30b130d84d05b16595b36c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$2::nani_cull_shader
// 信息 ID: f067dbfbfaaa151d609fbca4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$3::nani_cull_shader
// 信息 ID: f2153dd8eb15e1ed33f4e954
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$4::nani_cull_shader
// 信息 ID: 2654bed4a78d81ef74af5306
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$5::nani_cull_shader
// 信息 ID: 049ea44d6f9c7c05366a0d86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$6::nani_cull_shader
// 信息 ID: e0b69f889ca79e7d61ea44fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$7::nani_cull_shader
// 信息 ID: c30b3218def0c929f06657d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile>; module=597, inline_record=6868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$8::nani_cull_shader
// 信息 ID: 34597791a77f8f1eaa2caf93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank>; module=597, inline_record=5568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$9::nani_cull_shader
// 信息 ID: 5113792dcdc3520c4659b19e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank>; module=597, inline_record=8192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
