14146d030: push rbp
14146d031: push r15
14146d033: push r14
14146d035: push r13
14146d037: push r12
14146d039: push rsi
14146d03a: push rdi
14146d03b: push rbx
14146d03c: sub rsp, 0xa58
14146d043: lea rbp, [rsp + 0x80]
14146d04b: movaps xmmword ptr [rbp + 0x9c0], xmm10
14146d053: movaps xmmword ptr [rbp + 0x9b0], xmm9
14146d05b: movaps xmmword ptr [rbp + 0x9a0], xmm8
14146d063: movaps xmmword ptr [rbp + 0x990], xmm7
14146d06a: movaps xmmword ptr [rbp + 0x980], xmm6
14146d071: mov qword ptr [rbp + 0x978], 0xfffffffffffffffe
14146d07c: mov qword ptr [rbp + 0x960], r9
14146d083: mov qword ptr [rbp + 0x8e0], r8
14146d08a: mov qword ptr [rbp + 0x958], rdx
14146d091: mov qword ptr [rbp + 0x938], rcx
14146d098: mov rdi, qword ptr [rbp + 0xa80]
14146d09f: mov rsi, qword ptr [rdi]
14146d0a2: mov r13, qword ptr [rdi + 8]
14146d0a6: mov rax, qword ptr [rsi]
14146d0a9: xor ecx, ecx
14146d0ab: mov r10, rax
14146d0ae: sub r10, qword ptr [r13 + 0x18]
14146d0b2: cmovb r10, rcx
14146d0b6: sub rax, qword ptr [r13 + 0x38]
14146d0ba: cmovb rax, rcx
14146d0be: mov r12, qword ptr [r13 + 0x10]
14146d0c2: mov rdx, qword ptr [r13 + 0x30]
14146d0c6: mov qword ptr [rbp + 0x910], rdx
14146d0cd: sub rdx, rax
14146d0d0: cmovb rdx, rcx
14146d0d4: setbe r8b
14146d0d8: mov qword ptr [rbp + 0x950], r12
14146d0df: sub r12, r10
14146d0e2: cmovb r12, rcx
14146d0e6: setbe r9b
14146d0ea: test r9b, r8b
14146d0ed: lea r11, [rdx + r12]
14146d0f1: cmove r11, rcx
14146d0f5: mov rcx, qword ptr [r13 + 0x40]
14146d0f9: sub rcx, r11
14146d0fc: mov rbx, qword ptr [rbp + 0xa78]
14146d103: movsd xmm6, qword ptr [rbx + 0x90]
14146d10b: mov r11, qword ptr [rbx]
14146d10e: mov qword ptr [rbp + 0x8a8], r11
14146d115: mov r11, qword ptr [rbx + 0x10]
14146d119: mov qword ptr [rbp + 0x560], r11
14146d120: mov r11d, dword ptr [rbx + 0x1c]
14146d124: mov dword ptr [rbp + 0x8c0], r11d
14146d12b: mov r11, qword ptr [rbx + 0x20]
14146d12f: mov qword ptr [rbp + 0x558], r11
14146d136: mov r11, qword ptr [rbx + 0x30]
14146d13a: mov qword ptr [rbp + 0x8d8], r11
14146d141: mov r11d, dword ptr [rbx + 0x3c]
14146d145: mov dword ptr [rbp + 0x90c], r11d
14146d14c: mov r11, qword ptr [rbx + 0x40]
14146d150: mov qword ptr [rbp + 0x918], r11
14146d157: mov r11, qword ptr [rbx + 0x50]
14146d15b: mov qword ptr [rbp + 0x570], r11
14146d162: mov r11d, dword ptr [rbx + 0x5c]
14146d166: mov dword ptr [rbp + 0x8c4], r11d
14146d16d: mov r11, qword ptr [rbx + 0x60]
14146d171: mov qword ptr [rbp + 0x580], r11
14146d178: mov r11, qword ptr [rbx + 0x70]
14146d17c: mov qword ptr [rbp + 0x8b0], r11
14146d183: mov r11d, dword ptr [rbx + 0x7c]
14146d187: mov dword ptr [rbp + 0x8b8], r11d
14146d18e: mov r11, qword ptr [rbx + 0x80]
14146d195: mov qword ptr [rbp + 0x870], r11
14146d19c: mov r11, qword ptr [rbx + 0x88]
14146d1a3: mov qword ptr [rbp + 0x8f8], r11
14146d1aa: mov r11, qword ptr [rbx + 0x98]
14146d1b1: mov qword ptr [rbp + 0x568], r11
14146d1b8: mov r11, qword ptr [rbx + 0xa8]
14146d1bf: mov qword ptr [rbp + 0x528], r11
14146d1c6: mov r11d, dword ptr [rbx + 0xb4]
14146d1cd: mov dword ptr [rbp + 0x85c], r11d
14146d1d4: mov r11, qword ptr [rbx + 0xb8]
14146d1db: mov qword ptr [rbp + 0x878], r11
14146d1e2: mov r11, qword ptr [rbx + 0xc8]
14146d1e9: mov qword ptr [rbp + 0x530], r11
14146d1f0: mov r11d, dword ptr [rbx + 0xd4]
14146d1f7: mov dword ptr [rbp + 0x860], r11d
14146d1fe: mov r15, qword ptr [rdi + 0x28]
14146d202: mov r14, qword ptr [rdi + 0x38]
14146d206: mov ebx, dword ptr [rdi + 0x44]
14146d209: mov r11, qword ptr [rdi + 0x48]
14146d20d: mov qword ptr [rbp + 0x520], r11
14146d214: mov r11, qword ptr [rdi + 0x58]
14146d218: mov qword ptr [rbp + 0x578], r11
14146d21f: mov r11d, dword ptr [rdi + 0x64]
14146d223: mov dword ptr [rbp + 0x8c8], r11d
14146d22a: mov r11, qword ptr [rdi + 0x68]
14146d22e: mov qword ptr [rbp + 0x928], r11
14146d235: mov r11, qword ptr [rdi + 0x78]
14146d239: mov qword ptr [rbp + 0x900], r11
14146d240: mov r11d, dword ptr [rdi + 0x84]
14146d247: mov dword ptr [rbp + 0x948], r11d
14146d24e: mov r11, qword ptr [rdi + 0x88]
14146d255: mov qword ptr [rbp + 0x538], r11
14146d25c: mov r11, qword ptr [rdi + 0x98]
14146d263: mov qword ptr [rbp + 0x540], r11
14146d26a: mov r11d, dword ptr [rdi + 0xa4]
14146d271: mov dword ptr [rbp + 0x864], r11d
14146d278: mov r11, qword ptr [r13 + 8]
14146d27c: mov rdi, qword ptr [r13 + 0x28]
14146d280: mov qword ptr [rsi], rcx
14146d283: test r9b, r8b
14146d286: jne 0x14146d396
14146d28c: xor ecx, ecx
14146d28e: cmp qword ptr [rbp + 0x950], r10
14146d295: lea r8, [r10 + r10*4]
14146d299: lea r8, [r11 + r8*8]
14146d29d: mov r9d, 8
14146d2a3: cmovb r8, r9
14146d2a7: lea r10, [r12 + r12*4]
14146d2ab: lea r8, [r8 + r10*8 - 0x28]
14146d2b0: cmova rcx, r8
14146d2b4: cmp qword ptr [rbp + 0x910], rax
14146d2bb: lea rax, [rax + rax*4]
14146d2bf: lea rax, [rdi + rax*8]
14146d2c3: cmovb rax, r9
14146d2c7: lea rdx, [rdx + rdx*4]
14146d2cb: lea rax, [rax + rdx*8 - 0x28]
14146d2d0: cmovbe rax, rcx
14146d2d4: cmp dword ptr [rax], 0xe
14146d2d7: jne 0x14146d396
14146d2dd: mov rax, qword ptr [rbp + 0x958]
14146d2e4: mov rsi, qword ptr [rax]
14146d2e7: movabs rax, 0x8000000000000001
14146d2f1: cmp qword ptr [rsi + 0x18], rax
14146d2f5: jne 0x14146d38c
14146d2fb: lea rcx, [rsi + 0x20]
14146d2ff: mov qword ptr [rbp + 0x518], rcx
14146d306: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
14146d30b: mov r8, rdx
14146d30e: lea rcx, [rbp + 0x5b0]
14146d315: mov rdx, rax
14146d318: call 0x140d6a710 ; std::sys::pal::windows::fs::stat
14146d31d: cmp dword ptr [rbp + 0x5b0], 2
14146d324: je 0x14146f8bd
14146d32a: movups xmm0, xmmword ptr [rbp + 0x5b0]
14146d331: movups xmm1, xmmword ptr [rbp + 0x5c0]
14146d338: movups xmm2, xmmword ptr [rbp + 0x5d0]
14146d33f: movups xmm3, xmmword ptr [rbp + 0x5e0]
14146d346: movaps xmmword ptr [rbp + 0x360], xmm0
14146d34d: mov rax, qword ptr [rbp + 0x600]
14146d354: mov qword ptr [rbp + 0x3b0], rax
14146d35b: movups xmm0, xmmword ptr [rbp + 0x5f0]
14146d362: movaps xmmword ptr [rbp + 0x3a0], xmm0
14146d369: movaps xmmword ptr [rbp + 0x390], xmm3
14146d370: movaps xmmword ptr [rbp + 0x380], xmm2
14146d377: movaps xmmword ptr [rbp + 0x370], xmm1
14146d37e: cmp qword ptr [rbp + 0x360], 2
14146d386: je 0x14146f8d6
14146d38c: cmp byte ptr [rsi + 0x48], 0
14146d390: jne 0x14146f8a5
14146d396: mov rax, qword ptr [rbp + 0x938]
14146d39d: mov rax, qword ptr [rax]
14146d3a0: cmp dword ptr [rax], 0xe
14146d3a3: jne 0x14146f625
14146d3a9: mov rax, qword ptr [rbp + 0x958]
14146d3b0: mov r8, qword ptr [rax]
14146d3b3: movzx eax, byte ptr [r8 + 0x48]
14146d3b8: test eax, eax
14146d3ba: je 0x14146d5c4
14146d3c0: cmp eax, 2
14146d3c3: jne 0x14146d868
14146d3c9: mov rcx, qword ptr [rbp + 0x958]
14146d3d0: mov eax, dword ptr [rcx + 0x1c]
14146d3d3: mov rcx, qword ptr [rcx + 0x10]
14146d3d7: mov dword ptr [rcx], eax
14146d3d9: mov rcx, qword ptr [r8]
14146d3dc: mov rax, qword ptr [r8 + 8]
14146d3e0: mov rdx, qword ptr [r8 + 0x10]
14146d3e4: mov qword ptr [r8], 0
14146d3eb: mov qword ptr [r8 + 8], 8
14146d3f3: mov qword ptr [r8 + 0x10], 0
14146d3fb: mov rbx, rdx
14146d3fe: shl rbx, 5
14146d402: add rbx, rax
14146d405: mov qword ptr [rbp + 0x360], rax
14146d40c: mov qword ptr [rbp + 0x368], rax
14146d413: mov qword ptr [rbp + 0x370], rcx
14146d41a: mov qword ptr [rbp + 0x378], rbx
14146d421: test rdx, rdx
14146d424: je 0x14146d8fc
14146d42a: mov rcx, qword ptr [rbp + 0x8e0]
14146d431: mov edx, dword ptr [rcx + 0x1c]
14146d434: mov dword ptr [rbp + 0x958], edx
14146d43a: mov rdx, qword ptr [rcx]
14146d43d: mov qword ptr [rbp + 0x8f8], rdx
14146d444: mov rcx, qword ptr [rcx + 0x10]
14146d448: mov qword ptr [rbp + 0x938], rcx
14146d44f: mov rcx, qword ptr [rbp + 0x928]
14146d456: add rcx, 0x20
14146d45a: mov qword ptr [rbp + 0x918], rcx
14146d461: movabs r15, 0x8000000000000003
14146d46b: mov r14d, 1
14146d471: jmp 0x14146d4bb
14146d473: nop word ptr cs:[rax + rax]
14146d480: mov rdx, qword ptr [rbp + 0x928]
14146d487: mov rax, qword ptr [rdx + 0x28]
14146d48b: lea rcx, [r13*2]
14146d493: add rcx, r13
14146d496: mov qword ptr [rax + rcx*8], r12
14146d49a: movlps qword ptr [rax + rcx*8 + 8], xmm6
14146d49f: mov qword ptr [rax + rcx*8 + 0x10], rsi
14146d4a4: inc r13
14146d4a7: mov qword ptr [rdx + 0x30], r13
14146d4ab: inc qword ptr [rdx + 0x40]
14146d4af: mov rax, rdi
14146d4b2: cmp rdi, rbx
14146d4b5: je 0x14146d8fc
14146d4bb: lea rdi, [rax + 0x20]
14146d4bf: mov qword ptr [rbp + 0x368], rdi
14146d4c6: mov rcx, qword ptr [rax]
14146d4c9: cmp rcx, r15
14146d4cc: je 0x14146d8fc
14146d4d2: mov r12, qword ptr [rax + 8]
14146d4d6: movups xmm6, xmmword ptr [rax + 0x10]
14146d4da: mov rax, rcx
14146d4dd: movabs rdx, 0x8000000000000000
14146d4e7: xor rax, rdx
14146d4ea: cmp rax, 3
14146d4ee: cmovae rax, r14
14146d4f2: test rax, rax
14146d4f5: je 0x14146d550
14146d4f7: cmp rax, 1
14146d4fb: jne 0x14146d590
14146d501: mov eax, dword ptr [rbp + 0x958]
14146d507: mov rdx, qword ptr [rbp + 0x938]
14146d50e: mov dword ptr [rdx], eax
14146d510: mov qword ptr [rbp + 0x5b0], rcx
14146d517: mov qword ptr [rbp + 0x5b8], r12
14146d51e: movups xmmword ptr [rbp + 0x5c0], xmm6
14146d525: mov rcx, qword ptr [rbp + 0x8f8]
14146d52c: lea rdx, [rbp + 0x5b0]
14146d533: lea r8, [rip + 0x17ed8fe]
14146d53a: call 0x140b5a180 ; alloc::collections::vec_deque::spec_extend::impl$1::spec_extend<enum2$<country_core::meta_commander::MetaCommand>,alloc::collections::vec_deque::into_iter::IntoIter<enum2$<country_core::meta_commander::MetaCommand>,alloc::alloc::Global>,alloc::alloc::Global>
14146d53f: jmp 0x14146d4af
14146d544: nop word ptr cs:[rax + rax]
14146d550: mov rax, qword ptr [rbp + 0x900]
14146d557: mov ecx, dword ptr [rbp + 0x948]
14146d55d: mov dword ptr [rax], ecx
14146d55f: mov rax, qword ptr [rbp + 0x928]
14146d566: mov r13, qword ptr [rax + 0x30]
14146d56a: mov rsi, qword ptr [rax + 0x40]
14146d56e: cmp r13, qword ptr [rax + 0x20]
14146d572: jne 0x14146d480
14146d578: mov rcx, qword ptr [rbp + 0x918]
14146d57f: lea rdx, [rip + 0x17ccbc2]
14146d586: call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<relative_path::RelativePathBuf,alloc::alloc::Global>
14146d58b: jmp 0x14146d480
14146d590: mov qword ptr [rbp + 0x5b0], r12
14146d597: xor eax, eax
14146d599: xchg byte ptr [r12 + 0x10], al
14146d59e: lock dec qword ptr [r12]
14146d5a3: jne 0x14146d5b1
14146d5a5: lea rcx, [rbp + 0x5b0]
14146d5ac: call 0x1402e5d80 ; alloc::sync::Arc::drop_slow<core::sync::atomic::AtomicBool,alloc::alloc::Global>
14146d5b1: mov rdi, qword ptr [rbp + 0x368]
14146d5b8: mov rbx, qword ptr [rbp + 0x378]
14146d5bf: jmp 0x14146d4af
14146d5c4: mov rax, qword ptr [r8 + 0x50]
14146d5c8: mov qword ptr [rbp + 0x550], rax
14146d5cf: movzx eax, byte ptr [r8 + 0x49]
14146d5d4: mov byte ptr [rbp + 0x928], al
14146d5da: movzx ebx, byte ptr [r8 + 0x4a]
14146d5df: mov qword ptr [rbp + 0x900], r8
14146d5e6: movzx r14d, byte ptr [r8 + 0x4b]
14146d5eb: mov rax, qword ptr [rip + 0x21bedb6]
14146d5f2: mov rax, qword ptr [rax]
14146d5f5: cmp rax, 4
14146d5f9: mov r13, qword ptr [rbp + 0x520]
14146d600: mov rsi, qword ptr [rbp + 0x960]
14146d607: jb 0x14146d6e9
14146d60d: movabs rdi, 0x8000000000000000
14146d617: mov qword ptr [rbp + 0x930], rdi
14146d61e: mov byte ptr [rbp + 0x976], 1
14146d625: lea rcx, [rip + 0x17f6b04]
14146d62c: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14146d631: movups xmm0, xmmword ptr [rax]
14146d634: mov eax, dword ptr [rax + 0x10]
14146d637: mov qword ptr [rbp + 0x5e0], 4
14146d642: lea rcx, [rip + 0x17f6aff]
14146d649: mov qword ptr [rbp + 0x5e8], rcx
14146d650: mov qword ptr [rbp + 0x5f0], 0x26
14146d65b: lea rdx, [rip + 0x17f6abe]
14146d662: mov qword ptr [rbp + 0x600], rdx
14146d669: mov qword ptr [rbp + 0x608], 1
14146d674: mov qword ptr [rbp + 0x610], 8
14146d67f: mov qword ptr [rbp + 0x5b0], 0
14146d68a: xorps xmm1, xmm1
14146d68d: movups xmmword ptr [rbp + 0x618], xmm1
14146d694: mov qword ptr [rbp + 0x5b8], rcx
14146d69b: mov qword ptr [rbp + 0x5c0], 0x26
14146d6a6: mov qword ptr [rbp + 0x5c8], 0
14146d6b1: movups xmmword ptr [rbp + 0x5d0], xmm0
14146d6b8: mov dword ptr [rbp + 0x5f8], 1
14146d6c2: mov dword ptr [rbp + 0x5fc], eax
14146d6c8: mov qword ptr [rbp + 0x930], rdi
14146d6cf: mov byte ptr [rbp + 0x976], 1
14146d6d6: lea rcx, [rbp + 0x972]
14146d6dd: lea rdx, [rbp + 0x5b0]
14146d6e4: call 0x140d4a330 ; log::__private_api::impl$2::log
14146d6e9: mov rdx, qword ptr [rsi]
14146d6ec: add rdx, 0x60
14146d6f0: movabs rcx, 0x8000000000000000
14146d6fa: mov qword ptr [rbp + 0x8e8], rcx
14146d701: mov qword ptr [rbp + 0x930], rcx
14146d708: mov byte ptr [rbp + 0x976], 1
14146d70f: lea rcx, [rbp + 0x360]
14146d716: mov qword ptr [rbp + 0x8e0], rdx
14146d71d: call 0x140b46000 ; country_core::resources::session::SessionId::save_folder_path
14146d722: lea rcx, [rbp + 0x360]
14146d729: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
14146d72e: mov r8, rdx
14146d731: mov qword ptr [rsp + 0x20], 0xb
14146d73a: lea r9, [rip + 0x17f6a2d]
14146d741: lea rcx, [rbp + 0x1b0]
14146d748: mov rdx, rax
14146d74b: call 0x140d62810 ; std::path::Path::_join
14146d750: mov dword ptr [rbp + 0x5b0], 0
14146d75a: xorps xmm0, xmm0
14146d75d: movups xmmword ptr [rbp + 0x5b8], xmm0
14146d764: mov dword ptr [rbp + 0x5c8], 7
14146d76e: mov qword ptr [rbp + 0x5cc], 0
14146d779: mov word ptr [rbp + 0x5d4], 0
14146d782: mov byte ptr [rbp + 0x5d1], 1
14146d789: mov byte ptr [rbp + 0x5d4], 1
14146d790: mov byte ptr [rbp + 0x5d3], 1
14146d797: lea rcx, [rbp + 0x1b0]
14146d79e: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
14146d7a3: mov r8, rdx
14146d7a6: lea rcx, [rbp + 0x5b0]
14146d7ad: mov rdx, rax
14146d7b0: call 0x140d59dc0 ; std::fs::OpenOptions::_open
14146d7b5: mov r12, rax
14146d7b8: mov r15, rdx
14146d7bb: mov rdx, qword ptr [rbp + 0x1b0]
14146d7c2: test rdx, rdx
14146d7c5: je 0x14146d7d9
14146d7c7: mov rcx, qword ptr [rbp + 0x1b8]
14146d7ce: mov r8d, 1
14146d7d4: call 0x140613c20 ; 
14146d7d9: test r12, r12
14146d7dc: je 0x14146d9c8
14146d7e2: mov eax, r15d
14146d7e5: and eax, 3
14146d7e8: cmp eax, 1
14146d7eb: mov r12, qword ptr [rbp + 0x900]
14146d7f2: jne 0x14146d9d8
14146d7f8: mov rax, r15
14146d7fb: dec rax
14146d7fe: mov qword ptr [rbp + 0x910], rax
14146d805: mov rax, qword ptr [r15 - 1]
14146d809: mov qword ptr [rbp + 0x938], rax
14146d810: mov rax, qword ptr [r15 + 7]
14146d814: mov qword ptr [rbp + 0x948], rax
14146d81b: mov rax, qword ptr [rax]
14146d81e: test rax, rax
14146d821: je 0x14146d82c
14146d823: mov rcx, qword ptr [rbp + 0x938]
14146d82a: call rax
14146d82c: mov rcx, qword ptr [rbp + 0x938]
14146d833: mov rax, qword ptr [rbp + 0x948]
14146d83a: mov rdx, qword ptr [rax + 8]
14146d83e: test rdx, rdx
14146d841: je 0x14146d84c
14146d843: mov r8, qword ptr [rax + 0x10]
14146d847: call 0x140613c20 ; 
14146d84c: mov edx, 0x18
14146d851: mov r8d, 8
14146d857: mov rcx, qword ptr [rbp + 0x910]
14146d85e: call 0x140613c20 ; 
14146d863: jmp 0x14146d9d8
14146d868: mov qword ptr [rbp + 0x900], r8
14146d86f: mov r12d, dword ptr [r8 + 0x4c]
14146d873: test r12d, r12d
14146d876: js 0x14146d925
14146d87c: lea rax, [r15 + 0x20]
14146d880: mov qword ptr [rbp + 0x928], rax
14146d887: xor r13d, r13d
14146d88a: jmp 0x14146d8b5
14146d88c: nop dword ptr [rax]
14146d890: inc r13d
14146d893: mov rax, qword ptr [r15 + 0x28]
14146d897: mov rcx, rdi
14146d89a: shl rcx, 4
14146d89e: mov qword ptr [rax + rcx], rsi
14146d8a2: mov dword ptr [rax + rcx + 8], 0x3010101
14146d8aa: inc rdi
14146d8ad: mov qword ptr [r15 + 0x30], rdi
14146d8b1: inc qword ptr [r15 + 0x40]
14146d8b5: cmp r13d, r12d
14146d8b8: jge 0x14146d9a8
14146d8be: mov dword ptr [r14], ebx
14146d8c1: mov rdi, qword ptr [r15 + 0x30]
14146d8c5: mov rsi, qword ptr [r15 + 0x40]
14146d8c9: cmp rdi, qword ptr [r15 + 0x20]
14146d8cd: jne 0x14146d890
14146d8cf: movabs rax, 0x8000000000000000
14146d8d9: mov qword ptr [rbp + 0x930], rax
14146d8e0: mov byte ptr [rbp + 0x976], 1
14146d8e7: mov rcx, qword ptr [rbp + 0x928]
14146d8ee: lea rdx, [rip + 0x17cc853]
14146d8f5: call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<core::option::Option<bevy_ecs::entity::Entity> >,bool>,alloc::alloc::Global>
14146d8fa: jmp 0x14146d890
14146d8fc: movabs rax, 0x8000000000000000
14146d906: mov qword ptr [rbp + 0x930], rax
14146d90d: mov byte ptr [rbp + 0x976], 1
14146d914: lea rcx, [rbp + 0x360]
14146d91b: call 0x14087b2f0 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<country_core::systems::save::load_game::PostGameLoadCommand>,alloc::alloc::Global> >
14146d920: jmp 0x14146f625
14146d925: neg r12d
14146d928: lea rax, [r15 + 0x20]
14146d92c: mov qword ptr [rbp + 0x928], rax
14146d933: xor r13d, r13d
14146d936: jmp 0x14146d965
14146d938: nop dword ptr [rax + rax]
14146d940: inc r13d
14146d943: mov rax, qword ptr [r15 + 0x28]
14146d947: mov rcx, rdi
14146d94a: shl rcx, 4
14146d94e: mov qword ptr [rax + rcx], rsi
14146d952: mov dword ptr [rax + rcx + 8], 0x3010001
14146d95a: inc rdi
14146d95d: mov qword ptr [r15 + 0x30], rdi
14146d961: inc qword ptr [r15 + 0x40]
14146d965: cmp r13d, r12d
14146d968: jge 0x14146d9a8
14146d96a: mov dword ptr [r14], ebx
14146d96d: mov rdi, qword ptr [r15 + 0x30]
14146d971: mov rsi, qword ptr [r15 + 0x40]
14146d975: cmp rdi, qword ptr [r15 + 0x20]
14146d979: jne 0x14146d940
14146d97b: movabs rax, 0x8000000000000000
14146d985: mov qword ptr [rbp + 0x930], rax
14146d98c: mov byte ptr [rbp + 0x976], 1
14146d993: mov rcx, qword ptr [rbp + 0x928]
14146d99a: lea rdx, [rip + 0x17cc7a7]
14146d9a1: call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<core::option::Option<bevy_ecs::entity::Entity> >,bool>,alloc::alloc::Global>
14146d9a6: jmp 0x14146d940
14146d9a8: mov rcx, qword ptr [rbp + 0x958]
14146d9af: mov eax, dword ptr [rcx + 0x1c]
14146d9b2: mov rcx, qword ptr [rcx + 0x10]
14146d9b6: mov dword ptr [rcx], eax
14146d9b8: mov rax, qword ptr [rbp + 0x900]
14146d9bf: mov byte ptr [rax + 0x48], 2
14146d9c3: jmp 0x14146f625
14146d9c8: mov rcx, r15
14146d9cb: call qword ptr [rip + 0x1d24a1f]
14146d9d1: mov r12, qword ptr [rbp + 0x900]
14146d9d8: mov rdx, qword ptr [rbp + 0x360]
14146d9df: test rdx, rdx
14146d9e2: je 0x14146d9f6
14146d9e4: mov rcx, qword ptr [rbp + 0x368]
14146d9eb: mov r8d, 1
14146d9f1: call 0x140613c20 ; 
14146d9f6: test bl, bl
14146d9f8: je 0x14146db4d
14146d9fe: mov rax, qword ptr [rip + 0x21be9a3]
14146da05: mov rax, qword ptr [rax]
14146da08: cmp rax, 3
14146da0c: jb 0x14146daee
14146da12: movabs rsi, 0x8000000000000000
14146da1c: mov qword ptr [rbp + 0x930], rsi
14146da23: mov byte ptr [rbp + 0x976], 1
14146da2a: lea rcx, [rip + 0x17f6787]
14146da31: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14146da36: movups xmm0, xmmword ptr [rax]
14146da39: mov eax, dword ptr [rax + 0x10]
14146da3c: mov qword ptr [rbp + 0x5e0], 3
14146da47: lea rcx, [rip + 0x17f66fa]
14146da4e: mov qword ptr [rbp + 0x5e8], rcx
14146da55: mov qword ptr [rbp + 0x5f0], 0x26
14146da60: lea rdx, [rip + 0x17f6741]
14146da67: mov qword ptr [rbp + 0x600], rdx
14146da6e: mov qword ptr [rbp + 0x608], 1
14146da79: mov qword ptr [rbp + 0x610], 8
14146da84: mov qword ptr [rbp + 0x5b0], 0
14146da8f: xorps xmm1, xmm1
14146da92: movups xmmword ptr [rbp + 0x618], xmm1
14146da99: mov qword ptr [rbp + 0x5b8], rcx
14146daa0: mov qword ptr [rbp + 0x5c0], 0x26
14146daab: mov qword ptr [rbp + 0x5c8], 0
14146dab6: movups xmmword ptr [rbp + 0x5d0], xmm0
14146dabd: mov dword ptr [rbp + 0x5f8], 1
14146dac7: mov dword ptr [rbp + 0x5fc], eax
14146dacd: mov qword ptr [rbp + 0x930], rsi
14146dad4: mov byte ptr [rbp + 0x976], 1
14146dadb: lea rcx, [rbp + 0x972]
14146dae2: lea rdx, [rbp + 0x5b0]
14146dae9: call 0x140d4a330 ; log::__private_api::impl$2::log
14146daee: mov rax, qword ptr [rbp + 0x578]
14146daf5: mov ecx, dword ptr [rbp + 0x8c8]
14146dafb: mov dword ptr [rax], ecx
14146dafd: mov rsi, qword ptr [r13 + 0x30]
14146db01: mov rdi, qword ptr [r13 + 0x40]
14146db05: cmp rsi, qword ptr [r13 + 0x20]
14146db09: jne 0x14146db33
14146db0b: lea rcx, [r13 + 0x20]
14146db0f: movabs rax, 0x8000000000000000
14146db19: mov qword ptr [rbp + 0x930], rax
14146db20: mov byte ptr [rbp + 0x976], 1
14146db27: lea rdx, [rip + 0x17cc61a]
14146db2e: call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
14146db33: mov rax, qword ptr [r13 + 0x28]
14146db37: mov qword ptr [rax + rsi*8], rdi
14146db3b: inc rsi
14146db3e: mov qword ptr [r13 + 0x30], rsi
14146db42: inc qword ptr [r13 + 0x40]
14146db46: mov r12, qword ptr [rbp + 0x900]
14146db4d: mov rcx, qword ptr [rbp + 0x958]
14146db54: mov eax, dword ptr [rcx + 0x1c]
14146db57: mov rcx, qword ptr [rcx + 0x10]
14146db5b: mov dword ptr [rbp + 0x8cc], eax
14146db61: mov qword ptr [rbp + 0x588], rcx
14146db68: mov dword ptr [rcx], eax
14146db6a: mov rsi, qword ptr [r12 + 0x18]
14146db6f: mov rdi, qword ptr [r12 + 0x38]
14146db74: lea rax, [r12 + 0x20]
14146db79: mov rcx, qword ptr [r12 + 0x40]
14146db7e: mov qword ptr [rbp + 0x910], rcx
14146db85: movabs rbx, 0x8000000000000000
14146db8f: mov qword ptr [r12 + 0x18], rbx
14146db94: mov rdx, rsi
14146db97: xor rdx, rbx
14146db9a: cmp rdx, 2
14146db9e: mov ecx, 2
14146dba3: cmovb rcx, rdx
14146dba7: cmp rcx, 1
14146dbab: je 0x14146e136
14146dbb1: cmp rcx, 2
14146dbb5: jne 0x14146f9e1
14146dbbb: mov rcx, qword ptr [rax + 0x10]
14146dbbf: mov qword ptr [rbp + 0x5a0], rcx
14146dbc6: movups xmm0, xmmword ptr [rax]
14146dbc9: movaps xmmword ptr [rbp + 0x590], xmm0
14146dbd0: mov qword ptr [rbp + 0x830], rsi
14146dbd7: movaps xmm0, xmmword ptr [rbp + 0x590]
14146dbde: movups xmmword ptr [rbp + 0x838], xmm0
14146dbe5: mov rax, qword ptr [rbp + 0x5a0]
14146dbec: mov qword ptr [rbp + 0x848], rax
14146dbf3: test r14b, r14b
14146dbf6: je 0x14146dc02
14146dbf8: mov dword ptr [rdi + 0xc0], 0x1b
14146dc02: mov qword ptr [rbp + 0x948], rdi
14146dc09: mov qword ptr [rbp + 0x8d0], rbx
14146dc10: mov byte ptr [rbp + 0x971], 1
14146dc17: mov byte ptr [rbp + 0x970], 1
14146dc1e: lea rcx, [rbp + 0x830]
14146dc25: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
14146dc2a: mov r12, rax
14146dc2d: mov r14, rdx
14146dc30: mov rax, qword ptr [rbp + 0x8b0]
14146dc37: mov ecx, dword ptr [rbp + 0x8b8]
14146dc3d: mov dword ptr [rax], ecx
14146dc3f: mov rax, qword ptr [rbp + 0x560]
14146dc46: mov ecx, dword ptr [rbp + 0x8c0]
14146dc4c: mov dword ptr [rax], ecx
14146dc4e: mov rax, qword ptr [rbp + 0x8d8]
14146dc55: mov ecx, dword ptr [rbp + 0x90c]
14146dc5b: mov dword ptr [rax], ecx
14146dc5d: mov rax, qword ptr [rbp + 0x570]
14146dc64: mov ecx, dword ptr [rbp + 0x8c4]
14146dc6a: mov dword ptr [rax], ecx
14146dc6c: mov rax, qword ptr [rbp + 0x528]
14146dc73: mov ecx, dword ptr [rbp + 0x85c]
14146dc79: mov dword ptr [rax], ecx
14146dc7b: mov rcx, qword ptr [rbp + 0x948]
14146dc82: mov eax, dword ptr [rcx + 0xc0]
14146dc88: cmp eax, 0x1a
14146dc8b: mov rax, qword ptr [rip + 0x21be676]
14146dc92: setge byte ptr [rax]
14146dc95: setg byte ptr [rip + 0x21be375]
14146dc9c: lea rdx, [rcx + 0xc4]
14146dca3: mov qword ptr [rbp + 0x940], rbx
14146dcaa: mov byte ptr [rbp + 0x977], 1
14146dcb1: mov rcx, qword ptr [rbp + 0x8a8]
14146dcb8: mov r8b, 1
14146dcbb: call 0x140911520 ; country_core::systems::camera_rig::MainCamera::load_parameters
14146dcc0: movabs rax, 0x8000000000000000
14146dcca: mov qword ptr [rbp + 0x940], rax
14146dcd1: mov byte ptr [rbp + 0x977], 1
14146dcd8: mov rcx, qword ptr [rbp + 0x8a8]
14146dcdf: call 0x140911b50 ; country_core::systems::camera_rig::MainCamera::immediately_finish_interpolating
14146dce4: mov r8, qword ptr [rbp + 0x948]
14146dceb: movups xmm0, xmmword ptr [r8 + 0x94]
14146dcf3: movups xmm1, xmmword ptr [r8 + 0xa4]
14146dcfb: movups xmm2, xmmword ptr [r8 + 0xb0]
14146dd03: mov rax, qword ptr [rbp + 0x558]
14146dd0a: movups xmmword ptr [rax + 0x1c], xmm2
14146dd0e: movups xmmword ptr [rax + 0x10], xmm1
14146dd12: movups xmmword ptr [rax], xmm0
14146dd15: mov rax, qword ptr [r8 + 0x30]
14146dd19: mov qword ptr [rbp + 0x950], rax
14146dd20: mov rax, qword ptr [r8 + 0x38]
14146dd24: mov qword ptr [rbp + 0x960], rax
14146dd2b: mov rsi, qword ptr [r8 + 0x40]
14146dd2f: mov eax, dword ptr [r8 + 0x90]
14146dd36: mov rcx, qword ptr [rbp + 0x568]
14146dd3d: mov dword ptr [rcx + 0x40], eax
14146dd40: movups xmm0, xmmword ptr [r8 + 0x80]
14146dd48: movups xmmword ptr [rcx + 0x30], xmm0
14146dd4c: movups xmm0, xmmword ptr [r8 + 0x70]
14146dd51: movups xmmword ptr [rcx + 0x20], xmm0
14146dd55: movups xmm0, xmmword ptr [r8 + 0x60]
14146dd5a: movups xmmword ptr [rcx + 0x10], xmm0
14146dd5e: movups xmm0, xmmword ptr [r8 + 0x50]
14146dd63: movups xmmword ptr [rcx], xmm0
14146dd66: mov eax, dword ptr [r8 + 0xe8]
14146dd6d: mov dword ptr [rbp + 0x938], eax
14146dd73: mov eax, dword ptr [r8 + 0xec]
14146dd7a: mov dword ptr [rbp + 0x958], eax
14146dd80: movups xmm0, xmmword ptr [r8]
14146dd84: movaps xmmword ptr [rbp - 0x50], xmm0
14146dd88: mov r9, qword ptr [r8 + 0x10]
14146dd8c: mov rax, qword ptr [r8 + 0x18]
14146dd90: mov rcx, qword ptr [r8 + 0x20]
14146dd94: mov rdx, qword ptr [r8 + 0x28]
14146dd98: mov r10, qword ptr [r8 + 0x48]
14146dd9c: mov qword ptr [rbp + 0x8b8], r10
14146dda3: mov r8d, dword ptr [r8 + 0xc0]
14146ddaa: mov dword ptr [rbp + 0x90c], r8d
14146ddb1: shl rdx, 4
14146ddb5: add rdx, rcx
14146ddb8: mov qword ptr [rbp + 0x5b0], rcx
14146ddbf: mov qword ptr [rbp + 0x5b8], rcx
14146ddc6: mov qword ptr [rbp + 0x5c0], rax
14146ddcd: mov qword ptr [rbp + 0x5c8], rdx
14146ddd4: movaps xmmword ptr [rbp + 0x1b0], xmm0
14146dddb: mov qword ptr [rbp + 0x8b0], r9
14146dde2: mov qword ptr [rbp + 0x1c0], r9
14146dde9: lea r8, [rip + 0x17cae68]
14146ddf0: lea rcx, [rbp + 0x360]
14146ddf7: lea rdx, [rbp + 0x5b0]
14146ddfe: call 0x141737510 ; alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>,country_core::resources::history::HistoryChainSnapshot (*)(country_core::resources::history::ResolvedHistoryChainSnapshot)>,country_core::resources::history::HistoryChainSnapshot>
14146de03: mov qword ptr [rbp + 0x8d8], rsi
14146de0a: movups xmm0, xmmword ptr [rbp + 0x360]
14146de11: movups xmmword ptr [rbp + 0x5c8], xmm0
14146de18: mov rax, qword ptr [rbp + 0x370]
14146de1f: mov qword ptr [rbp + 0x5d8], rax
14146de26: movaps xmm0, xmmword ptr [rbp + 0x1b0]
14146de2d: movaps xmmword ptr [rbp + 0x5b0], xmm0
14146de34: mov rax, qword ptr [rbp + 0x1c0]
14146de3b: mov qword ptr [rbp + 0x5c0], rax
14146de42: mov rcx, qword ptr [rbp + 0x918]
14146de49: call 0x14088acf0 ; core::ptr::drop_in_place<country_core::resources::history::History>
14146de4e: movaps xmm0, xmmword ptr [rbp + 0x5b0]
14146de55: movaps xmm1, xmmword ptr [rbp + 0x5c0]
14146de5c: movaps xmm2, xmmword ptr [rbp + 0x5d0]
14146de63: mov rcx, qword ptr [rbp + 0x918]
14146de6a: movups xmmword ptr [rcx + 0x20], xmm2
14146de6e: movups xmmword ptr [rcx + 0x10], xmm1
14146de72: movups xmmword ptr [rcx], xmm0
14146de75: movabs rax, 0x8000000000000000
14146de7f: mov qword ptr [rcx + 0x30], rax
14146de83: mov rax, qword ptr [rbp + 0x8b0]
14146de8a: mov qword ptr [rcx + 0x58], rax
14146de8e: mov rax, qword ptr [rbp + 0x8b8]
14146de95: mov qword ptr [rcx + 0x60], rax
14146de99: mov eax, dword ptr [rbp + 0x938]
14146de9f: mov dword ptr [rcx + 0x68], eax
14146dea2: mov eax, dword ptr [rbp + 0x958]
14146dea8: mov dword ptr [rcx + 0x6c], eax
14146deab: mov eax, dword ptr [rbp + 0x90c]
14146deb1: mov dword ptr [rcx + 0x70], eax
14146deb4: bt qword ptr [rbp + 0x910], 0x38
14146debd: jae 0x14146df22
14146debf: mov rax, qword ptr [rbp + 0x578]
14146dec6: mov ecx, dword ptr [rbp + 0x8c8]
14146decc: mov dword ptr [rax], ecx
14146dece: mov rsi, qword ptr [r13 + 0x30]
14146ded2: mov rdi, qword ptr [r13 + 0x40]
14146ded6: cmp rsi, qword ptr [r13 + 0x20]
14146deda: mov rax, qword ptr [rbp + 0x960]
14146dee1: mov rdx, qword ptr [rbp + 0x950]
14146dee8: jne 0x14146df0f
14146deea: lea rcx, [r13 + 0x20]
14146deee: mov qword ptr [rbp + 0x940], rdx
14146def5: mov qword ptr [rbp + 0x920], rax
14146defc: mov byte ptr [rbp + 0x977], 0
14146df03: lea rdx, [rip + 0x17cc23e]
14146df0a: call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
14146df0f: mov rax, qword ptr [r13 + 0x28]
14146df13: mov qword ptr [rax + rsi*8], rdi
14146df17: inc rsi
14146df1a: mov qword ptr [r13 + 0x30], rsi
14146df1e: inc qword ptr [r13 + 0x40]
14146df22: cmp byte ptr [rbp + 0x928], 0
14146df29: jne 0x14146dfa6
14146df2b: mov r15, qword ptr [rbp + 0xa60]
14146df32: mov rax, qword ptr [rbp + 0x950]
14146df39: mov qword ptr [rbp + 0x940], rax
14146df40: mov rax, qword ptr [rbp + 0x960]
14146df47: mov qword ptr [rbp + 0x920], rax
14146df4e: mov byte ptr [rbp + 0x977], 0
14146df55: mov qword ptr [rsp + 0x20], r15
14146df5a: lea rcx, [rbp + 0x5b0]
14146df61: mov rdx, qword ptr [rbp + 0x918]
14146df68: mov r8, r12
14146df6b: mov r9, r14
14146df6e: call 0x1417a6e50 ; country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data
14146df73: mov rsi, qword ptr [rbp + 0x5b8]
14146df7a: cmp rsi, 3
14146df7e: je 0x14146dfa6
14146df80: mov edi, dword ptr [rbp + 0x5b0]
14146df86: lea rdx, [rbp + 0x5c0]
14146df8d: lea rcx, [rbp - 0x50]
14146df91: mov r8d, 0x1a8
14146df97: call 0x142923c00 ; 
14146df9c: cmp rsi, 2
14146dfa0: jne 0x14146eb31
14146dfa6: mov rdi, qword ptr [rbp + 0x918]
14146dfad: mov dword ptr [rdi + 0x68], 0xffffffff
14146dfb4: mov rax, qword ptr [rdi + 0x10]
14146dfb8: test rax, rax
14146dfbb: je 0x14146e7a6
14146dfc1: mov rcx, qword ptr [rdi + 8]
14146dfc5: lea rax, [rax + rax*4]
14146dfc9: lea rax, [rcx + rax*8]
14146dfcd: mov qword ptr [rbp + 0x938], rax
14146dfd4: xor r8d, r8d
14146dfd7: movabs r10, 0x8000000000000001
14146dfe1: movabs r11, 0x8000000000000010
14146dfeb: lea rdi, [rip + 0x17f6bfe]
14146dff2: xor r9d, r9d
14146dff5: jmp 0x14146e018
14146dff7: nop word ptr [rax + rax]
14146e000: mov rcx, qword ptr [rbp + 0x928]
14146e007: add rcx, 0x28
14146e00b: cmp rcx, qword ptr [rbp + 0x938]
14146e012: je 0x14146e7a9
14146e018: mov rax, qword ptr [rcx + 0x10]
14146e01c: test rax, rax
14146e01f: mov qword ptr [rbp + 0x928], rcx
14146e026: je 0x14146e000
14146e028: mov r12, qword ptr [rcx + 8]
14146e02c: lea rax, [rax + rax*2]
14146e030: lea rax, [r12 + rax*8]
14146e034: mov qword ptr [rbp + 0x958], rax
14146e03b: jmp 0x14146e06b
14146e03d: nop dword ptr [rax]
14146e040: mov rax, qword ptr [r12 + 8]
14146e045: movzx edx, byte ptr [rax + 0x10]
14146e049: lea rcx, [rip + 0x17f6b78]
14146e050: add rax, qword ptr [rcx + rdx*8]
14146e054: mov rax, qword ptr [rax]
14146e057: cmp rax, r9
14146e05a: cmova r9, rax
14146e05e: add r12, 0x18
14146e062: cmp r12, qword ptr [rbp + 0x958]
14146e069: je 0x14146e000
14146e06b: mov rax, qword ptr [r12]
14146e06f: cmp rax, 7
14146e073: je 0x14146e0c0
14146e075: cmp eax, 8
14146e078: je 0x14146e040
14146e07a: cmp eax, 9
14146e07d: jne 0x14146e05e
14146e07f: mov rax, qword ptr [r12 + 8]
14146e084: movzx edx, byte ptr [rax + 0x18]
14146e088: add dl, 0xfd
14146e08b: cmp dl, 0xe
14146e08e: movzx edx, dl
14146e091: mov ecx, 3
14146e096: cmovae edx, ecx
14146e099: movzx edx, dl
14146e09c: lea rcx, [rip + 0x17f6c4d]
14146e0a3: movsxd rdx, dword ptr [rcx + rdx*4]
14146e0a7: add rdx, rcx
14146e0aa: jmp rdx
14146e0ac: add rax, 0x20
14146e0b0: jmp 0x14146e054
14146e0b2: nop word ptr cs:[rax + rax]
14146e0c0: mov rsi, qword ptr [r12 + 0x10]
14146e0c5: test rsi, rsi
14146e0c8: je 0x14146e05e
14146e0ca: mov rax, qword ptr [r12 + 8]
14146e0cf: shl rsi, 4
14146e0d3: add rsi, rax
14146e0d6: jmp 0x14146e0ed
14146e0d8: nop dword ptr [rax + rax]
14146e0e0: add rax, 0x10
14146e0e4: cmp rax, rsi
14146e0e7: je 0x14146e05e
14146e0ed: mov rdx, qword ptr [rax + 8]
14146e0f1: test rdx, rdx
14146e0f4: je 0x14146e0e0
14146e0f6: mov rbx, qword ptr [rax]
14146e0f9: shl rdx, 3
14146e0fd: xor r14d, r14d
14146e100: mov r15, qword ptr [rbx + r14]
14146e104: mov rcx, qword ptr [r15 + 0x10]
14146e108: lea r13, [rcx + r10]
14146e10c: cmp rcx, r11
14146e10f: cmovge r13, r8
14146e113: mov rcx, qword ptr [rdi + r13*8]
14146e117: mov rcx, qword ptr [r15 + rcx]
14146e11b: cmp rcx, r9
14146e11e: cmova r9, rcx
14146e122: add r14, 8
14146e126: cmp rdx, r14
14146e129: jne 0x14146e100
14146e12b: jmp 0x14146e0e0
14146e12d: add rax, 0x10
14146e131: jmp 0x14146e054
14146e136: mov sil, r14b
14146e139: mov rcx, qword ptr [rax + 0x10]
14146e13d: mov qword ptr [rbp - 0x40], rcx
14146e141: movups xmm0, xmmword ptr [rax]
14146e144: movaps xmmword ptr [rbp - 0x50], xmm0
14146e148: mov qword ptr [rbp - 0x38], rdi
14146e14c: lea r15, [rbp - 0x50]
14146e150: mov rcx, r15
14146e153: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
14146e158: mov r8, rdx
14146e15b: mov rcx, qword ptr [rbp + 0xa70]
14146e162: mov r9, qword ptr [rcx]
14146e165: lea r14, [rbp + 0x1b0]
14146e16c: mov rcx, r14
14146e16f: mov rdx, rax
14146e172: call 0x141470d00 ; country_core::systems::save::load_game::parse_saved_game_data
14146e177: movzx ebx, byte ptr [rbp + 0x1bf]
14146e17e: cmp bl, 2
14146e181: jne 0x14146ea18
14146e187: mov rdi, qword ptr [rbp + 0x1b0]
14146e18e: mov qword ptr [rbp + 0x868], rdi
14146e195: lea rsi, [rbp + 0x5b0]
14146e19c: mov rcx, rsi
14146e19f: call 0x140c102e0 ; uuid::Uuid::new_v4
14146e1a4: mov rax, qword ptr [rbp + 0x960]
14146e1ab: mov ecx, dword ptr [rax + 0x1c]
14146e1ae: mov rax, qword ptr [rax + 0x10]
14146e1b2: mov dword ptr [rax], ecx
14146e1b4: movups xmm0, xmmword ptr [rbp + 0x5b0]
14146e1bb: mov rax, qword ptr [rbp + 0x8e0]
14146e1c2: movups xmmword ptr [rax], xmm0
14146e1c5: mov qword ptr [rbp + 0x1b0], r15
14146e1cc: lea rax, [rip - 0x70c6f3]
14146e1d3: mov qword ptr [rbp + 0x1b8], rax
14146e1da: lea rax, [rbp + 0x868]
14146e1e1: mov qword ptr [rbp + 0x1c0], rax
14146e1e8: lea rax, [rip - 0x7215df]
14146e1ef: mov qword ptr [rbp + 0x1c8], rax
14146e1f6: mov qword ptr [rbp + 0x5b0], 2
14146e201: mov qword ptr [rbp + 0x5c0], 2
14146e20c: mov qword ptr [rbp + 0x5d0], 0
14146e217: mov qword ptr [rbp + 0x5d8], 0x20
14146e222: mov byte ptr [rbp + 0x5e0], 3
14146e229: mov qword ptr [rbp + 0x5e8], 2
14146e234: mov qword ptr [rbp + 0x5f8], 2
14146e23f: mov qword ptr [rbp + 0x608], 1
14146e24a: movabs rax, 0x400000020
14146e254: mov qword ptr [rbp + 0x610], rax
14146e25b: mov byte ptr [rbp + 0x618], 3
14146e262: lea rax, [rip + 0x17f5fe7]
14146e269: mov qword ptr [rbp + 0x360], rax
14146e270: mov qword ptr [rbp + 0x368], 2
14146e27b: mov qword ptr [rbp + 0x370], r14
14146e282: mov qword ptr [rbp + 0x378], 2
14146e28d: mov qword ptr [rbp + 0x380], rsi
14146e294: mov qword ptr [rbp + 0x388], 2
14146e29f: lea rcx, [rbp + 0x178]
14146e2a6: lea rsi, [rbp + 0x360]
14146e2ad: mov rdx, rsi
14146e2b0: call 0x140d79f90 ; alloc::fmt::format::format_inner
14146e2b5: movups xmm0, xmmword ptr [rbp + 0x178]
14146e2bc: movaps xmmword ptr [rbp + 0x880], xmm0
14146e2c3: mov rax, qword ptr [rbp + 0x188]
14146e2ca: mov qword ptr [rbp + 0x890], rax
14146e2d1: mov rax, qword ptr [rip + 0x21be0d0]
14146e2d8: mov rax, qword ptr [rax]
14146e2db: test rax, rax
14146e2de: je 0x14146e3be
14146e2e4: lea rax, [rbp + 0x880]
14146e2eb: mov qword ptr [rbp + 0x360], rax
14146e2f2: lea rax, [rip - 0x1432829]
14146e2f9: mov qword ptr [rbp + 0x368], rax
14146e300: lea rcx, [rip + 0x17f5f79]
14146e307: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14146e30c: movups xmm0, xmmword ptr [rax]
14146e30f: mov eax, dword ptr [rax + 0x10]
14146e312: mov qword ptr [rbp + 0x5e0], 1
14146e31d: lea rcx, [rip + 0x17f5e24]
14146e324: mov qword ptr [rbp + 0x5e8], rcx
14146e32b: mov qword ptr [rbp + 0x5f0], 0x26
14146e336: lea rdx, [rip + 0x17f5f33]
14146e33d: mov qword ptr [rbp + 0x600], rdx
14146e344: mov qword ptr [rbp + 0x608], 1
14146e34f: mov qword ptr [rbp + 0x610], rsi
14146e356: mov qword ptr [rbp + 0x618], 1
14146e361: mov qword ptr [rbp + 0x620], 0
14146e36c: mov qword ptr [rbp + 0x5b0], 0
14146e377: mov qword ptr [rbp + 0x5b8], rcx
14146e37e: mov qword ptr [rbp + 0x5c0], 0x26
14146e389: mov qword ptr [rbp + 0x5c8], 0
14146e394: movups xmmword ptr [rbp + 0x5d0], xmm0
14146e39b: mov dword ptr [rbp + 0x5f8], 1
14146e3a5: mov dword ptr [rbp + 0x5fc], eax
14146e3ab: lea rcx, [rbp + 0x972]
14146e3b2: lea rdx, [rbp + 0x5b0]
14146e3b9: call 0x140d4a330 ; log::__private_api::impl$2::log
14146e3be: mov rcx, qword ptr [rbp + 0x888]
14146e3c5: mov rdx, qword ptr [rbp + 0x890]
14146e3cc: call 0x1408c10a0 ; country_core::systems::backend::flush_screen_log::log_error_to_screen
14146e3d1: mov byte ptr [r12 + 0x48], 2
14146e3d7: mov eax, dword ptr [rbp + 0x8cc]
14146e3dd: mov rcx, qword ptr [rbp + 0x588]
14146e3e4: mov dword ptr [rcx], eax
14146e3e6: mov rax, qword ptr [r12 + 0x10]
14146e3eb: mov qword ptr [rbp + 0x840], rax
14146e3f2: movups xmm0, xmmword ptr [r12]
14146e3f7: movaps xmmword ptr [rbp + 0x830], xmm0
14146e3fe: mov qword ptr [r12], 0
14146e406: mov qword ptr [r12 + 8], 8
14146e40f: mov qword ptr [r12 + 0x10], 0
14146e418: mov rax, qword ptr [rbp + 0x540]
14146e41f: mov ecx, dword ptr [rbp + 0x864]
14146e425: mov dword ptr [rax], ecx
14146e427: mov rbx, qword ptr [rbp + 0x538]
14146e42e: mov rsi, qword ptr [rbx + 0x30]
14146e432: mov rdi, qword ptr [rbx + 0x40]
14146e436: cmp rsi, qword ptr [rbx + 0x20]
14146e43a: jne 0x14146e453
14146e43c: lea rcx, [rbx + 0x20]
14146e440: mov byte ptr [rbp + 0x96f], 1
14146e447: lea rdx, [rip + 0x17cbcfa]
14146e44e: call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
14146e453: mov rax, qword ptr [rbp + 0xa50]
14146e45a: mov rcx, qword ptr [rbp + 0xa48]
14146e461: mov rdx, qword ptr [rbx + 0x28]
14146e465: mov qword ptr [rdx + rsi*8], rdi
14146e469: inc rsi
14146e46c: mov qword ptr [rbx + 0x30], rsi
14146e470: inc qword ptr [rbx + 0x40]
14146e474: mov edx, dword ptr [rax + 0x1c]
14146e477: mov r8, qword ptr [rax + 0x10]
14146e47b: mov dword ptr [r8], edx
14146e47e: mov rax, qword ptr [rax]
14146e481: mov byte ptr [rax], 2
14146e484: mov eax, dword ptr [rcx + 0x1c]
14146e487: mov rdx, qword ptr [rcx + 0x10]
14146e48b: mov dword ptr [rdx], eax
14146e48d: mov rax, qword ptr [rcx]
14146e490: inc qword ptr [rax + 0x1d8]
14146e497: mov qword ptr [rbp + 0x380], 1
14146e4a2: mov qword ptr [rbp + 0x388], 0
14146e4ad: movabs rsi, 0x8000000000000000
14146e4b7: mov qword ptr [rbp + 0x390], rsi
14146e4be: mov byte ptr [rbp + 0x3d0], 0
14146e4c5: mov qword ptr [rbp + 0x3d8], rsi
14146e4cc: mov qword ptr [rbp + 0x360], 0
14146e4d7: mov qword ptr [rbp + 0x368], 8
14146e4e2: xorps xmm6, xmm6
14146e4e5: movups xmmword ptr [rbp + 0x370], xmm6
14146e4ec: mov byte ptr [rbp + 0x3f8], 0
14146e4f3: mov rcx, qword ptr [rbp + 0x830]
14146e4fa: mov rax, qword ptr [rbp + 0x838]
14146e501: mov rdi, qword ptr [rbp + 0x840]
14146e508: shl rdi, 5
14146e50c: add rdi, rax
14146e50f: mov qword ptr [rbp + 0x1b0], rax
14146e516: mov qword ptr [rbp + 0x1b8], rax
14146e51d: mov qword ptr [rbp + 0x1c0], rcx
14146e524: mov qword ptr [rbp + 0x1c8], rdi
14146e52b: lea rbx, [rbp + 0x5c0]
14146e532: movabs r14, 0x8000000000000003
14146e53c: mov r15d, 1
14146e542: jmp 0x14146e582
14146e544: nop word ptr cs:[rax + rax]
14146e550: mov rax, qword ptr [rbp + 0x368]
14146e557: mov rcx, r12
14146e55a: shl rcx, 5
14146e55e: movups xmm0, xmmword ptr [rbp + 0x5b0]
14146e565: movups xmm1, xmmword ptr [rbp + 0x5c0]
14146e56c: movups xmmword ptr [rax + rcx + 0x10], xmm1
14146e571: movups xmmword ptr [rax + rcx], xmm0
14146e575: inc r12
14146e578: mov qword ptr [rbp + 0x370], r12
14146e57f: mov rax, r13
14146e582: cmp rax, rdi
14146e585: je 0x14146e71a
14146e58b: lea r13, [rax + 0x20]
14146e58f: mov qword ptr [rbp + 0x1b8], r13
14146e596: mov rcx, qword ptr [rax]
14146e599: cmp rcx, r14
14146e59c: je 0x14146e71a
14146e5a2: mov r8, rcx
14146e5a5: xor r8, rsi
14146e5a8: cmp r8, 3
14146e5ac: cmovae r8, r15
14146e5b0: test r8, r8
14146e5b3: je 0x14146e610
14146e5b5: mov rdx, qword ptr [rax + 8]
14146e5b9: cmp r8, 1
14146e5bd: jne 0x14146e6e0
14146e5c3: add rax, 0x10
14146e5c7: movups xmm0, xmmword ptr [rax]
14146e5ca: movaps xmmword ptr [rbp + 0x160], xmm0
14146e5d1: mov qword ptr [rbp + 0x5b0], rcx
14146e5d8: mov qword ptr [rbp + 0x5b8], rdx
14146e5df: movups xmmword ptr [rbx], xmm0
14146e5e2: mov r12, qword ptr [rbp + 0x370]
14146e5e9: cmp r12, qword ptr [rbp + 0x360]
14146e5f0: jne 0x14146e550
14146e5f6: lea rcx, [rbp + 0x360]
14146e5fd: lea rdx, [rip + 0x17f5d04]
14146e604: call 0x140562c30 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::water::WaterEraseEvent>,alloc::alloc::Global>
14146e609: jmp 0x14146e550
14146e60e: nop 
14146e610: mov rax, qword ptr [rip + 0x21bdd91]
14146e617: mov rcx, qword ptr [rax]
14146e61a: mov rax, r13
14146e61d: cmp rcx, 2
14146e621: jb 0x14146e582
14146e627: lea rcx, [rip + 0x17f5cc2]
14146e62e: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14146e633: movups xmm0, xmmword ptr [rax]
14146e636: mov eax, dword ptr [rax + 0x10]
14146e639: mov qword ptr [rbp + 0x5e0], 2
14146e644: lea rcx, [rip + 0x17f5afd]
14146e64b: mov qword ptr [rbp + 0x5e8], rcx
14146e652: mov qword ptr [rbp + 0x5f0], 0x26
14146e65d: lea rdx, [rip + 0x17f5c7c]
14146e664: mov qword ptr [rbp + 0x600], rdx
14146e66b: mov qword ptr [rbp + 0x608], 1
14146e676: mov qword ptr [rbp + 0x610], 8
14146e681: mov qword ptr [rbp + 0x5b0], 0
14146e68c: movups xmmword ptr [rbx + 0x58], xmm6
14146e690: mov qword ptr [rbp + 0x5b8], rcx
14146e697: mov qword ptr [rbp + 0x5c0], 0x26
14146e6a2: mov qword ptr [rbp + 0x5c8], 0
14146e6ad: movups xmmword ptr [rbp + 0x5d0], xmm0
14146e6b4: mov dword ptr [rbp + 0x5f8], 1
14146e6be: mov dword ptr [rbp + 0x5fc], eax
14146e6c4: lea rcx, [rbp + 0x972]
14146e6cb: lea rdx, [rbp + 0x5b0]
14146e6d2: call 0x140d4a330 ; log::__private_api::impl$2::log
14146e6d7: mov rax, r13
14146e6da: jmp 0x14146e582
14146e6df: nop 
14146e6e0: mov qword ptr [rbp + 0x5b8], rdx
14146e6e7: mov qword ptr [rbp + 0x5b0], rsi
14146e6ee: mov r12, qword ptr [rbp + 0x370]
14146e6f5: cmp r12, qword ptr [rbp + 0x360]
14146e6fc: jne 0x14146e550
14146e702: lea rcx, [rbp + 0x360]
14146e709: lea rdx, [rip + 0x17f5c10]
14146e710: call 0x140562c30 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::water::WaterEraseEvent>,alloc::alloc::Global>
14146e715: jmp 0x14146e550
14146e71a: mov byte ptr [rbp + 0x96e], 1
14146e721: lea rcx, [rbp + 0x1b0]
14146e728: call 0x14087b2f0 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<country_core::systems::save::load_game::PostGameLoadCommand>,alloc::alloc::Global> >
14146e72d: lea rdi, [rbp + 0x5b0]
14146e734: lea rdx, [rbp + 0x360]
14146e73b: mov r8d, 0xa0
14146e741: mov rcx, rdi
14146e744: call 0x142923c00 ; 
14146e749: mov byte ptr [rbp + 0x96e], 0
14146e750: lea r8, [rip + 0x17f5b41]
14146e757: mov rcx, qword ptr [rbp + 0xa58]
14146e75e: mov rdx, rdi
14146e761: call 0x1412e0b90 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::resources::session::NextSession>
14146e766: mov rdx, qword ptr [rbp + 0x880]
14146e76d: test rdx, rdx
14146e770: je 0x14146e784
14146e772: mov rcx, qword ptr [rbp + 0x888]
14146e779: mov r8d, 1
14146e77f: call 0x140613c20 ; 
14146e784: lea rcx, [rbp + 0x868]
14146e78b: call 0x140d4cc30 ; anyhow::error::impl$6::drop
14146e790: mov rdx, qword ptr [rbp - 0x50]
14146e794: test rdx, rdx
14146e797: je 0x14146f625
14146e79d: mov rcx, qword ptr [rbp - 0x48]
14146e7a1: jmp 0x14146f61a
14146e7a6: xor r9d, r9d
14146e7a9: mov rcx, qword ptr [rbp + 0x580]
14146e7b0: mov rax, qword ptr [rcx + 0xc0]
14146e7b7: cmp r9, rax
14146e7ba: cmova rax, r9
14146e7be: mov qword ptr [rcx + 0xc0], rax
14146e7c5: mov rcx, qword ptr [rbp + 0x918]
14146e7cc: mov eax, dword ptr [rcx + 0x6c]
14146e7cf: test eax, eax
14146e7d1: mov edx, 0
14146e7d6: cmovg edx, eax
14146e7d9: mov dword ptr [rcx + 0x6c], edx
14146e7dc: mov r12, qword ptr [rbp + 0x950]
14146e7e3: mov qword ptr [rbp + 0x940], r12
14146e7ea: mov rsi, qword ptr [rbp + 0x960]
14146e7f1: mov qword ptr [rbp + 0x920], rsi
14146e7f8: mov byte ptr [rbp + 0x977], 0
14146e7ff: call 0x140d635e0 ; std::time::Instant::now
14146e804: mov qword ptr [rbp + 0x590], rax
14146e80b: mov dword ptr [rbp + 0x598], edx
14146e811: mov rax, qword ptr [rbp + 0x948]
14146e818: mov rdi, qword ptr [rax + 0x48]
14146e81c: mov ebx, dword ptr [rax + 0xc0]
14146e822: mov qword ptr [rbp + 0x940], r12
14146e829: mov qword ptr [rbp + 0x920], rsi
14146e830: mov byte ptr [rbp + 0x977], 0
14146e837: lea r13, [rbp + 0x360]
14146e83e: mov rcx, r13
14146e841: mov rdx, qword ptr [rbp + 0x8a8]
14146e848: call 0x140911710 ; country_core::systems::camera_rig::MainCamera::store_parameters
14146e84d: mov rax, qword ptr [rbp + 0x558]
14146e854: mov r14w, word ptr [rax + 0x28]
14146e859: mov r15d, dword ptr [rax]
14146e85c: movups xmm8, xmmword ptr [rax + 4]
14146e861: movups xmm9, xmmword ptr [rax + 0x14]
14146e866: movss xmm10, dword ptr [rax + 0x24]
14146e86c: mov rax, qword ptr [rbp + 0x918]
14146e873: movsd xmm7, qword ptr [rax + 0x68]
14146e878: mov rdx, qword ptr [rax + 8]
14146e87c: mov rax, qword ptr [rax + 0x10]
14146e880: lea rax, [rax + rax*4]
14146e884: lea r8, [rdx + rax*8]
14146e888: mov qword ptr [rbp + 0x940], r12
14146e88f: mov qword ptr [rbp + 0x920], rsi
14146e896: mov byte ptr [rbp + 0x977], 0
14146e89d: lea r9, [rip + 0x17f55a4]
14146e8a4: lea rcx, [rbp - 0x50]
14146e8a8: call 0x14130dec0 ; alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::history::MultiEdit,core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<country_core::resources::history::MultiEdit> > >
14146e8ad: mov rax, qword ptr [rbp + 0x918]
14146e8b4: mov rdx, qword ptr [rax + 0x20]
14146e8b8: mov r8, qword ptr [rax + 0x28]
14146e8bc: shl r8, 4
14146e8c0: add r8, rdx
14146e8c3: lea r9, [rip + 0x17f557e]
14146e8ca: lea r12, [rbp + 0x1b0]
14146e8d1: mov rcx, r12
14146e8d4: call 0x1413056c0 ; alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::history::ResolvedHistoryChainSnapshot,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::history::HistoryChainSnapshot>,country_core::resources::history::impl$1::serialize::closure_env$0<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > > >
14146e8d9: mov rax, qword ptr [rbp + 0x950]
14146e8e0: neg rax
14146e8e3: jo 0x14146ee1e
14146e8e9: xor r13d, r13d
14146e8ec: mov ecx, 0x28
14146e8f1: mov rsi, qword ptr [rbp + 0x8d8]
14146e8f8: mov rax, rsi
14146e8fb: mul rcx
14146e8fe: mov r12, rax
14146e901: jo 0x14146f9ca
14146e907: movabs rax, 0x7ffffffffffffffc
14146e911: cmp r12, rax
14146e914: ja 0x14146f9ca
14146e91a: test r12, r12
14146e91d: je 0x14146eb15
14146e923: movzx eax, byte ptr [rip + 0x21c62df]
14146e92a: mov r13d, 4
14146e930: mov edx, 4
14146e935: mov rcx, r12
14146e938: call 0x140613c10 ; 
14146e93d: test rax, rax
14146e940: je 0x14146f9ca
14146e946: test rsi, rsi
14146e949: lea r13, [rbp + 0x360]
14146e950: je 0x14146ee0c
14146e956: mov r9, qword ptr [rbp + 0x960]
14146e95d: lea rcx, [r9 + 4]
14146e961: xor edx, edx
14146e963: mov r8, rsi
14146e966: jmp 0x14146e9e0
14146e968: nop dword ptr [rax + rax]
14146e970: dec r8
14146e973: mov r11d, dword ptr [rbp + 0x5b8]
14146e97a: mov dword ptr [rbp + 0x888], r11d
14146e981: mov r11, qword ptr [rbp + 0x5b0]
14146e988: mov qword ptr [rbp + 0x880], r11
14146e98f: mov dword ptr [rax + rdx], r10d
14146e993: mov r10d, dword ptr [rbp + 0x888]
14146e99a: mov dword ptr [rax + rdx + 0xc], r10d
14146e99f: mov r10, qword ptr [rbp + 0x880]
14146e9a6: mov qword ptr [rax + rdx + 4], r10
14146e9ab: mov r10, qword ptr [r9 + 0x10]
14146e9af: mov qword ptr [rax + rdx + 0x10], r10
14146e9b4: mov r10d, dword ptr [r9 + 0x18]
14146e9b8: mov dword ptr [rax + rdx + 0x18], r10d
14146e9bd: mov r10, qword ptr [r9 + 0x1c]
14146e9c1: mov qword ptr [rax + rdx + 0x1c], r10
14146e9c6: mov r10d, dword ptr [r9 + 0x24]
14146e9ca: add r9, 0x28
14146e9ce: mov dword ptr [rax + rdx + 0x24], r10d
14146e9d3: add rdx, 0x28
14146e9d7: test r8, r8
14146e9da: je 0x14146eb09
14146e9e0: cmp r12, rdx
14146e9e3: je 0x14146eb09
14146e9e9: xor r10d, r10d
14146e9ec: cmp dword ptr [r9], 1
14146e9f0: jne 0x14146e970
14146e9f6: mov r10d, dword ptr [rcx + rdx + 8]
14146e9fb: mov dword ptr [rbp + 0x5b8], r10d
14146ea02: mov r10, qword ptr [rcx + rdx]
14146ea06: mov qword ptr [rbp + 0x5b0], r10
14146ea0d: mov r10d, 1
14146ea13: jmp 0x14146e970
14146ea18: test byte ptr [rbp + 0x1bc], 1
14146ea1f: je 0x14146ea2a
14146ea21: test sil, sil
14146ea24: je 0x14146f65f
14146ea2a: mov rdi, qword ptr [rbp + 0x1b0]
14146ea31: mov eax, dword ptr [rbp + 0x1b8]
14146ea37: mov dword ptr [rbp + 0x8f0], eax
14146ea3d: mov eax, dword ptr [rbp + 0x1bb]
14146ea43: mov dword ptr [rbp + 0x8f3], eax
14146ea49: mov qword ptr [rbp + 0x5b0], rdi
14146ea50: mov eax, dword ptr [rbp + 0x8f0]
14146ea56: mov ecx, dword ptr [rbp + 0x8f3]
14146ea5c: mov dword ptr [rbp + 0x5b8], eax
14146ea62: mov dword ptr [rbp + 0x5bb], ecx
14146ea68: mov byte ptr [rbp + 0x5bf], bl
14146ea6e: lea rcx, [rbp - 0x50]
14146ea72: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
14146ea77: movabs rbx, 0x8000000000000000
14146ea81: mov r14b, sil
14146ea84: mov rcx, rax
14146ea87: call 0x140d61cd0 ; std::path::Path::parent
14146ea8c: test rax, rax
14146ea8f: je 0x14146fa64
14146ea95: mov r8, rdx
14146ea98: mov qword ptr [rsp + 0x20], 9
14146eaa1: lea r9, [rip + 0x17f5780]
14146eaa8: lea rcx, [rbp + 0x190]
14146eaaf: mov rdx, rax
14146eab2: call 0x140d62810 ; std::path::Path::_join
14146eab7: mov rax, qword ptr [rbp + 0x5b8]
14146eabe: mov qword ptr [rbp + 0x910], rax
14146eac5: mov rsi, qword ptr [rbp + 0x190]
14146eacc: movups xmm0, xmmword ptr [rbp + 0x198]
14146ead3: movaps xmmword ptr [rbp + 0x590], xmm0
14146eada: mov rax, qword ptr [rbp + 0x1a8]
14146eae1: mov qword ptr [rbp + 0x5a0], rax
14146eae8: mov rdx, qword ptr [rbp - 0x50]
14146eaec: test rdx, rdx
14146eaef: je 0x14146dbd0
14146eaf5: mov rcx, qword ptr [rbp - 0x48]
14146eaf9: mov r8d, 1
14146eaff: call 0x140613c20 ; 
14146eb04: jmp 0x14146dbd0
14146eb09: mov qword ptr [rbp + 0x8e8], rsi
14146eb10: jmp 0x14146ee17
14146eb15: mov eax, 4
14146eb1a: mov qword ptr [rbp + 0x8e8], 0
14146eb25: lea r13, [rbp + 0x360]
14146eb2c: jmp 0x14146ee17
14146eb31: mov qword ptr [rbp + 0x1b0], rsi
14146eb38: lea rcx, [rbp + 0x1b8]
14146eb3f: lea rdx, [rbp - 0x50]
14146eb43: mov r8d, 0x1a8
14146eb49: call 0x142923c00 ; 
14146eb4e: bt qword ptr [rbp + 0x910], 0x30
14146eb57: jae 0x14146ec9c
14146eb5d: mov rax, qword ptr [rip + 0x21bd844]
14146eb64: mov rax, qword ptr [rax]
14146eb67: cmp rax, 3
14146eb6b: jb 0x14146ec35
14146eb71: mov byte ptr [rbp + 0x973], 1
14146eb78: lea rcx, [rip + 0x17f5f21]
14146eb7f: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14146eb84: movups xmm0, xmmword ptr [rax]
14146eb87: mov eax, dword ptr [rax + 0x10]
14146eb8a: mov qword ptr [rbp + 0x5e0], 3
14146eb95: lea rcx, [rip + 0x17f55ac]
14146eb9c: mov qword ptr [rbp + 0x5e8], rcx
14146eba3: mov qword ptr [rbp + 0x5f0], 0x26
14146ebae: lea rdx, [rip + 0x17f5edb]
14146ebb5: mov qword ptr [rbp + 0x600], rdx
14146ebbc: mov qword ptr [rbp + 0x608], 1
14146ebc7: mov qword ptr [rbp + 0x610], 8
14146ebd2: mov qword ptr [rbp + 0x5b0], 0
14146ebdd: xorps xmm1, xmm1
14146ebe0: movups xmmword ptr [rbp + 0x618], xmm1
14146ebe7: mov qword ptr [rbp + 0x5b8], rcx
14146ebee: mov qword ptr [rbp + 0x5c0], 0x26
14146ebf9: mov qword ptr [rbp + 0x5c8], 0
14146ec04: movups xmmword ptr [rbp + 0x5d0], xmm0
14146ec0b: mov dword ptr [rbp + 0x5f8], 1
14146ec15: mov dword ptr [rbp + 0x5fc], eax
14146ec1b: mov byte ptr [rbp + 0x973], 1
14146ec22: lea rcx, [rbp + 0x972]
14146ec29: lea rdx, [rbp + 0x5b0]
14146ec30: call 0x140d4a330 ; log::__private_api::impl$2::log
14146ec35: mov rcx, qword ptr [rbp + 0x218]
14146ec3c: test rcx, rcx
14146ec3f: je 0x14146ec9c
14146ec41: mov rax, qword ptr [rbp + 0x210]
14146ec48: lea rcx, [rcx + rcx*4]
14146ec4c: lea rcx, [rax + rcx*8]
14146ec50: jmp 0x14146ec5b
14146ec52: add rax, 0x28
14146ec56: cmp rax, rcx
14146ec59: je 0x14146ec9c
14146ec5b: mov r8, qword ptr [rax + 0x20]
14146ec5f: test r8, r8
14146ec62: je 0x14146ec52
14146ec64: mov rdx, qword ptr [rax + 0x18]
14146ec68: imul r8, r8, 0x58
14146ec6c: xor r9d, r9d
14146ec6f: jmp 0x14146ec89
14146ec71: nop word ptr cs:[rax + rax]
14146ec80: add r9, 0x58
14146ec84: cmp r8, r9
14146ec87: je 0x14146ec52
14146ec89: cmp byte ptr [rdx + r9 + 0x52], 0x3a
14146ec8f: jne 0x14146ec80
14146ec91: mov dword ptr [rdx + r9 + 0x30], 0x3e7
14146ec9a: jmp 0x14146ec80
14146ec9c: bt qword ptr [rbp + 0x910], 0x28
14146eca5: jae 0x14146f79e
14146ecab: mov rax, qword ptr [rip + 0x21bd6f6]
14146ecb2: mov rax, qword ptr [rax]
14146ecb5: cmp rax, 3
14146ecb9: jb 0x14146ed83
14146ecbf: mov byte ptr [rbp + 0x973], 1
14146ecc6: lea rcx, [rip + 0x17f5e33]
14146eccd: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14146ecd2: movups xmm0, xmmword ptr [rax]
14146ecd5: mov eax, dword ptr [rax + 0x10]
14146ecd8: mov qword ptr [rbp + 0x5e0], 3
14146ece3: lea rcx, [rip + 0x17f545e]
14146ecea: mov qword ptr [rbp + 0x5e8], rcx
14146ecf1: mov qword ptr [rbp + 0x5f0], 0x26
14146ecfc: lea rdx, [rip + 0x17f5ded]
14146ed03: mov qword ptr [rbp + 0x600], rdx
14146ed0a: mov qword ptr [rbp + 0x608], 1
14146ed15: mov qword ptr [rbp + 0x610], 8
14146ed20: mov qword ptr [rbp + 0x5b0], 0
14146ed2b: xorps xmm1, xmm1
14146ed2e: movups xmmword ptr [rbp + 0x618], xmm1
14146ed35: mov qword ptr [rbp + 0x5b8], rcx
14146ed3c: mov qword ptr [rbp + 0x5c0], 0x26
14146ed47: mov qword ptr [rbp + 0x5c8], 0
14146ed52: movups xmmword ptr [rbp + 0x5d0], xmm0
14146ed59: mov dword ptr [rbp + 0x5f8], 1
14146ed63: mov dword ptr [rbp + 0x5fc], eax
14146ed69: mov byte ptr [rbp + 0x973], 1
14146ed70: lea rcx, [rbp + 0x972]
14146ed77: lea rdx, [rbp + 0x5b0]
14146ed7e: call 0x140d4a330 ; log::__private_api::impl$2::log
14146ed83: mov rax, qword ptr [rbp + 0x248]
14146ed8a: test rax, rax
14146ed8d: je 0x14146f6d8
14146ed93: mov rcx, qword ptr [rbp + 0x240]
14146ed9a: shl rax, 4
14146ed9e: xor edx, edx
14146eda0: lea r8, [rip + 0x17f5f01]
14146eda7: jmp 0x14146edbe
14146eda9: mov r9d, r10d
14146edac: mov dword ptr [rcx + rdx + 8], r9d
14146edb1: add rdx, 0x10
14146edb5: cmp rax, rdx
14146edb8: je 0x14146f6d8
14146edbe: mov r10d, dword ptr [rcx + rdx + 8]
14146edc3: cmp r10, 8
14146edc7: ja 0x14146eddb
14146edc9: movsxd r9, dword ptr [r8 + r10*4]
14146edcd: add r9, r8
14146edd0: jmp r9
14146edd3: mov r9d, 2
14146edd9: jmp 0x14146edac
14146eddb: mov r9d, 1
14146ede1: cmp r10d, 0x3e7
14146ede8: je 0x14146eda9
14146edea: jmp 0x14146edac
14146edec: mov r9d, 3
14146edf2: jmp 0x14146edac
14146edf4: mov r9d, 5
14146edfa: jmp 0x14146edac
14146edfc: mov r9d, 8
14146ee02: jmp 0x14146edac
14146ee04: mov r9d, 4
14146ee0a: jmp 0x14146edac
14146ee0c: mov qword ptr [rbp + 0x8e8], 0
14146ee17: lea r12, [rbp + 0x1b0]
14146ee1e: mov rsi, qword ptr [rbp + 0x568]
14146ee25: movzx ecx, byte ptr [rsi + 0x42]
14146ee29: movzx edx, word ptr [rsi + 0x40]
14146ee2d: mov r8d, dword ptr [rsi + 0x38]
14146ee31: movss xmm0, dword ptr [rsi + 0x3c]
14146ee36: movzx r9d, byte ptr [rsi + 0x14]
14146ee3b: mov r10d, dword ptr [rsi + 0x10]
14146ee3f: mov qword ptr [rbp + 0x5f8], rdi
14146ee46: mov dword ptr [rbp + 0x670], ebx
14146ee4c: movups xmm1, xmmword ptr [rbp + 0x360]
14146ee53: movups xmm2, xmmword ptr [rbp + 0x370]
14146ee5a: movups xmmword ptr [rbp + 0x674], xmm1
14146ee61: movups xmmword ptr [rbp + 0x684], xmm2
14146ee68: mov r11d, dword ptr [rbp + 0x380]
14146ee6f: mov dword ptr [rbp + 0x694], r11d
14146ee76: mov dword ptr [rbp + 0x644], r15d
14146ee7d: movups xmmword ptr [rbp + 0x648], xmm8
14146ee85: movups xmmword ptr [rbp + 0x658], xmm9
14146ee8d: movss dword ptr [rbp + 0x668], xmm10
14146ee96: mov word ptr [rbp + 0x66c], r14w
14146ee9e: movlps qword ptr [rbp + 0x698], xmm7
14146eea5: movups xmm1, xmmword ptr [rbp - 0x50]
14146eea9: movaps xmmword ptr [rbp + 0x5b0], xmm1
14146eeb0: mov r11, qword ptr [rbp - 0x40]
14146eeb4: mov qword ptr [rbp + 0x5c0], r11
14146eebb: mov r11, qword ptr [rbp + 0x1c0]
14146eec2: mov qword ptr [rbp + 0x5d8], r11
14146eec9: movups xmm1, xmmword ptr [rbp + 0x1b0]
14146eed0: movups xmmword ptr [rbp + 0x5c8], xmm1
14146eed7: mov r11, qword ptr [rbp + 0x8e8]
14146eede: mov qword ptr [rbp + 0x5e0], r11
14146eee5: mov qword ptr [rbp + 0x5e8], rax
14146eeec: mov rax, qword ptr [rbp + 0x8d8]
14146eef3: mov qword ptr [rbp + 0x5f0], rax
14146eefa: movups xmm1, xmmword ptr [rsi]
14146eefd: movaps xmmword ptr [rbp + 0x600], xmm1
14146ef04: mov dword ptr [rbp + 0x610], r10d
14146ef0b: mov byte ptr [rbp + 0x614], r9b
14146ef12: movups xmm1, xmmword ptr [rsi + 0x18]
14146ef16: movups xmmword ptr [rbp + 0x618], xmm1
14146ef1d: movups xmm1, xmmword ptr [rsi + 0x28]
14146ef21: movups xmmword ptr [rbp + 0x628], xmm1
14146ef28: mov dword ptr [rbp + 0x638], r8d
14146ef2f: movss dword ptr [rbp + 0x63c], xmm0
14146ef37: mov word ptr [rbp + 0x640], dx
14146ef3e: mov byte ptr [rbp + 0x642], cl
14146ef44: movzx eax, byte ptr [rip + 0x21c5cbe]
14146ef4b: mov ecx, 0xf0
14146ef50: mov edx, 8
14146ef55: call 0x140613c10 ; 
14146ef5a: test rax, rax
14146ef5d: mov rbx, qword ptr [rbp + 0x948]
14146ef64: mov r14, qword ptr [rbp + 0x900]
14146ef6b: je 0x14146f94e
14146ef71: lea rdx, [rbp + 0x5b0]
14146ef78: mov r8d, 0xf0
14146ef7e: mov qword ptr [rbp + 0x958], rax
14146ef85: mov rcx, rax
14146ef88: call 0x142923c00 ; 
14146ef8d: mov rax, qword ptr [rip + 0x21bd414]
14146ef94: mov rax, qword ptr [rax]
14146ef97: cmp rax, 4
14146ef9b: mov rdi, qword ptr [rbp + 0x918]
14146efa2: jb 0x14146f09b
14146efa8: lea rcx, [rbp + 0x590]
14146efaf: call 0x140d63910 ; std::time::Instant::elapsed
14146efb4: mov qword ptr [rbp + 0x360], rax
14146efbb: mov dword ptr [rbp + 0x368], edx
14146efc1: mov qword ptr [rbp + 0x1b0], r13
14146efc8: lea rax, [rip - 0x6e4a7f]
14146efcf: mov qword ptr [rbp + 0x1b8], rax
14146efd6: lea rcx, [rip + 0x17f5b7b]
14146efdd: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14146efe2: movups xmm0, xmmword ptr [rax]
14146efe5: mov eax, dword ptr [rax + 0x10]
14146efe8: mov qword ptr [rbp + 0x5e0], 4
14146eff3: lea rcx, [rip + 0x17f514e]
14146effa: mov qword ptr [rbp + 0x5e8], rcx
14146f001: mov qword ptr [rbp + 0x5f0], 0x26
14146f00c: lea rdx, [rip + 0x17f5b35]
14146f013: mov qword ptr [rbp + 0x600], rdx
14146f01a: mov qword ptr [rbp + 0x608], 1
14146f025: mov qword ptr [rbp + 0x610], r12
14146f02c: mov qword ptr [rbp + 0x618], 1
14146f037: mov qword ptr [rbp + 0x620], 0
14146f042: mov qword ptr [rbp + 0x5b0], 0
14146f04d: mov qword ptr [rbp + 0x5b8], rcx
14146f054: mov qword ptr [rbp + 0x5c0], 0x26
14146f05f: mov qword ptr [rbp + 0x5c8], 0
14146f06a: movups xmmword ptr [rbp + 0x5d0], xmm0
14146f071: mov dword ptr [rbp + 0x5f8], 1
14146f07b: mov dword ptr [rbp + 0x5fc], eax
14146f081: lea rcx, [rbp + 0x972]
14146f088: lea rdx, [rbp + 0x5b0]
14146f08f: call 0x140d4a330 ; log::__private_api::impl$2::log
14146f094: mov rbx, qword ptr [rbp + 0x948]
14146f09b: mov rsi, qword ptr [rbp + 0xa68]
14146f0a2: mov edx, 0xf0
14146f0a7: mov r8d, 8
14146f0ad: mov rcx, rbx
14146f0b0: call 0x140613c20 ; 
14146f0b5: mov rax, qword ptr [rsi]
14146f0b8: movss xmm0, dword ptr [rax + 0x114]
14146f0c0: mov rax, qword ptr [rbp + 0x560]
14146f0c7: mov ecx, dword ptr [rbp + 0x8c0]
14146f0cd: mov dword ptr [rax], ecx
14146f0cf: mov rax, qword ptr [rbp + 0x8a8]
14146f0d6: movss dword ptr [rax + 0x194], xmm0
14146f0de: mov edi, dword ptr [rdi + 0x68]
14146f0e1: mov rax, qword ptr [rbp + 0x910]
14146f0e8: sub eax, edi
14146f0ea: mov dword ptr [rbp + 0x858], eax
14146f0f0: mov rcx, qword ptr [rbp + 0x550]
14146f0f7: test rcx, rcx
14146f0fa: je 0x14146f1a5
14146f100: cmp edi, -1
14146f103: jne 0x14146fa13
14146f109: cmp eax, ecx
14146f10b: jge 0x14146f1a7
14146f111: lea rax, [rbp + 0x550]
14146f118: mov qword ptr [rbp + 0x360], rax
14146f11f: lea rax, [rip - 0x6e1a66]
14146f126: mov qword ptr [rbp + 0x368], rax
14146f12d: lea rax, [rbp + 0x858]
14146f134: mov qword ptr [rbp + 0x370], rax
14146f13b: lea rax, [rip - 0x6e1bc2]
14146f142: mov qword ptr [rbp + 0x378], rax
14146f149: lea rax, [rip + 0x17f5288]
14146f150: mov qword ptr [rbp + 0x5b0], rax
14146f157: mov qword ptr [rbp + 0x5b8], 3
14146f162: mov qword ptr [rbp + 0x5d0], 0
14146f16d: mov qword ptr [rbp + 0x5c0], r13
14146f174: mov qword ptr [rbp + 0x5c8], 2
14146f17f: mov byte ptr [rbp + 0x975], 1
14146f186: mov byte ptr [rbp + 0x974], 1
14146f18d: lea rdx, [rip + 0x17f5274]
14146f194: lea rcx, [rbp + 0x5b0]
14146f19b: call 0x1428d9390 ; core::panicking::panic_fmt
14146f1a0: jmp 0x14146fa70
14146f1a5: xor ecx, ecx
14146f1a7: mov edx, dword ptr [rbp + 0x8cc]
14146f1ad: mov r8, qword ptr [rbp + 0x588]
14146f1b4: mov dword ptr [r8], edx
14146f1b7: sub eax, ecx
14146f1b9: mov byte ptr [r14 + 0x48], 1
14146f1be: mov dword ptr [r14 + 0x4c], eax
14146f1c2: mov rax, qword ptr [rbp + 0x870]
14146f1c9: mov rcx, qword ptr [rax + 0xf8]
14146f1d0: mov rdx, qword ptr [rax + 0x100]
14146f1d7: movzx r8d, byte ptr [rax + 0x108]
14146f1df: test r8b, r8b
14146f1e2: je 0x14146f28b
14146f1e8: lea r9, [rbp + 0x370]
14146f1ef: lea r10, [rbp + 0x378]
14146f1f6: lea r11, [rbp + 0x368]
14146f1fd: lea rsi, [rdx*4]
14146f205: xor r15d, r15d
14146f208: mov rax, rcx
14146f20b: nop dword ptr [rax + rax]
14146f210: test rsi, rsi
14146f213: je 0x14146f371
14146f219: mov r14d, dword ptr [rax]
14146f21c: add rax, 4
14146f220: mov rbx, qword ptr [rbp + 0x8f8]
14146f227: mov rbx, qword ptr [rbx + 0x1a8]
14146f22e: lea r12, [r14 + r14*8]
14146f232: mov r14, qword ptr [rbx + r12*8 + 0x10]
14146f237: add rsi, -4
14146f23b: test r14, r14
14146f23e: je 0x14146f210
14146f240: lea rsi, [rbx + r12*8]
14146f244: mov rbx, qword ptr [rbp + 0x870]
14146f24b: mov rbx, qword ptr [rbx + 0x110]
14146f252: mov r15, qword ptr [rsi + 0x18]
14146f256: mov rsi, qword ptr [rsi + 0x38]
14146f25a: mov rsi, qword ptr [rsi + rbx*8]
14146f25e: not rsi
14146f261: lea r12, [rsi + rsi*2]
14146f265: shl r12, 4
14146f269: mov rsi, qword ptr [r15 + r12 + 0x28]
14146f26e: mov rbx, qword ptr [r15 + r12 + 0x10]
14146f273: mov r15, qword ptr [r15 + r12 + 0x20]
14146f278: mov qword ptr [rbp + 0x360], rbx
14146f27f: mov qword ptr [rbp + 0x368], r15
14146f286: jmp 0x14146f35a
14146f28b: lea r9, [rbp + 0x5c0]
14146f292: lea r10, [rbp + 0x5c8]
14146f299: lea r11, [rbp + 0x5b8]
14146f2a0: lea rsi, [rdx*4]
14146f2a8: xor r15d, r15d
14146f2ab: mov rax, rcx
14146f2ae: nop 
14146f2b0: test rsi, rsi
14146f2b3: je 0x14146f371
14146f2b9: mov r14d, dword ptr [rax]
14146f2bc: add rax, 4
14146f2c0: mov rbx, qword ptr [rbp + 0x8f8]
14146f2c7: mov rbx, qword ptr [rbx + 0x100]
14146f2ce: lea r12, [r14 + r14*4]
14146f2d2: shl r12, 5
14146f2d6: mov r14, qword ptr [rbx + r12 + 0x58]
14146f2db: add rsi, -4
14146f2df: test r14, r14
14146f2e2: je 0x14146f2b0
14146f2e4: add rbx, r12
14146f2e7: mov esi, dword ptr [rbx + 0x94]
14146f2ed: mov r15, qword ptr [rbp + 0x8f8]
14146f2f4: mov r15, qword ptr [r15 + 0x1a8]
14146f2fb: lea rsi, [rsi + rsi*8]
14146f2ff: mov r12, qword ptr [rbp + 0x870]
14146f306: mov r12, qword ptr [r12 + 0x110]
14146f30e: mov r13, qword ptr [r15 + rsi*8 + 0x18]
14146f313: mov rsi, qword ptr [r15 + rsi*8 + 0x38]
14146f318: mov rsi, qword ptr [rsi + r12*8]
14146f31c: not rsi
14146f31f: mov rbx, qword ptr [rbx + 0x50]
14146f323: lea r15, [rsi + rsi*2]
14146f327: shl r15, 4
14146f32b: mov esi, dword ptr [rbx + 8]
14146f32e: lea rbx, [rsi + rsi*4]
14146f332: shl rbx, 4
14146f336: add rbx, qword ptr [r13 + r15 + 0x10]
14146f33b: shl rsi, 2
14146f33f: mov r12, qword ptr [r13 + r15 + 0x20]
14146f344: add r12, rsi
14146f347: add rsi, qword ptr [r13 + r15 + 0x28]
14146f34c: mov qword ptr [rbp + 0x5b0], rbx
14146f353: mov qword ptr [rbp + 0x5b8], r12
14146f35a: mov qword ptr [r9], rsi
14146f35d: movlps qword ptr [r10], xmm6
14146f361: mov r9, qword ptr [r11 + 8]
14146f365: mov r10, qword ptr [r11 + 0x10]
14146f369: mov r15d, 1
14146f36f: jmp 0x14146f376
14146f371: xor r14d, r14d
14146f374: xor ebx, ebx
14146f376: mov rsi, qword ptr [rbp + 0x960]
14146f37d: mov r12, qword ptr [rbp + 0x950]
14146f384: lea rcx, [rcx + rdx*4]
14146f388: test r8b, r8b
14146f38b: je 0x14146f3c4
14146f38d: cmp r15, r14
14146f390: mov r11, qword ptr [rbp + 0x8f8]
14146f397: jne 0x14146f3f6
14146f399: nop dword ptr [rax]
14146f3a0: cmp rax, rcx
14146f3a3: je 0x14146f45f
14146f3a9: mov edx, dword ptr [rax]
14146f3ab: add rax, 4
14146f3af: mov r8, qword ptr [r11 + 0x1a8]
14146f3b6: lea rdx, [rdx + rdx*8]
14146f3ba: cmp qword ptr [r8 + rdx*8 + 0x10], 0
14146f3c0: je 0x14146f3a0
14146f3c2: jmp 0x14146f3f6
14146f3c4: cmp r15, r14
14146f3c7: mov r11, qword ptr [rbp + 0x8f8]
14146f3ce: jne 0x14146f3f6
14146f3d0: cmp rax, rcx
14146f3d3: je 0x14146f45f
14146f3d9: mov edx, dword ptr [rax]
14146f3db: add rax, 4
14146f3df: mov r8, qword ptr [r11 + 0x100]
14146f3e6: lea rdx, [rdx + rdx*4]
14146f3ea: shl rdx, 5
14146f3ee: cmp qword ptr [r8 + rdx + 0x58], 0
14146f3f4: je 0x14146f3d0
14146f3f6: test rbx, rbx
14146f3f9: sete al
14146f3fc: xor al, 1
14146f3fe: movzx eax, al
14146f401: mov qword ptr [rbp + 0x5b0], rax
14146f408: lea rax, [rip + 0x17f4ba3]
14146f40f: mov qword ptr [rbp + 0x5b8], rax
14146f416: mov qword ptr [rbp + 0x5c0], 0x5f
14146f421: mov byte ptr [rbp + 0x975], 1
14146f428: mov byte ptr [rbp + 0x974], 1
14146f42f: lea rax, [rip + 0x17f4fea]
14146f436: mov qword ptr [rsp + 0x20], rax
14146f43b: lea rcx, [rip + 0x17f4a3e]
14146f442: lea r9, [rip + 0x17f4a67]
14146f449: lea r8, [rbp + 0x5b0]
14146f450: mov edx, 0x2b
14146f455: call 0x1428d9760 ; core::result::unwrap_failed
14146f45a: jmp 0x14146fa70
14146f45f: test rbx, rbx
14146f462: sete al
14146f465: je 0x14146f3fc
14146f467: shr r10, 0x20
14146f46b: mov dword ptr [r9], r10d
14146f46e: mov rax, qword ptr [rbx]
14146f471: mov rcx, rax
14146f474: neg rcx
14146f477: jo 0x14146f492
14146f479: jae 0x14146f492
14146f47b: mov rcx, qword ptr [rbx + 8]
14146f47f: shl rax, 3
14146f483: lea rdx, [rax + rax*4]
14146f487: mov r8d, 4
14146f48d: call 0x140613c20 ; 
14146f492: mov qword ptr [rbx], r12
14146f495: mov qword ptr [rbx + 8], rsi
14146f499: mov rax, qword ptr [rbp + 0x8d8]
14146f4a0: mov qword ptr [rbx + 0x10], rax
14146f4a4: cmp dword ptr [rbp + 0x910], edi
14146f4aa: jne 0x14146f50f
14146f4ac: mov rax, qword ptr [rbp + 0x530]
14146f4b3: mov ecx, dword ptr [rbp + 0x860]
14146f4b9: mov dword ptr [rax], ecx
14146f4bb: mov rax, qword ptr [rbp + 0x878]
14146f4c2: mov rsi, qword ptr [rax + 0x30]
14146f4c6: mov rdi, qword ptr [rax + 0x40]
14146f4ca: cmp rsi, qword ptr [rax + 0x20]
14146f4ce: jne 0x14146f4f5
14146f4d0: mov rax, qword ptr [rbp + 0x878]
14146f4d7: lea rcx, [rax + 0x20]
14146f4db: mov byte ptr [rbp + 0x975], 0
14146f4e2: mov byte ptr [rbp + 0x974], 1
14146f4e9: lea rdx, [rip + 0x1819b88]
14146f4f0: call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
14146f4f5: mov rcx, qword ptr [rbp + 0x878]
14146f4fc: mov rax, qword ptr [rcx + 0x28]
14146f500: mov qword ptr [rax + rsi*8], rdi
14146f504: inc rsi
14146f507: mov qword ptr [rcx + 0x30], rsi
14146f50b: inc qword ptr [rcx + 0x40]
14146f50f: mov rax, qword ptr [rbp + 0x8e0]
14146f516: movups xmm0, xmmword ptr [rax]
14146f519: movaps xmmword ptr [rbp + 0x5b0], xmm0
14146f520: mov byte ptr [rbp + 0x975], 0
14146f527: mov byte ptr [rbp + 0x974], 1
14146f52e: lea rcx, [rbp + 0x1b0]
14146f535: lea rdx, [rbp + 0x5b0]
14146f53c: call 0x140ae8470 ; country_core::systems::save::save_backup_list::SaveBackupList::new_from_session
14146f541: mov rax, qword ptr [rbp + 0xa40]
14146f548: mov rdx, qword ptr [rbp + 0x918]
14146f54f: mov rcx, qword ptr [rdx + 0x10]
14146f553: mov r8, qword ptr [rbp + 0x570]
14146f55a: mov r9d, dword ptr [rbp + 0x8c4]
14146f561: mov dword ptr [r8], r9d
14146f564: mov qword ptr [rdx + 0x58], rcx
14146f568: mov r8d, dword ptr [rax + 0x1c]
14146f56c: mov rdx, qword ptr [rax]
14146f56f: mov rax, qword ptr [rax + 0x10]
14146f573: mov qword ptr [rdx + 0x120], rcx
14146f57a: mov ecx, dword ptr [rbp + 0x1c8]
14146f580: mov dword ptr [rax], r8d
14146f583: mov dword ptr [rdx + 0x140], ecx
14146f589: mov dword ptr [rdx + 0x144], 0
14146f593: mov rax, qword ptr [rbp + 0x8e0]
14146f59a: movups xmm0, xmmword ptr [rax]
14146f59d: movups xmmword ptr [rbp + 0x5e0], xmm0
14146f5a4: movups xmm0, xmmword ptr [rbp + 0x1b0]
14146f5ab: movups xmm1, xmmword ptr [rbp + 0x1c0]
14146f5b2: movups xmmword ptr [rbp + 0x5b8], xmm0
14146f5b9: movups xmmword ptr [rbp + 0x5c8], xmm1
14146f5c0: mov rax, qword ptr [rbp + 0x958]
14146f5c7: mov qword ptr [rbp + 0x5d8], rax
14146f5ce: mov qword ptr [rbp + 0x5b0], 3
14146f5d9: mov byte ptr [rbp + 0x975], 0
14146f5e0: mov byte ptr [rbp + 0x974], 0
14146f5e7: lea rcx, [rbp + 0x360]
14146f5ee: lea r8, [rbp + 0x5b0]
14146f5f5: call 0x1408da280 ; std::sync::mpmc::Sender::send<enum2$<country_core::systems::save::save_game::AsyncSaveCmd> >
14146f5fa: cmp dword ptr [rbp + 0x360], 0xb
14146f601: jne 0x14146f962
14146f607: mov rdx, qword ptr [rbp + 0x830]
14146f60e: test rdx, rdx
14146f611: je 0x14146f625
14146f613: mov rcx, qword ptr [rbp + 0x838]
14146f61a: mov r8d, 1
14146f620: call 0x140613c20 ; 
14146f625: movaps xmm6, xmmword ptr [rbp + 0x980]
14146f62c: movaps xmm7, xmmword ptr [rbp + 0x990]
14146f633: movaps xmm8, xmmword ptr [rbp + 0x9a0]
14146f63b: movaps xmm9, xmmword ptr [rbp + 0x9b0]
14146f643: movaps xmm10, xmmword ptr [rbp + 0x9c0]
14146f64b: add rsp, 0xa58
14146f652: pop rbx
14146f653: pop rdi
14146f654: pop rsi
14146f655: pop r12
14146f657: pop r13
14146f659: pop r14
14146f65b: pop r15
14146f65d: pop rbp
14146f65e: ret 
14146f65f: mov rdi, qword ptr [rbp + 0x1b0]
14146f666: mov bl, byte ptr [rbp + 0x1bf]
14146f66c: lea rcx, [rbp - 0x50]
14146f670: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
14146f675: mov rcx, rax
14146f678: call 0x140c96a80 ; utils::file::copy_file_to_backup
14146f67d: test rax, rax
14146f680: je 0x14146f87f
14146f686: lea rcx, [rip + 0x17f4dc3]
14146f68d: mov qword ptr [rbp + 0x360], rcx
14146f694: mov qword ptr [rbp + 0x368], 0x23
14146f69f: mov qword ptr [rbp + 0x370], rax
14146f6a6: mov qword ptr [rbp + 0x5b0], 3
14146f6b1: lea rcx, [rbp + 0x360]
14146f6b8: lea rdx, [rbp + 0x5b0]
14146f6bf: call 0x1428a8a20 ; anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,anyhow::Error> >
14146f6c4: mov rdi, rax
14146f6c7: lea rcx, [rbp + 0x1b0]
14146f6ce: call 0x14088fa00 ; core::ptr::drop_in_place<country_core::systems::save::load_game::ParsedGameData>
14146f6d3: jmp 0x14146e18e
14146f6d8: mov rcx, qword ptr [rbp + 0x218]
14146f6df: test rcx, rcx
14146f6e2: je 0x14146f79e
14146f6e8: mov rax, qword ptr [rbp + 0x210]
14146f6ef: lea rcx, [rcx + rcx*4]
14146f6f3: lea rcx, [rax + rcx*8]
14146f6f7: lea rdx, [rip + 0x17f55ce]
14146f6fe: jmp 0x14146f70d
14146f700: add rax, 0x28
14146f704: cmp rax, rcx
14146f707: je 0x14146f79e
14146f70d: mov r9, qword ptr [rax + 0x20]
14146f711: test r9, r9
14146f714: je 0x14146f700
14146f716: mov r8, qword ptr [rax + 0x18]
14146f71a: imul r9, r9, 0x58
14146f71e: xor r10d, r10d
14146f721: jmp 0x14146f741
14146f723: nop word ptr cs:[rax + rax]
14146f730: mov r11d, esi
14146f733: mov dword ptr [r8 + r10 + 0x30], r11d
14146f738: add r10, 0x58
14146f73c: cmp r9, r10
14146f73f: je 0x14146f700
14146f741: movzx r11d, byte ptr [r8 + r10 + 0x52]
14146f747: add r11b, 0xc8
14146f74b: cmp r11b, 2
14146f74f: ja 0x14146f738
14146f751: mov esi, dword ptr [r8 + r10 + 0x30]
14146f756: cmp rsi, 8
14146f75a: ja 0x14146f76e
14146f75c: movsxd r11, dword ptr [rdx + rsi*4]
14146f760: add r11, rdx
14146f763: jmp r11
14146f766: mov r11d, 2
14146f76c: jmp 0x14146f733
14146f76e: mov r11d, 1
14146f774: cmp esi, 0x3e7
14146f77a: je 0x14146f730
14146f77c: jmp 0x14146f733
14146f77e: mov r11d, 3
14146f784: jmp 0x14146f733
14146f786: mov r11d, 5
14146f78c: jmp 0x14146f733
14146f78e: mov r11d, 8
14146f794: jmp 0x14146f733
14146f796: mov r11d, 4
14146f79c: jmp 0x14146f733
14146f79e: lea r12, [rbp + 0x360]
14146f7a5: lea rdx, [rbp + 0x1b0]
14146f7ac: mov r8d, 0x1b0
14146f7b2: mov rcx, r12
14146f7b5: call 0x142923c00 ; 
14146f7ba: lea r14, [rbp + 0x5b0]
14146f7c1: mov r8d, 0x280
14146f7c7: mov rcx, r14
14146f7ca: mov rdx, r15
14146f7cd: call 0x142923c00 ; 
14146f7d2: mov byte ptr [rbp + 0x973], 0
14146f7d9: mov rcx, r12
14146f7dc: mov rdx, qword ptr [rbp + 0x580]
14146f7e3: mov r8, r14
14146f7e6: call 0x1417a9c50 ; country_core::systems::save::snapshot::init_game_state_from_snapshot
14146f7eb: bt qword ptr [rbp + 0x910], 0x28
14146f7f4: mov rax, qword ptr [rbp + 0x918]
14146f7fb: mov dword ptr [rax + 0x68], edi
14146f7fe: mov rdi, rax
14146f801: jae 0x14146dfb4
14146f807: mov rax, qword ptr [rdi + 0x20]
14146f80b: mov qword ptr [rbp + 0x928], rax
14146f812: mov rax, qword ptr [rdi + 0x28]
14146f816: mov qword ptr [rdi + 0x28], 0
14146f81e: mov qword ptr [rbp + 0x938], rax
14146f825: test rax, rax
14146f828: je 0x14146dfb4
14146f82e: mov rsi, qword ptr [rbp + 0x938]
14146f835: neg rsi
14146f838: mov ecx, 1
14146f83d: mov r15, qword ptr [rbp + 0x928]
14146f844: jmp 0x14146f865
14146f846: mov rcx, qword ptr [rbp + 0x958]
14146f84d: lea rax, [rsi + rcx]
14146f851: inc rax
14146f854: inc rcx
14146f857: add r15, 0x10
14146f85b: cmp rax, 1
14146f85f: je 0x14146dfb4
14146f865: mov qword ptr [rbp + 0x958], rcx
14146f86c: mov rax, qword ptr [r15]
14146f86f: lock dec qword ptr [rax]
14146f873: jne 0x14146f846
14146f875: mov rcx, r15
14146f878: call 0x1402e5d80 ; alloc::sync::Arc::drop_slow<core::sync::atomic::AtomicBool,alloc::alloc::Global>
14146f87d: jmp 0x14146f846
14146f87f: mov eax, dword ptr [rbp + 0x1b8]
14146f885: mov ecx, dword ptr [rbp + 0x1bb]
14146f88b: mov dword ptr [rbp + 0x8f3], ecx
14146f891: mov dword ptr [rbp + 0x8f0], eax
14146f897: cmp bl, 2
14146f89a: je 0x14146e18e
14146f8a0: jmp 0x14146ea49
14146f8a5: lea rcx, [rip + 0x17f47e4]
14146f8ac: lea r8, [rip + 0x17f4835]
14146f8b3: mov edx, 0x58
14146f8b8: call 0x1428d9430 ; core::panicking::panic
14146f8bd: mov rax, qword ptr [rbp + 0x5b8]
14146f8c4: mov qword ptr [rbp + 0x368], rax
14146f8cb: mov qword ptr [rbp + 0x360], 2
14146f8d6: lea rcx, [rbp + 0x368]
14146f8dd: call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error>
14146f8e2: lea rax, [rbp + 0x518]
14146f8e9: mov qword ptr [rbp + 0x360], rax
14146f8f0: lea rax, [rip - 0x1441007]
14146f8f7: mov qword ptr [rbp + 0x368], rax
14146f8fe: lea rax, [rip + 0x17f471b]
14146f905: mov qword ptr [rbp + 0x5b0], rax
14146f90c: mov qword ptr [rbp + 0x5b8], 2
14146f917: mov qword ptr [rbp + 0x5d0], 0
14146f922: lea rax, [rbp + 0x360]
14146f929: mov qword ptr [rbp + 0x5c0], rax
14146f930: mov qword ptr [rbp + 0x5c8], 1
14146f93b: lea rdx, [rip + 0x17f4736]
14146f942: lea rcx, [rbp + 0x5b0]
14146f949: call 0x1428d9390 ; core::panicking::panic_fmt
14146f94e: mov ecx, 8
14146f953: mov edx, 0xf0
14146f958: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14146f95d: jmp 0x14146fa70
14146f962: movups xmm0, xmmword ptr [rbp + 0x360]
14146f969: movups xmm1, xmmword ptr [rbp + 0x370]
14146f970: movups xmm2, xmmword ptr [rbp + 0x380]
14146f977: movups xmm3, xmmword ptr [rbp + 0x390]
14146f97e: movaps xmmword ptr [rbp + 0x5e0], xmm3
14146f985: movaps xmmword ptr [rbp + 0x5d0], xmm2
14146f98c: movaps xmmword ptr [rbp + 0x5c0], xmm1
14146f993: movaps xmmword ptr [rbp + 0x5b0], xmm0
14146f99a: lea rax, [rip + 0x17f4a97]
14146f9a1: mov qword ptr [rsp + 0x20], rax
14146f9a6: lea rcx, [rip + 0x17f44d3]
14146f9ad: lea r9, [rip + 0x17f44ac]
14146f9b4: lea r8, [rbp + 0x5b0]
14146f9bb: mov edx, 0x2b
14146f9c0: call 0x1428d9760 ; core::result::unwrap_failed
14146f9c5: jmp 0x14146fa70
14146f9ca: lea r8, [rip + 0x17c9177]
14146f9d1: mov rcx, r13
14146f9d4: mov rdx, r12
14146f9d7: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14146f9dc: jmp 0x14146fa70
14146f9e1: movabs rax, 0x8000000000000000
14146f9eb: mov qword ptr [rbp + 0x930], rax
14146f9f2: mov byte ptr [rbp + 0x976], 1
14146f9f9: lea rcx, [rip + 0x17f47d0]
14146fa00: lea r8, [rip + 0x17f47f1]
14146fa07: mov edx, 0x28
14146fa0c: call 0x1428d9430 ; core::panicking::panic
14146fa11: jmp 0x14146fa70
14146fa13: lea rax, [rip + 0x17f495e]
14146fa1a: mov qword ptr [rbp + 0x5b0], rax
14146fa21: mov qword ptr [rbp + 0x5b8], 1
14146fa2c: mov qword ptr [rbp + 0x5c0], 8
14146fa37: xorps xmm0, xmm0
14146fa3a: movups xmmword ptr [rbp + 0x5c8], xmm0
14146fa41: mov byte ptr [rbp + 0x975], 1
14146fa48: mov byte ptr [rbp + 0x974], 1
14146fa4f: lea rdx, [rip + 0x17f4932]
14146fa56: lea rcx, [rbp + 0x5b0]
14146fa5d: call 0x1428d9390 ; core::panicking::panic_fmt
14146fa62: jmp 0x14146fa70
14146fa64: lea rcx, [rip + 0x17f47a5]
14146fa6b: call 0x1428d9310 ; core::option::unwrap_failed
14146fa70: ud2 
14146fa72: nop word ptr cs:[rax + rax]
14146fa80: mov qword ptr [rsp + 0x10], rdx
14146fa85: push rbp
14146fa86: push r15
14146fa88: push r14
14146fa8a: push r13
14146fa8c: push r12
14146fa8e: push rsi
14146fa8f: push rdi
14146fa90: push rbx
14146fa91: sub rsp, 0x78
14146fa95: lea rbp, [rdx + 0x80]
14146fa9c: movaps xmmword ptr [rsp + 0x20], xmm10
14146faa2: movaps xmmword ptr [rsp + 0x30], xmm9
14146faa8: movaps xmmword ptr [rsp + 0x40], xmm8
14146faae: movaps xmmword ptr [rsp + 0x50], xmm7
14146fab3: movaps xmmword ptr [rsp + 0x60], xmm6
14146fab8: mov rdx, qword ptr [rbp + 0x360]
14146fabf: test rdx, rdx
14146fac2: je 0x14146fad6
14146fac4: mov rcx, qword ptr [rbp + 0x368]
14146facb: mov r8d, 1
14146fad1: call 0x140613c20 ; 
14146fad6: movabs rax, 0x8000000000000000
14146fae0: mov qword ptr [rbp + 0x930], rax
14146fae7: mov byte ptr [rbp + 0x976], 1
14146faee: movaps xmm6, xmmword ptr [rsp + 0x60]
14146faf3: movaps xmm7, xmmword ptr [rsp + 0x50]
14146faf8: movaps xmm8, xmmword ptr [rsp + 0x40]
14146fafe: movaps xmm9, xmmword ptr [rsp + 0x30]
14146fb04: movaps xmm10, xmmword ptr [rsp + 0x20]
14146fb0a: add rsp, 0x78
14146fb0e: pop rbx
14146fb0f: pop rdi
14146fb10: pop rsi
14146fb11: pop r12
14146fb13: pop r13
14146fb15: pop r14
14146fb17: pop r15
14146fb19: pop rbp
14146fb1a: ret 
14146fb1b: nop dword ptr [rax + rax]
14146fb20: mov qword ptr [rsp + 0x10], rdx
14146fb25: push rbp
14146fb26: push r15
14146fb28: push r14
14146fb2a: push r13
14146fb2c: push r12
14146fb2e: push rsi
14146fb2f: push rdi
14146fb30: push rbx
14146fb31: sub rsp, 0x78
14146fb35: lea rbp, [rdx + 0x80]
14146fb3c: movaps xmmword ptr [rsp + 0x20], xmm10
14146fb42: movaps xmmword ptr [rsp + 0x30], xmm9
14146fb48: movaps xmmword ptr [rsp + 0x40], xmm8
14146fb4e: movaps xmmword ptr [rsp + 0x50], xmm7
14146fb53: movaps xmmword ptr [rsp + 0x60], xmm6
14146fb58: mov rdx, qword ptr [rbp + 0x1b0]
14146fb5f: test rdx, rdx
14146fb62: je 0x14146fb76
14146fb64: mov rcx, qword ptr [rbp + 0x1b8]
14146fb6b: mov r8d, 1
14146fb71: call 0x140613c20 ; 
14146fb76: movaps xmm6, xmmword ptr [rsp + 0x60]
14146fb7b: movaps xmm7, xmmword ptr [rsp + 0x50]
14146fb80: movaps xmm8, xmmword ptr [rsp + 0x40]
14146fb86: movaps xmm9, xmmword ptr [rsp + 0x30]
14146fb8c: movaps xmm10, xmmword ptr [rsp + 0x20]
14146fb92: add rsp, 0x78
14146fb96: pop rbx
14146fb97: pop rdi
14146fb98: pop rsi
14146fb99: pop r12
14146fb9b: pop r13
14146fb9d: pop r14
14146fb9f: pop r15
14146fba1: pop rbp
14146fba2: ret 
14146fba3: nop word ptr cs:[rax + rax]
14146fbb0: mov qword ptr [rsp + 0x10], rdx
14146fbb5: push rbp
14146fbb6: push r15
14146fbb8: push r14
14146fbba: push r13
14146fbbc: push r12
14146fbbe: push rsi
14146fbbf: push rdi
14146fbc0: push rbx
14146fbc1: sub rsp, 0x78
14146fbc5: lea rbp, [rdx + 0x80]
14146fbcc: movaps xmmword ptr [rsp + 0x20], xmm10
14146fbd2: movaps xmmword ptr [rsp + 0x30], xmm9
14146fbd8: movaps xmmword ptr [rsp + 0x40], xmm8
14146fbde: movaps xmmword ptr [rsp + 0x50], xmm7
14146fbe3: movaps xmmword ptr [rsp + 0x60], xmm6
14146fbe8: mov rcx, qword ptr [rbp + 0x938]
14146fbef: mov rax, qword ptr [rbp + 0x948]
14146fbf6: mov rdx, qword ptr [rax + 8]
14146fbfa: test rdx, rdx
14146fbfd: je 0x14146fc0f
14146fbff: mov rax, qword ptr [rbp + 0x948]
14146fc06: mov r8, qword ptr [rax + 0x10]
14146fc0a: call 0x140613c20 ; 
14146fc0f: mov edx, 0x18
14146fc14: mov r8d, 8
14146fc1a: mov rcx, qword ptr [rbp + 0x910]
14146fc21: call 0x140613c20 ; 
14146fc26: movaps xmm6, xmmword ptr [rsp + 0x60]
14146fc2b: movaps xmm7, xmmword ptr [rsp + 0x50]
14146fc30: movaps xmm8, xmmword ptr [rsp + 0x40]
14146fc36: movaps xmm9, xmmword ptr [rsp + 0x30]
14146fc3c: movaps xmm10, xmmword ptr [rsp + 0x20]
14146fc42: add rsp, 0x78
14146fc46: pop rbx
14146fc47: pop rdi
14146fc48: pop rsi
14146fc49: pop r12
14146fc4b: pop r13
14146fc4d: pop r14
14146fc4f: pop r15
14146fc51: pop rbp
14146fc52: ret 
14146fc53: nop word ptr cs:[rax + rax]
14146fc60: mov qword ptr [rsp + 0x10], rdx
14146fc65: push rbp
14146fc66: push r15
14146fc68: push r14
14146fc6a: push r13
14146fc6c: push r12
14146fc6e: push rsi
14146fc6f: push rdi
14146fc70: push rbx
14146fc71: sub rsp, 0x78
14146fc75: lea rbp, [rdx + 0x80]
14146fc7c: movaps xmmword ptr [rsp + 0x20], xmm10
14146fc82: movaps xmmword ptr [rsp + 0x30], xmm9
14146fc88: movaps xmmword ptr [rsp + 0x40], xmm8
14146fc8e: movaps xmmword ptr [rsp + 0x50], xmm7
14146fc93: movaps xmmword ptr [rsp + 0x60], xmm6
14146fc98: mov rdx, qword ptr [rbp - 0x50]
14146fc9c: test rdx, rdx
14146fc9f: je 0x14146fcb0
14146fca1: mov rcx, qword ptr [rbp - 0x48]
14146fca5: mov r8d, 1
14146fcab: call 0x140613c20 ; 
14146fcb0: movabs rax, 0x8000000000000000
14146fcba: mov qword ptr [rbp + 0x930], rax
14146fcc1: mov byte ptr [rbp + 0x976], 1
14146fcc8: movaps xmm6, xmmword ptr [rsp + 0x60]
14146fccd: movaps xmm7, xmmword ptr [rsp + 0x50]
14146fcd2: movaps xmm8, xmmword ptr [rsp + 0x40]
14146fcd8: movaps xmm9, xmmword ptr [rsp + 0x30]
14146fcde: movaps xmm10, xmmword ptr [rsp + 0x20]
14146fce4: add rsp, 0x78
14146fce8: pop rbx
14146fce9: pop rdi
14146fcea: pop rsi
14146fceb: pop r12
14146fced: pop r13
14146fcef: pop r14
14146fcf1: pop r15
14146fcf3: pop rbp
14146fcf4: ret 
14146fcf5: nop word ptr cs:[rax + rax]
14146fd00: mov qword ptr [rsp + 0x10], rdx
14146fd05: push rbp
14146fd06: push r15
14146fd08: push r14
14146fd0a: push r13
14146fd0c: push r12
14146fd0e: push rsi
14146fd0f: push rdi
14146fd10: push rbx
14146fd11: sub rsp, 0x78
14146fd15: lea rbp, [rdx + 0x80]
14146fd1c: movaps xmmword ptr [rsp + 0x20], xmm10
14146fd22: movaps xmmword ptr [rsp + 0x30], xmm9
14146fd28: movaps xmmword ptr [rsp + 0x40], xmm8
14146fd2e: movaps xmmword ptr [rsp + 0x50], xmm7
14146fd33: movaps xmmword ptr [rsp + 0x60], xmm6
14146fd38: lea rcx, [rbp + 0x5b0]
14146fd3f: call 0x14088fa00 ; core::ptr::drop_in_place<country_core::systems::save::load_game::ParsedGameData>
14146fd44: movaps xmm6, xmmword ptr [rsp + 0x60]
14146fd49: movaps xmm7, xmmword ptr [rsp + 0x50]
14146fd4e: movaps xmm8, xmmword ptr [rsp + 0x40]
14146fd54: movaps xmm9, xmmword ptr [rsp + 0x30]
14146fd5a: movaps xmm10, xmmword ptr [rsp + 0x20]
14146fd60: add rsp, 0x78
14146fd64: pop rbx
14146fd65: pop rdi
14146fd66: pop rsi
14146fd67: pop r12
14146fd69: pop r13
14146fd6b: pop r14
14146fd6d: pop r15
14146fd6f: pop rbp
14146fd70: ret 
14146fd71: nop word ptr cs:[rax + rax]
14146fd80: mov qword ptr [rsp + 0x10], rdx
14146fd85: push rbp
14146fd86: push r15
14146fd88: push r14
14146fd8a: push r13
14146fd8c: push r12
14146fd8e: push rsi
14146fd8f: push rdi
14146fd90: push rbx
14146fd91: sub rsp, 0x78
14146fd95: lea rbp, [rdx + 0x80]
14146fd9c: movaps xmmword ptr [rsp + 0x20], xmm10
14146fda2: movaps xmmword ptr [rsp + 0x30], xmm9
14146fda8: movaps xmmword ptr [rsp + 0x40], xmm8
14146fdae: movaps xmmword ptr [rsp + 0x50], xmm7
14146fdb3: movaps xmmword ptr [rsp + 0x60], xmm6
14146fdb8: lea rcx, [rbp + 0x868]
14146fdbf: call 0x140d4cc30 ; anyhow::error::impl$6::drop
14146fdc4: movaps xmm6, xmmword ptr [rsp + 0x60]
14146fdc9: movaps xmm7, xmmword ptr [rsp + 0x50]
14146fdce: movaps xmm8, xmmword ptr [rsp + 0x40]
14146fdd4: movaps xmm9, xmmword ptr [rsp + 0x30]
14146fdda: movaps xmm10, xmmword ptr [rsp + 0x20]
14146fde0: add rsp, 0x78
14146fde4: pop rbx
14146fde5: pop rdi
14146fde6: pop rsi
14146fde7: pop r12
14146fde9: pop r13
14146fdeb: pop r14
14146fded: pop r15
14146fdef: pop rbp
14146fdf0: ret 
14146fdf1: nop word ptr cs:[rax + rax]
14146fe00: mov qword ptr [rsp + 0x10], rdx
14146fe05: push rbp
14146fe06: push r15
14146fe08: push r14
14146fe0a: push r13
14146fe0c: push r12
14146fe0e: push rsi
14146fe0f: push rdi
14146fe10: push rbx
14146fe11: sub rsp, 0x78
14146fe15: lea rbp, [rdx + 0x80]
14146fe1c: movaps xmmword ptr [rsp + 0x20], xmm10
14146fe22: movaps xmmword ptr [rsp + 0x30], xmm9
14146fe28: movaps xmmword ptr [rsp + 0x40], xmm8
14146fe2e: movaps xmmword ptr [rsp + 0x50], xmm7
14146fe33: movaps xmmword ptr [rsp + 0x60], xmm6
14146fe38: mov rdx, qword ptr [rbp + 0x880]
14146fe3f: test rdx, rdx
14146fe42: je 0x14146fe56
14146fe44: mov rcx, qword ptr [rbp + 0x888]
14146fe4b: mov r8d, 1
14146fe51: call 0x140613c20 ; 
14146fe56: movaps xmm6, xmmword ptr [rsp + 0x60]
14146fe5b: movaps xmm7, xmmword ptr [rsp + 0x50]
14146fe60: movaps xmm8, xmmword ptr [rsp + 0x40]
14146fe66: movaps xmm9, xmmword ptr [rsp + 0x30]
14146fe6c: movaps xmm10, xmmword ptr [rsp + 0x20]
14146fe72: add rsp, 0x78
14146fe76: pop rbx
14146fe77: pop rdi
14146fe78: pop rsi
14146fe79: pop r12
14146fe7b: pop r13
14146fe7d: pop r14
14146fe7f: pop r15
14146fe81: pop rbp
14146fe82: ret 
14146fe83: nop word ptr cs:[rax + rax]
14146fe90: mov qword ptr [rsp + 0x10], rdx
14146fe95: push rbp
14146fe96: push r15
14146fe98: push r14
14146fe9a: push r13
14146fe9c: push r12
14146fe9e: push rsi
14146fe9f: push rdi
14146fea0: push rbx
14146fea1: sub rsp, 0x78
14146fea5: lea rbp, [rdx + 0x80]
14146feac: movaps xmmword ptr [rsp + 0x20], xmm10
14146feb2: movaps xmmword ptr [rsp + 0x30], xmm9
14146feb8: movaps xmmword ptr [rsp + 0x40], xmm8
14146febe: movaps xmmword ptr [rsp + 0x50], xmm7
14146fec3: movaps xmmword ptr [rsp + 0x60], xmm6
14146fec8: lea rcx, [rbp + 0x1b0]
14146fecf: call 0x14087b2f0 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<country_core::systems::save::load_game::PostGameLoadCommand>,alloc::alloc::Global> >
14146fed4: mov byte ptr [rbp + 0x96e], 1
14146fedb: movaps xmm6, xmmword ptr [rsp + 0x60]
14146fee0: movaps xmm7, xmmword ptr [rsp + 0x50]
14146fee5: movaps xmm8, xmmword ptr [rsp + 0x40]
14146feeb: movaps xmm9, xmmword ptr [rsp + 0x30]
14146fef1: movaps xmm10, xmmword ptr [rsp + 0x20]
14146fef7: add rsp, 0x78
14146fefb: pop rbx
14146fefc: pop rdi
14146fefd: pop rsi
14146fefe: pop r12
14146ff00: pop r13
14146ff02: pop r14
14146ff04: pop r15
14146ff06: pop rbp
14146ff07: ret 
14146ff08: nop dword ptr [rax + rax]
14146ff10: mov qword ptr [rsp + 0x10], rdx
14146ff15: push rbp
14146ff16: push r15
14146ff18: push r14
14146ff1a: push r13
14146ff1c: push r12
14146ff1e: push rsi
14146ff1f: push rdi
14146ff20: push rbx
14146ff21: sub rsp, 0x78
14146ff25: lea rbp, [rdx + 0x80]
14146ff2c: movaps xmmword ptr [rsp + 0x20], xmm10
14146ff32: movaps xmmword ptr [rsp + 0x30], xmm9
14146ff38: movaps xmmword ptr [rsp + 0x40], xmm8
14146ff3e: movaps xmmword ptr [rsp + 0x50], xmm7
14146ff43: movaps xmmword ptr [rsp + 0x60], xmm6
14146ff48: lea rcx, [rbp + 0x5b0]
14146ff4f: call 0x141328890 ; core::ptr::drop_in_place<enum2$<country_core::resources::session::SessionOnLoadCmd> >
14146ff54: movaps xmm6, xmmword ptr [rsp + 0x60]
14146ff59: movaps xmm7, xmmword ptr [rsp + 0x50]
14146ff5e: movaps xmm8, xmmword ptr [rsp + 0x40]
14146ff64: movaps xmm9, xmmword ptr [rsp + 0x30]
14146ff6a: movaps xmm10, xmmword ptr [rsp + 0x20]
14146ff70: add rsp, 0x78
14146ff74: pop rbx
14146ff75: pop rdi
14146ff76: pop rsi
14146ff77: pop r12
14146ff79: pop r13
14146ff7b: pop r14
14146ff7d: pop r15
14146ff7f: pop rbp
14146ff80: ret 
14146ff81: nop word ptr cs:[rax + rax]
14146ff90: mov qword ptr [rsp + 0x10], rdx
14146ff95: push rbp
14146ff96: push r15
14146ff98: push r14
14146ff9a: push r13
14146ff9c: push r12
14146ff9e: push rsi
14146ff9f: push rdi
14146ffa0: push rbx
14146ffa1: sub rsp, 0x78
14146ffa5: lea rbp, [rdx + 0x80]
14146ffac: movaps xmmword ptr [rsp + 0x20], xmm10
14146ffb2: movaps xmmword ptr [rsp + 0x30], xmm9
14146ffb8: movaps xmmword ptr [rsp + 0x40], xmm8
14146ffbe: movaps xmmword ptr [rsp + 0x50], xmm7
14146ffc3: movaps xmmword ptr [rsp + 0x60], xmm6
14146ffc8: lea rcx, [rbp + 0x5b0]
14146ffcf: call 0x141328890 ; core::ptr::drop_in_place<enum2$<country_core::resources::session::SessionOnLoadCmd> >
14146ffd4: movaps xmm6, xmmword ptr [rsp + 0x60]
14146ffd9: movaps xmm7, xmmword ptr [rsp + 0x50]
14146ffde: movaps xmm8, xmmword ptr [rsp + 0x40]
14146ffe4: movaps xmm9, xmmword ptr [rsp + 0x30]
14146ffea: movaps xmm10, xmmword ptr [rsp + 0x20]
14146fff0: add rsp, 0x78
14146fff4: pop rbx
14146fff5: pop rdi
14146fff6: pop rsi
14146fff7: pop r12
14146fff9: pop r13
14146fffb: pop r14
14146fffd: pop r15
14146ffff: pop rbp
141470000: ret 
141470001: nop word ptr cs:[rax + rax]
141470010: mov qword ptr [rsp + 0x10], rdx
141470015: push rbp
141470016: push r15
141470018: push r14
14147001a: push r13
14147001c: push r12
14147001e: push rsi
14147001f: push rdi
141470020: push rbx
141470021: sub rsp, 0x78
141470025: lea rbp, [rdx + 0x80]
14147002c: movaps xmmword ptr [rsp + 0x20], xmm10
141470032: movaps xmmword ptr [rsp + 0x30], xmm9
141470038: movaps xmmword ptr [rsp + 0x40], xmm8
14147003e: movaps xmmword ptr [rsp + 0x50], xmm7
141470043: movaps xmmword ptr [rsp + 0x60], xmm6
141470048: cmp byte ptr [rbp + 0x96e], 0
14147004f: je 0x141470091
141470051: lea rcx, [rbp + 0x360]
141470058: call 0x14088c400 ; core::ptr::drop_in_place<country_core::resources::session::NextSession>
14147005d: mov byte ptr [rbp + 0x96f], 0
141470064: movaps xmm6, xmmword ptr [rsp + 0x60]
141470069: movaps xmm7, xmmword ptr [rsp + 0x50]
14147006e: movaps xmm8, xmmword ptr [rsp + 0x40]
141470074: movaps xmm9, xmmword ptr [rsp + 0x30]
14147007a: movaps xmm10, xmmword ptr [rsp + 0x20]
141470080: add rsp, 0x78
141470084: pop rbx
141470085: pop rdi
141470086: pop rsi
141470087: pop r12
141470089: pop r13
14147008b: pop r14
14147008d: pop r15
14147008f: pop rbp
141470090: ret 
141470091: mov byte ptr [rbp + 0x96f], 0
141470098: movaps xmm6, xmmword ptr [rsp + 0x60]
14147009d: movaps xmm7, xmmword ptr [rsp + 0x50]
1414700a2: movaps xmm8, xmmword ptr [rsp + 0x40]
1414700a8: movaps xmm9, xmmword ptr [rsp + 0x30]
1414700ae: movaps xmm10, xmmword ptr [rsp + 0x20]
1414700b4: add rsp, 0x78
1414700b8: pop rbx
1414700b9: pop rdi
1414700ba: pop rsi
1414700bb: pop r12
1414700bd: pop r13
1414700bf: pop r14
1414700c1: pop r15
1414700c3: pop rbp
1414700c4: ret 
1414700c5: nop word ptr cs:[rax + rax]
1414700d0: mov qword ptr [rsp + 0x10], rdx
1414700d5: push rbp
1414700d6: push r15
1414700d8: push r14
1414700da: push r13
1414700dc: push r12
1414700de: push rsi
1414700df: push rdi
1414700e0: push rbx
1414700e1: sub rsp, 0x78
1414700e5: lea rbp, [rdx + 0x80]
1414700ec: movaps xmmword ptr [rsp + 0x20], xmm10
1414700f2: movaps xmmword ptr [rsp + 0x30], xmm9
1414700f8: movaps xmmword ptr [rsp + 0x40], xmm8
1414700fe: movaps xmmword ptr [rsp + 0x50], xmm7
141470103: movaps xmmword ptr [rsp + 0x60], xmm6
141470108: cmp byte ptr [rbp + 0x96f], 0
14147010f: je 0x14147014a
141470111: lea rcx, [rbp + 0x830]
141470118: call 0x140876a30 ; core::ptr::drop_in_place<alloc::vec::Vec<enum2$<country_core::systems::save::load_game::PostGameLoadCommand>,alloc::alloc::Global> >
14147011d: movaps xmm6, xmmword ptr [rsp + 0x60]
141470122: movaps xmm7, xmmword ptr [rsp + 0x50]
141470127: movaps xmm8, xmmword ptr [rsp + 0x40]
14147012d: movaps xmm9, xmmword ptr [rsp + 0x30]
141470133: movaps xmm10, xmmword ptr [rsp + 0x20]
141470139: add rsp, 0x78
14147013d: pop rbx
14147013e: pop rdi
14147013f: pop rsi
141470140: pop r12
141470142: pop r13
141470144: pop r14
141470146: pop r15
141470148: pop rbp
141470149: ret 
14147014a: movaps xmm6, xmmword ptr [rsp + 0x60]
14147014f: movaps xmm7, xmmword ptr [rsp + 0x50]
141470154: movaps xmm8, xmmword ptr [rsp + 0x40]
14147015a: movaps xmm9, xmmword ptr [rsp + 0x30]
141470160: movaps xmm10, xmmword ptr [rsp + 0x20]
141470166: add rsp, 0x78
14147016a: pop rbx
14147016b: pop rdi
14147016c: pop rsi
14147016d: pop r12
14147016f: pop r13
141470171: pop r14
141470173: pop r15
141470175: pop rbp
141470176: ret 
141470177: nop word ptr [rax + rax]
141470180: mov qword ptr [rsp + 0x10], rdx
141470185: push rbp
141470186: push r15
141470188: push r14
14147018a: push r13
14147018c: push r12
14147018e: push rsi
14147018f: push rdi
141470190: push rbx
141470191: sub rsp, 0x78
141470195: lea rbp, [rdx + 0x80]
14147019c: movaps xmmword ptr [rsp + 0x20], xmm10
1414701a2: movaps xmmword ptr [rsp + 0x30], xmm9
1414701a8: movaps xmmword ptr [rsp + 0x40], xmm8
1414701ae: movaps xmmword ptr [rsp + 0x50], xmm7
1414701b3: movaps xmmword ptr [rsp + 0x60], xmm6
1414701b8: movzx r14d, byte ptr [rbp + 0x970]
1414701c0: movzx ebx, byte ptr [rbp + 0x971]
1414701c7: mov rsi, qword ptr [rbp + 0x8a0]
1414701ce: mov rdi, qword ptr [rbp + 0x8d0]
1414701d5: mov rdx, qword ptr [rbp + 0x830]
1414701dc: test rdx, rdx
1414701df: je 0x1414701f3
1414701e1: mov rcx, qword ptr [rbp + 0x838]
1414701e8: mov r8d, 1
1414701ee: call 0x140613c20 ; 
1414701f3: test r14b, 1
1414701f7: je 0x14147025f
1414701f9: mov r14, qword ptr [rbp + 0x948]
141470200: mov rcx, r14
141470203: call 0x14088eb40 ; core::ptr::drop_in_place<country_core::systems::save::save_game::GameSaveData>
141470208: mov edx, 0xf0
14147020d: mov r8d, 8
141470213: mov rcx, r14
141470216: call 0x140613c20 ; 
14147021b: mov qword ptr [rbp + 0x930], rdi
141470222: mov qword ptr [rbp + 0x548], rsi
141470229: and bl, 1
14147022c: mov byte ptr [rbp + 0x976], bl
141470232: movaps xmm6, xmmword ptr [rsp + 0x60]
141470237: movaps xmm7, xmmword ptr [rsp + 0x50]
14147023c: movaps xmm8, xmmword ptr [rsp + 0x40]
141470242: movaps xmm9, xmmword ptr [rsp + 0x30]
141470248: movaps xmm10, xmmword ptr [rsp + 0x20]
14147024e: add rsp, 0x78
141470252: pop rbx
141470253: pop rdi
141470254: pop rsi
141470255: pop r12
141470257: pop r13
141470259: pop r14
14147025b: pop r15
14147025d: pop rbp
14147025e: ret 
14147025f: mov qword ptr [rbp + 0x930], rdi
141470266: mov qword ptr [rbp + 0x548], rsi
14147026d: and bl, 1
141470270: mov byte ptr [rbp + 0x976], bl
141470276: movaps xmm6, xmmword ptr [rsp + 0x60]
14147027b: movaps xmm7, xmmword ptr [rsp + 0x50]
141470280: movaps xmm8, xmmword ptr [rsp + 0x40]
141470286: movaps xmm9, xmmword ptr [rsp + 0x30]
14147028c: movaps xmm10, xmmword ptr [rsp + 0x20]
141470292: add rsp, 0x78
141470296: pop rbx
141470297: pop rdi
141470298: pop rsi
141470299: pop r12
14147029b: pop r13
14147029d: pop r14
14147029f: pop r15
1414702a1: pop rbp
1414702a2: ret 
1414702a3: nop word ptr cs:[rax + rax]
1414702b0: mov qword ptr [rsp + 0x10], rdx
1414702b5: push rbp
1414702b6: push r15
1414702b8: push r14
1414702ba: push r13
1414702bc: push r12
1414702be: push rsi
1414702bf: push rdi
1414702c0: push rbx
1414702c1: sub rsp, 0x78
1414702c5: lea rbp, [rdx + 0x80]
1414702cc: movaps xmmword ptr [rsp + 0x20], xmm10
1414702d2: movaps xmmword ptr [rsp + 0x30], xmm9
1414702d8: movaps xmmword ptr [rsp + 0x40], xmm8
1414702de: movaps xmmword ptr [rsp + 0x50], xmm7
1414702e3: movaps xmmword ptr [rsp + 0x60], xmm6
1414702e8: mov rdi, qword ptr [rbp + 0x950]
1414702ef: mov qword ptr [rbp + 0x940], rdi
1414702f6: mov rsi, qword ptr [rbp + 0x960]
1414702fd: mov qword ptr [rbp + 0x920], rsi
141470304: mov byte ptr [rbp + 0x977], 0
14147030b: lea rcx, [rbp + 0x1b0]
141470312: call 0x140894d60 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::history::MultiEdit,alloc::alloc::Global> >
141470317: mov qword ptr [rbp + 0x940], rdi
14147031e: mov qword ptr [rbp + 0x920], rsi
141470325: mov byte ptr [rbp + 0x977], 0
14147032c: movaps xmm6, xmmword ptr [rsp + 0x60]
141470331: movaps xmm7, xmmword ptr [rsp + 0x50]
141470336: movaps xmm8, xmmword ptr [rsp + 0x40]
14147033c: movaps xmm9, xmmword ptr [rsp + 0x30]
141470342: movaps xmm10, xmmword ptr [rsp + 0x20]
141470348: add rsp, 0x78
14147034c: pop rbx
14147034d: pop rdi
14147034e: pop rsi
14147034f: pop r12
141470351: pop r13
141470353: pop r14
141470355: pop r15
141470357: pop rbp
141470358: ret 
141470359: nop dword ptr [rax]
141470360: mov qword ptr [rsp + 0x10], rdx
141470365: push rbp
141470366: push r15
141470368: push r14
14147036a: push r13
14147036c: push r12
14147036e: push rsi
14147036f: push rdi
141470370: push rbx
141470371: sub rsp, 0x78
141470375: lea rbp, [rdx + 0x80]
14147037c: movaps xmmword ptr [rsp + 0x20], xmm10
141470382: movaps xmmword ptr [rsp + 0x30], xmm9
141470388: movaps xmmword ptr [rsp + 0x40], xmm8
14147038e: movaps xmmword ptr [rsp + 0x50], xmm7
141470393: movaps xmmword ptr [rsp + 0x60], xmm6
141470398: movaps xmm0, xmmword ptr [rbp + 0x5b0]
14147039f: movaps xmm1, xmmword ptr [rbp + 0x5c0]
1414703a6: movaps xmm2, xmmword ptr [rbp + 0x5d0]
1414703ad: mov rcx, qword ptr [rbp + 0x918]
1414703b4: movups xmmword ptr [rcx + 0x20], xmm2
1414703b8: movups xmmword ptr [rcx + 0x10], xmm1
1414703bc: movups xmmword ptr [rcx], xmm0
1414703bf: movabs rax, 0x8000000000000000
1414703c9: mov qword ptr [rcx + 0x30], rax
1414703cd: mov rax, qword ptr [rbp + 0x8b0]
1414703d4: mov qword ptr [rcx + 0x58], rax
1414703d8: mov rax, qword ptr [rbp + 0x8b8]
1414703df: mov qword ptr [rcx + 0x60], rax
1414703e3: mov eax, dword ptr [rbp + 0x938]
1414703e9: mov dword ptr [rcx + 0x68], eax
1414703ec: mov eax, dword ptr [rbp + 0x958]
1414703f2: mov dword ptr [rcx + 0x6c], eax
1414703f5: mov eax, dword ptr [rbp + 0x90c]
1414703fb: mov dword ptr [rcx + 0x70], eax
1414703fe: mov rax, qword ptr [rbp + 0x950]
141470405: mov qword ptr [rbp + 0x940], rax
14147040c: mov rax, qword ptr [rbp + 0x960]
141470413: mov qword ptr [rbp + 0x920], rax
14147041a: mov byte ptr [rbp + 0x977], 0
141470421: movaps xmm6, xmmword ptr [rsp + 0x60]
141470426: movaps xmm7, xmmword ptr [rsp + 0x50]
14147042b: movaps xmm8, xmmword ptr [rsp + 0x40]
141470431: movaps xmm9, xmmword ptr [rsp + 0x30]
141470437: movaps xmm10, xmmword ptr [rsp + 0x20]
14147043d: add rsp, 0x78
141470441: pop rbx
141470442: pop rdi
141470443: pop rsi
141470444: pop r12
141470446: pop r13
141470448: pop r14
14147044a: pop r15
14147044c: pop rbp
14147044d: ret 
14147044e: nop 
141470450: mov qword ptr [rsp + 0x10], rdx
141470455: push rbp
141470456: push r15
141470458: push r14
14147045a: push r13
14147045c: push r12
14147045e: push rsi
14147045f: push rdi
141470460: push rbx
141470461: sub rsp, 0x78
141470465: lea rbp, [rdx + 0x80]
14147046c: movaps xmmword ptr [rsp + 0x20], xmm10
141470472: movaps xmmword ptr [rsp + 0x30], xmm9
141470478: movaps xmmword ptr [rsp + 0x40], xmm8
14147047e: movaps xmmword ptr [rsp + 0x50], xmm7
141470483: movaps xmmword ptr [rsp + 0x60], xmm6
141470488: mov rdi, qword ptr [rbp + 0x938]
14147048f: mov rbx, qword ptr [rbp + 0x958]
141470496: cmp rbx, rdi
141470499: mov rsi, qword ptr [rbp + 0x928]
1414704a0: jne 0x1414704de
1414704a2: mov byte ptr [rbp + 0x973], 0
1414704a9: movaps xmm6, xmmword ptr [rsp + 0x60]
1414704ae: movaps xmm7, xmmword ptr [rsp + 0x50]
1414704b3: movaps xmm8, xmmword ptr [rsp + 0x40]
1414704b9: movaps xmm9, xmmword ptr [rsp + 0x30]
1414704bf: movaps xmm10, xmmword ptr [rsp + 0x20]
1414704c5: add rsp, 0x78
1414704c9: pop rbx
1414704ca: pop rdi
1414704cb: pop rsi
1414704cc: pop r12
1414704ce: pop r13
1414704d0: pop r14
1414704d2: pop r15
1414704d4: pop rbp
1414704d5: ret 
1414704d6: inc rbx
1414704d9: cmp rbx, rdi
1414704dc: je 0x1414704a2
1414704de: mov rcx, rbx
1414704e1: shl rcx, 4
1414704e5: mov rax, qword ptr [rsi + rcx]
1414704e9: lock dec qword ptr [rax]
1414704ed: jne 0x1414704d6
1414704ef: add rcx, rsi
1414704f2: mov byte ptr [rbp + 0x973], 0
1414704f9: call 0x1402e5d80 ; alloc::sync::Arc::drop_slow<core::sync::atomic::AtomicBool,alloc::alloc::Global>
1414704fe: jmp 0x1414704d6
141470500: mov qword ptr [rsp + 0x10], rdx
141470505: push rbp
141470506: push r15
141470508: push r14
14147050a: push r13
14147050c: push r12
14147050e: push rsi
14147050f: push rdi
141470510: push rbx
141470511: sub rsp, 0x78
141470515: lea rbp, [rdx + 0x80]
14147051c: movaps xmmword ptr [rsp + 0x20], xmm10
141470522: movaps xmmword ptr [rsp + 0x30], xmm9
141470528: movaps xmmword ptr [rsp + 0x40], xmm8
14147052e: movaps xmmword ptr [rsp + 0x50], xmm7
141470533: movaps xmmword ptr [rsp + 0x60], xmm6
141470538: lea rcx, [rbp - 0x50]
14147053c: call 0x140894d60 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::history::MultiEdit,alloc::alloc::Global> >
141470541: mov rax, qword ptr [rbp + 0x950]
141470548: mov qword ptr [rbp + 0x940], rax
14147054f: mov rax, qword ptr [rbp + 0x960]
141470556: mov qword ptr [rbp + 0x920], rax
14147055d: mov byte ptr [rbp + 0x977], 0
141470564: movaps xmm6, xmmword ptr [rsp + 0x60]
141470569: movaps xmm7, xmmword ptr [rsp + 0x50]
14147056e: movaps xmm8, xmmword ptr [rsp + 0x40]
141470574: movaps xmm9, xmmword ptr [rsp + 0x30]
14147057a: movaps xmm10, xmmword ptr [rsp + 0x20]
141470580: add rsp, 0x78
141470584: pop rbx
141470585: pop rdi
141470586: pop rsi
141470587: pop r12
141470589: pop r13
14147058b: pop r14
14147058d: pop r15
14147058f: pop rbp
141470590: ret 
141470591: nop word ptr cs:[rax + rax]
1414705a0: mov qword ptr [rsp + 0x10], rdx
1414705a5: push rbp
1414705a6: push r15
1414705a8: push r14
1414705aa: push r13
1414705ac: push r12
1414705ae: push rsi
1414705af: push rdi
1414705b0: push rbx
1414705b1: sub rsp, 0x78
1414705b5: lea rbp, [rdx + 0x80]
1414705bc: movaps xmmword ptr [rsp + 0x20], xmm10
1414705c2: movaps xmmword ptr [rsp + 0x30], xmm9
1414705c8: movaps xmmword ptr [rsp + 0x40], xmm8
1414705ce: movaps xmmword ptr [rsp + 0x50], xmm7
1414705d3: movaps xmmword ptr [rsp + 0x60], xmm6
1414705d8: mov rdx, qword ptr [rbp + 0x1b0]
1414705df: test rdx, rdx
1414705e2: je 0x1414705fa
1414705e4: mov rcx, qword ptr [rbp + 0x1b8]
1414705eb: shl rdx, 4
1414705ef: mov r8d, 8
1414705f5: call 0x140613c20 ; 
1414705fa: movaps xmm6, xmmword ptr [rsp + 0x60]
1414705ff: movaps xmm7, xmmword ptr [rsp + 0x50]
141470604: movaps xmm8, xmmword ptr [rsp + 0x40]
14147060a: movaps xmm9, xmmword ptr [rsp + 0x30]
141470610: movaps xmm10, xmmword ptr [rsp + 0x20]
141470616: add rsp, 0x78
14147061a: pop rbx
14147061b: pop rdi
14147061c: pop rsi
14147061d: pop r12
14147061f: pop r13
141470621: pop r14
141470623: pop r15
141470625: pop rbp
141470626: ret 
141470627: nop word ptr [rax + rax]
141470630: mov qword ptr [rsp + 0x10], rdx
141470635: push rbp
141470636: push r15
141470638: push r14
14147063a: push r13
14147063c: push r12
14147063e: push rsi
14147063f: push rdi
141470640: push rbx
141470641: sub rsp, 0x78
141470645: lea rbp, [rdx + 0x80]
14147064c: movaps xmmword ptr [rsp + 0x20], xmm10
141470652: movaps xmmword ptr [rsp + 0x30], xmm9
141470658: movaps xmmword ptr [rsp + 0x40], xmm8
14147065e: movaps xmmword ptr [rsp + 0x50], xmm7
141470663: movaps xmmword ptr [rsp + 0x60], xmm6
141470668: mov rdi, qword ptr [rbp + 0x950]
14147066f: mov qword ptr [rbp + 0x940], rdi
141470676: mov rsi, qword ptr [rbp + 0x960]
14147067d: mov qword ptr [rbp + 0x920], rsi
141470684: mov byte ptr [rbp + 0x977], 0
14147068b: lea rcx, [rbp + 0x5b0]
141470692: call 0x14088eb40 ; core::ptr::drop_in_place<country_core::systems::save::save_game::GameSaveData>
141470697: mov qword ptr [rbp + 0x940], rdi
14147069e: mov qword ptr [rbp + 0x920], rsi
1414706a5: mov byte ptr [rbp + 0x977], 0
1414706ac: movaps xmm6, xmmword ptr [rsp + 0x60]
1414706b1: movaps xmm7, xmmword ptr [rsp + 0x50]
1414706b6: movaps xmm8, xmmword ptr [rsp + 0x40]
1414706bc: movaps xmm9, xmmword ptr [rsp + 0x30]
1414706c2: movaps xmm10, xmmword ptr [rsp + 0x20]
1414706c8: add rsp, 0x78
1414706cc: pop rbx
1414706cd: pop rdi
1414706ce: pop rsi
1414706cf: pop r12
1414706d1: pop r13
1414706d3: pop r14
1414706d5: pop r15
1414706d7: pop rbp
1414706d8: ret 
1414706d9: nop dword ptr [rax]
1414706e0: mov qword ptr [rsp + 0x10], rdx
1414706e5: push rbp
1414706e6: push r15
1414706e8: push r14
1414706ea: push r13
1414706ec: push r12
1414706ee: push rsi
1414706ef: push rdi
1414706f0: push rbx
1414706f1: sub rsp, 0x78
1414706f5: lea rbp, [rdx + 0x80]
1414706fc: movaps xmmword ptr [rsp + 0x20], xmm10
141470702: movaps xmmword ptr [rsp + 0x30], xmm9
141470708: movaps xmmword ptr [rsp + 0x40], xmm8
14147070e: movaps xmmword ptr [rsp + 0x50], xmm7
141470713: movaps xmmword ptr [rsp + 0x60], xmm6
141470718: mov rcx, qword ptr [rbp + 0x958]
14147071f: call 0x14146cb40 ; core::ptr::drop_in_place<alloc::boxed::Box<country_core::systems::save::save_game::GameSaveData,alloc::alloc::Global> >
141470724: mov rax, qword ptr [rbp + 0x950]
14147072b: mov qword ptr [rbp + 0x940], rax
141470732: mov rax, qword ptr [rbp + 0x960]
141470739: mov qword ptr [rbp + 0x920], rax
141470740: mov byte ptr [rbp + 0x977], 0
141470747: movaps xmm6, xmmword ptr [rsp + 0x60]
14147074c: movaps xmm7, xmmword ptr [rsp + 0x50]
141470751: movaps xmm8, xmmword ptr [rsp + 0x40]
141470757: movaps xmm9, xmmword ptr [rsp + 0x30]
14147075d: movaps xmm10, xmmword ptr [rsp + 0x20]
141470763: add rsp, 0x78
141470767: pop rbx
141470768: pop rdi
141470769: pop rsi
14147076a: pop r12
14147076c: pop r13
14147076e: pop r14
141470770: pop r15
141470772: pop rbp
141470773: ret 
141470774: nop word ptr cs:[rax + rax]
141470780: mov qword ptr [rsp + 0x10], rdx
141470785: push rbp
141470786: push r15
141470788: push r14
14147078a: push r13
14147078c: push r12
14147078e: push rsi
14147078f: push rdi
141470790: push rbx
141470791: sub rsp, 0x78
141470795: lea rbp, [rdx + 0x80]
14147079c: movaps xmmword ptr [rsp + 0x20], xmm10
1414707a2: movaps xmmword ptr [rsp + 0x30], xmm9
1414707a8: movaps xmmword ptr [rsp + 0x40], xmm8
1414707ae: movaps xmmword ptr [rsp + 0x50], xmm7
1414707b3: movaps xmmword ptr [rsp + 0x60], xmm6
1414707b8: cmp byte ptr [rbp + 0x973], 0
1414707bf: je 0x14147081d
1414707c1: lea rcx, [rbp + 0x1b0]
1414707c8: call 0x1408913b0 ; core::ptr::drop_in_place<country_core::systems::save::snapshot::LoadedSnapshotData>
1414707cd: mov rax, qword ptr [rbp + 0x950]
1414707d4: mov qword ptr [rbp + 0x940], rax
1414707db: mov rax, qword ptr [rbp + 0x960]
1414707e2: mov qword ptr [rbp + 0x920], rax
1414707e9: mov byte ptr [rbp + 0x977], 0
1414707f0: movaps xmm6, xmmword ptr [rsp + 0x60]
1414707f5: movaps xmm7, xmmword ptr [rsp + 0x50]
1414707fa: movaps xmm8, xmmword ptr [rsp + 0x40]
141470800: movaps xmm9, xmmword ptr [rsp + 0x30]
141470806: movaps xmm10, xmmword ptr [rsp + 0x20]
14147080c: add rsp, 0x78
141470810: pop rbx
141470811: pop rdi
141470812: pop rsi
141470813: pop r12
141470815: pop r13
141470817: pop r14
141470819: pop r15
14147081b: pop rbp
14147081c: ret 
14147081d: mov rax, qword ptr [rbp + 0x950]
141470824: mov qword ptr [rbp + 0x940], rax
14147082b: mov rax, qword ptr [rbp + 0x960]
141470832: mov qword ptr [rbp + 0x920], rax
141470839: mov byte ptr [rbp + 0x977], 0
141470840: movaps xmm6, xmmword ptr [rsp + 0x60]
141470845: movaps xmm7, xmmword ptr [rsp + 0x50]
14147084a: movaps xmm8, xmmword ptr [rsp + 0x40]
141470850: movaps xmm9, xmmword ptr [rsp + 0x30]
141470856: movaps xmm10, xmmword ptr [rsp + 0x20]
14147085c: add rsp, 0x78
141470860: pop rbx
141470861: pop rdi
141470862: pop rsi
141470863: pop r12
141470865: pop r13
141470867: pop r14
141470869: pop r15
14147086b: pop rbp
14147086c: ret 
14147086d: nop dword ptr [rax]
141470870: mov qword ptr [rsp + 0x10], rdx
141470875: push rbp
141470876: push r15
141470878: push r14
14147087a: push r13
14147087c: push r12
14147087e: push rsi
14147087f: push rdi
141470880: push rbx
141470881: sub rsp, 0x78
141470885: lea rbp, [rdx + 0x80]
14147088c: movaps xmmword ptr [rsp + 0x20], xmm10
141470892: movaps xmmword ptr [rsp + 0x30], xmm9
141470898: movaps xmmword ptr [rsp + 0x40], xmm8
14147089e: movaps xmmword ptr [rsp + 0x50], xmm7
1414708a3: movaps xmmword ptr [rsp + 0x60], xmm6
1414708a8: mov rsi, qword ptr [rbp + 0x920]
1414708af: mov rdi, qword ptr [rbp + 0x940]
1414708b6: cmp byte ptr [rbp + 0x977], 0
1414708bd: je 0x141470947
1414708c3: mov qword ptr [rbp + 0x8d0], rdi
1414708ca: mov qword ptr [rbp + 0x8a0], rsi
1414708d1: mov byte ptr [rbp + 0x971], 1
1414708d8: mov byte ptr [rbp + 0x970], 0
1414708df: mov rbx, qword ptr [rbp + 0x948]
1414708e6: mov rcx, rbx
1414708e9: call 0x140894d60 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::history::MultiEdit,alloc::alloc::Global> >
1414708ee: mov rdx, qword ptr [rbx + 0x18]
1414708f2: test rdx, rdx
1414708f5: je 0x141470911
1414708f7: mov rax, qword ptr [rbp + 0x948]
1414708fe: mov rcx, qword ptr [rax + 0x20]
141470902: shl rdx, 4
141470906: mov r8d, 8
14147090c: call 0x140613c20 ; 
141470911: mov rax, qword ptr [rbp + 0x948]
141470918: mov rax, qword ptr [rax + 0x30]
14147091c: lea rcx, [rax*2]
141470924: test rcx, rcx
141470927: je 0x141470947
141470929: mov rcx, qword ptr [rbp + 0x948]
141470930: mov rcx, qword ptr [rcx + 0x38]
141470934: shl rax, 3
141470938: lea rdx, [rax + rax*4]
14147093c: mov r8d, 4
141470942: call 0x140613c20 ; 
141470947: mov edx, 0xf0
14147094c: mov r8d, 8
141470952: mov rcx, qword ptr [rbp + 0x948]
141470959: call 0x140613c20 ; 
14147095e: mov qword ptr [rbp + 0x8d0], rdi
141470965: mov qword ptr [rbp + 0x8a0], rsi
14147096c: mov byte ptr [rbp + 0x971], 1
141470973: mov byte ptr [rbp + 0x970], 0
14147097a: movaps xmm6, xmmword ptr [rsp + 0x60]
14147097f: movaps xmm7, xmmword ptr [rsp + 0x50]
141470984: movaps xmm8, xmmword ptr [rsp + 0x40]
14147098a: movaps xmm9, xmmword ptr [rsp + 0x30]
141470990: movaps xmm10, xmmword ptr [rsp + 0x20]
141470996: add rsp, 0x78
14147099a: pop rbx
14147099b: pop rdi
14147099c: pop rsi
14147099d: pop r12
14147099f: pop r13
1414709a1: pop r14
1414709a3: pop r15
1414709a5: pop rbp
1414709a6: ret 
1414709a7: nop word ptr [rax + rax]
1414709b0: mov qword ptr [rsp + 0x10], rdx
1414709b5: push rbp
1414709b6: push r15
1414709b8: push r14
1414709ba: push r13
1414709bc: push r12
1414709be: push rsi
1414709bf: push rdi
1414709c0: push rbx
1414709c1: sub rsp, 0x78
1414709c5: lea rbp, [rdx + 0x80]
1414709cc: movaps xmmword ptr [rsp + 0x20], xmm10
1414709d2: movaps xmmword ptr [rsp + 0x30], xmm9
1414709d8: movaps xmmword ptr [rsp + 0x40], xmm8
1414709de: movaps xmmword ptr [rsp + 0x50], xmm7
1414709e3: movaps xmmword ptr [rsp + 0x60], xmm6
1414709e8: movzx ebx, byte ptr [rbp + 0x975]
1414709ef: cmp byte ptr [rbp + 0x974], 0
1414709f6: je 0x141470a5d
1414709f8: mov rcx, qword ptr [rbp + 0x958]
1414709ff: call 0x14146cb40 ; core::ptr::drop_in_place<alloc::boxed::Box<country_core::systems::save::save_game::GameSaveData,alloc::alloc::Global> >
141470a04: mov rax, qword ptr [rbp + 0x950]
141470a0b: mov qword ptr [rbp + 0x8d0], rax
141470a12: mov rax, qword ptr [rbp + 0x960]
141470a19: mov qword ptr [rbp + 0x8a0], rax
141470a20: and bl, 1
141470a23: mov byte ptr [rbp + 0x971], bl
141470a29: mov byte ptr [rbp + 0x970], 0
141470a30: movaps xmm6, xmmword ptr [rsp + 0x60]
141470a35: movaps xmm7, xmmword ptr [rsp + 0x50]
141470a3a: movaps xmm8, xmmword ptr [rsp + 0x40]
141470a40: movaps xmm9, xmmword ptr [rsp + 0x30]
141470a46: movaps xmm10, xmmword ptr [rsp + 0x20]
141470a4c: add rsp, 0x78
141470a50: pop rbx
141470a51: pop rdi
141470a52: pop rsi
141470a53: pop r12
141470a55: pop r13
141470a57: pop r14
141470a59: pop r15
141470a5b: pop rbp
141470a5c: ret 
141470a5d: mov rax, qword ptr [rbp + 0x950]
141470a64: mov qword ptr [rbp + 0x8d0], rax
141470a6b: mov rax, qword ptr [rbp + 0x960]
141470a72: mov qword ptr [rbp + 0x8a0], rax
141470a79: and bl, 1
141470a7c: mov byte ptr [rbp + 0x971], bl
141470a82: mov byte ptr [rbp + 0x970], 0
141470a89: movaps xmm6, xmmword ptr [rsp + 0x60]
141470a8e: movaps xmm7, xmmword ptr [rsp + 0x50]
141470a93: movaps xmm8, xmmword ptr [rsp + 0x40]
141470a99: movaps xmm9, xmmword ptr [rsp + 0x30]
141470a9f: movaps xmm10, xmmword ptr [rsp + 0x20]
141470aa5: add rsp, 0x78
141470aa9: pop rbx
141470aaa: pop rdi
141470aab: pop rsi
141470aac: pop r12
141470aae: pop r13
141470ab0: pop r14
141470ab2: pop r15
141470ab4: pop rbp
141470ab5: ret 
141470ab6: nop word ptr cs:[rax + rax]
141470ac0: mov qword ptr [rsp + 0x10], rdx
141470ac5: push rbp
141470ac6: push r15
141470ac8: push r14
141470aca: push r13
141470acc: push r12
141470ace: push rsi
141470acf: push rdi
141470ad0: push rbx
141470ad1: sub rsp, 0x78
141470ad5: lea rbp, [rdx + 0x80]
141470adc: movaps xmmword ptr [rsp + 0x20], xmm10
141470ae2: movaps xmmword ptr [rsp + 0x30], xmm9
141470ae8: movaps xmmword ptr [rsp + 0x40], xmm8
141470aee: movaps xmmword ptr [rsp + 0x50], xmm7
141470af3: movaps xmmword ptr [rsp + 0x60], xmm6
141470af8: mov rax, qword ptr [rbp + 0x930]
141470aff: lea rcx, [rax*2]
141470b07: test rcx, rcx
141470b0a: setne cl
141470b0d: test byte ptr [rbp + 0x976], cl
141470b13: je 0x141470b2f
141470b15: mov rcx, qword ptr [rbp + 0x548]
141470b1c: shl rax, 3
141470b20: lea rdx, [rax + rax*4]
141470b24: mov r8d, 4
141470b2a: call 0x140613c20 ; 
141470b2f: movaps xmm6, xmmword ptr [rsp + 0x60]
141470b34: movaps xmm7, xmmword ptr [rsp + 0x50]
141470b39: movaps xmm8, xmmword ptr [rsp + 0x40]
141470b3f: movaps xmm9, xmmword ptr [rsp + 0x30]
141470b45: movaps xmm10, xmmword ptr [rsp + 0x20]
141470b4b: add rsp, 0x78
141470b4f: pop rbx
141470b50: pop rdi
141470b51: pop rsi
141470b52: pop r12
141470b54: pop r13
141470b56: pop r14
141470b58: pop r15
141470b5a: pop rbp
141470b5b: ret 
141470b5c: nop dword ptr [rax]
141470b60: mov qword ptr [rsp + 0x10], rdx
141470b65: push rbp
141470b66: push r15
141470b68: push r14
141470b6a: push r13
141470b6c: push r12
141470b6e: push rsi
141470b6f: push rdi
141470b70: push rbx
141470b71: sub rsp, 0x78
141470b75: lea rbp, [rdx + 0x80]
141470b7c: movaps xmmword ptr [rsp + 0x20], xmm10
141470b82: movaps xmmword ptr [rsp + 0x30], xmm9
141470b88: movaps xmmword ptr [rsp + 0x40], xmm8
141470b8e: movaps xmmword ptr [rsp + 0x50], xmm7
141470b93: movaps xmmword ptr [rsp + 0x60], xmm6
141470b98: lea rcx, [rbp + 0x1b0]
141470b9f: call 0x14088fa00 ; core::ptr::drop_in_place<country_core::systems::save::load_game::ParsedGameData>
141470ba4: movaps xmm6, xmmword ptr [rsp + 0x60]
141470ba9: movaps xmm7, xmmword ptr [rsp + 0x50]
141470bae: movaps xmm8, xmmword ptr [rsp + 0x40]
141470bb4: movaps xmm9, xmmword ptr [rsp + 0x30]
141470bba: movaps xmm10, xmmword ptr [rsp + 0x20]
141470bc0: add rsp, 0x78
141470bc4: pop rbx
141470bc5: pop rdi
141470bc6: pop rsi
141470bc7: pop r12
141470bc9: pop r13
141470bcb: pop r14
141470bcd: pop r15
141470bcf: pop rbp
141470bd0: ret 
141470bd1: nop word ptr cs:[rax + rax]
141470be0: mov qword ptr [rsp + 0x10], rdx
141470be5: push rbp
141470be6: push r15
141470be8: push r14
141470bea: push r13
141470bec: push r12
141470bee: push rsi
141470bef: push rdi
141470bf0: push rbx
141470bf1: sub rsp, 0x78
141470bf5: lea rbp, [rdx + 0x80]
141470bfc: movaps xmmword ptr [rsp + 0x20], xmm10
141470c02: movaps xmmword ptr [rsp + 0x30], xmm9
141470c08: movaps xmmword ptr [rsp + 0x40], xmm8
141470c0e: movaps xmmword ptr [rsp + 0x50], xmm7
141470c13: movaps xmmword ptr [rsp + 0x60], xmm6
141470c18: mov byte ptr [rbp + 0x975], 0
141470c1f: mov byte ptr [rbp + 0x974], 0
141470c26: lea rcx, [rbp + 0x5b0]
141470c2d: call 0x14088e590 ; core::ptr::drop_in_place<enum2$<country_core::systems::save::save_game::AsyncSaveCmd> >
141470c32: mov byte ptr [rbp + 0x975], 0
141470c39: mov byte ptr [rbp + 0x974], 0
141470c40: movaps xmm6, xmmword ptr [rsp + 0x60]
141470c45: movaps xmm7, xmmword ptr [rsp + 0x50]
141470c4a: movaps xmm8, xmmword ptr [rsp + 0x40]
141470c50: movaps xmm9, xmmword ptr [rsp + 0x30]
141470c56: movaps xmm10, xmmword ptr [rsp + 0x20]
141470c5c: add rsp, 0x78
141470c60: pop rbx
141470c61: pop rdi
141470c62: pop rsi
141470c63: pop r12
141470c65: pop r13
141470c67: pop r14
141470c69: pop r15
141470c6b: pop rbp
141470c6c: ret 
141470c6d: nop dword ptr [rax]
141470c70: mov qword ptr [rsp + 0x10], rdx
141470c75: push rbp
141470c76: push r15
141470c78: push r14
141470c7a: push r13
141470c7c: push r12
141470c7e: push rsi
141470c7f: push rdi
141470c80: push rbx
141470c81: sub rsp, 0x78
141470c85: lea rbp, [rdx + 0x80]
141470c8c: movaps xmmword ptr [rsp + 0x20], xmm10
141470c92: movaps xmmword ptr [rsp + 0x30], xmm9
141470c98: movaps xmmword ptr [rsp + 0x40], xmm8
141470c9e: movaps xmmword ptr [rsp + 0x50], xmm7
141470ca3: movaps xmmword ptr [rsp + 0x60], xmm6
141470ca8: lea rcx, [rbp + 0x360]
141470caf: call 0x14087b2f0 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<country_core::systems::save::load_game::PostGameLoadCommand>,alloc::alloc::Global> >
141470cb4: movabs rax, 0x8000000000000000
141470cbe: mov qword ptr [rbp + 0x930], rax
141470cc5: mov byte ptr [rbp + 0x976], 1
141470ccc: movaps xmm6, xmmword ptr [rsp + 0x60]
141470cd1: movaps xmm7, xmmword ptr [rsp + 0x50]
141470cd6: movaps xmm8, xmmword ptr [rsp + 0x40]
141470cdc: movaps xmm9, xmmword ptr [rsp + 0x30]
141470ce2: movaps xmm10, xmmword ptr [rsp + 0x20]
141470ce8: add rsp, 0x78
141470cec: pop rbx
141470ced: pop rdi
141470cee: pop rsi
141470cef: pop r12
141470cf1: pop r13
141470cf3: pop r14
141470cf5: pop r15
141470cf7: pop rbp
141470cf8: ret 
