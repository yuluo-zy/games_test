// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/load_mesh_atlases.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5be82f03169867424726f849e9ff9107bbd44f7ecd780df99658cdf09bd884b5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_mesh_atlases.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1553, 1585, 1587, 1610, 1631, 1737
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 28, 36, 47, 53, 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AFB
// 首条位置: {"module":1587,"inlinee":72443}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":1587,"record_offset":39156,"parent_record_offset":39132,"ancestor_record_offset":39132,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":1587,"record_offset":68984,"parent_record_offset":68960,"ancestor_record_offset":68960,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 28, 36, 47, 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AFF
// 首条位置: {"module":1587,"inlinee":72447}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1587,"record_offset":38508,"parent_record_offset":38480,"ancestor_record_offset":38480,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1587,"record_offset":38532,"parent_record_offset":38508,"ancestor_record_offset":38480,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1587,"record_offset":38556,"parent_record_offset":38532,"ancestor_record_offset":38480,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1587,"record_offset":38580,"parent_record_offset":38556,"ancestor_record_offset":38480,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 28, 36, 47, 53, 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AF9
// 首条位置: {"module":1587,"inlinee":72441}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1587,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":3716,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1587,"record_offset":38532,"parent_record_offset":38508,"ancestor_record_offset":38340,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1587,"record_offset":3776,"parent_record_offset":3740,"ancestor_record_offset":3716,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1587,"record_offset":3840,"parent_record_offset":3740,"ancestor_record_offset":3716,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1587,"record_offset":38556,"parent_record_offset":38532,"ancestor_record_offset":38340,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1587,"record_offset":38580,"parent_record_offset":38556,"ancestor_record_offset":38340,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1587,"record_offset":38508,"parent_record_offset":38480,"ancestor_record_offset":38340,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1587,"record_offset":38480,"parent_record_offset":38452,"ancestor_record_offset":38340,"inlinee":72447,"source_line":28,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1587,"record_offset":3812,"parent_record_offset":3740,"ancestor_record_offset":3716,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 28, 36, 47, 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AFE
// 首条位置: {"module":1587,"inlinee":72446}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1587,"record_offset":38884,"parent_record_offset":37684,"ancestor_record_offset":37684,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1587,"record_offset":39368,"parent_record_offset":37684,"ancestor_record_offset":37684,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1587,"record_offset":38792,"parent_record_offset":37684,"ancestor_record_offset":37684,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1587,"record_offset":69816,"parent_record_offset":65692,"ancestor_record_offset":65692,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_atlases::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1587,"record_offset":69188,"parent_record_offset":65692,"ancestor_record_offset":65692,"inlinee":72450,"source_line":36,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1587,"record_offset":37732,"parent_record_offset":37684,"ancestor_record_offset":37684,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1587,"record_offset":38828,"parent_record_offset":37684,"ancestor_record_offset":37684,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1587,"record_offset":39576,"parent_record_offset":39552,"ancestor_record_offset":37684,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1587,"record_offset":39552,"parent_record_offset":39480,"ancestor_record_offset":37684,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1587,"record_offset":39516,"parent_record_offset":39480,"ancestor_record_offset":37684,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1587,"record_offset":39480,"parent_record_offset":39452,"ancestor_record_offset":37684,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1587,"record_offset":39452,"parent_record_offset":39424,"ancestor_record_offset":37684,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1587,"record_offset":59988,"parent_record_offset":59960,"ancestor_record_offset":58264,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1587,"record_offset":38532,"parent_record_offset":38508,"ancestor_record_offset":37684,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1587,"record_offset":37964,"parent_record_offset":37768,"ancestor_record_offset":37684,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1587,"record_offset":38092,"parent_record_offset":37768,"ancestor_record_offset":37684,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1587,"record_offset":60432,"parent_record_offset":60404,"ancestor_record_offset":58264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1587,"record_offset":39396,"parent_record_offset":39368,"ancestor_record_offset":37684,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1587,"record_offset":37928,"parent_record_offset":37768,"ancestor_record_offset":37684,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1587,"record_offset":38556,"parent_record_offset":38532,"ancestor_record_offset":37684,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1587,"record_offset":38580,"parent_record_offset":38556,"ancestor_record_offset":37684,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1587,"record_offset":38508,"parent_record_offset":38480,"ancestor_record_offset":37684,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1587,"record_offset":67364,"parent_record_offset":67340,"ancestor_record_offset":65692,"inlinee":72448,"source_line":45,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_atlases::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1587,"record_offset":39132,"parent_record_offset":39108,"ancestor_record_offset":37684,"inlinee":72443,"source_line":36,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1587,"record_offset":38480,"parent_record_offset":38452,"ancestor_record_offset":37684,"inlinee":72447,"source_line":28,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1587,"record_offset":38060,"parent_record_offset":38036,"ancestor_record_offset":37684,"inlinee":72441,"source_line":28,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":1587,"record_offset":37996,"parent_record_offset":37768,"ancestor_record_offset":37684,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1587,"record_offset":38372,"parent_record_offset":38340,"ancestor_record_offset":37684,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1587,"record_offset":37768,"parent_record_offset":37732,"ancestor_record_offset":37684,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1587,"record_offset":38944,"parent_record_offset":38912,"ancestor_record_offset":37684,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":1587,"record_offset":39156,"parent_record_offset":39132,"ancestor_record_offset":37684,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_seq
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":1587,"record_offset":59784,"parent_record_offset":59760,"ancestor_record_offset":58264,"inlinee":6904,"source_line":188,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":1587,"record_offset":68984,"parent_record_offset":68960,"ancestor_record_offset":65692,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1587,"record_offset":38452,"parent_record_offset":38372,"ancestor_record_offset":37684,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1587,"record_offset":38036,"parent_record_offset":37768,"ancestor_record_offset":37684,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1587,"record_offset":59760,"parent_record_offset":59736,"ancestor_record_offset":58264,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11B02
// 首条位置: {"module":1587,"inlinee":72450}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1631,"record_offset":169132,"rva":23039040,"va":5391748160}
// 编译实例 RVAs: 0x15f8c40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t19732797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391748160,"raw_hex":"e83d192d01","size":5,"soft_fail":false,"target_va":5411481152,"va":5391748350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1631,"record_offset":169328,"parent_record_offset":169132,"ancestor_record_offset":169132,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1631,"record_offset":169348,"parent_record_offset":169328,"ancestor_record_offset":169132,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1631,"record_offset":169368,"parent_record_offset":169348,"ancestor_record_offset":169132,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1631,"record_offset":169388,"parent_record_offset":169368,"ancestor_record_offset":169132,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AFC
// 首条位置: {"module":1587,"inlinee":72444}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1587,"record_offset":31968,"parent_record_offset":31912,"ancestor_record_offset":31912,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1587,"record_offset":31940,"parent_record_offset":31912,"ancestor_record_offset":31912,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::load_mesh_atlases
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1553,"record_offset":19880,"rva":19526656,"va":5388235776}
// 编译实例 RVAs: 0x129f400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=5
//     target_aliases=35 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18635631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e891a4e3fe","size":5,"soft_fail":false,"target_va":5369600800,"va":5388236426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13155607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8e94237ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388237106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10578712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8e8945eff","size":5,"soft_fail":false,"target_va":5377657648,"va":5388236355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140928100
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9926171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8e58968ff","size":5,"soft_fail":false,"target_va":5378310400,"va":5388236566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140928350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9926379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8158968ff","size":5,"soft_fail":false,"target_va":5378310992,"va":5388237366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8825852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8045479ff","size":5,"soft_fail":false,"target_va":5379411088,"va":5388236935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a371d0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8816450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8be7879ff","size":5,"soft_fail":false,"target_va":5379420624,"va":5388237069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a37760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8814077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8038279ff","size":5,"soft_fail":false,"target_va":5379422048,"va":5388236120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413018b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t400860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8dc1d0600","size":5,"soft_fail":false,"target_va":5388638384,"va":5388237519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141302d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t406955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8ab350600","size":5,"soft_fail":false,"target_va":5388643680,"va":5388236720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f7190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3505081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8b97b3500","size":5,"soft_fail":false,"target_va":5391741328,"va":5388236242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23611355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8db476801","size":5,"soft_fail":false,"target_va":5411848144,"va":5388236784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5396821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8aba6adff","size":5,"soft_fail":false,"target_va":5382840208,"va":5388237024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t4228913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e831874000","size":5,"soft_fail":false,"target_va":5392466848,"va":5388237930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8099107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8dd6a84ff","size":5,"soft_fail":false,"target_va":5380139920,"va":5388239022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23297991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388235776,"raw_hex":"e8c77f6301","size":5,"soft_fail":false,"target_va":5411537104,"va":5388239108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1553,"record_offset":21312,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":21148,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1553,"record_offset":21452,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":26267,"source_line":10,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1553,"record_offset":21396,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":21628,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":15252,"source_line":1326,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1553,"record_offset":21232,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1553,"record_offset":21340,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":28830,"source_line":35,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::prefab::PrefabLibrary::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/prefab.rs
//     evidence={"module":1553,"record_offset":20076,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":28029,"source_line":46,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1553,"record_offset":23504,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":71868,"source_line":197,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_atlases::generate_plaster_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1553,"record_offset":24844,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":71869,"source_line":234,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_atlases::generate_roof_floor_meshes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1553,"record_offset":21828,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":71867,"source_line":159,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_atlases::generate_stairs_floor_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1553,"record_offset":26504,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":71870,"source_line":255,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_terrain::generate_grid_cw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":1553,"record_offset":28220,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":28841,"source_line":264,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":20452,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":23164,"parent_record_offset":23140,"ancestor_record_offset":19880,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":23140,"parent_record_offset":23084,"ancestor_record_offset":19880,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1553,"record_offset":22876,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1553,"record_offset":22924,"parent_record_offset":22900,"ancestor_record_offset":19880,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":25352,"parent_record_offset":25328,"ancestor_record_offset":19880,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":23112,"parent_record_offset":23084,"ancestor_record_offset":19880,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":23084,"parent_record_offset":23060,"ancestor_record_offset":19880,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":25400,"parent_record_offset":25376,"ancestor_record_offset":19880,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":25376,"parent_record_offset":25352,"ancestor_record_offset":19880,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":19880,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1553,"record_offset":21476,"parent_record_offset":21452,"ancestor_record_offset":19880,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":25328,"parent_record_offset":25292,"ancestor_record_offset":19880,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":22616,"parent_record_offset":22592,"ancestor_record_offset":19880,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":21196,"parent_record_offset":21172,"ancestor_record_offset":19880,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":21364,"parent_record_offset":21340,"ancestor_record_offset":19880,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":25292,"parent_record_offset":24844,"ancestor_record_offset":19880,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":19880,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":22568,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":21172,"parent_record_offset":21148,"ancestor_record_offset":19880,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":22592,"parent_record_offset":22568,"ancestor_record_offset":19880,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1553,"record_offset":21420,"parent_record_offset":21396,"ancestor_record_offset":19880,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1553,"record_offset":21704,"parent_record_offset":21680,"ancestor_record_offset":19880,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":22312,"parent_record_offset":22284,"ancestor_record_offset":19880,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":19880,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":22340,"parent_record_offset":22284,"ancestor_record_offset":19880,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":22020,"parent_record_offset":21960,"ancestor_record_offset":19880,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":20332,"parent_record_offset":20304,"ancestor_record_offset":19880,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":20392,"parent_record_offset":20360,"ancestor_record_offset":19880,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1553,"record_offset":22816,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::signum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1553,"record_offset":22760,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":4989,"source_line":1471,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1553,"record_offset":22960,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":21960,"parent_record_offset":21936,"ancestor_record_offset":19880,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":21936,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":25496,"parent_record_offset":25472,"ancestor_record_offset":19880,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":25472,"parent_record_offset":24844,"ancestor_record_offset":19880,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1553,"record_offset":22532,"parent_record_offset":22508,"ancestor_record_offset":19880,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1553,"record_offset":22484,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1553,"record_offset":22388,"parent_record_offset":22364,"ancestor_record_offset":19880,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1553,"record_offset":20756,"parent_record_offset":20724,"ancestor_record_offset":19880,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1553,"record_offset":20724,"parent_record_offset":20692,"ancestor_record_offset":19880,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1553,"record_offset":21992,"parent_record_offset":21960,"ancestor_record_offset":19880,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":21652,"parent_record_offset":21628,"ancestor_record_offset":19880,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":22900,"parent_record_offset":22876,"ancestor_record_offset":19880,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$0::as_deref_mut::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":21680,"parent_record_offset":21652,"ancestor_record_offset":19880,"inlinee":15254,"source_line":1330,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":20276,"parent_record_offset":20248,"ancestor_record_offset":19880,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":22256,"parent_record_offset":22228,"ancestor_record_offset":19880,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":22988,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1553,"record_offset":21016,"parent_record_offset":20992,"ancestor_record_offset":19880,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1553,"record_offset":21276,"parent_record_offset":21232,"ancestor_record_offset":19880,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":22412,"parent_record_offset":22388,"ancestor_record_offset":19880,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1553,"record_offset":22652,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":21508,"parent_record_offset":21476,"ancestor_record_offset":19880,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":19880,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1553,"record_offset":22104,"parent_record_offset":22060,"ancestor_record_offset":19880,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_terrain::generate_grid_ccw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":1553,"record_offset":23708,"parent_record_offset":23504,"ancestor_record_offset":19880,"inlinee":28876,"source_line":311,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_terrain::generate_grid_cw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":1553,"record_offset":22060,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":28841,"source_line":264,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":20648,"parent_record_offset":20600,"ancestor_record_offset":19880,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":20692,"parent_record_offset":20648,"ancestor_record_offset":19880,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":20800,"parent_record_offset":20600,"ancestor_record_offset":19880,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":20600,"parent_record_offset":20548,"ancestor_record_offset":19880,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":20304,"parent_record_offset":20248,"ancestor_record_offset":19880,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":20360,"parent_record_offset":20304,"ancestor_record_offset":19880,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":20892,"parent_record_offset":20548,"ancestor_record_offset":19880,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":20132,"parent_record_offset":20104,"ancestor_record_offset":19880,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":20480,"parent_record_offset":20452,"ancestor_record_offset":19880,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":20992,"parent_record_offset":20548,"ancestor_record_offset":19880,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":20248,"parent_record_offset":20188,"ancestor_record_offset":19880,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":20548,"parent_record_offset":20508,"ancestor_record_offset":19880,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":20160,"parent_record_offset":20132,"ancestor_record_offset":19880,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":20220,"parent_record_offset":20188,"ancestor_record_offset":19880,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":20188,"parent_record_offset":20160,"ancestor_record_offset":19880,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":20508,"parent_record_offset":20480,"ancestor_record_offset":19880,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":20104,"parent_record_offset":20076,"ancestor_record_offset":19880,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":22132,"parent_record_offset":22104,"ancestor_record_offset":19880,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1553,"record_offset":22788,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1553,"record_offset":22732,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1553,"record_offset":22848,"parent_record_offset":21828,"ancestor_record_offset":19880,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":22180,"parent_record_offset":22156,"ancestor_record_offset":19880,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":22284,"parent_record_offset":22228,"ancestor_record_offset":19880,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":22156,"parent_record_offset":22132,"ancestor_record_offset":19880,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":22228,"parent_record_offset":22204,"ancestor_record_offset":19880,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":19880,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1631,"record_offset":155424,"rva":23032208,"va":5391741328}
// 编译实例 RVAs: 0x15f7190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b18f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17063474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8cea1fbfe","size":5,"soft_fail":false,"target_va":5374679280,"va":5391742749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17063777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e89fa0fbfe","size":5,"soft_fail":false,"target_va":5374679664,"va":5391743436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16845835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8f5f3fefe","size":5,"soft_fail":false,"target_va":5374900736,"va":5391746566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16660030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8c2c901ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5391741513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=19
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16661036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8d4c501ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5391742535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16662087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8b9c101ff","size":5,"soft_fail":false,"target_va":5375081536,"va":5391743618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087bd60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14139728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8b03e28ff","size":5,"soft_fail":false,"target_va":5377604960,"va":5391744683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140887a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14093339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8e5f328ff","size":5,"soft_fail":false,"target_va":5377653328,"va":5391746662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14088226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8de0729ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5391745869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408919b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14047628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e874a629ff","size":5,"soft_fail":false,"target_va":5377694128,"va":5391741751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13888763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e805132cff","size":5,"soft_fail":false,"target_va":5377858208,"va":5391746966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14091d540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13476058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8265f32ff","size":5,"soft_fail":false,"target_va":5378266432,"va":5391742485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12335770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e866c543ff","size":5,"soft_fail":false,"target_va":5379408240,"va":5391744005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12334559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e821ca43ff","size":5,"soft_fail":false,"target_va":5379409664,"va":5391744218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12333064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8f8cf43ff","size":5,"soft_fail":false,"target_va":5379411088,"va":5391744147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a355f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12330593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e89fd943ff","size":5,"soft_fail":false,"target_va":5379413488,"va":5391744076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a35b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12329382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e85ade43ff","size":5,"soft_fail":false,"target_va":5379414912,"va":5391744289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a371d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12323732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e86cf443ff","size":5,"soft_fail":false,"target_va":5379420624,"va":5391744351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b11f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11425265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e80faa51ff","size":5,"soft_fail":false,"target_va":5380317008,"va":5391742268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b75360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11020831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8e1d557ff","size":5,"soft_fail":false,"target_va":5380723552,"va":5391744378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b77de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11008350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8a20658ff","size":5,"soft_fail":false,"target_va":5380734432,"va":5391742777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14169c5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t676590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8ee520a00","size":5,"soft_fail":false,"target_va":5392418224,"va":5391741629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20103862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8b6c23201","size":5,"soft_fail":false,"target_va":5411847152,"va":5391743285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8902012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8842a78ff","size":5,"soft_fail":false,"target_va":5382840208,"va":5391742215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t19796202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8ea102e01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391744756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9091842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8fe4475ff","size":5,"soft_fail":false,"target_va":5382655024,"va":5391746861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19796793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e839132e01","size":5,"soft_fail":false,"target_va":5411541904,"va":5391745106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19797913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e899172e01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391744962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9032813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391741328,"raw_hex":"e8932b76ff","size":5,"soft_fail":false,"target_va":5382708672,"va":5391741480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*28942964(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5391741328,"raw_hex":"ff1574a2b901","size":6,"soft_fail":false,"va":5391745398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1631,"record_offset":156932,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":162040,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":156460,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":159008,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1631,"record_offset":156516,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1631,"record_offset":156664,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1631,"record_offset":157044,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1631,"record_offset":166236,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1631,"record_offset":157016,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1631,"record_offset":159700,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1631,"record_offset":155836,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1631,"record_offset":157128,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1631,"record_offset":159268,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::has_attribute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1631,"record_offset":160676,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":30873,"source_line":43,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::vertex_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1631,"record_offset":158044,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":30886,"source_line":212,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_prefabs::PrefabWatcher::watch_atlas
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1631,"record_offset":157200,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":71903,"source_line":417,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_reader
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1631,"record_offset":156352,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":72451,"source_line":98,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":159328,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1631,"record_offset":155720,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufreader::BufReader::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1631,"record_offset":155880,"parent_record_offset":155424,"ancestor_record_offset":155424,"inlinee":18036,"source_line":73,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":156124,"parent_record_offset":156100,"ancestor_record_offset":155424,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":156148,"parent_record_offset":156124,"ancestor_record_offset":155424,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":162296,"parent_record_offset":162272,"ancestor_record_offset":155424,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":157664,"parent_record_offset":157640,"ancestor_record_offset":155424,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":156100,"parent_record_offset":156072,"ancestor_record_offset":155424,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":162248,"parent_record_offset":162156,"ancestor_record_offset":155424,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":157640,"parent_record_offset":157584,"ancestor_record_offset":155424,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$2::new_uninit_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1631,"record_offset":155984,"parent_record_offset":155956,"ancestor_record_offset":155424,"inlinee":5319,"source_line":663,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1631,"record_offset":156980,"parent_record_offset":156956,"ancestor_record_offset":155424,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":159112,"parent_record_offset":159088,"ancestor_record_offset":155424,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":156012,"parent_record_offset":155984,"ancestor_record_offset":155424,"inlinee":5320,"source_line":123,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":162096,"parent_record_offset":162068,"ancestor_record_offset":155424,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":157612,"parent_record_offset":157584,"ancestor_record_offset":155424,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":157584,"parent_record_offset":157560,"ancestor_record_offset":155424,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":159160,"parent_record_offset":159136,"ancestor_record_offset":155424,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":159136,"parent_record_offset":159112,"ancestor_record_offset":155424,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":156072,"parent_record_offset":156040,"ancestor_record_offset":155424,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":156040,"parent_record_offset":156012,"ancestor_record_offset":155424,"inlinee":5321,"source_line":145,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":162124,"parent_record_offset":162096,"ancestor_record_offset":155424,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":157560,"parent_record_offset":157536,"ancestor_record_offset":155424,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":162192,"parent_record_offset":162156,"ancestor_record_offset":155424,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1631,"record_offset":157288,"parent_record_offset":157200,"ancestor_record_offset":155424,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":159088,"parent_record_offset":159064,"ancestor_record_offset":155424,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":159064,"parent_record_offset":159032,"ancestor_record_offset":155424,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":167508,"parent_record_offset":167480,"ancestor_record_offset":155424,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":159032,"parent_record_offset":159008,"ancestor_record_offset":155424,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1631,"record_offset":165628,"parent_record_offset":165604,"ancestor_record_offset":155424,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1631,"record_offset":165992,"parent_record_offset":165968,"ancestor_record_offset":155424,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1631,"record_offset":162068,"parent_record_offset":162040,"ancestor_record_offset":155424,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1631,"record_offset":162216,"parent_record_offset":162192,"ancestor_record_offset":155424,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1631,"record_offset":161448,"parent_record_offset":161420,"ancestor_record_offset":155424,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1631,"record_offset":158412,"parent_record_offset":158388,"ancestor_record_offset":155424,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1631,"record_offset":160948,"parent_record_offset":160924,"ancestor_record_offset":155424,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1631,"record_offset":158492,"parent_record_offset":158468,"ancestor_record_offset":155424,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1631,"record_offset":160972,"parent_record_offset":160948,"ancestor_record_offset":155424,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1631,"record_offset":161624,"parent_record_offset":160864,"ancestor_record_offset":155424,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1631,"record_offset":159876,"parent_record_offset":159848,"ancestor_record_offset":155424,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1631,"record_offset":161180,"parent_record_offset":161152,"ancestor_record_offset":155424,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1631,"record_offset":158348,"parent_record_offset":158324,"ancestor_record_offset":155424,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1631,"record_offset":157260,"parent_record_offset":157200,"ancestor_record_offset":155424,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1631,"record_offset":158976,"parent_record_offset":158044,"ancestor_record_offset":155424,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1631,"record_offset":156956,"parent_record_offset":156932,"ancestor_record_offset":155424,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=87
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1631,"record_offset":156204,"parent_record_offset":155916,"ancestor_record_offset":155424,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1631,"record_offset":156484,"parent_record_offset":156460,"ancestor_record_offset":155424,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1631,"record_offset":158552,"parent_record_offset":158528,"ancestor_record_offset":155424,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1631,"record_offset":156576,"parent_record_offset":156516,"ancestor_record_offset":155424,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1631,"record_offset":156604,"parent_record_offset":156516,"ancestor_record_offset":155424,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1631,"record_offset":156548,"parent_record_offset":156516,"ancestor_record_offset":155424,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1631,"record_offset":156628,"parent_record_offset":156604,"ancestor_record_offset":155424,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1631,"record_offset":157376,"parent_record_offset":157200,"ancestor_record_offset":155424,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1631,"record_offset":161532,"parent_record_offset":161504,"ancestor_record_offset":155424,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1631,"record_offset":161560,"parent_record_offset":161532,"ancestor_record_offset":155424,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1631,"record_offset":159204,"parent_record_offset":159032,"ancestor_record_offset":155424,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1631,"record_offset":161976,"parent_record_offset":161952,"ancestor_record_offset":155424,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1631,"record_offset":161952,"parent_record_offset":161888,"ancestor_record_offset":155424,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1631,"record_offset":161476,"parent_record_offset":161448,"ancestor_record_offset":155424,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1631,"record_offset":157316,"parent_record_offset":157288,"ancestor_record_offset":155424,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1631,"record_offset":157340,"parent_record_offset":157316,"ancestor_record_offset":155424,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1631,"record_offset":156692,"parent_record_offset":156664,"ancestor_record_offset":155424,"inlinee":26318,"source_line":40,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1631,"record_offset":158292,"parent_record_offset":158260,"ancestor_record_offset":155424,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1631,"record_offset":158324,"parent_record_offset":158292,"ancestor_record_offset":155424,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1631,"record_offset":158260,"parent_record_offset":158216,"ancestor_record_offset":155424,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":160924,"parent_record_offset":160864,"ancestor_record_offset":155424,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":158388,"parent_record_offset":158216,"ancestor_record_offset":155424,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":161652,"parent_record_offset":160864,"ancestor_record_offset":155424,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":158468,"parent_record_offset":158444,"ancestor_record_offset":155424,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":158444,"parent_record_offset":158216,"ancestor_record_offset":155424,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":161008,"parent_record_offset":160864,"ancestor_record_offset":155424,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1631,"record_offset":161804,"parent_record_offset":160864,"ancestor_record_offset":155424,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1631,"record_offset":161420,"parent_record_offset":161392,"ancestor_record_offset":155424,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":160740,"parent_record_offset":160708,"ancestor_record_offset":155424,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":160772,"parent_record_offset":160740,"ancestor_record_offset":155424,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":158708,"parent_record_offset":158684,"ancestor_record_offset":155424,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":159352,"parent_record_offset":159328,"ancestor_record_offset":155424,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":159384,"parent_record_offset":159352,"ancestor_record_offset":155424,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":161392,"parent_record_offset":161224,"ancestor_record_offset":155424,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":158140,"parent_record_offset":158104,"ancestor_record_offset":155424,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":159412,"parent_record_offset":159384,"ancestor_record_offset":155424,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":161332,"parent_record_offset":161248,"ancestor_record_offset":155424,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":161276,"parent_record_offset":161248,"ancestor_record_offset":155424,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":158528,"parent_record_offset":158216,"ancestor_record_offset":155424,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":161744,"parent_record_offset":160864,"ancestor_record_offset":155424,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":158812,"parent_record_offset":158756,"ancestor_record_offset":155424,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":158216,"parent_record_offset":158176,"ancestor_record_offset":155424,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":161248,"parent_record_offset":161224,"ancestor_record_offset":155424,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":160836,"parent_record_offset":160808,"ancestor_record_offset":155424,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":160808,"parent_record_offset":160772,"ancestor_record_offset":155424,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":158732,"parent_record_offset":158708,"ancestor_record_offset":155424,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":158784,"parent_record_offset":158756,"ancestor_record_offset":155424,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":160864,"parent_record_offset":160836,"ancestor_record_offset":155424,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":158756,"parent_record_offset":158732,"ancestor_record_offset":155424,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":161776,"parent_record_offset":160864,"ancestor_record_offset":155424,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":163700,"parent_record_offset":163676,"ancestor_record_offset":155424,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":158176,"parent_record_offset":158140,"ancestor_record_offset":155424,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":161224,"parent_record_offset":160864,"ancestor_record_offset":155424,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":1631,"record_offset":156376,"parent_record_offset":156352,"ancestor_record_offset":155424,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":160708,"parent_record_offset":160676,"ancestor_record_offset":155424,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":158684,"parent_record_offset":158660,"ancestor_record_offset":155424,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":158660,"parent_record_offset":158044,"ancestor_record_offset":155424,"inlinee":4948,"source_line":440,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":158104,"parent_record_offset":158068,"ancestor_record_offset":155424,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$61::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":158068,"parent_record_offset":158044,"ancestor_record_offset":155424,"inlinee":11891,"source_line":2551,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1631,"record_offset":155744,"parent_record_offset":155720,"ancestor_record_offset":155424,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1631,"record_offset":155804,"parent_record_offset":155720,"ancestor_record_offset":155424,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::BufReader::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1631,"record_offset":155916,"parent_record_offset":155880,"ancestor_record_offset":155424,"inlinee":5316,"source_line":102,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":1631,"record_offset":155956,"parent_record_offset":155916,"ancestor_record_offset":155424,"inlinee":5317,"source_line":34,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1631,"record_offset":156300,"parent_record_offset":156276,"ancestor_record_offset":155424,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1631,"record_offset":157092,"parent_record_offset":157068,"ancestor_record_offset":155424,"inlinee":8206,"source_line":1836,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$40::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1631,"record_offset":157068,"parent_record_offset":157044,"ancestor_record_offset":155424,"inlinee":8204,"source_line":1858,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1631,"record_offset":155772,"parent_record_offset":155744,"ancestor_record_offset":155424,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1631,"record_offset":143264,"rva":23026528,"va":5391735648}
// 编译实例 RVAs: 0x15f5b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b18f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17058119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8b9b6fbfe","size":5,"soft_fail":false,"target_va":5374679280,"va":5391737394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17058434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e87eb5fbfe","size":5,"soft_fail":false,"target_va":5374679664,"va":5391738093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d8fc0
//     owner=unresolved_namespace; count=2
//     target_aliases=23 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16899387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8c522fefe","size":5,"soft_fail":false,"target_va":5374840768,"va":5391740150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dd0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16882886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e83a63fefe","size":5,"soft_fail":false,"target_va":5374857392,"va":5391740273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16840251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8c509fffe","size":5,"soft_fail":false,"target_va":5374900736,"va":5391740982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16654353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8efdf01ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5391735836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16655024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e850dd01ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5391736523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16656748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e894d601ff","size":5,"soft_fail":false,"target_va":5375081536,"va":5391738279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087bd60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14134423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8695328ff","size":5,"soft_fail":false,"target_va":5377604960,"va":5391739378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140887a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14087755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8b50929ff","size":5,"soft_fail":false,"target_va":5377653328,"va":5391741078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14082738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e84e1d29ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5391740381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408919b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14042332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e824bb29ff","size":5,"soft_fail":false,"target_va":5377694128,"va":5391736455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12330426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e846da43ff","size":5,"soft_fail":false,"target_va":5379408240,"va":5391738661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12329209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e807df43ff","size":5,"soft_fail":false,"target_va":5379409664,"va":5391738868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12327716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8dce443ff","size":5,"soft_fail":false,"target_va":5379411088,"va":5391738799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a355f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12325247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e881ee43ff","size":5,"soft_fail":false,"target_va":5379413488,"va":5391738730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a35b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12324030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e842f343ff","size":5,"soft_fail":false,"target_va":5379414912,"va":5391738937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a371d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12318393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8470944ff","size":5,"soft_fail":false,"target_va":5379420624,"va":5391739012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b11f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11419963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8c5be51ff","size":5,"soft_fail":false,"target_va":5380317008,"va":5391736966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b75360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11015492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8bcea57ff","size":5,"soft_fail":false,"target_va":5380723552,"va":5391739039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b77de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11003000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8881b58ff","size":5,"soft_fail":false,"target_va":5380734432,"va":5391737427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e7650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3205024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e86018cfff","size":5,"soft_fail":false,"target_va":5388531280,"va":5391736299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14169c5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t682267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e81b690a00","size":5,"soft_fail":false,"target_va":5392418224,"va":5391735952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20109216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8a0d73201","size":5,"soft_fail":false,"target_va":5411847152,"va":5391737931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8896710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e83a3f78ff","size":5,"soft_fail":false,"target_va":5382840208,"va":5391736913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t19801501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e89d252e01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391739457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9086258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8ce5a75ff","size":5,"soft_fail":false,"target_va":5382655024,"va":5391741277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19802140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e81c282e01","size":5,"soft_fail":false,"target_va":5411541904,"va":5391739759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19803188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8342c2e01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391739687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_prefabs::PrefabWatcher::watch_atlas
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3461569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e83f2ecbff","size":5,"soft_fail":false,"target_va":5388275616,"va":5391737180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9027136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391735648,"raw_hex":"e8c04176ff","size":5,"soft_fail":false,"target_va":5382708672,"va":5391735803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*28948308(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5391735648,"raw_hex":"ff1554b7b901","size":6,"soft_fail":false,"va":5391740054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1631,"record_offset":146308,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":150572,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":144300,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":147540,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1631,"record_offset":144356,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1631,"record_offset":146040,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1631,"record_offset":146420,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1631,"record_offset":154768,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1631,"record_offset":146392,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1631,"record_offset":145424,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1631,"record_offset":143676,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1631,"record_offset":146504,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1631,"record_offset":147800,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::has_attribute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1631,"record_offset":149208,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":30873,"source_line":43,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::vertex_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1631,"record_offset":146576,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":30886,"source_line":212,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1631,"record_offset":144516,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":74553,"source_line":332,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_reader
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":1631,"record_offset":144192,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":72451,"source_line":98,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":147860,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1631,"record_offset":143560,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufreader::BufReader::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1631,"record_offset":143720,"parent_record_offset":143264,"ancestor_record_offset":143264,"inlinee":18036,"source_line":73,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":143964,"parent_record_offset":143940,"ancestor_record_offset":143264,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":143988,"parent_record_offset":143964,"ancestor_record_offset":143264,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":150828,"parent_record_offset":150804,"ancestor_record_offset":143264,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":145760,"parent_record_offset":145736,"ancestor_record_offset":143264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":143940,"parent_record_offset":143912,"ancestor_record_offset":143264,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":150780,"parent_record_offset":150688,"ancestor_record_offset":143264,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1631,"record_offset":145736,"parent_record_offset":145672,"ancestor_record_offset":143264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$2::new_uninit_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1631,"record_offset":143824,"parent_record_offset":143796,"ancestor_record_offset":143264,"inlinee":5319,"source_line":663,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1631,"record_offset":146356,"parent_record_offset":146332,"ancestor_record_offset":143264,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":147644,"parent_record_offset":147620,"ancestor_record_offset":143264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":143852,"parent_record_offset":143824,"ancestor_record_offset":143264,"inlinee":5320,"source_line":123,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":150628,"parent_record_offset":150600,"ancestor_record_offset":143264,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":145704,"parent_record_offset":145672,"ancestor_record_offset":143264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":145672,"parent_record_offset":145644,"ancestor_record_offset":143264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":147692,"parent_record_offset":147668,"ancestor_record_offset":143264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":147668,"parent_record_offset":147644,"ancestor_record_offset":143264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":143912,"parent_record_offset":143880,"ancestor_record_offset":143264,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":143880,"parent_record_offset":143852,"ancestor_record_offset":143264,"inlinee":5321,"source_line":145,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":150656,"parent_record_offset":150628,"ancestor_record_offset":143264,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":145644,"parent_record_offset":145616,"ancestor_record_offset":143264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1631,"record_offset":150724,"parent_record_offset":150688,"ancestor_record_offset":143264,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":147620,"parent_record_offset":147596,"ancestor_record_offset":143264,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":147596,"parent_record_offset":147564,"ancestor_record_offset":143264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":145476,"parent_record_offset":145452,"ancestor_record_offset":143264,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1631,"record_offset":147564,"parent_record_offset":147540,"ancestor_record_offset":143264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1631,"record_offset":154160,"parent_record_offset":154136,"ancestor_record_offset":143264,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1631,"record_offset":154524,"parent_record_offset":154500,"ancestor_record_offset":143264,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1631,"record_offset":150600,"parent_record_offset":150572,"ancestor_record_offset":143264,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1631,"record_offset":150748,"parent_record_offset":150724,"ancestor_record_offset":143264,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1631,"record_offset":145172,"parent_record_offset":145148,"ancestor_record_offset":143264,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1631,"record_offset":146944,"parent_record_offset":146920,"ancestor_record_offset":143264,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1631,"record_offset":144824,"parent_record_offset":144800,"ancestor_record_offset":143264,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1631,"record_offset":144908,"parent_record_offset":144884,"ancestor_record_offset":143264,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1631,"record_offset":144848,"parent_record_offset":144824,"ancestor_record_offset":143264,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1631,"record_offset":145240,"parent_record_offset":144716,"ancestor_record_offset":143264,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1631,"record_offset":145932,"parent_record_offset":145900,"ancestor_record_offset":143264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1631,"record_offset":145056,"parent_record_offset":145028,"ancestor_record_offset":143264,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1631,"record_offset":145028,"parent_record_offset":145000,"ancestor_record_offset":143264,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1631,"record_offset":147508,"parent_record_offset":146576,"ancestor_record_offset":143264,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1631,"record_offset":146332,"parent_record_offset":146308,"ancestor_record_offset":143264,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1631,"record_offset":144044,"parent_record_offset":143756,"ancestor_record_offset":143264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1631,"record_offset":144324,"parent_record_offset":144300,"ancestor_record_offset":143264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1631,"record_offset":147084,"parent_record_offset":147060,"ancestor_record_offset":143264,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1631,"record_offset":144428,"parent_record_offset":144356,"ancestor_record_offset":143264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1631,"record_offset":144456,"parent_record_offset":144356,"ancestor_record_offset":143264,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1631,"record_offset":144392,"parent_record_offset":144356,"ancestor_record_offset":143264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1631,"record_offset":144480,"parent_record_offset":144456,"ancestor_record_offset":143264,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1631,"record_offset":150064,"parent_record_offset":150036,"ancestor_record_offset":143264,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1631,"record_offset":150092,"parent_record_offset":150064,"ancestor_record_offset":143264,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1631,"record_offset":147736,"parent_record_offset":147564,"ancestor_record_offset":143264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1631,"record_offset":150508,"parent_record_offset":150484,"ancestor_record_offset":143264,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1631,"record_offset":150484,"parent_record_offset":150420,"ancestor_record_offset":143264,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1631,"record_offset":150008,"parent_record_offset":149980,"ancestor_record_offset":143264,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$19::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1631,"record_offset":145196,"parent_record_offset":145172,"ancestor_record_offset":143264,"inlinee":26318,"source_line":40,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1631,"record_offset":144968,"parent_record_offset":144940,"ancestor_record_offset":143264,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1631,"record_offset":145000,"parent_record_offset":144968,"ancestor_record_offset":143264,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1631,"record_offset":144940,"parent_record_offset":144716,"ancestor_record_offset":143264,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":144800,"parent_record_offset":144716,"ancestor_record_offset":143264,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":146920,"parent_record_offset":146748,"ancestor_record_offset":143264,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":145268,"parent_record_offset":144716,"ancestor_record_offset":143264,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":147000,"parent_record_offset":146976,"ancestor_record_offset":143264,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":146976,"parent_record_offset":146748,"ancestor_record_offset":143264,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":144884,"parent_record_offset":144716,"ancestor_record_offset":143264,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1631,"record_offset":144772,"parent_record_offset":144716,"ancestor_record_offset":143264,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1631,"record_offset":145148,"parent_record_offset":145124,"ancestor_record_offset":143264,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":144588,"parent_record_offset":144564,"ancestor_record_offset":143264,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":144612,"parent_record_offset":144588,"ancestor_record_offset":143264,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":147240,"parent_record_offset":147216,"ancestor_record_offset":143264,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":147884,"parent_record_offset":147860,"ancestor_record_offset":143264,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":147916,"parent_record_offset":147884,"ancestor_record_offset":143264,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":145124,"parent_record_offset":145100,"ancestor_record_offset":143264,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":146672,"parent_record_offset":146636,"ancestor_record_offset":143264,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":144640,"parent_record_offset":144612,"ancestor_record_offset":143264,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":149864,"parent_record_offset":149780,"ancestor_record_offset":143264,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":149808,"parent_record_offset":149780,"ancestor_record_offset":143264,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":147060,"parent_record_offset":146748,"ancestor_record_offset":143264,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":145360,"parent_record_offset":144716,"ancestor_record_offset":143264,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":147344,"parent_record_offset":147288,"ancestor_record_offset":143264,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":146748,"parent_record_offset":146708,"ancestor_record_offset":143264,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":149780,"parent_record_offset":149756,"ancestor_record_offset":143264,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":144692,"parent_record_offset":144668,"ancestor_record_offset":143264,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":144668,"parent_record_offset":144612,"ancestor_record_offset":143264,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":147264,"parent_record_offset":147240,"ancestor_record_offset":143264,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":147316,"parent_record_offset":147288,"ancestor_record_offset":143264,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":144716,"parent_record_offset":144692,"ancestor_record_offset":143264,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":147288,"parent_record_offset":147264,"ancestor_record_offset":143264,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":150308,"parent_record_offset":149396,"ancestor_record_offset":143264,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":152232,"parent_record_offset":152208,"ancestor_record_offset":143264,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":146708,"parent_record_offset":146672,"ancestor_record_offset":143264,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":145100,"parent_record_offset":144716,"ancestor_record_offset":143264,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1631,"record_offset":144564,"parent_record_offset":144540,"ancestor_record_offset":143264,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":1631,"record_offset":144216,"parent_record_offset":144192,"ancestor_record_offset":143264,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":149240,"parent_record_offset":149208,"ancestor_record_offset":143264,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":147216,"parent_record_offset":147192,"ancestor_record_offset":143264,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":147192,"parent_record_offset":146576,"ancestor_record_offset":143264,"inlinee":4948,"source_line":440,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":146636,"parent_record_offset":146600,"ancestor_record_offset":143264,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$61::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1631,"record_offset":146600,"parent_record_offset":146576,"ancestor_record_offset":143264,"inlinee":11891,"source_line":2551,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1631,"record_offset":144540,"parent_record_offset":144516,"ancestor_record_offset":143264,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1631,"record_offset":143584,"parent_record_offset":143560,"ancestor_record_offset":143264,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1631,"record_offset":143644,"parent_record_offset":143560,"ancestor_record_offset":143264,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::BufReader::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1631,"record_offset":143756,"parent_record_offset":143720,"ancestor_record_offset":143264,"inlinee":5316,"source_line":102,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":1631,"record_offset":143796,"parent_record_offset":143756,"ancestor_record_offset":143264,"inlinee":5317,"source_line":34,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1631,"record_offset":144140,"parent_record_offset":144116,"ancestor_record_offset":143264,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1631,"record_offset":146468,"parent_record_offset":146444,"ancestor_record_offset":143264,"inlinee":8206,"source_line":1836,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$40::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1631,"record_offset":146444,"parent_record_offset":146420,"ancestor_record_offset":143264,"inlinee":8204,"source_line":1858,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1631,"record_offset":143612,"parent_record_offset":143584,"ancestor_record_offset":143264,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::generate_roof_floor_meshes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118BB
// 首条位置: {"module":1553,"inlinee":71867}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1553,"record_offset":22876,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":22568,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1553,"record_offset":22816,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::signum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1553,"record_offset":22760,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":4989,"source_line":1471,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1553,"record_offset":22960,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":21936,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1553,"record_offset":22484,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":22988,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1553,"record_offset":22652,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_terrain::generate_grid_cw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":1553,"record_offset":22060,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":28841,"source_line":264,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1553,"record_offset":22788,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1553,"record_offset":22732,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1553,"record_offset":22848,"parent_record_offset":21828,"ancestor_record_offset":21828,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":23164,"parent_record_offset":23140,"ancestor_record_offset":21828,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":23140,"parent_record_offset":23084,"ancestor_record_offset":21828,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1553,"record_offset":22924,"parent_record_offset":22900,"ancestor_record_offset":21828,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":23112,"parent_record_offset":23084,"ancestor_record_offset":21828,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":23084,"parent_record_offset":23060,"ancestor_record_offset":21828,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":21828,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":22616,"parent_record_offset":22592,"ancestor_record_offset":21828,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":21828,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":22592,"parent_record_offset":22568,"ancestor_record_offset":21828,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":22312,"parent_record_offset":22284,"ancestor_record_offset":21828,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":21828,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":22340,"parent_record_offset":22284,"ancestor_record_offset":21828,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":22020,"parent_record_offset":21960,"ancestor_record_offset":21828,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":21960,"parent_record_offset":21936,"ancestor_record_offset":21828,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1553,"record_offset":22532,"parent_record_offset":22508,"ancestor_record_offset":21828,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1553,"record_offset":22388,"parent_record_offset":22364,"ancestor_record_offset":21828,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1553,"record_offset":21992,"parent_record_offset":21960,"ancestor_record_offset":21828,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":22900,"parent_record_offset":22876,"ancestor_record_offset":21828,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":22256,"parent_record_offset":22228,"ancestor_record_offset":21828,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":21828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1553,"record_offset":22696,"parent_record_offset":22652,"ancestor_record_offset":21828,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":22412,"parent_record_offset":22388,"ancestor_record_offset":21828,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1553,"record_offset":22104,"parent_record_offset":22060,"ancestor_record_offset":21828,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":22132,"parent_record_offset":22104,"ancestor_record_offset":21828,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":22180,"parent_record_offset":22156,"ancestor_record_offset":21828,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":22284,"parent_record_offset":22228,"ancestor_record_offset":21828,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":22156,"parent_record_offset":22132,"ancestor_record_offset":21828,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":22228,"parent_record_offset":22204,"ancestor_record_offset":21828,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":21828,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::generate_roof_floor_meshes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11ABE
// 首条位置: {"module":1585,"inlinee":72382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118BC
// 首条位置: {"module":1553,"inlinee":71868}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1553,"record_offset":24216,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1553,"record_offset":24300,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":23584,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1553,"record_offset":24132,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":24328,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_terrain::generate_grid_ccw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":1553,"record_offset":23708,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":28876,"source_line":311,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":24504,"parent_record_offset":24480,"ancestor_record_offset":23504,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":24480,"parent_record_offset":24424,"ancestor_record_offset":23504,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1553,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":23504,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":24452,"parent_record_offset":24424,"ancestor_record_offset":23504,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":24424,"parent_record_offset":24400,"ancestor_record_offset":23504,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":24400,"parent_record_offset":24376,"ancestor_record_offset":23504,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":23504,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":23960,"parent_record_offset":23932,"ancestor_record_offset":23504,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":23504,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":23988,"parent_record_offset":23932,"ancestor_record_offset":23504,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":23668,"parent_record_offset":23608,"ancestor_record_offset":23504,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":23608,"parent_record_offset":23584,"ancestor_record_offset":23504,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1553,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":23504,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1553,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":23504,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1553,"record_offset":23640,"parent_record_offset":23608,"ancestor_record_offset":23504,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":24240,"parent_record_offset":24216,"ancestor_record_offset":23504,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":23904,"parent_record_offset":23876,"ancestor_record_offset":23504,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":24352,"parent_record_offset":24328,"ancestor_record_offset":23504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":24060,"parent_record_offset":24036,"ancestor_record_offset":23504,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1553,"record_offset":23752,"parent_record_offset":23708,"ancestor_record_offset":23504,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":23780,"parent_record_offset":23752,"ancestor_record_offset":23504,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":23828,"parent_record_offset":23804,"ancestor_record_offset":23504,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":23932,"parent_record_offset":23876,"ancestor_record_offset":23504,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":23804,"parent_record_offset":23780,"ancestor_record_offset":23504,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":23876,"parent_record_offset":23852,"ancestor_record_offset":23504,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":23852,"parent_record_offset":23828,"ancestor_record_offset":23504,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 209
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12096
// 首条位置: {"module":1610,"inlinee":73878}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::generate_plaster_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118BD
// 首条位置: {"module":1553,"inlinee":71869}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":25292,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":25472,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1553,"record_offset":24916,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":25352,"parent_record_offset":25328,"ancestor_record_offset":24844,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":25400,"parent_record_offset":25376,"ancestor_record_offset":24844,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":25376,"parent_record_offset":25352,"ancestor_record_offset":24844,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":25328,"parent_record_offset":25292,"ancestor_record_offset":24844,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":25124,"parent_record_offset":25096,"ancestor_record_offset":24844,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":25176,"parent_record_offset":25152,"ancestor_record_offset":24844,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":25152,"parent_record_offset":25096,"ancestor_record_offset":24844,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":25520,"parent_record_offset":25496,"ancestor_record_offset":24844,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":25496,"parent_record_offset":25472,"ancestor_record_offset":24844,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1553,"record_offset":25200,"parent_record_offset":25176,"ancestor_record_offset":24844,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":25068,"parent_record_offset":25040,"ancestor_record_offset":24844,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":25224,"parent_record_offset":25200,"ancestor_record_offset":24844,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":24944,"parent_record_offset":24916,"ancestor_record_offset":24844,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":24992,"parent_record_offset":24968,"ancestor_record_offset":24844,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":25096,"parent_record_offset":25040,"ancestor_record_offset":24844,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":24968,"parent_record_offset":24944,"ancestor_record_offset":24844,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":25040,"parent_record_offset":25016,"ancestor_record_offset":24844,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":25016,"parent_record_offset":24992,"ancestor_record_offset":24844,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::generate_stairs_floor_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118BE
// 首条位置: {"module":1553,"inlinee":71870}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1553,"record_offset":27592,"parent_record_offset":26504,"ancestor_record_offset":26504,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":26968,"parent_record_offset":26504,"ancestor_record_offset":26504,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1553,"record_offset":27676,"parent_record_offset":26504,"ancestor_record_offset":26504,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":27148,"parent_record_offset":26504,"ancestor_record_offset":26504,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":27704,"parent_record_offset":26504,"ancestor_record_offset":26504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1553,"record_offset":26592,"parent_record_offset":26504,"ancestor_record_offset":26504,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":27880,"parent_record_offset":27856,"ancestor_record_offset":26504,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":27856,"parent_record_offset":27800,"ancestor_record_offset":26504,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1553,"record_offset":27640,"parent_record_offset":27616,"ancestor_record_offset":26504,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":27028,"parent_record_offset":27004,"ancestor_record_offset":26504,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":27828,"parent_record_offset":27800,"ancestor_record_offset":26504,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":27800,"parent_record_offset":27776,"ancestor_record_offset":26504,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":27076,"parent_record_offset":27052,"ancestor_record_offset":26504,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":27052,"parent_record_offset":27028,"ancestor_record_offset":26504,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":27776,"parent_record_offset":27752,"ancestor_record_offset":26504,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":27004,"parent_record_offset":26968,"ancestor_record_offset":26504,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":26800,"parent_record_offset":26772,"ancestor_record_offset":26504,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":26852,"parent_record_offset":26828,"ancestor_record_offset":26504,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":26828,"parent_record_offset":26772,"ancestor_record_offset":26504,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":27196,"parent_record_offset":27172,"ancestor_record_offset":26504,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":27172,"parent_record_offset":27148,"ancestor_record_offset":26504,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1553,"record_offset":26876,"parent_record_offset":26852,"ancestor_record_offset":26504,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":27616,"parent_record_offset":27592,"ancestor_record_offset":26504,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":26744,"parent_record_offset":26716,"ancestor_record_offset":26504,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":27728,"parent_record_offset":27704,"ancestor_record_offset":26504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":26900,"parent_record_offset":26876,"ancestor_record_offset":26504,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":26620,"parent_record_offset":26592,"ancestor_record_offset":26504,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":26668,"parent_record_offset":26644,"ancestor_record_offset":26504,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":26772,"parent_record_offset":26716,"ancestor_record_offset":26504,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":26644,"parent_record_offset":26620,"ancestor_record_offset":26504,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":26716,"parent_record_offset":26692,"ancestor_record_offset":26504,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":26692,"parent_record_offset":26668,"ancestor_record_offset":26504,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::reload_mesh_atlases
// 记录形式: compiled_procedure; instances=1
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1553,"record_offset":28764,"rva":19531808,"va":5388240928}
// 编译实例 RVAs: 0x12a0820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5599614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8828eaaff","size":5,"soft_fail":false,"target_va":5382644528,"va":5388244137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19261439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e80118dafe","size":5,"soft_fail":false,"target_va":5368983552,"va":5388244986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18640234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e89692e3fe","size":5,"soft_fail":false,"target_va":5369600800,"va":5388241029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18460325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e85b51e6fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5388242944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13431049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8f70e33ff","size":5,"soft_fail":false,"target_va":5374814320,"va":5388245364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13160552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8982f37ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388242051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12727922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e88ec93dff","size":5,"soft_fail":false,"target_va":5375516032,"va":5388243949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087bb00
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10640808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e858a25dff","size":5,"soft_fail":false,"target_va":5377604352,"va":5388245155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a37760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8821397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e86b6579ff","size":5,"soft_fail":false,"target_va":5379422048,"va":5388243440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a37e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8818506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8b67079ff","size":5,"soft_fail":false,"target_va":5379423824,"va":5388242325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b73f00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7526124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e814298dff","size":5,"soft_fail":false,"target_va":5380718336,"va":5388244455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130b3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t436712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8e8a90600","size":5,"soft_fail":false,"target_va":5388678096,"va":5388241379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415232f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2631835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e89b282800","size":5,"soft_fail":false,"target_va":5390873328,"va":5388241488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415265f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2644504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8185a2800","size":5,"soft_fail":false,"target_va":5390886384,"va":5388241875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152b320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2664371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8b3a72800","size":5,"soft_fail":false,"target_va":5390906144,"va":5388241768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141552d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2826729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8e9212b00","size":5,"soft_fail":false,"target_va":5391068528,"va":5388241794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f5b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3491451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e87b463500","size":5,"soft_fail":false,"target_va":5391735648,"va":5388244192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23604404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8b42c6801","size":5,"soft_fail":false,"target_va":5411847152,"va":5388242743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13040574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8420439ff","size":5,"soft_fail":false,"target_va":5375203792,"va":5388244361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23298226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8b2806301","size":5,"soft_fail":false,"target_va":5411542880,"va":5388244649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_world_subset_atlas
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8103044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e87c5b84ff","size":5,"soft_fail":false,"target_va":5380139136,"va":5388242175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8104327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8795684ff","size":5,"soft_fail":false,"target_va":5380139920,"va":5388244242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23294050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e862706301","size":5,"soft_fail":false,"target_va":5411538608,"va":5388244553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23294227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e813716301","size":5,"soft_fail":false,"target_va":5411538768,"va":5388244536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23294289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e851716301","size":5,"soft_fail":false,"target_va":5411538960,"va":5388244666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23292387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388240928,"raw_hex":"e8e3696301","size":5,"soft_fail":false,"target_va":5411537104,"va":5388244712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1553,"record_offset":45120,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":34488,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1553,"record_offset":34320,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":26267,"source_line":10,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1553,"record_offset":36444,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":1553,"record_offset":31844,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1553,"record_offset":45752,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":45316,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1553,"record_offset":32560,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":11831,"source_line":63,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":34956,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1553,"record_offset":34748,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1553,"record_offset":44892,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":28830,"source_line":35,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::prefab::PrefabLibrary::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/prefab.rs
//     evidence={"module":1553,"record_offset":37700,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":28029,"source_line":46,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_prefabs::PrefabWatcher::changed_world_subsets
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1553,"record_offset":29908,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":71872,"source_line":435,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1553,"record_offset":45348,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1553,"record_offset":45232,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":38060,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":38984,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":34432,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":45148,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":24753,"source_line":246,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":29000,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":35380,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":25277,"source_line":1513,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":35792,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":31644,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":35320,"parent_record_offset":35288,"ancestor_record_offset":28764,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":35288,"parent_record_offset":35132,"ancestor_record_offset":28764,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":34592,"parent_record_offset":34568,"ancestor_record_offset":28764,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":34640,"parent_record_offset":34616,"ancestor_record_offset":28764,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":34616,"parent_record_offset":34592,"ancestor_record_offset":28764,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1553,"record_offset":40508,"parent_record_offset":40484,"ancestor_record_offset":28764,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1553,"record_offset":34344,"parent_record_offset":34320,"ancestor_record_offset":28764,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1553,"record_offset":29952,"parent_record_offset":29908,"ancestor_record_offset":28764,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$47::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1553,"record_offset":39788,"parent_record_offset":39756,"ancestor_record_offset":28764,"inlinee":10452,"source_line":3246,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$49::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1553,"record_offset":39756,"parent_record_offset":39724,"ancestor_record_offset":28764,"inlinee":10450,"source_line":3295,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$60::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1553,"record_offset":40460,"parent_record_offset":40436,"ancestor_record_offset":28764,"inlinee":33623,"source_line":3568,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":45944,"parent_record_offset":45896,"ancestor_record_offset":28764,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":34568,"parent_record_offset":34544,"ancestor_record_offset":28764,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":32204,"parent_record_offset":32180,"ancestor_record_offset":28764,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":44916,"parent_record_offset":44892,"ancestor_record_offset":28764,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":45896,"parent_record_offset":45852,"ancestor_record_offset":28764,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":31160,"parent_record_offset":31132,"ancestor_record_offset":28764,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":31692,"parent_record_offset":31668,"ancestor_record_offset":28764,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":32180,"parent_record_offset":31932,"ancestor_record_offset":28764,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1553,"record_offset":39724,"parent_record_offset":39700,"ancestor_record_offset":28764,"inlinee":26314,"source_line":10,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::impl$12::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1553,"record_offset":40436,"parent_record_offset":40412,"ancestor_record_offset":28764,"inlinee":71878,"source_line":10,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1553,"record_offset":36468,"parent_record_offset":36444,"ancestor_record_offset":28764,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1553,"record_offset":45784,"parent_record_offset":45752,"ancestor_record_offset":28764,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":1553,"record_offset":45852,"parent_record_offset":45784,"ancestor_record_offset":28764,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":1553,"record_offset":31908,"parent_record_offset":31868,"ancestor_record_offset":28764,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":1553,"record_offset":31932,"parent_record_offset":31908,"ancestor_record_offset":28764,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":1553,"record_offset":33480,"parent_record_offset":33456,"ancestor_record_offset":28764,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":1553,"record_offset":33504,"parent_record_offset":33480,"ancestor_record_offset":28764,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1553,"record_offset":31868,"parent_record_offset":31844,"ancestor_record_offset":28764,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":29252,"parent_record_offset":29228,"ancestor_record_offset":28764,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":29304,"parent_record_offset":29280,"ancestor_record_offset":28764,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":29280,"parent_record_offset":29228,"ancestor_record_offset":28764,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":30504,"parent_record_offset":30480,"ancestor_record_offset":28764,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":39700,"parent_record_offset":39676,"ancestor_record_offset":28764,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":30864,"parent_record_offset":30836,"ancestor_record_offset":28764,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":39264,"parent_record_offset":39240,"ancestor_record_offset":28764,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":30924,"parent_record_offset":30892,"ancestor_record_offset":28764,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1553,"record_offset":40256,"parent_record_offset":40232,"ancestor_record_offset":28764,"inlinee":12248,"source_line":694,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1553,"record_offset":40668,"parent_record_offset":40640,"ancestor_record_offset":28764,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1553,"record_offset":40568,"parent_record_offset":40460,"ancestor_record_offset":28764,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1553,"record_offset":40412,"parent_record_offset":40256,"ancestor_record_offset":28764,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1553,"record_offset":42608,"parent_record_offset":42584,"ancestor_record_offset":28764,"inlinee":21871,"source_line":811,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":40340,"parent_record_offset":40312,"ancestor_record_offset":28764,"inlinee":9025,"source_line":185,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":40368,"parent_record_offset":40340,"ancestor_record_offset":28764,"inlinee":9026,"source_line":200,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":40312,"parent_record_offset":40288,"ancestor_record_offset":28764,"inlinee":9023,"source_line":178,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":40748,"parent_record_offset":40724,"ancestor_record_offset":28764,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":42680,"parent_record_offset":42656,"ancestor_record_offset":28764,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":40616,"parent_record_offset":40592,"ancestor_record_offset":28764,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":40772,"parent_record_offset":40748,"ancestor_record_offset":28764,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":40640,"parent_record_offset":40616,"ancestor_record_offset":28764,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":40820,"parent_record_offset":40772,"ancestor_record_offset":28764,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1553,"record_offset":41164,"parent_record_offset":40772,"ancestor_record_offset":28764,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":39288,"parent_record_offset":39264,"ancestor_record_offset":28764,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":40012,"parent_record_offset":39184,"ancestor_record_offset":28764,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1553,"record_offset":33556,"parent_record_offset":33532,"ancestor_record_offset":28764,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1553,"record_offset":33532,"parent_record_offset":33504,"ancestor_record_offset":28764,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1553,"record_offset":33636,"parent_record_offset":33612,"ancestor_record_offset":28764,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1553,"record_offset":32624,"parent_record_offset":32588,"ancestor_record_offset":28764,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1553,"record_offset":32588,"parent_record_offset":32560,"ancestor_record_offset":28764,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1553,"record_offset":33408,"parent_record_offset":33384,"ancestor_record_offset":28764,"inlinee":64699,"source_line":446,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$10::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1553,"record_offset":33384,"parent_record_offset":33360,"ancestor_record_offset":28764,"inlinee":64697,"source_line":278,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1553,"record_offset":33360,"parent_record_offset":32588,"ancestor_record_offset":28764,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1553,"record_offset":31804,"parent_record_offset":31780,"ancestor_record_offset":28764,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1553,"record_offset":33456,"parent_record_offset":33432,"ancestor_record_offset":28764,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1553,"record_offset":31188,"parent_record_offset":31160,"ancestor_record_offset":28764,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1553,"record_offset":31132,"parent_record_offset":29908,"ancestor_record_offset":28764,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1553,"record_offset":31780,"parent_record_offset":31668,"ancestor_record_offset":28764,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1553,"record_offset":31720,"parent_record_offset":31692,"ancestor_record_offset":28764,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1553,"record_offset":29328,"parent_record_offset":29304,"ancestor_record_offset":28764,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":35224,"parent_record_offset":35196,"ancestor_record_offset":28764,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":32132,"parent_record_offset":32100,"ancestor_record_offset":28764,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":31980,"parent_record_offset":31932,"ancestor_record_offset":28764,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":40908,"parent_record_offset":40820,"ancestor_record_offset":28764,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":40880,"parent_record_offset":40820,"ancestor_record_offset":28764,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1553,"record_offset":36108,"parent_record_offset":36084,"ancestor_record_offset":28764,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1553,"record_offset":32992,"parent_record_offset":32968,"ancestor_record_offset":28764,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1553,"record_offset":32680,"parent_record_offset":32624,"ancestor_record_offset":28764,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1553,"record_offset":33844,"parent_record_offset":33820,"ancestor_record_offset":28764,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":32652,"parent_record_offset":32624,"ancestor_record_offset":28764,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":33820,"parent_record_offset":33456,"ancestor_record_offset":28764,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":33612,"parent_record_offset":33532,"ancestor_record_offset":28764,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1553,"record_offset":45404,"parent_record_offset":45372,"ancestor_record_offset":28764,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1553,"record_offset":45432,"parent_record_offset":45372,"ancestor_record_offset":28764,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":30808,"parent_record_offset":30772,"ancestor_record_offset":28764,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":29200,"parent_record_offset":29172,"ancestor_record_offset":28764,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":31228,"parent_record_offset":29908,"ancestor_record_offset":28764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1553,"record_offset":31748,"parent_record_offset":31692,"ancestor_record_offset":28764,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1553,"record_offset":33212,"parent_record_offset":33188,"ancestor_record_offset":28764,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1553,"record_offset":33760,"parent_record_offset":33736,"ancestor_record_offset":28764,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1553,"record_offset":40532,"parent_record_offset":40508,"ancestor_record_offset":28764,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1553,"record_offset":34784,"parent_record_offset":34748,"ancestor_record_offset":28764,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":31052,"parent_record_offset":31024,"ancestor_record_offset":28764,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":29352,"parent_record_offset":29328,"ancestor_record_offset":28764,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1553,"record_offset":30440,"parent_record_offset":29908,"ancestor_record_offset":28764,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1553,"record_offset":39876,"parent_record_offset":39848,"ancestor_record_offset":28764,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1553,"record_offset":39904,"parent_record_offset":39876,"ancestor_record_offset":28764,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1553,"record_offset":32036,"parent_record_offset":31932,"ancestor_record_offset":28764,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1553,"record_offset":32404,"parent_record_offset":31932,"ancestor_record_offset":28764,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":32100,"parent_record_offset":32068,"ancestor_record_offset":28764,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":32068,"parent_record_offset":32036,"ancestor_record_offset":28764,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1553,"record_offset":32428,"parent_record_offset":32404,"ancestor_record_offset":28764,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1553,"record_offset":33736,"parent_record_offset":33712,"ancestor_record_offset":28764,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1553,"record_offset":33688,"parent_record_offset":33636,"ancestor_record_offset":28764,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1553,"record_offset":33712,"parent_record_offset":33688,"ancestor_record_offset":28764,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1553,"record_offset":39820,"parent_record_offset":39788,"ancestor_record_offset":28764,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":30360,"parent_record_offset":30332,"ancestor_record_offset":28764,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":31480,"parent_record_offset":31292,"ancestor_record_offset":28764,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":30048,"parent_record_offset":30016,"ancestor_record_offset":28764,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":31572,"parent_record_offset":31540,"ancestor_record_offset":28764,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":34368,"parent_record_offset":34344,"ancestor_record_offset":28764,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":30264,"parent_record_offset":30232,"ancestor_record_offset":28764,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":34392,"parent_record_offset":34368,"ancestor_record_offset":28764,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":30072,"parent_record_offset":30048,"ancestor_record_offset":28764,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":30288,"parent_record_offset":30264,"ancestor_record_offset":28764,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":31504,"parent_record_offset":31480,"ancestor_record_offset":28764,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1553,"record_offset":31596,"parent_record_offset":31572,"ancestor_record_offset":28764,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1553,"record_offset":44668,"parent_record_offset":44644,"ancestor_record_offset":28764,"inlinee":26318,"source_line":40,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$21::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1553,"record_offset":42584,"parent_record_offset":42560,"ancestor_record_offset":28764,"inlinee":71876,"source_line":42,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1553,"record_offset":33868,"parent_record_offset":33844,"ancestor_record_offset":28764,"inlinee":71874,"source_line":290,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":32936,"parent_record_offset":32904,"ancestor_record_offset":28764,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":32968,"parent_record_offset":32936,"ancestor_record_offset":28764,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":36148,"parent_record_offset":35988,"ancestor_record_offset":28764,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":32904,"parent_record_offset":32856,"ancestor_record_offset":28764,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":39240,"parent_record_offset":39184,"ancestor_record_offset":28764,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":30836,"parent_record_offset":30772,"ancestor_record_offset":28764,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":40040,"parent_record_offset":39184,"ancestor_record_offset":28764,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":30892,"parent_record_offset":30836,"ancestor_record_offset":28764,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":33088,"parent_record_offset":32856,"ancestor_record_offset":28764,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":39324,"parent_record_offset":39184,"ancestor_record_offset":28764,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1553,"record_offset":40164,"parent_record_offset":39184,"ancestor_record_offset":28764,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1553,"record_offset":39676,"parent_record_offset":39652,"ancestor_record_offset":28764,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":39048,"parent_record_offset":39016,"ancestor_record_offset":28764,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":30592,"parent_record_offset":30564,"ancestor_record_offset":28764,"inlinee":10289,"source_line":888,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":39080,"parent_record_offset":39048,"ancestor_record_offset":28764,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":45196,"parent_record_offset":45172,"ancestor_record_offset":28764,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":33964,"parent_record_offset":33940,"ancestor_record_offset":28764,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":33940,"parent_record_offset":33916,"ancestor_record_offset":28764,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":29412,"parent_record_offset":29056,"ancestor_record_offset":28764,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":39652,"parent_record_offset":39544,"ancestor_record_offset":28764,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":32792,"parent_record_offset":32764,"ancestor_record_offset":28764,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":32764,"parent_record_offset":32736,"ancestor_record_offset":28764,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":29056,"parent_record_offset":29028,"ancestor_record_offset":28764,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":40232,"parent_record_offset":39080,"ancestor_record_offset":28764,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":39592,"parent_record_offset":39568,"ancestor_record_offset":28764,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":39568,"parent_record_offset":39544,"ancestor_record_offset":28764,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":33188,"parent_record_offset":32856,"ancestor_record_offset":28764,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":40132,"parent_record_offset":39184,"ancestor_record_offset":28764,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":30772,"parent_record_offset":30680,"ancestor_record_offset":28764,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":32856,"parent_record_offset":32820,"ancestor_record_offset":28764,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":30620,"parent_record_offset":30592,"ancestor_record_offset":28764,"inlinee":6290,"source_line":1313,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":31000,"parent_record_offset":30620,"ancestor_record_offset":28764,"inlinee":6279,"source_line":1328,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":39152,"parent_record_offset":39120,"ancestor_record_offset":28764,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":39120,"parent_record_offset":39080,"ancestor_record_offset":28764,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":30652,"parent_record_offset":30620,"ancestor_record_offset":28764,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":35164,"parent_record_offset":35132,"ancestor_record_offset":28764,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":30744,"parent_record_offset":30680,"ancestor_record_offset":28764,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":30716,"parent_record_offset":30680,"ancestor_record_offset":28764,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":35076,"parent_record_offset":35052,"ancestor_record_offset":28764,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":39184,"parent_record_offset":39152,"ancestor_record_offset":28764,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":35132,"parent_record_offset":35076,"ancestor_record_offset":28764,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":35104,"parent_record_offset":35076,"ancestor_record_offset":28764,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":30680,"parent_record_offset":30652,"ancestor_record_offset":28764,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":40192,"parent_record_offset":39184,"ancestor_record_offset":28764,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":35196,"parent_record_offset":35164,"ancestor_record_offset":28764,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":35052,"parent_record_offset":35028,"ancestor_record_offset":28764,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":32820,"parent_record_offset":32792,"ancestor_record_offset":28764,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":39544,"parent_record_offset":39184,"ancestor_record_offset":28764,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1553,"record_offset":39016,"parent_record_offset":38984,"ancestor_record_offset":28764,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1553,"record_offset":30564,"parent_record_offset":30536,"ancestor_record_offset":28764,"inlinee":28302,"source_line":348,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1553,"record_offset":34456,"parent_record_offset":34432,"ancestor_record_offset":28764,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1553,"record_offset":45172,"parent_record_offset":45148,"ancestor_record_offset":28764,"inlinee":24754,"source_line":326,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1553,"record_offset":33916,"parent_record_offset":33892,"ancestor_record_offset":28764,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1553,"record_offset":31668,"parent_record_offset":31644,"ancestor_record_offset":28764,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1553,"record_offset":29028,"parent_record_offset":29000,"ancestor_record_offset":28764,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1553,"record_offset":32736,"parent_record_offset":32708,"ancestor_record_offset":28764,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1553,"record_offset":45460,"parent_record_offset":45372,"ancestor_record_offset":28764,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1553,"record_offset":45372,"parent_record_offset":45348,"ancestor_record_offset":28764,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1553,"record_offset":45484,"parent_record_offset":45460,"ancestor_record_offset":28764,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":37724,"parent_record_offset":37700,"ancestor_record_offset":28764,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":30536,"parent_record_offset":29908,"ancestor_record_offset":28764,"inlinee":28301,"source_line":275,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":33892,"parent_record_offset":33868,"ancestor_record_offset":28764,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":32708,"parent_record_offset":32680,"ancestor_record_offset":28764,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":29112,"parent_record_offset":29084,"ancestor_record_offset":28764,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":29228,"parent_record_offset":29172,"ancestor_record_offset":28764,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":40288,"parent_record_offset":40256,"ancestor_record_offset":28764,"inlinee":9021,"source_line":85,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":40724,"parent_record_offset":40256,"ancestor_record_offset":28764,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":42656,"parent_record_offset":42632,"ancestor_record_offset":28764,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":40592,"parent_record_offset":40568,"ancestor_record_offset":28764,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":29084,"parent_record_offset":29056,"ancestor_record_offset":28764,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":1553,"record_offset":30232,"parent_record_offset":30204,"ancestor_record_offset":28764,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":1553,"record_offset":30204,"parent_record_offset":30176,"ancestor_record_offset":28764,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1553,"record_offset":31292,"parent_record_offset":31256,"ancestor_record_offset":28764,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1553,"record_offset":30332,"parent_record_offset":30136,"ancestor_record_offset":28764,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1553,"record_offset":30136,"parent_record_offset":30108,"ancestor_record_offset":28764,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1553,"record_offset":29980,"parent_record_offset":29908,"ancestor_record_offset":28764,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1553,"record_offset":30108,"parent_record_offset":29980,"ancestor_record_offset":28764,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1553,"record_offset":30480,"parent_record_offset":29908,"ancestor_record_offset":28764,"inlinee":11201,"source_line":663,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1553,"record_offset":31256,"parent_record_offset":31228,"ancestor_record_offset":28764,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1553,"record_offset":30016,"parent_record_offset":29980,"ancestor_record_offset":28764,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1553,"record_offset":31540,"parent_record_offset":31256,"ancestor_record_offset":28764,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":29172,"parent_record_offset":29140,"ancestor_record_offset":28764,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":29140,"parent_record_offset":29112,"ancestor_record_offset":28764,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":1553,"record_offset":30176,"parent_record_offset":30136,"ancestor_record_offset":28764,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118C2
// 首条位置: {"module":1553,"inlinee":71874}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1553,"record_offset":33892,"parent_record_offset":33868,"ancestor_record_offset":33868,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":34128,"parent_record_offset":34100,"ancestor_record_offset":33868,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":34188,"parent_record_offset":34156,"ancestor_record_offset":33868,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":34224,"parent_record_offset":34072,"ancestor_record_offset":33868,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":34100,"parent_record_offset":34072,"ancestor_record_offset":33868,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":34156,"parent_record_offset":34100,"ancestor_record_offset":33868,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":33964,"parent_record_offset":33940,"ancestor_record_offset":33868,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":33940,"parent_record_offset":33916,"ancestor_record_offset":33868,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":34072,"parent_record_offset":34012,"ancestor_record_offset":33868,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":33988,"parent_record_offset":33964,"ancestor_record_offset":33868,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":34044,"parent_record_offset":34012,"ancestor_record_offset":33868,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":34012,"parent_record_offset":33988,"ancestor_record_offset":33868,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1553,"record_offset":33916,"parent_record_offset":33892,"ancestor_record_offset":33868,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 332
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12339
// 首条位置: {"module":1631,"inlinee":74553}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1631,"record_offset":144540,"parent_record_offset":144516,"ancestor_record_offset":144516,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1631,"record_offset":145172,"parent_record_offset":145148,"ancestor_record_offset":144516,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1631,"record_offset":144824,"parent_record_offset":144800,"ancestor_record_offset":144516,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1631,"record_offset":144908,"parent_record_offset":144884,"ancestor_record_offset":144516,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1631,"record_offset":144848,"parent_record_offset":144824,"ancestor_record_offset":144516,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1631,"record_offset":145240,"parent_record_offset":144716,"ancestor_record_offset":144516,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1631,"record_offset":145056,"parent_record_offset":145028,"ancestor_record_offset":144516,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1631,"record_offset":145028,"parent_record_offset":145000,"ancestor_record_offset":144516,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1631,"record_offset":145196,"parent_record_offset":145172,"ancestor_record_offset":144516,"inlinee":26318,"source_line":40,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1631,"record_offset":144968,"parent_record_offset":144940,"ancestor_record_offset":144516,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1631,"record_offset":145000,"parent_record_offset":144968,"ancestor_record_offset":144516,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1631,"record_offset":144940,"parent_record_offset":144716,"ancestor_record_offset":144516,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":144800,"parent_record_offset":144716,"ancestor_record_offset":144516,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":145268,"parent_record_offset":144716,"ancestor_record_offset":144516,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1631,"record_offset":144884,"parent_record_offset":144716,"ancestor_record_offset":144516,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1631,"record_offset":144772,"parent_record_offset":144716,"ancestor_record_offset":144516,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1631,"record_offset":145148,"parent_record_offset":145124,"ancestor_record_offset":144516,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":144588,"parent_record_offset":144564,"ancestor_record_offset":144516,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":144612,"parent_record_offset":144588,"ancestor_record_offset":144516,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":145124,"parent_record_offset":145100,"ancestor_record_offset":144516,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1631,"record_offset":144640,"parent_record_offset":144612,"ancestor_record_offset":144516,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":145360,"parent_record_offset":144716,"ancestor_record_offset":144516,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":144692,"parent_record_offset":144668,"ancestor_record_offset":144516,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":144668,"parent_record_offset":144612,"ancestor_record_offset":144516,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":144716,"parent_record_offset":144692,"ancestor_record_offset":144516,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1631,"record_offset":145100,"parent_record_offset":144716,"ancestor_record_offset":144516,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1631,"record_offset":144564,"parent_record_offset":144540,"ancestor_record_offset":144516,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01631.jsonl.gz"}
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
// 完整结构及原始类型记录: load_mesh_atlases.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_mesh_atlases::MeshAttribAdd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::load_mesh_atlases::MeshAttribAdd,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::startup::load_mesh_atlases::MeshAttribAdd", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshAttribAdd>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::startup::load_mesh_atlases::MeshAttribAdd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshAttribAdd> >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshAttribAdd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshAttribAdd>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::startup::load_mesh_atlases::MeshAttribAdd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshAttribAdd> >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshAttribAdd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_mesh_atlases::MeshAttribConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::load_mesh_atlases::MeshAttribConfig>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::load_mesh_atlases::MeshAttribConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_mesh_atlases::MeshConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::load_mesh_atlases::MeshConfig,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshConfig>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshConfig> >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshConfig>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshConfig> >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig> >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig> >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "constructor": "core::marker::PhantomData", "arguments": ["alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_reader_seed<std::io::buffered::bufreader::BufReader<std::fs::File>,core::marker::PhantomData<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >,alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "constructor": "ron::options::Options::from_reader_seed", "arguments": ["std::io::buffered::bufreader::BufReader<std::fs::File>", "core::marker::PhantomData<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_mesh_atlases.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_mesh_atlases.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 40; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize
// 信息 ID: 92ab5de0dc16f6ef1a618e74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshConfig> >; module=1587, inline_record=15912; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshAttribAdd> >; module=1587, inline_record=16956; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig> >; module=1587, inline_record=18008; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=39132; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=68960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 6bb92bad06fe365b10f52014
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=38480; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=49812; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=59076; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=66568; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_identifier<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__FieldVisitor>; module=1587, inline_record=72348; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__FieldVisitor>; module=1737, inline_record=95340; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__FieldVisitor>; module=1737, inline_record=95764; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__FieldVisitor>; module=1737, inline_record=96560; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 9777df9245b3d6e2575e2d3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Field> > @ 0x1314476 -> 0x15f8c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 2163954fa444faf4af4669b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Field> >; module=1587, inline_record=3716; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=38060; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=38216; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=38340; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=49392; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=49548; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=49672; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=58656; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 639529ed081c4a08c407bd41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=31912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 0dedc3d211b09ce51df1fa5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=37684; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=49000; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=58264; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=65692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes
// 信息 ID: 28004048597ed60127398377
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=1553, inline_record=23504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure$0
// 信息 ID: afd84387a89cd797259491b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,7>,country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<array$<f32,3>,7>,core::ops::range::Range<u32>,country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0<array$<array$<f32,3>,7>,tuple$<>,core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<array$<f32,3>,7>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,7>,country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<array$<f32,3>,7>,core::ops::range::Range<u32>,country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0<array$<array$<f32,3>,7>,tuple$<>,core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<array$<f32,3>,7>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,7>,country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<f32,3>,7>,core::ops::range::Range<u32>,country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0<array$<array$<f32,3>,7>,tuple$<>,core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<array$<f32,3>,7>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,7>,country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0> > > > > >; module=1610, inline_record=80088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::generate_plaster_mesh
// 信息 ID: 2e5255c9548d31c85812a8d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=1553, inline_record=24844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::generate_roof_floor_meshes
// 信息 ID: 4dcd2c708e805d7fd7ace4c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=1553, inline_record=21828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::generate_roof_floor_meshes::closure$0
// 信息 ID: ec0f30581f6fd248dd4889f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_mesh_atlases::generate_roof_floor_meshes", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_mesh_atlases::generate_roof_floor_meshes::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,2>,country_core::startup::load_mesh_atlases::generate_roof_floor_meshes::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,2>,country_core::startup::load_mesh_atlases::generate_roof_floor_meshes::closure_env$0> >; module=1585, inline_record=300420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::generate_stairs_floor_mesh
// 信息 ID: 6334ac6c0eb93bc62e3ae942
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=1553, inline_record=26504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::impl$0::default
// 信息 ID: 0dd5a008b8278ac4ee20cc83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=69188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::load_mesh_atlases
// 信息 ID: ae4b0a9a72d8648f839204c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 84e0afc3a536305c1906f44b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >),void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >)> > > @ 0x14d3f69 -> 0x129f400 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >),void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >)> > @ 0x15f3a07 -> 0x129f400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
// 信息 ID: f28158b9265f8a81c88eaf84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::load_mesh_atlases::load_nani_meshes -> ["country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0x129f5d2 -> 0x15f7190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
// 信息 ID: 030e7b791c1fdd3a5e399f52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::load_mesh_atlases::load_nani_meshes -> ["country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::reload_mesh_atlases @ 0x12a14e0 -> 0x15f5b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::reload_mesh_atlases
// 信息 ID: feb95a67df2833454fbe798f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 18374ebcff7a4d70bf3cb802: void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)> > > @ 0x14c909a -> 0x12a0820 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)> > @ 0x15f2fb8 -> 0x12a0820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure$0
// 信息 ID: 848fe7d339ae632e6c07b3c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_mesh_atlases::reload_mesh_atlases", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::reload_mesh_atlases; module=1553, inline_record=33868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure$1
// 信息 ID: c466c0ce70d2b6e39c466b91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_mesh_atlases::reload_mesh_atlases", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1 <- country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>; module=1631, inline_record=144516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
