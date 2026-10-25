140926bf0: push rbp
140926bf1: push r15
140926bf3: push r14
140926bf5: push r13
140926bf7: push r12
140926bf9: push rsi
140926bfa: push rdi
140926bfb: push rbx
140926bfc: sub rsp, 0x198
140926c03: lea rbp, [rsp + 0x80]
140926c0b: movaps xmmword ptr [rbp + 0x100], xmm8
140926c13: movaps xmmword ptr [rbp + 0xf0], xmm7
140926c1a: movaps xmmword ptr [rbp + 0xe0], xmm6
140926c21: mov qword ptr [rbp + 0xd8], 0xfffffffffffffffe
140926c2c: mov rbx, r9
140926c2f: mov rdi, r8
140926c32: mov r14, rdx
140926c35: mov r15, rcx
140926c38: xor ecx, ecx
140926c3a: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
140926c3f: test rax, rax
140926c42: je 0x140927c16
140926c48: movups xmm0, xmmword ptr [rax]
140926c4b: inc qword ptr [rax]
140926c4e: movups xmm7, xmmword ptr [rip + 0x219f87b]
140926c55: movups xmmword ptr [rbp + 0xa8], xmm7
140926c5c: movups xmm8, xmmword ptr [rip + 0x219f85c]
140926c64: movups xmmword ptr [rbp + 0x98], xmm8
140926c6c: movups xmmword ptr [rbp + 0xb8], xmm0
140926c73: mov qword ptr [rbp + 0x60], 0
140926c7b: mov qword ptr [rbp + 0x68], 4
140926c83: mov qword ptr [rbp + 0x70], 0
140926c8b: movabs rax, 0x8000000000000000
140926c95: mov qword ptr [rbp + 0x78], rax
140926c99: lea rsi, [rip + 0x218864a]
140926ca0: mov qword ptr [rbp + 0xc8], rsi
140926ca7: mov qword ptr [rbp + 0xd0], 7
140926cb2: mov dword ptr [rsp + 0x20], 0x5a
140926cba: lea rcx, [rbp + 0x60]
140926cbe: xor edx, edx
140926cc0: mov r8d, 0x5a
140926cc6: mov r9d, 0x5a
140926ccc: call 0x140928100 ; country_core::startup::startup_terrain::grid_gen_cw
140926cd1: mov rcx, qword ptr [rbp + 0x190]
140926cd8: mov r12, qword ptr [rbp + 0x188]
140926cdf: mov rax, qword ptr [rbp + 0xd0]
140926ce6: mov qword ptr [rbp + 0x40], rax
140926cea: movups xmm0, xmmword ptr [rbp + 0xc0]
140926cf1: movaps xmmword ptr [rbp + 0x30], xmm0
140926cf5: movups xmm0, xmmword ptr [rbp + 0xb0]
140926cfc: movaps xmmword ptr [rbp + 0x20], xmm0
140926d00: movups xmm0, xmmword ptr [rbp + 0xa0]
140926d07: movaps xmmword ptr [rbp + 0x10], xmm0
140926d0b: movups xmm0, xmmword ptr [rbp + 0x60]
140926d0f: movups xmm1, xmmword ptr [rbp + 0x70]
140926d13: movups xmm2, xmmword ptr [rbp + 0x80]
140926d1a: movups xmm3, xmmword ptr [rbp + 0x90]
140926d21: movaps xmmword ptr [rbp], xmm3
140926d25: movaps xmmword ptr [rbp - 0x10], xmm2
140926d29: movaps xmmword ptr [rbp - 0x20], xmm1
140926d2d: movaps xmmword ptr [rbp - 0x30], xmm0
140926d31: mov r9, qword ptr [rdi]
140926d34: mov rdi, rcx
140926d37: mov rax, qword ptr [r14]
140926d3a: mov qword ptr [rbp + 0x58], rax
140926d3e: mov qword ptr [rsp + 0x20], rax
140926d43: lea rdx, [rip + 0x21881de]
140926d4a: lea rcx, [rbp - 0x30]
140926d4e: mov r8d, 5
140926d54: mov qword ptr [rbp + 0x50], r9
140926d58: call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
140926d5d: mov qword ptr [rbp - 0x48], rax
140926d61: mov rax, qword ptr [r15]
140926d64: mov qword ptr [rbp + 0x48], rax
140926d68: mov rcx, qword ptr [rax]
140926d6b: add rcx, 0x10
140926d6f: mov rax, qword ptr [r12]
140926d73: mov rbx, qword ptr [rbx]
140926d76: mov qword ptr [rsp + 0x30], rbx
140926d7b: mov qword ptr [rbp - 0x38], rax
140926d7f: mov qword ptr [rsp + 0x28], rax
140926d84: mov qword ptr [rsp + 0x20], 0xc
140926d8d: lea rdx, [rip + 0x2188199]
140926d94: lea r9, [rip + 0x21881b4]
140926d9b: mov r8d, 0x22
140926da1: call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
140926da6: mov r15, rax
140926da9: lea rcx, [rbp + 0x60]
140926dad: mov rdx, rdi
140926db0: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
140926db5: mov r12, qword ptr [rbp + 0x68]
140926db9: test r12, r12
140926dbc: jne 0x140926dc2
140926dbe: mov r12, qword ptr [rbp + 0x70]
140926dc2: mov rdx, qword ptr [rbp + 0x80]
140926dc9: mov rax, qword ptr [r12]
140926dcd: mov r13, qword ptr [r12 + 0x10]
140926dd2: sub rax, r13
140926dd5: cmp rax, 0x77
140926dd9: jbe 0x140927a64
140926ddf: mov rax, qword ptr [r12 + 8]
140926de4: lea r14, [rip + 0x9b0e85]
140926deb: mov qword ptr [rax + r13], r14
140926def: movaps xmm6, xmmword ptr [rip + 0x20172ca]
140926df6: movups xmmword ptr [rax + r13 + 8], xmm6
140926dfc: mov qword ptr [rax + r13 + 0x18], 0
140926e05: mov dword ptr [rax + r13 + 0x20], 0
140926e0e: movabs rcx, 0x3f8000003f800000
140926e18: mov qword ptr [rax + r13 + 0x24], rcx
140926e1d: mov dword ptr [rax + r13 + 0x2c], 0x3f800000
140926e26: mov rcx, qword ptr [rbp - 0x48]
140926e2a: mov qword ptr [rax + r13 + 0x38], rcx
140926e2f: mov qword ptr [rax + r13 + 0x40], r15
140926e34: mov byte ptr [rax + r13 + 0x48], 0
140926e3a: mov qword ptr [rbp - 0x40], rdx
140926e3e: mov qword ptr [rax + r13 + 0x58], rdx
140926e43: lea rcx, [rip + 0xc3dee6]
140926e4a: mov qword ptr [rax + r13 + 0x68], rcx
140926e4f: add r13, 0x78
140926e53: mov qword ptr [r12 + 0x10], r13
140926e58: cmp byte ptr [rip + 0x2d0df31], 0
140926e5f: je 0x14092704b
140926e65: xor ecx, ecx
140926e67: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
140926e6c: test rax, rax
140926e6f: je 0x140927c16
140926e75: movups xmm0, xmmword ptr [rax]
140926e78: inc qword ptr [rax]
140926e7b: movups xmmword ptr [rbp + 0xa8], xmm7
140926e82: movups xmmword ptr [rbp + 0x98], xmm8
140926e8a: movups xmmword ptr [rbp + 0xb8], xmm0
140926e91: mov qword ptr [rbp + 0x60], 0
140926e99: mov qword ptr [rbp + 0x68], 4
140926ea1: mov qword ptr [rbp + 0x70], 0
140926ea9: movabs rax, 0x8000000000000000
140926eb3: mov qword ptr [rbp + 0x78], rax
140926eb7: mov qword ptr [rbp + 0xc8], rsi
140926ebe: mov qword ptr [rbp + 0xd0], 7
140926ec9: mov dword ptr [rsp + 0x20], 0x2d
140926ed1: lea rcx, [rbp + 0x60]
140926ed5: xor edx, edx
140926ed7: mov r8d, 0x2d
140926edd: mov r9d, 0x2d
140926ee3: call 0x140928100 ; country_core::startup::startup_terrain::grid_gen_cw
140926ee8: mov rax, qword ptr [rbp + 0xd0]
140926eef: mov qword ptr [rbp + 0x40], rax
140926ef3: movups xmm0, xmmword ptr [rbp + 0xc0]
140926efa: movaps xmmword ptr [rbp + 0x30], xmm0
140926efe: movups xmm0, xmmword ptr [rbp + 0xb0]
140926f05: movaps xmmword ptr [rbp + 0x20], xmm0
140926f09: movups xmm0, xmmword ptr [rbp + 0xa0]
140926f10: movaps xmmword ptr [rbp + 0x10], xmm0
140926f14: movups xmm0, xmmword ptr [rbp + 0x60]
140926f18: movups xmm1, xmmword ptr [rbp + 0x70]
140926f1c: movups xmm2, xmmword ptr [rbp + 0x80]
140926f23: movups xmm3, xmmword ptr [rbp + 0x90]
140926f2a: movaps xmmword ptr [rbp], xmm3
140926f2e: movaps xmmword ptr [rbp - 0x10], xmm2
140926f32: movaps xmmword ptr [rbp - 0x20], xmm1
140926f36: movaps xmmword ptr [rbp - 0x30], xmm0
140926f3a: mov rax, qword ptr [rbp + 0x58]
140926f3e: mov qword ptr [rsp + 0x20], rax
140926f43: lea rdx, [rip + 0x2188043]
140926f4a: lea rcx, [rbp - 0x30]
140926f4e: mov r8d, 0xf
140926f54: mov r9, qword ptr [rbp + 0x50]
140926f58: call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
140926f5d: mov r15, rax
140926f60: mov rax, qword ptr [rbp + 0x48]
140926f64: mov rcx, qword ptr [rax]
140926f67: add rcx, 0x10
140926f6b: mov qword ptr [rsp + 0x30], rbx
140926f70: mov rax, qword ptr [rbp - 0x38]
140926f74: mov qword ptr [rsp + 0x28], rax
140926f79: mov qword ptr [rsp + 0x20], 0x14
140926f82: lea rdx, [rip + 0x2188013]
140926f89: lea r9, [rip + 0x218802d]
140926f90: mov r8d, 0x21
140926f96: call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
140926f9b: mov r12, rax
140926f9e: lea rcx, [rbp + 0x60]
140926fa2: mov rdx, rdi
140926fa5: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
140926faa: mov r13, qword ptr [rbp + 0x68]
140926fae: test r13, r13
140926fb1: jne 0x140926fb7
140926fb3: mov r13, qword ptr [rbp + 0x70]
140926fb7: mov r14, qword ptr [rbp + 0x80]
140926fbe: mov rax, qword ptr [r13]
140926fc2: mov rsi, qword ptr [r13 + 0x10]
140926fc6: sub rax, rsi
140926fc9: cmp rax, 0x87
140926fcf: jbe 0x140927bf1
140926fd5: mov rax, qword ptr [r13 + 8]
140926fd9: lea rcx, [rip + 0x9b3460]
140926fe0: mov qword ptr [rax + rsi], rcx
140926fe4: movups xmmword ptr [rax + rsi + 8], xmm6
140926fe9: mov qword ptr [rax + rsi + 0x18], 0
140926ff2: mov dword ptr [rax + rsi + 0x20], 0
140926ffa: movabs rcx, 0x3f8000003f800000
140927004: mov qword ptr [rax + rsi + 0x24], rcx
140927009: mov dword ptr [rax + rsi + 0x2c], 0x3f800000
140927011: mov qword ptr [rax + rsi + 0x38], r15
140927016: mov qword ptr [rax + rsi + 0x40], r12
14092701b: mov dword ptr [rax + rsi + 0x48], 0x10
140927023: mov byte ptr [rax + rsi + 0x58], 0
140927028: mov qword ptr [rax + rsi + 0x68], r14
14092702d: lea rcx, [rip + 0xc3dcfc]
140927034: mov qword ptr [rax + rsi + 0x78], rcx
140927039: add rsi, 0x88
140927040: mov qword ptr [r13 + 0x10], rsi
140927044: lea r14, [rip + 0x9b0c25]
14092704b: mov rax, qword ptr [rbp + 0x180]
140927052: mov rcx, qword ptr [rbp + 0x48]
140927056: mov r8, qword ptr [rcx]
140927059: add r8, 0x10
14092705d: mov r9, qword ptr [rax]
140927060: lea rcx, [rip + 0x2187f6a]
140927067: mov edx, 0x1e
14092706c: call 0x1408bbf50 ; country_core::resources::rocky_terrain::RockyTerrainMesh::load
140927071: test al, 1
140927073: jne 0x140927c22
140927079: mov rsi, qword ptr [rdi + 8]
14092707d: test rsi, rsi
140927080: jne 0x140927086
140927082: mov rsi, qword ptr [rdi + 0x10]
140927086: mov rax, qword ptr [rsi]
140927089: mov r15, qword ptr [rsi + 0x10]
14092708d: sub rax, r15
140927090: cmp rax, 0xf
140927094: jbe 0x140927a8f
14092709a: mov rax, qword ptr [rsi + 8]
14092709e: lea rcx, [rip + 0x9b09fb]
1409270a5: mov qword ptr [rax + r15], rcx
1409270a9: mov qword ptr [rax + r15 + 8], rdx
1409270ae: add r15, 0x10
1409270b2: mov qword ptr [rsi + 0x10], r15
1409270b6: mov rax, qword ptr [rbp + 0x48]
1409270ba: mov rcx, qword ptr [rax]
1409270bd: add rcx, 0x10
1409270c1: mov qword ptr [rsp + 0x30], rbx
1409270c6: mov rax, qword ptr [rbp - 0x38]
1409270ca: mov qword ptr [rsp + 0x28], rax
1409270cf: mov qword ptr [rsp + 0x20], 0xd
1409270d8: lea rdx, [rip + 0x2187f29]
1409270df: lea r9, [rip + 0x2187f4a]
1409270e6: mov r8d, 0x28
1409270ec: call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
1409270f1: mov r15, rax
1409270f4: lea rcx, [rbp + 0x60]
1409270f8: mov rdx, rdi
1409270fb: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
140927100: mov r12, qword ptr [rbp + 0x68]
140927104: test r12, r12
140927107: jne 0x14092710d
140927109: mov r12, qword ptr [rbp + 0x70]
14092710d: mov r13, qword ptr [rbp + 0x80]
140927114: mov rax, qword ptr [r12]
140927118: mov rsi, qword ptr [r12 + 0x10]
14092711d: sub rax, rsi
140927120: cmp rax, 0x77
140927124: mov rdx, qword ptr [rbp - 0x40]
140927128: jbe 0x140927ac1
14092712e: mov rax, qword ptr [r12 + 8]
140927133: mov qword ptr [rax + rsi], r14
140927137: movups xmmword ptr [rax + rsi + 8], xmm6
14092713c: mov qword ptr [rax + rsi + 0x18], 0
140927145: mov dword ptr [rax + rsi + 0x20], 0
14092714d: movabs rcx, 0x3f8000003f800000
140927157: mov qword ptr [rax + rsi + 0x24], rcx
14092715c: mov dword ptr [rax + rsi + 0x2c], 0x3f800000
140927164: mov rcx, qword ptr [rbp - 0x48]
140927168: mov qword ptr [rax + rsi + 0x38], rcx
14092716d: mov qword ptr [rax + rsi + 0x40], r15
140927172: mov byte ptr [rax + rsi + 0x48], 0
140927177: mov qword ptr [rax + rsi + 0x58], r13
14092717c: lea rcx, [rip + 0xc3dbad]
140927183: mov qword ptr [rax + rsi + 0x68], rcx
140927188: add rsi, 0x78
14092718c: mov qword ptr [r12 + 0x10], rsi
140927191: mov rsi, qword ptr [rdi + 8]
140927195: test rsi, rsi
140927198: jne 0x14092719e
14092719a: mov rsi, qword ptr [rdi + 0x10]
14092719e: mov rax, qword ptr [rsi]
1409271a1: mov r14, qword ptr [rsi + 0x10]
1409271a5: sub rax, r14
1409271a8: cmp rax, 0x17
1409271ac: jbe 0x140927aea
1409271b2: mov rax, qword ptr [rsi + 8]
1409271b6: lea rcx, [rip + 0x9b1943]
1409271bd: mov qword ptr [rax + r14], rcx
1409271c1: mov qword ptr [rax + r14 + 8], rdx
1409271c6: mov qword ptr [rax + r14 + 0x10], r13
1409271cb: add r14, 0x18
1409271cf: mov qword ptr [rsi + 0x10], r14
1409271d3: lea rdx, [rip + 0x2187e63]
1409271da: lea rcx, [rbp + 0x60]
1409271de: mov r8d, 0x1e
1409271e4: call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
1409271e9: xor eax, eax
1409271eb: cmp rax, qword ptr [rbp + 0x60]
1409271ef: jo 0x140927c53
1409271f5: mov rax, qword ptr [rbp + 0xd0]
1409271fc: mov qword ptr [rbp + 0x40], rax
140927200: movups xmm0, xmmword ptr [rbp + 0xc0]
140927207: movaps xmmword ptr [rbp + 0x30], xmm0
14092720b: movups xmm0, xmmword ptr [rbp + 0xb0]
140927212: movaps xmmword ptr [rbp + 0x20], xmm0
140927216: movups xmm0, xmmword ptr [rbp + 0xa0]
14092721d: movaps xmmword ptr [rbp + 0x10], xmm0
140927221: movups xmm0, xmmword ptr [rbp + 0x60]
140927225: movups xmm1, xmmword ptr [rbp + 0x70]
140927229: movups xmm2, xmmword ptr [rbp + 0x80]
140927230: movups xmm3, xmmword ptr [rbp + 0x90]
140927237: movaps xmmword ptr [rbp], xmm3
14092723b: movaps xmmword ptr [rbp - 0x10], xmm2
14092723f: movaps xmmword ptr [rbp - 0x20], xmm1
140927243: movaps xmmword ptr [rbp - 0x30], xmm0
140927247: mov rax, qword ptr [rbp + 0x58]
14092724b: mov qword ptr [rsp + 0x20], rax
140927250: lea rdx, [rip + 0x2187e21]
140927257: lea rcx, [rbp - 0x30]
14092725b: mov r8d, 0x10
140927261: mov r9, qword ptr [rbp + 0x50]
140927265: call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
14092726a: mov r15, rax
14092726d: mov rax, qword ptr [rbp + 0x48]
140927271: mov rcx, qword ptr [rax]
140927274: add rcx, 0x10
140927278: mov qword ptr [rsp + 0x30], rbx
14092727d: mov rax, qword ptr [rbp - 0x38]
140927281: mov qword ptr [rsp + 0x28], rax
140927286: mov qword ptr [rsp + 0x20], 0xb
14092728f: lea rdx, [rip + 0x2187df2]
140927296: lea r9, [rip + 0x2187e1c]
14092729d: mov r8d, 0x31
1409272a3: call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
1409272a8: mov r14, rax
1409272ab: lea rcx, [rbp + 0x60]
1409272af: mov rdx, rdi
1409272b2: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1409272b7: mov r12, qword ptr [rbp + 0x68]
1409272bb: test r12, r12
1409272be: jne 0x1409272c4
1409272c0: mov r12, qword ptr [rbp + 0x70]
1409272c4: mov r13, qword ptr [rbp + 0x80]
1409272cb: mov rax, qword ptr [r12]
1409272cf: mov rsi, qword ptr [r12 + 0x10]
1409272d4: sub rax, rsi
1409272d7: cmp rax, 0x87
1409272dd: jbe 0x140927b13
1409272e3: mov rax, qword ptr [r12 + 8]
1409272e8: lea rcx, [rip + 0x9b5701]
1409272ef: mov qword ptr [rax + rsi], rcx
1409272f3: movups xmmword ptr [rax + rsi + 8], xmm6
1409272f8: mov qword ptr [rax + rsi + 0x18], 0
140927301: mov dword ptr [rax + rsi + 0x20], 0
140927309: movabs rcx, 0x3f8000003f800000
140927313: mov qword ptr [rax + rsi + 0x24], rcx
140927318: mov dword ptr [rax + rsi + 0x2c], 0x3f800000
140927320: mov qword ptr [rax + rsi + 0x38], r15
140927325: mov qword ptr [rax + rsi + 0x40], r14
14092732a: mov byte ptr [rax + rsi + 0x48], 0
14092732f: mov byte ptr [rax + rsi + 0x58], 0
140927334: mov qword ptr [rax + rsi + 0x68], r13
140927339: lea rcx, [rip + 0xc3d9f0]
140927340: mov qword ptr [rax + rsi + 0x78], rcx
140927345: add rsi, 0x88
14092734c: mov qword ptr [r12 + 0x10], rsi
140927351: lea rdx, [rip + 0x2187d6c]
140927358: lea rcx, [rbp + 0x60]
14092735c: mov r8d, 0x1e
140927362: call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
140927367: xor eax, eax
140927369: cmp rax, qword ptr [rbp + 0x60]
14092736d: jo 0x140927c88
140927373: mov rax, qword ptr [rbp + 0xd0]
14092737a: mov qword ptr [rbp + 0x40], rax
14092737e: movups xmm0, xmmword ptr [rbp + 0xc0]
140927385: movaps xmmword ptr [rbp + 0x30], xmm0
140927389: movups xmm0, xmmword ptr [rbp + 0xb0]
140927390: movaps xmmword ptr [rbp + 0x20], xmm0
140927394: movups xmm0, xmmword ptr [rbp + 0xa0]
14092739b: movaps xmmword ptr [rbp + 0x10], xmm0
14092739f: movups xmm0, xmmword ptr [rbp + 0x60]
1409273a3: movups xmm1, xmmword ptr [rbp + 0x70]
1409273a7: movups xmm2, xmmword ptr [rbp + 0x80]
1409273ae: movups xmm3, xmmword ptr [rbp + 0x90]
1409273b5: movaps xmmword ptr [rbp], xmm3
1409273b9: movaps xmmword ptr [rbp - 0x10], xmm2
1409273bd: movaps xmmword ptr [rbp - 0x20], xmm1
1409273c1: movaps xmmword ptr [rbp - 0x30], xmm0
1409273c5: mov rax, qword ptr [rbp + 0x58]
1409273c9: mov qword ptr [rsp + 0x20], rax
1409273ce: lea rdx, [rip + 0x2187d2b]
1409273d5: lea rcx, [rbp - 0x30]
1409273d9: mov r8d, 0x10
1409273df: mov r9, qword ptr [rbp + 0x50]
1409273e3: call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
1409273e8: mov r15, rax
1409273eb: mov rax, qword ptr [rbp + 0x48]
1409273ef: mov rcx, qword ptr [rax]
1409273f2: add rcx, 0x10
1409273f6: mov qword ptr [rsp + 0x30], rbx
1409273fb: mov rax, qword ptr [rbp - 0x38]
1409273ff: mov qword ptr [rsp + 0x28], rax
140927404: mov qword ptr [rsp + 0x20], 0xb
14092740d: lea rdx, [rip + 0x2187cfc]
140927414: lea r9, [rip + 0x2187d0d]
14092741b: mov r8d, 0x18
140927421: call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
140927426: mov r12, rax
140927429: lea rcx, [rbp + 0x60]
14092742d: mov rdx, rdi
140927430: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
140927435: mov r13, qword ptr [rbp + 0x68]
140927439: test r13, r13
14092743c: jne 0x140927442
14092743e: mov r13, qword ptr [rbp + 0x70]
140927442: mov rdi, qword ptr [rbp + 0x80]
140927449: mov rax, qword ptr [r13]
14092744d: mov rsi, qword ptr [r13 + 0x10]
140927451: sub rax, rsi
140927454: cmp rax, 0x77
140927458: jbe 0x140927b38
14092745e: mov rax, qword ptr [r13 + 8]
140927462: lea rcx, [rip + 0x9b0807]
140927469: mov qword ptr [rax + rsi], rcx
14092746d: movups xmmword ptr [rax + rsi + 8], xmm6
140927472: mov qword ptr [rax + rsi + 0x18], 0
14092747b: mov dword ptr [rax + rsi + 0x20], 0
140927483: movabs rcx, 0x3f8000003f800000
14092748d: mov qword ptr [rax + rsi + 0x24], rcx
140927492: mov dword ptr [rax + rsi + 0x2c], 0x3f800000
14092749a: mov qword ptr [rax + rsi + 0x38], r15
14092749f: mov qword ptr [rax + rsi + 0x40], r12
1409274a4: mov byte ptr [rax + rsi + 0x48], 0
1409274a9: mov qword ptr [rax + rsi + 0x58], rdi
1409274ae: lea rcx, [rip + 0xc3d87b]
1409274b5: mov qword ptr [rax + rsi + 0x68], rcx
1409274ba: add rsi, 0x78
1409274be: mov qword ptr [r13 + 0x10], rsi
1409274c2: lea rdx, [rip + 0x2187c6a]
1409274c9: lea rcx, [rbp + 0x60]
1409274cd: mov r8d, 0x20
1409274d3: call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
1409274d8: xor eax, eax
1409274da: cmp rax, qword ptr [rbp + 0x60]
1409274de: jo 0x140927cbd
1409274e4: mov rax, qword ptr [rbp + 0xd0]
1409274eb: mov qword ptr [rbp + 0x40], rax
1409274ef: movups xmm0, xmmword ptr [rbp + 0xc0]
1409274f6: movaps xmmword ptr [rbp + 0x30], xmm0
1409274fa: movups xmm0, xmmword ptr [rbp + 0xb0]
140927501: movaps xmmword ptr [rbp + 0x20], xmm0
140927505: movups xmm0, xmmword ptr [rbp + 0xa0]
14092750c: movaps xmmword ptr [rbp + 0x10], xmm0
140927510: movups xmm0, xmmword ptr [rbp + 0x60]
140927514: movups xmm1, xmmword ptr [rbp + 0x70]
140927518: movups xmm2, xmmword ptr [rbp + 0x80]
14092751f: movups xmm3, xmmword ptr [rbp + 0x90]
140927526: movaps xmmword ptr [rbp], xmm3
14092752a: movaps xmmword ptr [rbp - 0x10], xmm2
14092752e: movaps xmmword ptr [rbp - 0x20], xmm1
140927532: movaps xmmword ptr [rbp - 0x30], xmm0
140927536: mov rax, qword ptr [rbp + 0x58]
14092753a: mov qword ptr [rsp + 0x20], rax
14092753f: lea rdx, [rip + 0x2187c2a]
140927546: lea rcx, [rbp - 0x30]
14092754a: mov r8d, 0x12
140927550: mov r9, qword ptr [rbp + 0x50]
140927554: call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
140927559: mov r15, rax
14092755c: lea rcx, [rbp + 0x60]
140927560: mov rdi, qword ptr [rbp + 0x190]
140927567: mov rdx, rdi
14092756a: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
14092756f: mov r12, qword ptr [rbp + 0x68]
140927573: test r12, r12
140927576: jne 0x14092757c
140927578: mov r12, qword ptr [rbp + 0x70]
14092757c: mov r13, qword ptr [rbp + 0x80]
140927583: mov rax, qword ptr [r12]
140927587: mov rsi, qword ptr [r12 + 0x10]
14092758c: sub rax, rsi
14092758f: cmp rax, 0x87
140927595: jbe 0x140927b5d
14092759b: mov rax, qword ptr [r12 + 8]
1409275a0: lea rcx, [rip + 0x9b5449]
1409275a7: mov qword ptr [rax + rsi], rcx
1409275ab: movups xmmword ptr [rax + rsi + 8], xmm6
1409275b0: mov qword ptr [rax + rsi + 0x18], 0
1409275b9: mov dword ptr [rax + rsi + 0x20], 0
1409275c1: movabs rcx, 0x3f8000003f800000
1409275cb: mov qword ptr [rax + rsi + 0x24], rcx
1409275d0: mov dword ptr [rax + rsi + 0x2c], 0x3f800000
1409275d8: mov qword ptr [rax + rsi + 0x38], r15
1409275dd: mov qword ptr [rax + rsi + 0x40], r14
1409275e2: mov byte ptr [rax + rsi + 0x48], 1
1409275e7: mov byte ptr [rax + rsi + 0x58], 0
1409275ec: mov qword ptr [rax + rsi + 0x68], r13
1409275f1: lea rcx, [rip + 0xc3d738]
1409275f8: mov qword ptr [rax + rsi + 0x78], rcx
1409275fd: add rsi, 0x88
140927604: mov qword ptr [r12 + 0x10], rsi
140927609: lea rdx, [rip + 0x2187b72]
140927610: lea rcx, [rbp + 0x60]
140927614: mov r8d, 0x1a
14092761a: call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
14092761f: xor eax, eax
140927621: cmp rax, qword ptr [rbp + 0x60]
140927625: jo 0x140927cf2
14092762b: mov rax, qword ptr [rbp + 0xd0]
140927632: mov qword ptr [rbp + 0x40], rax
140927636: movups xmm0, xmmword ptr [rbp + 0xc0]
14092763d: movaps xmmword ptr [rbp + 0x30], xmm0
140927641: movups xmm0, xmmword ptr [rbp + 0xb0]
140927648: movaps xmmword ptr [rbp + 0x20], xmm0
14092764c: movups xmm0, xmmword ptr [rbp + 0xa0]
140927653: movaps xmmword ptr [rbp + 0x10], xmm0
140927657: movups xmm0, xmmword ptr [rbp + 0x60]
14092765b: movups xmm1, xmmword ptr [rbp + 0x70]
14092765f: movups xmm2, xmmword ptr [rbp + 0x80]
140927666: movups xmm3, xmmword ptr [rbp + 0x90]
14092766d: movaps xmmword ptr [rbp], xmm3
140927671: movaps xmmword ptr [rbp - 0x10], xmm2
140927675: movaps xmmword ptr [rbp - 0x20], xmm1
140927679: movaps xmmword ptr [rbp - 0x30], xmm0
14092767d: mov rax, qword ptr [rbp + 0x58]
140927681: mov qword ptr [rsp + 0x20], rax
140927686: lea rdx, [rip + 0x2187b2b]
14092768d: lea rcx, [rbp - 0x30]
140927691: mov r8d, 7
140927697: mov r9, qword ptr [rbp + 0x50]
14092769b: call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
1409276a0: mov r14, rax
1409276a3: lea rdx, [rip + 0x2187b15]
1409276aa: mov r8d, 0x1d
1409276b0: mov rcx, rbx
1409276b3: call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
1409276b8: test al, 1
1409276ba: je 0x140927d24
1409276c0: mov r15, rdx
1409276c3: lea rcx, [rbp + 0x60]
1409276c7: mov rdx, rdi
1409276ca: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1409276cf: mov r12, qword ptr [rbp + 0x68]
1409276d3: test r12, r12
1409276d6: jne 0x1409276dc
1409276d8: mov r12, qword ptr [rbp + 0x70]
1409276dc: mov r13, qword ptr [rbp + 0x80]
1409276e3: mov rax, qword ptr [r12]
1409276e7: mov rsi, qword ptr [r12 + 0x10]
1409276ec: sub rax, rsi
1409276ef: cmp rax, 0x87
1409276f5: jbe 0x140927b82
1409276fb: mov rax, qword ptr [r12 + 8]
140927700: lea rcx, [rip + 0x9b43f9]
140927707: mov qword ptr [rax + rsi], rcx
14092770b: movups xmmword ptr [rax + rsi + 8], xmm6
140927710: mov qword ptr [rax + rsi + 0x18], 0
140927719: mov dword ptr [rax + rsi + 0x20], 0
140927721: movabs rcx, 0x3f8000003f800000
14092772b: mov qword ptr [rax + rsi + 0x24], rcx
140927730: mov dword ptr [rax + rsi + 0x2c], 0x3f800000
140927738: mov qword ptr [rax + rsi + 0x38], r14
14092773d: mov qword ptr [rax + rsi + 0x40], r15
140927742: mov dword ptr [rax + rsi + 0x48], 0x3f666666
14092774a: mov byte ptr [rax + rsi + 0x58], 0
14092774f: mov qword ptr [rax + rsi + 0x68], r13
140927754: lea rcx, [rip + 0xc3d5d5]
14092775b: mov qword ptr [rax + rsi + 0x78], rcx
140927760: add rsi, 0x88
140927767: mov qword ptr [r12 + 0x10], rsi
14092776c: lea rdx, [rip + 0x2187a85]
140927773: lea rcx, [rbp + 0x60]
140927777: mov r8d, 0x20
14092777d: call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
140927782: xor eax, eax
140927784: cmp rax, qword ptr [rbp + 0x60]
140927788: jo 0x140927d30
14092778e: mov rax, qword ptr [rbp + 0xd0]
140927795: mov qword ptr [rbp + 0x40], rax
140927799: movups xmm0, xmmword ptr [rbp + 0xc0]
1409277a0: movaps xmmword ptr [rbp + 0x30], xmm0
1409277a4: movups xmm0, xmmword ptr [rbp + 0xb0]
1409277ab: movaps xmmword ptr [rbp + 0x20], xmm0
1409277af: movups xmm0, xmmword ptr [rbp + 0xa0]
1409277b6: movaps xmmword ptr [rbp + 0x10], xmm0
1409277ba: movups xmm0, xmmword ptr [rbp + 0x60]
1409277be: movups xmm1, xmmword ptr [rbp + 0x70]
1409277c2: movups xmm2, xmmword ptr [rbp + 0x80]
1409277c9: movups xmm3, xmmword ptr [rbp + 0x90]
1409277d0: movaps xmmword ptr [rbp], xmm3
1409277d4: movaps xmmword ptr [rbp - 0x10], xmm2
1409277d8: movaps xmmword ptr [rbp - 0x20], xmm1
1409277dc: movaps xmmword ptr [rbp - 0x30], xmm0
1409277e0: mov rax, qword ptr [rbp + 0x58]
1409277e4: mov qword ptr [rsp + 0x20], rax
1409277e9: lea rdx, [rip + 0x2187a40]
1409277f0: lea rcx, [rbp - 0x30]
1409277f4: mov r8d, 0xd
1409277fa: mov r9, qword ptr [rbp + 0x50]
1409277fe: call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
140927803: mov r14, rax
140927806: lea rdx, [rip + 0x2187a30]
14092780d: mov r8d, 0x12
140927813: mov rcx, rbx
140927816: call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
14092781b: test al, 1
14092781d: je 0x140927d62
140927823: mov r15, rdx
140927826: lea rcx, [rbp + 0x60]
14092782a: mov rdx, rdi
14092782d: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
140927832: mov r12, qword ptr [rbp + 0x68]
140927836: test r12, r12
140927839: jne 0x14092783f
14092783b: mov r12, qword ptr [rbp + 0x70]
14092783f: mov r13, qword ptr [rbp + 0x80]
140927846: mov rax, qword ptr [r12]
14092784a: mov rsi, qword ptr [r12 + 0x10]
14092784f: sub rax, rsi
140927852: cmp rax, 0x77
140927856: jbe 0x140927ba7
14092785c: mov rax, qword ptr [r12 + 8]
140927861: lea rcx, [rip + 0x9b1f28]
140927868: mov qword ptr [rax + rsi], rcx
14092786c: movups xmmword ptr [rax + rsi + 8], xmm6
140927871: mov qword ptr [rax + rsi + 0x18], 0
14092787a: mov dword ptr [rax + rsi + 0x20], 0
140927882: movabs rcx, 0x3f8000003f800000
14092788c: mov qword ptr [rax + rsi + 0x24], rcx
140927891: mov dword ptr [rax + rsi + 0x2c], 0x3f800000
140927899: mov qword ptr [rax + rsi + 0x38], r14
14092789e: mov qword ptr [rax + rsi + 0x40], r15
1409278a3: mov byte ptr [rax + rsi + 0x48], 0
1409278a8: mov qword ptr [rax + rsi + 0x58], r13
1409278ad: lea rcx, [rip + 0xc3d47c]
1409278b4: mov qword ptr [rax + rsi + 0x68], rcx
1409278b9: add rsi, 0x78
1409278bd: mov qword ptr [r12 + 0x10], rsi
1409278c2: lea rdx, [rip + 0x218799f]
1409278c9: lea rcx, [rbp + 0x60]
1409278cd: mov r8d, 0x23
1409278d3: call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
1409278d8: xor eax, eax
1409278da: cmp rax, qword ptr [rbp + 0x60]
1409278de: jo 0x140927d6e
1409278e4: mov rax, qword ptr [rbp + 0xd0]
1409278eb: mov qword ptr [rbp + 0x40], rax
1409278ef: movups xmm0, xmmword ptr [rbp + 0xc0]
1409278f6: movaps xmmword ptr [rbp + 0x30], xmm0
1409278fa: movups xmm0, xmmword ptr [rbp + 0xb0]
140927901: movaps xmmword ptr [rbp + 0x20], xmm0
140927905: movups xmm0, xmmword ptr [rbp + 0xa0]
14092790c: movaps xmmword ptr [rbp + 0x10], xmm0
140927910: movups xmm0, xmmword ptr [rbp + 0x60]
140927914: movups xmm1, xmmword ptr [rbp + 0x70]
140927918: movups xmm2, xmmword ptr [rbp + 0x80]
14092791f: movups xmm3, xmmword ptr [rbp + 0x90]
140927926: movaps xmmword ptr [rbp], xmm3
14092792a: movaps xmmword ptr [rbp - 0x10], xmm2
14092792e: movaps xmmword ptr [rbp - 0x20], xmm1
140927932: movaps xmmword ptr [rbp - 0x30], xmm0
140927936: mov rax, qword ptr [rbp + 0x58]
14092793a: mov qword ptr [rsp + 0x20], rax
14092793f: lea rdx, [rip + 0x2187962]
140927946: lea rcx, [rbp - 0x30]
14092794a: mov r8d, 0x10
140927950: mov r9, qword ptr [rbp + 0x50]
140927954: call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
140927959: mov rdi, rax
14092795c: mov rax, qword ptr [rbp + 0x48]
140927960: mov rcx, qword ptr [rax]
140927963: add rcx, 0x10
140927967: mov qword ptr [rsp + 0x30], rbx
14092796c: mov rax, qword ptr [rbp - 0x38]
140927970: mov qword ptr [rsp + 0x28], rax
140927975: mov qword ptr [rsp + 0x20], 0x16
14092797e: lea rdx, [rip + 0x2187933]
140927985: lea r9, [rip + 0x2187948]
14092798c: mov r8d, 0x1c
140927992: call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
140927997: mov rbx, rax
14092799a: lea rcx, [rbp + 0x60]
14092799e: mov rdx, qword ptr [rbp + 0x190]
1409279a5: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1409279aa: mov rsi, qword ptr [rbp + 0x68]
1409279ae: test rsi, rsi
1409279b1: jne 0x1409279b7
1409279b3: mov rsi, qword ptr [rbp + 0x70]
1409279b7: mov r15, qword ptr [rbp + 0x80]
1409279be: mov rax, qword ptr [rsi]
1409279c1: mov r14, qword ptr [rsi + 0x10]
1409279c5: sub rax, r14
1409279c8: cmp rax, 0x77
1409279cc: jbe 0x140927bcc
1409279d2: mov rax, qword ptr [rsi + 8]
1409279d6: lea rcx, [rip + 0x9b38c3]
1409279dd: mov qword ptr [rax + r14], rcx
1409279e1: movups xmmword ptr [rax + r14 + 8], xmm6
1409279e7: mov qword ptr [rax + r14 + 0x18], 0
1409279f0: mov dword ptr [rax + r14 + 0x20], 0
1409279f9: movabs rcx, 0x3f8000003f800000
140927a03: mov qword ptr [rax + r14 + 0x24], rcx
140927a08: mov dword ptr [rax + r14 + 0x2c], 0x3f800000
140927a11: mov qword ptr [rax + r14 + 0x38], rdi
140927a16: mov qword ptr [rax + r14 + 0x40], rbx
140927a1b: mov byte ptr [rax + r14 + 0x48], 0
140927a21: mov qword ptr [rax + r14 + 0x58], r15
140927a26: lea rcx, [rip + 0xc3d303]
140927a2d: mov qword ptr [rax + r14 + 0x68], rcx
140927a32: add r14, 0x78
140927a36: mov qword ptr [rsi + 0x10], r14
140927a3a: movaps xmm6, xmmword ptr [rbp + 0xe0]
140927a41: movaps xmm7, xmmword ptr [rbp + 0xf0]
140927a48: movaps xmm8, xmmword ptr [rbp + 0x100]
140927a50: add rsp, 0x198
140927a57: pop rbx
140927a58: pop rdi
140927a59: pop rsi
140927a5a: pop r12
140927a5c: pop r13
140927a5e: pop r14
140927a60: pop r15
140927a62: pop rbp
140927a63: ret 
140927a64: mov qword ptr [rsp + 0x20], 1
140927a6d: mov r8d, 0x78
140927a73: mov r9d, 1
140927a79: mov rcx, r12
140927a7c: mov r14, rdx
140927a7f: mov rdx, r13
140927a82: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927a87: mov rdx, r14
140927a8a: jmp 0x140926ddf
140927a8f: mov qword ptr [rsp + 0x20], 1
140927a98: mov r8d, 0x10
140927a9e: mov r9d, 1
140927aa4: mov rcx, rsi
140927aa7: mov rdi, rdx
140927aaa: mov rdx, r15
140927aad: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927ab2: mov rdx, rdi
140927ab5: mov rdi, qword ptr [rbp + 0x190]
140927abc: jmp 0x14092709a
140927ac1: mov qword ptr [rsp + 0x20], 1
140927aca: mov r8d, 0x78
140927ad0: mov r9d, 1
140927ad6: mov rcx, r12
140927ad9: mov rdx, rsi
140927adc: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927ae1: mov rdx, qword ptr [rbp - 0x40]
140927ae5: jmp 0x14092712e
140927aea: mov qword ptr [rsp + 0x20], 1
140927af3: mov r8d, 0x18
140927af9: mov r9d, 1
140927aff: mov rcx, rsi
140927b02: mov rdx, r14
140927b05: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927b0a: mov rdx, qword ptr [rbp - 0x40]
140927b0e: jmp 0x1409271b2
140927b13: mov qword ptr [rsp + 0x20], 1
140927b1c: mov r8d, 0x88
140927b22: mov r9d, 1
140927b28: mov rcx, r12
140927b2b: mov rdx, rsi
140927b2e: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927b33: jmp 0x1409272e3
140927b38: mov qword ptr [rsp + 0x20], 1
140927b41: mov r8d, 0x78
140927b47: mov r9d, 1
140927b4d: mov rcx, r13
140927b50: mov rdx, rsi
140927b53: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927b58: jmp 0x14092745e
140927b5d: mov qword ptr [rsp + 0x20], 1
140927b66: mov r8d, 0x88
140927b6c: mov r9d, 1
140927b72: mov rcx, r12
140927b75: mov rdx, rsi
140927b78: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927b7d: jmp 0x14092759b
140927b82: mov qword ptr [rsp + 0x20], 1
140927b8b: mov r8d, 0x88
140927b91: mov r9d, 1
140927b97: mov rcx, r12
140927b9a: mov rdx, rsi
140927b9d: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927ba2: jmp 0x1409276fb
140927ba7: mov qword ptr [rsp + 0x20], 1
140927bb0: mov r8d, 0x78
140927bb6: mov r9d, 1
140927bbc: mov rcx, r12
140927bbf: mov rdx, rsi
140927bc2: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927bc7: jmp 0x14092785c
140927bcc: mov qword ptr [rsp + 0x20], 1
140927bd5: mov r8d, 0x78
140927bdb: mov r9d, 1
140927be1: mov rcx, rsi
140927be4: mov rdx, r14
140927be7: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927bec: jmp 0x1409279d2
140927bf1: mov qword ptr [rsp + 0x20], 1
140927bfa: mov r8d, 0x88
140927c00: mov r9d, 1
140927c06: mov rcx, r13
140927c09: mov rdx, rsi
140927c0c: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140927c11: jmp 0x140926fd5
140927c16: lea rcx, [rip + 0x2312e83]
140927c1d: call 0x1428d80d0 ; std::thread::local::panic_access_error
140927c22: mov qword ptr [rbp + 0x60], rdx
140927c26: lea rax, [rip + 0x21873c3]
140927c2d: mov qword ptr [rsp + 0x20], rax
140927c32: lea rcx, [rip + 0x21868d7]
140927c39: lea r9, [rip + 0x21868b0]
140927c40: lea r8, [rbp + 0x60]
140927c44: mov edx, 0x2b
140927c49: call 0x1428d9760 ; core::result::unwrap_failed
140927c4e: jmp 0x140927d9e
140927c53: mov rax, qword ptr [rbp + 0x68]
140927c57: mov qword ptr [rbp - 0x30], rax
140927c5b: lea rax, [rip + 0x21873fe]
140927c62: mov qword ptr [rsp + 0x20], rax
140927c67: lea rcx, [rip + 0x21868a2]
140927c6e: lea r9, [rip + 0x218687b]
140927c75: lea r8, [rbp - 0x30]
140927c79: mov edx, 0x2b
140927c7e: call 0x1428d9760 ; core::result::unwrap_failed
140927c83: jmp 0x140927d9e
140927c88: mov rax, qword ptr [rbp + 0x68]
140927c8c: mov qword ptr [rbp - 0x30], rax
140927c90: lea rax, [rip + 0x2187451]
140927c97: mov qword ptr [rsp + 0x20], rax
140927c9c: lea rcx, [rip + 0x218686d]
140927ca3: lea r9, [rip + 0x2186846]
140927caa: lea r8, [rbp - 0x30]
140927cae: mov edx, 0x2b
140927cb3: call 0x1428d9760 ; core::result::unwrap_failed
140927cb8: jmp 0x140927d9e
140927cbd: mov rax, qword ptr [rbp + 0x68]
140927cc1: mov qword ptr [rbp - 0x30], rax
140927cc5: lea rax, [rip + 0x218748c]
140927ccc: mov qword ptr [rsp + 0x20], rax
140927cd1: lea rcx, [rip + 0x2186838]
140927cd8: lea r9, [rip + 0x2186811]
140927cdf: lea r8, [rbp - 0x30]
140927ce3: mov edx, 0x2b
140927ce8: call 0x1428d9760 ; core::result::unwrap_failed
140927ced: jmp 0x140927d9e
140927cf2: mov rax, qword ptr [rbp + 0x68]
140927cf6: mov qword ptr [rbp - 0x30], rax
140927cfa: lea rax, [rip + 0x218749f]
140927d01: mov qword ptr [rsp + 0x20], rax
140927d06: lea rcx, [rip + 0x2186803]
140927d0d: lea r9, [rip + 0x21867dc]
140927d14: lea r8, [rbp - 0x30]
140927d18: mov edx, 0x2b
140927d1d: call 0x1428d9760 ; core::result::unwrap_failed
140927d22: jmp 0x140927d9e
140927d24: lea rcx, [rip + 0x21874b5]
140927d2b: call 0x1428d9310 ; core::option::unwrap_failed
140927d30: mov rax, qword ptr [rbp + 0x68]
140927d34: mov qword ptr [rbp - 0x30], rax
140927d38: lea rax, [rip + 0x21874d9]
140927d3f: mov qword ptr [rsp + 0x20], rax
140927d44: lea rcx, [rip + 0x21867c5]
140927d4b: lea r9, [rip + 0x218679e]
140927d52: lea r8, [rbp - 0x30]
140927d56: mov edx, 0x2b
140927d5b: call 0x1428d9760 ; core::result::unwrap_failed
140927d60: jmp 0x140927d9e
140927d62: lea rcx, [rip + 0x21874e7]
140927d69: call 0x1428d9310 ; core::option::unwrap_failed
140927d6e: mov rax, qword ptr [rbp + 0x68]
140927d72: mov qword ptr [rbp - 0x30], rax
140927d76: lea rax, [rip + 0x2187513]
140927d7d: mov qword ptr [rsp + 0x20], rax
140927d82: lea rcx, [rip + 0x2186787]
140927d89: lea r9, [rip + 0x2186760]
140927d90: lea r8, [rbp - 0x30]
140927d94: mov edx, 0x2b
140927d99: call 0x1428d9760 ; core::result::unwrap_failed
140927d9e: ud2 
140927da0: mov qword ptr [rsp + 0x10], rdx
140927da5: push rbp
140927da6: push r15
140927da8: push r14
140927daa: push r13
140927dac: push r12
140927dae: push rsi
140927daf: push rdi
140927db0: push rbx
140927db1: sub rsp, 0x68
140927db5: lea rbp, [rdx + 0x80]
140927dbc: movaps xmmword ptr [rsp + 0x30], xmm8
140927dc2: movaps xmmword ptr [rsp + 0x40], xmm7
140927dc7: movaps xmmword ptr [rsp + 0x50], xmm6
140927dcc: lea rcx, [rbp + 0x60]
140927dd0: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
140927dd5: movaps xmm6, xmmword ptr [rsp + 0x50]
140927dda: movaps xmm7, xmmword ptr [rsp + 0x40]
140927ddf: movaps xmm8, xmmword ptr [rsp + 0x30]
140927de5: add rsp, 0x68
140927de9: pop rbx
140927dea: pop rdi
140927deb: pop rsi
140927dec: pop r12
140927dee: pop r13
140927df0: pop r14
140927df2: pop r15
140927df4: pop rbp
140927df5: ret 
140927df6: nop word ptr cs:[rax + rax]
140927e00: mov qword ptr [rsp + 0x10], rdx
140927e05: push rbp
140927e06: push r15
140927e08: push r14
140927e0a: push r13
140927e0c: push r12
140927e0e: push rsi
140927e0f: push rdi
140927e10: push rbx
140927e11: sub rsp, 0x68
140927e15: lea rbp, [rdx + 0x80]
140927e1c: movaps xmmword ptr [rsp + 0x30], xmm8
140927e22: movaps xmmword ptr [rsp + 0x40], xmm7
140927e27: movaps xmmword ptr [rsp + 0x50], xmm6
140927e2c: lea rcx, [rbp + 0x60]
140927e30: call 0x140d4cc30 ; anyhow::error::impl$6::drop
140927e35: movaps xmm6, xmmword ptr [rsp + 0x50]
140927e3a: movaps xmm7, xmmword ptr [rsp + 0x40]
140927e3f: movaps xmm8, xmmword ptr [rsp + 0x30]
140927e45: add rsp, 0x68
140927e49: pop rbx
140927e4a: pop rdi
140927e4b: pop rsi
140927e4c: pop r12
140927e4e: pop r13
140927e50: pop r14
140927e52: pop r15
140927e54: pop rbp
140927e55: ret 
140927e56: nop word ptr cs:[rax + rax]
140927e60: mov qword ptr [rsp + 0x10], rdx
140927e65: push rbp
140927e66: push r15
140927e68: push r14
140927e6a: push r13
140927e6c: push r12
140927e6e: push rsi
140927e6f: push rdi
140927e70: push rbx
140927e71: sub rsp, 0x68
140927e75: lea rbp, [rdx + 0x80]
140927e7c: movaps xmmword ptr [rsp + 0x30], xmm8
140927e82: movaps xmmword ptr [rsp + 0x40], xmm7
140927e87: movaps xmmword ptr [rsp + 0x50], xmm6
140927e8c: lea rcx, [rbp - 0x30]
140927e90: call 0x140d4cc30 ; anyhow::error::impl$6::drop
140927e95: movaps xmm6, xmmword ptr [rsp + 0x50]
140927e9a: movaps xmm7, xmmword ptr [rsp + 0x40]
140927e9f: movaps xmm8, xmmword ptr [rsp + 0x30]
140927ea5: add rsp, 0x68
140927ea9: pop rbx
140927eaa: pop rdi
140927eab: pop rsi
140927eac: pop r12
140927eae: pop r13
140927eb0: pop r14
140927eb2: pop r15
140927eb4: pop rbp
140927eb5: ret 
140927eb6: nop word ptr cs:[rax + rax]
140927ec0: mov qword ptr [rsp + 0x10], rdx
140927ec5: push rbp
140927ec6: push r15
140927ec8: push r14
140927eca: push r13
140927ecc: push r12
140927ece: push rsi
140927ecf: push rdi
140927ed0: push rbx
140927ed1: sub rsp, 0x68
140927ed5: lea rbp, [rdx + 0x80]
140927edc: movaps xmmword ptr [rsp + 0x30], xmm8
140927ee2: movaps xmmword ptr [rsp + 0x40], xmm7
140927ee7: movaps xmmword ptr [rsp + 0x50], xmm6
140927eec: lea rcx, [rbp + 0x60]
140927ef0: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
140927ef5: movaps xmm6, xmmword ptr [rsp + 0x50]
140927efa: movaps xmm7, xmmword ptr [rsp + 0x40]
140927eff: movaps xmm8, xmmword ptr [rsp + 0x30]
140927f05: add rsp, 0x68
140927f09: pop rbx
140927f0a: pop rdi
140927f0b: pop rsi
140927f0c: pop r12
140927f0e: pop r13
140927f10: pop r14
140927f12: pop r15
140927f14: pop rbp
140927f15: ret 
140927f16: nop word ptr cs:[rax + rax]
140927f20: mov qword ptr [rsp + 0x10], rdx
140927f25: push rbp
140927f26: push r15
140927f28: push r14
140927f2a: push r13
140927f2c: push r12
140927f2e: push rsi
140927f2f: push rdi
140927f30: push rbx
140927f31: sub rsp, 0x68
140927f35: lea rbp, [rdx + 0x80]
140927f3c: movaps xmmword ptr [rsp + 0x30], xmm8
140927f42: movaps xmmword ptr [rsp + 0x40], xmm7
140927f47: movaps xmmword ptr [rsp + 0x50], xmm6
140927f4c: lea rcx, [rbp - 0x30]
140927f50: call 0x140d4cc30 ; anyhow::error::impl$6::drop
140927f55: movaps xmm6, xmmword ptr [rsp + 0x50]
140927f5a: movaps xmm7, xmmword ptr [rsp + 0x40]
140927f5f: movaps xmm8, xmmword ptr [rsp + 0x30]
140927f65: add rsp, 0x68
140927f69: pop rbx
140927f6a: pop rdi
140927f6b: pop rsi
140927f6c: pop r12
140927f6e: pop r13
140927f70: pop r14
140927f72: pop r15
140927f74: pop rbp
140927f75: ret 
140927f76: nop word ptr cs:[rax + rax]
140927f80: mov qword ptr [rsp + 0x10], rdx
140927f85: push rbp
140927f86: push r15
140927f88: push r14
140927f8a: push r13
140927f8c: push r12
140927f8e: push rsi
140927f8f: push rdi
140927f90: push rbx
140927f91: sub rsp, 0x68
140927f95: lea rbp, [rdx + 0x80]
140927f9c: movaps xmmword ptr [rsp + 0x30], xmm8
140927fa2: movaps xmmword ptr [rsp + 0x40], xmm7
140927fa7: movaps xmmword ptr [rsp + 0x50], xmm6
140927fac: lea rcx, [rbp - 0x30]
140927fb0: call 0x140d4cc30 ; anyhow::error::impl$6::drop
140927fb5: movaps xmm6, xmmword ptr [rsp + 0x50]
140927fba: movaps xmm7, xmmword ptr [rsp + 0x40]
140927fbf: movaps xmm8, xmmword ptr [rsp + 0x30]
140927fc5: add rsp, 0x68
140927fc9: pop rbx
140927fca: pop rdi
140927fcb: pop rsi
140927fcc: pop r12
140927fce: pop r13
140927fd0: pop r14
140927fd2: pop r15
140927fd4: pop rbp
140927fd5: ret 
140927fd6: nop word ptr cs:[rax + rax]
140927fe0: mov qword ptr [rsp + 0x10], rdx
140927fe5: push rbp
140927fe6: push r15
140927fe8: push r14
140927fea: push r13
140927fec: push r12
140927fee: push rsi
140927fef: push rdi
140927ff0: push rbx
140927ff1: sub rsp, 0x68
140927ff5: lea rbp, [rdx + 0x80]
140927ffc: movaps xmmword ptr [rsp + 0x30], xmm8
140928002: movaps xmmword ptr [rsp + 0x40], xmm7
140928007: movaps xmmword ptr [rsp + 0x50], xmm6
14092800c: lea rcx, [rbp - 0x30]
140928010: call 0x140d4cc30 ; anyhow::error::impl$6::drop
140928015: movaps xmm6, xmmword ptr [rsp + 0x50]
14092801a: movaps xmm7, xmmword ptr [rsp + 0x40]
14092801f: movaps xmm8, xmmword ptr [rsp + 0x30]
140928025: add rsp, 0x68
140928029: pop rbx
14092802a: pop rdi
14092802b: pop rsi
14092802c: pop r12
14092802e: pop r13
140928030: pop r14
140928032: pop r15
140928034: pop rbp
140928035: ret 
140928036: nop word ptr cs:[rax + rax]
140928040: mov qword ptr [rsp + 0x10], rdx
140928045: push rbp
140928046: push r15
140928048: push r14
14092804a: push r13
14092804c: push r12
14092804e: push rsi
14092804f: push rdi
140928050: push rbx
140928051: sub rsp, 0x68
140928055: lea rbp, [rdx + 0x80]
14092805c: movaps xmmword ptr [rsp + 0x30], xmm8
140928062: movaps xmmword ptr [rsp + 0x40], xmm7
140928067: movaps xmmword ptr [rsp + 0x50], xmm6
14092806c: lea rcx, [rbp - 0x30]
140928070: call 0x140d4cc30 ; anyhow::error::impl$6::drop
140928075: movaps xmm6, xmmword ptr [rsp + 0x50]
14092807a: movaps xmm7, xmmword ptr [rsp + 0x40]
14092807f: movaps xmm8, xmmword ptr [rsp + 0x30]
140928085: add rsp, 0x68
140928089: pop rbx
14092808a: pop rdi
14092808b: pop rsi
14092808c: pop r12
14092808e: pop r13
140928090: pop r14
140928092: pop r15
140928094: pop rbp
140928095: ret 
140928096: nop word ptr cs:[rax + rax]
1409280a0: mov qword ptr [rsp + 0x10], rdx
1409280a5: push rbp
1409280a6: push r15
1409280a8: push r14
1409280aa: push r13
1409280ac: push r12
1409280ae: push rsi
1409280af: push rdi
1409280b0: push rbx
1409280b1: sub rsp, 0x68
1409280b5: lea rbp, [rdx + 0x80]
1409280bc: movaps xmmword ptr [rsp + 0x30], xmm8
1409280c2: movaps xmmword ptr [rsp + 0x40], xmm7
1409280c7: movaps xmmword ptr [rsp + 0x50], xmm6
1409280cc: lea rcx, [rbp - 0x30]
1409280d0: call 0x140d4cc30 ; anyhow::error::impl$6::drop
1409280d5: movaps xmm6, xmmword ptr [rsp + 0x50]
1409280da: movaps xmm7, xmmword ptr [rsp + 0x40]
1409280df: movaps xmm8, xmmword ptr [rsp + 0x30]
1409280e5: add rsp, 0x68
1409280e9: pop rbx
1409280ea: pop rdi
1409280eb: pop rsi
1409280ec: pop r12
1409280ee: pop r13
1409280f0: pop r14
1409280f2: pop r15
1409280f4: pop rbp
1409280f5: ret 
