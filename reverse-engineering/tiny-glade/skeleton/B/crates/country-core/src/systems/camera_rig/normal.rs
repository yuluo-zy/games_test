// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/camera_rig/normal.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1ded4a766756bf8e30677a224166d7b146f143ddc129bd2ee2e556091f326289
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: normal.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 504, 512, 1585
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":17928,"rva":9740256,"va":5378449376}
// 编译实例 RVAs: 0x949fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$glam..f32..sse2..quat..Quat$u20$as$u20$glam..euler..FromEuler$GT$::from_euler_angles
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4199139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378449376,"raw_hex":"e8e3124000","size":5,"soft_fail":false,"target_va":5382648896,"va":5378449752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3484212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378449376,"raw_hex":"e8342a3500","size":5,"soft_fail":false,"target_va":5381934560,"va":5378450343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33398338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378449376,"raw_hex":"e8429efd01","size":5,"soft_fail":false,"target_va":5411847872,"va":5378449529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":25560,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":7686,"source_line":2096,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":25532,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":512,"record_offset":25148,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":512,"record_offset":22472,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":512,"record_offset":22388,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::camera_body::pitch_yaw_rot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/camera_body.rs
//     evidence={"module":512,"record_offset":22500,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":28506,"source_line":24,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::NormalCamera::rotation
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":512,"record_offset":18144,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":29374,"source_line":39,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::constrain_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":512,"record_offset":18856,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":29377,"source_line":211,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":21472,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":22792,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":25347,"source_line":923,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":18404,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":29375,"source_line":830,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":22184,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":18316,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":18768,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":18680,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":25320,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":22616,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":512,"record_offset":23688,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":18288,"parent_record_offset":17928,"ancestor_record_offset":17928,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":25624,"parent_record_offset":25584,"ancestor_record_offset":17928,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":25584,"parent_record_offset":25560,"ancestor_record_offset":17928,"inlinee":7687,"source_line":2122,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":26304,"parent_record_offset":26280,"ancestor_record_offset":17928,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":25680,"parent_record_offset":25652,"ancestor_record_offset":17928,"inlinee":7691,"source_line":2100,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":26280,"parent_record_offset":26256,"ancestor_record_offset":17928,"inlinee":7696,"source_line":2156,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":25652,"parent_record_offset":25584,"ancestor_record_offset":17928,"inlinee":7689,"source_line":2176,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":512,"record_offset":22840,"parent_record_offset":22816,"ancestor_record_offset":17928,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":21916,"parent_record_offset":21496,"ancestor_record_offset":17928,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":21824,"parent_record_offset":21768,"ancestor_record_offset":17928,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":21592,"parent_record_offset":21568,"ancestor_record_offset":17928,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":21624,"parent_record_offset":21540,"ancestor_record_offset":17928,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":21688,"parent_record_offset":21496,"ancestor_record_offset":17928,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":25764,"parent_record_offset":25728,"ancestor_record_offset":17928,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":19028,"parent_record_offset":18904,"ancestor_record_offset":17928,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":19596,"parent_record_offset":19540,"ancestor_record_offset":17928,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":18196,"parent_record_offset":18168,"ancestor_record_offset":17928,"inlinee":28525,"source_line":867,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":512,"record_offset":25964,"parent_record_offset":25920,"ancestor_record_offset":17928,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":512,"record_offset":25172,"parent_record_offset":25148,"ancestor_record_offset":17928,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":512,"record_offset":26232,"parent_record_offset":25584,"ancestor_record_offset":17928,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":18344,"parent_record_offset":18316,"ancestor_record_offset":17928,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":18616,"parent_record_offset":18492,"ancestor_record_offset":17928,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":18560,"parent_record_offset":18492,"ancestor_record_offset":17928,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":18992,"parent_record_offset":18964,"ancestor_record_offset":17928,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":18796,"parent_record_offset":18768,"ancestor_record_offset":17928,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":25348,"parent_record_offset":25320,"ancestor_record_offset":17928,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":18708,"parent_record_offset":18680,"ancestor_record_offset":17928,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":512,"record_offset":25196,"parent_record_offset":25172,"ancestor_record_offset":17928,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":512,"record_offset":26256,"parent_record_offset":26232,"ancestor_record_offset":17928,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":512,"record_offset":22436,"parent_record_offset":22412,"ancestor_record_offset":17928,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":512,"record_offset":22412,"parent_record_offset":22388,"ancestor_record_offset":17928,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":19240,"parent_record_offset":19196,"ancestor_record_offset":17928,"inlinee":28491,"source_line":159,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":19480,"parent_record_offset":19196,"ancestor_record_offset":17928,"inlinee":28493,"source_line":164,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":19196,"parent_record_offset":18904,"ancestor_record_offset":17928,"inlinee":28571,"source_line":138,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::camera_body::pitch_yaw_rot
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/camera_body.rs
//     evidence={"module":512,"record_offset":18168,"parent_record_offset":18144,"ancestor_record_offset":17928,"inlinee":28506,"source_line":24,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::ActiveRotation::is_finished
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":512,"record_offset":25728,"parent_record_offset":25704,"ancestor_record_offset":17928,"inlinee":29383,"source_line":202,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::constrain_position_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":512,"record_offset":20228,"parent_record_offset":18856,"ancestor_record_offset":17928,"inlinee":29379,"source_line":224,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::constrain_position_semi_ortho
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":512,"record_offset":18904,"parent_record_offset":18856,"ancestor_record_offset":17928,"inlinee":29378,"source_line":243,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::impl$1::update::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":512,"record_offset":25704,"parent_record_offset":25680,"ancestor_record_offset":17928,"inlinee":29381,"source_line":186,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":512,"record_offset":18492,"parent_record_offset":18404,"ancestor_record_offset":17928,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":18252,"parent_record_offset":18168,"ancestor_record_offset":17928,"inlinee":28507,"source_line":184,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":17928,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":21496,"parent_record_offset":21472,"ancestor_record_offset":17928,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":512,"record_offset":21944,"parent_record_offset":21496,"ancestor_record_offset":17928,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":512,"record_offset":22888,"parent_record_offset":22864,"ancestor_record_offset":17928,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":512,"record_offset":22864,"parent_record_offset":22840,"ancestor_record_offset":17928,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19516,"parent_record_offset":19480,"ancestor_record_offset":17928,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19264,"parent_record_offset":19240,"ancestor_record_offset":17928,"inlinee":28492,"source_line":764,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":17928,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19292,"parent_record_offset":19264,"ancestor_record_offset":17928,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19540,"parent_record_offset":19516,"ancestor_record_offset":17928,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19084,"parent_record_offset":18904,"ancestor_record_offset":17928,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19828,"parent_record_offset":19480,"ancestor_record_offset":17928,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19140,"parent_record_offset":18904,"ancestor_record_offset":17928,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":18964,"parent_record_offset":18904,"ancestor_record_offset":17928,"inlinee":11304,"source_line":1744,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":19948,"parent_record_offset":18904,"ancestor_record_offset":17928,"inlinee":29375,"source_line":830,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":18460,"parent_record_offset":18436,"ancestor_record_offset":17928,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":18436,"parent_record_offset":18404,"ancestor_record_offset":17928,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":20272,"parent_record_offset":20228,"ancestor_record_offset":17928,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":20448,"parent_record_offset":20228,"ancestor_record_offset":17928,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":21232,"parent_record_offset":20228,"ancestor_record_offset":17928,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":512,"record_offset":21568,"parent_record_offset":21540,"ancestor_record_offset":17928,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":512,"record_offset":21540,"parent_record_offset":21496,"ancestor_record_offset":17928,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":18532,"parent_record_offset":18492,"ancestor_record_offset":17928,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F5D
// 首条位置: {"module":504,"inlinee":28509}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::NormalCamera::rotation
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72BE
// 首条位置: {"module":512,"inlinee":29374}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::camera_body::pitch_yaw_rot
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/camera_body.rs
//     evidence={"module":512,"record_offset":18168,"parent_record_offset":18144,"ancestor_record_offset":18144,"inlinee":28506,"source_line":24,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":18196,"parent_record_offset":18168,"ancestor_record_offset":18144,"inlinee":28525,"source_line":867,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":18252,"parent_record_offset":18168,"ancestor_record_offset":18144,"inlinee":28507,"source_line":184,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":15652,"rva":9739456,"va":5378448576}
// 编译实例 RVAs: 0x949cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141625db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13484157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378448576,"raw_hex":"e87dc0cd00","size":5,"soft_fail":false,"target_va":5391932848,"va":5378448686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":16044,"parent_record_offset":15652,"ancestor_record_offset":15652,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":16016,"parent_record_offset":15652,"ancestor_record_offset":15652,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":512,"record_offset":15932,"parent_record_offset":15652,"ancestor_record_offset":15652,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":512,"record_offset":15904,"parent_record_offset":15652,"ancestor_record_offset":15652,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":512,"record_offset":15820,"parent_record_offset":15652,"ancestor_record_offset":15652,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":512,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":15652,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":512,"record_offset":15956,"parent_record_offset":15932,"ancestor_record_offset":15652,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":512,"record_offset":15868,"parent_record_offset":15844,"ancestor_record_offset":15652,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":512,"record_offset":15844,"parent_record_offset":15820,"ancestor_record_offset":15652,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::constrain_to_be_above_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":16076,"rva":9739712,"va":5378448832}
// 编译实例 RVAs: 0x949dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e9640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t653350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378448832,"raw_hex":"e826f80900","size":5,"soft_fail":false,"target_va":5379102272,"va":5378448917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":16436,"parent_record_offset":16076,"ancestor_record_offset":16076,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":16248,"parent_record_offset":16076,"ancestor_record_offset":16076,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":16356,"parent_record_offset":16076,"ancestor_record_offset":16076,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":16280,"parent_record_offset":16248,"ancestor_record_offset":16076,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":16384,"parent_record_offset":16356,"ancestor_record_offset":16076,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::translate_screen_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":16468,"rva":9739888,"va":5378449008}
// 编译实例 RVAs: 0x949e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33398663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378449008,"raw_hex":"e8879ffd01","size":5,"soft_fail":false,"target_va":5411847808,"va":5378449140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33399010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378449008,"raw_hex":"e8e2a0fd01","size":5,"soft_fail":false,"target_va":5411848144,"va":5378449129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_direction
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3460810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378449008,"raw_hex":"e8cace3400","size":5,"soft_fail":false,"target_va":5381909888,"va":5378449073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3460766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378449008,"raw_hex":"e89ece3400","size":5,"soft_fail":false,"target_va":5381909824,"va":5378449053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":16648,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":16676,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":28525,"source_line":867,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":16704,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":25694,"source_line":168,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":16852,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":25347,"source_line":923,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":17748,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":17836,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":512,"record_offset":16900,"parent_record_offset":16876,"ancestor_record_offset":16468,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":17404,"parent_record_offset":16984,"ancestor_record_offset":16468,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":17312,"parent_record_offset":17256,"ancestor_record_offset":16468,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":17080,"parent_record_offset":17056,"ancestor_record_offset":16468,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":17112,"parent_record_offset":17028,"ancestor_record_offset":16468,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":17176,"parent_record_offset":16984,"ancestor_record_offset":16468,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":17776,"parent_record_offset":17748,"ancestor_record_offset":16468,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":17864,"parent_record_offset":17836,"ancestor_record_offset":16468,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":512,"record_offset":16732,"parent_record_offset":16704,"ancestor_record_offset":16468,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":16820,"parent_record_offset":16704,"ancestor_record_offset":16468,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":16876,"parent_record_offset":16852,"ancestor_record_offset":16468,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":16984,"parent_record_offset":16876,"ancestor_record_offset":16468,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":512,"record_offset":17432,"parent_record_offset":16984,"ancestor_record_offset":16468,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":512,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":16468,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":512,"record_offset":16924,"parent_record_offset":16900,"ancestor_record_offset":16468,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":512,"record_offset":17056,"parent_record_offset":17028,"ancestor_record_offset":16468,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":512,"record_offset":17028,"parent_record_offset":16984,"ancestor_record_offset":16468,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":16780,"parent_record_offset":16756,"ancestor_record_offset":16468,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":16468,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72C5
// 首条位置: {"module":512,"inlinee":29381}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::ActiveRotation::is_finished
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":512,"record_offset":25728,"parent_record_offset":25704,"ancestor_record_offset":25704,"inlinee":29383,"source_line":202,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":25764,"parent_record_offset":25728,"ancestor_record_offset":25704,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":25876,"parent_record_offset":25728,"ancestor_record_offset":25704,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::ActiveRotation::is_finished
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72C7
// 首条位置: {"module":512,"inlinee":29383}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":25764,"parent_record_offset":25728,"ancestor_record_offset":25728,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":25876,"parent_record_offset":25728,"ancestor_record_offset":25728,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::constrain_position
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72C1
// 首条位置: {"module":512,"inlinee":29377}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::constrain_position_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":512,"record_offset":20228,"parent_record_offset":18856,"ancestor_record_offset":18856,"inlinee":29379,"source_line":224,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::constrain_position_semi_ortho
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":512,"record_offset":18904,"parent_record_offset":18856,"ancestor_record_offset":18856,"inlinee":29378,"source_line":243,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":19028,"parent_record_offset":18904,"ancestor_record_offset":18856,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":19596,"parent_record_offset":19540,"ancestor_record_offset":18856,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":19852,"parent_record_offset":19828,"ancestor_record_offset":18856,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":18856,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":19412,"parent_record_offset":19348,"ancestor_record_offset":18856,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":18992,"parent_record_offset":18964,"ancestor_record_offset":18856,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":19164,"parent_record_offset":19140,"ancestor_record_offset":18856,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":19240,"parent_record_offset":19196,"ancestor_record_offset":18856,"inlinee":28491,"source_line":159,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":19480,"parent_record_offset":19196,"ancestor_record_offset":18856,"inlinee":28493,"source_line":164,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":19196,"parent_record_offset":18904,"ancestor_record_offset":18856,"inlinee":28571,"source_line":138,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":512,"record_offset":19348,"parent_record_offset":19264,"ancestor_record_offset":18856,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19516,"parent_record_offset":19480,"ancestor_record_offset":18856,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19264,"parent_record_offset":19240,"ancestor_record_offset":18856,"inlinee":28492,"source_line":764,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":18856,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19292,"parent_record_offset":19264,"ancestor_record_offset":18856,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19540,"parent_record_offset":19516,"ancestor_record_offset":18856,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19084,"parent_record_offset":18904,"ancestor_record_offset":18856,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19828,"parent_record_offset":19480,"ancestor_record_offset":18856,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19140,"parent_record_offset":18904,"ancestor_record_offset":18856,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":18964,"parent_record_offset":18904,"ancestor_record_offset":18856,"inlinee":11304,"source_line":1744,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":19948,"parent_record_offset":18904,"ancestor_record_offset":18856,"inlinee":29375,"source_line":830,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":20004,"parent_record_offset":19980,"ancestor_record_offset":18856,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":19980,"parent_record_offset":19948,"ancestor_record_offset":18856,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":20272,"parent_record_offset":20228,"ancestor_record_offset":18856,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":20448,"parent_record_offset":20228,"ancestor_record_offset":18856,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":21232,"parent_record_offset":20228,"ancestor_record_offset":18856,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":19384,"parent_record_offset":19348,"ancestor_record_offset":18856,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::constrain_position_normal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 224
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72C3
// 首条位置: {"module":512,"inlinee":29379}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":20536,"parent_record_offset":20228,"ancestor_record_offset":20228,"inlinee":28571,"source_line":138,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":20272,"parent_record_offset":20228,"ancestor_record_offset":20228,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":20448,"parent_record_offset":20228,"ancestor_record_offset":20228,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":21232,"parent_record_offset":20228,"ancestor_record_offset":20228,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":20908,"parent_record_offset":20880,"ancestor_record_offset":20228,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":20936,"parent_record_offset":20880,"ancestor_record_offset":20228,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":20476,"parent_record_offset":20448,"ancestor_record_offset":20228,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":20300,"parent_record_offset":20272,"ancestor_record_offset":20228,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":20752,"parent_record_offset":20688,"ancestor_record_offset":20228,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":20996,"parent_record_offset":20972,"ancestor_record_offset":20228,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":20580,"parent_record_offset":20536,"ancestor_record_offset":20228,"inlinee":28491,"source_line":159,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":20820,"parent_record_offset":20536,"ancestor_record_offset":20228,"inlinee":28493,"source_line":164,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":512,"record_offset":20688,"parent_record_offset":20604,"ancestor_record_offset":20228,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":20856,"parent_record_offset":20820,"ancestor_record_offset":20228,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":20604,"parent_record_offset":20580,"ancestor_record_offset":20228,"inlinee":28492,"source_line":764,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":20656,"parent_record_offset":20632,"ancestor_record_offset":20228,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":20632,"parent_record_offset":20604,"ancestor_record_offset":20228,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":20880,"parent_record_offset":20856,"ancestor_record_offset":20228,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":21112,"parent_record_offset":20820,"ancestor_record_offset":20228,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":21168,"parent_record_offset":20820,"ancestor_record_offset":20228,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":20972,"parent_record_offset":20820,"ancestor_record_offset":20228,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":20724,"parent_record_offset":20688,"ancestor_record_offset":20228,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::constrain_position_semi_ortho
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72C2
// 首条位置: {"module":512,"inlinee":29378}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":19028,"parent_record_offset":18904,"ancestor_record_offset":18904,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":19196,"parent_record_offset":18904,"ancestor_record_offset":18904,"inlinee":28571,"source_line":138,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19084,"parent_record_offset":18904,"ancestor_record_offset":18904,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19892,"parent_record_offset":18904,"ancestor_record_offset":18904,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19140,"parent_record_offset":18904,"ancestor_record_offset":18904,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":18964,"parent_record_offset":18904,"ancestor_record_offset":18904,"inlinee":11304,"source_line":1744,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":19948,"parent_record_offset":18904,"ancestor_record_offset":18904,"inlinee":29375,"source_line":830,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":19568,"parent_record_offset":19540,"ancestor_record_offset":18904,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":19596,"parent_record_offset":19540,"ancestor_record_offset":18904,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":19852,"parent_record_offset":19828,"ancestor_record_offset":18904,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":18904,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":19412,"parent_record_offset":19348,"ancestor_record_offset":18904,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":18992,"parent_record_offset":18964,"ancestor_record_offset":18904,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":19164,"parent_record_offset":19140,"ancestor_record_offset":18904,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":19240,"parent_record_offset":19196,"ancestor_record_offset":18904,"inlinee":28491,"source_line":159,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":512,"record_offset":19480,"parent_record_offset":19196,"ancestor_record_offset":18904,"inlinee":28493,"source_line":164,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":512,"record_offset":19348,"parent_record_offset":19264,"ancestor_record_offset":18904,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19516,"parent_record_offset":19480,"ancestor_record_offset":18904,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19264,"parent_record_offset":19240,"ancestor_record_offset":18904,"inlinee":28492,"source_line":764,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":18904,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19292,"parent_record_offset":19264,"ancestor_record_offset":18904,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19540,"parent_record_offset":19516,"ancestor_record_offset":18904,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19772,"parent_record_offset":19480,"ancestor_record_offset":18904,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19828,"parent_record_offset":19480,"ancestor_record_offset":18904,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":19632,"parent_record_offset":19480,"ancestor_record_offset":18904,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":20004,"parent_record_offset":19980,"ancestor_record_offset":18904,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":19980,"parent_record_offset":19948,"ancestor_record_offset":18904,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":19384,"parent_record_offset":19348,"ancestor_record_offset":18904,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00512.jsonl.gz"}
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
// 完整结构及原始类型记录: normal.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::normal::ActiveRotation
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::systems::camera_rig::normal::ActiveRotation,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::systems::camera_rig::normal::ActiveRotation", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::systems::camera_rig::normal::ActiveRotation"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::systems::camera_rig::normal::ActiveRotation>", "constructor": "ref$", "arguments": ["country_core::systems::camera_rig::normal::ActiveRotation"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >", "constructor": "core::option::IntoIter", "arguments": ["ref$<country_core::systems::camera_rig::normal::ActiveRotation>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>", "core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >", "country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> >", "constructor": "core::iter::traits::accum::impl$100::sum::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["ref$<country_core::systems::camera_rig::normal::ActiveRotation>", "f32", "f32", "country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0", "core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::fold<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >,f32,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> > > >", "constructor": "core::iter::adapters::chain::impl$1::fold", "arguments": ["core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>", "core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >", "f32", "core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::normal::NormalCamera
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::camera_rig::normal::NormalCamera>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::camera_rig::normal::NormalCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: normal.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::camera_rig::normal::ActiveRotation", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: normal.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::ActiveRotation::is_finished
// 信息 ID: 44f61a19e2adc3548abc6f43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=25728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=26040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::NormalCamera::rotation
// 信息 ID: 4bc7c2d9087787897b12c01d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=18144; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=21328; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=26352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=26496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::constrain_position
// 信息 ID: 1bf0458802c42e56247cca31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=18856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::constrain_position_normal
// 信息 ID: 8da5a5a2bd64f39103db8967
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=20228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::constrain_position_semi_ortho
// 信息 ID: 3dcf0c0effda7469eeb69459
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=18904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::constrain_to_be_above_terrain
// 信息 ID: 68c303eae53cd13c5c12e568
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch
// 信息 ID: c1bd1ec073dd226a8e9cde16
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::translate_screen_space
// 信息 ID: 1def44ce2003b8ceed4409ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::update
// 信息 ID: b8410ff8b77952625932a9e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::update::closure$0
// 信息 ID: 6ffe8b6af79dc4d4074686d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::camera_rig::normal::impl$1::update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=25704; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=512, inline_record=26016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$3::default
// 信息 ID: cdfdf299159fc5af8f330c19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::MainCamera::new; module=504, inline_record=8944; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::MainCamera::new; module=504, inline_record=13908; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::MainCamera::new; module=504, inline_record=14020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
