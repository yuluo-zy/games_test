// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/camera_rig/free.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 98d0ce36cf7158f85e27c73c65faf9ae3c6c6c363898ddc1aa01434a3bb9941a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: free.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 538
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use dolly; // namespace candidate
//   package identities: [{"name":"dolly","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::translate_screen_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":108528,"rva":10891280,"va":5379600400}
// 编译实例 RVAs: 0xa63010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: free.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2154029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379600400,"raw_hex":"e82dde2000","size":5,"soft_fail":false,"target_va":5381754944,"va":5379600910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: free.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37457000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379600400,"raw_hex":"e8688c3b02","size":5,"soft_fail":false,"target_va":5417057760,"va":5379600755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37020903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379600400,"raw_hex":"e8e7e43402","size":5,"soft_fail":false,"target_va":5416622240,"va":5379601332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*56(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379600400,"raw_hex":"ff5038","size":3,"soft_fail":false,"va":5379600660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":538,"record_offset":109992,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":538,"record_offset":110436,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::position::Position::translate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/position.rs
//     evidence={"module":538,"record_offset":111368,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":33036,"source_line":34,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":109616,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":32976,"source_line":48,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":110752,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":32135,"source_line":31,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$33::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":538,"record_offset":108728,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":11718,"source_line":1174,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":110204,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":109876,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":110320,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":110088,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":111280,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":538,"record_offset":109964,"parent_record_offset":108528,"ancestor_record_offset":108528,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":110944,"parent_record_offset":110920,"ancestor_record_offset":108528,"inlinee":10219,"source_line":264,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":109804,"parent_record_offset":109780,"ancestor_record_offset":108528,"inlinee":10140,"source_line":228,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":109828,"parent_record_offset":109804,"ancestor_record_offset":108528,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":111064,"parent_record_offset":111036,"ancestor_record_offset":108528,"inlinee":9375,"source_line":930,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":538,"record_offset":108776,"parent_record_offset":108752,"ancestor_record_offset":108528,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":538,"record_offset":109272,"parent_record_offset":108860,"ancestor_record_offset":108528,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":538,"record_offset":109180,"parent_record_offset":109124,"ancestor_record_offset":108528,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":538,"record_offset":108948,"parent_record_offset":108924,"ancestor_record_offset":108528,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":538,"record_offset":108980,"parent_record_offset":108900,"ancestor_record_offset":108528,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":538,"record_offset":109044,"parent_record_offset":108860,"ancestor_record_offset":108528,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":538,"record_offset":109904,"parent_record_offset":109876,"ancestor_record_offset":108528,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":538,"record_offset":110232,"parent_record_offset":110204,"ancestor_record_offset":108528,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":538,"record_offset":111308,"parent_record_offset":111280,"ancestor_record_offset":108528,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":538,"record_offset":110348,"parent_record_offset":110320,"ancestor_record_offset":108528,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":538,"record_offset":110116,"parent_record_offset":110088,"ancestor_record_offset":108528,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":111012,"parent_record_offset":110752,"ancestor_record_offset":108528,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":109748,"parent_record_offset":109692,"ancestor_record_offset":108528,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":109720,"parent_record_offset":109692,"ancestor_record_offset":108528,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":109664,"parent_record_offset":109640,"ancestor_record_offset":108528,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":109692,"parent_record_offset":109664,"ancestor_record_offset":108528,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":110804,"parent_record_offset":110780,"ancestor_record_offset":108528,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":110832,"parent_record_offset":110804,"ancestor_record_offset":108528,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":109640,"parent_record_offset":109616,"ancestor_record_offset":108528,"inlinee":32977,"source_line":58,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":110780,"parent_record_offset":110752,"ancestor_record_offset":108528,"inlinee":32136,"source_line":41,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":111544,"parent_record_offset":111520,"ancestor_record_offset":108528,"inlinee":32983,"source_line":49,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":111036,"parent_record_offset":111012,"ancestor_record_offset":108528,"inlinee":32141,"source_line":32,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":109780,"parent_record_offset":109664,"ancestor_record_offset":108528,"inlinee":32979,"source_line":61,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":110920,"parent_record_offset":110804,"ancestor_record_offset":108528,"inlinee":32139,"source_line":44,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":538,"record_offset":108752,"parent_record_offset":108728,"ancestor_record_offset":108528,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":538,"record_offset":108860,"parent_record_offset":108752,"ancestor_record_offset":108528,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":538,"record_offset":109300,"parent_record_offset":108860,"ancestor_record_offset":108528,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":538,"record_offset":108824,"parent_record_offset":108800,"ancestor_record_offset":108528,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":538,"record_offset":108800,"parent_record_offset":108776,"ancestor_record_offset":108528,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":111400,"parent_record_offset":111368,"ancestor_record_offset":108528,"inlinee":33037,"source_line":1081,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":538,"record_offset":108924,"parent_record_offset":108900,"ancestor_record_offset":108528,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":538,"record_offset":108900,"parent_record_offset":108860,"ancestor_record_offset":108528,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::FreeCamera::get_position
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":11516,"rva":10854816,"va":5379563936}
// 编译实例 RVAs: 0xa5a1a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37058066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563936,"raw_hex":"e812763502","size":5,"soft_fail":false,"target_va":5416622240,"va":5379564169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*56(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379563936,"raw_hex":"ff5038","size":3,"soft_fail":false,"va":5379564016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":538,"record_offset":12020,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":11672,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":32976,"source_line":48,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":11860,"parent_record_offset":11840,"ancestor_record_offset":11516,"inlinee":10140,"source_line":228,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":11884,"parent_record_offset":11860,"ancestor_record_offset":11516,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":11980,"parent_record_offset":11952,"ancestor_record_offset":11516,"inlinee":9375,"source_line":930,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":11928,"parent_record_offset":11672,"ancestor_record_offset":11516,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":11808,"parent_record_offset":11752,"ancestor_record_offset":11516,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":11780,"parent_record_offset":11752,"ancestor_record_offset":11516,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":11724,"parent_record_offset":11700,"ancestor_record_offset":11516,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":11752,"parent_record_offset":11724,"ancestor_record_offset":11516,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":11700,"parent_record_offset":11672,"ancestor_record_offset":11516,"inlinee":32977,"source_line":58,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":11952,"parent_record_offset":11928,"ancestor_record_offset":11516,"inlinee":32983,"source_line":49,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":11840,"parent_record_offset":11724,"ancestor_record_offset":11516,"inlinee":32979,"source_line":61,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":11516,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$33::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_mint.rs
//     evidence={"module":538,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":11516,"inlinee":32981,"source_line":41,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::FreeCamera::get_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":12108,"rva":10855056,"va":5379564176}
// 编译实例 RVAs: 0xa5a290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37057840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564176,"raw_hex":"e830753502","size":5,"soft_fail":false,"target_va":5416622240,"va":5379564395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*56(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379564176,"raw_hex":"ff5038","size":3,"soft_fail":false,"va":5379564256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":12260,"parent_record_offset":12108,"ancestor_record_offset":12108,"inlinee":32976,"source_line":48,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":12444,"parent_record_offset":12424,"ancestor_record_offset":12108,"inlinee":10140,"source_line":228,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":12468,"parent_record_offset":12444,"ancestor_record_offset":12108,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":12564,"parent_record_offset":12536,"ancestor_record_offset":12108,"inlinee":9375,"source_line":930,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":12512,"parent_record_offset":12260,"ancestor_record_offset":12108,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":12392,"parent_record_offset":12336,"ancestor_record_offset":12108,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":12364,"parent_record_offset":12336,"ancestor_record_offset":12108,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":12108,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":12336,"parent_record_offset":12308,"ancestor_record_offset":12108,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":12284,"parent_record_offset":12260,"ancestor_record_offset":12108,"inlinee":32977,"source_line":58,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":12108,"inlinee":32983,"source_line":49,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":12424,"parent_record_offset":12308,"ancestor_record_offset":12108,"inlinee":32979,"source_line":61,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::FreeCamera::set
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":12608,"rva":10855296,"va":5379564416}
// 编译实例 RVAs: 0xa5a380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007e380
//     owner=unresolved_namespace; count=2
//     target_aliases=39 candidates; complete list: free.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10338668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564416,"raw_hex":"e8943e62ff","size":5,"soft_fail":false,"target_va":5369226112,"va":5379564775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] dolly::rig::CameraRigBuilder$LT$H$GT$::with
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16301928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564416,"raw_hex":"e868bff800","size":5,"soft_fail":false,"target_va":5395866464,"va":5379564531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379564416,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5379564688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":13188,"parent_record_offset":12608,"ancestor_record_offset":12608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::position::Position::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/position.rs
//     evidence={"module":538,"record_offset":12792,"parent_record_offset":12608,"ancestor_record_offset":12608,"inlinee":32150,"source_line":24,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::smooth::Smooth::new_position_rotation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/smooth.rs
//     evidence={"module":538,"record_offset":12876,"parent_record_offset":12608,"ancestor_record_offset":12608,"inlinee":32152,"source_line":61,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::builder
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":12764,"parent_record_offset":12608,"ancestor_record_offset":12608,"inlinee":32148,"source_line":86,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRigBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":12904,"parent_record_offset":12608,"ancestor_record_offset":12608,"inlinee":32154,"source_line":105,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":13020,"parent_record_offset":12996,"ancestor_record_offset":12608,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":12996,"parent_record_offset":12948,"ancestor_record_offset":12608,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":538,"record_offset":12816,"parent_record_offset":12792,"ancestor_record_offset":12608,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":13156,"parent_record_offset":12904,"ancestor_record_offset":12608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":13120,"parent_record_offset":13052,"ancestor_record_offset":12608,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":13088,"parent_record_offset":13052,"ancestor_record_offset":12608,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":13052,"parent_record_offset":12948,"ancestor_record_offset":12608,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":12948,"parent_record_offset":12904,"ancestor_record_offset":12608,"inlinee":32155,"source_line":67,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$34::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_mint.rs
//     evidence={"module":538,"record_offset":12840,"parent_record_offset":12816,"ancestor_record_offset":12608,"inlinee":32985,"source_line":47,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::rotate_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":108024,"rva":10891008,"va":5379600128}
// 编译实例 RVAs: 0xa62f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37021842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379600128,"raw_hex":"e892e83402","size":5,"soft_fail":false,"target_va":5416622240,"va":5379600393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] dolly::drivers::yaw_pitch::YawPitch::rotate_yaw_pitch
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t17714533","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379600128,"raw_hex":"e9654d0e01","size":5,"soft_fail":false,"target_va":5397314864,"va":5379600326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*64(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379600128,"raw_hex":"ff5040","size":3,"soft_fail":false,"va":5379600224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":108176,"parent_record_offset":108024,"ancestor_record_offset":108024,"inlinee":32135,"source_line":31,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":108364,"parent_record_offset":108344,"ancestor_record_offset":108024,"inlinee":10219,"source_line":264,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":108388,"parent_record_offset":108364,"ancestor_record_offset":108024,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":538,"record_offset":108484,"parent_record_offset":108456,"ancestor_record_offset":108024,"inlinee":9375,"source_line":930,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":108432,"parent_record_offset":108176,"ancestor_record_offset":108024,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":108312,"parent_record_offset":108256,"ancestor_record_offset":108024,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":108284,"parent_record_offset":108256,"ancestor_record_offset":108024,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":108228,"parent_record_offset":108204,"ancestor_record_offset":108024,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":108256,"parent_record_offset":108228,"ancestor_record_offset":108024,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":108204,"parent_record_offset":108176,"ancestor_record_offset":108024,"inlinee":32136,"source_line":41,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":108456,"parent_record_offset":108432,"ancestor_record_offset":108024,"inlinee":32141,"source_line":32,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":108344,"parent_record_offset":108228,"ancestor_record_offset":108024,"inlinee":32139,"source_line":44,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::constrain_to_be_above_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":111616,"rva":13072,"va":5368722192}
// 编译实例 RVAs: 0x3310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":111772,"rva":10892288,"va":5379601408}
// 编译实例 RVAs: 0xa63400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379601408,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5379601522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":538,"record_offset":111916,"parent_record_offset":111772,"ancestor_record_offset":111772,"inlinee":32155,"source_line":67,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] dolly::transform::Transform::into_position_rotation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/transform.rs
//     evidence={"module":538,"record_offset":112088,"parent_record_offset":111772,"ancestor_record_offset":111772,"inlinee":33039,"source_line":31,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":111988,"parent_record_offset":111964,"ancestor_record_offset":111772,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":111964,"parent_record_offset":111916,"ancestor_record_offset":111772,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":112048,"parent_record_offset":112020,"ancestor_record_offset":111772,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":112020,"parent_record_offset":111916,"ancestor_record_offset":111772,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
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
// 完整结构及原始类型记录: free.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::free::FreeCamera
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::camera_rig::free::FreeCamera>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::camera_rig::free::FreeCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: free.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: free.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 881; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::collision::world::Raycasting", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >", "bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>", "bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::FreeCamera::get_position
// 信息 ID: ffcee33cbf85f7a036273b4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_debug::do_freecam_save @ 0x187e8b -> 0xa5a1a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::FreeCamera::get_yaw_pitch
// 信息 ID: 6ddc7e797c08b54c1cfea8e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_debug::do_freecam_save @ 0x187e93 -> 0xa5a290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::FreeCamera::set
// 信息 ID: 43ee3a5db98892681c41ba61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_debug::do_freecam_save @ 0x188010 -> 0xa5a380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::constrain_to_be_above_terrain
// 信息 ID: 9c254d5713beb35d35aba10f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 000cf4217700e36734691cdd: void (*)(bevy_ecs::change_detection::ResMut<toolbar::quick_tool_palette::QuickToolPalette>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::system::query::Query<ref$<country_core::components::Ui2dExtent>,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// [泛型宿主/闭包/直接引用方参数候选] 00605d96b14cdb92382e4f3e: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// [泛型宿主/闭包/直接引用方参数候选] 0094895827cf75842dfd9081: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 0103bc1d475ed50b624e8f2f: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)
// [泛型宿主/闭包/直接引用方参数候选] 01a560f3d409056d7626bac9: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// [泛型宿主/闭包/直接引用方参数候选] 01e5ad06aacd15ca43210285: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)
// [泛型宿主/闭包/直接引用方参数候选] 01f77029bb05d937ca08973d: void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// [泛型宿主/闭包/直接引用方参数候选] 0203a61afd44dd916d707a37: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 其余候选接口见完整附档，共 881 条。
// 引用方: 原程序直接转移 13667 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> @ 0x2d7e -> 0x3310 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> @ 0x2dbe -> 0x3310 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> | country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> @ 0x78c77 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> | country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> @ 0x78c97 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$0>,never$> > | core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$0>,never$> >,tuple$<ref$<std::sync::once::OnceState> > > @ 0x791b6 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$1>,never$> > | core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$1>,never$> >,tuple$<ref$<std::sync::once::OnceState> > > @ 0x792ec -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::after<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)>,void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)> @ 0x795ac -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x796bf -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x796e3 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x79726 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)> @ 0x7d59f -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)> @ 0x7d5c3 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)> @ 0x7d60c -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] priority_queue::store::Store::with_capacity_and_hasher<alloc::string::String,u32,std::hash::random::RandomState> @ 0xc8e45 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] priority_queue::store::Store::with_capacity_and_hasher<alloc::string::String,u32,std::hash::random::RandomState> @ 0xc8de8 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] priority_queue::store::Store::with_capacity_and_hasher<alloc::string::String,u32,std::hash::random::RandomState> @ 0xc8e06 -> 0x3310 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::rotate_yaw_pitch
// 信息 ID: 049faa366baed3c405f551b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::translate_screen_space
// 信息 ID: 05053d6ce649b975101303c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::update
// 信息 ID: 7bb64e250a43bc3ab07ff7fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
