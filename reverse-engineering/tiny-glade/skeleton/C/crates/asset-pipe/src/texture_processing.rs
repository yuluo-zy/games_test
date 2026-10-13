// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset-pipe/src/texture_processing.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 66b06e5a34b6b8923cc321c9fb4625ed55e1e795adb62dea5d39f1bf430edb51
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: texture_processing.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1667, 2061, 3255, 3256, 3257, 3260, 3579, 4581, 4582, 4972
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.24.0"}]
// use core; // namespace candidate
//   package identities: []
// use ddsfile; // namespace candidate
//   package identities: [{"name":"ddsfile","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.1"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use intel_tex_2; // namespace candidate
//   package identities: [{"name":"intel_tex_2","source":"git+https://github.com/PounceLight/intel-tex-rs-2.git?rev=4fb0e0b70f868bccd529d45e1626de2f55a20b83#4fb0e0b70f868bccd529d45e1626de2f55a20b83","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::resize_image
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14227
// 首条位置: {"module":1667,"inlinee":82471}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::pad_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14229
// 首条位置: {"module":1667,"inlinee":82473}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture_processing::resize_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":11092,"parent_record_offset":10996,"ancestor_record_offset":10996,"inlinee":82471,"source_line":12,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":11400,"parent_record_offset":10996,"ancestor_record_offset":10996,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":11856,"parent_record_offset":10996,"ancestor_record_offset":10996,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] image::image::GenericImage::copy_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":1667,"record_offset":11120,"parent_record_offset":10996,"ancestor_record_offset":10996,"inlinee":28880,"source_line":963,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":11276,"parent_record_offset":11188,"ancestor_record_offset":10996,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":11220,"parent_record_offset":11188,"ancestor_record_offset":10996,"inlinee":5303,"source_line":211,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":11188,"parent_record_offset":11160,"ancestor_record_offset":10996,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":11160,"parent_record_offset":11120,"ancestor_record_offset":10996,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":11244,"parent_record_offset":11220,"ancestor_record_offset":10996,"inlinee":5304,"source_line":832,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_regular_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":4308,"rva":24980432,"va":5393689552}
// 编译实例 RVAs: 0x17d2bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-18013440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e80023edfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393692139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142594090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14423038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e8fe13dc00","size":5,"soft_fail":false,"target_va":5408112784,"va":5393689741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425943c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14420806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e8460bdc00","size":5,"soft_fail":false,"target_va":5408113600,"va":5393692789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142594b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14424449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e88119dc00","size":5,"soft_fail":false,"target_va":5408115504,"va":5393691050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142595b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14427810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e8a226dc00","size":5,"soft_fail":false,"target_va":5408119600,"va":5393691785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142595d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14428719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e82f2adc00","size":5,"soft_fail":false,"target_va":5408120160,"va":5393691436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23367869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e8bd906401","size":5,"soft_fail":false,"target_va":5417057936,"va":5393690062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture_processing::alpha_distribution_visibility
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e8ad180000","size":5,"soft_fail":false,"target_va":5393697776,"va":5393691454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture_processing::process_regular_image::closure$2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e8320f0000","size":5,"soft_fail":false,"target_va":5393694992,"va":5393691097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t22928245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e875db5d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5393693350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22928914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e812de5d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5393693289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t22929239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e857df5d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5393692996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t14113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e821370000","size":5,"soft_fail":false,"target_va":5393707008,"va":5393692890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t22929006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e86ede5d01","size":5,"soft_fail":false,"target_va":5416622320,"va":5393693309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba32f
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10668944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e870345dff","size":5,"soft_fail":false,"target_va":5383020976,"va":5393689915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10672211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e8ad275dff","size":5,"soft_fail":false,"target_va":5383019440,"va":5393691646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::resize_exact
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10669641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e8b7315dff","size":5,"soft_fail":false,"target_va":5383021088,"va":5393690724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10666749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e8033d5dff","size":5,"soft_fail":false,"target_va":5383023680,"va":5393690424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::load_from_memory
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10665727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393689552,"raw_hex":"e801415dff","size":5,"soft_fail":false,"target_va":5383024000,"va":5393689722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":5840,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1667,"record_offset":4508,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":38724,"source_line":82,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::apply_alpha_mip_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":6028,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":82472,"source_line":267,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::apply_transfer_function
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":4624,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":82464,"source_line":497,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::open_regular_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":4532,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":82463,"source_line":263,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::resize_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":5656,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":82471,"source_line":12,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":5784,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":5812,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1667,"record_offset":5488,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":5868,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_multiple_of
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":5600,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":82470,"source_line":3691,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":5756,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":9804,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":9868,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":4560,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":4596,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":4308,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":4308,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":4308,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":8572,"parent_record_offset":8516,"ancestor_record_offset":4308,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":8544,"parent_record_offset":8516,"ancestor_record_offset":4308,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":4308,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":4308,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":4308,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":4308,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":4308,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":7104,"parent_record_offset":6952,"ancestor_record_offset":4308,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::srgb_eotf
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":5284,"parent_record_offset":4624,"ancestor_record_offset":4308,"inlinee":82465,"source_line":517,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":5932,"parent_record_offset":5896,"ancestor_record_offset":4308,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1667,"record_offset":8716,"parent_record_offset":6028,"ancestor_record_offset":4308,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1667,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":4308,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1667,"record_offset":6912,"parent_record_offset":6028,"ancestor_record_offset":4308,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1667,"record_offset":6952,"parent_record_offset":6912,"ancestor_record_offset":4308,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":5964,"parent_record_offset":5896,"ancestor_record_offset":4308,"inlinee":5303,"source_line":211,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":5896,"parent_record_offset":5868,"ancestor_record_offset":4308,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":6856,"parent_record_offset":6028,"ancestor_record_offset":4308,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":6800,"parent_record_offset":6028,"ancestor_record_offset":4308,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":4824,"parent_record_offset":4800,"ancestor_record_offset":4308,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":5128,"parent_record_offset":5096,"ancestor_record_offset":4308,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":4308,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":9636,"parent_record_offset":9604,"ancestor_record_offset":4308,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":4308,"inlinee":5304,"source_line":832,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":4776,"parent_record_offset":4748,"ancestor_record_offset":4308,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":5004,"parent_record_offset":4708,"ancestor_record_offset":4308,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1667,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":4308,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":8348,"parent_record_offset":6028,"ancestor_record_offset":4308,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":8308,"parent_record_offset":8180,"ancestor_record_offset":4308,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":7140,"parent_record_offset":7104,"ancestor_record_offset":4308,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":7196,"parent_record_offset":7104,"ancestor_record_offset":4308,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":4308,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":6680,"parent_record_offset":6184,"ancestor_record_offset":4308,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_checked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":5240,"parent_record_offset":5208,"ancestor_record_offset":4308,"inlinee":8567,"source_line":2192,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":6732,"parent_record_offset":6704,"ancestor_record_offset":4308,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":6548,"parent_record_offset":6220,"ancestor_record_offset":4308,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":5040,"parent_record_offset":4708,"ancestor_record_offset":4308,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":6604,"parent_record_offset":6576,"ancestor_record_offset":4308,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":5096,"parent_record_offset":5068,"ancestor_record_offset":4308,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":6576,"parent_record_offset":6548,"ancestor_record_offset":4308,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":4308,"inlinee":11639,"source_line":533,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1667,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":4308,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1667,"record_offset":5208,"parent_record_offset":5176,"ancestor_record_offset":4308,"inlinee":9636,"source_line":2051,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4748,"parent_record_offset":4708,"ancestor_record_offset":4308,"inlinee":9623,"source_line":807,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":6220,"parent_record_offset":6184,"ancestor_record_offset":4308,"inlinee":9631,"source_line":713,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4708,"parent_record_offset":4672,"ancestor_record_offset":4308,"inlinee":42061,"source_line":892,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":6184,"parent_record_offset":6028,"ancestor_record_offset":4308,"inlinee":19456,"source_line":720,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4672,"parent_record_offset":4624,"ancestor_record_offset":4308,"inlinee":9627,"source_line":898,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4308,"inlinee":9625,"source_line":809,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":4308,"inlinee":9626,"source_line":810,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":5176,"parent_record_offset":4624,"ancestor_record_offset":4308,"inlinee":9635,"source_line":98,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":1667,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":4308,"inlinee":82467,"source_line":929,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::impl$4::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":1667,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":4308,"inlinee":82469,"source_line":893,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1667,"record_offset":5320,"parent_record_offset":5284,"ancestor_record_offset":4308,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_regular_image::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF25
// 首条位置: {"module":3256,"inlinee":118565}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8556,"parent_record_offset":8464,"ancestor_record_offset":8464,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::append_uncompressed
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":9700,"parent_record_offset":8464,"ancestor_record_offset":8464,"inlinee":82495,"source_line":469,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3256,"record_offset":8520,"parent_record_offset":8464,"ancestor_record_offset":8464,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":8464,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":9260,"parent_record_offset":9236,"ancestor_record_offset":8464,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":8464,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":9212,"parent_record_offset":9156,"ancestor_record_offset":8464,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":8464,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":8892,"parent_record_offset":8864,"ancestor_record_offset":8464,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":9184,"parent_record_offset":9156,"ancestor_record_offset":8464,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":8464,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":8864,"parent_record_offset":8828,"ancestor_record_offset":8464,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":8828,"parent_record_offset":8796,"ancestor_record_offset":8464,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":8464,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8720,"parent_record_offset":8676,"ancestor_record_offset":8464,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":10756,"parent_record_offset":9700,"ancestor_record_offset":8464,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":9900,"parent_record_offset":9868,"ancestor_record_offset":8464,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8980,"parent_record_offset":8720,"ancestor_record_offset":8464,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8768,"parent_record_offset":8720,"ancestor_record_offset":8464,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8592,"parent_record_offset":8556,"ancestor_record_offset":8464,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":9836,"parent_record_offset":9700,"ancestor_record_offset":8464,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":10460,"parent_record_offset":10432,"ancestor_record_offset":8464,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3256,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":8464,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3256,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":8464,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":10524,"parent_record_offset":10460,"ancestor_record_offset":8464,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":10716,"parent_record_offset":9900,"ancestor_record_offset":8464,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":10660,"parent_record_offset":10636,"ancestor_record_offset":8464,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":9868,"parent_record_offset":9836,"ancestor_record_offset":8464,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":8676,"parent_record_offset":8556,"ancestor_record_offset":8464,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":10784,"parent_record_offset":10756,"ancestor_record_offset":8464,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::append_uncompressed::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":10400,"parent_record_offset":10372,"ancestor_record_offset":8464,"inlinee":82461,"source_line":474,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":10212,"parent_record_offset":10180,"ancestor_record_offset":8464,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":9948,"parent_record_offset":9900,"ancestor_record_offset":8464,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":10372,"parent_record_offset":10244,"ancestor_record_offset":8464,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":10244,"parent_record_offset":10212,"ancestor_record_offset":8464,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":10180,"parent_record_offset":9900,"ancestor_record_offset":8464,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":10432,"parent_record_offset":10372,"ancestor_record_offset":8464,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":3256,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":8464,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":3256,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":8464,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":9012,"parent_record_offset":8676,"ancestor_record_offset":8464,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3256,"record_offset":8944,"parent_record_offset":8720,"ancestor_record_offset":8464,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":10492,"parent_record_offset":10460,"ancestor_record_offset":8464,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":9780,"parent_record_offset":9700,"ancestor_record_offset":8464,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":10328,"parent_record_offset":10292,"ancestor_record_offset":8464,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":9804,"parent_record_offset":9780,"ancestor_record_offset":8464,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":10292,"parent_record_offset":10244,"ancestor_record_offset":8464,"inlinee":11952,"source_line":1896,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":8464,"inlinee":6688,"source_line":1904,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":9752,"parent_record_offset":9700,"ancestor_record_offset":8464,"inlinee":9744,"source_line":1110,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_regular_image::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":10676,"rva":24985872,"va":5393694992}
// 编译实例 RVAs: 0x17d4110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$image..dynimage..DynamicImage$u20$as$u20$image..image..GenericImage$GT$::put_pixel
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-10671136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393694992,"raw_hex":"e8e02b5dff","size":5,"soft_fail":false,"target_va":5383024208,"va":5393695339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$image..dynimage..DynamicImage$u20$as$u20$image..image..GenericImageView$GT$::get_pixel
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-10668638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393694992,"raw_hex":"e8a2355dff","size":5,"soft_fail":false,"target_va":5383026688,"va":5393695321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22926554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393694992,"raw_hex":"e8dad45d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5393695649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22926812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393694992,"raw_hex":"e8dcd55d01","size":5,"soft_fail":false,"target_va":5416622560,"va":5393695743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t11408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393694992,"raw_hex":"e8902c0000","size":5,"soft_fail":false,"target_va":5393707008,"va":5393695595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::error::ImageError> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393694992,"raw_hex":"e81b290000","size":5,"soft_fail":false,"target_va":5393706464,"va":5393695936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22925899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393694992,"raw_hex":"e84bd25d01","size":5,"soft_fail":false,"target_va":5416621632,"va":5393695728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::new_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10671404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393694992,"raw_hex":"e8d42a5dff","size":5,"soft_fail":false,"target_va":5383023824,"va":5393695223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::resize_exact
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10674011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393694992,"raw_hex":"e8a5205dff","size":5,"soft_fail":false,"target_va":5383021088,"va":5393695094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture_processing::pad_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":10996,"parent_record_offset":10676,"ancestor_record_offset":10676,"inlinee":82473,"source_line":71,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::resize_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":10892,"parent_record_offset":10676,"ancestor_record_offset":10676,"inlinee":82471,"source_line":12,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":10836,"parent_record_offset":10676,"ancestor_record_offset":10676,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_multiple_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":10920,"parent_record_offset":10676,"ancestor_record_offset":10676,"inlinee":82470,"source_line":3691,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::resize_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":11092,"parent_record_offset":10996,"ancestor_record_offset":10676,"inlinee":82471,"source_line":12,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":11276,"parent_record_offset":11188,"ancestor_record_offset":10676,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":11220,"parent_record_offset":11188,"ancestor_record_offset":10676,"inlinee":5303,"source_line":211,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":11188,"parent_record_offset":11160,"ancestor_record_offset":10676,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":11160,"parent_record_offset":11120,"ancestor_record_offset":10676,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":11244,"parent_record_offset":11220,"ancestor_record_offset":10676,"inlinee":5304,"source_line":832,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":11856,"parent_record_offset":10996,"ancestor_record_offset":10676,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::image::GenericImage::copy_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":1667,"record_offset":11120,"parent_record_offset":10996,"ancestor_record_offset":10676,"inlinee":28880,"source_line":963,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_regular_image::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF23
// 首条位置: {"module":3256,"inlinee":118563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1296,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::append_uncompressed
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":2440,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":82495,"source_line":469,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3256,"record_offset":1260,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1204,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1204,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1204,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":1952,"parent_record_offset":1896,"ancestor_record_offset":1204,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1536,"parent_record_offset":1508,"ancestor_record_offset":1204,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1632,"parent_record_offset":1604,"ancestor_record_offset":1204,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":1204,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":1204,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1604,"parent_record_offset":1568,"ancestor_record_offset":1204,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1568,"parent_record_offset":1536,"ancestor_record_offset":1204,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":1204,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1460,"parent_record_offset":1416,"ancestor_record_offset":1204,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":3496,"parent_record_offset":2440,"ancestor_record_offset":1204,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":2640,"parent_record_offset":2608,"ancestor_record_offset":1204,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1720,"parent_record_offset":1460,"ancestor_record_offset":1204,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1508,"parent_record_offset":1460,"ancestor_record_offset":1204,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1332,"parent_record_offset":1296,"ancestor_record_offset":1204,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":2576,"parent_record_offset":2440,"ancestor_record_offset":1204,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":3200,"parent_record_offset":3172,"ancestor_record_offset":1204,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3256,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":1204,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3256,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":1204,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":3264,"parent_record_offset":3200,"ancestor_record_offset":1204,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":3456,"parent_record_offset":2640,"ancestor_record_offset":1204,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":1204,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":2608,"parent_record_offset":2576,"ancestor_record_offset":1204,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":1416,"parent_record_offset":1296,"ancestor_record_offset":1204,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":1204,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::append_uncompressed::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":3140,"parent_record_offset":3112,"ancestor_record_offset":1204,"inlinee":82461,"source_line":474,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":2952,"parent_record_offset":2920,"ancestor_record_offset":1204,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":2688,"parent_record_offset":2640,"ancestor_record_offset":1204,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":3112,"parent_record_offset":2984,"ancestor_record_offset":1204,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":1204,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":2920,"parent_record_offset":2640,"ancestor_record_offset":1204,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":3172,"parent_record_offset":3112,"ancestor_record_offset":1204,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":3256,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":1204,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":3256,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":1204,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":1752,"parent_record_offset":1416,"ancestor_record_offset":1204,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3256,"record_offset":1684,"parent_record_offset":1460,"ancestor_record_offset":1204,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":3232,"parent_record_offset":3200,"ancestor_record_offset":1204,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":2520,"parent_record_offset":2440,"ancestor_record_offset":1204,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":3068,"parent_record_offset":3032,"ancestor_record_offset":1204,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":1204,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":3032,"parent_record_offset":2984,"ancestor_record_offset":1204,"inlinee":11952,"source_line":1896,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":1204,"inlinee":6688,"source_line":1904,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":2492,"parent_record_offset":2440,"ancestor_record_offset":1204,"inlinee":9744,"source_line":1110,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::open_regular_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1421F
// 首条位置: {"module":1667,"inlinee":82463}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::open_regular_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF1F
// 首条位置: {"module":3255,"inlinee":118559}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3255,"record_offset":3364,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3255,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":3344,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03255.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3255,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":3344,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03255.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::apply_alpha_mip_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14228
// 首条位置: {"module":1667,"inlinee":82472}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1667,"record_offset":8716,"parent_record_offset":6028,"ancestor_record_offset":6028,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1667,"record_offset":6912,"parent_record_offset":6028,"ancestor_record_offset":6028,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":6856,"parent_record_offset":6028,"ancestor_record_offset":6028,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":6800,"parent_record_offset":6028,"ancestor_record_offset":6028,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":8348,"parent_record_offset":6028,"ancestor_record_offset":6028,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::pixels
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":6184,"parent_record_offset":6028,"ancestor_record_offset":6028,"inlinee":19456,"source_line":720,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":6028,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":6028,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":6028,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":8572,"parent_record_offset":8516,"ancestor_record_offset":6028,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":8544,"parent_record_offset":8516,"ancestor_record_offset":6028,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":6028,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":6028,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":6028,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":6028,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":6028,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":7104,"parent_record_offset":6952,"ancestor_record_offset":6028,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1667,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":6028,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1667,"record_offset":6952,"parent_record_offset":6912,"ancestor_record_offset":6028,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":6332,"parent_record_offset":6308,"ancestor_record_offset":6028,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":6636,"parent_record_offset":6604,"ancestor_record_offset":6028,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":6028,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":9636,"parent_record_offset":9604,"ancestor_record_offset":6028,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":6284,"parent_record_offset":6256,"ancestor_record_offset":6028,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":6512,"parent_record_offset":6220,"ancestor_record_offset":6028,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1667,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":6028,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":6028,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":8308,"parent_record_offset":8180,"ancestor_record_offset":6028,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":7140,"parent_record_offset":7104,"ancestor_record_offset":6028,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":7196,"parent_record_offset":7104,"ancestor_record_offset":6028,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":6028,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":6680,"parent_record_offset":6184,"ancestor_record_offset":6028,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":7060,"parent_record_offset":7028,"ancestor_record_offset":6028,"inlinee":8567,"source_line":2192,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":6732,"parent_record_offset":6704,"ancestor_record_offset":6028,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":6548,"parent_record_offset":6220,"ancestor_record_offset":6028,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":6604,"parent_record_offset":6576,"ancestor_record_offset":6028,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":6576,"parent_record_offset":6548,"ancestor_record_offset":6028,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1667,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":6028,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1667,"record_offset":7028,"parent_record_offset":6996,"ancestor_record_offset":6028,"inlinee":9636,"source_line":2051,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":6256,"parent_record_offset":6220,"ancestor_record_offset":6028,"inlinee":9623,"source_line":807,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":6220,"parent_record_offset":6184,"ancestor_record_offset":6028,"inlinee":9631,"source_line":713,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":6028,"inlinee":9625,"source_line":809,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":6028,"inlinee":9626,"source_line":810,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":6996,"parent_record_offset":6952,"ancestor_record_offset":6028,"inlinee":9635,"source_line":98,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":1667,"record_offset":8740,"parent_record_offset":8716,"ancestor_record_offset":6028,"inlinee":82467,"source_line":929,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::impl$4::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":1667,"record_offset":9164,"parent_record_offset":9140,"ancestor_record_offset":6028,"inlinee":82469,"source_line":893,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::apply_alpha_mip_mode::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF32
// 首条位置: {"module":3257,"inlinee":118578}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::alpha_distribution_visibility
// 记录形式: compiled_procedure; instances=1
// 源码行: 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":15464,"rva":24988656,"va":5393697776}
// 编译实例 RVAs: 0x17d4bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t22919373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e8cdb85d01","size":5,"soft_fail":false,"target_va":5416621000,"va":5393701622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18019168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e8a00cedfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393697851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18020019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e84d09edfe","size":5,"soft_fail":false,"target_va":5375678736,"va":5393698750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t-18022493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e8a3ffecfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393701192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=5
//     target_aliases=1699 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24975649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e8dfe682fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393697836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b66d0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24245704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e8380a8efe","size":5,"soft_fail":false,"target_va":5369456336,"va":5393702035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425952a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14416247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e877f9db00","size":5,"soft_fail":false,"target_va":5408117408,"va":5393701156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425954a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14419470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e80e06dc00","size":5,"soft_fail":false,"target_va":5408117920,"va":5393698445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425959b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14418842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e89a03dc00","size":5,"soft_fail":false,"target_va":5408119216,"va":5393700369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22651567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e8afa25901","size":5,"soft_fail":false,"target_va":5416351168,"va":5393699596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23355966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e83e626401","size":5,"soft_fail":false,"target_va":5417057072,"va":5393701101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23357598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e89e686401","size":5,"soft_fail":false,"target_va":5417057664,"va":5393700061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22920560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e870bd5d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5393700204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22869177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e8b9f45c01","size":5,"soft_fail":false,"target_va":5416568672,"va":5393699490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t22922591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e85fc55d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5393698198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture_processing::distribute_alpha_visibility
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e8a2edffff","size":5,"soft_fail":false,"target_va":5393696032,"va":5393700729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t22922037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e835c35d01","size":5,"soft_fail":false,"target_va":5416621762,"va":5393699720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset_pipe..texture_processing..AlphaDistributionPyramidLevel$GT$
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t6160765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697776,"raw_hex":"e87d015e00","size":5,"soft_fail":false,"target_va":5399862144,"va":5393701374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-18019266","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393697776,"raw_hex":"e93e0cedfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393697965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1667,"record_offset":17540,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1667,"record_offset":15716,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":15688,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":16524,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":21112,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":24408,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":16844,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":21340,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":23468,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":24576,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::build_alpha_distribution_pyramid_parent_level
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":17728,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":82480,"source_line":377,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1667,"record_offset":22160,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1667,"record_offset":23576,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":23832,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":15904,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":21436,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":17660,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":16552,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":17624,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1667,"record_offset":21720,"parent_record_offset":15464,"ancestor_record_offset":15464,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":15808,"parent_record_offset":15784,"ancestor_record_offset":15464,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":15832,"parent_record_offset":15808,"ancestor_record_offset":15464,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":16740,"parent_record_offset":16716,"ancestor_record_offset":15464,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":16764,"parent_record_offset":16740,"ancestor_record_offset":15464,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":15856,"parent_record_offset":15832,"ancestor_record_offset":15464,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":18252,"parent_record_offset":18228,"ancestor_record_offset":15464,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":16788,"parent_record_offset":16764,"ancestor_record_offset":15464,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":15784,"parent_record_offset":15748,"ancestor_record_offset":15464,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":18180,"parent_record_offset":18032,"ancestor_record_offset":15464,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":16716,"parent_record_offset":16624,"ancestor_record_offset":15464,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1667,"record_offset":15748,"parent_record_offset":15716,"ancestor_record_offset":15464,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1667,"record_offset":26556,"parent_record_offset":26532,"ancestor_record_offset":15464,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":15464,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":16920,"parent_record_offset":16896,"ancestor_record_offset":15464,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":17972,"parent_record_offset":17944,"ancestor_record_offset":15464,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":16652,"parent_record_offset":16624,"ancestor_record_offset":15464,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":16624,"parent_record_offset":16600,"ancestor_record_offset":15464,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":19320,"parent_record_offset":19296,"ancestor_record_offset":15464,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":15464,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":16976,"parent_record_offset":16944,"ancestor_record_offset":15464,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":16944,"parent_record_offset":16920,"ancestor_record_offset":15464,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":18000,"parent_record_offset":17972,"ancestor_record_offset":15464,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":16600,"parent_record_offset":16576,"ancestor_record_offset":15464,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":17016,"parent_record_offset":16976,"ancestor_record_offset":15464,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":17564,"parent_record_offset":17540,"ancestor_record_offset":15464,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19248,"parent_record_offset":19208,"ancestor_record_offset":15464,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":22940,"parent_record_offset":22908,"ancestor_record_offset":15464,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":17256,"parent_record_offset":16868,"ancestor_record_offset":15464,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":17588,"parent_record_offset":17564,"ancestor_record_offset":15464,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":18720,"parent_record_offset":17728,"ancestor_record_offset":15464,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19180,"parent_record_offset":17728,"ancestor_record_offset":15464,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19208,"parent_record_offset":19180,"ancestor_record_offset":15464,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":24540,"parent_record_offset":24440,"ancestor_record_offset":15464,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":24440,"parent_record_offset":24408,"ancestor_record_offset":15464,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":22468,"parent_record_offset":22428,"ancestor_record_offset":15464,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":16896,"parent_record_offset":16868,"ancestor_record_offset":15464,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":17916,"parent_record_offset":17728,"ancestor_record_offset":15464,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19480,"parent_record_offset":17728,"ancestor_record_offset":15464,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19576,"parent_record_offset":17728,"ancestor_record_offset":15464,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":15932,"parent_record_offset":15904,"ancestor_record_offset":15464,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":23228,"parent_record_offset":23200,"ancestor_record_offset":15464,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":20468,"parent_record_offset":20440,"ancestor_record_offset":15464,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":1667,"record_offset":15988,"parent_record_offset":15960,"ancestor_record_offset":15464,"inlinee":10798,"source_line":249,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":1667,"record_offset":15960,"parent_record_offset":15932,"ancestor_record_offset":15464,"inlinee":8793,"source_line":232,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$3::collect_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":1667,"record_offset":16016,"parent_record_offset":15988,"ancestor_record_offset":15464,"inlinee":10803,"source_line":389,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":16304,"parent_record_offset":16280,"ancestor_record_offset":15464,"inlinee":10805,"source_line":362,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1667,"record_offset":23292,"parent_record_offset":23228,"ancestor_record_offset":15464,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1667,"record_offset":22908,"parent_record_offset":22428,"ancestor_record_offset":15464,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1667,"record_offset":16868,"parent_record_offset":16844,"ancestor_record_offset":15464,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1667,"record_offset":22400,"parent_record_offset":22372,"ancestor_record_offset":15464,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1667,"record_offset":22428,"parent_record_offset":22400,"ancestor_record_offset":15464,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::alpha_distribution_visibility::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":17696,"parent_record_offset":17660,"ancestor_record_offset":15464,"inlinee":82479,"source_line":325,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::alpha_distribution_visibility::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":16396,"parent_record_offset":16228,"ancestor_record_offset":15464,"inlinee":82478,"source_line":318,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1667,"record_offset":17040,"parent_record_offset":17016,"ancestor_record_offset":15464,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":16192,"parent_record_offset":16096,"ancestor_record_offset":15464,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1667,"record_offset":16424,"parent_record_offset":16396,"ancestor_record_offset":15464,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1667,"record_offset":21744,"parent_record_offset":21720,"ancestor_record_offset":15464,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1667,"record_offset":17088,"parent_record_offset":17064,"ancestor_record_offset":15464,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1667,"record_offset":16228,"parent_record_offset":16016,"ancestor_record_offset":15464,"inlinee":10804,"source_line":132,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1667,"record_offset":16280,"parent_record_offset":16256,"ancestor_record_offset":15464,"inlinee":6629,"source_line":629,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1667,"record_offset":16256,"parent_record_offset":16228,"ancestor_record_offset":15464,"inlinee":6627,"source_line":606,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":23624,"parent_record_offset":23600,"ancestor_record_offset":15464,"inlinee":35898,"source_line":1209,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$16::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":23600,"parent_record_offset":23576,"ancestor_record_offset":15464,"inlinee":35897,"source_line":1355,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":16136,"parent_record_offset":16096,"ancestor_record_offset":15464,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":16096,"parent_record_offset":16060,"ancestor_record_offset":15464,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":16060,"parent_record_offset":16016,"ancestor_record_offset":15464,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$130::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1667,"record_offset":21464,"parent_record_offset":21436,"ancestor_record_offset":15464,"inlinee":32848,"source_line":182,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$130::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1667,"record_offset":21584,"parent_record_offset":21492,"ancestor_record_offset":15464,"inlinee":32761,"source_line":186,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$50::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1667,"record_offset":21900,"parent_record_offset":21872,"ancestor_record_offset":15464,"inlinee":82482,"source_line":71,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$50::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1667,"record_offset":22012,"parent_record_offset":21928,"ancestor_record_offset":15464,"inlinee":82484,"source_line":75,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1667,"record_offset":24680,"parent_record_offset":24656,"ancestor_record_offset":15464,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":23004,"parent_record_offset":22972,"ancestor_record_offset":15464,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":22972,"parent_record_offset":22940,"ancestor_record_offset":15464,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":23200,"parent_record_offset":23004,"ancestor_record_offset":15464,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":17064,"parent_record_offset":17040,"ancestor_record_offset":15464,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":17852,"parent_record_offset":17728,"ancestor_record_offset":15464,"inlinee":10418,"source_line":3657,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":22708,"parent_record_offset":22680,"ancestor_record_offset":15464,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":16160,"parent_record_offset":16136,"ancestor_record_offset":15464,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":16684,"parent_record_offset":16652,"ancestor_record_offset":15464,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1667,"record_offset":22076,"parent_record_offset":22044,"ancestor_record_offset":15464,"inlinee":35784,"source_line":105,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1667,"record_offset":21640,"parent_record_offset":21612,"ancestor_record_offset":15464,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$17::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1667,"record_offset":22044,"parent_record_offset":22012,"ancestor_record_offset":15464,"inlinee":82486,"source_line":38,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$56::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1667,"record_offset":21612,"parent_record_offset":21584,"ancestor_record_offset":15464,"inlinee":32763,"source_line":38,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1667,"record_offset":23656,"parent_record_offset":23624,"ancestor_record_offset":15464,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":16576,"parent_record_offset":16552,"ancestor_record_offset":15464,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1667,"record_offset":17444,"parent_record_offset":17256,"ancestor_record_offset":15464,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":16328,"parent_record_offset":16304,"ancestor_record_offset":15464,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":21556,"parent_record_offset":21492,"ancestor_record_offset":15464,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":16352,"parent_record_offset":16328,"ancestor_record_offset":15464,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":16472,"parent_record_offset":16016,"ancestor_record_offset":15464,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":19508,"parent_record_offset":19480,"ancestor_record_offset":15464,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":19604,"parent_record_offset":19576,"ancestor_record_offset":15464,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":19536,"parent_record_offset":19508,"ancestor_record_offset":15464,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":19632,"parent_record_offset":19604,"ancestor_record_offset":15464,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1667,"record_offset":21492,"parent_record_offset":21464,"ancestor_record_offset":15464,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::alpha_distribution_visibility::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 318
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1422E
// 首条位置: {"module":1667,"inlinee":82478}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1667,"record_offset":16424,"parent_record_offset":16396,"ancestor_record_offset":16396,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::alpha_distribution_visibility::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1422F
// 首条位置: {"module":1667,"inlinee":82479}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::alpha_distribution_visibility::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 342
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF2B
// 首条位置: {"module":3257,"inlinee":118571}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3257,"record_offset":7688,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3257,"record_offset":8008,"parent_record_offset":7984,"ancestor_record_offset":7660,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3257,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":7660,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3257,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":7660,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3257,"record_offset":7984,"parent_record_offset":7816,"ancestor_record_offset":7660,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3257,"record_offset":7752,"parent_record_offset":7720,"ancestor_record_offset":7660,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3257,"record_offset":7816,"parent_record_offset":7784,"ancestor_record_offset":7660,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3257,"record_offset":7784,"parent_record_offset":7752,"ancestor_record_offset":7660,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3257,"record_offset":7856,"parent_record_offset":7816,"ancestor_record_offset":7660,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":3257,"record_offset":7720,"parent_record_offset":7688,"ancestor_record_offset":7660,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3257,"record_offset":7884,"parent_record_offset":7856,"ancestor_record_offset":7660,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3257,"record_offset":7940,"parent_record_offset":7912,"ancestor_record_offset":7660,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3257,"record_offset":7912,"parent_record_offset":7884,"ancestor_record_offset":7660,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03257.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::alpha_distribution_visibility::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 373
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF29
// 首条位置: {"module":3257,"inlinee":118569}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::build_alpha_distribution_pyramid_parent_level
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 377
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14230
// 首条位置: {"module":1667,"inlinee":82480}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":18720,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19180,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":17916,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19480,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19576,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":18748,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":17852,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":10418,"source_line":3657,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":19864,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":18204,"parent_record_offset":18180,"ancestor_record_offset":17728,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":18228,"parent_record_offset":18204,"ancestor_record_offset":17728,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":20048,"parent_record_offset":20024,"ancestor_record_offset":17728,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":20072,"parent_record_offset":20048,"ancestor_record_offset":17728,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":18252,"parent_record_offset":18228,"ancestor_record_offset":17728,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":17728,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":18180,"parent_record_offset":18032,"ancestor_record_offset":17728,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":20024,"parent_record_offset":19936,"ancestor_record_offset":17728,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":17728,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":17972,"parent_record_offset":17944,"ancestor_record_offset":17728,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":19964,"parent_record_offset":19936,"ancestor_record_offset":17728,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":19936,"parent_record_offset":19912,"ancestor_record_offset":17728,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":19320,"parent_record_offset":19296,"ancestor_record_offset":17728,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":17728,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":18032,"parent_record_offset":18000,"ancestor_record_offset":17728,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":18000,"parent_record_offset":17972,"ancestor_record_offset":17728,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":19912,"parent_record_offset":19888,"ancestor_record_offset":17728,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":18068,"parent_record_offset":18032,"ancestor_record_offset":17728,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19248,"parent_record_offset":19208,"ancestor_record_offset":17728,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":19208,"parent_record_offset":19180,"ancestor_record_offset":17728,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":20468,"parent_record_offset":20440,"ancestor_record_offset":17728,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1667,"record_offset":17944,"parent_record_offset":17916,"ancestor_record_offset":17728,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1667,"record_offset":18092,"parent_record_offset":18068,"ancestor_record_offset":17728,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":18800,"parent_record_offset":18772,"ancestor_record_offset":17728,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1667,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":17728,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":18828,"parent_record_offset":18772,"ancestor_record_offset":17728,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1667,"record_offset":18772,"parent_record_offset":18748,"ancestor_record_offset":17728,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":17728,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":18852,"parent_record_offset":18828,"ancestor_record_offset":17728,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":19992,"parent_record_offset":19964,"ancestor_record_offset":17728,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":19888,"parent_record_offset":19864,"ancestor_record_offset":17728,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":19360,"parent_record_offset":19208,"ancestor_record_offset":17728,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":19508,"parent_record_offset":19480,"ancestor_record_offset":17728,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":19604,"parent_record_offset":19576,"ancestor_record_offset":17728,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":19536,"parent_record_offset":19508,"ancestor_record_offset":17728,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":19632,"parent_record_offset":19604,"ancestor_record_offset":17728,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::distribute_alpha_visibility
// 记录形式: compiled_procedure; instances=1
// 源码行: 409
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":11896,"rva":24986912,"va":5393696032}
// 编译实例 RVAs: 0x17d4520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18017554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e8ee12edfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393696237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18018192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e87010edfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393696891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24974032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e830ed82fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393696219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db6180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6166709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e8b5185e00","size":5,"soft_fail":false,"target_va":5399863680,"va":5393696966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22654336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e880ad5901","size":5,"soft_fail":false,"target_va":5416351056,"va":5393696715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23361300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e814776401","size":5,"soft_fail":false,"target_va":5417057792,"va":5393696487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22924589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e82dcd5d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5393696200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t22924767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e8dfcd5d01","size":5,"soft_fail":false,"target_va":5416621762,"va":5393696990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22925523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e8d3d05d01","size":5,"soft_fail":false,"target_va":5416622560,"va":5393697032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6167972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393696032,"raw_hex":"e8a41d5e00","size":5,"soft_fail":false,"target_va":5399865136,"va":5393697159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":13624,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":12996,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":12580,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":13284,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":13804,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::alpha_distribution_tiebreaker
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":13588,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":82476,"source_line":456,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":12552,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":12204,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":13560,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":14016,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":12092,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":12120,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1667,"record_offset":13228,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1667,"record_offset":13476,"parent_record_offset":11896,"ancestor_record_offset":11896,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":12888,"parent_record_offset":12864,"ancestor_record_offset":11896,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":12912,"parent_record_offset":12888,"ancestor_record_offset":11896,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":14272,"parent_record_offset":14248,"ancestor_record_offset":11896,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":11896,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":12936,"parent_record_offset":12912,"ancestor_record_offset":11896,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":11896,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":12864,"parent_record_offset":12684,"ancestor_record_offset":11896,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":14248,"parent_record_offset":14160,"ancestor_record_offset":11896,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":13104,"parent_record_offset":13072,"ancestor_record_offset":11896,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":11896,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":14188,"parent_record_offset":14160,"ancestor_record_offset":11896,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":14160,"parent_record_offset":14136,"ancestor_record_offset":11896,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":13152,"parent_record_offset":13128,"ancestor_record_offset":11896,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":13128,"parent_record_offset":13104,"ancestor_record_offset":11896,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":12684,"parent_record_offset":12652,"ancestor_record_offset":11896,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":12652,"parent_record_offset":12628,"ancestor_record_offset":11896,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":14136,"parent_record_offset":14112,"ancestor_record_offset":11896,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":12720,"parent_record_offset":12684,"ancestor_record_offset":11896,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":13660,"parent_record_offset":13624,"ancestor_record_offset":11896,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":13072,"parent_record_offset":13028,"ancestor_record_offset":11896,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":13028,"parent_record_offset":12996,"ancestor_record_offset":11896,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":12604,"parent_record_offset":12580,"ancestor_record_offset":11896,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":12380,"parent_record_offset":12352,"ancestor_record_offset":11896,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":11896,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":14088,"parent_record_offset":14064,"ancestor_record_offset":11896,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::distribute_alpha_visibility::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":12352,"parent_record_offset":12324,"ancestor_record_offset":11896,"inlinee":82475,"source_line":423,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1667,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":11896,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1667,"record_offset":13500,"parent_record_offset":13476,"ancestor_record_offset":11896,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1667,"record_offset":13736,"parent_record_offset":13696,"ancestor_record_offset":11896,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1667,"record_offset":12824,"parent_record_offset":12768,"ancestor_record_offset":11896,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1667,"record_offset":12260,"parent_record_offset":12232,"ancestor_record_offset":11896,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1667,"record_offset":12324,"parent_record_offset":12288,"ancestor_record_offset":11896,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1667,"record_offset":12232,"parent_record_offset":12204,"ancestor_record_offset":11896,"inlinee":8405,"source_line":49,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1667,"record_offset":12480,"parent_record_offset":12324,"ancestor_record_offset":11896,"inlinee":8441,"source_line":53,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":11896,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":12796,"parent_record_offset":12768,"ancestor_record_offset":11896,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":12508,"parent_record_offset":12288,"ancestor_record_offset":11896,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":14216,"parent_record_offset":14188,"ancestor_record_offset":11896,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":14064,"parent_record_offset":14040,"ancestor_record_offset":11896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1667,"record_offset":12168,"parent_record_offset":12144,"ancestor_record_offset":11896,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":13860,"parent_record_offset":13804,"ancestor_record_offset":11896,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":13252,"parent_record_offset":13228,"ancestor_record_offset":11896,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":11896,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":13192,"parent_record_offset":13028,"ancestor_record_offset":11896,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":12408,"parent_record_offset":12380,"ancestor_record_offset":11896,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":12436,"parent_record_offset":12408,"ancestor_record_offset":11896,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1667,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":11896,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1667,"record_offset":12288,"parent_record_offset":12260,"ancestor_record_offset":11896,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1667,"record_offset":13696,"parent_record_offset":13660,"ancestor_record_offset":11896,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::distribute_alpha_visibility::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 423
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1422B
// 首条位置: {"module":1667,"inlinee":82475}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":12380,"parent_record_offset":12352,"ancestor_record_offset":12352,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":12408,"parent_record_offset":12380,"ancestor_record_offset":12352,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":12436,"parent_record_offset":12408,"ancestor_record_offset":12352,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::distribute_alpha_visibility::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=40
// 源码行: 441
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17522
// 首条位置: {"module":2061,"inlinee":95522}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ordering::then_with
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2061,"record_offset":652,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":19935,"source_line":652,"archive":"_pdb/C/modules/02061.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2061,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/02061.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::alpha_distribution_tiebreaker
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 456
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1422C
// 首条位置: {"module":1667,"inlinee":82476}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::append_uncompressed
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 469
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1423F
// 首条位置: {"module":3256,"inlinee":82495}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":3496,"parent_record_offset":2440,"ancestor_record_offset":2440,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":2576,"parent_record_offset":2440,"ancestor_record_offset":2440,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":2520,"parent_record_offset":2440,"ancestor_record_offset":2440,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":2492,"parent_record_offset":2440,"ancestor_record_offset":2440,"inlinee":9744,"source_line":1110,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":2772,"parent_record_offset":2744,"ancestor_record_offset":2440,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":2440,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":2840,"parent_record_offset":2804,"ancestor_record_offset":2440,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":2804,"parent_record_offset":2772,"ancestor_record_offset":2440,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":3552,"parent_record_offset":3524,"ancestor_record_offset":2440,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":2640,"parent_record_offset":2608,"ancestor_record_offset":2440,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":3812,"parent_record_offset":3552,"ancestor_record_offset":2440,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":2744,"parent_record_offset":2640,"ancestor_record_offset":2440,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":3200,"parent_record_offset":3172,"ancestor_record_offset":2440,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":3264,"parent_record_offset":3200,"ancestor_record_offset":2440,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":3456,"parent_record_offset":2640,"ancestor_record_offset":2440,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":2440,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":2608,"parent_record_offset":2576,"ancestor_record_offset":2440,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":2440,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::append_uncompressed::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":3140,"parent_record_offset":3112,"ancestor_record_offset":2440,"inlinee":82461,"source_line":474,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":2952,"parent_record_offset":2920,"ancestor_record_offset":2440,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":2688,"parent_record_offset":2640,"ancestor_record_offset":2440,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":3112,"parent_record_offset":2984,"ancestor_record_offset":2440,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":2440,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":2920,"parent_record_offset":2640,"ancestor_record_offset":2440,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":3172,"parent_record_offset":3112,"ancestor_record_offset":2440,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":3304,"parent_record_offset":2984,"ancestor_record_offset":2440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3256,"record_offset":3776,"parent_record_offset":3552,"ancestor_record_offset":2440,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":3232,"parent_record_offset":3200,"ancestor_record_offset":2440,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":3068,"parent_record_offset":3032,"ancestor_record_offset":2440,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":2440,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":3032,"parent_record_offset":2984,"ancestor_record_offset":2440,"inlinee":11952,"source_line":1896,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::size_hint
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":2440,"inlinee":6688,"source_line":1904,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::append_uncompressed::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 474
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1421D
// 首条位置: {"module":1667,"inlinee":82461}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 478
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3256,"record_offset":7524,"rva":39406384,"va":5408115504}
// 编译实例 RVAs: 0x2594b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-32437166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408115504,"raw_hex":"e8520c11fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5408115865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8236185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408115504,"raw_hex":"e899ac7d00","size":5,"soft_fail":false,"target_va":5416353040,"va":5408116850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8941219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408115504,"raw_hex":"e8a36e8800","size":5,"soft_fail":false,"target_va":5417057056,"va":5408115832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture_processing::compress
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14410934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408115504,"raw_hex":"e84a1b24ff","size":5,"soft_fail":false,"target_va":5393704848,"va":5408115777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture_processing::srgb_oetf
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14409293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408115504,"raw_hex":"e8b32124ff","size":5,"soft_fail":false,"target_va":5393706336,"va":5408115624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8504810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408115504,"raw_hex":"e8eac58100","size":5,"soft_fail":false,"target_va":5416621600,"va":5408116785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8505549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408115504,"raw_hex":"e8cdc88100","size":5,"soft_fail":false,"target_va":5416622320,"va":5408116766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25096304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408115504,"raw_hex":"e8900f81fe","size":5,"soft_fail":false,"target_va":5383019440,"va":5408115739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba32f
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25096005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408115504,"raw_hex":"e8bb1081fe","size":5,"soft_fail":false,"target_va":5383019552,"va":5408115552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture_processing::apply_transfer_function
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":7752,"parent_record_offset":7524,"ancestor_record_offset":7524,"inlinee":82464,"source_line":497,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::process_regular_image::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":8464,"parent_record_offset":7524,"ancestor_record_offset":7524,"inlinee":118565,"source_line":192,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3256,"record_offset":11480,"parent_record_offset":7524,"ancestor_record_offset":7524,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":11120,"parent_record_offset":7524,"ancestor_record_offset":7524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::DynamicImage::as_rgba8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3256,"record_offset":11452,"parent_record_offset":7524,"ancestor_record_offset":7524,"inlinee":118564,"source_line":391,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3256,"record_offset":8436,"parent_record_offset":7524,"ancestor_record_offset":7524,"inlinee":82467,"source_line":929,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":7524,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":9260,"parent_record_offset":9236,"ancestor_record_offset":7524,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":7524,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":9212,"parent_record_offset":9156,"ancestor_record_offset":7524,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":7524,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":8892,"parent_record_offset":8864,"ancestor_record_offset":7524,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":9184,"parent_record_offset":9156,"ancestor_record_offset":7524,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":7524,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":8864,"parent_record_offset":8828,"ancestor_record_offset":7524,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":8828,"parent_record_offset":8796,"ancestor_record_offset":7524,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":7524,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8720,"parent_record_offset":8676,"ancestor_record_offset":7524,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":10756,"parent_record_offset":9700,"ancestor_record_offset":7524,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":9900,"parent_record_offset":9868,"ancestor_record_offset":7524,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8980,"parent_record_offset":8720,"ancestor_record_offset":7524,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8768,"parent_record_offset":8720,"ancestor_record_offset":7524,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8592,"parent_record_offset":8556,"ancestor_record_offset":7524,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":8556,"parent_record_offset":8464,"ancestor_record_offset":7524,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":10460,"parent_record_offset":10432,"ancestor_record_offset":7524,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3256,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":7524,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3256,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":7524,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":10524,"parent_record_offset":10460,"ancestor_record_offset":7524,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":10716,"parent_record_offset":9900,"ancestor_record_offset":7524,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":10660,"parent_record_offset":10636,"ancestor_record_offset":7524,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":9868,"parent_record_offset":9836,"ancestor_record_offset":7524,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":8676,"parent_record_offset":8556,"ancestor_record_offset":7524,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":10784,"parent_record_offset":10756,"ancestor_record_offset":7524,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::append_uncompressed
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":9700,"parent_record_offset":8464,"ancestor_record_offset":7524,"inlinee":82495,"source_line":469,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::append_uncompressed::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":10400,"parent_record_offset":10372,"ancestor_record_offset":7524,"inlinee":82461,"source_line":474,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":10212,"parent_record_offset":10180,"ancestor_record_offset":7524,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":9948,"parent_record_offset":9900,"ancestor_record_offset":7524,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":10372,"parent_record_offset":10244,"ancestor_record_offset":7524,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":10244,"parent_record_offset":10212,"ancestor_record_offset":7524,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":10180,"parent_record_offset":9900,"ancestor_record_offset":7524,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":10432,"parent_record_offset":10372,"ancestor_record_offset":7524,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":3256,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":7524,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":3256,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":7524,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3256,"record_offset":7928,"parent_record_offset":7908,"ancestor_record_offset":7524,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3256,"record_offset":8228,"parent_record_offset":8196,"ancestor_record_offset":7524,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3256,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":7524,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3256,"record_offset":14616,"parent_record_offset":14584,"ancestor_record_offset":7524,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3256,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":7524,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3256,"record_offset":8104,"parent_record_offset":7820,"ancestor_record_offset":7524,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":9012,"parent_record_offset":8676,"ancestor_record_offset":7524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3256,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":7524,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":10492,"parent_record_offset":10460,"ancestor_record_offset":7524,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":9780,"parent_record_offset":9700,"ancestor_record_offset":7524,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":10328,"parent_record_offset":10292,"ancestor_record_offset":7524,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":8332,"parent_record_offset":8304,"ancestor_record_offset":7524,"inlinee":8567,"source_line":2192,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":8364,"parent_record_offset":8332,"ancestor_record_offset":7524,"inlinee":11985,"source_line":2092,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3256,"record_offset":8140,"parent_record_offset":7820,"ancestor_record_offset":7524,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3256,"record_offset":8196,"parent_record_offset":8168,"ancestor_record_offset":7524,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3256,"record_offset":8168,"parent_record_offset":8140,"ancestor_record_offset":7524,"inlinee":11639,"source_line":533,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":9804,"parent_record_offset":9780,"ancestor_record_offset":7524,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":10292,"parent_record_offset":10244,"ancestor_record_offset":7524,"inlinee":11952,"source_line":1896,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":7524,"inlinee":6688,"source_line":1904,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":8304,"parent_record_offset":8276,"ancestor_record_offset":7524,"inlinee":9636,"source_line":2051,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":7860,"parent_record_offset":7820,"ancestor_record_offset":7524,"inlinee":9623,"source_line":807,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":7524,"inlinee":42061,"source_line":892,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":7788,"parent_record_offset":7752,"ancestor_record_offset":7524,"inlinee":9627,"source_line":898,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":7524,"inlinee":9625,"source_line":809,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":8016,"parent_record_offset":7992,"ancestor_record_offset":7524,"inlinee":9626,"source_line":810,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":9752,"parent_record_offset":9700,"ancestor_record_offset":7524,"inlinee":9744,"source_line":1110,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":8276,"parent_record_offset":7752,"ancestor_record_offset":7524,"inlinee":9635,"source_line":98,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 478
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3256,"record_offset":264,"rva":39404480,"va":5408113600}
// 编译实例 RVAs: 0x25943c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-32435262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408113600,"raw_hex":"e8c21311fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5408113961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8238089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408113600,"raw_hex":"e809b47d00","size":5,"soft_fail":false,"target_va":5416353040,"va":5408114946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8943123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408113600,"raw_hex":"e813768800","size":5,"soft_fail":false,"target_va":5417057056,"va":5408113928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture_processing::compress
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14409030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408113600,"raw_hex":"e8ba2224ff","size":5,"soft_fail":false,"target_va":5393704848,"va":5408113873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture_processing::srgb_oetf
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14407389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408113600,"raw_hex":"e8232924ff","size":5,"soft_fail":false,"target_va":5393706336,"va":5408113720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8506714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408113600,"raw_hex":"e85acd8100","size":5,"soft_fail":false,"target_va":5416621600,"va":5408114881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8507453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408113600,"raw_hex":"e83dd08100","size":5,"soft_fail":false,"target_va":5416622320,"va":5408114862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25094400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408113600,"raw_hex":"e8001781fe","size":5,"soft_fail":false,"target_va":5383019440,"va":5408113835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba32f
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25094101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408113600,"raw_hex":"e82b1881fe","size":5,"soft_fail":false,"target_va":5383019552,"va":5408113648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture_processing::apply_transfer_function
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":492,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":82464,"source_line":497,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::process_regular_image::closure$3
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":1204,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":118563,"source_line":218,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3256,"record_offset":4220,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":3860,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::DynamicImage::as_rgba8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3256,"record_offset":4192,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":118564,"source_line":391,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3256,"record_offset":1176,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":82467,"source_line":929,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3256,"record_offset":1952,"parent_record_offset":1896,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1536,"parent_record_offset":1508,"ancestor_record_offset":264,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1632,"parent_record_offset":1604,"ancestor_record_offset":264,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1604,"parent_record_offset":1568,"ancestor_record_offset":264,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1568,"parent_record_offset":1536,"ancestor_record_offset":264,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3256,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1460,"parent_record_offset":1416,"ancestor_record_offset":264,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":3496,"parent_record_offset":2440,"ancestor_record_offset":264,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":2640,"parent_record_offset":2608,"ancestor_record_offset":264,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1720,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1508,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1332,"parent_record_offset":1296,"ancestor_record_offset":264,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":1296,"parent_record_offset":1204,"ancestor_record_offset":264,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3256,"record_offset":3200,"parent_record_offset":3172,"ancestor_record_offset":264,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3256,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":264,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3256,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":264,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":3264,"parent_record_offset":3200,"ancestor_record_offset":264,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":3456,"parent_record_offset":2640,"ancestor_record_offset":264,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3256,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":264,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":2608,"parent_record_offset":2576,"ancestor_record_offset":264,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":1416,"parent_record_offset":1296,"ancestor_record_offset":264,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3256,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":264,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::append_uncompressed
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":2440,"parent_record_offset":1204,"ancestor_record_offset":264,"inlinee":82495,"source_line":469,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::append_uncompressed::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":3256,"record_offset":3140,"parent_record_offset":3112,"ancestor_record_offset":264,"inlinee":82461,"source_line":474,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":2952,"parent_record_offset":2920,"ancestor_record_offset":264,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":2688,"parent_record_offset":2640,"ancestor_record_offset":264,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3256,"record_offset":3112,"parent_record_offset":2984,"ancestor_record_offset":264,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":264,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":2920,"parent_record_offset":2640,"ancestor_record_offset":264,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3256,"record_offset":3172,"parent_record_offset":3112,"ancestor_record_offset":264,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":3256,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":264,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":3256,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":264,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3256,"record_offset":668,"parent_record_offset":648,"ancestor_record_offset":264,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3256,"record_offset":968,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3256,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":264,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3256,"record_offset":7356,"parent_record_offset":7324,"ancestor_record_offset":264,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3256,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":264,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3256,"record_offset":844,"parent_record_offset":560,"ancestor_record_offset":264,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":1752,"parent_record_offset":1416,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3256,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":264,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3256,"record_offset":3232,"parent_record_offset":3200,"ancestor_record_offset":264,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":2520,"parent_record_offset":2440,"ancestor_record_offset":264,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":3068,"parent_record_offset":3032,"ancestor_record_offset":264,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":264,"inlinee":8567,"source_line":2192,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":1104,"parent_record_offset":1072,"ancestor_record_offset":264,"inlinee":11985,"source_line":2092,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3256,"record_offset":880,"parent_record_offset":560,"ancestor_record_offset":264,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3256,"record_offset":936,"parent_record_offset":908,"ancestor_record_offset":264,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3256,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":264,"inlinee":11639,"source_line":533,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":264,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":3032,"parent_record_offset":2984,"ancestor_record_offset":264,"inlinee":11952,"source_line":1896,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":264,"inlinee":6688,"source_line":1904,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3256,"record_offset":1044,"parent_record_offset":1016,"ancestor_record_offset":264,"inlinee":9636,"source_line":2051,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":600,"parent_record_offset":560,"ancestor_record_offset":264,"inlinee":9623,"source_line":807,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":560,"parent_record_offset":528,"ancestor_record_offset":264,"inlinee":42061,"source_line":892,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":528,"parent_record_offset":492,"ancestor_record_offset":264,"inlinee":9627,"source_line":898,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":264,"inlinee":9625,"source_line":809,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":264,"inlinee":9626,"source_line":810,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":2492,"parent_record_offset":2440,"ancestor_record_offset":264,"inlinee":9744,"source_line":1110,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":3256,"record_offset":1016,"parent_record_offset":492,"ancestor_record_offset":264,"inlinee":9635,"source_line":98,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::apply_transfer_function
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14220
// 首条位置: {"module":1667,"inlinee":82464}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture_processing::srgb_eotf
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":5284,"parent_record_offset":4624,"ancestor_record_offset":4624,"inlinee":82465,"source_line":517,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4672,"parent_record_offset":4624,"ancestor_record_offset":4624,"inlinee":9627,"source_line":898,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":5176,"parent_record_offset":4624,"ancestor_record_offset":4624,"inlinee":9635,"source_line":98,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":4824,"parent_record_offset":4800,"ancestor_record_offset":4624,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":5128,"parent_record_offset":5096,"ancestor_record_offset":4624,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":4624,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":4776,"parent_record_offset":4748,"ancestor_record_offset":4624,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":5004,"parent_record_offset":4708,"ancestor_record_offset":4624,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3256,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":492,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_checked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":5240,"parent_record_offset":5208,"ancestor_record_offset":4624,"inlinee":8567,"source_line":2192,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3256,"record_offset":1104,"parent_record_offset":1072,"ancestor_record_offset":492,"inlinee":11985,"source_line":2092,"archive":"_pdb/C/modules/03256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":5040,"parent_record_offset":4708,"ancestor_record_offset":4624,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":5096,"parent_record_offset":5068,"ancestor_record_offset":4624,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":4624,"inlinee":11639,"source_line":533,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1667,"record_offset":5208,"parent_record_offset":5176,"ancestor_record_offset":4624,"inlinee":9636,"source_line":2051,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4748,"parent_record_offset":4708,"ancestor_record_offset":4624,"inlinee":9623,"source_line":807,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4708,"parent_record_offset":4672,"ancestor_record_offset":4624,"inlinee":42061,"source_line":892,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4624,"inlinee":9625,"source_line":809,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":4624,"inlinee":9626,"source_line":810,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1667,"record_offset":5320,"parent_record_offset":5284,"ancestor_record_offset":4624,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::srgb_oetf
// 记录形式: compiled_procedure; instances=1
// 源码行: 509
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":36576,"rva":24997216,"va":5393706336}
// 编译实例 RVAs: 0x17d6d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23351570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393706336,"raw_hex":"e812516401","size":5,"soft_fail":false,"target_va":5417057936,"va":5393706361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1667,"record_offset":36704,"parent_record_offset":36576,"ancestor_record_offset":36576,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::srgb_eotf
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 517
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14221
// 首条位置: {"module":1667,"inlinee":82465}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1667,"record_offset":5320,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_dds_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 525
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":1088,"rva":24978544,"va":5393687664}
// 编译实例 RVAs: 0x17d2470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9079011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e81d7775ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5393688414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18010242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e87e2fedfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393688925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-18009746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e86e31edfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393688445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24966725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8bb0983fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393688912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d6da0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8e8410000","size":5,"soft_fail":false,"target_va":5393706400,"va":5393689523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425939c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14423267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8e314dc00","size":5,"soft_fail":false,"target_va":5408111040,"va":5393687768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142594250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14425437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e85d1ddc00","size":5,"soft_fail":false,"target_va":5408113232,"va":5393687790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23369195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8eb956401","size":5,"soft_fail":false,"target_va":5417057056,"va":5393687856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8893223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8d94c78ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5393688082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22931479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e817e85d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5393689310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23162881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e801706101","size":5,"soft_fail":false,"target_va":5416851728,"va":5393688842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23095805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8fd696001","size":5,"soft_fail":false,"target_va":5416785104,"va":5393689294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22933522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e812f05d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5393688713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22932418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8c2eb5d01","size":5,"soft_fail":false,"target_va":5416621632,"va":5393689209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_d3d_format
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5842639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8cf265900","size":5,"soft_fail":false,"target_va":5399530672,"va":5393688028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5843543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8572a5900","size":5,"soft_fail":false,"target_va":5399532016,"va":5393688468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_dxgi_format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t22082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e842560000","size":5,"soft_fail":false,"target_va":5393710704,"va":5393688617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":2628,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":1667,"record_offset":2508,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":1460,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1667,"record_offset":2536,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":6246,"source_line":818,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1652,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":2020,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":2304,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":1308,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":1348,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_depth
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1667,"record_offset":1516,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":42296,"source_line":249,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_dxgi_format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1667,"record_offset":1572,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":42293,"source_line":218,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_num_mipmap_levels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1667,"record_offset":2472,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":82459,"source_line":322,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1667,"record_offset":1908,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":1376,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1667,"record_offset":1280,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":23648,"source_line":98,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":1088,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":1088,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":1088,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1088,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":1088,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1088,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":1088,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2172,"parent_record_offset":2116,"ancestor_record_offset":1088,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":1088,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":1088,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2776,"parent_record_offset":2748,"ancestor_record_offset":1088,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2144,"parent_record_offset":2116,"ancestor_record_offset":1088,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1088,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2836,"parent_record_offset":2804,"ancestor_record_offset":1088,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2804,"parent_record_offset":2776,"ancestor_record_offset":1088,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1088,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":2656,"parent_record_offset":2628,"ancestor_record_offset":1088,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":2684,"parent_record_offset":2656,"ancestor_record_offset":1088,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":2712,"parent_record_offset":2684,"ancestor_record_offset":1088,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":2748,"parent_record_offset":2712,"ancestor_record_offset":1088,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::process_dds_image::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":1680,"parent_record_offset":1652,"ancestor_record_offset":1088,"inlinee":82457,"source_line":538,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::process_dds_image::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":1792,"parent_record_offset":1652,"ancestor_record_offset":1088,"inlinee":82458,"source_line":539,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$138::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":1667,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":1088,"inlinee":10438,"source_line":296,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":1088,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":1088,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":1088,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1667,"record_offset":2996,"parent_record_offset":2712,"ancestor_record_offset":1088,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":1088,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":1088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":1088,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":1088,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::format::dxgi::DxgiFormat::try_from_pixel_format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/format/dxgi.rs
//     evidence={"module":1667,"record_offset":1604,"parent_record_offset":1572,"ancestor_record_offset":1088,"inlinee":42295,"source_line":416,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":1088,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1667,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":1088,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":1088,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_dds_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 530
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF20
// 首条位置: {"module":3255,"inlinee":118560}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3255,"record_offset":4084,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3255,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":4064,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03255.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3255,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":4064,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03255.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_dds_image::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 538
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14219
// 首条位置: {"module":1667,"inlinee":82457}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":1680,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":1680,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":1680,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_dds_image::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 539
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1421A
// 首条位置: {"module":1667,"inlinee":82458}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":1792,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":1792,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":1792,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_image_array_from_layers
// 记录形式: compiled_procedure; instances=1
// 源码行: 589
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":27884,"rva":24993712,"va":5393702832}
// 编译实例 RVAs: 0x17d5fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18024450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e8fef7ecfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393703133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-18024512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e8c0f7ecfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393703211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24980927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e841d282fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393703114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22649507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e8a39a5901","size":5,"soft_fail":false,"target_va":5416353040,"va":5393703528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23353892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e8245a6401","size":5,"soft_fail":false,"target_va":5417057056,"va":5393703159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-8908862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e8c20f78ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5393703721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22917237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e875b05d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5393703552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-620009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e8178af6ff","size":5,"soft_fail":false,"target_va":5393082944,"va":5393702948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23147797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e815356101","size":5,"soft_fail":false,"target_va":5416851552,"va":5393703750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22917081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e8d9af5d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5393704514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$core..iter..adapters..enumerate..Enumerate$LT$alloc..vec..into_iter..IntoIter$LT$alloc..sync..Arc$LT$asset..texture..blob..TextureAssetBlob$GT$$GT$$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6154021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e825e75d00","size":5,"soft_fail":false,"target_va":5399857600,"va":5393703574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22917135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393702832,"raw_hex":"e80fb05d01","size":5,"soft_fail":false,"target_va":5416621632,"va":5393704492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":30112,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":29740,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":28644,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":28072,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1667,"record_offset":29544,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":16288,"source_line":14,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":29628,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1667,"record_offset":30196,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1667,"record_offset":28416,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":30076,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":28188,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":28608,"parent_record_offset":27884,"ancestor_record_offset":27884,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":29056,"parent_record_offset":29032,"ancestor_record_offset":27884,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":29080,"parent_record_offset":29056,"ancestor_record_offset":27884,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":29432,"parent_record_offset":29408,"ancestor_record_offset":27884,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":29456,"parent_record_offset":29432,"ancestor_record_offset":27884,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":29104,"parent_record_offset":29080,"ancestor_record_offset":27884,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":29480,"parent_record_offset":29456,"ancestor_record_offset":27884,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":29032,"parent_record_offset":28880,"ancestor_record_offset":27884,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":29408,"parent_record_offset":29352,"ancestor_record_offset":27884,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":30160,"parent_record_offset":30136,"ancestor_record_offset":27884,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":29872,"parent_record_offset":29844,"ancestor_record_offset":27884,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":28812,"parent_record_offset":28780,"ancestor_record_offset":27884,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":29380,"parent_record_offset":29352,"ancestor_record_offset":27884,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":29352,"parent_record_offset":29328,"ancestor_record_offset":27884,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":29932,"parent_record_offset":29900,"ancestor_record_offset":27884,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":29900,"parent_record_offset":29872,"ancestor_record_offset":27884,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":28880,"parent_record_offset":28844,"ancestor_record_offset":27884,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":28844,"parent_record_offset":28812,"ancestor_record_offset":27884,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":29328,"parent_record_offset":29304,"ancestor_record_offset":27884,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":28920,"parent_record_offset":28880,"ancestor_record_offset":27884,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":28740,"parent_record_offset":28672,"ancestor_record_offset":27884,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1667,"record_offset":28288,"parent_record_offset":28220,"ancestor_record_offset":27884,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1667,"record_offset":28220,"parent_record_offset":28188,"ancestor_record_offset":27884,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":29796,"parent_record_offset":29768,"ancestor_record_offset":27884,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":28712,"parent_record_offset":28672,"ancestor_record_offset":27884,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":30036,"parent_record_offset":29796,"ancestor_record_offset":27884,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":29844,"parent_record_offset":29796,"ancestor_record_offset":27884,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":28780,"parent_record_offset":28740,"ancestor_record_offset":27884,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":28672,"parent_record_offset":28644,"ancestor_record_offset":27884,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1667,"record_offset":28444,"parent_record_offset":28416,"ancestor_record_offset":27884,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1667,"record_offset":29768,"parent_record_offset":29740,"ancestor_record_offset":27884,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":1667,"record_offset":29708,"parent_record_offset":29684,"ancestor_record_offset":27884,"inlinee":38726,"source_line":197,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":1667,"record_offset":29652,"parent_record_offset":29628,"ancestor_record_offset":27884,"inlinee":82488,"source_line":5,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1667,"record_offset":28944,"parent_record_offset":28920,"ancestor_record_offset":27884,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1667,"record_offset":29592,"parent_record_offset":29568,"ancestor_record_offset":27884,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1667,"record_offset":29568,"parent_record_offset":29544,"ancestor_record_offset":27884,"inlinee":16289,"source_line":159,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1667,"record_offset":28992,"parent_record_offset":28968,"ancestor_record_offset":27884,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":1667,"record_offset":28100,"parent_record_offset":28072,"ancestor_record_offset":27884,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":1667,"record_offset":28124,"parent_record_offset":28100,"ancestor_record_offset":27884,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":28968,"parent_record_offset":28944,"ancestor_record_offset":27884,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":29956,"parent_record_offset":29932,"ancestor_record_offset":27884,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":30136,"parent_record_offset":30112,"ancestor_record_offset":27884,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1667,"record_offset":29160,"parent_record_offset":28740,"ancestor_record_offset":27884,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":29184,"parent_record_offset":29160,"ancestor_record_offset":27884,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":29256,"parent_record_offset":29228,"ancestor_record_offset":27884,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1667,"record_offset":28156,"parent_record_offset":28072,"ancestor_record_offset":27884,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":28516,"parent_record_offset":28444,"ancestor_record_offset":27884,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":28340,"parent_record_offset":28288,"ancestor_record_offset":27884,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":28544,"parent_record_offset":28444,"ancestor_record_offset":27884,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1667,"record_offset":28484,"parent_record_offset":28444,"ancestor_record_offset":27884,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":28568,"parent_record_offset":28544,"ancestor_record_offset":27884,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":28372,"parent_record_offset":28288,"ancestor_record_offset":27884,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":28260,"parent_record_offset":28220,"ancestor_record_offset":27884,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_texture_array_layer
// 记录形式: compiled_procedure; instances=1
// 源码行: 665
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":15056,"rva":24988352,"va":5393697472}
// 编译实例 RVAs: 0x17d4ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9088319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697472,"raw_hex":"e8c15275ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5393697722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture_processing::process_regular_image
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393697472,"raw_hex":"e8f6dfffff","size":5,"soft_fail":false,"target_va":5393689552,"va":5393697749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":15292,"parent_record_offset":15056,"ancestor_record_offset":15056,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1667,"record_offset":15348,"parent_record_offset":15056,"ancestor_record_offset":15056,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":15208,"parent_record_offset":15056,"ancestor_record_offset":15056,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":15056,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":15232,"parent_record_offset":15208,"ancestor_record_offset":15056,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":15256,"parent_record_offset":15232,"ancestor_record_offset":15056,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":15056,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1667,"record_offset":15372,"parent_record_offset":15348,"ancestor_record_offset":15056,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":15420,"parent_record_offset":15396,"ancestor_record_offset":15056,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::impl$1::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 686
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":41608,"rva":25001488,"va":5393710608}
// 编译实例 RVAs: 0x17d7e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-8886562","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393710608,"raw_hex":"e9de6678ff","size":5,"soft_fail":false,"target_va":5384824080,"va":5393710637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::BcMode::block_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 701
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1423C
// 首条位置: {"module":1667,"inlinee":82492}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::compress
// 记录形式: compiled_procedure; instances=1
// 源码行: 709
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":32328,"rva":24995728,"va":5393704848}
// 编译实例 RVAs: 0x17d6790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9095897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e8273575ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5393705300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18026693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e83befecfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393705376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18026326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e8aaf0ecfe","size":5,"soft_fail":false,"target_va":5375678736,"va":5393705057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-18026892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e874eeecfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393705591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24982857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e8b7ca82fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393705044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142595820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_processing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14413311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e8ffeddb00","size":5,"soft_fail":false,"target_va":5408118816,"va":5393705500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22914718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e89ea65d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5393706071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22915538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e8d2a95d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5393706057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22916270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e8aeac5d01","size":5,"soft_fail":false,"target_va":5416622320,"va":5393706045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] intel_tex_2::bc4::compress_blocks_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"intel_tex_2","version":"0.4.0","source":"git+https://github.com/PounceLight/intel-tex-rs-2.git?rev=4fb0e0b70f868bccd529d45e1626de2f55a20b83#4fb0e0b70f868bccd529d45e1626de2f55a20b83"}]
//     evidence={"assembly":"\tcallq\t14415144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e828f5db00","size":5,"soft_fail":false,"target_va":5408121104,"va":5393705955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] intel_tex_2::bc5::compress_blocks_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"intel_tex_2","version":"0.4.0","source":"git+https://github.com/PounceLight/intel-tex-rs-2.git?rev=4fb0e0b70f868bccd529d45e1626de2f55a20b83#4fb0e0b70f868bccd529d45e1626de2f55a20b83"}]
//     evidence={"assembly":"\tcallq\t14415671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e837f7db00","size":5,"soft_fail":false,"target_va":5408121232,"va":5393705556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] intel_tex_2::bc7::compress_blocks_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"intel_tex_2","version":"0.4.0","source":"git+https://github.com/PounceLight/intel-tex-rs-2.git?rev=4fb0e0b70f868bccd529d45e1626de2f55a20b83#4fb0e0b70f868bccd529d45e1626de2f55a20b83"}]
//     evidence={"assembly":"\tcallq\t14415617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393704848,"raw_hex":"e801f7db00","size":5,"soft_fail":false,"target_va":5408121360,"va":5393705738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":33200,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture_processing::BcMode::block_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":33172,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":82492,"source_line":701,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/lib.rs
//     evidence={"module":1667,"record_offset":35116,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":8834,"source_line":408,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":33600,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":32972,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":33832,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":35228,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":33776,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":32492,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":19456,"source_line":720,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1667,"record_offset":33664,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":33504,"parent_record_offset":32328,"ancestor_record_offset":32328,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":33396,"parent_record_offset":33372,"ancestor_record_offset":32328,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":33420,"parent_record_offset":33396,"ancestor_record_offset":32328,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":35412,"parent_record_offset":35388,"ancestor_record_offset":32328,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":35436,"parent_record_offset":35412,"ancestor_record_offset":32328,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":34352,"parent_record_offset":34328,"ancestor_record_offset":32328,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":33444,"parent_record_offset":33420,"ancestor_record_offset":32328,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":35460,"parent_record_offset":35436,"ancestor_record_offset":32328,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":34280,"parent_record_offset":34236,"ancestor_record_offset":32328,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":33372,"parent_record_offset":33332,"ancestor_record_offset":32328,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":35388,"parent_record_offset":35300,"ancestor_record_offset":32328,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":34152,"parent_record_offset":34112,"ancestor_record_offset":32328,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":33264,"parent_record_offset":33232,"ancestor_record_offset":32328,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":35328,"parent_record_offset":35300,"ancestor_record_offset":32328,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":32328,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":33332,"parent_record_offset":33296,"ancestor_record_offset":32328,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":34192,"parent_record_offset":34152,"ancestor_record_offset":32328,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":33296,"parent_record_offset":33264,"ancestor_record_offset":32328,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":35276,"parent_record_offset":35252,"ancestor_record_offset":32328,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":34440,"parent_record_offset":34412,"ancestor_record_offset":32328,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":34072,"parent_record_offset":33964,"ancestor_record_offset":32328,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":34112,"parent_record_offset":34072,"ancestor_record_offset":32328,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":33876,"parent_record_offset":33832,"ancestor_record_offset":32328,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":34700,"parent_record_offset":34668,"ancestor_record_offset":32328,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1667,"record_offset":34744,"parent_record_offset":34700,"ancestor_record_offset":32328,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1667,"record_offset":34412,"parent_record_offset":33964,"ancestor_record_offset":32328,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1667,"record_offset":33232,"parent_record_offset":33200,"ancestor_record_offset":32328,"inlinee":9075,"source_line":49,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1667,"record_offset":33920,"parent_record_offset":33876,"ancestor_record_offset":32328,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1667,"record_offset":33964,"parent_record_offset":33920,"ancestor_record_offset":32328,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::compress::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":33132,"parent_record_offset":33108,"ancestor_record_offset":32328,"inlinee":82490,"source_line":714,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::compress::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":34820,"parent_record_offset":34636,"ancestor_record_offset":32328,"inlinee":82493,"source_line":729,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/internal.rs
//     evidence={"module":1667,"record_offset":35140,"parent_record_offset":35116,"ancestor_record_offset":32328,"inlinee":8836,"source_line":254,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/internal.rs
//     evidence={"module":1667,"record_offset":35164,"parent_record_offset":35140,"ancestor_record_offset":32328,"inlinee":8837,"source_line":353,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1667,"record_offset":34496,"parent_record_offset":34468,"ancestor_record_offset":32328,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1667,"record_offset":34016,"parent_record_offset":33964,"ancestor_record_offset":32328,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1667,"record_offset":34636,"parent_record_offset":34524,"ancestor_record_offset":32328,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1667,"record_offset":35076,"parent_record_offset":35048,"ancestor_record_offset":32328,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":33108,"parent_record_offset":32996,"ancestor_record_offset":32328,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":34524,"parent_record_offset":34496,"ancestor_record_offset":32328,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":34468,"parent_record_offset":34440,"ancestor_record_offset":32328,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":34668,"parent_record_offset":34636,"ancestor_record_offset":32328,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1667,"record_offset":32996,"parent_record_offset":32972,"ancestor_record_offset":32328,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1667,"record_offset":35188,"parent_record_offset":35164,"ancestor_record_offset":32328,"inlinee":9288,"source_line":372,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":32624,"parent_record_offset":32604,"ancestor_record_offset":32328,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":32924,"parent_record_offset":32892,"ancestor_record_offset":32328,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":32648,"parent_record_offset":32624,"ancestor_record_offset":32328,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1667,"record_offset":35356,"parent_record_offset":35328,"ancestor_record_offset":32328,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":32580,"parent_record_offset":32556,"ancestor_record_offset":32328,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":33628,"parent_record_offset":33600,"ancestor_record_offset":32328,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":32800,"parent_record_offset":32520,"ancestor_record_offset":32328,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1667,"record_offset":34980,"parent_record_offset":34952,"ancestor_record_offset":32328,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":35252,"parent_record_offset":35228,"ancestor_record_offset":32328,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":34780,"parent_record_offset":34700,"ancestor_record_offset":32328,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":33072,"parent_record_offset":33048,"ancestor_record_offset":32328,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1667,"record_offset":34952,"parent_record_offset":34920,"ancestor_record_offset":32328,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":32836,"parent_record_offset":32520,"ancestor_record_offset":32328,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":32892,"parent_record_offset":32864,"ancestor_record_offset":32328,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1667,"record_offset":32864,"parent_record_offset":32836,"ancestor_record_offset":32328,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1667,"record_offset":33800,"parent_record_offset":33776,"ancestor_record_offset":32328,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1667,"record_offset":33048,"parent_record_offset":33024,"ancestor_record_offset":32328,"inlinee":11952,"source_line":1896,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1667,"record_offset":34040,"parent_record_offset":34016,"ancestor_record_offset":32328,"inlinee":6688,"source_line":1904,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":33532,"parent_record_offset":33504,"ancestor_record_offset":32328,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":33560,"parent_record_offset":33532,"ancestor_record_offset":32328,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":32556,"parent_record_offset":32520,"ancestor_record_offset":32328,"inlinee":9623,"source_line":807,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":32520,"parent_record_offset":32492,"ancestor_record_offset":32328,"inlinee":9631,"source_line":713,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":33024,"parent_record_offset":32996,"ancestor_record_offset":32328,"inlinee":19458,"source_line":34,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":32604,"parent_record_offset":32580,"ancestor_record_offset":32328,"inlinee":9625,"source_line":809,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":1667,"record_offset":32712,"parent_record_offset":32688,"ancestor_record_offset":32328,"inlinee":9626,"source_line":810,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":33712,"parent_record_offset":33688,"ancestor_record_offset":32328,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1667,"record_offset":33688,"parent_record_offset":33664,"ancestor_record_offset":32328,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":33736,"parent_record_offset":33712,"ancestor_record_offset":32328,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::compress::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1423A
// 首条位置: {"module":1667,"inlinee":82490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::compress::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 729
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1423D
// 首条位置: {"module":1667,"inlinee":82493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::compress::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 745
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF2A
// 首条位置: {"module":3257,"inlinee":118570}
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
// 完整结构及原始类型记录: texture_processing.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture_processing::AlphaDistributionPyramidLevel
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<asset_pipe::texture_processing::AlphaDistributionPyramidLevel,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["asset_pipe::texture_processing::AlphaDistributionPyramidLevel", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset_pipe::texture_processing::AlphaDistributionPyramidLevel>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset_pipe::texture_processing::AlphaDistributionPyramidLevel"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<asset_pipe::texture_processing::AlphaDistributionPyramidLevel>", "constructor": "core::slice::iter::Iter", "arguments": ["asset_pipe::texture_processing::AlphaDistributionPyramidLevel"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture_processing::AlphaDistributionPyramidLevel>,asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<asset_pipe::texture_processing::AlphaDistributionPyramidLevel>", "asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<usize,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture_processing::AlphaDistributionPyramidLevel>,asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<usize,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture_processing::AlphaDistributionPyramidLevel>,asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture_processing::BcMode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: texture_processing.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: texture_processing.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::BcMode::block_bytes
// 信息 ID: c78ce097efd56102dbfea806
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::compress; module=1667, inline_record=33172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::alpha_distribution_tiebreaker
// 信息 ID: 3b882943d4348cd089753ba8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::distribute_alpha_visibility; module=1667, inline_record=13588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::alpha_distribution_visibility
// 信息 ID: c4744d7784ffc367e6aa993b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture_processing::process_regular_image @ 0x17d333e -> 0x17d4bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::process_regular_image @ 0x17d349b -> 0x17d4bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::alpha_distribution_visibility::closure$0
// 信息 ID: ea55c21c1d04e0dc224626d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture_processing::alpha_distribution_visibility", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::alpha_distribution_visibility; module=1667, inline_record=17696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::alpha_distribution_visibility::closure$1
// 信息 ID: 0096265dcf3cf1aeac1bc857
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture_processing::alpha_distribution_visibility", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<usize,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture_processing::AlphaDistributionPyramidLevel>,asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<usize,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture_processing::AlphaDistributionPyramidLevel>,asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$1> >; module=3257, inline_record=7660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::alpha_distribution_visibility::closure$2
// 信息 ID: c52579bcadfcbd42e640cd32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture_processing::alpha_distribution_visibility", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$2 <- alloc::vec::in_place_collect::impl$1::from_iter<bool,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global>,asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<bool,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global>,asset_pipe::texture_processing::alpha_distribution_visibility::closure_env$2> >; module=3257, inline_record=1344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::alpha_distribution_visibility::closure$3
// 信息 ID: 8845f8a558505e19ca949cca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset_pipe::texture_processing::alpha_distribution_visibility", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::alpha_distribution_visibility; module=1667, inline_record=16396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::append_uncompressed
// 信息 ID: 5b2458a1852305cc5d2c34f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >; module=3256, inline_record=2440; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >; module=3256, inline_record=5500; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >; module=3256, inline_record=9700; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >; module=3256, inline_record=12760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::append_uncompressed::closure$0
// 信息 ID: be04400b42e8151eb812c522
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture_processing::append_uncompressed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >; module=3256, inline_record=3140; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >; module=3256, inline_record=6200; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >; module=3256, inline_record=10400; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >; module=3256, inline_record=13460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::apply_alpha_mip_mode
// 信息 ID: f2691f33e6624d0507dd4c3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image; module=1667, inline_record=6028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::apply_alpha_mip_mode::closure$0
// 信息 ID: b4f6cc46d49c9bd6e32f1fe1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture_processing::apply_alpha_mip_mode", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture_processing::apply_alpha_mip_mode::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<image::buffer_::Pixels<image::color::Rgba<u8> >,asset_pipe::texture_processing::apply_alpha_mip_mode::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<image::buffer_::Pixels<image::color::Rgba<u8> >,asset_pipe::texture_processing::apply_alpha_mip_mode::closure_env$0> >; module=3257, inline_record=10076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::apply_transfer_function
// 信息 ID: c2133c1c0e42dc984e965a85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image; module=1667, inline_record=4624; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >; module=3256, inline_record=492; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >; module=3256, inline_record=7752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::build_alpha_distribution_pyramid_parent_level
// 信息 ID: 6c7569bde2f1f56084b246b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::alpha_distribution_visibility; module=1667, inline_record=17728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::compress
// 信息 ID: 5477326c6d60b79f7fe5d61c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> > @ 0x25944d1 -> 0x17d6790 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> > @ 0x259456f -> 0x17d6790 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> > @ 0x2594c41 -> 0x17d6790 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> > @ 0x2594cdf -> 0x17d6790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::compress::closure$0
// 信息 ID: 6c88b60c25e6d02eb67375da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture_processing::compress", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::compress; module=1667, inline_record=33132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::compress::closure$1
// 信息 ID: ed5b1c6643ec55970085cb71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture_processing::compress", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::compress; module=1667, inline_record=34820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::compress::closure$2
// 信息 ID: 344dd27d825767bee77f05db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture_processing::compress", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture_processing::compress::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<u8,2>,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u8>,asset_pipe::texture_processing::compress::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<u8,2>,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u8>,asset_pipe::texture_processing::compress::closure_env$2> >; module=3257, inline_record=6120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::distribute_alpha_visibility
// 信息 ID: 7d1c4ab61a39965aa141c689
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture_processing::alpha_distribution_visibility @ 0x17d5779 -> 0x17d4520 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::alpha_distribution_visibility @ 0x17d58b6 -> 0x17d4520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::distribute_alpha_visibility::closure$0
// 信息 ID: 57b91ee4fcbbc0c9d41070b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture_processing::distribute_alpha_visibility", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::distribute_alpha_visibility; module=1667, inline_record=12352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::distribute_alpha_visibility::closure$1
// 信息 ID: 70f8fce82f94faffc8a64723
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture_processing::distribute_alpha_visibility", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1 <- core::slice::sort::shared::smallsort::sort8_stable<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1 <- core::slice::sort::stable::driftsort_main<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1>,alloc::vec::Vec<tuple$<usize,f32,u32>,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1 <- core::slice::sort::stable::drift::sort<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1 <- core::slice::sort::stable::merge::merge<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 35 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; module=2061, inline_record=600; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; module=2061, inline_record=736; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; module=2061, inline_record=996; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; module=2061, inline_record=1148; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; module=2061, inline_record=1476; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; module=2061, inline_record=2380; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; module=2061, inline_record=2972; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<usize,f32,u32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32,u32>,asset_pipe::texture_processing::distribute_alpha_visibility::closure_env$1> >; module=2061, inline_record=4040; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::impl$1::fmt
// 信息 ID: 4536c94f02f8ddd55ab3a3ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::open_regular_image
// 信息 ID: 208aa0df5b1fdf339aefe11d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image; module=1667, inline_record=4532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::open_regular_image::closure$0
// 信息 ID: 8af8c69aa1ea897dbf9a919d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture_processing::open_regular_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture_processing::open_regular_image::closure_env$0 <- anyhow::context::impl$0::with_context<enum2$<image::dynimage::DynamicImage>,enum2$<image::error::ImageError>,alloc::string::String,asset_pipe::texture_processing::open_regular_image::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<enum2$<image::dynimage::DynamicImage>,enum2$<image::error::ImageError>,alloc::string::String,asset_pipe::texture_processing::open_regular_image::closure_env$0>; module=3255, inline_record=3344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::pad_image
// 信息 ID: d50ea4df581c251e9e127cdd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image::closure$2; module=1667, inline_record=10996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_dds_image
// 信息 ID: b3e5746ba9911804a1b682c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fe6d5 -> 0x17d2470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_dds_image::closure$0
// 信息 ID: bf438ddfa43f9b35048cba54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture_processing::process_dds_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture_processing::process_dds_image::closure_env$0 <- anyhow::context::impl$0::with_context<ddsfile::Dds,enum2$<ddsfile::error::Error>,alloc::string::String,asset_pipe::texture_processing::process_dds_image::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<ddsfile::Dds,enum2$<ddsfile::error::Error>,alloc::string::String,asset_pipe::texture_processing::process_dds_image::closure_env$0>; module=3255, inline_record=4064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_dds_image::closure$1
// 信息 ID: fef886b7fea9f51ea4387404
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture_processing::process_dds_image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::process_dds_image; module=1667, inline_record=1680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_dds_image::closure$2
// 信息 ID: afa79ed75125dc03322244b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture_processing::process_dds_image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::process_dds_image; module=1667, inline_record=1792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_image_array_from_layers
// 信息 ID: a064c63e7d7a2fb879ec890e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8feba4 -> 0x17d5fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_regular_image
// 信息 ID: c1ad3995e50982a524d9d309
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fe709 -> 0x17d2bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::process_texture_array_layer @ 0x17d4bd5 -> 0x17d2bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_regular_image::closure$1
// 信息 ID: 129a46ec44133cd7b39c2292
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture_processing::process_regular_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture_processing::process_regular_image::closure_env$1 <- asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >; module=3256, inline_record=8464; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >; module=3256, inline_record=11516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_regular_image::closure$2
// 信息 ID: b7b27484a2d83f15b7e1d40a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture_processing::process_regular_image", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture_processing::process_regular_image @ 0x17d31d9 -> 0x17d4110 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::process_regular_image @ 0x17d38a2 -> 0x17d4110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_regular_image::closure$3
// 信息 ID: bfb5ec02a6387ec77cfc4185
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset_pipe::texture_processing::process_regular_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture_processing::process_regular_image::closure_env$3 <- asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >; module=3256, inline_record=1204; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >; module=3256, inline_record=4256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_texture_array_layer
// 信息 ID: f737f0b77ae15918ce488f34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> @ 0x8fd9d2 -> 0x17d4ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::resize_image
// 信息 ID: 3541042fa2e47184935ff800
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image; module=1667, inline_record=5656; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image::closure$2; module=1667, inline_record=10892; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image::closure$2; module=1667, inline_record=11092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::srgb_eotf
// 信息 ID: f7e740ed03c0487bde2e74a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image; module=1667, inline_record=5284; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image; module=1667, inline_record=5352; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture_processing::process_regular_image; module=1667, inline_record=5416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::srgb_oetf
// 信息 ID: ad225a9628e857fe5d1b7b03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> > @ 0x2594438 -> 0x17d6d60 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> > @ 0x259444b -> 0x17d6d60 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> > @ 0x259445f -> 0x17d6d60 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> > @ 0x2594ba8 -> 0x17d6d60 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> > @ 0x2594bbb -> 0x17d6d60 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> > @ 0x2594bcf -> 0x17d6d60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$1,tuple$<> >
// 信息 ID: 9aa46f34477177137563bd6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] asset_pipe::texture_processing::with_rgba8 -> ["asset_pipe::texture_processing::process_regular_image::closure_env$1", "tuple$<>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture_processing::process_regular_image @ 0x17d31aa -> 0x2594b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::with_rgba8<asset_pipe::texture_processing::process_regular_image::closure_env$3,tuple$<> >
// 信息 ID: 6e987304e144d2e54394d6b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] asset_pipe::texture_processing::with_rgba8 -> ["asset_pipe::texture_processing::process_regular_image::closure_env$3", "tuple$<>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture_processing::process_regular_image @ 0x17d3875 -> 0x25943c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
