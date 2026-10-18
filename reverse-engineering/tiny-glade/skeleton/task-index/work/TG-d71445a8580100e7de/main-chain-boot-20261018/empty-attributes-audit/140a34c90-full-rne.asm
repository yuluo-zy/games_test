140a34c90: push rbp
140a34c91: push r15
140a34c93: push r14
140a34c95: push r13
140a34c97: push r12
140a34c99: push rsi
140a34c9a: push rdi
140a34c9b: push rbx
140a34c9c: sub rsp, 0xe8
140a34ca3: lea rbp, [rsp + 0x80]
140a34cab: movdqa xmmword ptr [rbp + 0x50], xmm8
140a34cb1: movdqa xmmword ptr [rbp + 0x40], xmm7
140a34cb6: movdqa xmmword ptr [rbp + 0x30], xmm6
140a34cbb: mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
140a34cc3: mov r13, r9
140a34cc6: mov rdi, r8
140a34cc9: mov rbx, rdx
140a34ccc: mov rsi, rcx
140a34ccf: cmp qword ptr [rcx + 0x50], 0
140a34cd4: mov qword ptr [rbp], r9
140a34cd8: je 0x140a34dc5
140a34cde: lea rcx, [rsi + 0x58]
140a34ce2: mov rdx, rbx
140a34ce5: mov r8, rdi
140a34ce8: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
140a34ced: mov r14, qword ptr [rsi + 0x38]
140a34cf1: mov rcx, qword ptr [rsi + 0x40]
140a34cf5: mov r12, rcx
140a34cf8: and r12, rax
140a34cfb: shr rax, 0x39
140a34cff: movd xmm0, eax
140a34d03: punpcklbw xmm0, xmm0
140a34d07: pshuflw xmm0, xmm0, 0
140a34d0c: pshufd xmm6, xmm0, 0
140a34d11: lea rdx, [r14 - 0x38]
140a34d15: xor r8d, r8d
140a34d18: pcmpeqd xmm7, xmm7
140a34d1c: movdqu xmm8, xmmword ptr [r14 + r12]
140a34d22: movdqa xmm0, xmm8
140a34d27: pcmpeqb xmm0, xmm6
140a34d2b: pmovmskb r13d, xmm0
140a34d30: test r13d, r13d
140a34d33: je 0x140a34da0
140a34d35: mov qword ptr [rbp + 0x10], rcx
140a34d39: mov qword ptr [rbp + 0x18], rbx
140a34d3d: mov qword ptr [rbp + 8], r8
140a34d41: tzcnt eax, r13d
140a34d46: add rax, r12
140a34d49: and rax, rcx
140a34d4c: neg rax
140a34d4f: imul r15, rax, 0x38
140a34d53: cmp qword ptr [rdx + r15 + 0x10], rdi
140a34d58: jne 0x140a34d7c
140a34d5a: lea rax, [rdx + r15]
140a34d5e: mov rbx, rdx
140a34d61: mov rdx, qword ptr [rax + 8]
140a34d65: mov rcx, qword ptr [rbp + 0x18]
140a34d69: mov r8, rdi
140a34d6c: call 0x142923bf0
140a34d71: mov rdx, rbx
140a34d74: test eax, eax
140a34d76: je 0x140a34dfc
140a34d7c: lea eax, [r13 - 1]
140a34d80: and ax, r13w
140a34d84: mov r13d, eax
140a34d87: mov rcx, qword ptr [rbp + 0x10]
140a34d8b: mov rbx, qword ptr [rbp + 0x18]
140a34d8f: mov r8, qword ptr [rbp + 8]
140a34d93: jne 0x140a34d41
140a34d95: nop word ptr cs:[rax + rax]
140a34da0: pcmpeqb xmm8, xmm7
140a34da5: pmovmskb eax, xmm8
140a34daa: test eax, eax
140a34dac: mov r13, qword ptr [rbp]
140a34db0: jne 0x140a34dc5
140a34db2: add r12, r8
140a34db5: add r12, 0x10
140a34db9: add r8, 0x10
140a34dbd: and r12, rcx
140a34dc0: jmp 0x140a34d1c
140a34dc5: add rsi, 0x38
140a34dc9: test rdi, rdi
140a34dcc: js 0x140a34ff9
140a34dd2: je 0x140a34e4f
140a34dd4: movzx eax, byte ptr [rip + 0x2bffe2e]
140a34ddb: mov r15d, 1
140a34de1: mov edx, 1
140a34de6: mov rcx, rdi
140a34de9: call 0x140613c10
140a34dee: test rax, rax
140a34df1: je 0x140a34ffc
140a34df7: mov r14, rax
140a34dfa: jmp 0x140a34e55
140a34dfc: mov rax, qword ptr [rbp]
140a34e00: mov rbx, qword ptr [rax]
140a34e03: mov rdi, qword ptr [rax + 8]
140a34e07: mov rsi, qword ptr [rax + 0x10]
140a34e0b: test rsi, rsi
140a34e0e: je 0x140a34fd7
140a34e14: add r14, r15
140a34e17: mov rax, qword ptr [r14 - 0x20]
140a34e1b: cmp rax, 3
140a34e1f: ja 0x140a34f1c
140a34e25: lea rcx, [rip + 0x2098ad8]
140a34e2c: movsxd rax, dword ptr [rcx + rax*4]
140a34e30: add rax, rcx
140a34e33: jmp rax
140a34e35: mov rdx, qword ptr [r14 - 0x18]
140a34e39: test rdx, rdx
140a34e3c: je 0x140a34f38
140a34e42: mov rcx, qword ptr [r14 - 0x10]
140a34e46: shl rdx, 2
140a34e4a: jmp 0x140a34f2d
140a34e4f: mov r14d, 1
140a34e55: mov rcx, r14
140a34e58: mov rdx, rbx
140a34e5b: mov r8, rdi
140a34e5e: call 0x142923c00
140a34e63: mov qword ptr [rbp - 0x38], rdi
140a34e67: mov qword ptr [rbp - 0x30], r14
140a34e6b: mov qword ptr [rbp - 0x28], rdi
140a34e6f: mov rdx, qword ptr [r13]
140a34e73: mov rcx, qword ptr [r13 + 8]
140a34e77: mov rax, qword ptr [r13 + 0x10]
140a34e7b: test rax, rax
140a34e7e: mov qword ptr [rbp + 0x18], rax
140a34e82: je 0x140a34fb5
140a34e88: mov qword ptr [rbp - 0x50], rdx
140a34e8c: mov qword ptr [rbp - 0x48], rcx
140a34e90: mov qword ptr [rbp - 0x40], rax
140a34e94: mov qword ptr [rbp - 0x58], 3
140a34e9c: lea rcx, [rbp - 0x20]
140a34ea0: lea r8, [rbp - 0x38]
140a34ea4: lea r9, [rbp - 0x58]
140a34ea8: mov rdx, rsi
140a34eab: call 0x14152a030 ; hashbrown::map::HashMap::insert<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues>,std::hash::random::RandomState,alloc::alloc::Global>
140a34eb0: mov rax, qword ptr [rbp - 0x20]
140a34eb4: cmp rax, 5
140a34eb8: je 0x140a34f94
140a34ebe: cmp rax, 3
140a34ec2: ja 0x140a34f78
140a34ec8: lea rcx, [rip + 0x2098a45]
140a34ecf: movsxd rax, dword ptr [rcx + rax*4]
140a34ed3: add rax, rcx
140a34ed6: jmp rax
140a34ed8: mov rdx, qword ptr [rbp - 0x18]
140a34edc: test rdx, rdx
140a34edf: je 0x140a34f94
140a34ee5: mov rcx, qword ptr [rbp - 0x10]
140a34ee9: shl rdx, 2
140a34eed: jmp 0x140a34f89
140a34ef2: mov rdx, qword ptr [r14 - 0x18]
140a34ef6: test rdx, rdx
140a34ef9: je 0x140a34f38
140a34efb: mov rcx, qword ptr [r14 - 0x10]
140a34eff: shl rdx, 3
140a34f03: jmp 0x140a34f2d
140a34f05: mov rax, qword ptr [r14 - 0x18]
140a34f09: test rax, rax
140a34f0c: je 0x140a34f38
140a34f0e: mov rcx, qword ptr [r14 - 0x10]
140a34f12: shl rax, 2
140a34f16: lea rdx, [rax + rax*2]
140a34f1a: jmp 0x140a34f2d
140a34f1c: mov rdx, qword ptr [r14 - 0x18]
140a34f20: test rdx, rdx
140a34f23: je 0x140a34f38
140a34f25: mov rcx, qword ptr [r14 - 0x10]
140a34f29: shl rdx, 4
140a34f2d: mov r8d, 4
140a34f33: call 0x140613c20
140a34f38: mov qword ptr [r14 - 0x20], 3
140a34f40: mov qword ptr [r14 - 0x18], rbx
140a34f44: mov qword ptr [r14 - 0x10], rdi
140a34f48: mov qword ptr [r14 - 8], rsi
140a34f4c: jmp 0x140a34f94
140a34f4e: mov rdx, qword ptr [rbp - 0x18]
140a34f52: test rdx, rdx
140a34f55: je 0x140a34f94
140a34f57: mov rcx, qword ptr [rbp - 0x10]
140a34f5b: shl rdx, 3
140a34f5f: jmp 0x140a34f89
140a34f61: mov rax, qword ptr [rbp - 0x18]
140a34f65: test rax, rax
140a34f68: je 0x140a34f94
140a34f6a: mov rcx, qword ptr [rbp - 0x10]
140a34f6e: shl rax, 2
140a34f72: lea rdx, [rax + rax*2]
140a34f76: jmp 0x140a34f89
140a34f78: mov rdx, qword ptr [rbp - 0x18]
140a34f7c: test rdx, rdx
140a34f7f: je 0x140a34f94
140a34f81: mov rcx, qword ptr [rbp - 0x10]
140a34f85: shl rdx, 4
140a34f89: mov r8d, 4
140a34f8f: call 0x140613c20
140a34f94: movaps xmm6, xmmword ptr [rbp + 0x30]
140a34f98: movaps xmm7, xmmword ptr [rbp + 0x40]
140a34f9c: movaps xmm8, xmmword ptr [rbp + 0x50]
140a34fa1: add rsp, 0xe8
140a34fa8: pop rbx
140a34fa9: pop rdi
140a34faa: pop rsi
140a34fab: pop r12
140a34fad: pop r13
140a34faf: pop r14
140a34fb1: pop r15
140a34fb3: pop rbp
140a34fb4: ret 
140a34fb5: mov qword ptr [rbp + 0x10], rdx
140a34fb9: mov qword ptr [rbp + 8], rcx
140a34fbd: lea rcx, [rip + 0x2092019]
140a34fc4: lea r8, [rip + 0x2097935]
140a34fcb: mov edx, 0x1f
140a34fd0: call 0x1428d9430 ; core::panicking::panic
140a34fd5: jmp 0x140a35012
140a34fd7: mov qword ptr [rbp + 0x18], rbx
140a34fdb: mov qword ptr [rbp + 0x10], rdi
140a34fdf: lea rcx, [rip + 0x2091ff7]
140a34fe6: lea r8, [rip + 0x20978fb]
140a34fed: mov edx, 0x1f
140a34ff2: call 0x1428d9430 ; core::panicking::panic
140a34ff7: jmp 0x140a35012
140a34ff9: xor r15d, r15d
140a34ffc: mov byte ptr [rbp + 0x27], 1
140a35000: lea r8, [rip + 0x2097779]
140a35007: mov rcx, r15
140a3500a: mov rdx, rdi
140a3500d: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a35012: ud2 
140a35014: nop word ptr cs:[rax + rax]
140a35020: mov qword ptr [rsp + 0x10], rdx
140a35025: push rbp
140a35026: push r15
140a35028: push r14
140a3502a: push r13
140a3502c: push r12
140a3502e: push rsi
140a3502f: push rdi
140a35030: push rbx
140a35031: sub rsp, 0x58
140a35035: lea rbp, [rdx + 0x80]
140a3503c: movdqa xmmword ptr [rsp + 0x20], xmm8
140a35043: movdqa xmmword ptr [rsp + 0x30], xmm7
140a35049: movdqa xmmword ptr [rsp + 0x40], xmm6
140a3504f: mov rax, qword ptr [rbp + 0x18]
140a35053: test rax, rax
140a35056: je 0x140a3506f
140a35058: shl rax, 2
140a3505c: lea rdx, [rax + rax*2]
140a35060: mov r8d, 4
140a35066: mov rcx, qword ptr [rbp + 0x10]
140a3506a: call 0x140613c20
140a3506f: mov byte ptr [rbp + 0x27], 0
140a35073: movaps xmm6, xmmword ptr [rsp + 0x40]
140a35078: movaps xmm7, xmmword ptr [rsp + 0x30]
140a3507d: movaps xmm8, xmmword ptr [rsp + 0x20]
140a35083: add rsp, 0x58
140a35087: pop rbx
140a35088: pop rdi
140a35089: pop rsi
140a3508a: pop r12
140a3508c: pop r13
140a3508e: pop r14
140a35090: pop r15
140a35092: pop rbp
140a35093: ret 
140a35094: nop word ptr cs:[rax + rax]
140a350a0: mov qword ptr [rsp + 0x10], rdx
140a350a5: push rbp
140a350a6: push r15
140a350a8: push r14
140a350aa: push r13
140a350ac: push r12
140a350ae: push rsi
140a350af: push rdi
140a350b0: push rbx
140a350b1: sub rsp, 0x58
140a350b5: lea rbp, [rdx + 0x80]
140a350bc: movdqa xmmword ptr [rsp + 0x20], xmm8
140a350c3: movdqa xmmword ptr [rsp + 0x30], xmm7
140a350c9: movdqa xmmword ptr [rsp + 0x40], xmm6
140a350cf: mov rax, qword ptr [rbp + 0x10]
140a350d3: test rax, rax
140a350d6: je 0x140a350ef
140a350d8: shl rax, 2
140a350dc: lea rdx, [rax + rax*2]
140a350e0: mov r8d, 4
140a350e6: mov rcx, qword ptr [rbp + 8]
140a350ea: call 0x140613c20
140a350ef: movdqa xmm6, xmmword ptr [rsp + 0x40]
140a350f5: movdqa xmm7, xmmword ptr [rsp + 0x30]
140a350fb: movdqa xmm8, xmmword ptr [rsp + 0x20]
140a35102: add rsp, 0x58
140a35106: pop rbx
140a35107: pop rdi
140a35108: pop rsi
140a35109: pop r12
140a3510b: pop r13
140a3510d: pop r14
140a3510f: pop r15
140a35111: pop rbp
140a35112: ret 
140a35113: nop word ptr cs:[rax + rax]
140a35120: mov qword ptr [rsp + 0x10], rdx
140a35125: push rbp
140a35126: push r15
140a35128: push r14
140a3512a: push r13
140a3512c: push r12
140a3512e: push rsi
140a3512f: push rdi
140a35130: push rbx
140a35131: sub rsp, 0x58
140a35135: lea rbp, [rdx + 0x80]
140a3513c: movdqa xmmword ptr [rsp + 0x20], xmm8
140a35143: movdqa xmmword ptr [rsp + 0x30], xmm7
140a35149: movdqa xmmword ptr [rsp + 0x40], xmm6
140a3514f: cmp qword ptr [rbp + 0x18], 0
140a35154: jne 0x140a35196
140a35156: mov rdx, qword ptr [rbp - 0x38]
140a3515a: test rdx, rdx
140a3515d: je 0x140a3516e
140a3515f: mov rcx, qword ptr [rbp - 0x30]
140a35163: mov r8d, 1
140a35169: call 0x140613c20
140a3516e: mov byte ptr [rbp + 0x27], 0
140a35172: movdqa xmm6, xmmword ptr [rsp + 0x40]
140a35178: movdqa xmm7, xmmword ptr [rsp + 0x30]
140a3517e: movdqa xmm8, xmmword ptr [rsp + 0x20]
140a35185: add rsp, 0x58
140a35189: pop rbx
140a3518a: pop rdi
140a3518b: pop rsi
140a3518c: pop r12
140a3518e: pop r13
140a35190: pop r14
140a35192: pop r15
140a35194: pop rbp
140a35195: ret 
140a35196: mov byte ptr [rbp + 0x27], 0
140a3519a: movdqa xmm6, xmmword ptr [rsp + 0x40]
140a351a0: movdqa xmm7, xmmword ptr [rsp + 0x30]
140a351a6: movdqa xmm8, xmmword ptr [rsp + 0x20]
140a351ad: add rsp, 0x58
140a351b1: pop rbx
140a351b2: pop rdi
140a351b3: pop rsi
140a351b4: pop r12
140a351b6: pop r13
140a351b8: pop r14
140a351ba: pop r15
140a351bc: pop rbp
140a351bd: ret 
140a351be: nop 
140a351c0: mov qword ptr [rsp + 0x10], rdx
140a351c5: push rbp
140a351c6: push r15
140a351c8: push r14
140a351ca: push r13
140a351cc: push r12
140a351ce: push rsi
140a351cf: push rdi
140a351d0: push rbx
140a351d1: sub rsp, 0x58
140a351d5: lea rbp, [rdx + 0x80]
140a351dc: movdqa xmmword ptr [rsp + 0x20], xmm8
140a351e3: movdqa xmmword ptr [rsp + 0x30], xmm7
140a351e9: movdqa xmmword ptr [rsp + 0x40], xmm6
140a351ef: cmp byte ptr [rbp + 0x27], 0
140a351f3: mov rcx, qword ptr [rbp]
140a351f7: je 0x140a35218
140a351f9: mov rax, qword ptr [rcx]
140a351fc: test rax, rax
140a351ff: je 0x140a35218
140a35201: mov rcx, qword ptr [rcx + 8]
140a35205: shl rax, 2
140a35209: lea rdx, [rax + rax*2]
140a3520d: mov r8d, 4
140a35213: call 0x140613c20
140a35218: movaps xmm6, xmmword ptr [rsp + 0x40]
140a3521d: movaps xmm7, xmmword ptr [rsp + 0x30]
140a35222: movaps xmm8, xmmword ptr [rsp + 0x20]
140a35228: add rsp, 0x58
140a3522c: pop rbx
140a3522d: pop rdi
140a3522e: pop rsi
140a3522f: pop r12
140a35231: pop r13
140a35233: pop r14
140a35235: pop r15
140a35237: pop rbp
140a35238: ret 
