// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/setup.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 05c4a7168d2726e463edd6d05aa719465894b6d83253993a0246d80a6b927bbb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: setup.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 12, 71, 92, 96, 104, 175, 184
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use winit; // namespace candidate
//   package identities: [{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::setup_window
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":12,"record_offset":5408,"rva":276144,"va":5368985264}
// 编译实例 RVAs: 0x436b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::try_fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5123114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e82a2c4e00","size":5,"soft_fail":false,"target_va":5374109840,"va":5368986721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t6633327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e86f376500","size":5,"soft_fail":false,"target_va":5375621152,"va":5368987820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t13657847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8f766d000","size":5,"soft_fail":false,"target_va":5382644528,"va":5368986676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t6633903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8af396500","size":5,"soft_fail":false,"target_va":5375621104,"va":5368987196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6634478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8ee3b6500","size":5,"soft_fail":false,"target_va":5375620560,"va":5368986077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400430c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8bcdeffff","size":5,"soft_fail":false,"target_va":5368983744,"va":5368992255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t381398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8d6d10500","size":5,"soft_fail":false,"target_va":5369374080,"va":5368992677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140175450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1250979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8a3161300","size":5,"soft_fail":false,"target_va":5370238032,"va":5368987048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5822483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e813d85800","size":5,"soft_fail":false,"target_va":5374814320,"va":5368991832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5909532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e81c2c5a00","size":5,"soft_fail":false,"target_va":5374900736,"va":5368991199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebc10
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5926942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e81e705a00","size":5,"soft_fail":false,"target_va":5374917648,"va":5368990701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eeac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5937415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e807995a00","size":5,"soft_fail":false,"target_va":5374929600,"va":5368992180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6095584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8e0025d00","size":5,"soft_fail":false,"target_va":5375081488,"va":5368985899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=19
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6095245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e88d015d00","size":5,"soft_fail":false,"target_va":5375081504,"va":5368986254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6529538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e802a26300","size":5,"soft_fail":false,"target_va":5375516032,"va":5368986489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140697a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6635076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8443e6500","size":5,"soft_fail":false,"target_va":5375621776,"va":5368986695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42323272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e848cd8502","size":5,"soft_fail":false,"target_va":5411313216,"va":5368989939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42861065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e809028e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5368986098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42550807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e817468902","size":5,"soft_fail":false,"target_va":5411540963,"va":5368990151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6631554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e882306500","size":5,"soft_fail":false,"target_va":5375621392,"va":5368989833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42551578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e81a498902","size":5,"soft_fail":false,"target_va":5411541584,"va":5368990001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42551440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e890488902","size":5,"soft_fail":false,"target_va":5411541808,"va":5368990363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<image::dynimage::DynamicImage>,enum2$<image::error::ImageError> > > >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-6215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8b9e7ffff","size":5,"soft_fail":false,"target_va":5368982976,"va":5368989186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e893e7ffff","size":5,"soft_fail":false,"target_va":5368983632,"va":5368989880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42552786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8d24d8902","size":5,"soft_fail":false,"target_va":5411542880,"va":5368990089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e851faffff","size":5,"soft_fail":false,"target_va":5368983888,"va":5368985338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42557940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8f4618902","size":5,"soft_fail":false,"target_va":5411543360,"va":5368985415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11965273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e85993b600","size":5,"soft_fail":false,"target_va":5380954320,"va":5368989042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$R$GT$::decode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-88973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e873a4feff","size":5,"soft_fail":false,"target_va":5368899984,"va":5368988952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$std..io..buffered..bufreader..BufReader$LT$std..fs..File$GT$$GT$::open_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11924884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e894f5b500","size":5,"soft_fail":false,"target_va":5380913680,"va":5368988791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12399683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e84334bd00","size":5,"soft_fail":false,"target_va":5381385536,"va":5368985848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12404872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e88848bd00","size":5,"soft_fail":false,"target_va":5381390640,"va":5368985763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12396852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e83429bd00","size":5,"soft_fail":false,"target_va":5381386480,"va":5368989623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12399381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e81533bd00","size":5,"soft_fail":false,"target_va":5381384704,"va":5368985318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42548584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8683d8902","size":5,"soft_fail":false,"target_va":5411538608,"va":5368990019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42548754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8123e8902","size":5,"soft_fail":false,"target_va":5411538768,"va":5368990009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42548853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8753e8902","size":5,"soft_fail":false,"target_va":5411538960,"va":5368990102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42547110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8a6378902","size":5,"soft_fail":false,"target_va":5411537104,"va":5368989989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::setup::find_best_fullscreen_video_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8f1110000","size":5,"soft_fail":false,"target_va":5368992704,"va":5368988106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::icon::Icon::from_rgba
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6689902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e86e146600","size":5,"soft_fail":false,"target_va":5375679024,"va":5368989117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6633382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8a6376500","size":5,"soft_fail":false,"target_va":5375621552,"va":5368988165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6634556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e83c3c6500","size":5,"soft_fail":false,"target_va":5375621904,"va":5368987343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::primary_monitor
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6634866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8723d6500","size":5,"soft_fail":false,"target_va":5375621872,"va":5368987001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t236885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368985264,"raw_hex":"e8559d0300","size":5,"soft_fail":false,"target_va":5369226208,"va":5368989318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5368985264,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5368987429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":15532,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":12,"record_offset":11320,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":12,"record_offset":12976,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5973,"source_line":1859,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":12,"record_offset":9528,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":12,"record_offset":9500,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":12,"record_offset":11576,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":12,"record_offset":10596,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":11008,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":19036,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":16508,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":12,"record_offset":16480,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5993,"source_line":688,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":12,"record_offset":16360,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5991,"source_line":1083,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":15564,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":15448,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":12,"record_offset":7072,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":12,"record_offset":5640,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":12,"record_offset":6096,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::setup::impl$1::from_winit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":12,"record_offset":15128,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5986,"source_line":206,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$18::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":14264,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5979,"source_line":302,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":12,"record_offset":17720,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5996,"source_line":305,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":11612,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5958,"source_line":125,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":12600,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5969,"source_line":118,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":17776,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5999,"source_line":508,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":8056,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5917,"source_line":205,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_fullscreen
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":15740,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5989,"source_line":318,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":11544,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5956,"source_line":222,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_position
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":14464,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5980,"source_line":274,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":8660,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5922,"source_line":307,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":9472,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5925,"source_line":340,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_window_icon
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":17416,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":5994,"source_line":406,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":5408,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":5408,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":9416,"parent_record_offset":9392,"ancestor_record_offset":5408,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":8468,"parent_record_offset":8440,"ancestor_record_offset":5408,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":9392,"parent_record_offset":9336,"ancestor_record_offset":5408,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":12,"record_offset":12096,"parent_record_offset":12068,"ancestor_record_offset":5408,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":8380,"parent_record_offset":8352,"ancestor_record_offset":5408,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":9364,"parent_record_offset":9336,"ancestor_record_offset":5408,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":5408,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":8440,"parent_record_offset":8408,"ancestor_record_offset":5408,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":8408,"parent_record_offset":8380,"ancestor_record_offset":5408,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":9312,"parent_record_offset":9288,"ancestor_record_offset":5408,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":8316,"parent_record_offset":8288,"ancestor_record_offset":5408,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":8288,"parent_record_offset":8260,"ancestor_record_offset":5408,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":8232,"parent_record_offset":8204,"ancestor_record_offset":5408,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":8260,"parent_record_offset":8232,"ancestor_record_offset":5408,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":8204,"parent_record_offset":8176,"ancestor_record_offset":5408,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":12,"record_offset":8176,"parent_record_offset":8140,"ancestor_record_offset":5408,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":12,"record_offset":8732,"parent_record_offset":8704,"ancestor_record_offset":5408,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":12,"record_offset":18024,"parent_record_offset":18000,"ancestor_record_offset":5408,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":12,"record_offset":18000,"parent_record_offset":17960,"ancestor_record_offset":5408,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":12,"record_offset":17576,"parent_record_offset":17548,"ancestor_record_offset":5408,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":8352,"parent_record_offset":8316,"ancestor_record_offset":5408,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":12,"record_offset":12128,"parent_record_offset":12096,"ancestor_record_offset":5408,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":12,"record_offset":7408,"parent_record_offset":7380,"ancestor_record_offset":5408,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":12,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":5408,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":12,"record_offset":7436,"parent_record_offset":7408,"ancestor_record_offset":5408,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":12,"record_offset":7356,"parent_record_offset":7328,"ancestor_record_offset":5408,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":12,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":5408,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":12,"record_offset":7552,"parent_record_offset":7380,"ancestor_record_offset":5408,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":12,"record_offset":7664,"parent_record_offset":7640,"ancestor_record_offset":5408,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":9684,"parent_record_offset":9560,"ancestor_record_offset":5408,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":12,"record_offset":8704,"parent_record_offset":8660,"ancestor_record_offset":5408,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$103::from
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":12,"record_offset":14108,"parent_record_offset":14084,"ancestor_record_offset":5408,"inlinee":5977,"source_line":77,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$108::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":12,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":5408,"inlinee":5955,"source_line":77,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":12,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":5408,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":12,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":5408,"inlinee":5946,"source_line":115,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":9948,"parent_record_offset":9560,"ancestor_record_offset":5408,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":12,"record_offset":7484,"parent_record_offset":7460,"ancestor_record_offset":5408,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":12,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":5408,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":12,"record_offset":10184,"parent_record_offset":10152,"ancestor_record_offset":5408,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":12,"record_offset":5796,"parent_record_offset":5764,"ancestor_record_offset":5408,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":12,"record_offset":7224,"parent_record_offset":7180,"ancestor_record_offset":5408,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":12,"record_offset":9772,"parent_record_offset":9740,"ancestor_record_offset":5408,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":12,"record_offset":9800,"parent_record_offset":9740,"ancestor_record_offset":5408,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":12,"record_offset":6044,"parent_record_offset":6016,"ancestor_record_offset":5408,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":12,"record_offset":8012,"parent_record_offset":7180,"ancestor_record_offset":5408,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":12,"record_offset":8568,"parent_record_offset":8316,"ancestor_record_offset":5408,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=68
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":7616,"parent_record_offset":7328,"ancestor_record_offset":5408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":12,"record_offset":11896,"parent_record_offset":11868,"ancestor_record_offset":5408,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":12,"record_offset":11944,"parent_record_offset":11920,"ancestor_record_offset":5408,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":12,"record_offset":11920,"parent_record_offset":11896,"ancestor_record_offset":5408,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":18048,"parent_record_offset":18024,"ancestor_record_offset":5408,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":5408,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":5408,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":6692,"parent_record_offset":6664,"ancestor_record_offset":5408,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":7508,"parent_record_offset":7484,"ancestor_record_offset":5408,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":11692,"parent_record_offset":11648,"ancestor_record_offset":5408,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":12688,"parent_record_offset":12640,"ancestor_record_offset":5408,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":11728,"parent_record_offset":11648,"ancestor_record_offset":5408,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":16412,"parent_record_offset":16360,"ancestor_record_offset":5408,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":16448,"parent_record_offset":16360,"ancestor_record_offset":5408,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":5408,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":5408,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":6764,"parent_record_offset":6736,"ancestor_record_offset":5408,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":6412,"parent_record_offset":6296,"ancestor_record_offset":5408,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":12,"record_offset":5764,"parent_record_offset":5736,"ancestor_record_offset":5408,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":12,"record_offset":6124,"parent_record_offset":6096,"ancestor_record_offset":5408,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":12,"record_offset":6264,"parent_record_offset":6232,"ancestor_record_offset":5408,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":12,"record_offset":6232,"parent_record_offset":6196,"ancestor_record_offset":5408,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":12,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":5408,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":12,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":5408,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":12,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":5408,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":12,"record_offset":6736,"parent_record_offset":6196,"ancestor_record_offset":5408,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":12,"record_offset":5736,"parent_record_offset":5704,"ancestor_record_offset":5408,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":12,"record_offset":5704,"parent_record_offset":5676,"ancestor_record_offset":5408,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":12,"record_offset":5896,"parent_record_offset":5864,"ancestor_record_offset":5408,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":12,"record_offset":5992,"parent_record_offset":5896,"ancestor_record_offset":5408,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":12,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":5408,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":12,"record_offset":5676,"parent_record_offset":5640,"ancestor_record_offset":5408,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":12,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":5408,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":12,"record_offset":6296,"parent_record_offset":6264,"ancestor_record_offset":5408,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":12,"record_offset":6580,"parent_record_offset":6296,"ancestor_record_offset":5408,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":12,"record_offset":6552,"parent_record_offset":6296,"ancestor_record_offset":5408,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":12,"record_offset":6356,"parent_record_offset":6296,"ancestor_record_offset":5408,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":18460,"parent_record_offset":18432,"ancestor_record_offset":5408,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":18832,"parent_record_offset":18644,"ancestor_record_offset":5408,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":18152,"parent_record_offset":18120,"ancestor_record_offset":5408,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":18924,"parent_record_offset":18892,"ancestor_record_offset":5408,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":17612,"parent_record_offset":17576,"ancestor_record_offset":5408,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":5408,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":18176,"parent_record_offset":18152,"ancestor_record_offset":5408,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":5408,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":18856,"parent_record_offset":18832,"ancestor_record_offset":5408,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":17636,"parent_record_offset":17612,"ancestor_record_offset":5408,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":18948,"parent_record_offset":18924,"ancestor_record_offset":5408,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":17668,"parent_record_offset":17576,"ancestor_record_offset":5408,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/io/reader.rs
//     evidence={"module":12,"record_offset":16384,"parent_record_offset":16360,"ancestor_record_offset":5408,"inlinee":5206,"source_line":293,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":9828,"parent_record_offset":9740,"ancestor_record_offset":5408,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":9716,"parent_record_offset":9560,"ancestor_record_offset":5408,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":5408,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":5408,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":9600,"parent_record_offset":9560,"ancestor_record_offset":5408,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":12,"record_offset":7108,"parent_record_offset":7072,"ancestor_record_offset":5408,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":12,"record_offset":19168,"parent_record_offset":19136,"ancestor_record_offset":5408,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":12,"record_offset":19684,"parent_record_offset":19424,"ancestor_record_offset":5408,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":12,"record_offset":7252,"parent_record_offset":7224,"ancestor_record_offset":5408,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":12,"record_offset":7328,"parent_record_offset":7180,"ancestor_record_offset":5408,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":12,"record_offset":7588,"parent_record_offset":7328,"ancestor_record_offset":5408,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":12,"record_offset":11868,"parent_record_offset":11840,"ancestor_record_offset":5408,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":12,"record_offset":11840,"parent_record_offset":11812,"ancestor_record_offset":5408,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":12,"record_offset":18332,"parent_record_offset":18304,"ancestor_record_offset":5408,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":12,"record_offset":18304,"parent_record_offset":18276,"ancestor_record_offset":5408,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":12,"record_offset":18644,"parent_record_offset":18608,"ancestor_record_offset":5408,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":12,"record_offset":18432,"parent_record_offset":18240,"ancestor_record_offset":5408,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":12,"record_offset":18240,"parent_record_offset":18212,"ancestor_record_offset":5408,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":12,"record_offset":18084,"parent_record_offset":17960,"ancestor_record_offset":5408,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":12,"record_offset":18212,"parent_record_offset":18084,"ancestor_record_offset":5408,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":12,"record_offset":18608,"parent_record_offset":18580,"ancestor_record_offset":5408,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":12,"record_offset":18120,"parent_record_offset":18084,"ancestor_record_offset":5408,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":12,"record_offset":18892,"parent_record_offset":18608,"ancestor_record_offset":5408,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":12,"record_offset":7280,"parent_record_offset":7252,"ancestor_record_offset":5408,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":12,"record_offset":7180,"parent_record_offset":7144,"ancestor_record_offset":5408,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":12,"record_offset":7144,"parent_record_offset":7108,"ancestor_record_offset":5408,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":12,"record_offset":18276,"parent_record_offset":18240,"ancestor_record_offset":5408,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::setup::setup_window::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":12,"record_offset":9560,"parent_record_offset":9528,"ancestor_record_offset":5408,"inlinee":5927,"source_line":23,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::setup::setup_window::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":12,"record_offset":10624,"parent_record_offset":10596,"ancestor_record_offset":5408,"inlinee":5947,"source_line":37,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":14548,"parent_record_offset":14524,"ancestor_record_offset":5408,"inlinee":5984,"source_line":281,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=18
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":11368,"parent_record_offset":11344,"ancestor_record_offset":5408,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":5408,"inlinee":5975,"source_line":308,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$30::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":11344,"parent_record_offset":11320,"ancestor_record_offset":5408,"inlinee":5951,"source_line":460,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":12772,"parent_record_offset":12744,"ancestor_record_offset":5408,"inlinee":5972,"source_line":466,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":14524,"parent_record_offset":14500,"ancestor_record_offset":5408,"inlinee":5982,"source_line":573,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":12,"record_offset":9892,"parent_record_offset":9560,"ancestor_record_offset":5408,"inlinee":5942,"source_line":313,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":12,"record_offset":10944,"parent_record_offset":10624,"ancestor_record_offset":5408,"inlinee":5948,"source_line":329,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":15160,"parent_record_offset":15128,"ancestor_record_offset":5408,"inlinee":5988,"source_line":51,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":12,"record_offset":17744,"parent_record_offset":17720,"ancestor_record_offset":5408,"inlinee":5998,"source_line":210,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":12,"record_offset":9916,"parent_record_offset":9892,"ancestor_record_offset":5408,"inlinee":5944,"source_line":490,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":12,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":5408,"inlinee":5949,"source_line":494,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":12,"record_offset":11648,"parent_record_offset":11612,"ancestor_record_offset":5408,"inlinee":5960,"source_line":211,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":12,"record_offset":12640,"parent_record_offset":12600,"ancestor_record_offset":5408,"inlinee":5970,"source_line":185,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":12,"record_offset":18992,"parent_record_offset":17960,"ancestor_record_offset":5408,"inlinee":6012,"source_line":348,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":12,"record_offset":17892,"parent_record_offset":17776,"ancestor_record_offset":5408,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":12,"record_offset":17836,"parent_record_offset":17776,"ancestor_record_offset":5408,"inlinee":6001,"source_line":100,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":12,"record_offset":17960,"parent_record_offset":17928,"ancestor_record_offset":5408,"inlinee":6005,"source_line":164,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":8092,"parent_record_offset":8056,"ancestor_record_offset":5408,"inlinee":5919,"source_line":116,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":8140,"parent_record_offset":8092,"ancestor_record_offset":5408,"inlinee":5921,"source_line":162,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":12,"record_offset":17928,"parent_record_offset":17892,"ancestor_record_offset":5408,"inlinee":6004,"source_line":514,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::setup_window::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1727
// 首条位置: {"module":12,"inlinee":5927}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":9684,"parent_record_offset":9560,"ancestor_record_offset":9560,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":9948,"parent_record_offset":9560,"ancestor_record_offset":9560,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":10004,"parent_record_offset":9560,"ancestor_record_offset":9560,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":9716,"parent_record_offset":9560,"ancestor_record_offset":9560,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":9600,"parent_record_offset":9560,"ancestor_record_offset":9560,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":12,"record_offset":9892,"parent_record_offset":9560,"ancestor_record_offset":9560,"inlinee":5942,"source_line":313,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":9560,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":10216,"parent_record_offset":10124,"ancestor_record_offset":9560,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":10152,"parent_record_offset":10124,"ancestor_record_offset":9560,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":10124,"parent_record_offset":10100,"ancestor_record_offset":9560,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":9560,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":12,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":9560,"inlinee":5946,"source_line":115,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":12,"record_offset":10184,"parent_record_offset":10152,"ancestor_record_offset":9560,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":12,"record_offset":9772,"parent_record_offset":9740,"ancestor_record_offset":9560,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":12,"record_offset":9800,"parent_record_offset":9740,"ancestor_record_offset":9560,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":9560,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":9560,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":9560,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":9828,"parent_record_offset":9740,"ancestor_record_offset":9560,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":9560,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":9560,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":12,"record_offset":9916,"parent_record_offset":9892,"ancestor_record_offset":9560,"inlinee":5944,"source_line":490,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::setup_window::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x379D
// 首条位置: {"module":184,"inlinee":14237}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":184,"record_offset":35184,"parent_record_offset":35044,"ancestor_record_offset":35044,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":36004,"parent_record_offset":35044,"ancestor_record_offset":35044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::render_config::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":184,"record_offset":35384,"parent_record_offset":35044,"ancestor_record_offset":35044,"inlinee":10677,"source_line":33,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":184,"record_offset":35216,"parent_record_offset":35044,"ancestor_record_offset":35044,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":184,"record_offset":35100,"parent_record_offset":35044,"ancestor_record_offset":35044,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":36228,"parent_record_offset":36204,"ancestor_record_offset":35044,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":36204,"parent_record_offset":36148,"ancestor_record_offset":35044,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":36176,"parent_record_offset":36148,"ancestor_record_offset":35044,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":36148,"parent_record_offset":36124,"ancestor_record_offset":35044,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":36124,"parent_record_offset":36100,"ancestor_record_offset":35044,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":184,"record_offset":35460,"parent_record_offset":35424,"ancestor_record_offset":35044,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":184,"record_offset":35488,"parent_record_offset":35460,"ancestor_record_offset":35044,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":184,"record_offset":35596,"parent_record_offset":35384,"ancestor_record_offset":35044,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":184,"record_offset":35620,"parent_record_offset":35596,"ancestor_record_offset":35044,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":184,"record_offset":35424,"parent_record_offset":35384,"ancestor_record_offset":35044,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":184,"record_offset":35268,"parent_record_offset":35240,"ancestor_record_offset":35044,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":184,"record_offset":35296,"parent_record_offset":35240,"ancestor_record_offset":35044,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":36028,"parent_record_offset":36004,"ancestor_record_offset":35044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":184,"record_offset":35516,"parent_record_offset":35488,"ancestor_record_offset":35044,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":184,"record_offset":35544,"parent_record_offset":35516,"ancestor_record_offset":35044,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":35124,"parent_record_offset":35100,"ancestor_record_offset":35044,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":35148,"parent_record_offset":35124,"ancestor_record_offset":35044,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":184,"record_offset":35324,"parent_record_offset":35240,"ancestor_record_offset":35044,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":184,"record_offset":35240,"parent_record_offset":35216,"ancestor_record_offset":35044,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":184,"record_offset":35348,"parent_record_offset":35324,"ancestor_record_offset":35044,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::setup_window::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x173B
// 首条位置: {"module":12,"inlinee":5947}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":10736,"parent_record_offset":10624,"ancestor_record_offset":10624,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":10768,"parent_record_offset":10624,"ancestor_record_offset":10624,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":10652,"parent_record_offset":10624,"ancestor_record_offset":10624,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":12,"record_offset":10944,"parent_record_offset":10624,"ancestor_record_offset":10624,"inlinee":5948,"source_line":329,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":12,"record_offset":10824,"parent_record_offset":10792,"ancestor_record_offset":10624,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":12,"record_offset":10852,"parent_record_offset":10792,"ancestor_record_offset":10624,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":10624,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":10624,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":10880,"parent_record_offset":10792,"ancestor_record_offset":10624,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":10624,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":10624,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":12,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":10624,"inlinee":5949,"source_line":494,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::find_best_fullscreen_video_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":12,"record_offset":20444,"rva":283584,"va":5368992704}
// 编译实例 RVAs: 0x453c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t13650581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8954ad000","size":5,"soft_fail":false,"target_va":5382644528,"va":5368993942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8e8c4ffff","size":5,"soft_fail":false,"target_va":5368983552,"va":5368998675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140175990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1242111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8fff31200","size":5,"soft_fail":false,"target_va":5370239376,"va":5368997260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402be600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2591851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e86b8c2700","size":5,"soft_fail":false,"target_va":5371586048,"va":5368994192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c8b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2632460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e80c2b2800","size":5,"soft_fail":false,"target_va":5371628320,"va":5368995855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403e5080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3796671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8bfee3900","size":5,"soft_fail":false,"target_va":5372792960,"va":5368996284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048bd20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4480745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8e95e4400","size":5,"soft_fail":false,"target_va":5373476128,"va":5368995378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4484452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8646d4400","size":5,"soft_fail":false,"target_va":5373479776,"va":5368995319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14052c590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5140003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8236e4e00","size":5,"soft_fail":false,"target_va":5374133648,"va":5368993640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5685288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e828c05600","size":5,"soft_fail":false,"target_va":5374679664,"va":5368994371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c3060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5753440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e860ca5700","size":5,"soft_fail":false,"target_va":5374750816,"va":5368997371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5814238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8deb75800","size":5,"soft_fail":false,"target_va":5374814320,"va":5369000077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6280
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5897019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e83bfb5900","size":5,"soft_fail":false,"target_va":5374894720,"va":5368997696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=3
//     target_aliases=92 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5902456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e878105a00","size":5,"soft_fail":false,"target_va":5374900736,"va":5368998275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=2
//     target_aliases=18 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5972961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8e1235b00","size":5,"soft_fail":false,"target_va":5374971760,"va":5368998794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fc1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5984812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e82c525b00","size":5,"soft_fail":false,"target_va":5374984656,"va":5368999839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060c210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6051692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e86c575c00","size":5,"soft_fail":false,"target_va":5375050256,"va":5368998559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6088490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e82ae75c00","size":5,"soft_fail":false,"target_va":5375081488,"va":5368992993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=26
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6088415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8dfe65c00","size":5,"soft_fail":false,"target_va":5375081504,"va":5368993084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6522269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e89d856300","size":5,"soft_fail":false,"target_va":5375516032,"va":5368993758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13834789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8251ad300","size":5,"soft_fail":false,"target_va":5382827728,"va":5368992934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42330519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e897e98502","size":5,"soft_fail":false,"target_va":5411324784,"va":5368994260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42852648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e828e18d02","size":5,"soft_fail":false,"target_va":5411847152,"va":5368994499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42854137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8f9e68d02","size":5,"soft_fail":false,"target_va":5411847168,"va":5368993026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42543400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e828298902","size":5,"soft_fail":false,"target_va":5411540992,"va":5368997587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42546287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e86f348902","size":5,"soft_fail":false,"target_va":5411540963,"va":5368994671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42544056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8b82b8902","size":5,"soft_fail":false,"target_va":5411541776,"va":5368997715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42544731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e85b2e8902","size":5,"soft_fail":false,"target_va":5411542064,"va":5368997328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42545371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8db308902","size":5,"soft_fail":false,"target_va":5411542880,"va":5368997504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5228228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8c4c64f00","size":5,"soft_fail":false,"target_va":5374226016,"va":5368997783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42541191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e887208902","size":5,"soft_fail":false,"target_va":5411538608,"va":5368997412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13835910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8861ed300","size":5,"soft_fail":false,"target_va":5382828864,"va":5368992949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42541368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e838218902","size":5,"soft_fail":false,"target_va":5411538768,"va":5368997395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42541427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e873218902","size":5,"soft_fail":false,"target_va":5411538960,"va":5368997528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6625518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8ee186500","size":5,"soft_fail":false,"target_va":5375621552,"va":5368996029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6629108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e8f4266500","size":5,"soft_fail":false,"target_va":5375621904,"va":5368992791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::decode_wide
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6620989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368992704,"raw_hex":"e83d076500","size":5,"soft_fail":false,"target_va":5375613920,"va":5368992926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*51696042(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5368992704,"raw_hex":"ff15aad11403","size":6,"soft_fail":false,"va":5368993456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":29136,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":6053,"source_line":641,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":24640,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":6034,"source_line":238,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":12,"record_offset":30136,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":24820,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":30828,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":12,"record_offset":31932,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":26836,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":24556,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":30884,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":6058,"source_line":3225,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":12,"record_offset":32080,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":28676,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":25540,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":34244,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":28500,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":26752,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":12,"record_offset":24848,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":12,"record_offset":25580,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":12,"record_offset":30772,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":12,"record_offset":25076,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":20672,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":6014,"source_line":112,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":27508,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":5958,"source_line":125,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":33044,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":6068,"source_line":137,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":34124,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":5969,"source_line":118,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":22668,"parent_record_offset":20444,"ancestor_record_offset":20444,"inlinee":6021,"source_line":164,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":21440,"parent_record_offset":21416,"ancestor_record_offset":20444,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":21464,"parent_record_offset":21440,"ancestor_record_offset":20444,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":21776,"parent_record_offset":21752,"ancestor_record_offset":20444,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":24020,"parent_record_offset":23992,"ancestor_record_offset":20444,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":21416,"parent_record_offset":21240,"ancestor_record_offset":20444,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":21752,"parent_record_offset":21728,"ancestor_record_offset":20444,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":12,"record_offset":23992,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":12,"record_offset":23336,"parent_record_offset":23308,"ancestor_record_offset":20444,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":12,"record_offset":24228,"parent_record_offset":24204,"ancestor_record_offset":20444,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\set.rs
//     evidence={"module":12,"record_offset":24176,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":6029,"source_line":905,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\set.rs
//     evidence={"module":12,"record_offset":22860,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":6024,"source_line":344,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\set.rs
//     evidence={"module":12,"record_offset":24204,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":6031,"source_line":1536,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":21180,"parent_record_offset":21152,"ancestor_record_offset":20444,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":22036,"parent_record_offset":22000,"ancestor_record_offset":20444,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":21728,"parent_record_offset":21704,"ancestor_record_offset":20444,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":21240,"parent_record_offset":21208,"ancestor_record_offset":20444,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":21208,"parent_record_offset":21180,"ancestor_record_offset":20444,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":21704,"parent_record_offset":21680,"ancestor_record_offset":20444,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":21276,"parent_record_offset":21240,"ancestor_record_offset":20444,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":21116,"parent_record_offset":21088,"ancestor_record_offset":20444,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":21088,"parent_record_offset":21060,"ancestor_record_offset":20444,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":21032,"parent_record_offset":21004,"ancestor_record_offset":20444,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":21060,"parent_record_offset":21032,"ancestor_record_offset":20444,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":21004,"parent_record_offset":20976,"ancestor_record_offset":20444,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":24676,"parent_record_offset":24640,"ancestor_record_offset":20444,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":12,"record_offset":29160,"parent_record_offset":29136,"ancestor_record_offset":20444,"inlinee":6055,"source_line":75,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":12,"record_offset":20976,"parent_record_offset":20948,"ancestor_record_offset":20444,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":12,"record_offset":20920,"parent_record_offset":20716,"ancestor_record_offset":20444,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$41::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":12,"record_offset":20948,"parent_record_offset":20920,"ancestor_record_offset":20444,"inlinee":6020,"source_line":2904,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":27288,"parent_record_offset":27264,"ancestor_record_offset":20444,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":27020,"parent_record_offset":26976,"ancestor_record_offset":20444,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":21152,"parent_record_offset":21116,"ancestor_record_offset":20444,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":24580,"parent_record_offset":24556,"ancestor_record_offset":20444,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":29620,"parent_record_offset":29596,"ancestor_record_offset":20444,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":12,"record_offset":32140,"parent_record_offset":32116,"ancestor_record_offset":20444,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":12,"record_offset":32388,"parent_record_offset":32364,"ancestor_record_offset":20444,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":12,"record_offset":31112,"parent_record_offset":31076,"ancestor_record_offset":20444,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":12,"record_offset":27264,"parent_record_offset":26976,"ancestor_record_offset":20444,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":12,"record_offset":26940,"parent_record_offset":26904,"ancestor_record_offset":20444,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":12,"record_offset":26976,"parent_record_offset":26940,"ancestor_record_offset":20444,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":12,"record_offset":23368,"parent_record_offset":23336,"ancestor_record_offset":20444,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":12,"record_offset":21300,"parent_record_offset":21276,"ancestor_record_offset":20444,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":12,"record_offset":21324,"parent_record_offset":21300,"ancestor_record_offset":20444,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":22972,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":23960,"parent_record_offset":23936,"ancestor_record_offset":20444,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":26396,"parent_record_offset":26368,"ancestor_record_offset":20444,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":12,"record_offset":25896,"parent_record_offset":25872,"ancestor_record_offset":20444,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":12,"record_offset":25980,"parent_record_offset":25956,"ancestor_record_offset":20444,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":12,"record_offset":21540,"parent_record_offset":21516,"ancestor_record_offset":20444,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":12,"record_offset":24752,"parent_record_offset":24712,"ancestor_record_offset":20444,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":12,"record_offset":21372,"parent_record_offset":21348,"ancestor_record_offset":20444,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":12,"record_offset":23896,"parent_record_offset":23872,"ancestor_record_offset":20444,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":12,"record_offset":24516,"parent_record_offset":24492,"ancestor_record_offset":20444,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":12,"record_offset":31832,"parent_record_offset":31028,"ancestor_record_offset":20444,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":12,"record_offset":31076,"parent_record_offset":31028,"ancestor_record_offset":20444,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":12,"record_offset":24604,"parent_record_offset":24580,"ancestor_record_offset":20444,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":27312,"parent_record_offset":27288,"ancestor_record_offset":20444,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":27336,"parent_record_offset":27312,"ancestor_record_offset":20444,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":24492,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":30976,"parent_record_offset":30884,"ancestor_record_offset":20444,"inlinee":6059,"source_line":3258,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by_key::key::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":31332,"parent_record_offset":31296,"ancestor_record_offset":20444,"inlinee":6063,"source_line":3232,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":31028,"parent_record_offset":30976,"ancestor_record_offset":20444,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":12,"record_offset":23848,"parent_record_offset":23824,"ancestor_record_offset":20444,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":12,"record_offset":23392,"parent_record_offset":23368,"ancestor_record_offset":20444,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":12,"record_offset":23824,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":12,"record_offset":21348,"parent_record_offset":21324,"ancestor_record_offset":20444,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":12,"record_offset":28804,"parent_record_offset":28776,"ancestor_record_offset":20444,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$3::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":12,"record_offset":31524,"parent_record_offset":31364,"ancestor_record_offset":20444,"inlinee":6067,"source_line":3449,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":12,"record_offset":26128,"parent_record_offset":26100,"ancestor_record_offset":20444,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":12,"record_offset":26100,"parent_record_offset":26072,"ancestor_record_offset":20444,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":12,"record_offset":31296,"parent_record_offset":31260,"ancestor_record_offset":20444,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":12,"record_offset":31260,"parent_record_offset":31076,"ancestor_record_offset":20444,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":12,"record_offset":31796,"parent_record_offset":31028,"ancestor_record_offset":20444,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":12,"record_offset":24316,"parent_record_offset":24284,"ancestor_record_offset":20444,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":12,"record_offset":24344,"parent_record_offset":24284,"ancestor_record_offset":20444,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":12,"record_offset":21516,"parent_record_offset":21116,"ancestor_record_offset":20444,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":21604,"parent_record_offset":20716,"ancestor_record_offset":20444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":12,"record_offset":30852,"parent_record_offset":30828,"ancestor_record_offset":20444,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":12,"record_offset":26248,"parent_record_offset":26224,"ancestor_record_offset":20444,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":12,"record_offset":23136,"parent_record_offset":23104,"ancestor_record_offset":20444,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":12,"record_offset":23184,"parent_record_offset":23160,"ancestor_record_offset":20444,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":12,"record_offset":23160,"parent_record_offset":23136,"ancestor_record_offset":20444,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":12,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":20444,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":31172,"parent_record_offset":31112,"ancestor_record_offset":20444,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":31200,"parent_record_offset":31112,"ancestor_record_offset":20444,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":31144,"parent_record_offset":31112,"ancestor_record_offset":20444,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":31224,"parent_record_offset":31200,"ancestor_record_offset":20444,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":24128,"parent_record_offset":24100,"ancestor_record_offset":20444,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":27584,"parent_record_offset":27540,"ancestor_record_offset":20444,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":33124,"parent_record_offset":33076,"ancestor_record_offset":20444,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":20792,"parent_record_offset":20716,"ancestor_record_offset":20444,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":12,"record_offset":27620,"parent_record_offset":27540,"ancestor_record_offset":20444,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":12,"record_offset":26480,"parent_record_offset":26452,"ancestor_record_offset":20444,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":12,"record_offset":26508,"parent_record_offset":26480,"ancestor_record_offset":20444,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":34272,"parent_record_offset":34244,"ancestor_record_offset":20444,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":12,"record_offset":24712,"parent_record_offset":24676,"ancestor_record_offset":20444,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":12,"record_offset":26424,"parent_record_offset":26396,"ancestor_record_offset":20444,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":25460,"parent_record_offset":25432,"ancestor_record_offset":20444,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":32884,"parent_record_offset":32696,"ancestor_record_offset":20444,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":24996,"parent_record_offset":24972,"ancestor_record_offset":20444,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":25148,"parent_record_offset":25116,"ancestor_record_offset":20444,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":32976,"parent_record_offset":32944,"ancestor_record_offset":20444,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":22912,"parent_record_offset":22888,"ancestor_record_offset":20444,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":25172,"parent_record_offset":25148,"ancestor_record_offset":20444,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":22936,"parent_record_offset":22912,"ancestor_record_offset":20444,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":32908,"parent_record_offset":32884,"ancestor_record_offset":20444,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":33000,"parent_record_offset":32976,"ancestor_record_offset":20444,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":12,"record_offset":26040,"parent_record_offset":26012,"ancestor_record_offset":20444,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":12,"record_offset":26072,"parent_record_offset":26040,"ancestor_record_offset":20444,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":12,"record_offset":26012,"parent_record_offset":25760,"ancestor_record_offset":20444,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":12,"record_offset":25872,"parent_record_offset":25760,"ancestor_record_offset":20444,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":12,"record_offset":26600,"parent_record_offset":25760,"ancestor_record_offset":20444,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":12,"record_offset":25956,"parent_record_offset":25760,"ancestor_record_offset":20444,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":12,"record_offset":25844,"parent_record_offset":25760,"ancestor_record_offset":20444,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":12,"record_offset":26368,"parent_record_offset":26340,"ancestor_record_offset":20444,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":12,"record_offset":25628,"parent_record_offset":25604,"ancestor_record_offset":20444,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":12,"record_offset":25652,"parent_record_offset":25628,"ancestor_record_offset":20444,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":12,"record_offset":26340,"parent_record_offset":26172,"ancestor_record_offset":20444,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":12,"record_offset":25684,"parent_record_offset":25652,"ancestor_record_offset":20444,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":12,"record_offset":26280,"parent_record_offset":26196,"ancestor_record_offset":20444,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":12,"record_offset":26224,"parent_record_offset":26196,"ancestor_record_offset":20444,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":12,"record_offset":26692,"parent_record_offset":25760,"ancestor_record_offset":20444,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":12,"record_offset":26196,"parent_record_offset":26172,"ancestor_record_offset":20444,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":12,"record_offset":25736,"parent_record_offset":25712,"ancestor_record_offset":20444,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":12,"record_offset":25712,"parent_record_offset":25652,"ancestor_record_offset":20444,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":12,"record_offset":25760,"parent_record_offset":25736,"ancestor_record_offset":20444,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":12,"record_offset":25816,"parent_record_offset":25760,"ancestor_record_offset":20444,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":12,"record_offset":26172,"parent_record_offset":25760,"ancestor_record_offset":20444,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":12,"record_offset":25604,"parent_record_offset":25580,"ancestor_record_offset":20444,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":12,"record_offset":30796,"parent_record_offset":30772,"ancestor_record_offset":20444,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":24372,"parent_record_offset":24284,"ancestor_record_offset":20444,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":24260,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":24284,"parent_record_offset":24260,"ancestor_record_offset":20444,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":20444,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":22888,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":12,"record_offset":24972,"parent_record_offset":24948,"ancestor_record_offset":20444,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":12,"record_offset":24872,"parent_record_offset":24848,"ancestor_record_offset":20444,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":12,"record_offset":24948,"parent_record_offset":24920,"ancestor_record_offset":20444,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":12,"record_offset":24896,"parent_record_offset":24872,"ancestor_record_offset":20444,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":12,"record_offset":24920,"parent_record_offset":24896,"ancestor_record_offset":20444,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":12,"record_offset":20892,"parent_record_offset":20716,"ancestor_record_offset":20444,"inlinee":6016,"source_line":906,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":12,"record_offset":20860,"parent_record_offset":20836,"ancestor_record_offset":20444,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":12,"record_offset":20836,"parent_record_offset":20716,"ancestor_record_offset":20444,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":12,"record_offset":23104,"parent_record_offset":23068,"ancestor_record_offset":20444,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":12,"record_offset":23068,"parent_record_offset":23036,"ancestor_record_offset":20444,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":12,"record_offset":25332,"parent_record_offset":25304,"ancestor_record_offset":20444,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":12,"record_offset":25304,"parent_record_offset":25276,"ancestor_record_offset":20444,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":12,"record_offset":32696,"parent_record_offset":32660,"ancestor_record_offset":20444,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":12,"record_offset":25432,"parent_record_offset":25236,"ancestor_record_offset":20444,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":12,"record_offset":25236,"parent_record_offset":25208,"ancestor_record_offset":20444,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":12,"record_offset":25208,"parent_record_offset":25076,"ancestor_record_offset":20444,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":12,"record_offset":32660,"parent_record_offset":32632,"ancestor_record_offset":20444,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":12,"record_offset":25116,"parent_record_offset":25076,"ancestor_record_offset":20444,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":12,"record_offset":32944,"parent_record_offset":32660,"ancestor_record_offset":20444,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":12,"record_offset":25276,"parent_record_offset":25236,"ancestor_record_offset":20444,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::setup::find_best_fullscreen_video_mode::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":12,"record_offset":34344,"parent_record_offset":34244,"ancestor_record_offset":20444,"inlinee":6070,"source_line":146,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::setup::find_best_fullscreen_video_mode::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":12,"record_offset":31364,"parent_record_offset":31332,"ancestor_record_offset":20444,"inlinee":6065,"source_line":163,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$73::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":34400,"parent_record_offset":34344,"ancestor_record_offset":20444,"inlinee":6072,"source_line":410,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":31436,"parent_record_offset":31364,"ancestor_record_offset":20444,"inlinee":5988,"source_line":51,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":12,"record_offset":20716,"parent_record_offset":20672,"ancestor_record_offset":20444,"inlinee":6015,"source_line":165,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":12,"record_offset":27540,"parent_record_offset":27508,"ancestor_record_offset":20444,"inlinee":5960,"source_line":211,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":12,"record_offset":33076,"parent_record_offset":33044,"ancestor_record_offset":20444,"inlinee":6069,"source_line":194,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":12,"record_offset":34160,"parent_record_offset":34124,"ancestor_record_offset":20444,"inlinee":5970,"source_line":185,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":12,"record_offset":22716,"parent_record_offset":22668,"ancestor_record_offset":20444,"inlinee":6022,"source_line":229,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::util::has_flag
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/util.rs
//     evidence={"module":12,"record_offset":23936,"parent_record_offset":22716,"ancestor_record_offset":20444,"inlinee":5213,"source_line":53,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::find_best_fullscreen_video_mode::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17B6
// 首条位置: {"module":12,"inlinee":6070}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] winit::dpi::impl$73::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":34400,"parent_record_offset":34344,"ancestor_record_offset":34344,"inlinee":6072,"source_line":410,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":34372,"parent_record_offset":34344,"ancestor_record_offset":34344,"inlinee":5988,"source_line":51,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":34424,"parent_record_offset":34400,"ancestor_record_offset":34344,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::find_best_fullscreen_video_mode::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17B1
// 首条位置: {"module":12,"inlinee":6065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$3::abs
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":12,"record_offset":31524,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":6067,"source_line":3449,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":31436,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":5988,"source_line":51,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::impl$0::from_winit
// 记录形式: compiled_procedure; instances=1
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":71,"record_offset":58872,"rva":1528912,"va":5370238032}
// 编译实例 RVAs: 0x175450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5382650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e8fa215200","size":5,"soft_fail":false,"target_va":5375621152,"va":5370238497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5382535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e887215200","size":5,"soft_fail":false,"target_va":5375621104,"va":5370238564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4661447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e8c7204700","size":5,"soft_fail":false,"target_va":5374900736,"va":5370239284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4843231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e8dfe64900","size":5,"soft_fail":false,"target_va":5375081488,"va":5370238252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4843159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e897e64900","size":5,"soft_fail":false,"target_va":5375081504,"va":5370238340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12589530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e8da19c000","size":5,"soft_fail":false,"target_va":5382827728,"va":5370238193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41608875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e8abe67a02","size":5,"soft_fail":false,"target_va":5411847168,"va":5370238288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41302181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e8a5387602","size":5,"soft_fail":false,"target_va":5411540963,"va":5370238777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41304127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e83f407602","size":5,"soft_fail":false,"target_va":5411542880,"va":5370238748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12590651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e83b1ec000","size":5,"soft_fail":false,"target_va":5382828864,"va":5370238208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t5383810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e882265200","size":5,"soft_fail":false,"target_va":5375621904,"va":5370238089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::decode_wide
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t5375730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370238032,"raw_hex":"e8f2065200","size":5,"soft_fail":false,"target_va":5375613920,"va":5370238185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5370238032,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5370238429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":71,"record_offset":61972,"parent_record_offset":58872,"ancestor_record_offset":58872,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":71,"record_offset":62556,"parent_record_offset":58872,"ancestor_record_offset":58872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":71,"record_offset":59012,"parent_record_offset":58872,"ancestor_record_offset":58872,"inlinee":6014,"source_line":112,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":71,"record_offset":61000,"parent_record_offset":58872,"ancestor_record_offset":58872,"inlinee":5958,"source_line":125,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":71,"record_offset":62180,"parent_record_offset":58872,"ancestor_record_offset":58872,"inlinee":5969,"source_line":118,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":71,"record_offset":59772,"parent_record_offset":59748,"ancestor_record_offset":58872,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":71,"record_offset":59796,"parent_record_offset":59772,"ancestor_record_offset":58872,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":71,"record_offset":60108,"parent_record_offset":60084,"ancestor_record_offset":58872,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":71,"record_offset":59748,"parent_record_offset":59572,"ancestor_record_offset":58872,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":71,"record_offset":60084,"parent_record_offset":60060,"ancestor_record_offset":58872,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":71,"record_offset":61468,"parent_record_offset":61440,"ancestor_record_offset":58872,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":71,"record_offset":59512,"parent_record_offset":59484,"ancestor_record_offset":58872,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":71,"record_offset":60368,"parent_record_offset":60332,"ancestor_record_offset":58872,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":71,"record_offset":60060,"parent_record_offset":60036,"ancestor_record_offset":58872,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":71,"record_offset":59572,"parent_record_offset":59540,"ancestor_record_offset":58872,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":71,"record_offset":59540,"parent_record_offset":59512,"ancestor_record_offset":58872,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":71,"record_offset":60036,"parent_record_offset":60012,"ancestor_record_offset":58872,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":71,"record_offset":59608,"parent_record_offset":59572,"ancestor_record_offset":58872,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":71,"record_offset":59448,"parent_record_offset":59420,"ancestor_record_offset":58872,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":71,"record_offset":59420,"parent_record_offset":59392,"ancestor_record_offset":58872,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":71,"record_offset":59364,"parent_record_offset":59336,"ancestor_record_offset":58872,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":71,"record_offset":59392,"parent_record_offset":59364,"ancestor_record_offset":58872,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":71,"record_offset":59336,"parent_record_offset":59308,"ancestor_record_offset":58872,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":71,"record_offset":59308,"parent_record_offset":59280,"ancestor_record_offset":58872,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":71,"record_offset":59252,"parent_record_offset":59052,"ancestor_record_offset":58872,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$41::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":71,"record_offset":59280,"parent_record_offset":59252,"ancestor_record_offset":58872,"inlinee":6020,"source_line":2904,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":71,"record_offset":59484,"parent_record_offset":59448,"ancestor_record_offset":58872,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":71,"record_offset":61500,"parent_record_offset":61468,"ancestor_record_offset":58872,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":71,"record_offset":59632,"parent_record_offset":59608,"ancestor_record_offset":58872,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":71,"record_offset":59656,"parent_record_offset":59632,"ancestor_record_offset":58872,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":71,"record_offset":62056,"parent_record_offset":62032,"ancestor_record_offset":58872,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$103::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":71,"record_offset":62080,"parent_record_offset":62056,"ancestor_record_offset":58872,"inlinee":5977,"source_line":77,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$108::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":71,"record_offset":62428,"parent_record_offset":62404,"ancestor_record_offset":58872,"inlinee":5955,"source_line":77,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":71,"record_offset":59872,"parent_record_offset":59848,"ancestor_record_offset":58872,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":71,"record_offset":59704,"parent_record_offset":59680,"ancestor_record_offset":58872,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":71,"record_offset":61524,"parent_record_offset":61500,"ancestor_record_offset":58872,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":71,"record_offset":59680,"parent_record_offset":59656,"ancestor_record_offset":58872,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":71,"record_offset":59848,"parent_record_offset":59448,"ancestor_record_offset":58872,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":71,"record_offset":59936,"parent_record_offset":59052,"ancestor_record_offset":58872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":71,"record_offset":61268,"parent_record_offset":61240,"ancestor_record_offset":58872,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":71,"record_offset":61316,"parent_record_offset":61292,"ancestor_record_offset":58872,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":71,"record_offset":61292,"parent_record_offset":61268,"ancestor_record_offset":58872,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":71,"record_offset":61068,"parent_record_offset":61032,"ancestor_record_offset":58872,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":71,"record_offset":59124,"parent_record_offset":59052,"ancestor_record_offset":58872,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":71,"record_offset":61100,"parent_record_offset":61032,"ancestor_record_offset":58872,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":71,"record_offset":59224,"parent_record_offset":59052,"ancestor_record_offset":58872,"inlinee":6016,"source_line":906,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":71,"record_offset":59192,"parent_record_offset":59168,"ancestor_record_offset":58872,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":71,"record_offset":59168,"parent_record_offset":59052,"ancestor_record_offset":58872,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":71,"record_offset":61240,"parent_record_offset":61212,"ancestor_record_offset":58872,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":71,"record_offset":61212,"parent_record_offset":61184,"ancestor_record_offset":58872,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":71,"record_offset":62032,"parent_record_offset":62000,"ancestor_record_offset":58872,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":71,"record_offset":62000,"parent_record_offset":61972,"ancestor_record_offset":58872,"inlinee":5975,"source_line":308,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":71,"record_offset":62348,"parent_record_offset":62316,"ancestor_record_offset":58872,"inlinee":5972,"source_line":466,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":71,"record_offset":59052,"parent_record_offset":59012,"ancestor_record_offset":58872,"inlinee":6015,"source_line":165,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":71,"record_offset":61032,"parent_record_offset":61000,"ancestor_record_offset":58872,"inlinee":5960,"source_line":211,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":71,"record_offset":62220,"parent_record_offset":62180,"ancestor_record_offset":58872,"inlinee":5970,"source_line":185,"archive":"_pdb/B/modules/00071.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::impl$1::from_winit
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=8
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1762
// 首条位置: {"module":12,"inlinee":5986}
// 编译实例 RVAs: 0x175990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5381692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370239376,"raw_hex":"e83c1e5200","size":5,"soft_fail":false,"target_va":5375621104,"va":5370239407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t5382154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370239376,"raw_hex":"e80a205200","size":5,"soft_fail":false,"target_va":5375621552,"va":5370239393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":12,"record_offset":15188,"parent_record_offset":15128,"ancestor_record_offset":15128,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":12,"record_offset":15160,"parent_record_offset":15128,"ancestor_record_offset":15128,"inlinee":5988,"source_line":51,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":12,"record_offset":15264,"parent_record_offset":15240,"ancestor_record_offset":15128,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$108::from
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":12,"record_offset":15288,"parent_record_offset":15264,"ancestor_record_offset":15128,"inlinee":5955,"source_line":77,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":15240,"parent_record_offset":15212,"ancestor_record_offset":15128,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":12,"record_offset":15212,"parent_record_offset":15188,"ancestor_record_offset":15128,"inlinee":5972,"source_line":466,"archive":"_pdb/B/modules/00012.jsonl.gz"}
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
// 完整结构及原始类型记录: setup.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: setup.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: setup.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::find_best_fullscreen_video_mode
// 信息 ID: eb4efc4daac6d66c0e4719eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x441ca -> 0x453c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::find_best_fullscreen_video_mode::closure$0
// 信息 ID: 493a606eb2303643824e0af3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::setup::find_best_fullscreen_video_mode; module=12, inline_record=34344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::find_best_fullscreen_video_mode::closure$1
// 信息 ID: 3c9b542e27fed97224468f48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d96c03f858954aa9834a31e1: core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1 <- alloc::vec::into_iter::impl$5::fold<winit::monitor::VideoMode,alloc::alloc::Global,tuple$<i64,winit::monitor::VideoMode>,core::iter::adapters::map::map_fold::closure_env$0<winit::monitor::VideoMode,tuple$<i64,winit::monitor::VideoMode>,tuple$<i64,winit::monitor::VideoMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<winit::monitor::VideoMode,i64,tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,winit::monitor::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<winit::monitor::VideoMode,i64,tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] tiny_glade::setup::find_best_fullscreen_video_mode; module=12, inline_record=31364; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::fold<winit::monitor::VideoMode,alloc::alloc::Global,tuple$<i64,winit::monitor::VideoMode>,core::iter::adapters::map::map_fold::closure_env$0<winit::monitor::VideoMode,tuple$<i64,winit::monitor::VideoMode>,tuple$<i64,winit::monitor::VideoMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<winit::monitor::VideoMode,i64,tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,winit::monitor::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)> > >; module=175, inline_record=9812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::impl$0::from_winit
// 信息 ID: 53a4d5ee4982a1bd7041fb8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43da8 -> 0x175450 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::setup::setup_window::closure$0::closure_env$0> > > @ 0x48ec3d -> 0x175450 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::event_loop::run_event_loop::closure_env$2> > > @ 0x48f17f -> 0x175450 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::game_loop::GameLoop::extract_available_monitors_info @ 0x4e25f6 -> 0x175450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::impl$1::from_winit
// 信息 ID: 44786445053563630e3fe581
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d79336355d1adce860522d53: country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x4658c -> 0x175990 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tiny_glade::setup::setup_window; module=12, inline_record=15128; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::event_loop::run_event_loop; module=92, inline_record=18956; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::render_config::VideoMode,core::iter::adapters::map::Map<core::slice::iter::Iter<winit::monitor::VideoMode>,country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)> >; module=104, inline_record=130816; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::setup_window
// 信息 ID: 7c6247b5063afd7b7cda1edd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x594157 -> 0x436b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::setup_window::closure$0
// 信息 ID: c46b01156ed60170745db9c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::setup::setup_window", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::setup::setup_window; module=12, inline_record=9560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::setup_window::closure$0::closure$0
// 信息 ID: 332cb3db5e183dfc7e4c466f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::setup::setup_window", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::setup::setup_window::closure$0::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::setup::setup_window::closure$0::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::setup::setup_window::closure$0::closure_env$0 <- alloc::collections::vec_deque::into_iter::impl$2::try_fold<winit::platform_impl::platform::monitor::MonitorHandle,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::setup::setup_window::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::setup::setup_window::closure$0::closure_env$0> > >; module=184, inline_record=35044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::setup_window::closure$1
// 信息 ID: 84159096d8f383c4f218fea7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::setup::setup_window", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::setup::setup_window; module=12, inline_record=10624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
