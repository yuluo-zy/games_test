// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/color_menu.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 6b0d29a1354a63b383c8b629c484da9951562ed53f461c00502069a8e0751779
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: color_menu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2709
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_button_animations
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2709,"record_offset":69296,"rva":35128096,"va":5403837216}
// 编译实例 RVAs: 0x2180320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7355368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403837216,"raw_hex":"e8e83b7000","size":5,"soft_fail":false,"target_va":5411192736,"va":5403837363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20540528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403837216,"raw_hex":"e89093c6fe","size":5,"soft_fail":false,"target_va":5383297200,"va":5403837723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":69708,"parent_record_offset":69296,"ancestor_record_offset":69296,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":69960,"parent_record_offset":69296,"ancestor_record_offset":69296,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":69464,"parent_record_offset":69296,"ancestor_record_offset":69296,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2709,"record_offset":69676,"parent_record_offset":69296,"ancestor_record_offset":69296,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2709,"record_offset":72448,"parent_record_offset":69296,"ancestor_record_offset":69296,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":72788,"parent_record_offset":69296,"ancestor_record_offset":69296,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":72564,"parent_record_offset":69296,"ancestor_record_offset":69296,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":69904,"parent_record_offset":69296,"ancestor_record_offset":69296,"inlinee":21757,"source_line":1343,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":2709,"record_offset":69764,"parent_record_offset":69296,"ancestor_record_offset":69296,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":71420,"parent_record_offset":71396,"ancestor_record_offset":69296,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":71468,"parent_record_offset":71444,"ancestor_record_offset":69296,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":71444,"parent_record_offset":71420,"ancestor_record_offset":69296,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":71396,"parent_record_offset":71372,"ancestor_record_offset":69296,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":71372,"parent_record_offset":71348,"ancestor_record_offset":69296,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":70272,"parent_record_offset":70248,"ancestor_record_offset":69296,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":70296,"parent_record_offset":70272,"ancestor_record_offset":69296,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":71348,"parent_record_offset":71324,"ancestor_record_offset":69296,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":71096,"parent_record_offset":71072,"ancestor_record_offset":69296,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":71324,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":70912,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":71576,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":70828,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":69732,"parent_record_offset":69708,"ancestor_record_offset":69296,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":71652,"parent_record_offset":71628,"ancestor_record_offset":69296,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":70464,"parent_record_offset":70440,"ancestor_record_offset":69296,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":70356,"parent_record_offset":70332,"ancestor_record_offset":69296,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":71628,"parent_record_offset":71604,"ancestor_record_offset":69296,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":71676,"parent_record_offset":71652,"ancestor_record_offset":69296,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":71020,"parent_record_offset":70996,"ancestor_record_offset":69296,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":71072,"parent_record_offset":71044,"ancestor_record_offset":69296,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":70440,"parent_record_offset":70332,"ancestor_record_offset":69296,"inlinee":18910,"source_line":2219,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":70492,"parent_record_offset":70464,"ancestor_record_offset":69296,"inlinee":18912,"source_line":2224,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$95::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":71604,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":18908,"source_line":2520,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":69600,"parent_record_offset":69488,"ancestor_record_offset":69296,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":69984,"parent_record_offset":69960,"ancestor_record_offset":69296,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2709,"record_offset":70996,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":18906,"source_line":182,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2709,"record_offset":70332,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":18905,"source_line":195,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":70564,"parent_record_offset":70540,"ancestor_record_offset":69296,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":70540,"parent_record_offset":70516,"ancestor_record_offset":69296,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":70404,"parent_record_offset":70380,"ancestor_record_offset":69296,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":70380,"parent_record_offset":70356,"ancestor_record_offset":69296,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":70516,"parent_record_offset":70492,"ancestor_record_offset":69296,"inlinee":18913,"source_line":510,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":70248,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":71700,"parent_record_offset":71676,"ancestor_record_offset":69296,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":70164,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":69540,"parent_record_offset":69516,"ancestor_record_offset":69296,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":69564,"parent_record_offset":69540,"ancestor_record_offset":69296,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":69488,"parent_record_offset":69464,"ancestor_record_offset":69296,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":69516,"parent_record_offset":69488,"ancestor_record_offset":69296,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":2709,"record_offset":69636,"parent_record_offset":69600,"ancestor_record_offset":69296,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2709,"record_offset":71872,"parent_record_offset":71844,"ancestor_record_offset":69296,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2709,"record_offset":69872,"parent_record_offset":69764,"ancestor_record_offset":69296,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2709,"record_offset":69816,"parent_record_offset":69764,"ancestor_record_offset":69296,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2709,"record_offset":69928,"parent_record_offset":69904,"ancestor_record_offset":69296,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2709,"record_offset":69788,"parent_record_offset":69764,"ancestor_record_offset":69296,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2709,"record_offset":70648,"parent_record_offset":70588,"ancestor_record_offset":69296,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":70672,"parent_record_offset":70648,"ancestor_record_offset":69296,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2709,"record_offset":71896,"parent_record_offset":71872,"ancestor_record_offset":69296,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2709,"record_offset":70132,"parent_record_offset":70072,"ancestor_record_offset":69296,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2709,"record_offset":70104,"parent_record_offset":70072,"ancestor_record_offset":69296,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2709,"record_offset":70188,"parent_record_offset":70164,"ancestor_record_offset":69296,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2709,"record_offset":71520,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2709,"record_offset":70212,"parent_record_offset":70188,"ancestor_record_offset":69296,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2709,"record_offset":71544,"parent_record_offset":71520,"ancestor_record_offset":69296,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2709,"record_offset":70072,"parent_record_offset":69984,"ancestor_record_offset":69296,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":72816,"parent_record_offset":72788,"ancestor_record_offset":69296,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":72900,"parent_record_offset":72788,"ancestor_record_offset":69296,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2709,"record_offset":71724,"parent_record_offset":71700,"ancestor_record_offset":69296,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2709,"record_offset":408,"rva":35093696,"va":5403802816}
// 编译实例 RVAs: 0x2177cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=32
//     evidence={"assembly":"\tcallq\t-28131507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e84dbf52fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5403810190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=21
//     evidence={"assembly":"\tcallq\t-28131753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e857be52fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403810452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=32
//     target_aliases=1699 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35087979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e89599e8fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5403810166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=8
//     target_aliases=12 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25700641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8dfd677fe","size":5,"soft_fail":false,"target_va":5378120480,"va":5403821116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b30
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25704968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8f8c577fe","size":5,"soft_fail":false,"target_va":5378120496,"va":5403825459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25697060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8dce477fe","size":5,"soft_fail":false,"target_va":5378126944,"va":5403823999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb520
//     owner=unresolved_namespace; count=8
//     target_aliases=12 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25687009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e81f0c78fe","size":5,"soft_fail":false,"target_va":5378127136,"va":5403814140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a3e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25018858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8163e82fe","size":5,"soft_fail":false,"target_va":5378817680,"va":5403836533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142894170
//     owner=unresolved_namespace; count=16
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7450043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8bbad7100","size":5,"soft_fail":false,"target_va":5411258736,"va":5403808688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12532158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8be39bf00","size":5,"soft_fail":false,"target_va":5416350384,"va":5403818221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12531171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8e335bf00","size":5,"soft_fail":false,"target_va":5416350720,"va":5403819544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=26
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12530368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8c032bf00","size":5,"soft_fail":false,"target_va":5416350944,"va":5403820571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12544659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8936abf00","size":5,"soft_fail":false,"target_va":5416351056,"va":5403806392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13243035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e89b12ca00","size":5,"soft_fail":false,"target_va":5417057056,"va":5403814016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12788039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e84721c300","size":5,"soft_fail":false,"target_va":5416620769,"va":5403832725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t-19013681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8cfdfddfe","size":5,"soft_fail":false,"target_va":5384794864,"va":5403808540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t12787852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e88c20c300","size":5,"soft_fail":false,"target_va":5416620794,"va":5403832937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t12788858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e87a24c300","size":5,"soft_fail":false,"target_va":5416621600,"va":5403832737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t12788901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8a524c300","size":5,"soft_fail":false,"target_va":5416621762,"va":5403832856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12789669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8a527c300","size":5,"soft_fail":false,"target_va":5416622560,"va":5403832886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12788789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e83524c300","size":5,"soft_fail":false,"target_va":5416621632,"va":5403832838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25400963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e87d697cfe","size":5,"soft_fail":false,"target_va":5378404688,"va":5403805646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_icon::ColorMenuBrightnessValue::to_f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10208613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e89b3a64ff","size":5,"soft_fail":false,"target_va":5393612032,"va":5403820640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23267735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e869f69cfe","size":5,"soft_fail":false,"target_va":5380552224,"va":5403819954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::try_start_input_recording
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23267624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8d8f69cfe","size":5,"soft_fail":false,"target_va":5380551856,"va":5403819475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_wall_state::FreehandHeightType::toggle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22169650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8ceb7adfe","size":5,"soft_fail":false,"target_va":5381642112,"va":5403811757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22921769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8d73da2fe","size":5,"soft_fail":false,"target_va":5380903536,"va":5403825300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22923359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8a137a2fe","size":5,"soft_fail":false,"target_va":5380902448,"va":5403825802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=package_namespace_candidate; count=15
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25688303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8110778fe","size":5,"soft_fail":false,"target_va":5378120752,"va":5403809050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=15
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25687875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8bd0878fe","size":5,"soft_fail":false,"target_va":5378120848,"va":5403808718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23782947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8dd1995fe","size":5,"soft_fail":false,"target_va":5380025344,"va":5403808286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_click
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23783079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8591995fe","size":5,"soft_fail":false,"target_va":5380025376,"va":5403808450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23783242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8b61895fe","size":5,"soft_fail":false,"target_va":5380025136,"va":5403808373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23783879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8391695fe","size":5,"soft_fail":false,"target_va":5380024784,"va":5403808658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23793993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8b7ee94fe","size":5,"soft_fail":false,"target_va":5380024576,"va":5403818564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23796703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e821e494fe","size":5,"soft_fail":false,"target_va":5380024896,"va":5403821594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::angle_range::AngleRange::clamp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7581910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e8d6b07300","size":5,"soft_fail":false,"target_va":5411403184,"va":5403821269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::cartesian_to_polar
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20544986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e82682c6fe","size":5,"soft_fail":false,"target_va":5383276240,"va":5403821221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::polar_to_cartesian
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20544971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403802816,"raw_hex":"e83582c6fe","size":5,"soft_fail":false,"target_va":5383276320,"va":5403821286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2709,"record_offset":15652,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2709,"record_offset":32080,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":31092,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":4268,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=51
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2709,"record_offset":1136,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2709,"record_offset":31156,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":4672,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":11799,"source_line":1524,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":672,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2709,"record_offset":36024,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2709,"record_offset":17240,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2709,"record_offset":15736,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2709,"record_offset":1536,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2709,"record_offset":4504,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":44220,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":4324,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":14503,"source_line":1754,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":34564,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":29452,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":45844,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":4476,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":39300,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":15180,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2709,"record_offset":18344,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2709,"record_offset":63396,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2709,"record_offset":4588,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2709,"record_offset":37748,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":2709,"record_offset":43388,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":13205,"source_line":87,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":2709,"record_offset":43352,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::impl$68::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":2709,"record_offset":4388,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":21924,"source_line":359,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":34784,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":34724,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":34636,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] system_color::color_menu::color_menu::closure$0
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":2709,"record_offset":15764,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":108341,"source_line":289,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] system_color::color_menu::color_menu::closure$1
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":2709,"record_offset":15916,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":108342,"source_line":300,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] system_color::color_menu::color_menu::closure$2
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":2709,"record_offset":16116,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":108343,"source_line":326,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] system_color::material_color_mapping::stone_to_wood
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/material_color_mapping.rs
//     evidence={"module":2709,"record_offset":39428,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":108233,"source_line":3,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] system_color::material_color_mapping::wood_to_stair
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/material_color_mapping.rs
//     evidence={"module":2709,"record_offset":39492,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":108234,"source_line":40,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] system_color::material_color_mapping::wood_to_wall
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/material_color_mapping.rs
//     evidence={"module":2709,"record_offset":49316,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":108347,"source_line":23,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2709,"record_offset":17536,"parent_record_offset":17512,"ancestor_record_offset":408,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2709,"record_offset":17560,"parent_record_offset":17536,"ancestor_record_offset":408,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2709,"record_offset":18520,"parent_record_offset":18496,"ancestor_record_offset":408,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2709,"record_offset":18544,"parent_record_offset":18520,"ancestor_record_offset":408,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2709,"record_offset":17584,"parent_record_offset":17560,"ancestor_record_offset":408,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2709,"record_offset":18568,"parent_record_offset":18544,"ancestor_record_offset":408,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2709,"record_offset":17512,"parent_record_offset":17484,"ancestor_record_offset":408,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2709,"record_offset":18496,"parent_record_offset":18440,"ancestor_record_offset":408,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2709,"record_offset":32148,"parent_record_offset":32112,"ancestor_record_offset":408,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2709,"record_offset":32112,"parent_record_offset":32080,"ancestor_record_offset":408,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2709,"record_offset":15700,"parent_record_offset":15676,"ancestor_record_offset":408,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":408,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":17436,"parent_record_offset":17412,"ancestor_record_offset":408,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":18468,"parent_record_offset":18440,"ancestor_record_offset":408,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":18440,"parent_record_offset":18416,"ancestor_record_offset":408,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":408,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":408,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":408,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":17460,"parent_record_offset":17436,"ancestor_record_offset":408,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":18416,"parent_record_offset":18392,"ancestor_record_offset":408,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2709,"record_offset":17336,"parent_record_offset":17312,"ancestor_record_offset":408,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2709,"record_offset":17360,"parent_record_offset":17336,"ancestor_record_offset":408,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2709,"record_offset":17384,"parent_record_offset":17360,"ancestor_record_offset":408,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2709,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":408,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2709,"record_offset":17288,"parent_record_offset":17264,"ancestor_record_offset":408,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2709,"record_offset":17264,"parent_record_offset":17240,"ancestor_record_offset":408,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":1336,"parent_record_offset":1292,"ancestor_record_offset":408,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":408,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":408,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":408,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":408,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":1256,"parent_record_offset":1212,"ancestor_record_offset":408,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":1292,"parent_record_offset":1256,"ancestor_record_offset":408,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":17412,"parent_record_offset":17384,"ancestor_record_offset":408,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":408,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":408,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":3524,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":2856,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":2940,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":3776,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":14436,"parent_record_offset":4704,"ancestor_record_offset":408,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":2772,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=74
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":1496,"parent_record_offset":1168,"ancestor_record_offset":408,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=49
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":15320,"parent_record_offset":15296,"ancestor_record_offset":408,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=38
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":408,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":15968,"parent_record_offset":15916,"ancestor_record_offset":408,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2709,"record_offset":14208,"parent_record_offset":4704,"ancestor_record_offset":408,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2709,"record_offset":14232,"parent_record_offset":14208,"ancestor_record_offset":408,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2709,"record_offset":14340,"parent_record_offset":14316,"ancestor_record_offset":408,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2709,"record_offset":14364,"parent_record_offset":14340,"ancestor_record_offset":408,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$3::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2709,"record_offset":4440,"parent_record_offset":4416,"ancestor_record_offset":408,"inlinee":28791,"source_line":410,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2709,"record_offset":1168,"parent_record_offset":1136,"ancestor_record_offset":408,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2709,"record_offset":31248,"parent_record_offset":31156,"ancestor_record_offset":408,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=55
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2709,"record_offset":1212,"parent_record_offset":1168,"ancestor_record_offset":408,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":408,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$128::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":13944,"parent_record_offset":4704,"ancestor_record_offset":408,"inlinee":108339,"source_line":2520,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":5412,"parent_record_offset":5380,"ancestor_record_offset":408,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":5684,"parent_record_offset":5380,"ancestor_record_offset":408,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":5708,"parent_record_offset":5684,"ancestor_record_offset":408,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":45628,"parent_record_offset":44304,"ancestor_record_offset":408,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":45684,"parent_record_offset":45656,"ancestor_record_offset":408,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":408,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":1976,"parent_record_offset":1948,"ancestor_record_offset":408,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":408,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":408,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":5380,"parent_record_offset":4772,"ancestor_record_offset":408,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":5440,"parent_record_offset":5412,"ancestor_record_offset":408,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":3804,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":824,"parent_record_offset":704,"ancestor_record_offset":408,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":856,"parent_record_offset":824,"ancestor_record_offset":408,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":408,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":1572,"parent_record_offset":1536,"ancestor_record_offset":408,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$15::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2709,"record_offset":4772,"parent_record_offset":4704,"ancestor_record_offset":408,"inlinee":108337,"source_line":182,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2709,"record_offset":3032,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2709,"record_offset":1948,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2709,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":408,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":408,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":2068,"parent_record_offset":2040,"ancestor_record_offset":408,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":408,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":408,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":408,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":2428,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":2012,"parent_record_offset":1976,"ancestor_record_offset":408,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":408,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":1864,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":408,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":1780,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2709,"record_offset":2364,"parent_record_offset":1976,"ancestor_record_offset":408,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2709,"record_offset":2308,"parent_record_offset":1976,"ancestor_record_offset":408,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":408,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":2709,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":408,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":408,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":44276,"parent_record_offset":44252,"ancestor_record_offset":408,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":4704,"parent_record_offset":4672,"ancestor_record_offset":408,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":788,"parent_record_offset":764,"ancestor_record_offset":408,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":704,"parent_record_offset":672,"ancestor_record_offset":408,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":740,"parent_record_offset":704,"ancestor_record_offset":408,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":59580,"parent_record_offset":59552,"ancestor_record_offset":408,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2709,"record_offset":4072,"parent_record_offset":4044,"ancestor_record_offset":408,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":2709,"record_offset":14472,"parent_record_offset":14436,"ancestor_record_offset":408,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2709,"record_offset":59052,"parent_record_offset":59024,"ancestor_record_offset":408,"inlinee":17499,"source_line":1878,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2709,"record_offset":36112,"parent_record_offset":36084,"ancestor_record_offset":408,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2709,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":408,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2709,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":408,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2709,"record_offset":34932,"parent_record_offset":34904,"ancestor_record_offset":408,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2709,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":408,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2709,"record_offset":34752,"parent_record_offset":34724,"ancestor_record_offset":408,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2709,"record_offset":15824,"parent_record_offset":15800,"ancestor_record_offset":408,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2709,"record_offset":15880,"parent_record_offset":15856,"ancestor_record_offset":408,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2709,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":408,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":408,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":408,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":14316,"parent_record_offset":14232,"ancestor_record_offset":408,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":4352,"parent_record_offset":4324,"ancestor_record_offset":408,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":408,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":2180,"parent_record_offset":2040,"ancestor_record_offset":408,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":15676,"parent_record_offset":15652,"ancestor_record_offset":408,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":14408,"parent_record_offset":4704,"ancestor_record_offset":408,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":16148,"parent_record_offset":16116,"ancestor_record_offset":408,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":4208,"parent_record_offset":1536,"ancestor_record_offset":408,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":4236,"parent_record_offset":1536,"ancestor_record_offset":408,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2709,"record_offset":17640,"parent_record_offset":17384,"ancestor_record_offset":408,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2709,"record_offset":17664,"parent_record_offset":17640,"ancestor_record_offset":408,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2709,"record_offset":18368,"parent_record_offset":18344,"ancestor_record_offset":408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2709,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":408,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2709,"record_offset":1748,"parent_record_offset":1688,"ancestor_record_offset":408,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2709,"record_offset":1720,"parent_record_offset":1688,"ancestor_record_offset":408,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2709,"record_offset":1456,"parent_record_offset":1292,"ancestor_record_offset":408,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2709,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":408,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2709,"record_offset":2624,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2709,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":408,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2709,"record_offset":2260,"parent_record_offset":2228,"ancestor_record_offset":408,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2709,"record_offset":1060,"parent_record_offset":888,"ancestor_record_offset":408,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2709,"record_offset":1688,"parent_record_offset":1596,"ancestor_record_offset":408,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":2709,"record_offset":59024,"parent_record_offset":58988,"ancestor_record_offset":408,"inlinee":14084,"source_line":30,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::impl$22::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":2709,"record_offset":36172,"parent_record_offset":36144,"ancestor_record_offset":408,"inlinee":14661,"source_line":28,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":2709,"record_offset":36052,"parent_record_offset":36024,"ancestor_record_offset":408,"inlinee":34507,"source_line":70,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":2709,"record_offset":36236,"parent_record_offset":36208,"ancestor_record_offset":408,"inlinee":34509,"source_line":140,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":2709,"record_offset":48676,"parent_record_offset":48648,"ancestor_record_offset":408,"inlinee":27145,"source_line":147,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$113::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2709,"record_offset":39376,"parent_record_offset":39332,"ancestor_record_offset":408,"inlinee":14663,"source_line":364,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$89::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2709,"record_offset":37424,"parent_record_offset":37396,"ancestor_record_offset":408,"inlinee":27213,"source_line":323,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":2709,"record_offset":47740,"parent_record_offset":47712,"ancestor_record_offset":408,"inlinee":13211,"source_line":28,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$27::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":2709,"record_offset":48208,"parent_record_offset":48180,"ancestor_record_offset":408,"inlinee":13215,"source_line":50,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$35::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":2709,"record_offset":49196,"parent_record_offset":49148,"ancestor_record_offset":408,"inlinee":13207,"source_line":58,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":2709,"record_offset":43320,"parent_record_offset":43292,"ancestor_record_offset":408,"inlinee":13205,"source_line":87,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$53::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":2709,"record_offset":45904,"parent_record_offset":45876,"ancestor_record_offset":408,"inlinee":13213,"source_line":184,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::impl$49::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":2709,"record_offset":59120,"parent_record_offset":59092,"ancestor_record_offset":408,"inlinee":108350,"source_line":213,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2709,"record_offset":14520,"parent_record_offset":14496,"ancestor_record_offset":408,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2709,"record_offset":14544,"parent_record_offset":14520,"ancestor_record_offset":408,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2709,"record_offset":14496,"parent_record_offset":14472,"ancestor_record_offset":408,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":34872,"parent_record_offset":34812,"ancestor_record_offset":408,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":15856,"parent_record_offset":15764,"ancestor_record_offset":408,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":15800,"parent_record_offset":15764,"ancestor_record_offset":408,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2709,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":408,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2709,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":408,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2709,"record_offset":34904,"parent_record_offset":34872,"ancestor_record_offset":408,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] system_color::color_menu::color_menu::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":2709,"record_offset":37884,"parent_record_offset":37748,"ancestor_record_offset":408,"inlinee":108344,"source_line":888,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] system_color::color_menu::color_menu::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":2709,"record_offset":39268,"parent_record_offset":39132,"ancestor_record_offset":408,"inlinee":108345,"source_line":923,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] system_color::color_menu::color_menu::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":2709,"record_offset":44252,"parent_record_offset":44220,"ancestor_record_offset":408,"inlinee":108346,"source_line":1096,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] system_color::color_menu::color_menu::closure$8
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":2709,"record_offset":51368,"parent_record_offset":51216,"ancestor_record_offset":408,"inlinee":108348,"source_line":1241,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 289
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A735
// 首条位置: {"module":2709,"inlinee":108341}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":15856,"parent_record_offset":15764,"ancestor_record_offset":15764,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2709,"record_offset":15800,"parent_record_offset":15764,"ancestor_record_offset":15764,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2709,"record_offset":15824,"parent_record_offset":15800,"ancestor_record_offset":15764,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2709,"record_offset":15880,"parent_record_offset":15856,"ancestor_record_offset":15764,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 300
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A736
// 首条位置: {"module":2709,"inlinee":108342}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":15968,"parent_record_offset":15916,"ancestor_record_offset":15916,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":16024,"parent_record_offset":15916,"ancestor_record_offset":15916,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2709,"record_offset":15992,"parent_record_offset":15968,"ancestor_record_offset":15916,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 326
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A737
// 首条位置: {"module":2709,"inlinee":108343}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":16148,"parent_record_offset":16116,"ancestor_record_offset":16116,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 888
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A738
// 首条位置: {"module":2709,"inlinee":108344}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 923
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A739
// 首条位置: {"module":2709,"inlinee":108345}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1096
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A73A
// 首条位置: {"module":2709,"inlinee":108346}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":44276,"parent_record_offset":44252,"ancestor_record_offset":44252,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":44936,"parent_record_offset":44908,"ancestor_record_offset":44252,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":44992,"parent_record_offset":44964,"ancestor_record_offset":44252,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":44964,"parent_record_offset":44936,"ancestor_record_offset":44252,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":44908,"parent_record_offset":44880,"ancestor_record_offset":44252,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":44880,"parent_record_offset":44844,"ancestor_record_offset":44252,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":44844,"parent_record_offset":44816,"ancestor_record_offset":44252,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":44612,"parent_record_offset":44304,"ancestor_record_offset":44252,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2709,"record_offset":44356,"parent_record_offset":44304,"ancestor_record_offset":44252,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2709,"record_offset":44384,"parent_record_offset":44356,"ancestor_record_offset":44252,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2709,"record_offset":44512,"parent_record_offset":44480,"ancestor_record_offset":44252,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2709,"record_offset":44536,"parent_record_offset":44512,"ancestor_record_offset":44252,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":45656,"parent_record_offset":45628,"ancestor_record_offset":44252,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":45080,"parent_record_offset":44304,"ancestor_record_offset":44252,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":45108,"parent_record_offset":45080,"ancestor_record_offset":44252,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":45628,"parent_record_offset":44304,"ancestor_record_offset":44252,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":45684,"parent_record_offset":45656,"ancestor_record_offset":44252,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2709,"record_offset":45572,"parent_record_offset":45544,"ancestor_record_offset":44252,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":45224,"parent_record_offset":45196,"ancestor_record_offset":44252,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":45196,"parent_record_offset":45168,"ancestor_record_offset":44252,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":45376,"parent_record_offset":45348,"ancestor_record_offset":44252,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":45168,"parent_record_offset":45136,"ancestor_record_offset":44252,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":45136,"parent_record_offset":45108,"ancestor_record_offset":44252,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":45712,"parent_record_offset":45684,"ancestor_record_offset":44252,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":44816,"parent_record_offset":44304,"ancestor_record_offset":44252,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2709,"record_offset":45544,"parent_record_offset":45520,"ancestor_record_offset":44252,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":45520,"parent_record_offset":45492,"ancestor_record_offset":44252,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":44304,"parent_record_offset":44276,"ancestor_record_offset":44252,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":2709,"record_offset":44648,"parent_record_offset":44612,"ancestor_record_offset":44252,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2709,"record_offset":45280,"parent_record_offset":45252,"ancestor_record_offset":44252,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":45252,"parent_record_offset":45224,"ancestor_record_offset":44252,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":45304,"parent_record_offset":45280,"ancestor_record_offset":44252,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":44480,"parent_record_offset":44384,"ancestor_record_offset":44252,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":45348,"parent_record_offset":45196,"ancestor_record_offset":44252,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":44580,"parent_record_offset":44304,"ancestor_record_offset":44252,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2709,"record_offset":44764,"parent_record_offset":44728,"ancestor_record_offset":44252,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2709,"record_offset":44416,"parent_record_offset":44384,"ancestor_record_offset":44252,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2709,"record_offset":44444,"parent_record_offset":44416,"ancestor_record_offset":44252,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2709,"record_offset":45440,"parent_record_offset":45404,"ancestor_record_offset":44252,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2709,"record_offset":44700,"parent_record_offset":44676,"ancestor_record_offset":44252,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2709,"record_offset":44728,"parent_record_offset":44700,"ancestor_record_offset":44252,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2709,"record_offset":44676,"parent_record_offset":44648,"ancestor_record_offset":44252,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2709,"record_offset":45748,"parent_record_offset":45712,"ancestor_record_offset":44252,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2709,"record_offset":45404,"parent_record_offset":45376,"ancestor_record_offset":44252,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A73C
// 首条位置: {"module":2709,"inlinee":108348}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::delete_color_icon_entities
// 记录形式: compiled_procedure; instances=1
// 源码行: 1519
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2709,"record_offset":73076,"rva":35129728,"va":5403838848}
// 编译实例 RVAs: 0x2180980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-19420191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403838848,"raw_hex":"e8e1abd7fe","size":5,"soft_fail":false,"target_va":5384418976,"va":5403839162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":73652,"parent_record_offset":73076,"ancestor_record_offset":73076,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":2709,"record_offset":76404,"parent_record_offset":73076,"ancestor_record_offset":73076,"inlinee":10329,"source_line":442,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":73220,"parent_record_offset":73076,"ancestor_record_offset":73076,"inlinee":10312,"source_line":673,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":73484,"parent_record_offset":73460,"ancestor_record_offset":73076,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":73532,"parent_record_offset":73508,"ancestor_record_offset":73076,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2709,"record_offset":73508,"parent_record_offset":73484,"ancestor_record_offset":73076,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":73460,"parent_record_offset":73436,"ancestor_record_offset":73076,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":73436,"parent_record_offset":73404,"ancestor_record_offset":73076,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":73980,"parent_record_offset":73956,"ancestor_record_offset":73076,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":74004,"parent_record_offset":73980,"ancestor_record_offset":73076,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2709,"record_offset":73404,"parent_record_offset":73380,"ancestor_record_offset":73076,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":75884,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":75128,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":75212,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":76136,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10336,"source_line":338,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":76164,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2709,"record_offset":75044,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":74832,"parent_record_offset":74808,"ancestor_record_offset":73076,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":75328,"parent_record_offset":75304,"ancestor_record_offset":73076,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":74064,"parent_record_offset":74040,"ancestor_record_offset":73076,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":74808,"parent_record_offset":74784,"ancestor_record_offset":73076,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":74856,"parent_record_offset":74832,"ancestor_record_offset":73076,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2709,"record_offset":74784,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":73356,"parent_record_offset":73244,"ancestor_record_offset":73076,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":73380,"parent_record_offset":73356,"ancestor_record_offset":73076,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2709,"record_offset":73688,"parent_record_offset":73652,"ancestor_record_offset":73076,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2709,"record_offset":75304,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2709,"record_offset":74040,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2709,"record_offset":74536,"parent_record_offset":74512,"ancestor_record_offset":73076,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":74180,"parent_record_offset":74152,"ancestor_record_offset":73076,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":74152,"parent_record_offset":74124,"ancestor_record_offset":73076,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2709,"record_offset":74360,"parent_record_offset":74336,"ancestor_record_offset":73076,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":74588,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":74124,"parent_record_offset":74092,"ancestor_record_offset":73076,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":74092,"parent_record_offset":74064,"ancestor_record_offset":73076,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":73956,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":76288,"parent_record_offset":76264,"ancestor_record_offset":73076,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":73872,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2709,"record_offset":74512,"parent_record_offset":74488,"ancestor_record_offset":73076,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2709,"record_offset":74488,"parent_record_offset":74464,"ancestor_record_offset":73076,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":73244,"parent_record_offset":73220,"ancestor_record_offset":73076,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":73296,"parent_record_offset":73272,"ancestor_record_offset":73076,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":73320,"parent_record_offset":73296,"ancestor_record_offset":73076,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2709,"record_offset":73272,"parent_record_offset":73244,"ancestor_record_offset":73076,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2709,"record_offset":74880,"parent_record_offset":74856,"ancestor_record_offset":73076,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2709,"record_offset":74272,"parent_record_offset":74208,"ancestor_record_offset":73076,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":74296,"parent_record_offset":74272,"ancestor_record_offset":73076,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2709,"record_offset":74336,"parent_record_offset":74152,"ancestor_record_offset":73076,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2709,"record_offset":73600,"parent_record_offset":73576,"ancestor_record_offset":73076,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2709,"record_offset":73840,"parent_record_offset":73780,"ancestor_record_offset":73076,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2709,"record_offset":73812,"parent_record_offset":73780,"ancestor_record_offset":73076,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2709,"record_offset":73896,"parent_record_offset":73872,"ancestor_record_offset":73076,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2709,"record_offset":76080,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2709,"record_offset":73920,"parent_record_offset":73896,"ancestor_record_offset":73076,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2709,"record_offset":74416,"parent_record_offset":74384,"ancestor_record_offset":73076,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2709,"record_offset":73576,"parent_record_offset":73404,"ancestor_record_offset":73076,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2709,"record_offset":73780,"parent_record_offset":73688,"ancestor_record_offset":73076,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2709,"record_offset":76320,"parent_record_offset":76288,"ancestor_record_offset":73076,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02709.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2709,"record_offset":74384,"parent_record_offset":74360,"ancestor_record_offset":73076,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02709.jsonl.gz"}
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
// 完整结构及原始类型记录: color_menu.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: color_menu.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: color_menu.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_button_animations
// 信息 ID: 6128bd4ef7aff53f08b80387
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 700d28d21d1bc8ddd520f542: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x10bf530 -> 0x2180320 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x21601e1 -> 0x2180320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu
// 信息 ID: 1c4949fb57b4dd432ac9e7fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] bd8e8c3933714649588af257: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >)> @ 0x10bf341 -> 0x2177cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >)> > @ 0x215aa2b -> 0x2177cc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$0
// 信息 ID: 99e16daea698b52a52909cb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=15764; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=16292; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=16756; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=18740; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=19268; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=19800; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=20264; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=22328; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$1
// 信息 ID: ca6fc3b4f54ec2f32038276c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=15916; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=16444; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=16908; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=18892; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=19420; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=19952; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=20416; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=22480; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$2
// 信息 ID: c6d900c7ef4f7504f58fd403
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=16116; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=17108; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=17172; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=19088; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=19620; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=20616; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=20680; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=22676; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$4
// 信息 ID: 75c87c675841c217361369b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=37884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$5
// 信息 ID: 04715d0836c1b48c936f9d17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=39268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$6
// 信息 ID: a8a7e0e7e9f853c3501b5e9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=44252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$8
// 信息 ID: 5031387627e494fa84212589
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=51368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::delete_color_icon_entities
// 信息 ID: f3d6cb437731f7dc5db27982
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b72ca01a2f46e826efb23874: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)> @ 0x10bdaf3 -> 0x2180980 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0x215d0a2 -> 0x2180980 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0x2168422 -> 0x2180980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
