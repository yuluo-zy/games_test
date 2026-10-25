1423179a0: push rbp
1423179a1: push r15
1423179a3: push r14
1423179a5: push r13
1423179a7: push r12
1423179a9: push rsi
1423179aa: push rdi
1423179ab: push rbx
1423179ac: sub rsp, 0x298
1423179b3: lea rbp, [rsp + 0x80]
1423179bb: movaps xmmword ptr [rbp + 0x200], xmm15
1423179c3: movaps xmmword ptr [rbp + 0x1f0], xmm14
1423179cb: movaps xmmword ptr [rbp + 0x1e0], xmm13
1423179d3: movaps xmmword ptr [rbp + 0x1d0], xmm12
1423179db: movaps xmmword ptr [rbp + 0x1c0], xmm11
1423179e3: movaps xmmword ptr [rbp + 0x1b0], xmm10
1423179eb: movaps xmmword ptr [rbp + 0x1a0], xmm9
1423179f3: movaps xmmword ptr [rbp + 0x190], xmm8
1423179fb: movaps xmmword ptr [rbp + 0x180], xmm7
142317a02: movaps xmmword ptr [rbp + 0x170], xmm6
142317a09: mov qword ptr [rbp + 0x168], 0xfffffffffffffffe
142317a14: mov r15, r9
142317a17: mov rbx, r8
142317a1a: mov rsi, rdx
142317a1d: mov r14, rcx
142317a20: call 0x140c16a00 ; puffin::are_scopes_on
142317a25: test al, al
142317a27: je 0x142317a88
142317a29: lea r12, [rip + 0xd334d0]
142317a30: mov edi, 0x32
142317a35: mov edx, 0x32
142317a3a: mov rcx, r12
142317a3d: call 0x142317580 ; core::str::impl$0::rfind<ref$<str$> >
142317a42: cmp rax, 1
142317a46: jne 0x142317afd
142317a4c: mov r9, rdx
142317a4f: test rdx, rdx
142317a52: je 0x142317a91
142317a54: cmp r9, 0x32
142317a58: jae 0x142317a8f
142317a5a: lea rax, [rip + 0xd3349f]
142317a61: cmp byte ptr [r9 + rax], 0xbf
142317a66: jg 0x142317a91
142317a68: lea rax, [rip + 0xd33571]
142317a6f: mov qword ptr [rsp + 0x20], rax
142317a74: lea rcx, [rip + 0xd33485]
142317a7b: mov edx, 0x32
142317a80: xor r8d, r8d
142317a83: call 0x1428d9940 ; core::str::slice_error_fail
142317a88: xor eax, eax
142317a8a: jmp 0x142317c7a
142317a8f: jne 0x142317a68
142317a91: lea r12, [rip + 0xd33468]
142317a98: mov rcx, r12
142317a9b: mov rdx, r9
142317a9e: call 0x142317580 ; core::str::impl$0::rfind<ref$<str$> >
142317aa3: cmp rax, 1
142317aa7: jne 0x142317afd
142317aa9: mov r8, rdx
142317aac: add r8, 2
142317ab0: je 0x142317aeb
142317ab2: cmp r8, 0x32
142317ab6: jae 0x142317ae9
142317ab8: lea rax, [rip + 0xd33441]
142317abf: cmp byte ptr [r8 + rax], 0xbf
142317ac4: jg 0x142317aeb
142317ac6: lea rax, [rip + 0xd3352b]
142317acd: mov qword ptr [rsp + 0x20], rax
142317ad2: lea rcx, [rip + 0xd33427]
142317ad9: mov edx, 0x32
142317ade: mov r9d, 0x32
142317ae4: call 0x1428d9940 ; core::str::slice_error_fail
142317ae9: jne 0x142317ac6
142317aeb: mov edi, 0x30
142317af0: sub rdi, rdx
142317af3: lea r12, [rip + 0xd33406]
142317afa: add r12, r8
142317afd: lea r8, [rip + 0xd33549]
142317b04: lea r13, [rip + 0xd3351d]
142317b0b: mov qword ptr [rbp + 0x150], r15
142317b12: nop word ptr cs:[rax + rax]
142317b20: movsx eax, byte ptr [r8 - 1]
142317b25: test eax, eax
142317b27: js 0x142317b40
142317b29: dec r8
142317b2c: cmp eax, 0x5c
142317b2f: jne 0x142317b94
142317b31: jmp 0x142317bab
142317b33: nop word ptr cs:[rax + rax]
142317b40: movzx ecx, byte ptr [r8 - 2]
142317b45: cmp cl, 0xc0
142317b48: jge 0x142317b6d
142317b4a: movzx edx, byte ptr [r8 - 3]
142317b4f: cmp dl, 0xc0
142317b52: jge 0x142317b76
142317b54: movzx r9d, byte ptr [r8 - 4]
142317b59: add r8, -4
142317b5d: and r9d, 7
142317b61: shl r9d, 6
142317b65: and edx, 0x3f
142317b68: or edx, r9d
142317b6b: jmp 0x142317b7d
142317b6d: add r8, -2
142317b71: and ecx, 0x1f
142317b74: jmp 0x142317b85
142317b76: add r8, -3
142317b7a: and edx, 0xf
142317b7d: shl edx, 6
142317b80: and ecx, 0x3f
142317b83: or ecx, edx
142317b85: shl ecx, 6
142317b88: and al, 0x3f
142317b8a: movzx eax, al
142317b8d: or eax, ecx
142317b8f: cmp eax, 0x5c
142317b92: je 0x142317bab
142317b94: cmp eax, 0x2f
142317b97: je 0x142317bab
142317b99: cmp r8, r13
142317b9c: jne 0x142317b20
142317b9e: mov r15, rbx
142317ba1: mov rbx, rsi
142317ba4: mov esi, 0x25
142317ba9: jmp 0x142317bfd
142317bab: lea r13, [rip + 0xd33476]
142317bb2: sub r8, r13
142317bb5: inc r8
142317bb8: je 0x142317bec
142317bba: cmp r8, 0x25
142317bbe: jae 0x142317bea
142317bc0: cmp byte ptr [r8 + r13], 0xbf
142317bc5: jg 0x142317bec
142317bc7: lea rax, [rip + 0xd333f2]
142317bce: mov qword ptr [rsp + 0x20], rax
142317bd3: lea rcx, [rip + 0xd3344e]
142317bda: mov edx, 0x25
142317bdf: mov r9d, 0x25
142317be5: call 0x1428d9940 ; core::str::slice_error_fail
142317bea: jne 0x142317bc7
142317bec: mov r15, rbx
142317bef: mov rbx, rsi
142317bf2: mov esi, 0x25
142317bf7: sub rsi, r8
142317bfa: add r13, r8
142317bfd: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142317c02: mov rcx, rax
142317c05: mov rax, qword ptr [rax]
142317c08: cmp rax, 1
142317c0c: jne 0x1423188ed
142317c12: add rcx, 8
142317c16: cmp qword ptr [rcx], 0
142317c1a: jne 0x142318911
142317c20: mov qword ptr [rcx], 0xffffffffffffffff
142317c27: mov qword ptr [rbp + 0x130], rcx
142317c2e: add rcx, 8
142317c32: mov qword ptr [rsp + 0x20], rsi
142317c37: mov qword ptr [rsp + 0x30], 0
142317c40: mov qword ptr [rsp + 0x28], 1
142317c49: mov rdx, r12
142317c4c: mov r8, rdi
142317c4f: mov r9, r13
142317c52: call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
142317c57: mov rcx, qword ptr [rbp + 0x130]
142317c5e: inc qword ptr [rcx]
142317c61: mov qword ptr [rbp + 0xe0], rax
142317c68: mov eax, 1
142317c6d: mov rsi, rbx
142317c70: mov rbx, r15
142317c73: mov r15, qword ptr [rbp + 0x150]
142317c7a: mov qword ptr [rbp + 0xd8], rax
142317c81: mov rcx, qword ptr [r15]
142317c84: lea rdx, [rip + 0xd3265d]
142317c8b: call 0x14230e780 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
142317c90: mov rdi, rax
142317c93: test rax, rax
142317c96: je 0x142318953
142317c9c: lea rax, [rip + 0xd3272d]
142317ca3: mov qword ptr [rbp + 0x10], rax
142317ca7: mov qword ptr [rbp + 0x18], 0x3f
142317caf: mov r15, rdi
142317cb2: add r15, 0x10
142317cb6: cmp qword ptr [rdi + 0x18], 0x3f
142317cbb: jne 0x14231891d
142317cc1: mov rdx, qword ptr [r15]
142317cc4: lea rcx, [rip + 0xd32705]
142317ccb: mov r8d, 0x3f
142317cd1: call 0x142923bf0 ; 
142317cd6: test eax, eax
142317cd8: jne 0x14231891d
142317cde: mov rcx, qword ptr [rdi]
142317ce1: mov rax, qword ptr [rdi + 8]
142317ce5: call qword ptr [rax + 0x18]
142317ce8: test rax, rax
142317ceb: je 0x142318953
142317cf1: mov rcx, qword ptr [rbp + 0x288]
142317cf8: mov rdx, qword ptr [rax]
142317cfb: mov qword ptr [rbp + 0x88], rdx
142317d02: mov rax, qword ptr [rax + 8]
142317d06: mov qword ptr [rbp + 0x90], rax
142317d0d: mov rcx, qword ptr [rcx]
142317d10: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
142317d15: mov rdx, qword ptr [rbp + 0x280]
142317d1c: lea rcx, [rax + 8]
142317d20: lea r8, [rax + 0x20]
142317d24: lea r9, [rax + 0x10]
142317d28: mov r10, rax
142317d2b: add r10, 0x28
142317d2f: cmp byte ptr [rax + 0x298], 0
142317d36: cmovne rcx, r8
142317d3a: cmove r10, r9
142317d3e: mov rax, qword ptr [r10]
142317d41: mov rcx, qword ptr [rcx]
142317d44: mov qword ptr [rbp + 0x50], rcx
142317d48: mov qword ptr [rbp + 0x58], rax
142317d4c: lea rcx, [rbp + 0x10]
142317d50: call 0x140ae4c20 ; country_core::systems::paths::draw_path::PathRastersResources::as_float_grid
142317d55: mov rax, qword ptr [rbp + 0x290]
142317d5c: mov r15, qword ptr [r14]
142317d5f: mov r12, qword ptr [r14 + 8]
142317d63: mov ecx, dword ptr [r14 + 0x14]
142317d67: mov dword ptr [rbp + 0x158], ecx
142317d6d: mov r13, qword ptr [r15 + 0xf8]
142317d74: mov rcx, qword ptr [r15 + 0x100]
142317d7b: lea rcx, [r13 + rcx*4]
142317d80: mov qword ptr [rbp + 0xe8], rcx
142317d87: movzx ecx, byte ptr [r15 + 0x108]
142317d8f: mov byte ptr [rbp + 0x163], cl
142317d95: mov rcx, qword ptr [rbx]
142317d98: mov qword ptr [rbp + 0xa0], rcx
142317d9f: mov rcx, qword ptr [rsi]
142317da2: mov qword ptr [rbp + 0x80], rcx
142317da9: mov rax, qword ptr [rax]
142317dac: mov qword ptr [rbp + 0xa8], rax
142317db3: mov eax, 8
142317db8: mov qword ptr [rbp + 0xf0], rax
142317dbf: movss xmm12, dword ptr [rip + 0x60db3c]
142317dc8: movss xmm13, dword ptr [rip + 0x60dbb3]
142317dd1: movss xmm6, dword ptr [rip + 0x76387f]
142317dd9: xorps xmm14, xmm14
142317ddd: movss xmm15, dword ptr [rip + 0x60daf2]
142317de6: mov eax, 0
142317deb: mov qword ptr [rbp + 0x108], rax
142317df2: mov qword ptr [rbp + 0x140], 0
142317dfd: mov dword ptr [rbp + 0x164], 0
142317e07: mov qword ptr [rbp + 0x138], 0
142317e12: mov qword ptr [rbp + 0x130], 0
142317e1d: xor r14d, r14d
142317e20: mov eax, 8
142317e25: mov qword ptr [rbp + 0xf8], rax
142317e2c: mov qword ptr [rbp + 0x148], 0
142317e37: mov dword ptr [rbp + 0x15c], 0
142317e41: mov qword ptr [rbp + 0x110], r15
142317e48: mov qword ptr [rbp + 0x118], r12
142317e4f: cmp byte ptr [rbp + 0x163], 0
142317e56: je 0x142317f00
142317e5c: mov r8, qword ptr [rbp + 0x130]
142317e63: cmp r8, r14
142317e66: mov rdx, qword ptr [rbp + 0xe8]
142317e6d: jne 0x1423180f8
142317e73: nop word ptr cs:[rax + rax]
142317e80: cmp r13, rdx
142317e83: je 0x14231881c
142317e89: mov ecx, dword ptr [r13]
142317e8d: add r13, 4
142317e91: mov rax, qword ptr [r12 + 0x1a8]
142317e99: lea rcx, [rcx + rcx*8]
142317e9d: mov r14, qword ptr [rax + rcx*8 + 0x10]
142317ea2: test r14, r14
142317ea5: je 0x142317e80
142317ea7: lea rax, [rax + rcx*8]
142317eab: mov rdx, qword ptr [r15 + 0x110]
142317eb2: mov rcx, qword ptr [rax + 0x38]
142317eb6: mov r8, qword ptr [rax + 0x40]
142317eba: cmp r8, rdx
142317ebd: jbe 0x142317fb7
142317ec3: mov r9, qword ptr [rcx + rdx*8]
142317ec7: mov rdx, qword ptr [rax + 0x18]
142317ecb: test r9, r9
142317ece: je 0x142317ff8
142317ed4: not r9
142317ed7: lea r9, [r9 + r9*2]
142317edb: shl r9, 4
142317edf: mov r9, qword ptr [rdx + r9 + 0x10]
142317ee4: mov qword ptr [rbp + 0x140], r9
142317eeb: mov r9, qword ptr [r15 + 0x120]
142317ef2: cmp r8, r9
142317ef5: ja 0x14231800f
142317efb: jmp 0x14231803d
142317f00: mov r8, qword ptr [rbp + 0x130]
142317f07: cmp r8, r14
142317f0a: mov rdx, qword ptr [rbp + 0xe8]
142317f11: jne 0x14231824c
142317f17: nop word ptr [rax + rax]
142317f20: cmp r13, rdx
142317f23: je 0x14231881c
142317f29: mov ecx, dword ptr [r13]
142317f2d: add r13, 4
142317f31: mov rax, qword ptr [r12 + 0x100]
142317f39: lea rcx, [rcx + rcx*4]
142317f3d: shl rcx, 5
142317f41: mov r14, qword ptr [rax + rcx + 0x58]
142317f46: test r14, r14
142317f49: je 0x142317f20
142317f4b: add rax, rcx
142317f4e: mov ecx, dword ptr [rax + 0x94]
142317f54: mov r8, qword ptr [r12 + 0x1a8]
142317f5c: lea r10, [rcx + rcx*8]
142317f60: lea rdx, [r8 + r10*8]
142317f64: mov r9, qword ptr [r15 + 0x110]
142317f6b: mov rcx, qword ptr [r8 + r10*8 + 0x38]
142317f70: mov r8, qword ptr [r8 + r10*8 + 0x40]
142317f75: cmp r8, r9
142317f78: jbe 0x142317fd4
142317f7a: mov r9, qword ptr [rcx + r9*8]
142317f7e: mov rdx, qword ptr [rdx + 0x18]
142317f82: test r9, r9
142317f85: je 0x14231810b
142317f8b: not r9
142317f8e: lea r9, [r9 + r9*2]
142317f92: shl r9, 4
142317f96: mov r9, qword ptr [rdx + r9 + 0x10]
142317f9b: mov qword ptr [rbp + 0x140], r9
142317fa2: mov r9, qword ptr [r15 + 0x120]
142317fa9: cmp r8, r9
142317fac: ja 0x142318122
142317fb2: jmp 0x142318157
142317fb7: mov rdx, qword ptr [rax + 0x18]
142317fbb: mov qword ptr [rbp + 0x140], 0
142317fc6: mov r9, qword ptr [r15 + 0x120]
142317fcd: cmp r8, r9
142317fd0: jbe 0x14231803d
142317fd2: jmp 0x14231800f
142317fd4: mov rdx, qword ptr [rdx + 0x18]
142317fd8: mov qword ptr [rbp + 0x140], 0
142317fe3: mov r9, qword ptr [r15 + 0x120]
142317fea: cmp r8, r9
142317fed: jbe 0x142318157
142317ff3: jmp 0x142318122
142317ff8: mov qword ptr [rbp + 0x140], 0
142318003: mov r9, qword ptr [r15 + 0x120]
14231800a: cmp r8, r9
14231800d: jbe 0x14231803d
14231800f: mov r9, qword ptr [rcx + r9*8]
142318013: test r9, r9
142318016: je 0x14231803d
142318018: not r9
14231801b: lea r9, [r9 + r9*2]
14231801f: shl r9, 4
142318023: mov r9, qword ptr [rdx + r9 + 0x10]
142318028: mov qword ptr [rbp + 0x148], r9
14231802f: mov r9, qword ptr [r15 + 0x128]
142318036: cmp r8, r9
142318039: ja 0x142318054
14231803b: jmp 0x142318096
14231803d: mov qword ptr [rbp + 0x148], 0
142318048: mov r9, qword ptr [r15 + 0x128]
14231804f: cmp r8, r9
142318052: jbe 0x142318096
142318054: mov r9, qword ptr [rcx + r9*8]
142318058: test r9, r9
14231805b: je 0x142318096
14231805d: not r9
142318060: lea r9, [r9 + r9*2]
142318064: shl r9, 4
142318068: mov r9, qword ptr [rdx + r9 + 0x10]
14231806d: mov qword ptr [rbp + 0x138], r9
142318074: mov r9b, 1
142318077: mov dword ptr [rbp + 0x164], r9d
14231807e: mov r9, qword ptr [r15 + 0x130]
142318085: cmp r8, r9
142318088: ja 0x1423180ac
14231808a: mov dword ptr [rbp + 0x15c], 0
142318094: jmp 0x1423180bc
142318096: mov dword ptr [rbp + 0x164], 0
1423180a0: mov r9, qword ptr [r15 + 0x130]
1423180a7: cmp r8, r9
1423180aa: jbe 0x14231808a
1423180ac: cmp qword ptr [rcx + r9*8], 0
1423180b1: setne r8b
1423180b5: mov dword ptr [rbp + 0x15c], r8d
1423180bc: mov r8, qword ptr [r15 + 0x118]
1423180c3: mov rcx, qword ptr [rcx + r8*8]
1423180c7: not rcx
1423180ca: lea rcx, [rcx + rcx*2]
1423180ce: shl rcx, 4
1423180d2: mov r8, qword ptr [rdx + rcx + 0x10]
1423180d7: mov qword ptr [rbp + 0x108], r8
1423180de: mov rcx, qword ptr [rdx + rcx + 0x28]
1423180e3: mov qword ptr [rbp + 0x100], rcx
1423180ea: mov rax, qword ptr [rax + 8]
1423180ee: mov qword ptr [rbp + 0xf8], rax
1423180f5: xor r8d, r8d
1423180f8: mov rax, qword ptr [rbp + 0xf8]
1423180ff: lea rax, [rax + r8*8]
142318103: mov edi, r8d
142318106: jmp 0x142318262
14231810b: mov qword ptr [rbp + 0x140], 0
142318116: mov r9, qword ptr [r15 + 0x120]
14231811d: cmp r8, r9
142318120: jbe 0x142318157
142318122: mov r9, qword ptr [rcx + r9*8]
142318126: test r9, r9
142318129: je 0x142318157
14231812b: not r9
14231812e: lea r9, [r9 + r9*2]
142318132: shl r9, 4
142318136: mov r9, qword ptr [rdx + r9 + 0x10]
14231813b: mov qword ptr [rbp + 0x148], r9
142318142: mov r10, qword ptr [r15 + 0x128]
142318149: mov r9, qword ptr [rax + 0x88]
142318150: cmp r9, r10
142318153: ja 0x142318175
142318155: jmp 0x1423181c0
142318157: mov qword ptr [rbp + 0x148], 0
142318162: mov r10, qword ptr [r15 + 0x128]
142318169: mov r9, qword ptr [rax + 0x88]
142318170: cmp r9, r10
142318173: jbe 0x1423181c0
142318175: mov r11, qword ptr [rax + 0x80]
14231817c: cmp qword ptr [r11 + r10*8], 0
142318181: je 0x1423181c0
142318183: mov r11b, 1
142318186: mov dword ptr [rbp + 0x164], r11d
14231818d: cmp r8, r10
142318190: jbe 0x1423181ef
142318192: mov r8, qword ptr [rcx + r10*8]
142318196: test r8, r8
142318199: je 0x1423181ef
14231819b: not r8
14231819e: lea r8, [r8 + r8*2]
1423181a2: shl r8, 4
1423181a6: mov r8, qword ptr [rdx + r8 + 0x10]
1423181ab: mov qword ptr [rbp + 0x138], r8
1423181b2: mov r8, qword ptr [r15 + 0x130]
1423181b9: cmp r9, r8
1423181bc: ja 0x1423181d6
1423181be: jmp 0x142318206
1423181c0: mov dword ptr [rbp + 0x164], 0
1423181ca: mov r8, qword ptr [r15 + 0x130]
1423181d1: cmp r9, r8
1423181d4: jbe 0x142318206
1423181d6: mov r9, qword ptr [rax + 0x80]
1423181dd: cmp qword ptr [r9 + r8*8], 0
1423181e2: setne r8b
1423181e6: mov dword ptr [rbp + 0x15c], r8d
1423181ed: jmp 0x142318210
1423181ef: mov qword ptr [rbp + 0x138], 0
1423181fa: mov r8, qword ptr [r15 + 0x130]
142318201: cmp r9, r8
142318204: ja 0x1423181d6
142318206: mov dword ptr [rbp + 0x15c], 0
142318210: mov r8, qword ptr [r15 + 0x118]
142318217: mov rcx, qword ptr [rcx + r8*8]
14231821b: not rcx
14231821e: lea rcx, [rcx + rcx*2]
142318222: shl rcx, 4
142318226: mov r8, qword ptr [rdx + rcx + 0x10]
14231822b: mov qword ptr [rbp + 0x108], r8
142318232: mov rcx, qword ptr [rdx + rcx + 0x28]
142318237: mov qword ptr [rbp + 0x100], rcx
14231823e: mov rax, qword ptr [rax + 0x50]
142318242: mov qword ptr [rbp + 0xf0], rax
142318249: xor r8d, r8d
14231824c: mov rcx, r8
14231824f: shl rcx, 4
142318253: mov rdx, qword ptr [rbp + 0xf0]
14231825a: lea rax, [rdx + rcx]
14231825e: mov edi, dword ptr [rdx + rcx + 8]
142318262: lea rbx, [rdi + rdi*2]
142318266: shl rbx, 4
14231826a: add rbx, qword ptr [rbp + 0x140]
142318271: mov rcx, qword ptr [rbp + 0x100]
142318278: lea r9, [rcx + rdi*4]
14231827c: mov rcx, qword ptr [rbp + 0x148]
142318283: lea rcx, [rcx + rdi*8]
142318287: mov qword ptr [rbp + 0x150], rcx
14231828e: mov rdx, r8
142318291: imul rsi, rdi, 0x68
142318295: shl rdi, 4
142318299: add rdi, qword ptr [rbp + 0x108]
1423182a0: add rsi, qword ptr [rbp + 0x138]
1423182a7: test byte ptr [rbp + 0x164], 1
1423182ae: mov ecx, 0
1423182b3: cmove rsi, rcx
1423182b7: mov rcx, qword ptr [rax]
1423182ba: inc rdx
1423182bd: mov qword ptr [rbp + 0x130], rdx
1423182c4: mov eax, dword ptr [rbp + 0x158]
1423182ca: mov dword ptr [r9], eax
1423182cd: cmp dword ptr [rdi], 1
1423182d0: jne 0x142317e4f
1423182d6: mov qword ptr [rbp + 0x98], rcx
1423182dd: mov qword ptr [rbp + 0x120], r9
1423182e4: movss xmm3, dword ptr [rdi + 4]
1423182e9: movss xmm0, dword ptr [rdi + 0xc]
1423182ee: movss xmm1, dword ptr [rbx + 0x10]
1423182f3: movss xmm2, dword ptr [rbx + 0x18]
1423182f8: subss xmm3, xmm1
1423182fc: movaps xmm4, xmm0
1423182ff: subss xmm4, xmm2
142318303: movaps xmm5, xmm3
142318306: mulss xmm5, xmm3
14231830a: movaps xmm7, xmm4
14231830d: mulss xmm7, xmm4
142318311: addss xmm7, xmm5
142318315: sqrtss xmm7, xmm7
142318319: movaps xmm5, xmm12
14231831d: divss xmm5, xmm7
142318321: movd eax, xmm5
142318325: test eax, eax
142318327: setns cl
14231832a: mov edx, eax
14231832c: and edx, 0x7fffffff
142318332: add edx, 0xff800000
142318338: cmp edx, 0x7f000000
14231833e: setb dl
142318341: and dl, cl
142318343: dec eax
142318345: cmp eax, 0x7fffff
14231834a: setb al
14231834d: or al, dl
14231834f: jne 0x142318359
142318351: xorps xmm4, xmm4
142318354: xorps xmm3, xmm3
142318357: jmp 0x142318361
142318359: mulss xmm3, xmm5
14231835d: mulss xmm4, xmm5
142318361: movss dword ptr [rbp + 0x128], xmm3
142318369: movss dword ptr [rbp + 0x12c], xmm4
142318371: addss xmm3, dword ptr [rdi + 4]
142318376: addss xmm0, xmm4
14231837a: movss dword ptr [rsp + 0x20], xmm0
142318380: lea rcx, [rbp - 0x30]
142318384: call 0x140c97560 ; utils::geometry::circle::Circle2d::new_rotated
142318389: lea rcx, [rbp - 0x48]
14231838d: lea rdx, [rbp - 0x30]
142318391: call 0x14261f450 ; utils::geometry::line_relation::LineInterval::line_segment
142318396: movss xmm7, dword ptr [rbp + 0x128]
14231839e: mulss xmm7, xmm13
1423183a3: movss xmm8, dword ptr [rbp + 0x12c]
1423183ac: mulss xmm8, xmm13
1423183b1: addss xmm7, dword ptr [rbx + 0x10]
1423183b6: addss xmm8, dword ptr [rbx + 0x18]
1423183bc: mov dword ptr [rsp + 0x20], 0x43020000
1423183c4: lea rcx, [rbp + 0x50]
1423183c8: movaps xmm1, xmm7
1423183cb: movaps xmm2, xmm8
1423183cf: movaps xmm3, xmm6
1423183d2: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
1423183d7: mov rax, qword ptr [rbp + 0x150]
1423183de: cmp dword ptr [rax], 2
1423183e1: jne 0x142318410
1423183e3: movaps xmm9, xmm0
1423183e7: mov rcx, qword ptr [rbp + 0x88]
1423183ee: mov rdx, qword ptr [rbp + 0x90]
1423183f5: movaps xmm2, xmm7
1423183f8: movaps xmm3, xmm8
1423183fc: call 0x1409ea800 ; country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz
142318401: ucomiss xmm9, xmm14
142318405: seta cl
142318408: or cl, al
14231840a: jne 0x1423187f4
142318410: test byte ptr [rbp + 0x15c], 1
142318417: je 0x1423185ba
14231841d: movss xmm8, dword ptr [rbx + 0x10]
142318423: movss xmm9, dword ptr [rbx + 0x18]
142318429: mov dword ptr [rsp + 0x20], 0x43020000
142318431: lea rcx, [rbp + 0x10]
142318435: movaps xmm1, xmm8
142318439: movaps xmm2, xmm9
14231843d: movaps xmm3, xmm6
142318440: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142318445: movaps xmm7, xmm0
142318448: mov dword ptr [rsp + 0x20], 0x43020000
142318450: lea rcx, [rbp + 0x20]
142318454: movaps xmm1, xmm8
142318458: movaps xmm2, xmm9
14231845c: movaps xmm3, xmm6
14231845f: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142318464: movaps xmm10, xmm0
142318468: mov dword ptr [rsp + 0x20], 0x43020000
142318470: lea rcx, [rbp + 0x30]
142318474: movaps xmm1, xmm8
142318478: movaps xmm2, xmm9
14231847c: movaps xmm3, xmm6
14231847f: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142318484: movaps xmm11, xmm0
142318488: mov dword ptr [rsp + 0x20], 0x43020000
142318490: lea rcx, [rbp + 0x40]
142318494: movaps xmm1, xmm8
142318498: movaps xmm2, xmm9
14231849c: movaps xmm3, xmm6
14231849f: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
1423184a4: addss xmm7, xmm10
1423184a9: addss xmm7, xmm11
1423184ae: addss xmm7, xmm0
1423184b2: xorps xmm0, xmm0
1423184b5: maxss xmm0, xmm7
1423184b9: movaps xmm1, xmm12
1423184bd: minss xmm1, xmm0
1423184c1: ucomiss xmm1, xmm14
1423184c5: movaps xmm7, xmm15
1423184c9: jbe 0x1423184ef
1423184cb: movss xmm0, dword ptr [rdi + 4]
1423184d0: mulss xmm0, xmm0
1423184d4: movsd xmm1, qword ptr [rdi + 8]
1423184d9: mulps xmm1, xmm1
1423184dc: addss xmm0, xmm1
1423184e0: movshdup xmm1, xmm1
1423184e4: addss xmm1, xmm0
1423184e8: xorps xmm7, xmm7
1423184eb: sqrtss xmm7, xmm1
1423184ef: movss xmm8, dword ptr [rbp + 0x128]
1423184f8: mulss xmm8, xmm7
1423184fd: mulss xmm7, dword ptr [rbp + 0x12c]
142318505: addss xmm8, dword ptr [rbx + 0x10]
14231850b: addss xmm7, dword ptr [rbx + 0x18]
142318510: mov dword ptr [rsp + 0x20], 0x43020000
142318518: lea rcx, [rbp + 0x10]
14231851c: movaps xmm1, xmm8
142318520: movaps xmm2, xmm7
142318523: movaps xmm3, xmm6
142318526: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
14231852b: movaps xmm9, xmm0
14231852f: mov dword ptr [rsp + 0x20], 0x43020000
142318537: lea rcx, [rbp + 0x20]
14231853b: movaps xmm1, xmm8
14231853f: movaps xmm2, xmm7
142318542: movaps xmm3, xmm6
142318545: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
14231854a: movaps xmm10, xmm0
14231854e: mov dword ptr [rsp + 0x20], 0x43020000
142318556: lea rcx, [rbp + 0x30]
14231855a: movaps xmm1, xmm8
14231855e: movaps xmm2, xmm7
142318561: movaps xmm3, xmm6
142318564: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142318569: movaps xmm11, xmm0
14231856d: mov dword ptr [rsp + 0x20], 0x43020000
142318575: lea rcx, [rbp + 0x40]
142318579: movaps xmm1, xmm8
14231857d: movaps xmm2, xmm7
142318580: movaps xmm3, xmm6
142318583: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142318588: addss xmm9, xmm10
14231858d: addss xmm9, xmm11
142318592: addss xmm9, xmm0
142318597: xorps xmm0, xmm0
14231859a: maxss xmm0, xmm9
14231859f: movaps xmm1, xmm12
1423185a3: minss xmm1, xmm0
1423185a7: ucomiss xmm1, xmm14
1423185ab: ja 0x1423187f4
1423185b1: test rsi, rsi
1423185b4: setne r12b
1423185b8: jmp 0x1423185c6
1423185ba: mov r12b, 1
1423185bd: test rsi, rsi
1423185c0: je 0x142318730
1423185c6: mov rax, qword ptr [rbp + 0x150]
1423185cd: mov eax, dword ptr [rax]
1423185cf: cmp eax, 6
1423185d2: ja 0x1423185e2
1423185d4: mov ecx, 0x63
1423185d9: bt ecx, eax
1423185dc: jb 0x142318809
1423185e2: lea rax, [rbx + 0x10]
1423185e6: mov ecx, dword ptr [rax + 8]
1423185e9: mov dword ptr [rbp + 0xb8], ecx
1423185ef: mov rax, qword ptr [rax]
1423185f2: mov qword ptr [rbp + 0xb0], rax
1423185f9: lea rcx, [rbp + 0x70]
1423185fd: lea rdx, [rbp + 0x128]
142318604: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
142318609: movss xmm0, dword ptr [rbp + 0x78]
14231860e: movaps xmm1, xmm0
142318611: mulss xmm1, xmm0
142318615: movsd xmm2, qword ptr [rbp + 0x70]
14231861a: movaps xmm3, xmm2
14231861d: mulps xmm3, xmm2
142318620: movshdup xmm4, xmm3
142318624: addss xmm4, xmm3
142318628: addss xmm4, xmm1
14231862c: xorps xmm1, xmm1
14231862f: sqrtss xmm1, xmm4
142318633: movaps xmm3, xmm12
142318637: divss xmm3, xmm1
14231863b: movsldup xmm1, xmm3
14231863f: mulps xmm1, xmm2
142318642: mulss xmm3, xmm0
142318646: mov rax, qword ptr [rbp + 0xb0]
14231864d: mov qword ptr [rbp + 0xc0], rax
142318654: mov eax, dword ptr [rbp + 0xb8]
14231865a: mov dword ptr [rbp + 0xc8], eax
142318660: movlps qword ptr [rbp + 0xcc], xmm1
142318667: movss dword ptr [rbp + 0xd4], xmm3
14231866f: movss xmm3, dword ptr [rip + 0x6fe641]
142318677: test r12b, r12b
14231867a: jne 0x142318684
14231867c: movss xmm3, dword ptr [rip + 0x60d1ec]
142318684: lea rcx, [rbp - 0x30]
142318688: mov rdx, qword ptr [rbp + 0xa8]
14231868f: lea r8, [rbp + 0xc0]
142318696: call 0x1408ccc90 ; country_core::systems::collision::world::RaycastWorld::legacy_spherecast
14231869b: cmp dword ptr [rbp], 2
14231869f: je 0x142318809
1423186a5: movss xmm0, dword ptr [rbp - 0x14]
1423186aa: subss xmm0, dword ptr [rbx + 0x14]
1423186af: movups xmm1, xmmword ptr [rbp - 0x18]
1423186b3: mulss xmm0, xmm0
1423186b7: movss xmm2, dword ptr [rbp - 0x10]
1423186bc: movlhps xmm2, xmm1
1423186bf: shufps xmm2, xmm1, 0xe2
1423186c3: movss xmm1, dword ptr [rbx + 0x10]
1423186c8: movss xmm3, dword ptr [rbx + 0x18]
1423186cd: unpcklps xmm1, xmm3
1423186d0: subps xmm2, xmm1
1423186d3: mulps xmm2, xmm2
1423186d6: addss xmm0, xmm2
1423186da: movshdup xmm1, xmm2
1423186de: addss xmm1, xmm0
1423186e2: xorps xmm0, xmm0
1423186e5: sqrtss xmm0, xmm1
1423186e9: ucomiss xmm12, xmm0
1423186ed: jbe 0x142318809
1423186f3: movss xmm1, dword ptr [rip + 0x60d275]
1423186fb: ucomiss xmm1, xmm0
1423186fe: jbe 0x1423187f4
142318704: test byte ptr [rbp + 0x15c], 1
14231870b: je 0x1423187f4
142318711: mov rcx, qword ptr [rbp + 0x298]
142318718: mov rdx, qword ptr [rbp + 0x98]
14231871f: lea r8, [rip + 0xd32972]
142318726: call 0x14261f3e0 ; bevy_ecs::event::writer::EventWriter::write<system_critters::spawn_ducks::DespawnDuckCmd>
14231872b: jmp 0x1423187f4
142318730: movss xmm1, dword ptr [rbx + 0x10]
142318735: movss xmm2, dword ptr [rbx + 0x18]
14231873a: mov rcx, qword ptr [rbp + 0xa0]
142318741: call 0x140aa5730 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
142318746: mov r15, rax
142318749: imul r12, rdx, 0x38
14231874d: add r12, rax
142318750: cmp r15, r12
142318753: je 0x142318809
142318759: mov rsi, r15
14231875c: mov rdx, qword ptr [r15 + 0x28]
142318760: mov rcx, qword ptr [rbp + 0x80]
142318767: lea r8, [rip + 0xd328fa]
14231876e: call 0x140a91ef0 ; country_core::resources::walls::public_walls::PublicWalls::get
142318773: test rax, rax
142318776: je 0x142318945
14231877c: lea r15, [rsi + 0x38]
142318780: mov rcx, qword ptr [rbp + 0x150]
142318787: cmp dword ptr [rcx], 3
14231878a: je 0x14231879e
14231878c: mov rcx, rax
14231878f: call 0x140b2a680 ; country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
142318794: movss xmm1, dword ptr [rbx + 0x14]
142318799: ucomiss xmm1, xmm0
14231879c: ja 0x142318750
14231879e: movss xmm1, dword ptr [rsi]
1423187a2: movss xmm2, dword ptr [rsi + 4]
1423187a7: movss xmm3, dword ptr [rsi + 8]
1423187ac: movss xmm0, dword ptr [rsi + 0xc]
1423187b1: movss dword ptr [rsp + 0x20], xmm0
1423187b7: lea rcx, [rbp + 0xc0]
1423187be: call 0x140c97560 ; utils::geometry::circle::Circle2d::new_rotated
1423187c3: lea rcx, [rbp - 0x30]
1423187c7: lea rdx, [rbp + 0xc0]
1423187ce: call 0x14261f450 ; utils::geometry::line_relation::LineInterval::line_segment
1423187d3: lea rcx, [rbp + 0xc0]
1423187da: lea rdx, [rbp - 0x48]
1423187de: lea r8, [rbp - 0x30]
1423187e2: call 0x14261f470 ; utils::geometry::line_relation::LineInterval::relate
1423187e7: cmp dword ptr [rbp + 0xc0], 0
1423187ee: jne 0x142318750
1423187f4: mov eax, dword ptr [rbp + 0x158]
1423187fa: mov rcx, qword ptr [rbp + 0x120]
142318801: mov dword ptr [rcx], eax
142318803: mov dword ptr [rdi], 0
142318809: mov r15, qword ptr [rbp + 0x110]
142318810: mov r12, qword ptr [rbp + 0x118]
142318817: jmp 0x142317e4f
14231881c: cmp qword ptr [rbp + 0xd8], 0
142318824: je 0x142318871
142318826: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14231882b: mov rcx, rax
14231882e: mov rax, qword ptr [rax]
142318831: cmp rax, 1
142318835: jne 0x1423188d3
14231883b: add rcx, 8
14231883f: cmp qword ptr [rcx], 0
142318843: jne 0x142318911
142318849: mov qword ptr [rcx], 0xffffffffffffffff
142318850: mov qword ptr [rbp + 0x150], rcx
142318857: add rcx, 8
14231885b: mov rdx, qword ptr [rbp + 0xe0]
142318862: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
142318867: mov rax, qword ptr [rbp + 0x150]
14231886e: inc qword ptr [rax]
142318871: movaps xmm6, xmmword ptr [rbp + 0x170]
142318878: movaps xmm7, xmmword ptr [rbp + 0x180]
14231887f: movaps xmm8, xmmword ptr [rbp + 0x190]
142318887: movaps xmm9, xmmword ptr [rbp + 0x1a0]
14231888f: movaps xmm10, xmmword ptr [rbp + 0x1b0]
142318897: movaps xmm11, xmmword ptr [rbp + 0x1c0]
14231889f: movaps xmm12, xmmword ptr [rbp + 0x1d0]
1423188a7: movaps xmm13, xmmword ptr [rbp + 0x1e0]
1423188af: movaps xmm14, xmmword ptr [rbp + 0x1f0]
1423188b7: movaps xmm15, xmmword ptr [rbp + 0x200]
1423188bf: add rsp, 0x298
1423188c6: pop rbx
1423188c7: pop rdi
1423188c8: pop rsi
1423188c9: pop r12
1423188cb: pop r13
1423188cd: pop r14
1423188cf: pop r15
1423188d1: pop rbp
1423188d2: ret 
1423188d3: test rax, rax
1423188d6: jne 0x142318905
1423188d8: xor edx, edx
1423188da: call 0x14291b030 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1423188df: mov rcx, rax
1423188e2: test rax, rax
1423188e5: jne 0x14231883f
1423188eb: jmp 0x142318905
1423188ed: test rax, rax
1423188f0: jne 0x142318905
1423188f2: xor edx, edx
1423188f4: call 0x14291b030 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1423188f9: mov rcx, rax
1423188fc: test rax, rax
1423188ff: jne 0x142317c16
142318905: lea rcx, [rip + 0xd317c4]
14231890c: call 0x1428d80d0 ; std::thread::local::panic_access_error
142318911: lea rcx, [rip + 0xd31828]
142318918: call 0x1428d9250 ; core::cell::panic_already_borrowed
14231891d: mov qword ptr [rbp - 0x30], 0
142318925: lea rax, [rip + 0xd31994]
14231892c: mov qword ptr [rsp + 0x20], rax
142318931: lea rdx, [rbp + 0x10]
142318935: lea r9, [rbp - 0x30]
142318939: xor ecx, ecx
14231893b: mov r8, r15
14231893e: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
142318943: jmp 0x14231895f
142318945: lea rcx, [rip + 0xd32734]
14231894c: call 0x1428d9310 ; core::option::unwrap_failed
142318951: jmp 0x14231895f
142318953: lea rcx, [rip + 0xd326f6]
14231895a: call 0x1428d9310 ; core::option::unwrap_failed
14231895f: ud2 
142318961: nop word ptr cs:[rax + rax]
142318970: mov qword ptr [rsp + 0x10], rdx
142318975: push rbp
142318976: push r15
142318978: push r14
14231897a: push r13
14231897c: push r12
14231897e: push rsi
14231897f: push rdi
142318980: push rbx
142318981: sub rsp, 0xd8
142318988: lea rbp, [rdx + 0x80]
14231898f: movaps xmmword ptr [rsp + 0x30], xmm15
142318995: movaps xmmword ptr [rsp + 0x40], xmm14
14231899b: movaps xmmword ptr [rsp + 0x50], xmm13
1423189a1: movaps xmmword ptr [rsp + 0x60], xmm12
1423189a7: movaps xmmword ptr [rsp + 0x70], xmm11
1423189ad: movaps xmmword ptr [rsp + 0x80], xmm10
1423189b6: movaps xmmword ptr [rsp + 0x90], xmm9
1423189bf: movaps xmmword ptr [rsp + 0xa0], xmm8
1423189c8: movaps xmmword ptr [rsp + 0xb0], xmm7
1423189d0: movaps xmmword ptr [rsp + 0xc0], xmm6
1423189d8: mov rax, qword ptr [rbp + 0x130]
1423189df: inc qword ptr [rax]
1423189e2: movaps xmm6, xmmword ptr [rsp + 0xc0]
1423189ea: movaps xmm7, xmmword ptr [rsp + 0xb0]
1423189f2: movaps xmm8, xmmword ptr [rsp + 0xa0]
1423189fb: movaps xmm9, xmmword ptr [rsp + 0x90]
142318a04: movaps xmm10, xmmword ptr [rsp + 0x80]
142318a0d: movaps xmm11, xmmword ptr [rsp + 0x70]
142318a13: movaps xmm12, xmmword ptr [rsp + 0x60]
142318a19: movaps xmm13, xmmword ptr [rsp + 0x50]
142318a1f: movaps xmm14, xmmword ptr [rsp + 0x40]
142318a25: movaps xmm15, xmmword ptr [rsp + 0x30]
142318a2b: add rsp, 0xd8
142318a32: pop rbx
142318a33: pop rdi
142318a34: pop rsi
142318a35: pop r12
142318a37: pop r13
142318a39: pop r14
142318a3b: pop r15
142318a3d: pop rbp
142318a3e: ret 
142318a3f: nop 
142318a40: mov qword ptr [rsp + 0x10], rdx
142318a45: push rbp
142318a46: push r15
142318a48: push r14
142318a4a: push r13
142318a4c: push r12
142318a4e: push rsi
142318a4f: push rdi
142318a50: push rbx
142318a51: sub rsp, 0xd8
142318a58: lea rbp, [rdx + 0x80]
142318a5f: movaps xmmword ptr [rsp + 0x30], xmm15
142318a65: movaps xmmword ptr [rsp + 0x40], xmm14
142318a6b: movaps xmmword ptr [rsp + 0x50], xmm13
142318a71: movaps xmmword ptr [rsp + 0x60], xmm12
142318a77: movaps xmmword ptr [rsp + 0x70], xmm11
142318a7d: movaps xmmword ptr [rsp + 0x80], xmm10
142318a86: movaps xmmword ptr [rsp + 0x90], xmm9
142318a8f: movaps xmmword ptr [rsp + 0xa0], xmm8
142318a98: movaps xmmword ptr [rsp + 0xb0], xmm7
142318aa0: movaps xmmword ptr [rsp + 0xc0], xmm6
142318aa8: mov rax, qword ptr [rbp + 0x150]
142318aaf: inc qword ptr [rax]
142318ab2: movaps xmm6, xmmword ptr [rsp + 0xc0]
142318aba: movaps xmm7, xmmword ptr [rsp + 0xb0]
142318ac2: movaps xmm8, xmmword ptr [rsp + 0xa0]
142318acb: movaps xmm9, xmmword ptr [rsp + 0x90]
142318ad4: movaps xmm10, xmmword ptr [rsp + 0x80]
142318add: movaps xmm11, xmmword ptr [rsp + 0x70]
142318ae3: movaps xmm12, xmmword ptr [rsp + 0x60]
142318ae9: movaps xmm13, xmmword ptr [rsp + 0x50]
142318aef: movaps xmm14, xmmword ptr [rsp + 0x40]
142318af5: movaps xmm15, xmmword ptr [rsp + 0x30]
142318afb: add rsp, 0xd8
142318b02: pop rbx
142318b03: pop rdi
142318b04: pop rsi
142318b05: pop r12
142318b07: pop r13
142318b09: pop r14
142318b0b: pop r15
142318b0d: pop rbp
142318b0e: ret 
142318b0f: nop 
142318b10: mov qword ptr [rsp + 0x10], rdx
142318b15: push rbp
142318b16: push r15
142318b18: push r14
142318b1a: push r13
142318b1c: push r12
142318b1e: push rsi
142318b1f: push rdi
142318b20: push rbx
142318b21: sub rsp, 0xd8
142318b28: lea rbp, [rdx + 0x80]
142318b2f: movaps xmmword ptr [rsp + 0x30], xmm15
142318b35: movaps xmmword ptr [rsp + 0x40], xmm14
142318b3b: movaps xmmword ptr [rsp + 0x50], xmm13
142318b41: movaps xmmword ptr [rsp + 0x60], xmm12
142318b47: movaps xmmword ptr [rsp + 0x70], xmm11
142318b4d: movaps xmmword ptr [rsp + 0x80], xmm10
142318b56: movaps xmmword ptr [rsp + 0x90], xmm9
142318b5f: movaps xmmword ptr [rsp + 0xa0], xmm8
142318b68: movaps xmmword ptr [rsp + 0xb0], xmm7
142318b70: movaps xmmword ptr [rsp + 0xc0], xmm6
142318b78: cmp qword ptr [rbp + 0xd8], 0
142318b80: je 0x142318ba5
142318b82: lea rax, [rbp + 0xe0]
142318b89: mov qword ptr [rbp + 0x68], rax
142318b8d: lea rax, [rbp + 0x68]
142318b91: mov qword ptr [rbp + 0x60], rax
142318b95: lea rcx, [rip + 0xd323c4]
142318b9c: lea rdx, [rbp + 0x60]
142318ba0: call 0x14230e5f0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
142318ba5: movaps xmm6, xmmword ptr [rsp + 0xc0]
142318bad: movaps xmm7, xmmword ptr [rsp + 0xb0]
142318bb5: movaps xmm8, xmmword ptr [rsp + 0xa0]
142318bbe: movaps xmm9, xmmword ptr [rsp + 0x90]
142318bc7: movaps xmm10, xmmword ptr [rsp + 0x80]
142318bd0: movaps xmm11, xmmword ptr [rsp + 0x70]
142318bd6: movaps xmm12, xmmword ptr [rsp + 0x60]
142318bdc: movaps xmm13, xmmword ptr [rsp + 0x50]
142318be2: movaps xmm14, xmmword ptr [rsp + 0x40]
142318be8: movaps xmm15, xmmword ptr [rsp + 0x30]
142318bee: add rsp, 0xd8
142318bf5: pop rbx
142318bf6: pop rdi
142318bf7: pop rsi
142318bf8: pop r12
142318bfa: pop r13
142318bfc: pop r14
142318bfe: pop r15
142318c00: pop rbp
142318c01: ret 
