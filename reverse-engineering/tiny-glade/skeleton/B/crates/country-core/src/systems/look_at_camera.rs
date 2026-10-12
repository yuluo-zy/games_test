// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/look_at_camera.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 66aadbb142cec4bc2e57d4de5f9e9f7cf9166b0c3a5bf009f2d988c398306b54
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: look_at_camera.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 484
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::look_at_camera::impl$0::clone_behavior
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":484,"record_offset":65080,"rva":9072,"va":5368718192}
// 编译实例 RVAs: 0x2370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::look_at_camera::impl$0::register_required_components
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":484,"record_offset":58820,"rva":8869600,"va":5377578720}
// 编译实例 RVAs: 0x8756e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140051340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: look_at_camera.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8538248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e878b77dff","size":5,"soft_fail":false,"target_va":5369041728,"va":5377579971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: look_at_camera.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2641677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e8f3b0d7ff","size":5,"soft_fail":false,"target_va":5374937936,"va":5377579608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: look_at_camera.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2498539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e815e0d9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377580038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408756e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: look_at_camera.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e861fbffff","size":5,"soft_fail":false,"target_va":5377578720,"va":5377579898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f4340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: look_at_camera.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11004195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e823e9a700","size":5,"soft_fail":false,"target_va":5388583744,"va":5377579544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: look_at_camera.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34267574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e8b6e10a02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377579589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3195340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e8ccc13000","size":5,"soft_fail":false,"target_va":5380775408,"va":5377580063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::ComponentInfo::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5021299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e8739e4c00","size":5,"soft_fail":false,"target_va":5382600816,"va":5377579512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::QueuedRegistration::register
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5021084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e89c9d4c00","size":5,"soft_fail":false,"target_va":5382601280,"va":5377580191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::enforce_no_required_components_recursion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5027387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e83bb64c00","size":5,"soft_fail":false,"target_va":5382606192,"va":5377578800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::reserve_rehash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33931869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377578720,"raw_hex":"e85dc20502","size":5,"soft_fail":false,"target_va":5411512256,"va":5377580382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":65040,"parent_record_offset":58820,"ancestor_record_offset":58820,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":64844,"parent_record_offset":58820,"ancestor_record_offset":58820,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::component::ComponentsRegistrator::register_component
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":484,"record_offset":58988,"parent_record_offset":58820,"ancestor_record_offset":58820,"inlinee":5611,"source_line":1715,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":64556,"parent_record_offset":64532,"ancestor_record_offset":58820,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":64532,"parent_record_offset":64440,"ancestor_record_offset":58820,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":63976,"parent_record_offset":63952,"ancestor_record_offset":58820,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":64468,"parent_record_offset":64440,"ancestor_record_offset":58820,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":64440,"parent_record_offset":64416,"ancestor_record_offset":58820,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":64024,"parent_record_offset":64000,"ancestor_record_offset":58820,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":64000,"parent_record_offset":63976,"ancestor_record_offset":58820,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":64416,"parent_record_offset":64392,"ancestor_record_offset":58820,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":63952,"parent_record_offset":63928,"ancestor_record_offset":58820,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":63928,"parent_record_offset":63904,"ancestor_record_offset":58820,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":59032,"parent_record_offset":58988,"ancestor_record_offset":58820,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":63904,"parent_record_offset":61900,"ancestor_record_offset":58820,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentDescriptor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":484,"record_offset":61968,"parent_record_offset":61900,"ancestor_record_offset":58820,"inlinee":5647,"source_line":1101,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentIds::next_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":484,"record_offset":61868,"parent_record_offset":59060,"ancestor_record_offset":58820,"inlinee":5644,"source_line":1334,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::register_component_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":484,"record_offset":61996,"parent_record_offset":61900,"ancestor_record_offset":58820,"inlinee":5648,"source_line":1972,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_checked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":484,"record_offset":59060,"parent_record_offset":58988,"ancestor_record_offset":58820,"inlinee":5612,"source_line":1721,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":484,"record_offset":61900,"parent_record_offset":59060,"ancestor_record_offset":58820,"inlinee":5645,"source_line":1755,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$17::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":484,"record_offset":62132,"parent_record_offset":61900,"ancestor_record_offset":58820,"inlinee":13108,"source_line":1614,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$90::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":484,"record_offset":63876,"parent_record_offset":61900,"ancestor_record_offset":58820,"inlinee":5662,"source_line":2812,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":484,"record_offset":59128,"parent_record_offset":59060,"ancestor_record_offset":58820,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":484,"record_offset":60168,"parent_record_offset":59060,"ancestor_record_offset":58820,"inlinee":5624,"source_line":1141,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":484,"record_offset":59908,"parent_record_offset":59884,"ancestor_record_offset":58820,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":484,"record_offset":59956,"parent_record_offset":59932,"ancestor_record_offset":58820,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_cmpeq_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":484,"record_offset":61428,"parent_record_offset":61400,"ancestor_record_offset":58820,"inlinee":5636,"source_line":842,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":484,"record_offset":63316,"parent_record_offset":63292,"ancestor_record_offset":58820,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":484,"record_offset":59356,"parent_record_offset":59332,"ancestor_record_offset":58820,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":484,"record_offset":59440,"parent_record_offset":59416,"ancestor_record_offset":58820,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":484,"record_offset":59380,"parent_record_offset":59356,"ancestor_record_offset":58820,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":484,"record_offset":64500,"parent_record_offset":64468,"ancestor_record_offset":58820,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":484,"record_offset":61288,"parent_record_offset":61240,"ancestor_record_offset":58820,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":484,"record_offset":61664,"parent_record_offset":61636,"ancestor_record_offset":58820,"inlinee":5639,"source_line":135,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":484,"record_offset":61692,"parent_record_offset":61636,"ancestor_record_offset":58820,"inlinee":5640,"source_line":161,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":484,"record_offset":59592,"parent_record_offset":59564,"ancestor_record_offset":58820,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":484,"record_offset":59564,"parent_record_offset":59536,"ancestor_record_offset":58820,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":484,"record_offset":62692,"parent_record_offset":62328,"ancestor_record_offset":58820,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":484,"record_offset":62716,"parent_record_offset":62692,"ancestor_record_offset":58820,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":484,"record_offset":61200,"parent_record_offset":61176,"ancestor_record_offset":58820,"inlinee":5634,"source_line":682,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":62100,"parent_record_offset":61996,"ancestor_record_offset":58820,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":61788,"parent_record_offset":61764,"ancestor_record_offset":58820,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":59824,"parent_record_offset":59800,"ancestor_record_offset":58820,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":63804,"parent_record_offset":63780,"ancestor_record_offset":58820,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":61812,"parent_record_offset":61788,"ancestor_record_offset":58820,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":63828,"parent_record_offset":63804,"ancestor_record_offset":58820,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":484,"record_offset":62044,"parent_record_offset":61996,"ancestor_record_offset":58820,"inlinee":5649,"source_line":619,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":62068,"parent_record_offset":62044,"ancestor_record_offset":58820,"inlinee":5650,"source_line":232,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":484,"record_offset":60100,"parent_record_offset":60076,"ancestor_record_offset":58820,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":484,"record_offset":60124,"parent_record_offset":60100,"ancestor_record_offset":58820,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":484,"record_offset":59932,"parent_record_offset":59908,"ancestor_record_offset":58820,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":484,"record_offset":59884,"parent_record_offset":59860,"ancestor_record_offset":58820,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::leading_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":484,"record_offset":61636,"parent_record_offset":61240,"ancestor_record_offset":58820,"inlinee":5637,"source_line":86,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":484,"record_offset":59500,"parent_record_offset":59472,"ancestor_record_offset":58820,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":484,"record_offset":59536,"parent_record_offset":59500,"ancestor_record_offset":58820,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":484,"record_offset":59472,"parent_record_offset":59252,"ancestor_record_offset":58820,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":484,"record_offset":59332,"parent_record_offset":59252,"ancestor_record_offset":58820,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":484,"record_offset":63292,"parent_record_offset":63232,"ancestor_record_offset":58820,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":484,"record_offset":59636,"parent_record_offset":59252,"ancestor_record_offset":58820,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":484,"record_offset":62780,"parent_record_offset":62752,"ancestor_record_offset":58820,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":484,"record_offset":59416,"parent_record_offset":59252,"ancestor_record_offset":58820,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/tag.rs
//     evidence={"module":484,"record_offset":63568,"parent_record_offset":63536,"ancestor_record_offset":58820,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::find_or_find_insert_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":484,"record_offset":62216,"parent_record_offset":62172,"ancestor_record_offset":58820,"inlinee":5653,"source_line":1804,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":484,"record_offset":59152,"parent_record_offset":59128,"ancestor_record_offset":58820,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":484,"record_offset":59176,"parent_record_offset":59152,"ancestor_record_offset":58820,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":484,"record_offset":62172,"parent_record_offset":62132,"ancestor_record_offset":58820,"inlinee":5652,"source_line":1790,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":484,"record_offset":60200,"parent_record_offset":60168,"ancestor_record_offset":58820,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":484,"record_offset":60232,"parent_record_offset":60200,"ancestor_record_offset":58820,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":484,"record_offset":59860,"parent_record_offset":59752,"ancestor_record_offset":58820,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":59800,"parent_record_offset":59776,"ancestor_record_offset":58820,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":61764,"parent_record_offset":61072,"ancestor_record_offset":58820,"inlinee":5642,"source_line":488,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::to_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":61152,"parent_record_offset":61128,"ancestor_record_offset":58820,"inlinee":5631,"source_line":334,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":63780,"parent_record_offset":63504,"ancestor_record_offset":58820,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":59720,"parent_record_offset":59252,"ancestor_record_offset":58820,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":59776,"parent_record_offset":59752,"ancestor_record_offset":58820,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":61128,"parent_record_offset":61100,"ancestor_record_offset":58820,"inlinee":5630,"source_line":705,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase_no_drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":61100,"parent_record_offset":61072,"ancestor_record_offset":58820,"inlinee":5629,"source_line":766,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":59228,"parent_record_offset":59204,"ancestor_record_offset":58820,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find_or_find_insert_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":62252,"parent_record_offset":62216,"ancestor_record_offset":58820,"inlinee":5654,"source_line":1109,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":59204,"parent_record_offset":59176,"ancestor_record_offset":58820,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_in_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":63504,"parent_record_offset":62172,"ancestor_record_offset":58820,"inlinee":5665,"source_line":1145,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":61072,"parent_record_offset":60256,"ancestor_record_offset":58820,"inlinee":5628,"source_line":785,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":60256,"parent_record_offset":60232,"ancestor_record_offset":58820,"inlinee":5627,"source_line":797,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":62292,"parent_record_offset":62252,"ancestor_record_offset":58820,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::erase
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":61240,"parent_record_offset":61100,"ancestor_record_offset":58820,"inlinee":5635,"source_line":3062,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":59252,"parent_record_offset":59228,"ancestor_record_offset":58820,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":62752,"parent_record_offset":62328,"ancestor_record_offset":58820,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_or_find_insert_slot_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":62328,"parent_record_offset":62252,"ancestor_record_offset":58820,"inlinee":5656,"source_line":1648,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":63232,"parent_record_offset":62328,"ancestor_record_offset":58820,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":63264,"parent_record_offset":63232,"ancestor_record_offset":58820,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":59304,"parent_record_offset":59252,"ancestor_record_offset":58820,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::record_item_insert_at
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":63536,"parent_record_offset":63504,"ancestor_record_offset":58820,"inlinee":5666,"source_line":2311,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":61724,"parent_record_offset":61240,"ancestor_record_offset":58820,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":63604,"parent_record_offset":63536,"ancestor_record_offset":58820,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":64168,"parent_record_offset":64144,"ancestor_record_offset":58820,"inlinee":4995,"source_line":3323,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":59752,"parent_record_offset":59252,"ancestor_record_offset":58820,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find_or_find_insert_slot::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":62964,"parent_record_offset":62328,"ancestor_record_offset":58820,"inlinee":5660,"source_line":1127,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::offset_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":484,"record_offset":61176,"parent_record_offset":61152,"ancestor_record_offset":58820,"inlinee":5633,"source_line":18,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":484,"record_offset":60052,"parent_record_offset":60028,"ancestor_record_offset":58820,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":484,"record_offset":60076,"parent_record_offset":60052,"ancestor_record_offset":58820,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":484,"record_offset":60028,"parent_record_offset":59060,"ancestor_record_offset":58820,"inlinee":5618,"source_line":631,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::look_at_camera::look_at_camera
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":484,"record_offset":49828,"rva":8863200,"va":5377572320}
// 编译实例 RVAs: 0x873de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: look_at_camera.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34096446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e83e450802","size":5,"soft_fail":false,"target_va":5411669968,"va":5377573517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33968004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e8844f0602","size":5,"soft_fail":false,"target_va":5411541584,"va":5377573575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e825e3ffff","size":5,"soft_fail":false,"target_va":5377565008,"va":5377572390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33970895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e8cf5a0602","size":5,"soft_fail":false,"target_va":5411543360,"va":5377572460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3812618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e80a2d3a00","size":5,"soft_fail":false,"target_va":5381385536,"va":5377572913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3817804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e84c413a00","size":5,"soft_fail":false,"target_va":5381390640,"va":5377572831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3812997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e8852e3a00","size":5,"soft_fail":false,"target_va":5381386480,"va":5377573478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3812338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e8f22b3a00","size":5,"soft_fail":false,"target_va":5381384704,"va":5377572361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11251581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e87dafab00","size":5,"soft_fail":false,"target_va":5388825296,"va":5377573710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33963536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e8103e0602","size":5,"soft_fail":false,"target_va":5411537104,"va":5377573563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4336453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e8452b4200","size":5,"soft_fail":false,"target_va":5381909824,"va":5377573366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::components::transform::Transform::look_at
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4361185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377572320,"raw_hex":"e8e18b4200","size":5,"soft_fail":false,"target_va":5381934592,"va":5377573402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":484,"record_offset":55140,"parent_record_offset":49828,"ancestor_record_offset":49828,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":484,"record_offset":52800,"parent_record_offset":49828,"ancestor_record_offset":49828,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":484,"record_offset":52392,"parent_record_offset":49828,"ancestor_record_offset":49828,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":55196,"parent_record_offset":49828,"ancestor_record_offset":49828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":484,"record_offset":51424,"parent_record_offset":49828,"ancestor_record_offset":49828,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":484,"record_offset":49992,"parent_record_offset":49828,"ancestor_record_offset":49828,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":484,"record_offset":50448,"parent_record_offset":49828,"ancestor_record_offset":49828,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":52632,"parent_record_offset":52608,"ancestor_record_offset":49828,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":52680,"parent_record_offset":52656,"ancestor_record_offset":49828,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":52656,"parent_record_offset":52632,"ancestor_record_offset":49828,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":52608,"parent_record_offset":52584,"ancestor_record_offset":49828,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":52584,"parent_record_offset":52552,"ancestor_record_offset":49828,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":53104,"parent_record_offset":53080,"ancestor_record_offset":49828,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":53128,"parent_record_offset":53104,"ancestor_record_offset":49828,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":52552,"parent_record_offset":52524,"ancestor_record_offset":49828,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":484,"record_offset":54652,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":484,"record_offset":53324,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":484,"record_offset":54040,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":484,"record_offset":54848,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":484,"record_offset":53240,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":484,"record_offset":55164,"parent_record_offset":55140,"ancestor_record_offset":49828,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":484,"record_offset":53948,"parent_record_offset":53924,"ancestor_record_offset":49828,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":484,"record_offset":54156,"parent_record_offset":54132,"ancestor_record_offset":49828,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":484,"record_offset":53432,"parent_record_offset":53408,"ancestor_record_offset":49828,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":484,"record_offset":53924,"parent_record_offset":53900,"ancestor_record_offset":49828,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":484,"record_offset":53972,"parent_record_offset":53948,"ancestor_record_offset":49828,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":484,"record_offset":53900,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":484,"record_offset":52500,"parent_record_offset":52416,"ancestor_record_offset":49828,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":484,"record_offset":52524,"parent_record_offset":52500,"ancestor_record_offset":49828,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":484,"record_offset":52828,"parent_record_offset":52800,"ancestor_record_offset":49828,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":484,"record_offset":54132,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":484,"record_offset":53408,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":484,"record_offset":53804,"parent_record_offset":53780,"ancestor_record_offset":49828,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":484,"record_offset":53512,"parent_record_offset":53488,"ancestor_record_offset":49828,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":484,"record_offset":53488,"parent_record_offset":53464,"ancestor_record_offset":49828,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":484,"record_offset":53648,"parent_record_offset":53624,"ancestor_record_offset":49828,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":484,"record_offset":53464,"parent_record_offset":53432,"ancestor_record_offset":49828,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":484,"record_offset":53080,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":484,"record_offset":53996,"parent_record_offset":53972,"ancestor_record_offset":49828,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":484,"record_offset":52996,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":484,"record_offset":53836,"parent_record_offset":53432,"ancestor_record_offset":49828,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":484,"record_offset":53780,"parent_record_offset":53432,"ancestor_record_offset":49828,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":484,"record_offset":53860,"parent_record_offset":53836,"ancestor_record_offset":49828,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":484,"record_offset":52468,"parent_record_offset":52444,"ancestor_record_offset":49828,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":484,"record_offset":52416,"parent_record_offset":52392,"ancestor_record_offset":49828,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":484,"record_offset":52444,"parent_record_offset":52416,"ancestor_record_offset":49828,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":484,"record_offset":55004,"parent_record_offset":54976,"ancestor_record_offset":49828,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":484,"record_offset":51744,"parent_record_offset":51716,"ancestor_record_offset":49828,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":484,"record_offset":51796,"parent_record_offset":51772,"ancestor_record_offset":49828,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":484,"record_offset":51772,"parent_record_offset":51744,"ancestor_record_offset":49828,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":484,"record_offset":51692,"parent_record_offset":51664,"ancestor_record_offset":49828,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":484,"record_offset":51716,"parent_record_offset":51692,"ancestor_record_offset":49828,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":484,"record_offset":51888,"parent_record_offset":51716,"ancestor_record_offset":49828,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":484,"record_offset":52000,"parent_record_offset":51976,"ancestor_record_offset":49828,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":484,"record_offset":51820,"parent_record_offset":51796,"ancestor_record_offset":49828,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":484,"record_offset":50148,"parent_record_offset":50116,"ancestor_record_offset":49828,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":484,"record_offset":51560,"parent_record_offset":51520,"ancestor_record_offset":49828,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":484,"record_offset":53536,"parent_record_offset":53512,"ancestor_record_offset":49828,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":484,"record_offset":53584,"parent_record_offset":53560,"ancestor_record_offset":49828,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":484,"record_offset":53624,"parent_record_offset":53488,"ancestor_record_offset":49828,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":484,"record_offset":50396,"parent_record_offset":50368,"ancestor_record_offset":49828,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":484,"record_offset":52348,"parent_record_offset":51520,"ancestor_record_offset":49828,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":51952,"parent_record_offset":51664,"ancestor_record_offset":49828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":52748,"parent_record_offset":52724,"ancestor_record_offset":49828,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":484,"record_offset":50860,"parent_record_offset":50836,"ancestor_record_offset":49828,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":484,"record_offset":50836,"parent_record_offset":50812,"ancestor_record_offset":49828,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":484,"record_offset":51044,"parent_record_offset":51016,"ancestor_record_offset":49828,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":51844,"parent_record_offset":51820,"ancestor_record_offset":49828,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":484,"record_offset":53020,"parent_record_offset":52996,"ancestor_record_offset":49828,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":53728,"parent_record_offset":53704,"ancestor_record_offset":49828,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":53044,"parent_record_offset":53020,"ancestor_record_offset":49828,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":53704,"parent_record_offset":53672,"ancestor_record_offset":49828,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":484,"record_offset":52724,"parent_record_offset":52552,"ancestor_record_offset":49828,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":50788,"parent_record_offset":50764,"ancestor_record_offset":49828,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":50812,"parent_record_offset":50788,"ancestor_record_offset":49828,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":51116,"parent_record_offset":51088,"ancestor_record_offset":49828,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":52924,"parent_record_offset":52828,"ancestor_record_offset":49828,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":50764,"parent_record_offset":50648,"ancestor_record_offset":49828,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":484,"record_offset":50116,"parent_record_offset":50088,"ancestor_record_offset":49828,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":484,"record_offset":50476,"parent_record_offset":50448,"ancestor_record_offset":49828,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":484,"record_offset":50616,"parent_record_offset":50584,"ancestor_record_offset":49828,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":484,"record_offset":50584,"parent_record_offset":50548,"ancestor_record_offset":49828,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":484,"record_offset":50524,"parent_record_offset":50500,"ancestor_record_offset":49828,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":484,"record_offset":50548,"parent_record_offset":50524,"ancestor_record_offset":49828,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":484,"record_offset":50500,"parent_record_offset":50476,"ancestor_record_offset":49828,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":484,"record_offset":51088,"parent_record_offset":50548,"ancestor_record_offset":49828,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":484,"record_offset":50088,"parent_record_offset":50056,"ancestor_record_offset":49828,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":484,"record_offset":50056,"parent_record_offset":50028,"ancestor_record_offset":49828,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":484,"record_offset":50248,"parent_record_offset":50216,"ancestor_record_offset":49828,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":484,"record_offset":50344,"parent_record_offset":50248,"ancestor_record_offset":49828,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":484,"record_offset":50216,"parent_record_offset":50192,"ancestor_record_offset":49828,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":484,"record_offset":50028,"parent_record_offset":49992,"ancestor_record_offset":49828,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":484,"record_offset":50368,"parent_record_offset":50344,"ancestor_record_offset":49828,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":484,"record_offset":50648,"parent_record_offset":50616,"ancestor_record_offset":49828,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":484,"record_offset":50932,"parent_record_offset":50648,"ancestor_record_offset":49828,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":484,"record_offset":50904,"parent_record_offset":50648,"ancestor_record_offset":49828,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":484,"record_offset":50708,"parent_record_offset":50648,"ancestor_record_offset":49828,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":484,"record_offset":53672,"parent_record_offset":53648,"ancestor_record_offset":49828,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":484,"record_offset":51456,"parent_record_offset":51424,"ancestor_record_offset":49828,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":484,"record_offset":55260,"parent_record_offset":55228,"ancestor_record_offset":49828,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":484,"record_offset":55760,"parent_record_offset":55500,"ancestor_record_offset":49828,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":484,"record_offset":51588,"parent_record_offset":51560,"ancestor_record_offset":49828,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":484,"record_offset":51664,"parent_record_offset":51520,"ancestor_record_offset":49828,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":484,"record_offset":51924,"parent_record_offset":51664,"ancestor_record_offset":49828,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":484,"record_offset":51616,"parent_record_offset":51588,"ancestor_record_offset":49828,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":484,"record_offset":51520,"parent_record_offset":51488,"ancestor_record_offset":49828,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":484,"record_offset":51488,"parent_record_offset":51456,"ancestor_record_offset":49828,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00484.jsonl.gz"}
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
// 完整结构及原始类型记录: look_at_camera.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::look_at_camera::LookAtCamera
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 59
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::systems::look_at_camera::LookAtCamera>", "constructor": "ref$", "arguments": ["country_core::systems::look_at_camera::LookAtCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >", "constructor": "tuple$", "arguments": ["ref_mut$<utils::components::transform::Transform>", "ref$<country_core::systems::look_at_camera::LookAtCamera>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >", "constructor": "bevy_ecs::system::query::Query", "arguments": ["tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$4::into_configs<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::schedule::config::impl$4::into_configs", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::look_at_camera::LookAtCamera>", "constructor": "bevy_ecs::query::fetch::impl$26::init_state", "arguments": ["country_core::systems::look_at_camera::LookAtCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::init_query_param<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >", "constructor": "bevy_ecs::system::system_param::init_query_param", "arguments": ["tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>),void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>)", "void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>),void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>),void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)>", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::world_query::impl$2::update_component_access<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >", "constructor": "bevy_ecs::query::world_query::impl$2::update_component_access", "arguments": ["ref_mut$<utils::components::transform::Transform>", "ref$<country_core::systems::look_at_camera::LookAtCamera>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new_with_access<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >", "constructor": "bevy_ecs::query::state::QueryState::new_with_access", "arguments": ["tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::update_archetype_component_access<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >", "constructor": "bevy_ecs::query::state::QueryState::update_archetype_component_access", "arguments": ["tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentsRegistrator::register_component<country_core::systems::look_at_camera::LookAtCamera>", "constructor": "bevy_ecs::component::ComponentsRegistrator::register_component", "arguments": ["country_core::systems::look_at_camera::LookAtCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview>", "constructor": "tuple$", "arguments": ["country_core::components::KeepOnLevelUnload", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "country_core::systems::look_at_camera::LookAtCamera", "country_core::components::FollowMouse", "country_core::components::drawable::UiPass", "country_core::systems::brush_preview::BrushPreview"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> >", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass>", "constructor": "tuple$", "arguments": ["country_core::components::KeepOnLevelUnload", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "country_core::systems::brush_preview::BrushPreview", "country_core::systems::look_at_camera::LookAtCamera", "country_core::components::drawable::UiPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> >", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass>", "constructor": "tuple$", "arguments": ["country_core::components::KeepOnLevelUnload", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "country_core::systems::brush_preview::BrushPreview", "country_core::components::ArrowGizmo", "country_core::systems::look_at_camera::LookAtCamera", "country_core::components::FollowMouse", "country_core::components::drawable::UiPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> >", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> >", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$27::component_ids<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> > >", "constructor": "bevy_ecs::bundle::impl$27::component_ids", "arguments": ["country_core::components::KeepOnLevelUnload", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "country_core::systems::brush_preview::BrushPreview", "country_core::systems::look_at_camera::LookAtCamera", "country_core::components::drawable::UiPass", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> >", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$30::component_ids<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> > >", "constructor": "bevy_ecs::bundle::impl$30::component_ids", "arguments": ["country_core::components::KeepOnLevelUnload", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "country_core::systems::look_at_camera::LookAtCamera", "country_core::components::FollowMouse", "country_core::components::drawable::UiPass", "country_core::systems::brush_preview::BrushPreview", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> >", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$33::component_ids<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> > >", "constructor": "bevy_ecs::bundle::impl$33::component_ids", "arguments": ["country_core::components::KeepOnLevelUnload", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "country_core::systems::brush_preview::BrushPreview", "country_core::components::ArrowGizmo", "country_core::systems::look_at_camera::LookAtCamera", "country_core::components::FollowMouse", "country_core::components::drawable::UiPass", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera>", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["country_core::systems::look_at_camera::LookAtCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::BundleInfo::write_components<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::BundleInfo::write_components", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::BundleInfo::write_components<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::BundleInfo::write_components", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::BundleInfo::write_components<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::BundleInfo::write_components", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::look_at_camera::LookAtCamera>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass> >", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview> >", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass> >", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<country_core::systems::look_at_camera::LookAtCamera>", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["country_core::systems::look_at_camera::LookAtCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<country_core::systems::look_at_camera::LookAtCamera>", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["country_core::systems::look_at_camera::LookAtCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<country_core::systems::look_at_camera::LookAtCamera,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["country_core::systems::look_at_camera::LookAtCamera", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<country_core::systems::look_at_camera::LookAtCamera>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: look_at_camera.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: look_at_camera.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::world::deferred_world::DeferredWorld,bevy_ecs::observer::ObserverTrigger,bevy_ptr::PtrMut<bevy_ptr::Aligned>,ref_mut$<bool>)
// 候选参数类型: ["bevy_ecs::world::deferred_world::DeferredWorld", "bevy_ecs::observer::ObserverTrigger", "bevy_ptr::PtrMut<bevy_ptr::Aligned>", "ref_mut$<bool>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>

// ------------------------------------------------------------------
// 函数: country_core::systems::look_at_camera::impl$0::clone_behavior
// 信息 ID: 48837d170dc45033a68fb0a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 183 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::look_at_camera::LookAtCamera> @ 0x3bdf7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::world_query::impl$2::init_state<ref$<country_core::resources::critters::sheep::UmbrellaHolder>,ref$<country_core::resources::critters::sheep::FloatieHolder> > @ 0x81034 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::world_query::impl$2::init_state<ref$<country_core::resources::critters::sheep::UmbrellaHolder>,ref$<country_core::resources::critters::sheep::FloatieHolder> > @ 0x814f2 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::critters::sheep::Sheep> @ 0x81f47 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::critters::sheep::SheepAudio> @ 0x825d7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xba779 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xeed7f -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xeee3a -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$13::closure$1::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xfc149 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$13::closure$1::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xfe760 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::ui::signifiers::SignifierAnimation> @ 0x157a17 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> @ 0x1580a7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::world_query::impl$3::init_state<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> > @ 0x25bfe7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> @ 0x2b1887 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | 其余 9 个同地址别名见共享索引 @ 0x32c4f0 -> 0x2370 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::prefab::PrefabInstance> @ 0x32cbb7 -> 0x2370 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::look_at_camera::impl$0::register_required_components
// 信息 ID: a975bce46bae9888bf8a862b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::look_at_camera::LookAtCamera> @ 0x3c00b -> 0x8756e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::look_at_camera::impl$0::register_required_components @ 0x875b7a -> 0x8756e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::component::ComponentsRegistrator::register_component<country_core::systems::look_at_camera::LookAtCamera> @ 0xa33ed2 -> 0x8756e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::bundle::impl$0::component_ids<country_core::systems::look_at_camera::LookAtCamera,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<country_core::systems::look_at_camera::LookAtCamera> > @ 0x286a227 -> 0x8756e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::look_at_camera::look_at_camera
// 信息 ID: 8f587ea9904ff97c5c8cdef9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fadfb67e242e85315fa7dbe5: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > @ 0x399fc -> 0x873de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > > @ 0x47b905 -> 0x873de0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
