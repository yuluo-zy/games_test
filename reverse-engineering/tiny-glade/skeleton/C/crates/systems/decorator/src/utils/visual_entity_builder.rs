// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/utils/visual_entity_builder.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e572c9582d739402dc73f617e6b8f8d9e146015e9005f73de59310889d64c130
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: visual_entity_builder.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 431, 446, 447, 448, 449, 450, 451, 454, 455, 467, 468, 469, 470, 471, 472, 476, 1542, 1543, 1545
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1398C
// 首条位置: {"module":1545,"inlinee":80268}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1545,"record_offset":24244,"parent_record_offset":24168,"ancestor_record_offset":24168,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":1545,"record_offset":24764,"parent_record_offset":24736,"ancestor_record_offset":24168,"inlinee":6914,"source_line":236,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1545,"record_offset":24568,"parent_record_offset":24536,"ancestor_record_offset":24168,"inlinee":80272,"source_line":150,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$35::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1545,"record_offset":24360,"parent_record_offset":24332,"ancestor_record_offset":24168,"inlinee":80270,"source_line":158,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$39::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1545,"record_offset":24496,"parent_record_offset":24464,"ancestor_record_offset":24168,"inlinee":80274,"source_line":173,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$84::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1545,"record_offset":24632,"parent_record_offset":24600,"ancestor_record_offset":24168,"inlinee":80276,"source_line":342,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$102::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1545,"record_offset":24300,"parent_record_offset":24272,"ancestor_record_offset":24168,"inlinee":9872,"source_line":20,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13971
// 首条位置: {"module":1543,"inlinee":80241}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1543,"record_offset":31460,"parent_record_offset":31060,"ancestor_record_offset":31060,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":31396,"parent_record_offset":31060,"ancestor_record_offset":31060,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":31424,"parent_record_offset":31060,"ancestor_record_offset":31060,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1543,"record_offset":31168,"parent_record_offset":31060,"ancestor_record_offset":31060,"inlinee":27949,"source_line":4,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1543,"record_offset":32292,"parent_record_offset":32224,"ancestor_record_offset":31060,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1543,"record_offset":32316,"parent_record_offset":32292,"ancestor_record_offset":31060,"inlinee":16063,"source_line":143,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1543,"record_offset":33220,"parent_record_offset":33160,"ancestor_record_offset":31060,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1543,"record_offset":32388,"parent_record_offset":32364,"ancestor_record_offset":31060,"inlinee":13096,"source_line":1880,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1543,"record_offset":31516,"parent_record_offset":31488,"ancestor_record_offset":31060,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmpeq_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":31316,"parent_record_offset":31284,"ancestor_record_offset":31060,"inlinee":11124,"source_line":503,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":32740,"parent_record_offset":32692,"ancestor_record_offset":31060,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1543,"record_offset":31544,"parent_record_offset":31516,"ancestor_record_offset":31060,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1543,"record_offset":32364,"parent_record_offset":32340,"ancestor_record_offset":31060,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1543,"record_offset":31576,"parent_record_offset":31544,"ancestor_record_offset":31060,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1543,"record_offset":31488,"parent_record_offset":31460,"ancestor_record_offset":31060,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":1543,"record_offset":32572,"parent_record_offset":32540,"ancestor_record_offset":31060,"inlinee":14084,"source_line":30,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1543,"record_offset":32472,"parent_record_offset":32444,"ancestor_record_offset":31060,"inlinee":80251,"source_line":150,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$37::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1543,"record_offset":31944,"parent_record_offset":31908,"ancestor_record_offset":31060,"inlinee":80245,"source_line":158,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$41::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1543,"record_offset":32224,"parent_record_offset":32196,"ancestor_record_offset":31060,"inlinee":80247,"source_line":173,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$74::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1543,"record_offset":32596,"parent_record_offset":32572,"ancestor_record_offset":31060,"inlinee":80253,"source_line":320,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$87::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1543,"record_offset":32692,"parent_record_offset":32660,"ancestor_record_offset":31060,"inlinee":80255,"source_line":342,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::texture_presets::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/texture_presets.rs
//     evidence={"module":1543,"record_offset":31648,"parent_record_offset":31620,"ancestor_record_offset":31060,"inlinee":80243,"source_line":13,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_render_slots::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_render_slots.rs
//     evidence={"module":1543,"record_offset":32264,"parent_record_offset":32224,"ancestor_record_offset":31060,"inlinee":80249,"source_line":27,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$44::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":31236,"parent_record_offset":31168,"ancestor_record_offset":31060,"inlinee":11119,"source_line":1260,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmpeq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1543,"record_offset":31284,"parent_record_offset":31260,"ancestor_record_offset":31060,"inlinee":11123,"source_line":409,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$2::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1543,"record_offset":31260,"parent_record_offset":31236,"ancestor_record_offset":31060,"inlinee":11121,"source_line":1148,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1543,"record_offset":32032,"parent_record_offset":31944,"ancestor_record_offset":31060,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":31200,"parent_record_offset":31168,"ancestor_record_offset":31060,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
// 记录形式: inline_instance, inlinee_declaration; instances=61
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6844
// 首条位置: {"module":431,"inlinee":26692}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
// 记录形式: inline_instance, inlinee_declaration; instances=37
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6845
// 首条位置: {"module":431,"inlinee":26693}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::has_birdspot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A33
// 首条位置: {"module":455,"inlinee":27187}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A28
// 首条位置: {"module":454,"inlinee":27176}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::lantern_light
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6848
// 首条位置: {"module":431,"inlinee":26696}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader_name
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A32
// 首条位置: {"module":455,"inlinee":27186}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6847
// 首条位置: {"module":431,"inlinee":26695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::albedo_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6846
// 首条位置: {"module":431,"inlinee":26694}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::brick_deformation
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69AC
// 首条位置: {"module":446,"inlinee":27052}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::clothline
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BBE
// 首条位置: {"module":467,"inlinee":27582}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::smoke
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69C0
// 首条位置: {"module":448,"inlinee":27072}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_roof_cloth
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C11
// 首条位置: {"module":476,"inlinee":27665}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_wall_cloth
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C12
// 首条位置: {"module":476,"inlinee":27666}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69AA
// 首条位置: {"module":446,"inlinee":27050}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorPersistentVisualState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 198
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1543,"record_offset":15516,"rva":21616784,"va":5390325904}
// 编译实例 RVAs: 0x149d890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=10
//     target_aliases=1699 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21604253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e86358b6fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390326440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=3
//     target_aliases=17 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20786197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8ebd3c2fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5390328192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=6
//     target_aliases=77 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14499114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8d6c222ff","size":5,"soft_fail":false,"target_va":5375829056,"va":5390328165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408d8260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12349979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8e58d43ff","size":5,"soft_fail":false,"target_va":5377983072,"va":5390333046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12221806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8928245ff","size":5,"soft_fail":false,"target_va":5378112288,"va":5390334089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12216435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e88d9745ff","size":5,"soft_fail":false,"target_va":5378112304,"va":5390328734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12207136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8e0bb45ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5390334075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a88a0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e87f890000","size":5,"soft_fail":false,"target_va":5390370976,"va":5390335772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414aab40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e876a90000","size":5,"soft_fail":false,"target_va":5390379840,"va":5390336453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414c6ee0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t166493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e85d8a0200","size":5,"soft_fail":false,"target_va":5390495456,"va":5390328958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424ef490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17109296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e830110501","size":5,"soft_fail":false,"target_va":5407437968,"va":5390328667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424ef600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17108381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e89d0d0501","size":5,"soft_fail":false,"target_va":5407438336,"va":5390329950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424ef780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17104944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e830000501","size":5,"soft_fail":false,"target_va":5407438720,"va":5390333771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=25
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26018661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e865038d01","size":5,"soft_fail":false,"target_va":5416353040,"va":5390334374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26730423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8b7df9701","size":5,"soft_fail":false,"target_va":5417057040,"va":5390326612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::reserve_entity
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6100264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8d8eaa2ff","size":5,"soft_fail":false,"target_va":5384226192,"va":5390326451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26286284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8cc189101","size":5,"soft_fail":false,"target_va":5416621600,"va":5390335311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26286769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8b11a9101","size":5,"soft_fail":false,"target_va":5416622208,"va":5390335434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26286582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8f6199101","size":5,"soft_fail":false,"target_va":5416622240,"va":5390335653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11057980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8c44457ff","size":5,"soft_fail":false,"target_va":5379270736,"va":5390328711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10806395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e8851b5bff","size":5,"soft_fail":false,"target_va":5379521760,"va":5390328150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11049330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e88e6657ff","size":5,"soft_fail":false,"target_va":5379278928,"va":5390328253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6920321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e87f6796ff","size":5,"soft_fail":false,"target_va":5383408256,"va":5390328572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=15
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7001697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390325904,"raw_hex":"e89f2995ff","size":5,"soft_fail":false,"target_va":5383324736,"va":5390326428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":17064,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":23296,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1543,"record_offset":21120,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":18864,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":22548,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":18892,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":18964,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1543,"record_offset":16824,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":1543,"record_offset":20408,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorId::get_inner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1543,"record_offset":25360,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":26975,"source_line":166,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":15868,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":15812,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":28616,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":16736,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1543,"record_offset":20436,"parent_record_offset":15516,"ancestor_record_offset":15516,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1543,"record_offset":17248,"parent_record_offset":17220,"ancestor_record_offset":15516,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1543,"record_offset":17332,"parent_record_offset":17308,"ancestor_record_offset":15516,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1543,"record_offset":17308,"parent_record_offset":17276,"ancestor_record_offset":15516,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1543,"record_offset":17276,"parent_record_offset":17248,"ancestor_record_offset":15516,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1543,"record_offset":17220,"parent_record_offset":17184,"ancestor_record_offset":15516,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":17156,"parent_record_offset":17128,"ancestor_record_offset":15516,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":17184,"parent_record_offset":17156,"ancestor_record_offset":15516,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":17100,"parent_record_offset":17064,"ancestor_record_offset":15516,"inlinee":11375,"source_line":262,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":17128,"parent_record_offset":17064,"ancestor_record_offset":15516,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1543,"record_offset":16852,"parent_record_offset":16824,"ancestor_record_offset":15516,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1543,"record_offset":15892,"parent_record_offset":15868,"ancestor_record_offset":15516,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1543,"record_offset":20740,"parent_record_offset":20716,"ancestor_record_offset":15516,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":16396,"parent_record_offset":15976,"ancestor_record_offset":15516,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":16304,"parent_record_offset":16248,"ancestor_record_offset":15516,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":16072,"parent_record_offset":16048,"ancestor_record_offset":15516,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":20716,"parent_record_offset":20692,"ancestor_record_offset":15516,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":16104,"parent_record_offset":16020,"ancestor_record_offset":15516,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":16168,"parent_record_offset":15976,"ancestor_record_offset":15516,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1543,"record_offset":16764,"parent_record_offset":16736,"ancestor_record_offset":15516,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":22572,"parent_record_offset":22548,"ancestor_record_offset":15516,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1543,"record_offset":16880,"parent_record_offset":16852,"ancestor_record_offset":15516,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1543,"record_offset":16912,"parent_record_offset":16880,"ancestor_record_offset":15516,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1543,"record_offset":20460,"parent_record_offset":20436,"ancestor_record_offset":15516,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1543,"record_offset":20504,"parent_record_offset":20460,"ancestor_record_offset":15516,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":15976,"parent_record_offset":15868,"ancestor_record_offset":15516,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1543,"record_offset":16424,"parent_record_offset":15976,"ancestor_record_offset":15516,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1543,"record_offset":15940,"parent_record_offset":15916,"ancestor_record_offset":15516,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1543,"record_offset":15916,"parent_record_offset":15892,"ancestor_record_offset":15516,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1543,"record_offset":20692,"parent_record_offset":20460,"ancestor_record_offset":15516,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":15836,"parent_record_offset":15812,"ancestor_record_offset":15516,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1543,"record_offset":16048,"parent_record_offset":16020,"ancestor_record_offset":15516,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1543,"record_offset":16020,"parent_record_offset":15976,"ancestor_record_offset":15516,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 198
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1543,"record_offset":272,"rva":21604720,"va":5390313840}
// 编译实例 RVAs: 0x149a970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=10
//     target_aliases=1699 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21592189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e88387b6fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390314376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=3
//     target_aliases=17 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20774133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e80b03c3fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5390316128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=6
//     target_aliases=77 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14487050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8f6f122ff","size":5,"soft_fail":false,"target_va":5375829056,"va":5390316101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408d8070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12338411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e815bb43ff","size":5,"soft_fail":false,"target_va":5377982576,"va":5390320982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12209742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8b2b145ff","size":5,"soft_fail":false,"target_va":5378112288,"va":5390322025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12204371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8adc645ff","size":5,"soft_fail":false,"target_va":5378112304,"va":5390316670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12195072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e800eb45ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5390322011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a88a0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t47263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e89fb80000","size":5,"soft_fail":false,"target_va":5390370976,"va":5390323708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414aab40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t55446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e896d80000","size":5,"soft_fail":false,"target_va":5390379840,"va":5390324389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414c6ee0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t178557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e87db90200","size":5,"soft_fail":false,"target_va":5390495456,"va":5390316894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424ef490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17121360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e850400501","size":5,"soft_fail":false,"target_va":5407437968,"va":5390316603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424ef600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17120445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8bd3c0501","size":5,"soft_fail":false,"target_va":5407438336,"va":5390317886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424ef780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17117008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8502f0501","size":5,"soft_fail":false,"target_va":5407438720,"va":5390321707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=25
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26030725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e885328d01","size":5,"soft_fail":false,"target_va":5416353040,"va":5390322310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26742487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8d70e9801","size":5,"soft_fail":false,"target_va":5417057040,"va":5390314548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::reserve_entity
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6088200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8f819a3ff","size":5,"soft_fail":false,"target_va":5384226192,"va":5390314387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26298348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8ec479101","size":5,"soft_fail":false,"target_va":5416621600,"va":5390323247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26298833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8d1499101","size":5,"soft_fail":false,"target_va":5416622208,"va":5390323370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26298646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e816499101","size":5,"soft_fail":false,"target_va":5416622240,"va":5390323589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11045916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8e47357ff","size":5,"soft_fail":false,"target_va":5379270736,"va":5390316647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10794331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8a54a5bff","size":5,"soft_fail":false,"target_va":5379521760,"va":5390316086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11037266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8ae9557ff","size":5,"soft_fail":false,"target_va":5379278928,"va":5390316189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6908257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e89f9696ff","size":5,"soft_fail":false,"target_va":5383408256,"va":5390316508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=15
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6989633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390313840,"raw_hex":"e8bf5895ff","size":5,"soft_fail":false,"target_va":5383324736,"va":5390314364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":1820,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":8052,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1543,"record_offset":5876,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":3620,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":7304,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":3648,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":3720,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1543,"record_offset":1580,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":1543,"record_offset":5164,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorId::get_inner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1543,"record_offset":10116,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26975,"source_line":166,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":624,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":568,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":13372,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":1492,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1543,"record_offset":5192,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1543,"record_offset":2004,"parent_record_offset":1976,"ancestor_record_offset":272,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1543,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":272,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1543,"record_offset":2064,"parent_record_offset":2032,"ancestor_record_offset":272,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1543,"record_offset":2032,"parent_record_offset":2004,"ancestor_record_offset":272,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1543,"record_offset":1976,"parent_record_offset":1940,"ancestor_record_offset":272,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":272,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":1940,"parent_record_offset":1912,"ancestor_record_offset":272,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":1856,"parent_record_offset":1820,"ancestor_record_offset":272,"inlinee":11375,"source_line":262,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1543,"record_offset":1884,"parent_record_offset":1820,"ancestor_record_offset":272,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1543,"record_offset":1608,"parent_record_offset":1580,"ancestor_record_offset":272,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1543,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1543,"record_offset":5496,"parent_record_offset":5472,"ancestor_record_offset":272,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":1152,"parent_record_offset":732,"ancestor_record_offset":272,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":1060,"parent_record_offset":1004,"ancestor_record_offset":272,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":272,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":272,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":860,"parent_record_offset":776,"ancestor_record_offset":272,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":924,"parent_record_offset":732,"ancestor_record_offset":272,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1543,"record_offset":1520,"parent_record_offset":1492,"ancestor_record_offset":272,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1543,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1543,"record_offset":1636,"parent_record_offset":1608,"ancestor_record_offset":272,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1543,"record_offset":1668,"parent_record_offset":1636,"ancestor_record_offset":272,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1543,"record_offset":5216,"parent_record_offset":5192,"ancestor_record_offset":272,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1543,"record_offset":5260,"parent_record_offset":5216,"ancestor_record_offset":272,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":732,"parent_record_offset":624,"ancestor_record_offset":272,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1543,"record_offset":1180,"parent_record_offset":732,"ancestor_record_offset":272,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1543,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":272,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1543,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":272,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1543,"record_offset":5448,"parent_record_offset":5216,"ancestor_record_offset":272,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":272,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1543,"record_offset":804,"parent_record_offset":776,"ancestor_record_offset":272,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1543,"record_offset":776,"parent_record_offset":732,"ancestor_record_offset":272,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/01543.jsonl.gz"}
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
// 完整结构及原始类型记录: visual_entity_builder.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 23; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>", "constructor": "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build", "arguments": ["system_decorator::decorator_visual_state::DecoratorTransientVisualState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorPersistentVisualState>", "constructor": "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build", "arguments": ["system_decorator::decorator_visual_state::DecoratorPersistentVisualState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::cmp::impl$4::equal_same_length<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>", "constructor": "core::slice::cmp::impl$4::equal_same_length", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder", "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>", "constructor": "core::slice::iter::Iter", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>,system_decorator::blueprint::instantiate_decorator_colliders::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>", "system_decorator::blueprint::instantiate_decorator_colliders::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>,system_decorator::blueprint::instantiate_decorator_colliders::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref$<str$>", "core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>,system_decorator::blueprint::instantiate_decorator_colliders::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: visual_entity_builder.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: visual_entity_builder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::albedo_map
// 信息 ID: b78ddfe6e9dd3d50464beb53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=12488; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=476, inline_record=21212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::brick_deformation
// 信息 ID: 55c8f8bd9b62602dcd5f1acc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=446, inline_record=14608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorPersistentVisualState>
// 信息 ID: 22f048aa69cecc9205ef0487
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build -> ["system_decorator::decorator_visual_state::DecoratorPersistentVisualState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::blueprint::instantiate_blueprints @ 0x8ef5fa -> 0x149d890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>
// 信息 ID: 397a3ed551d05b5e1a2e4870
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build -> ["system_decorator::decorator_visual_state::DecoratorTransientVisualState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::flag_connections::clothline @ 0x88c673 -> 0x149a970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x8bc0d8 -> 0x149a970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::clothline
// 信息 ID: 5f21bf190ef530112e1a82b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals; module=467, inline_record=17692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
// 信息 ID: 98591a0ea7eadacc4b47b5b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag_connections::clothline; module=454, inline_record=30504; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=469, inline_record=2068; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=469, inline_record=2476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
// 信息 ID: baf91c1e5dcc8ba67c9863ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=12760; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=17556; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=21940; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::trap_doors::generate_trap_doors; module=447, inline_record=10496; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag_connections::clothline; module=454, inline_record=30532; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals; module=467, inline_record=17664; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=469, inline_record=2012; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=469, inline_record=2420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader_name
// 信息 ID: 9b9103d64634701ddcaca1e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=455, inline_record=8236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_roof_cloth
// 信息 ID: 5e49d2f024b20c1730a8d45c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=476, inline_record=19336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_wall_cloth
// 信息 ID: a2b77b8aaf348694134a325a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=476, inline_record=22796; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=476, inline_record=23404; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=476, inline_record=23996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::has_birdspot
// 信息 ID: cbe635fe6df31f903d31054f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=455, inline_record=8264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::lantern_light
// 信息 ID: c8d58b97596896ece858bf1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=12788; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=17584; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=21968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
// 信息 ID: 44cf17905ba2855eeb64196c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 26 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=12212; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=17280; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=21664; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=446, inline_record=7696; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=446, inline_record=14028; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=446, inline_record=14580; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=12180; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=450, inline_record=7784; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
// 信息 ID: f92d5946512c246ee1a0bf96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=446, inline_record=14056; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=450, inline_record=10716; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=20664; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=468, inline_record=10508; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=469, inline_record=17156; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows; module=472, inline_record=11416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
// 信息 ID: 344167fe6a60e0bfbaf0e1a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 45 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=12184; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=12460; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=12732; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=17252; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=17528; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=21636; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=431, inline_record=21912; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=446, inline_record=7668; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::smoke
// 信息 ID: 90d3812059832beee42e39e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_roof::generate_roof_chimneys; module=448, inline_record=6804; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=13812; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=15696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::impl$1::clone
// 信息 ID: 3a4834d2c4237423aec5330b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>; module=1545, inline_record=24168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::impl$3::eq
// 信息 ID: a08e8b9f32484c726e5214bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal_same_length<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>; module=1543, inline_record=31060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
