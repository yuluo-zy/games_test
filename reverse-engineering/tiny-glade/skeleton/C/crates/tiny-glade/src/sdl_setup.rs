// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/sdl_setup.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e56dc0165e662428b16c38d5962be18b746a8d3680bc1c436c4a61ff33bb5899
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sdl_setup.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 8, 55, 81, 112, 122, 159, 177, 188, 228
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use usdl; // namespace candidate
//   package identities: [{"name":"usdl","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::find_best_fullscreen_video_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":32520,"rva":4167968,"va":5372877088}
// 编译实例 RVAs: 0x3f9920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11917215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e89fd7b500","size":5,"soft_fail":false,"target_va":5384795984,"va":5372878764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t43736408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8585d9b02","size":5,"soft_fail":false,"target_va":5416616112,"va":5372879699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t43736488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8a85d9b02","size":5,"soft_fail":false,"target_va":5416616304,"va":5372879811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t11731683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8e302b300","size":5,"soft_fail":false,"target_va":5384609408,"va":5372877720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2801482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e84abf2a00","size":5,"soft_fail":false,"target_va":5375678688,"va":5372877201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=18
//     evidence={"assembly":"\tcallq\t2801430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e816bf2a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372877269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4154999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e88999c0ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5372877186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3639784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e81876c8ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5372880227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3615791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8d1d3c8ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5372880906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3449656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8c85ccbff","size":5,"soft_fail":false,"target_va":5369431360,"va":5372881011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c9e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3343137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8dffcccff","size":5,"soft_fail":false,"target_va":5369536144,"va":5372879276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d34f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2256426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8d691ddff","size":5,"soft_fail":false,"target_va":5370623216,"va":5372879637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403048f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1004687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e871abf0ff","size":5,"soft_fail":false,"target_va":5371873520,"va":5372878202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040fae0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t88855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8175b0100","size":5,"soft_fail":false,"target_va":5372967648,"va":5372878788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d130
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t603316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8b4340900","size":5,"soft_fail":false,"target_va":5373481264,"va":5372877943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140549890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1373291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e86bf41400","size":5,"soft_fail":false,"target_va":5374253200,"va":5372879904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069b420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2758262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e876162a00","size":5,"soft_fail":false,"target_va":5375636512,"va":5372878245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43473326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8ae599702","size":5,"soft_fail":false,"target_va":5416350832,"va":5372877501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43479585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e821729702","size":5,"soft_fail":false,"target_va":5416359264,"va":5372879674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44178950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8061ea202","size":5,"soft_fail":false,"target_va":5417057040,"va":5372878085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43740910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8ee6e9b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5372879879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43741838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e88e729b02","size":5,"soft_fail":false,"target_va":5416621632,"va":5372879789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t43739974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8466b9b02","size":5,"soft_fail":false,"target_va":5416619696,"va":5372879717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::PixelFormatEnum::byte_size_per_pixel
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2953643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8ab112d00","size":5,"soft_fail":false,"target_va":5375832512,"va":5372878864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::current_display_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2953039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e84f0f2d00","size":5,"soft_fail":false,"target_va":5375831872,"va":5372878828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::desktop_display_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2953153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8c10f2d00","size":5,"soft_fail":false,"target_va":5375832064,"va":5372878906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::display_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2953866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e88a122d00","size":5,"soft_fail":false,"target_va":5375831328,"va":5372877457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::display_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2954252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e80c142d00","size":5,"soft_fail":false,"target_va":5375831424,"va":5372877167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::num_display_modes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2954432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372877088,"raw_hex":"e8c0142d00","size":5,"soft_fail":false,"target_va":5375831744,"va":5372877307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":37496,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":11201,"source_line":596,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":35088,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":8431,"source_line":247,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":35060,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":33872,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":33900,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":42232,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":34292,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":122,"record_offset":34124,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":122,"record_offset":37440,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":122,"record_offset":39752,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":9018,"source_line":3350,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":34460,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":36100,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":32740,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":122,"record_offset":37356,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":34348,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":34208,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":122,"record_offset":35396,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":122,"record_offset":36140,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":122,"record_offset":39152,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":122,"record_offset":35640,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_setup::video_mode_from_sdl
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":39208,"parent_record_offset":32520,"ancestor_record_offset":32520,"inlinee":13040,"source_line":267,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33112,"parent_record_offset":33088,"ancestor_record_offset":32520,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33136,"parent_record_offset":33112,"ancestor_record_offset":32520,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33468,"parent_record_offset":33444,"ancestor_record_offset":32520,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33492,"parent_record_offset":33468,"ancestor_record_offset":32520,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33160,"parent_record_offset":33136,"ancestor_record_offset":32520,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33520,"parent_record_offset":33492,"ancestor_record_offset":32520,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33088,"parent_record_offset":33060,"ancestor_record_offset":32520,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33444,"parent_record_offset":33388,"ancestor_record_offset":32520,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":34000,"parent_record_offset":33968,"ancestor_record_offset":32520,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33000,"parent_record_offset":32972,"ancestor_record_offset":32520,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33416,"parent_record_offset":33388,"ancestor_record_offset":32520,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33388,"parent_record_offset":33364,"ancestor_record_offset":32520,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":34048,"parent_record_offset":34024,"ancestor_record_offset":32520,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":34024,"parent_record_offset":34000,"ancestor_record_offset":32520,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33060,"parent_record_offset":33028,"ancestor_record_offset":32520,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33028,"parent_record_offset":33000,"ancestor_record_offset":32520,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33364,"parent_record_offset":33340,"ancestor_record_offset":32520,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":32880,"parent_record_offset":32840,"ancestor_record_offset":32520,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":32908,"parent_record_offset":32880,"ancestor_record_offset":32520,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":32936,"parent_record_offset":32908,"ancestor_record_offset":32520,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":32840,"parent_record_offset":32812,"ancestor_record_offset":32520,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":35124,"parent_record_offset":35088,"ancestor_record_offset":32520,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":122,"record_offset":37520,"parent_record_offset":37496,"ancestor_record_offset":32520,"inlinee":11203,"source_line":75,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":122,"record_offset":32812,"parent_record_offset":32780,"ancestor_record_offset":32520,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":122,"record_offset":36900,"parent_record_offset":36872,"ancestor_record_offset":32520,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":33968,"parent_record_offset":33928,"ancestor_record_offset":32520,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":33928,"parent_record_offset":33900,"ancestor_record_offset":32520,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":32972,"parent_record_offset":32936,"ancestor_record_offset":32520,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":37464,"parent_record_offset":37440,"ancestor_record_offset":32520,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":37976,"parent_record_offset":37948,"ancestor_record_offset":32520,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":40436,"parent_record_offset":40412,"ancestor_record_offset":32520,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":40484,"parent_record_offset":40460,"ancestor_record_offset":32520,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":39924,"parent_record_offset":39892,"ancestor_record_offset":32520,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":122,"record_offset":36928,"parent_record_offset":36900,"ancestor_record_offset":32520,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":39664,"parent_record_offset":39632,"ancestor_record_offset":32520,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":34172,"parent_record_offset":34148,"ancestor_record_offset":32520,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":36872,"parent_record_offset":36844,"ancestor_record_offset":32520,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":122,"record_offset":36432,"parent_record_offset":36408,"ancestor_record_offset":32520,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":122,"record_offset":36516,"parent_record_offset":36492,"ancestor_record_offset":32520,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":122,"record_offset":35200,"parent_record_offset":35160,"ancestor_record_offset":32520,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":122,"record_offset":40356,"parent_record_offset":39844,"ancestor_record_offset":32520,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":122,"record_offset":39892,"parent_record_offset":39844,"ancestor_record_offset":32520,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":122,"record_offset":34148,"parent_record_offset":34124,"ancestor_record_offset":32520,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":122,"record_offset":39800,"parent_record_offset":39752,"ancestor_record_offset":32520,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by_key::key::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":122,"record_offset":40140,"parent_record_offset":40108,"ancestor_record_offset":32520,"inlinee":5406,"source_line":3357,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":122,"record_offset":39844,"parent_record_offset":39800,"ancestor_record_offset":32520,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":38444,"parent_record_offset":38416,"ancestor_record_offset":32520,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$3::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":122,"record_offset":40252,"parent_record_offset":40172,"ancestor_record_offset":32520,"inlinee":5412,"source_line":3604,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":122,"record_offset":36664,"parent_record_offset":36636,"ancestor_record_offset":32520,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":122,"record_offset":36636,"parent_record_offset":36608,"ancestor_record_offset":32520,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":122,"record_offset":40108,"parent_record_offset":40076,"ancestor_record_offset":32520,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":34316,"parent_record_offset":34292,"ancestor_record_offset":32520,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":40076,"parent_record_offset":39892,"ancestor_record_offset":32520,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":41192,"parent_record_offset":39752,"ancestor_record_offset":32520,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":122,"record_offset":33216,"parent_record_offset":32936,"ancestor_record_offset":32520,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":33240,"parent_record_offset":33216,"ancestor_record_offset":32520,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":33292,"parent_record_offset":32780,"ancestor_record_offset":32520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":35296,"parent_record_offset":35256,"ancestor_record_offset":32520,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":36780,"parent_record_offset":36756,"ancestor_record_offset":32520,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":40044,"parent_record_offset":39924,"ancestor_record_offset":32520,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":39988,"parent_record_offset":39924,"ancestor_record_offset":32520,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":39960,"parent_record_offset":39924,"ancestor_record_offset":32520,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":40012,"parent_record_offset":39988,"ancestor_record_offset":32520,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":34088,"parent_record_offset":33928,"ancestor_record_offset":32520,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":122,"record_offset":36956,"parent_record_offset":36928,"ancestor_record_offset":32520,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":122,"record_offset":36984,"parent_record_offset":36956,"ancestor_record_offset":32520,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":122,"record_offset":37380,"parent_record_offset":37356,"ancestor_record_offset":32520,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":122,"record_offset":35256,"parent_record_offset":35160,"ancestor_record_offset":32520,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":122,"record_offset":35160,"parent_record_offset":35124,"ancestor_record_offset":32520,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":36020,"parent_record_offset":35992,"ancestor_record_offset":32520,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":41476,"parent_record_offset":41288,"ancestor_record_offset":32520,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":35560,"parent_record_offset":35536,"ancestor_record_offset":32520,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":35712,"parent_record_offset":35680,"ancestor_record_offset":32520,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":41568,"parent_record_offset":41536,"ancestor_record_offset":32520,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":34232,"parent_record_offset":34208,"ancestor_record_offset":32520,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":35736,"parent_record_offset":35712,"ancestor_record_offset":32520,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":34256,"parent_record_offset":34232,"ancestor_record_offset":32520,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":41500,"parent_record_offset":41476,"ancestor_record_offset":32520,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":41592,"parent_record_offset":41568,"ancestor_record_offset":32520,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":122,"record_offset":36576,"parent_record_offset":36548,"ancestor_record_offset":32520,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":122,"record_offset":36608,"parent_record_offset":36576,"ancestor_record_offset":32520,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":122,"record_offset":36548,"parent_record_offset":36296,"ancestor_record_offset":32520,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":122,"record_offset":36408,"parent_record_offset":36296,"ancestor_record_offset":32520,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":122,"record_offset":37068,"parent_record_offset":36296,"ancestor_record_offset":32520,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":122,"record_offset":36492,"parent_record_offset":36296,"ancestor_record_offset":32520,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":122,"record_offset":36352,"parent_record_offset":36296,"ancestor_record_offset":32520,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":122,"record_offset":36844,"parent_record_offset":36816,"ancestor_record_offset":32520,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":122,"record_offset":36188,"parent_record_offset":36164,"ancestor_record_offset":32520,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":122,"record_offset":36816,"parent_record_offset":36708,"ancestor_record_offset":32520,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":122,"record_offset":36220,"parent_record_offset":36188,"ancestor_record_offset":32520,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":122,"record_offset":36756,"parent_record_offset":36732,"ancestor_record_offset":32520,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":122,"record_offset":37160,"parent_record_offset":36296,"ancestor_record_offset":32520,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":122,"record_offset":36732,"parent_record_offset":36708,"ancestor_record_offset":32520,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":122,"record_offset":36272,"parent_record_offset":36248,"ancestor_record_offset":32520,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":122,"record_offset":36248,"parent_record_offset":36188,"ancestor_record_offset":32520,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":122,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":32520,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":122,"record_offset":36380,"parent_record_offset":36296,"ancestor_record_offset":32520,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":122,"record_offset":36708,"parent_record_offset":36296,"ancestor_record_offset":32520,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":122,"record_offset":36164,"parent_record_offset":36140,"ancestor_record_offset":32520,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":122,"record_offset":39176,"parent_record_offset":39152,"ancestor_record_offset":32520,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":34396,"parent_record_offset":34372,"ancestor_record_offset":32520,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":34372,"parent_record_offset":34348,"ancestor_record_offset":32520,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":34420,"parent_record_offset":34396,"ancestor_record_offset":32520,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":122,"record_offset":35536,"parent_record_offset":35512,"ancestor_record_offset":32520,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":122,"record_offset":35424,"parent_record_offset":35396,"ancestor_record_offset":32520,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":122,"record_offset":35512,"parent_record_offset":35480,"ancestor_record_offset":32520,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":122,"record_offset":35452,"parent_record_offset":35424,"ancestor_record_offset":32520,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":122,"record_offset":35480,"parent_record_offset":35452,"ancestor_record_offset":32520,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":122,"record_offset":35892,"parent_record_offset":35864,"ancestor_record_offset":32520,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":122,"record_offset":35864,"parent_record_offset":35836,"ancestor_record_offset":32520,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":122,"record_offset":41288,"parent_record_offset":41252,"ancestor_record_offset":32520,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":122,"record_offset":35992,"parent_record_offset":35800,"ancestor_record_offset":32520,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":122,"record_offset":35800,"parent_record_offset":35772,"ancestor_record_offset":32520,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":122,"record_offset":35772,"parent_record_offset":35640,"ancestor_record_offset":32520,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":122,"record_offset":41252,"parent_record_offset":41224,"ancestor_record_offset":32520,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":122,"record_offset":35680,"parent_record_offset":35640,"ancestor_record_offset":32520,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":122,"record_offset":41536,"parent_record_offset":41252,"ancestor_record_offset":32520,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":122,"record_offset":35836,"parent_record_offset":35800,"ancestor_record_offset":32520,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":32780,"parent_record_offset":32740,"ancestor_record_offset":32520,"inlinee":13043,"source_line":187,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":40172,"parent_record_offset":40140,"ancestor_record_offset":32520,"inlinee":5408,"source_line":242,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::mode_bit_depth
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":39232,"parent_record_offset":39208,"ancestor_record_offset":32520,"inlinee":5410,"source_line":275,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::init_sdl
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":43868,"rva":4171952,"va":5372881072}
// 编译实例 RVAs: 0x3fa8b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400999d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3543176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372881072,"raw_hex":"e878efc9ff","size":5,"soft_fail":false,"target_va":5369338320,"va":5372881491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cb770
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2952623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372881072,"raw_hex":"e8af0d2d00","size":5,"soft_fail":false,"target_va":5375833968,"va":5372881340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cb7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2952681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372881072,"raw_hex":"e8e90d2d00","size":5,"soft_fail":false,"target_va":5375834016,"va":5372881330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d67620
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43437250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372881072,"raw_hex":"e8c2cc9602","size":5,"soft_fail":false,"target_va":5416318496,"va":5372881241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43468799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372881072,"raw_hex":"e8ff479702","size":5,"soft_fail":false,"target_va":5416350000,"va":5372881196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::event::set_event_enabled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2948332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372881072,"raw_hex":"e8ecfc2c00","size":5,"soft_fail":false,"target_va":5375829456,"va":5372881119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::sdl::Sdl::event_pump
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2947230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372881072,"raw_hex":"e89ef82c00","size":5,"soft_fail":false,"target_va":5375828368,"va":5372881133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::sdl::Sdl::video
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2947448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372881072,"raw_hex":"e878f92c00","size":5,"soft_fail":false,"target_va":5375828608,"va":5372881155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::sdl::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2947732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372881072,"raw_hex":"e894fa2c00","size":5,"soft_fail":false,"target_va":5375828832,"va":5372881095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":122,"record_offset":44172,"parent_record_offset":43868,"ancestor_record_offset":43868,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":44356,"parent_record_offset":43868,"ancestor_record_offset":43868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":43992,"parent_record_offset":43868,"ancestor_record_offset":43868,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":122,"record_offset":44220,"parent_record_offset":44196,"ancestor_record_offset":43868,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":122,"record_offset":44196,"parent_record_offset":44172,"ancestor_record_offset":43868,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":122,"record_offset":44024,"parent_record_offset":43992,"ancestor_record_offset":43868,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":44384,"parent_record_offset":44356,"ancestor_record_offset":43868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::init_sdl::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":44596,"rva":47640880,"va":5416350000}
// 编译实例 RVAs: 0x2d6f130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-40671381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416350000,"raw_hex":"e86b6793fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416350080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d69060
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416350000,"raw_hex":"e8fa9effff","size":5,"soft_fail":false,"target_va":5416325216,"va":5416350049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":44724,"parent_record_offset":44596,"ancestor_record_offset":44596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":44888,"parent_record_offset":44864,"ancestor_record_offset":44596,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":44912,"parent_record_offset":44888,"ancestor_record_offset":44596,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":44936,"parent_record_offset":44912,"ancestor_record_offset":44596,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":44864,"parent_record_offset":44808,"ancestor_record_offset":44596,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":44836,"parent_record_offset":44808,"ancestor_record_offset":44596,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":44808,"parent_record_offset":44784,"ancestor_record_offset":44596,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":44784,"parent_record_offset":44764,"ancestor_record_offset":44596,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":44744,"parent_record_offset":44724,"ancestor_record_offset":44596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::SdlWindowBackend::setup_window
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":19904,"rva":4163024,"va":5372872144}
// 编译实例 RVAs: 0x3f85d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11922664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8e8ecb500","size":5,"soft_fail":false,"target_va":5384795984,"va":5372873315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11785059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e863d3b300","size":5,"soft_fail":false,"target_va":5384659264,"va":5372874200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t11736970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e88a17b300","size":5,"soft_fail":false,"target_va":5384609408,"va":5372872433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=19
//     evidence={"assembly":"\tcallq\t2806055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e827d12a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372872644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3537902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e81204caff","size":5,"soft_fail":false,"target_va":5369337680,"va":5372875577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009bcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3528779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8b527caff","size":5,"soft_fail":false,"target_va":5369347248,"va":5372876022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c5a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3356542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e882c8ccff","size":5,"soft_fail":false,"target_va":5369518704,"va":5372875241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cac10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3332935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8b924cdff","size":5,"soft_fail":false,"target_va":5369539600,"va":5372872530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ee920
//     owner=unresolved_namespace; count=2
//     target_aliases=248 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3187424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8205dcfff","size":5,"soft_fail":false,"target_va":5369686304,"va":5372873723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f10
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3101668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e81cacd0ff","size":5,"soft_fail":false,"target_va":5369773840,"va":5372875503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1224153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8d9ad1200","size":5,"soft_fail":false,"target_va":5374098704,"va":5372874546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cb770
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2959596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8ec282d00","size":5,"soft_fail":false,"target_va":5375833968,"va":5372874367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cb7b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2959670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e836292d00","size":5,"soft_fail":false,"target_va":5375834032,"va":5372874357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d67700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43444395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8abe89602","size":5,"soft_fail":false,"target_va":5416318720,"va":5372874320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_setup::enumerate_displays
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e86b0e0000","size":5,"soft_fail":false,"target_va":5372876144,"va":5372872448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_setup::find_best_fullscreen_video_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8180c0000","size":5,"soft_fail":false,"target_va":5372877088,"va":5372873987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::rect::Rect::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2955734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8d6192d00","size":5,"soft_fail":false,"target_va":5375829440,"va":5372873701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::rect::Rect::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2955560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e828192d00","size":5,"soft_fail":false,"target_va":5375829376,"va":5372873811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::rect::Rect::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2955744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8e0192d00","size":5,"soft_fail":false,"target_va":5375829424,"va":5372873675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::rect::Rect::y
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2955616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e860192d00","size":5,"soft_fail":false,"target_va":5375829360,"va":5372873739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::PixelFormatEnum::byte_size_per_pixel
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2958423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e857242d00","size":5,"soft_fail":false,"target_va":5375832512,"va":5372874084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::current_video_driver
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2958498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8a2242d00","size":5,"soft_fail":false,"target_va":5375831968,"va":5372873465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::display_bounds
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2958079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8ff222d00","size":5,"soft_fail":false,"target_va":5375831648,"va":5372873564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::window
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2958699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e86b252d00","size":5,"soft_fail":false,"target_va":5375832160,"va":5372873456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::set_display_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2959102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8fe262d00","size":5,"soft_fail":false,"target_va":5375833136,"va":5372874029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::set_fullscreen
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2958955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e86b262d00","size":5,"soft_fail":false,"target_va":5375833024,"va":5372874064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::WindowBuilder::allow_highdpi
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2957283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8e31f2d00","size":5,"soft_fail":false,"target_va":5375830816,"va":5372873528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::WindowBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2956911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e86f1e2d00","size":5,"soft_fail":false,"target_va":5375830832,"va":5372873916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::WindowBuilder::hidden
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2957743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8af212d00","size":5,"soft_fail":false,"target_va":5375831248,"va":5372873500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::WindowBuilder::position
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2957510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8c6202d00","size":5,"soft_fail":false,"target_va":5375831280,"va":5372873765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::WindowBuilder::resizable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2957797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8e5212d00","size":5,"soft_fail":false,"target_va":5375831312,"va":5372873510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::WindowBuilder::vulkan
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2957740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372872144,"raw_hex":"e8ac212d00","size":5,"soft_fail":false,"target_va":5375831264,"va":5372873519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":122,"record_offset":25972,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":24788,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":25604,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":13037,"source_line":2344,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":20108,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":20080,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":22736,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":24420,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":24960,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::render_config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":122,"record_offset":24348,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":11479,"source_line":36,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_setup::apply_fullscreen
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":26120,"parent_record_offset":19904,"ancestor_record_offset":19904,"inlinee":13039,"source_line":117,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21228,"parent_record_offset":21204,"ancestor_record_offset":19904,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21252,"parent_record_offset":21228,"ancestor_record_offset":19904,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21276,"parent_record_offset":21252,"ancestor_record_offset":19904,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21204,"parent_record_offset":21148,"ancestor_record_offset":19904,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":21176,"parent_record_offset":21148,"ancestor_record_offset":19904,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":21148,"parent_record_offset":21124,"ancestor_record_offset":19904,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":21124,"parent_record_offset":21100,"ancestor_record_offset":19904,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":20480,"parent_record_offset":20436,"ancestor_record_offset":19904,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":20728,"parent_record_offset":20696,"ancestor_record_offset":19904,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":20696,"parent_record_offset":20668,"ancestor_record_offset":19904,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":21376,"parent_record_offset":21352,"ancestor_record_offset":19904,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":23192,"parent_record_offset":23052,"ancestor_record_offset":19904,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":20812,"parent_record_offset":20784,"ancestor_record_offset":19904,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":122,"record_offset":26052,"parent_record_offset":26012,"ancestor_record_offset":19904,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":122,"record_offset":26012,"parent_record_offset":25972,"ancestor_record_offset":19904,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":26296,"parent_record_offset":26232,"ancestor_record_offset":19904,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":24812,"parent_record_offset":24788,"ancestor_record_offset":19904,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":20268,"parent_record_offset":20144,"ancestor_record_offset":19904,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":24860,"parent_record_offset":24836,"ancestor_record_offset":19904,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":20784,"parent_record_offset":20756,"ancestor_record_offset":19904,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":20756,"parent_record_offset":20728,"ancestor_record_offset":19904,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":122,"record_offset":20600,"parent_record_offset":20436,"ancestor_record_offset":19904,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":122,"record_offset":20512,"parent_record_offset":20480,"ancestor_record_offset":19904,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":122,"record_offset":20536,"parent_record_offset":20512,"ancestor_record_offset":19904,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":21520,"parent_record_offset":21488,"ancestor_record_offset":19904,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":122,"record_offset":20628,"parent_record_offset":20600,"ancestor_record_offset":19904,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":20292,"parent_record_offset":20268,"ancestor_record_offset":19904,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":24380,"parent_record_offset":24348,"ancestor_record_offset":19904,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":122,"record_offset":20884,"parent_record_offset":20860,"ancestor_record_offset":19904,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=83
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":20668,"parent_record_offset":20436,"ancestor_record_offset":19904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":20568,"parent_record_offset":20480,"ancestor_record_offset":19904,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":20860,"parent_record_offset":20836,"ancestor_record_offset":19904,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23260,"parent_record_offset":23192,"ancestor_record_offset":19904,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":19904,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23288,"parent_record_offset":23192,"ancestor_record_offset":19904,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23232,"parent_record_offset":23192,"ancestor_record_offset":19904,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":23312,"parent_record_offset":23288,"ancestor_record_offset":19904,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":122,"record_offset":24884,"parent_record_offset":24860,"ancestor_record_offset":19904,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":122,"record_offset":24912,"parent_record_offset":24884,"ancestor_record_offset":19904,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":122,"record_offset":24836,"parent_record_offset":24812,"ancestor_record_offset":19904,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":20208,"parent_record_offset":20184,"ancestor_record_offset":19904,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":20232,"parent_record_offset":20208,"ancestor_record_offset":19904,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":20372,"parent_record_offset":20348,"ancestor_record_offset":19904,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":20324,"parent_record_offset":20144,"ancestor_record_offset":19904,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":19904,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":19904,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":20184,"parent_record_offset":20144,"ancestor_record_offset":19904,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::find_monitor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":20436,"parent_record_offset":20144,"ancestor_record_offset":19904,"inlinee":13033,"source_line":170,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::impl$0::setup_window::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":20144,"parent_record_offset":20108,"ancestor_record_offset":19904,"inlinee":13032,"source_line":53,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::impl$0::setup_window::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":22768,"parent_record_offset":22736,"ancestor_record_offset":19904,"inlinee":13034,"source_line":57,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::impl$0::setup_window::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":25000,"parent_record_offset":24960,"ancestor_record_offset":19904,"inlinee":13038,"source_line":90,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::mode_bit_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":26264,"parent_record_offset":26232,"ancestor_record_offset":19904,"inlinee":5410,"source_line":275,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::primary_monitor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":23052,"parent_record_offset":22768,"ancestor_record_offset":19904,"inlinee":13035,"source_line":176,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::video_mode_from_sdl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":26232,"parent_record_offset":26120,"ancestor_record_offset":19904,"inlinee":13040,"source_line":267,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::impl$0::setup_window::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32E8
// 首条位置: {"module":122,"inlinee":13032}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":20268,"parent_record_offset":20144,"ancestor_record_offset":20144,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":20324,"parent_record_offset":20144,"ancestor_record_offset":20144,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":20184,"parent_record_offset":20144,"ancestor_record_offset":20144,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_setup::find_monitor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":20436,"parent_record_offset":20144,"ancestor_record_offset":20144,"inlinee":13033,"source_line":170,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21228,"parent_record_offset":21204,"ancestor_record_offset":20144,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21252,"parent_record_offset":21228,"ancestor_record_offset":20144,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21276,"parent_record_offset":21252,"ancestor_record_offset":20144,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21204,"parent_record_offset":21148,"ancestor_record_offset":20144,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":21176,"parent_record_offset":21148,"ancestor_record_offset":20144,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":21148,"parent_record_offset":21124,"ancestor_record_offset":20144,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":21124,"parent_record_offset":21100,"ancestor_record_offset":20144,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":20480,"parent_record_offset":20436,"ancestor_record_offset":20144,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":20728,"parent_record_offset":20696,"ancestor_record_offset":20144,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":20696,"parent_record_offset":20668,"ancestor_record_offset":20144,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":21376,"parent_record_offset":21352,"ancestor_record_offset":20144,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":20812,"parent_record_offset":20784,"ancestor_record_offset":20144,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":20784,"parent_record_offset":20756,"ancestor_record_offset":20144,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":20756,"parent_record_offset":20728,"ancestor_record_offset":20144,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":122,"record_offset":20600,"parent_record_offset":20436,"ancestor_record_offset":20144,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":122,"record_offset":20512,"parent_record_offset":20480,"ancestor_record_offset":20144,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":122,"record_offset":20536,"parent_record_offset":20512,"ancestor_record_offset":20144,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":21520,"parent_record_offset":21488,"ancestor_record_offset":20144,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":122,"record_offset":20628,"parent_record_offset":20600,"ancestor_record_offset":20144,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":20292,"parent_record_offset":20268,"ancestor_record_offset":20144,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":122,"record_offset":20884,"parent_record_offset":20860,"ancestor_record_offset":20144,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":20668,"parent_record_offset":20436,"ancestor_record_offset":20144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":20568,"parent_record_offset":20480,"ancestor_record_offset":20144,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":20860,"parent_record_offset":20836,"ancestor_record_offset":20144,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":20144,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":20208,"parent_record_offset":20184,"ancestor_record_offset":20144,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":20232,"parent_record_offset":20208,"ancestor_record_offset":20144,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":20372,"parent_record_offset":20348,"ancestor_record_offset":20144,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":20144,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":20144,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::impl$0::setup_window::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32EA
// 首条位置: {"module":122,"inlinee":13034}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":22884,"parent_record_offset":22768,"ancestor_record_offset":22768,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":22940,"parent_record_offset":22768,"ancestor_record_offset":22768,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":22800,"parent_record_offset":22768,"ancestor_record_offset":22768,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_setup::primary_monitor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":23052,"parent_record_offset":22768,"ancestor_record_offset":22768,"inlinee":13035,"source_line":176,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":23692,"parent_record_offset":23668,"ancestor_record_offset":22768,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":23716,"parent_record_offset":23692,"ancestor_record_offset":22768,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":23740,"parent_record_offset":23716,"ancestor_record_offset":22768,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":23668,"parent_record_offset":23612,"ancestor_record_offset":22768,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":23640,"parent_record_offset":23612,"ancestor_record_offset":22768,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":23612,"parent_record_offset":23588,"ancestor_record_offset":22768,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":23588,"parent_record_offset":23564,"ancestor_record_offset":22768,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22768,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":23816,"parent_record_offset":23372,"ancestor_record_offset":22768,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":23372,"parent_record_offset":23348,"ancestor_record_offset":22768,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":22768,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":23192,"parent_record_offset":23052,"ancestor_record_offset":22768,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":23888,"parent_record_offset":23864,"ancestor_record_offset":22768,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":23864,"parent_record_offset":23840,"ancestor_record_offset":22768,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":23840,"parent_record_offset":23816,"ancestor_record_offset":22768,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":122,"record_offset":23104,"parent_record_offset":23076,"ancestor_record_offset":22768,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":122,"record_offset":23128,"parent_record_offset":23104,"ancestor_record_offset":22768,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":24164,"parent_record_offset":24136,"ancestor_record_offset":22768,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":22908,"parent_record_offset":22884,"ancestor_record_offset":22768,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":122,"record_offset":23960,"parent_record_offset":23936,"ancestor_record_offset":22768,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":23348,"parent_record_offset":23052,"ancestor_record_offset":22768,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":23160,"parent_record_offset":23076,"ancestor_record_offset":22768,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":23936,"parent_record_offset":23912,"ancestor_record_offset":22768,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23260,"parent_record_offset":23192,"ancestor_record_offset":22768,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23912,"parent_record_offset":23888,"ancestor_record_offset":22768,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23288,"parent_record_offset":23192,"ancestor_record_offset":22768,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23232,"parent_record_offset":23192,"ancestor_record_offset":22768,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":23312,"parent_record_offset":23288,"ancestor_record_offset":22768,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":22824,"parent_record_offset":22800,"ancestor_record_offset":22768,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":22848,"parent_record_offset":22824,"ancestor_record_offset":22768,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":22988,"parent_record_offset":22964,"ancestor_record_offset":22768,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":22964,"parent_record_offset":22940,"ancestor_record_offset":22768,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":22768,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::impl$0::setup_window::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32EE
// 首条位置: {"module":122,"inlinee":13038}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":25028,"parent_record_offset":25000,"ancestor_record_offset":25000,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":25204,"parent_record_offset":25180,"ancestor_record_offset":25000,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":25228,"parent_record_offset":25204,"ancestor_record_offset":25000,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":25252,"parent_record_offset":25228,"ancestor_record_offset":25000,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":25180,"parent_record_offset":25124,"ancestor_record_offset":25000,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":25152,"parent_record_offset":25124,"ancestor_record_offset":25000,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":25124,"parent_record_offset":25100,"ancestor_record_offset":25000,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":25100,"parent_record_offset":25076,"ancestor_record_offset":25000,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":25052,"parent_record_offset":25028,"ancestor_record_offset":25000,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::apply_fullscreen
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32EF
// 首条位置: {"module":122,"inlinee":13039}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":26440,"parent_record_offset":26120,"ancestor_record_offset":26120,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":26636,"parent_record_offset":26120,"ancestor_record_offset":26120,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":27060,"parent_record_offset":26120,"ancestor_record_offset":26120,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":26356,"parent_record_offset":26120,"ancestor_record_offset":26120,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_setup::video_mode_from_sdl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":26232,"parent_record_offset":26120,"ancestor_record_offset":26120,"inlinee":13040,"source_line":267,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":26812,"parent_record_offset":26788,"ancestor_record_offset":26120,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":26836,"parent_record_offset":26812,"ancestor_record_offset":26120,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":26860,"parent_record_offset":26836,"ancestor_record_offset":26120,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":26788,"parent_record_offset":26732,"ancestor_record_offset":26120,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":26760,"parent_record_offset":26732,"ancestor_record_offset":26120,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":26732,"parent_record_offset":26708,"ancestor_record_offset":26120,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":26708,"parent_record_offset":26684,"ancestor_record_offset":26120,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":26296,"parent_record_offset":26232,"ancestor_record_offset":26120,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":26464,"parent_record_offset":26440,"ancestor_record_offset":26120,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":26660,"parent_record_offset":26636,"ancestor_record_offset":26120,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":26380,"parent_record_offset":26356,"ancestor_record_offset":26120,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":26404,"parent_record_offset":26380,"ancestor_record_offset":26120,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":27108,"parent_record_offset":27084,"ancestor_record_offset":26120,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":27084,"parent_record_offset":27060,"ancestor_record_offset":26120,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":27132,"parent_record_offset":27108,"ancestor_record_offset":26120,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::mode_bit_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":26264,"parent_record_offset":26232,"ancestor_record_offset":26120,"inlinee":5410,"source_line":275,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::enumerate_displays
// 记录形式: compiled_procedure; instances=1
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":29988,"rva":4167024,"va":5372876144}
// 编译实例 RVAs: 0x3f9570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2802294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876144,"raw_hex":"e876c22a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372876405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bbc40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3398389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876144,"raw_hex":"e80b25ccff","size":5,"soft_fail":false,"target_va":5369478208,"va":5372876592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43474221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876144,"raw_hex":"e82d5d9702","size":5,"soft_fail":false,"target_va":5416350608,"va":5372876382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_setup::monitor_match_from_sdl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876144,"raw_hex":"e821010000","size":5,"soft_fail":false,"target_va":5372876624,"va":5372876330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::num_video_displays
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2955589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876144,"raw_hex":"e845192d00","size":5,"soft_fail":false,"target_va":5375831808,"va":5372876214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":30128,"parent_record_offset":29988,"ancestor_record_offset":29988,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":30156,"parent_record_offset":29988,"ancestor_record_offset":29988,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":122,"record_offset":30728,"parent_record_offset":29988,"ancestor_record_offset":29988,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":30812,"parent_record_offset":29988,"ancestor_record_offset":29988,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":30600,"parent_record_offset":30576,"ancestor_record_offset":29988,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":30624,"parent_record_offset":30600,"ancestor_record_offset":29988,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":30648,"parent_record_offset":30624,"ancestor_record_offset":29988,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":30576,"parent_record_offset":30552,"ancestor_record_offset":29988,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":30268,"parent_record_offset":30236,"ancestor_record_offset":29988,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":30960,"parent_record_offset":30932,"ancestor_record_offset":29988,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":30552,"parent_record_offset":30528,"ancestor_record_offset":29988,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":30316,"parent_record_offset":30292,"ancestor_record_offset":29988,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":30292,"parent_record_offset":30268,"ancestor_record_offset":29988,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":30528,"parent_record_offset":30504,"ancestor_record_offset":29988,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":30236,"parent_record_offset":30188,"ancestor_record_offset":29988,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":30188,"parent_record_offset":30156,"ancestor_record_offset":29988,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":30776,"parent_record_offset":30752,"ancestor_record_offset":29988,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":122,"record_offset":30752,"parent_record_offset":30728,"ancestor_record_offset":29988,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":30384,"parent_record_offset":30188,"ancestor_record_offset":29988,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":30356,"parent_record_offset":30188,"ancestor_record_offset":29988,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::find_monitor
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32E9
// 首条位置: {"module":122,"inlinee":13033}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":20480,"parent_record_offset":20436,"ancestor_record_offset":20436,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":122,"record_offset":20600,"parent_record_offset":20436,"ancestor_record_offset":20436,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":20668,"parent_record_offset":20436,"ancestor_record_offset":20436,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21228,"parent_record_offset":21204,"ancestor_record_offset":20436,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21252,"parent_record_offset":21228,"ancestor_record_offset":20436,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21276,"parent_record_offset":21252,"ancestor_record_offset":20436,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":21204,"parent_record_offset":21148,"ancestor_record_offset":20436,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":21176,"parent_record_offset":21148,"ancestor_record_offset":20436,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":21148,"parent_record_offset":21124,"ancestor_record_offset":20436,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":21124,"parent_record_offset":21100,"ancestor_record_offset":20436,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":20728,"parent_record_offset":20696,"ancestor_record_offset":20436,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":20696,"parent_record_offset":20668,"ancestor_record_offset":20436,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":21376,"parent_record_offset":21352,"ancestor_record_offset":20436,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":20812,"parent_record_offset":20784,"ancestor_record_offset":20436,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":20784,"parent_record_offset":20756,"ancestor_record_offset":20436,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":20756,"parent_record_offset":20728,"ancestor_record_offset":20436,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":122,"record_offset":20512,"parent_record_offset":20480,"ancestor_record_offset":20436,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":122,"record_offset":20536,"parent_record_offset":20512,"ancestor_record_offset":20436,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":21520,"parent_record_offset":21488,"ancestor_record_offset":20436,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":122,"record_offset":20628,"parent_record_offset":20600,"ancestor_record_offset":20436,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":122,"record_offset":20884,"parent_record_offset":20860,"ancestor_record_offset":20436,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":20936,"parent_record_offset":20696,"ancestor_record_offset":20436,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":20568,"parent_record_offset":20480,"ancestor_record_offset":20436,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":20860,"parent_record_offset":20836,"ancestor_record_offset":20436,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":20436,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::find_monitor::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156E
// 首条位置: {"module":8,"inlinee":5486}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::render_config::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":8,"record_offset":19168,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":5488,"source_line":36,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":8,"record_offset":19232,"parent_record_offset":19200,"ancestor_record_offset":19140,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":8,"record_offset":19256,"parent_record_offset":19232,"ancestor_record_offset":19140,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":8,"record_offset":19200,"parent_record_offset":19168,"ancestor_record_offset":19140,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":8,"record_offset":19280,"parent_record_offset":19256,"ancestor_record_offset":19140,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":8,"record_offset":19304,"parent_record_offset":19280,"ancestor_record_offset":19140,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::primary_monitor
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32EB
// 首条位置: {"module":122,"inlinee":13035}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":23052,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":23192,"parent_record_offset":23052,"ancestor_record_offset":23052,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":23348,"parent_record_offset":23052,"ancestor_record_offset":23052,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":23692,"parent_record_offset":23668,"ancestor_record_offset":23052,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":23716,"parent_record_offset":23692,"ancestor_record_offset":23052,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":23740,"parent_record_offset":23716,"ancestor_record_offset":23052,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":23668,"parent_record_offset":23612,"ancestor_record_offset":23052,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":23640,"parent_record_offset":23612,"ancestor_record_offset":23052,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":23612,"parent_record_offset":23588,"ancestor_record_offset":23052,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":23588,"parent_record_offset":23564,"ancestor_record_offset":23052,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":23816,"parent_record_offset":23372,"ancestor_record_offset":23052,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":23372,"parent_record_offset":23348,"ancestor_record_offset":23052,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":23052,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":122,"record_offset":23888,"parent_record_offset":23864,"ancestor_record_offset":23052,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":23864,"parent_record_offset":23840,"ancestor_record_offset":23052,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":122,"record_offset":23840,"parent_record_offset":23816,"ancestor_record_offset":23052,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":122,"record_offset":23104,"parent_record_offset":23076,"ancestor_record_offset":23052,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":122,"record_offset":23128,"parent_record_offset":23104,"ancestor_record_offset":23052,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":24164,"parent_record_offset":24136,"ancestor_record_offset":23052,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":122,"record_offset":23960,"parent_record_offset":23936,"ancestor_record_offset":23052,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":23400,"parent_record_offset":23372,"ancestor_record_offset":23052,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":23160,"parent_record_offset":23076,"ancestor_record_offset":23052,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":122,"record_offset":23936,"parent_record_offset":23912,"ancestor_record_offset":23052,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23260,"parent_record_offset":23192,"ancestor_record_offset":23052,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23912,"parent_record_offset":23888,"ancestor_record_offset":23052,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23288,"parent_record_offset":23192,"ancestor_record_offset":23052,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":23232,"parent_record_offset":23192,"ancestor_record_offset":23052,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":23312,"parent_record_offset":23288,"ancestor_record_offset":23052,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32F3
// 首条位置: {"module":122,"inlinee":13043}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":122,"record_offset":32812,"parent_record_offset":32780,"ancestor_record_offset":32780,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":33292,"parent_record_offset":32780,"ancestor_record_offset":32780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33112,"parent_record_offset":33088,"ancestor_record_offset":32780,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33136,"parent_record_offset":33112,"ancestor_record_offset":32780,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33468,"parent_record_offset":33444,"ancestor_record_offset":32780,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33492,"parent_record_offset":33468,"ancestor_record_offset":32780,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33160,"parent_record_offset":33136,"ancestor_record_offset":32780,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33520,"parent_record_offset":33492,"ancestor_record_offset":32780,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33088,"parent_record_offset":33060,"ancestor_record_offset":32780,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":33444,"parent_record_offset":33388,"ancestor_record_offset":32780,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33000,"parent_record_offset":32972,"ancestor_record_offset":32780,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33416,"parent_record_offset":33388,"ancestor_record_offset":32780,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33388,"parent_record_offset":33364,"ancestor_record_offset":32780,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33060,"parent_record_offset":33028,"ancestor_record_offset":32780,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33028,"parent_record_offset":33000,"ancestor_record_offset":32780,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":33364,"parent_record_offset":33340,"ancestor_record_offset":32780,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":32880,"parent_record_offset":32840,"ancestor_record_offset":32780,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":32908,"parent_record_offset":32880,"ancestor_record_offset":32780,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":32936,"parent_record_offset":32908,"ancestor_record_offset":32780,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":122,"record_offset":32840,"parent_record_offset":32812,"ancestor_record_offset":32780,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":32972,"parent_record_offset":32936,"ancestor_record_offset":32780,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":122,"record_offset":33216,"parent_record_offset":32936,"ancestor_record_offset":32780,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":33240,"parent_record_offset":33216,"ancestor_record_offset":32780,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":33316,"parent_record_offset":33292,"ancestor_record_offset":32780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=66
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x24EF
// 首条位置: {"module":55,"inlinee":9455}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::sdl_setup::mode_bit_depth
//     owner=private_source_ownership_candidate; count=62
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":55,"record_offset":4812,"parent_record_offset":4780,"ancestor_record_offset":4780,"inlinee":5410,"source_line":275,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B16
// 首条位置: {"module":81,"inlinee":11030}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":81,"record_offset":218152,"parent_record_offset":218128,"ancestor_record_offset":218128,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":81,"record_offset":218200,"parent_record_offset":218176,"ancestor_record_offset":218128,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":81,"record_offset":218176,"parent_record_offset":218152,"ancestor_record_offset":218128,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1520
// 首条位置: {"module":8,"inlinee":5408}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$3::abs
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":8,"record_offset":6396,"parent_record_offset":6328,"ancestor_record_offset":6328,"inlinee":5412,"source_line":3604,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_setup::mode_bit_depth
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":8,"record_offset":6368,"parent_record_offset":6328,"ancestor_record_offset":6328,"inlinee":5410,"source_line":275,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::monitor_match_from_sdl
// 记录形式: compiled_procedure; instances=1
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":31128,"rva":4167504,"va":5372876624}
// 编译实例 RVAs: 0x3f9750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2801950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876624,"raw_hex":"e81ec12a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372876749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ee920
//     owner=unresolved_namespace; count=1
//     target_aliases=248 candidates; complete list: sdl_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3190540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876624,"raw_hex":"e8f450cfff","size":5,"soft_fail":false,"target_va":5369686304,"va":5372876839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::rect::Rect::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2952559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876624,"raw_hex":"e86f0d2d00","size":5,"soft_fail":false,"target_va":5375829440,"va":5372876876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::rect::Rect::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2952647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876624,"raw_hex":"e8c70d2d00","size":5,"soft_fail":false,"target_va":5375829376,"va":5372876724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::rect::Rect::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2952556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876624,"raw_hex":"e86c0d2d00","size":5,"soft_fail":false,"target_va":5375829424,"va":5372876863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::rect::Rect::y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2952504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876624,"raw_hex":"e8380d2d00","size":5,"soft_fail":false,"target_va":5375829360,"va":5372876851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::display_bounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2954981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876624,"raw_hex":"e8e5162d00","size":5,"soft_fail":false,"target_va":5375831648,"va":5372876662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::display_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2954655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372876624,"raw_hex":"e89f152d00","size":5,"soft_fail":false,"target_va":5375831424,"va":5372876764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":32236,"parent_record_offset":31128,"ancestor_record_offset":31128,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":31888,"parent_record_offset":31128,"ancestor_record_offset":31128,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":31268,"parent_record_offset":31128,"ancestor_record_offset":31128,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":31128,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":31512,"parent_record_offset":31488,"ancestor_record_offset":31128,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":31536,"parent_record_offset":31512,"ancestor_record_offset":31128,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":31464,"parent_record_offset":31412,"ancestor_record_offset":31128,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":31436,"parent_record_offset":31412,"ancestor_record_offset":31128,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":31412,"parent_record_offset":31388,"ancestor_record_offset":31128,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":31388,"parent_record_offset":31368,"ancestor_record_offset":31128,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":31328,"parent_record_offset":31300,"ancestor_record_offset":31128,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::monitor_match_from_sdl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":31300,"parent_record_offset":31268,"ancestor_record_offset":31128,"inlinee":13042,"source_line":258,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::monitor_match_from_sdl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32F2
// 首条位置: {"module":122,"inlinee":13042}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":31328,"parent_record_offset":31300,"ancestor_record_offset":31300,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":31300,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":31512,"parent_record_offset":31488,"ancestor_record_offset":31300,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":31536,"parent_record_offset":31512,"ancestor_record_offset":31300,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":31464,"parent_record_offset":31412,"ancestor_record_offset":31300,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":31436,"parent_record_offset":31412,"ancestor_record_offset":31300,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":31412,"parent_record_offset":31388,"ancestor_record_offset":31300,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":31388,"parent_record_offset":31368,"ancestor_record_offset":31300,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":31348,"parent_record_offset":31328,"ancestor_record_offset":31300,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::video_mode_from_sdl
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32F0
// 首条位置: {"module":122,"inlinee":13040}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":26296,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_setup::mode_bit_depth
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":122,"record_offset":26264,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":5410,"source_line":275,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_setup::mode_bit_depth
// 记录形式: inline_instance, inlinee_declaration; instances=78
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1522
// 首条位置: {"module":8,"inlinee":5410}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: sdl_setup.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::sdl_setup::SdlMonitorInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::sdl_setup::SdlMonitorInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tiny_glade::sdl_setup::SdlMonitorInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_setup::find_monitor::closure_env$0>", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["tiny_glade::sdl_setup::SdlMonitorInfo", "tiny_glade::sdl_setup::find_monitor::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["tiny_glade::sdl_setup::SdlMonitorInfo", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::try_fold<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_setup::find_monitor::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> > > >", "constructor": "alloc::vec::into_iter::impl$6::try_fold", "arguments": ["tiny_glade::sdl_setup::SdlMonitorInfo", "alloc::alloc::Global", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_setup::find_monitor::closure_env$0>", "enum2$<core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure_env$0>", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["tiny_glade::sdl_setup::SdlMonitorInfo", "tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::try_fold<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> > > >", "constructor": "alloc::vec::into_iter::impl$6::try_fold", "arguments": ["tiny_glade::sdl_setup::SdlMonitorInfo", "alloc::alloc::Global", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure_env$0>", "enum2$<core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::sdl_setup::SdlWindowBackend
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::sdl_setup::SdlWindowBackend>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::sdl_setup::SdlWindowBackend"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sdl_setup.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sdl_setup.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 20; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >", "ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// 候选参数类型: ["ref$<tuple$<i64,usdl::video::DisplayMode> >", "ref$<tuple$<i64,usdl::video::DisplayMode> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >)
// 候选参数类型: ["ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >", "ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::SdlWindowBackend::setup_window
// 信息 ID: 267d6612d26146a1b6ee577d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42ca6d -> 0x3f85d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::apply_fullscreen
// 信息 ID: 0283a24c3b97eece68c2b0c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_setup::SdlWindowBackend::setup_window; module=122, inline_record=26120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::enumerate_displays
// 信息 ID: 567de355060d1f36787d34e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8700 -> 0x3f9570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8932 -> 0x3f9570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_event_loop::extract_available_monitors_info @ 0x51719b -> 0x3f9570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::find_best_fullscreen_video_mode
// 信息 ID: dbb3a0919aabede0ed448842
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8d03 -> 0x3f9920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$0
// 信息 ID: 8f536904d2be234a9736e6c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::sdl_setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_setup::find_best_fullscreen_video_mode; module=122, inline_record=32780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$1
// 信息 ID: 6851436b9d9df50313f0c404
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::sdl_setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1 <- core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1 <- core::slice::sort::shared::smallsort::bidirectional_merge<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1 <- core::slice::sort::stable::merge::merge<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1 <- core::slice::sort::stable::drift::sort<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 62 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=4780; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=4848; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=5224; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=5280; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=30060; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=30128; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=30412; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=30480; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$2
// 信息 ID: 57af883802e869eaa5597470
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a4cc73cfe63fb403c18f57c3: core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::sdl_setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2 <- alloc::vec::into_iter::impl$6::fold<usdl::video::DisplayMode,alloc::alloc::Global,tuple$<i64,usdl::video::DisplayMode>,core::iter::adapters::map::map_fold::closure_env$0<usdl::video::DisplayMode,tuple$<i64,usdl::video::DisplayMode>,tuple$<i64,usdl::video::DisplayMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<usdl::video::DisplayMode,i64,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,usdl::video::DisplayMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<usdl::video::DisplayMode,alloc::alloc::Global,tuple$<i64,usdl::video::DisplayMode>,core::iter::adapters::map::map_fold::closure_env$0<usdl::video::DisplayMode,tuple$<i64,usdl::video::DisplayMode>,tuple$<i64,usdl::video::DisplayMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<usdl::video::DisplayMode,i64,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,usdl::video::DisplayMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)> > >; module=8, inline_record=6328; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::sdl_setup::find_best_fullscreen_video_mode; module=122, inline_record=40172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure$3
// 信息 ID: 9b4dcd7a6ba492223dd7d381
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::sdl_setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::map::Map<core::slice::iter::Iter<usdl::video::DisplayMode>,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::map::Map<core::slice::iter::Iter<usdl::video::DisplayMode>,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$3> >; module=81, inline_record=218128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::find_monitor
// 信息 ID: ccdeab8f4ece4282f8153f6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_setup::SdlWindowBackend::setup_window; module=122, inline_record=20436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::find_monitor::closure$0
// 信息 ID: 18250650e9d170ec13d47510
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::sdl_setup::find_monitor", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::sdl_setup::find_monitor::closure_env$0 <- alloc::vec::into_iter::impl$6::try_fold<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_setup::find_monitor::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::try_fold<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_setup::find_monitor::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> > > >; module=8, inline_record=19140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::impl$0::setup_window::closure$0
// 信息 ID: f3a76a3c1170b977fad86c61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::sdl_setup::impl$0::setup_window", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_setup::SdlWindowBackend::setup_window; module=122, inline_record=20144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::impl$0::setup_window::closure$1
// 信息 ID: 6d7051a32584bcbaf656111f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::sdl_setup::impl$0::setup_window", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_setup::SdlWindowBackend::setup_window; module=122, inline_record=22768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::impl$0::setup_window::closure$2
// 信息 ID: 5ebdfa790c64db530b847ff2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::sdl_setup::impl$0::setup_window", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_setup::SdlWindowBackend::setup_window; module=122, inline_record=25000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::init_sdl
// 信息 ID: e01d6a3fd38b67a406119f62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42ca50 -> 0x3fa8b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::init_sdl::closure$0
// 信息 ID: 317459c52e6a0a52e2fbd80b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::sdl_setup::init_sdl", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::init_sdl @ 0x3fa978 -> 0x2d6f130 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::init_sdl @ 0x3fa92c -> 0x2d6f130 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::init_sdl @ 0x3fa9e9 -> 0x2d6f130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::mode_bit_depth
// 信息 ID: 8187fffff656bb232343b041
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a4cc73cfe63fb403c18f57c3: core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// 引用方: 原程序直接转移 0 条；内联宿主 71 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<usdl::video::DisplayMode,alloc::alloc::Global,tuple$<i64,usdl::video::DisplayMode>,core::iter::adapters::map::map_fold::closure_env$0<usdl::video::DisplayMode,tuple$<i64,usdl::video::DisplayMode>,tuple$<i64,usdl::video::DisplayMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<usdl::video::DisplayMode,i64,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,usdl::video::DisplayMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)> > >; module=8, inline_record=6368; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=4812; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=4876; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=5248; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=5312; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=30092; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=30156; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >; module=55, inline_record=30444; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::monitor_match_from_sdl
// 信息 ID: d9f8d1a3fb6641311e5db35d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::enumerate_displays @ 0x3f962a -> 0x3f9750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::monitor_match_from_sdl::closure$0
// 信息 ID: 1f5b40b55670144f5ca84853
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::sdl_setup::monitor_match_from_sdl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_setup::monitor_match_from_sdl; module=122, inline_record=31300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::primary_monitor
// 信息 ID: a4d7ac507851a86efcea1ef8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_setup::SdlWindowBackend::setup_window; module=122, inline_record=23052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_setup::video_mode_from_sdl
// 信息 ID: 1319f4fd68fe4b4b48b65473
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_setup::SdlWindowBackend::setup_window; module=122, inline_record=26232; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::sdl_setup::find_best_fullscreen_video_mode; module=122, inline_record=39208; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::sdl_setup::find_best_fullscreen_video_mode; module=122, inline_record=39576; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::sdl_setup::find_best_fullscreen_video_mode; module=122, inline_record=39632; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::run_event_loop; module=177, inline_record=12420; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::extract_available_monitors_info; module=177, inline_record=36188; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::extract_available_monitors_info; module=177, inline_record=37300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
