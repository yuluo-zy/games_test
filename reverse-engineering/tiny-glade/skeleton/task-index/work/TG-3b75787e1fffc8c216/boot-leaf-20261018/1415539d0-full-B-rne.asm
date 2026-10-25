1415539d0: 55 push rbp
1415539d1: 4157 push r15
1415539d3: 4156 push r14
1415539d5: 4155 push r13
1415539d7: 4154 push r12
1415539d9: 56 push rsi
1415539da: 57 push rdi
1415539db: 53 push rbx
1415539dc: 4881ece8020000 sub rsp, 0x2e8
1415539e3: 488dac2480000000 lea rbp, [rsp + 0x80]
1415539eb: 440f29bd50020000 movaps xmmword ptr [rbp + 0x250], xmm15
1415539f3: 440f29b540020000 movaps xmmword ptr [rbp + 0x240], xmm14
1415539fb: 440f29ad30020000 movaps xmmword ptr [rbp + 0x230], xmm13
141553a03: 440f29a520020000 movaps xmmword ptr [rbp + 0x220], xmm12
141553a0b: 440f299d10020000 movaps xmmword ptr [rbp + 0x210], xmm11
141553a13: 440f299500020000 movaps xmmword ptr [rbp + 0x200], xmm10
141553a1b: 440f298df0010000 movaps xmmword ptr [rbp + 0x1f0], xmm9
141553a23: 440f2985e0010000 movaps xmmword ptr [rbp + 0x1e0], xmm8
141553a2b: 0f29bdd0010000 movaps xmmword ptr [rbp + 0x1d0], xmm7
141553a32: 0f29b5c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm6
141553a39: 48c785b8010000feffffff mov qword ptr [rbp + 0x1b8], 0xfffffffffffffffe
141553a44: 4c894508 mov qword ptr [rbp + 8], r8
141553a48: 48895510 mov qword ptr [rbp + 0x10], rdx
141553a4c: 48894d18 mov qword ptr [rbp + 0x18], rcx
141553a50: 48c7859801000000000000 mov qword ptr [rbp + 0x198], 0
141553a5b: 48c785a001000004000000 mov qword ptr [rbp + 0x1a0], 4
141553a66: 48c785a801000000000000 mov qword ptr [rbp + 0x1a8], 0
141553a71: 488d05ae8e7101 lea rax, [rip + 0x1718eae] ; @142c6c926 'trees/rocks.jsontrees/clearing_edge_rocks.jsontrees/stumps.jsontrees/clearing_edge_tree_stumps.j'
141553a78: 488985c8000000 mov qword ptr [rbp + 0xc8], rax
141553a7f: 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
141553a8a: c685d800000000 mov byte ptr [rbp + 0xd8], 0
141553a91: f20f1005f7d36c01 movsd xmm0, qword ptr [rip + 0x16cd3f7]
141553a99: f20f1185dc000000 movsd qword ptr [rbp + 0xdc], xmm0
141553aa1: 488d058e8e7101 lea rax, [rip + 0x1718e8e] ; @142c6c936 'trees/clearing_edge_rocks.jsontrees/stumps.jsontrees/clearing_edge_tree_stumps.jsontrees/logs.js'
141553aa8: 488985e8000000 mov qword ptr [rbp + 0xe8], rax
141553aaf: 48c785f00000001e000000 mov qword ptr [rbp + 0xf0], 0x1e
141553aba: c685f800000000 mov byte ptr [rbp + 0xf8], 0
141553ac1: f20f1185fc000000 movsd qword ptr [rbp + 0xfc], xmm0
141553ac9: 488d05848e7101 lea rax, [rip + 0x1718e84] ; @142c6c954 'trees/stumps.jsontrees/clearing_edge_tree_stumps.jsontrees/logs.jsontrees/clearing_edge_tree_log'
141553ad0: 48898508010000 mov qword ptr [rbp + 0x108], rax
141553ad7: 48c7851001000011000000 mov qword ptr [rbp + 0x110], 0x11
141553ae2: c6851801000002 mov byte ptr [rbp + 0x118], 2
141553ae9: f20f10051f867101 movsd xmm0, qword ptr [rip + 0x171861f]
141553af1: f20f11851c010000 movsd qword ptr [rbp + 0x11c], xmm0
141553af9: 488d05658e7101 lea rax, [rip + 0x1718e65] ; @142c6c965 'trees/clearing_edge_tree_stumps.jsontrees/logs.jsontrees/clearing_edge_tree_logs.jsonassets/mesh'
141553b00: 48898528010000 mov qword ptr [rbp + 0x128], rax
141553b07: 48c7853001000024000000 mov qword ptr [rbp + 0x130], 0x24
141553b12: c6853801000002 mov byte ptr [rbp + 0x138], 2
141553b19: f20f11853c010000 movsd qword ptr [rbp + 0x13c], xmm0
141553b21: 488d05618e7101 lea rax, [rip + 0x1718e61] ; @142c6c989 'trees/logs.jsontrees/clearing_edge_tree_logs.jsonassets/meshes/clover.jsoncrates/country-core/sr'
141553b28: 48898548010000 mov qword ptr [rbp + 0x148], rax
141553b2f: 48c785500100000f000000 mov qword ptr [rbp + 0x150], 0xf
141553b3a: c6855801000001 mov byte ptr [rbp + 0x158], 1
141553b41: f20f1005d7857101 movsd xmm0, qword ptr [rip + 0x17185d7]
141553b49: f20f11855c010000 movsd qword ptr [rbp + 0x15c], xmm0
141553b51: 488d05408e7101 lea rax, [rip + 0x1718e40] ; @142c6c998 'trees/clearing_edge_tree_logs.jsonassets/meshes/clover.jsoncrates/country-core/src/startup/glade'
141553b58: 48898568010000 mov qword ptr [rbp + 0x168], rax
141553b5f: 48c7857001000022000000 mov qword ptr [rbp + 0x170], 0x22
141553b6a: c6857801000001 mov byte ptr [rbp + 0x178], 1
141553b71: f20f11857c010000 movsd qword ptr [rbp + 0x17c], xmm0
141553b79: 48c7859001000006000000 mov qword ptr [rbp + 0x190], 6
141553b84: 410fb64158 movzx eax, byte ptr [r9 + 0x58]
141553b89: 48894500 mov qword ptr [rbp], rax
141553b8d: 31c0 xor eax, eax
141553b8f: 4c8d7520 lea r14, [rbp + 0x20]
141553b93: 450f57c9 xorps xmm9, xmm9
141553b97: f3440f1005641d3d01 movss xmm8, dword ptr [rip + 0x13d1d64]
141553ba0: 450f57db xorps xmm11, xmm11
141553ba4: 488d3dfd8e7101 lea rdi, [rip + 0x1718efd]
141553bab: 488d75c8 lea rsi, [rbp - 0x38]
141553baf: eb12 jmp 0x141553bc3
141553bb1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141553bc0: 4889d8 mov rax, rbx
141553bc3: 4883f806 cmp rax, 6
141553bc7: 0f84eb020000 je 0x141553eb8
141553bcd: 488d5801 lea rbx, [rax + 1]
141553bd1: 48899d88010000 mov qword ptr [rbp + 0x188], rbx
141553bd8: 48c1e005 shl rax, 5
141553bdc: 440fb6a405d8000000 movzx r12d, byte ptr [rbp + rax + 0xd8]
141553be5: 4180fc03 cmp r12b, 3
141553be9: 0f84c9020000 je 0x141553eb8
141553bef: 4c8bbc05c8000000 mov r15, qword ptr [rbp + rax + 0xc8]
141553bf7: 4c8bac05d0000000 mov r13, qword ptr [rbp + rax + 0xd0]
141553bff: f3440f10a405dc000000 movss xmm12, dword ptr [rbp + rax + 0xdc]
141553c09: f3440f10ac05e0000000 movss xmm13, dword ptr [rbp + rax + 0xe0]
141553c13: 488b4500 mov rax, qword ptr [rbp]
141553c17: 488d0daa8f7101 lea rcx, [rip + 0x1718faa]
141553c1e: 4c8b0cc1 mov r9, qword ptr [rcx + rax*8]
141553c22: 488d0dc78f7101 lea rcx, [rip + 0x1718fc7]
141553c29: 488b04c1 mov rax, qword ptr [rcx + rax*8]
141553c2d: 4889442420 mov qword ptr [rsp + 0x20], rax
141553c32: 41b80c000000 mov r8d, 0xc
141553c38: 4c89f1 mov rcx, r14
141553c3b: 488d15d8b85901 lea rdx, [rip + 0x159b8d8] ; @142aef51a 'assets/gladestatecrates/country-core/src/resources/session.rsloading_background.jpg'
141553c42: e8c9eb80ff call 0x140d62810 ; std::path::Path::_join
141553c47: 4c89f1 mov rcx, r14
141553c4a: e8813282ff call 0x140d76ed0 ; std::sys::os_str::wtf8::Buf::as_mut_slice | alloc::boxed::convert::impl$19::from::impl$0::description
141553c4f: 4989d0 mov r8, rdx
141553c52: 4c896c2420 mov qword ptr [rsp + 0x20], r13
141553c57: 488d8da0000000 lea rcx, [rbp + 0xa0]
141553c5e: 4889c2 mov rdx, rax
141553c61: 4d89f9 mov r9, r15
141553c64: e8a7eb80ff call 0x140d62810 ; std::path::Path::_join
141553c69: 488d4de0 lea rcx, [rbp - 0x20]
141553c6d: 488b5510 mov rdx, qword ptr [rbp + 0x10]
141553c71: 4c8b4508 mov r8, qword ptr [rbp + 8]
141553c75: 4c8d8da0000000 lea r9, [rbp + 0xa0]
141553c7c: e84f293eff call 0x1409365d0 ; country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary::load<std::path::PathBuf>
141553c81: 488b45e8 mov rax, qword ptr [rbp - 0x18]
141553c85: 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
141553c8c: 488b45f0 mov rax, qword ptr [rbp - 0x10]
141553c90: 4885c0 test rax, rax
141553c93: 0f84d7010000 je 0x141553e70
141553c99: 4c8d3c40 lea r15, [rax + rax*2]
141553c9d: 49c1e704 shl r15, 4
141553ca1: 4c8badb0010000 mov r13, qword ptr [rbp + 0x1b0]
141553ca8: 4d01ef add r15, r13
141553cab: eb42 jmp 0x141553cef
141553cad: 0f1f00 nop dword ptr [rax]
141553cb0: 488b85a0010000 mov rax, qword ptr [rbp + 0x1a0]
141553cb7: 4b8d0cf6 lea rcx, [r14 + r14*8]
141553cbb: 488d0c49 lea rcx, [rcx + rcx*2]
141553cbf: 4c01f1 add rcx, r14
141553cc2: 488b55d8 mov rdx, qword ptr [rbp - 0x28]
141553cc6: 4889540810 mov qword ptr [rax + rcx + 0x10], rdx
141553ccb: 0f1045c8 movups xmm0, xmmword ptr [rbp - 0x38]
141553ccf: 0f110408 movups xmmword ptr [rax + rcx], xmm0
141553cd3: 4488640818 mov byte ptr [rax + rcx + 0x18], r12b
141553cd8: 49ffc6 inc r14
141553cdb: 4c89b5a8010000 mov qword ptr [rbp + 0x1a8], r14
141553ce2: 4983c530 add r13, 0x30
141553ce6: 4d39fd cmp r13, r15
141553ce9: 0f8481010000 je 0x141553e70
141553cef: f3450f107500 movss xmm14, dword ptr [r13]
141553cf5: f3450f107d08 movss xmm15, dword ptr [r13 + 8]
141553cfb: f3410f107d10 movss xmm7, dword ptr [r13 + 0x10]
141553d01: f30f593d671b3d01 mulss xmm7, dword ptr [rip + 0x13d1b67]
141553d09: 0f28c7 movaps xmm0, xmm7
141553d0c: e8bf023d01 call 0x142923fd0 ; 
141553d11: 0f28f0 movaps xmm6, xmm0
141553d14: 0f28c7 movaps xmm0, xmm7
141553d17: e864013d01 call 0x142923e80 ; 
141553d1c: 0f28d6 movaps xmm2, xmm6
141553d1f: f3410f59d1 mulss xmm2, xmm9
141553d24: f3410f104d0c movss xmm1, dword ptr [r13 + 0xc]
141553d2a: f30f118da0000000 movss dword ptr [rbp + 0xa0], xmm1
141553d32: f30f118da4000000 movss dword ptr [rbp + 0xa4], xmm1
141553d3a: f30f118da8000000 movss dword ptr [rbp + 0xa8], xmm1
141553d42: 0f109da4000000 movups xmm3, xmmword ptr [rbp + 0xa4]
141553d49: 0f28ee movaps xmm5, xmm6
141553d4c: 0f14ea unpcklps xmm5, xmm2
141553d4f: 0f28e5 movaps xmm4, xmm5
141553d52: 0f58e5 addps xmm4, xmm5
141553d55: f30f59d4 mulss xmm2, xmm4
141553d59: 0f59ec mulps xmm5, xmm4
141553d5c: f30f16fc movshdup xmm7, xmm4
141553d60: f30f59f7 mulss xmm6, xmm7
141553d64: f30f59f8 mulss xmm7, xmm0
141553d68: f30f59e0 mulss xmm4, xmm0
141553d6c: f30f16c5 movshdup xmm0, xmm5
141553d70: f30f58e8 addss xmm5, xmm0
141553d74: 450f28d0 movaps xmm10, xmm8
141553d78: f3440f5cd5 subss xmm10, xmm5
141553d7d: 0f28e8 movaps xmm5, xmm0
141553d80: f30f5cec subss xmm5, xmm4
141553d84: f30f5cd7 subss xmm2, xmm7
141553d88: f30f58fe addss xmm7, xmm6
141553d8c: 0f16d7 movlhps xmm2, xmm7
141553d8f: f30f58e0 addss xmm4, xmm0
141553d93: 410f16e2 movlhps xmm4, xmm10
141553d97: 440f16d5 movlhps xmm10, xmm5
141553d9b: f20f1285a0000000 movddup xmm0, qword ptr [rbp + 0xa0]
141553da3: 410f59c2 mulps xmm0, xmm10
141553da7: f20f12ada4000000 movddup xmm5, qword ptr [rbp + 0xa4]
141553daf: 0f59ea mulps xmm5, xmm2
141553db2: f30f16d3 movshdup xmm2, xmm3
141553db6: 0f28f3 movaps xmm6, xmm3
141553db9: 0fc6f355 shufps xmm6, xmm3, 0x55
141553dbd: 0f59f4 mulps xmm6, xmm4
141553dc0: 410f28de movaps xmm3, xmm14
141553dc4: 410f16df movlhps xmm3, xmm15
141553dc8: 410f59eb mulps xmm5, xmm11
141553dcc: 0f58e8 addps xmm5, xmm0
141553dcf: 410f59f3 mulps xmm6, xmm11
141553dd3: 0f58f5 addps xmm6, xmm5
141553dd6: 410f59db mulps xmm3, xmm11
141553dda: 0f58de addps xmm3, xmm6
141553ddd: 0f28c3 movaps xmm0, xmm3
141553de0: 660f15c3 unpckhpd xmm0, xmm3
141553de4: 0f28e3 movaps xmm4, xmm3
141553de7: f30f59e3 mulss xmm4, xmm3
141553deb: 0f28e8 movaps xmm5, xmm0
141553dee: f30f59e8 mulss xmm5, xmm0
141553df2: f30f58ec addss xmm5, xmm4
141553df6: 0f57e4 xorps xmm4, xmm4
141553df9: f30f51e5 sqrtss xmm4, xmm5
141553dfd: 410f28e8 movaps xmm5, xmm8
141553e01: f30f5eec divss xmm5, xmm4
141553e05: f30f59dd mulss xmm3, xmm5
141553e09: f30f59e8 mulss xmm5, xmm0
141553e0d: f3410f59d5 mulss xmm2, xmm13
141553e12: f3410f59cc mulss xmm1, xmm12
141553e17: 48897c2438 mov qword ptr [rsp + 0x38], rdi
141553e1c: f3440f117c2430 movss dword ptr [rsp + 0x30], xmm15
141553e23: f3440f11742428 movss dword ptr [rsp + 0x28], xmm14
141553e2a: f30f116c2420 movss dword ptr [rsp + 0x20], xmm5
141553e30: 4889f1 mov rcx, rsi
141553e33: e8484b74ff call 0x140c98980 ; utils::geometry::rectangle::Rectangle2d::new
141553e38: 4c8bb5a8010000 mov r14, qword ptr [rbp + 0x1a8]
141553e3f: 4c3bb598010000 cmp r14, qword ptr [rbp + 0x198]
141553e46: 0f8564feffff jne 0x141553cb0
141553e4c: 488d8d98010000 lea rcx, [rbp + 0x198]
141553e53: 488d15668c7101 lea rdx, [rip + 0x1718c66]
141553e5a: e8218d23ff call 0x14078cb80 ; alloc::raw_vec::RawVec::grow_one<system_wall_constructor::place_pillars::PillarPlacementProposal,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::decorator::DecoratorId,system_decorator::decorator_visual::lantern::GpuLightId,country_core::resources::color_id::ColorId,f32,glam::f32::vec3::Vec3>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<country_core::systems::camera_rig::normal::ActiveRotation,alloc::alloc::Global>
141553e5f: e94cfeffff jmp 0x141553cb0
141553e64: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141553e70: 488b45e0 mov rax, qword ptr [rbp - 0x20]
141553e74: 4885c0 test rax, rax
141553e77: 741a je 0x141553e93
141553e79: 48c1e004 shl rax, 4
141553e7d: 488d1440 lea rdx, [rax + rax*2]
141553e81: 41b804000000 mov r8d, 4
141553e87: 488b8db0010000 mov rcx, qword ptr [rbp + 0x1b0]
141553e8e: e88dfd0bff call 0x140613c20 ; 
141553e93: 488b5520 mov rdx, qword ptr [rbp + 0x20]
141553e97: 4885d2 test rdx, rdx
141553e9a: 4c8d7520 lea r14, [rbp + 0x20]
141553e9e: 0f841cfdffff je 0x141553bc0
141553ea4: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
141553ea8: 41b801000000 mov r8d, 1
141553eae: e86dfd0bff call 0x140613c20 ; 
141553eb3: e908fdffff jmp 0x141553bc0
141553eb8: 488d15fb8a7101 lea rdx, [rip + 0x1718afb]
141553ebf: 488d8dc8000000 lea rcx, [rbp + 0xc8]
141553ec6: 41b819000000 mov r8d, 0x19
141553ecc: e8dfaf5bff call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
141553ed1: 31c0 xor eax, eax
141553ed3: 483b85c8000000 cmp rax, qword ptr [rbp + 0xc8]
141553eda: 0f80da030000 jo 0x1415542ba
141553ee0: 488b8538010000 mov rax, qword ptr [rbp + 0x138]
141553ee7: 48898590000000 mov qword ptr [rbp + 0x90], rax
141553eee: 0f108528010000 movups xmm0, xmmword ptr [rbp + 0x128]
141553ef5: 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
141553efc: 0f108518010000 movups xmm0, xmmword ptr [rbp + 0x118]
141553f03: 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
141553f07: 0f108508010000 movups xmm0, xmmword ptr [rbp + 0x108]
141553f0e: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
141553f12: 0f1085c8000000 movups xmm0, xmmword ptr [rbp + 0xc8]
141553f19: 0f108dd8000000 movups xmm1, xmmword ptr [rbp + 0xd8]
141553f20: 0f1095e8000000 movups xmm2, xmmword ptr [rbp + 0xe8]
141553f27: 0f109df8000000 movups xmm3, xmmword ptr [rbp + 0xf8]
141553f2e: 0f295d50 movaps xmmword ptr [rbp + 0x50], xmm3
141553f32: 0f295540 movaps xmmword ptr [rbp + 0x40], xmm2
141553f36: 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
141553f3a: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
141553f3e: 488b85e0020000 mov rax, qword ptr [rbp + 0x2e0]
141553f45: 4889442420 mov qword ptr [rsp + 0x20], rax
141553f4a: 488d15cf8a7101 lea rdx, [rip + 0x1718acf] ; @142c6ca20 'clover'
141553f51: 488d4d20 lea rcx, [rbp + 0x20]
141553f55: 41b806000000 mov r8d, 6
141553f5b: 4c8b8dd8020000 mov r9, qword ptr [rbp + 0x2d8]
141553f62: e8396d5cff call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
141553f67: 4889c6 mov rsi, rax
141553f6a: 488d1582897101 lea rdx, [rip + 0x1718982]
141553f71: 41b80d000000 mov r8d, 0xd
141553f77: 488b8de8020000 mov rcx, qword ptr [rbp + 0x2e8]
141553f7e: e81d3b3eff call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
141553f83: a801 test al, 1
141553f85: 0f8464030000 je 0x1415542ef
141553f8b: 4889d7 mov rdi, rdx
141553f8e: 488d8dc8000000 lea rcx, [rbp + 0xc8]
141553f95: 488b95d0020000 mov rdx, qword ptr [rbp + 0x2d0]
141553f9c: e8ff067cff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
141553fa1: 488b9dd0000000 mov rbx, qword ptr [rbp + 0xd0]
141553fa8: 4885db test rbx, rbx
141553fab: 7507 jne 0x141553fb4
141553fad: 488b9dd8000000 mov rbx, qword ptr [rbp + 0xd8]
141553fb4: 4c8ba5e8000000 mov r12, qword ptr [rbp + 0xe8]
141553fbb: 488b03 mov rax, qword ptr [rbx]
141553fbe: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
141553fc2: 4c29f0 sub rax, r14
141553fc5: 483d97000000 cmp rax, 0x97
141553fcb: 0f869f020000 jbe 0x141554270
141553fd1: 488b4308 mov rax, qword ptr [rbx + 8]
141553fd5: 488d0d2454d8ff lea rcx, [rip - 0x27abdc]
141553fdc: 4a890c30 mov qword ptr [rax + r14], rcx
141553fe0: 0f2835d9a03e01 movaps xmm6, xmmword ptr [rip + 0x13ea0d9]
141553fe7: 420f11743008 movups xmmword ptr [rax + r14 + 8], xmm6
141553fed: 4ac744301800000000 mov qword ptr [rax + r14 + 0x18], 0
141553ff6: 42c744302000000000 mov dword ptr [rax + r14 + 0x20], 0
141553fff: 49bf0000803f0000803f movabs r15, 0x3f8000003f800000
141554009: 4e897c3024 mov qword ptr [rax + r14 + 0x24], r15
14155400e: 42c744302c0000803f mov dword ptr [rax + r14 + 0x2c], 0x3f800000
141554017: 4a89743038 mov qword ptr [rax + r14 + 0x38], rsi
14155401c: 4a897c3040 mov qword ptr [rax + r14 + 0x40], rdi
141554021: 48b94dd1e06c5c4bae23 movabs rcx, 0x23ae4b5c6ce0d14d
14155402b: 4a894c3048 mov qword ptr [rax + r14 + 0x48], rcx
141554030: 48b9f2177273805a1e69 movabs rcx, 0x691e5a80737217f2
14155403a: 4a894c3050 mov qword ptr [rax + r14 + 0x50], rcx
14155403f: 48b901142c352fa8f3fb movabs rcx, 0xfbf3a82f352c1401
141554049: 4a894c3058 mov qword ptr [rax + r14 + 0x58], rcx
14155404e: 42c644306800 mov byte ptr [rax + r14 + 0x68], 0
141554054: 4e89643078 mov qword ptr [rax + r14 + 0x78], r12
141554059: 4c8d25d00c0100 lea r12, [rip + 0x10cd0]
141554060: 4e89a43088000000 mov qword ptr [rax + r14 + 0x88], r12
141554068: 4981c698000000 add r14, 0x98
14155406f: 4c897310 mov qword ptr [rbx + 0x10], r14
141554073: 488d15c6897101 lea rdx, [rip + 0x17189c6] ; @142c6ca40 'assets/meshes/clover_flowers.json'
14155407a: 488d8dc8000000 lea rcx, [rbp + 0xc8]
141554081: 41b821000000 mov r8d, 0x21
141554087: e824ae5bff call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
14155408c: 31c0 xor eax, eax
14155408e: 483b85c8000000 cmp rax, qword ptr [rbp + 0xc8]
141554095: 0f8062020000 jo 0x1415542fd
14155409b: 488b8538010000 mov rax, qword ptr [rbp + 0x138]
1415540a2: 48898590000000 mov qword ptr [rbp + 0x90], rax
1415540a9: 0f108528010000 movups xmm0, xmmword ptr [rbp + 0x128]
1415540b0: 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
1415540b7: 0f108518010000 movups xmm0, xmmword ptr [rbp + 0x118]
1415540be: 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
1415540c2: 0f108508010000 movups xmm0, xmmword ptr [rbp + 0x108]
1415540c9: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
1415540cd: 0f1085c8000000 movups xmm0, xmmword ptr [rbp + 0xc8]
1415540d4: 0f108dd8000000 movups xmm1, xmmword ptr [rbp + 0xd8]
1415540db: 0f1095e8000000 movups xmm2, xmmword ptr [rbp + 0xe8]
1415540e2: 0f109df8000000 movups xmm3, xmmword ptr [rbp + 0xf8]
1415540e9: 0f295d50 movaps xmmword ptr [rbp + 0x50], xmm3
1415540ed: 0f295540 movaps xmmword ptr [rbp + 0x40], xmm2
1415540f1: 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
1415540f5: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
1415540f9: 488b85e0020000 mov rax, qword ptr [rbp + 0x2e0]
141554100: 4889442420 mov qword ptr [rsp + 0x20], rax
141554105: 488d1574897101 lea rdx, [rip + 0x1718974] ; @142c6ca80 'clover_flowers'
14155410c: 488d4d20 lea rcx, [rbp + 0x20]
141554110: 41b80e000000 mov r8d, 0xe
141554116: 4c8b8dd8020000 mov r9, qword ptr [rbp + 0x2d8]
14155411d: e87e6b5cff call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
141554122: 4889c6 mov rsi, rax
141554125: 488d15c7877101 lea rdx, [rip + 0x17187c7]
14155412c: 41b80d000000 mov r8d, 0xd
141554132: 488b8de8020000 mov rcx, qword ptr [rbp + 0x2e8]
141554139: e862393eff call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
14155413e: a801 test al, 1
141554140: 0f84ec010000 je 0x141554332
141554146: 4889d7 mov rdi, rdx
141554149: 488d8dc8000000 lea rcx, [rbp + 0xc8]
141554150: 488b95d0020000 mov rdx, qword ptr [rbp + 0x2d0]
141554157: e844057cff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
14155415c: 488b9dd0000000 mov rbx, qword ptr [rbp + 0xd0]
141554163: 4885db test rbx, rbx
141554166: 7507 jne 0x14155416f
141554168: 488b9dd8000000 mov rbx, qword ptr [rbp + 0xd8]
14155416f: 4c8bade8000000 mov r13, qword ptr [rbp + 0xe8]
141554176: 488b03 mov rax, qword ptr [rbx]
141554179: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
14155417d: 4c29f0 sub rax, r14
141554180: 483d87000000 cmp rax, 0x87
141554186: 0f8609010000 jbe 0x141554295
14155418c: 488b4308 mov rax, qword ptr [rbx + 8]
141554190: 488d0dd950d8ff lea rcx, [rip - 0x27af27]
141554197: 4a890c30 mov qword ptr [rax + r14], rcx
14155419b: 420f11743008 movups xmmword ptr [rax + r14 + 8], xmm6
1415541a1: 4ac744301800000000 mov qword ptr [rax + r14 + 0x18], 0
1415541aa: 42c744302000000000 mov dword ptr [rax + r14 + 0x20], 0
1415541b3: 4e897c3024 mov qword ptr [rax + r14 + 0x24], r15
1415541b8: 42c744302c0000803f mov dword ptr [rax + r14 + 0x2c], 0x3f800000
1415541c1: 4a89743038 mov qword ptr [rax + r14 + 0x38], rsi
1415541c6: 4a897c3040 mov qword ptr [rax + r14 + 0x40], rdi
1415541cb: 48b9d45dcb8b7213b82b movabs rcx, 0x2bb813728bcb5dd4
1415541d5: 4a894c3048 mov qword ptr [rax + r14 + 0x48], rcx
1415541da: 42c644305800 mov byte ptr [rax + r14 + 0x58], 0
1415541e0: 4e896c3068 mov qword ptr [rax + r14 + 0x68], r13
1415541e5: 4e89643078 mov qword ptr [rax + r14 + 0x78], r12
1415541ea: 4981c688000000 add r14, 0x88
1415541f1: 4c897310 mov qword ptr [rbx + 0x10], r14
1415541f5: 488b8da8010000 mov rcx, qword ptr [rbp + 0x1a8]
1415541fc: 488b4518 mov rax, qword ptr [rbp + 0x18]
141554200: 48894810 mov qword ptr [rax + 0x10], rcx
141554204: 0f108598010000 movups xmm0, xmmword ptr [rbp + 0x198]
14155420b: 0f1100 movups xmmword ptr [rax], xmm0
14155420e: 0f28b5c0010000 movaps xmm6, xmmword ptr [rbp + 0x1c0]
141554215: 0f28bdd0010000 movaps xmm7, xmmword ptr [rbp + 0x1d0]
14155421c: 440f2885e0010000 movaps xmm8, xmmword ptr [rbp + 0x1e0]
141554224: 440f288df0010000 movaps xmm9, xmmword ptr [rbp + 0x1f0]
14155422c: 440f289500020000 movaps xmm10, xmmword ptr [rbp + 0x200]
141554234: 440f289d10020000 movaps xmm11, xmmword ptr [rbp + 0x210]
14155423c: 440f28a520020000 movaps xmm12, xmmword ptr [rbp + 0x220]
141554244: 440f28ad30020000 movaps xmm13, xmmword ptr [rbp + 0x230]
14155424c: 440f28b540020000 movaps xmm14, xmmword ptr [rbp + 0x240]
141554254: 440f28bd50020000 movaps xmm15, xmmword ptr [rbp + 0x250]
14155425c: 4881c4e8020000 add rsp, 0x2e8
141554263: 5b pop rbx
141554264: 5f pop rdi
141554265: 5e pop rsi
141554266: 415c pop r12
141554268: 415d pop r13
14155426a: 415e pop r14
14155426c: 415f pop r15
14155426e: 5d pop rbp
14155426f: c3 ret 
141554270: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
141554279: 41b898000000 mov r8d, 0x98
14155427f: 41b901000000 mov r9d, 1
141554285: 4889d9 mov rcx, rbx
141554288: 4c89f2 mov rdx, r14
14155428b: e8d0573501 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
141554290: e93cfdffff jmp 0x141553fd1
141554295: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
14155429e: 41b888000000 mov r8d, 0x88
1415542a4: 41b901000000 mov r9d, 1
1415542aa: 4889d9 mov rcx, rbx
1415542ad: 4c89f2 mov rdx, r14
1415542b0: e8ab573501 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1415542b5: e9d2feffff jmp 0x14155418c
1415542ba: 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
1415542c1: 48894520 mov qword ptr [rbp + 0x20], rax
1415542c5: 488d053c877101 lea rax, [rip + 0x171873c]
1415542cc: 4889442420 mov qword ptr [rsp + 0x20], rax
1415542d1: 488d0de87e7101 lea rcx, [rip + 0x1717ee8] ; @142c6c1c0 'called `Result::unwrap()` on an `Err` value'
1415542d8: 4c8d0dc17e7101 lea r9, [rip + 0x1717ec1]
1415542df: 4c8d4520 lea r8, [rbp + 0x20]
1415542e3: ba2b000000 mov edx, 0x2b
1415542e8: e873543801 call 0x1428d9760 ; core::result::unwrap_failed
1415542ed: eb4f jmp 0x14155433e
1415542ef: 488d0d32877101 lea rcx, [rip + 0x1718732]
1415542f6: e815503801 call 0x1428d9310 ; core::option::unwrap_failed
1415542fb: eb41 jmp 0x14155433e
1415542fd: 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
141554304: 48894520 mov qword ptr [rbp + 0x20], rax
141554308: 488d0559877101 lea rax, [rip + 0x1718759]
14155430f: 4889442420 mov qword ptr [rsp + 0x20], rax
141554314: 488d0da57e7101 lea rcx, [rip + 0x1717ea5] ; @142c6c1c0 'called `Result::unwrap()` on an `Err` value'
14155431b: 4c8d0d7e7e7101 lea r9, [rip + 0x1717e7e]
141554322: 4c8d4520 lea r8, [rbp + 0x20]
141554326: ba2b000000 mov edx, 0x2b
14155432b: e830543801 call 0x1428d9760 ; core::result::unwrap_failed
141554330: eb0c jmp 0x14155433e
141554332: 488d0d57877101 lea rcx, [rip + 0x1718757]
141554339: e8d24f3801 call 0x1428d9310 ; core::option::unwrap_failed
14155433e: 0f0b ud2 
141554340: 4889542410 mov qword ptr [rsp + 0x10], rdx
141554345: 55 push rbp
141554346: 4157 push r15
141554348: 4156 push r14
14155434a: 4155 push r13
14155434c: 4154 push r12
14155434e: 56 push rsi
14155434f: 57 push rdi
141554350: 53 push rbx
141554351: 4881ece8000000 sub rsp, 0xe8
141554358: 488daa80000000 lea rbp, [rdx + 0x80]
14155435f: 440f297c2440 movaps xmmword ptr [rsp + 0x40], xmm15
141554365: 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
14155436b: 440f296c2460 movaps xmmword ptr [rsp + 0x60], xmm13
141554371: 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
141554377: 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
141554380: 440f29942490000000 movaps xmmword ptr [rsp + 0x90], xmm10
141554389: 440f298c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm9
141554392: 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
14155439b: 0f29bc24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm7
1415543a3: 0f29b424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm6
1415543ab: 488b8598010000 mov rax, qword ptr [rbp + 0x198]
1415543b2: 4885c0 test rax, rax
1415543b5: 741d je 0x1415543d4
1415543b7: 488b8da0010000 mov rcx, qword ptr [rbp + 0x1a0]
1415543be: 488d14c0 lea rdx, [rax + rax*8]
1415543c2: 488d1452 lea rdx, [rdx + rdx*2]
1415543c6: 4801c2 add rdx, rax
1415543c9: 41b804000000 mov r8d, 4
1415543cf: e84cf80bff call 0x140613c20 ; 
1415543d4: 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
1415543dc: 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
1415543e4: 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
1415543ed: 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
1415543f6: 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
1415543ff: 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
141554408: 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
14155440e: 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
141554414: 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
14155441a: 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
141554420: 4881c4e8000000 add rsp, 0xe8
141554427: 5b pop rbx
141554428: 5f pop rdi
141554429: 5e pop rsi
14155442a: 415c pop r12
14155442c: 415d pop r13
14155442e: 415e pop r14
141554430: 415f pop r15
141554432: 5d pop rbp
141554433: c3 ret 
141554434: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141554440: 4889542410 mov qword ptr [rsp + 0x10], rdx
141554445: 55 push rbp
141554446: 4157 push r15
141554448: 4156 push r14
14155444a: 4155 push r13
14155444c: 4154 push r12
14155444e: 56 push rsi
14155444f: 57 push rdi
141554450: 53 push rbx
141554451: 4881ece8000000 sub rsp, 0xe8
141554458: 488daa80000000 lea rbp, [rdx + 0x80]
14155445f: 440f297c2440 movaps xmmword ptr [rsp + 0x40], xmm15
141554465: 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
14155446b: 440f296c2460 movaps xmmword ptr [rsp + 0x60], xmm13
141554471: 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
141554477: 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
141554480: 440f29942490000000 movaps xmmword ptr [rsp + 0x90], xmm10
141554489: 440f298c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm9
141554492: 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
14155449b: 0f29bc24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm7
1415544a3: 0f29b424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm6
1415544ab: 488b5520 mov rdx, qword ptr [rbp + 0x20]
1415544af: 4885d2 test rdx, rdx
1415544b2: 740f je 0x1415544c3
1415544b4: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
1415544b8: 41b801000000 mov r8d, 1
1415544be: e85df70bff call 0x140613c20 ; 
1415544c3: 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
1415544cb: 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
1415544d3: 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
1415544dc: 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
1415544e5: 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
1415544ee: 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
1415544f7: 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
1415544fd: 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
141554503: 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
141554509: 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
14155450f: 4881c4e8000000 add rsp, 0xe8
141554516: 5b pop rbx
141554517: 5f pop rdi
141554518: 5e pop rsi
141554519: 415c pop r12
14155451b: 415d pop r13
14155451d: 415e pop r14
14155451f: 415f pop r15
141554521: 5d pop rbp
141554522: c3 ret 
141554523: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141554530: 4889542410 mov qword ptr [rsp + 0x10], rdx
141554535: 55 push rbp
141554536: 4157 push r15
141554538: 4156 push r14
14155453a: 4155 push r13
14155453c: 4154 push r12
14155453e: 56 push rsi
14155453f: 57 push rdi
141554540: 53 push rbx
141554541: 4881ece8000000 sub rsp, 0xe8
141554548: 488daa80000000 lea rbp, [rdx + 0x80]
14155454f: 440f297c2440 movaps xmmword ptr [rsp + 0x40], xmm15
141554555: 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
14155455b: 440f296c2460 movaps xmmword ptr [rsp + 0x60], xmm13
141554561: 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
141554567: 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
141554570: 440f29942490000000 movaps xmmword ptr [rsp + 0x90], xmm10
141554579: 440f298c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm9
141554582: 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
14155458b: 0f29bc24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm7
141554593: 0f29b424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm6
14155459b: 488b45e0 mov rax, qword ptr [rbp - 0x20]
14155459f: 4885c0 test rax, rax
1415545a2: 741a je 0x1415545be
1415545a4: 48c1e004 shl rax, 4
1415545a8: 488d1440 lea rdx, [rax + rax*2]
1415545ac: 41b804000000 mov r8d, 4
1415545b2: 488b8db0010000 mov rcx, qword ptr [rbp + 0x1b0]
1415545b9: e862f60bff call 0x140613c20 ; 
1415545be: 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
1415545c6: 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
1415545ce: 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
1415545d7: 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
1415545e0: 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
1415545e9: 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
1415545f2: 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
1415545f8: 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
1415545fe: 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
141554604: 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
14155460a: 4881c4e8000000 add rsp, 0xe8
141554611: 5b pop rbx
141554612: 5f pop rdi
141554613: 5e pop rsi
141554614: 415c pop r12
141554616: 415d pop r13
141554618: 415e pop r14
14155461a: 415f pop r15
14155461c: 5d pop rbp
14155461d: c3 ret 
14155461e: 6690 nop 
141554620: 4889542410 mov qword ptr [rsp + 0x10], rdx
141554625: 55 push rbp
141554626: 4157 push r15
141554628: 4156 push r14
14155462a: 4155 push r13
14155462c: 4154 push r12
14155462e: 56 push rsi
14155462f: 57 push rdi
141554630: 53 push rbx
141554631: 4881ece8000000 sub rsp, 0xe8
141554638: 488daa80000000 lea rbp, [rdx + 0x80]
14155463f: 440f297c2440 movaps xmmword ptr [rsp + 0x40], xmm15
141554645: 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
14155464b: 440f296c2460 movaps xmmword ptr [rsp + 0x60], xmm13
141554651: 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
141554657: 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
141554660: 440f29942490000000 movaps xmmword ptr [rsp + 0x90], xmm10
141554669: 440f298c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm9
141554672: 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
14155467b: 0f29bc24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm7
141554683: 0f29b424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm6
14155468b: 488d4d20 lea rcx, [rbp + 0x20]
14155468f: e89c857fff call 0x140d4cc30 ; anyhow::error::impl$6::drop
141554694: 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
14155469c: 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
1415546a4: 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
1415546ad: 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
1415546b6: 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
1415546bf: 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
1415546c8: 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
1415546ce: 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
1415546d4: 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
1415546da: 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
1415546e0: 4881c4e8000000 add rsp, 0xe8
1415546e7: 5b pop rbx
1415546e8: 5f pop rdi
1415546e9: 5e pop rsi
1415546ea: 415c pop r12
1415546ec: 415d pop r13
1415546ee: 415e pop r14
1415546f0: 415f pop r15
1415546f2: 5d pop rbp
1415546f3: c3 ret 
1415546f4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141554700: 4889542410 mov qword ptr [rsp + 0x10], rdx
141554705: 55 push rbp
141554706: 4157 push r15
141554708: 4156 push r14
14155470a: 4155 push r13
14155470c: 4154 push r12
14155470e: 56 push rsi
14155470f: 57 push rdi
141554710: 53 push rbx
141554711: 4881ece8000000 sub rsp, 0xe8
141554718: 488daa80000000 lea rbp, [rdx + 0x80]
14155471f: 440f297c2440 movaps xmmword ptr [rsp + 0x40], xmm15
141554725: 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
14155472b: 440f296c2460 movaps xmmword ptr [rsp + 0x60], xmm13
141554731: 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
141554737: 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
141554740: 440f29942490000000 movaps xmmword ptr [rsp + 0x90], xmm10
141554749: 440f298c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm9
141554752: 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
14155475b: 0f29bc24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm7
141554763: 0f29b424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm6
14155476b: 488d4d20 lea rcx, [rbp + 0x20]
14155476f: e8bc847fff call 0x140d4cc30 ; anyhow::error::impl$6::drop
141554774: 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
14155477c: 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
141554784: 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
14155478d: 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
141554796: 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
14155479f: 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
1415547a8: 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
1415547ae: 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
1415547b4: 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
1415547ba: 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
1415547c0: 4881c4e8000000 add rsp, 0xe8
1415547c7: 5b pop rbx
1415547c8: 5f pop rdi
1415547c9: 5e pop rsi
1415547ca: 415c pop r12
1415547cc: 415d pop r13
1415547ce: 415e pop r14
1415547d0: 415f pop r15
1415547d2: 5d pop rbp
1415547d3: c3 ret 
