142290820: push rbp
142290821: push r15
142290823: push r14
142290825: push r13
142290827: push r12
142290829: push rsi
14229082a: push rdi
14229082b: push rbx
14229082c: sub rsp, 0x1e8
142290833: lea rbp, [rsp + 0x80]
14229083b: movaps xmmword ptr [rbp + 0x150], xmm8
142290843: movaps xmmword ptr [rbp + 0x140], xmm7
14229084a: movaps xmmword ptr [rbp + 0x130], xmm6
142290851: mov qword ptr [rbp + 0x128], 0xfffffffffffffffe
14229085c: mov rsi, r9
14229085f: mov rbx, rcx
142290862: mov r9, qword ptr [r8]
142290865: mov r14, qword ptr [r8 + 8]
142290869: mov rax, qword ptr [r14 + 0x30]
14229086d: mov rcx, qword ptr [r14 + 0x40]
142290871: xor r8d, r8d
142290874: sub rcx, qword ptr [r9]
142290877: cmovae r8, rcx
14229087b: add rax, qword ptr [r14 + 0x10]
14229087f: cmp rax, r8
142290882: cmovae rax, r8
142290886: test rax, rax
142290889: je 0x142290e05
14229088f: mov qword ptr [rbp + 0xf8], r9
142290896: mov rcx, qword ptr [rdx]
142290899: lea rdx, [rip + 0xea5e90]
1422908a0: call 0x1425f18b0
1422908a5: test rax, rax
1422908a8: je 0x142290e95
1422908ae: mov rdi, rax
1422908b1: lea rax, [rip + 0xea5f78]
1422908b8: mov qword ptr [rbp - 0x40], rax
1422908bc: mov qword ptr [rbp - 0x38], 0x32
1422908c4: cmp qword ptr [rdi + 0x18], 0x32
1422908c9: jne 0x142290e47
1422908cf: mov rdx, qword ptr [rdi + 0x10]
1422908d3: lea rcx, [rip + 0xea5f56]
1422908da: mov r8d, 0x32
1422908e0: call 0x142923bf0
1422908e5: test eax, eax
1422908e7: jne 0x142290e47
1422908ed: mov rcx, qword ptr [rdi]
1422908f0: mov rax, qword ptr [rdi + 8]
1422908f4: call qword ptr [rax + 0x18]
1422908f7: mov qword ptr [rbp + 0x78], rax
1422908fb: test rax, rax
1422908fe: je 0x142290e95
142290904: mov qword ptr [rbp + 0x70], rbx
142290908: mov rax, qword ptr [rbp + 0x1e0]
14229090f: mov rdi, qword ptr [rbp + 0xf8]
142290916: mov r8, qword ptr [rdi]
142290919: xor edx, edx
14229091b: mov r9, r8
14229091e: sub r9, qword ptr [r14 + 0x18]
142290922: cmovb r9, rdx
142290926: sub r8, qword ptr [r14 + 0x38]
14229092a: mov rcx, qword ptr [rbp + 0x1d8]
142290931: cmovb r8, rdx
142290935: lea rbx, [r9 + r9*2]
142290939: shl rbx, 3
14229093d: add rbx, qword ptr [r14 + 8]
142290941: mov r10, qword ptr [r14 + 0x10]
142290945: mov r11, qword ptr [r14 + 0x30]
142290949: sub r10, r9
14229094c: cmovb r10, rdx
142290950: mov r9d, 8
142290956: cmovb rbx, r9
14229095a: lea r15, [r8 + r8*2]
14229095e: shl r15, 3
142290962: add r15, qword ptr [r14 + 0x28]
142290966: sub r11, r8
142290969: cmovb r11, rdx
14229096d: cmovb r15, r9
142290971: mov rdx, qword ptr [rbp + 0x1d0]
142290978: mov r8, qword ptr [r14 + 0x40]
14229097c: sub r8, r11
14229097f: sub r8, r10
142290982: mov qword ptr [rdi], r8
142290985: lea r8, [r10 + r10*2]
142290989: lea r8, [rbx + r8*8]
14229098d: mov qword ptr [rbp + 0x20], r8
142290991: lea r8, [r11 + r11*2]
142290995: mov qword ptr [rbp + 0xe0], r15
14229099c: lea r8, [r15 + r8*8]
1422909a0: mov qword ptr [rbp + 0x40], r8
1422909a4: mov r14, qword ptr [rdx]
1422909a7: mov r8, qword ptr [rdx + 0x10]
1422909ab: mov qword ptr [rbp + 0x10], r8
1422909af: mov edx, dword ptr [rdx + 0x1c]
1422909b2: mov dword ptr [rbp + 0x110], edx
1422909b8: mov rcx, qword ptr [rcx]
1422909bb: mov qword ptr [rbp + 0x18], rcx
1422909bf: mov ecx, dword ptr [rsi + 0x1c]
1422909c2: mov dword ptr [rbp + 0x118], ecx
1422909c8: mov rcx, qword ptr [rsi]
1422909cb: mov rdx, qword ptr [rsi + 0x10]
1422909cf: mov qword ptr [rbp + 0x48], rdx
1422909d3: mov edx, dword ptr [rax + 0x1c]
1422909d6: mov dword ptr [rbp + 0x114], edx
1422909dc: mov rdx, qword ptr [rax]
1422909df: mov rax, qword ptr [rax + 0x10]
1422909e3: mov qword ptr [rbp + 0x28], rax
1422909e7: lea rax, [rdx + 0x40]
1422909eb: mov qword ptr [rbp + 0x30], rax
1422909ef: lea rax, [rdx + 0x20]
1422909f3: mov qword ptr [rbp + 0x38], rax
1422909f7: lea rax, [rdx + 0x30]
1422909fb: mov qword ptr [rbp + 0x120], rax
142290a02: add rdx, 0x28
142290a06: mov qword ptr [rbp + 0xf0], rdx
142290a0d: mov rax, qword ptr [rbp + 0x1e8]
142290a14: mov edx, dword ptr [rax + 0x1c]
142290a17: mov dword ptr [rbp + 0x11c], edx
142290a1d: mov rdx, qword ptr [rax]
142290a20: mov rax, qword ptr [rax + 0x10]
142290a24: mov qword ptr [rbp + 0x58], rax
142290a28: lea rax, [rdx + 0x40]
142290a2c: mov qword ptr [rbp + 0x60], rax
142290a30: lea rax, [rdx + 0x20]
142290a34: mov qword ptr [rbp + 0x68], rax
142290a38: lea rax, [rdx + 0x30]
142290a3c: mov qword ptr [rbp + 0x100], rax
142290a43: add rdx, 0x28
142290a47: mov qword ptr [rbp + 0x108], rdx
142290a4e: movaps xmm6, xmmword ptr [rip + 0x8323cb]
142290a55: movaps xmm7, xmmword ptr [rip + 0x8323d4]
142290a5c: movss xmm8, dword ptr [rip + 0x694e0b]
142290a65: mov qword ptr [rbp + 0xe8], rcx
142290a6c: lea rax, [rcx + 0x20]
142290a70: mov qword ptr [rbp + 0x50], rax
142290a74: jmp 0x142290ab7
142290a76: nop word ptr cs:[rax + rax]
142290a80: lea rdx, [rip + 0xdaa841]
142290a87: call 0x140562630
142290a8c: mov rax, qword ptr [rbp + 0xf0]
142290a93: mov rcx, qword ptr [rbp + 0x120]
142290a9a: mov rax, qword ptr [rax]
142290a9d: mov r8, r15
142290aa0: shl r8, 4
142290aa4: mov qword ptr [rax + r8], r12
142290aa8: mov qword ptr [rax + r8 + 8], rsi
142290aad: inc r15
142290ab0: mov qword ptr [rcx], r15
142290ab3: inc qword ptr [r13]
142290ab7: test rbx, rbx
142290aba: je 0x142290ad0
142290abc: cmp rbx, qword ptr [rbp + 0x20]
142290ac0: je 0x142290ad0
142290ac2: lea rax, [rbx + 0x18]
142290ac6: mov r15, rbx
142290ac9: jmp 0x142290afa
142290acb: nop dword ptr [rax + rax]
142290ad0: mov rax, qword ptr [rbp + 0xe0]
142290ad7: test rax, rax
142290ada: je 0x142290d85
142290ae0: cmp rax, qword ptr [rbp + 0x40]
142290ae4: je 0x142290d85
142290aea: mov r15, rax
142290aed: lea rcx, [rax + 0x18]
142290af1: xor eax, eax
142290af3: mov qword ptr [rbp + 0xe0], rcx
142290afa: mov rbx, rax
142290afd: lea r13, [r15 + 8]
142290b01: inc qword ptr [rdi]
142290b04: mov rcx, r13
142290b07: call 0x1400885d0
142290b0c: cmp rax, 0x5000
142290b12: jae 0x142290e2f
142290b18: mov rdx, qword ptr [r14 + 0x10]
142290b1c: cmp rax, rdx
142290b1f: jae 0x142290e77
142290b25: mov rcx, qword ptr [r14 + 8]
142290b29: mov esi, dword ptr [rcx + rax*4]
142290b2c: mov ecx, dword ptr [r15 + 0x10]
142290b30: mov rdx, qword ptr [rbp + 0x10]
142290b34: mov r8d, dword ptr [rbp + 0x110]
142290b3b: mov dword ptr [rdx], r8d
142290b3e: mov rdx, qword ptr [r14 + 0x10]
142290b42: cmp rax, rdx
142290b45: jae 0x142290e86
142290b4b: mov rdx, qword ptr [r14 + 8]
142290b4f: mov dword ptr [rdx + rax*4], ecx
142290b52: mov rdx, qword ptr [r13]
142290b56: mov rcx, qword ptr [rbp + 0x18]
142290b5a: lea r8, [rip + 0xdaa67f]
142290b61: call 0x140a91ef0
142290b66: test rax, rax
142290b69: mov rdx, qword ptr [rbp + 0x120]
142290b70: je 0x142290ab7
142290b76: cmp byte ptr [r15 + 0x14], 0
142290b7b: je 0x142290cc8
142290b81: mov r12, rax
142290b84: mov rcx, rax
142290b87: add rcx, 0x299
142290b8e: call 0x1408c5670
142290b93: mov edi, eax
142290b95: mov rcx, qword ptr [r12 + 0x48]
142290b9a: movaps xmm0, xmm7
142290b9d: movaps xmm1, xmm6
142290ba0: test rcx, rcx
142290ba3: je 0x142290c01
142290ba5: mov rax, qword ptr [r12 + 0x40]
142290baa: lea rcx, [rcx + rcx*2]
142290bae: lea rcx, [rax + rcx*4]
142290bb2: movaps xmm0, xmm7
142290bb5: movaps xmm2, xmm6
142290bb8: movaps xmm1, xmm6
142290bbb: nop dword ptr [rax + rax]
142290bc0: movss xmm3, dword ptr [rax]
142290bc4: movss xmm4, dword ptr [rax + 8]
142290bc9: unpcklps xmm3, xmm4
142290bcc: movaps xmm4, xmm3
142290bcf: minps xmm4, xmm0
142290bd2: cmpunordps xmm0, xmm0
142290bd6: movaps xmm5, xmm3
142290bd9: andps xmm5, xmm0
142290bdc: andnps xmm0, xmm4
142290bdf: orps xmm0, xmm5
142290be2: cmpunordps xmm1, xmm1
142290be6: movaps xmm4, xmm3
142290be9: andps xmm4, xmm1
142290bec: maxps xmm3, xmm2
142290bef: andnps xmm1, xmm3
142290bf2: orps xmm1, xmm4
142290bf5: add rax, 0xc
142290bf9: movaps xmm2, xmm1
142290bfc: cmp rax, rcx
142290bff: jne 0x142290bc0
142290c01: movlhps xmm0, xmm1
142290c04: movups xmmword ptr [rbp - 0x40], xmm0
142290c08: lea rcx, [rbp - 0x40]
142290c0c: call 0x1412400c0
142290c11: movss dword ptr [rbp + 0xd0], xmm0
142290c19: movss dword ptr [rbp + 0xd4], xmm1
142290c21: mov rcx, r12
142290c24: call 0x140b2a620
142290c29: mulss xmm0, xmm8
142290c2e: lea rcx, [rbp + 0x88]
142290c35: lea rdx, [rbp + 0xd0]
142290c3c: movaps xmm2, xmm0
142290c3f: call 0x140c9cde0
142290c44: mov byte ptr [rbp + 0x80], 6
142290c4b: mov byte ptr [rbp + 0x81], dil
142290c52: mov eax, dword ptr [rbp + 0x118]
142290c58: mov rcx, qword ptr [rbp + 0x48]
142290c5c: mov dword ptr [rcx], eax
142290c5e: mov rax, qword ptr [rbp + 0xe8]
142290c65: mov r12, qword ptr [rax + 0x30]
142290c69: mov rdi, qword ptr [rax + 0x40]
142290c6d: cmp r12, qword ptr [rax + 0x20]
142290c71: jne 0x142290c83
142290c73: mov rcx, qword ptr [rbp + 0x50]
142290c77: lea rdx, [rip + 0xdaa64a]
142290c7e: call 0x140562c30
142290c83: mov r8, qword ptr [rbp + 0xe8]
142290c8a: mov rax, qword ptr [r8 + 0x28]
142290c8e: mov rcx, r12
142290c91: shl rcx, 5
142290c95: mov qword ptr [rax + rcx], rdi
142290c99: mov edx, dword ptr [rbp + 0x90]
142290c9f: mov dword ptr [rax + rcx + 0x18], edx
142290ca3: movups xmm0, xmmword ptr [rbp + 0x80]
142290caa: movups xmmword ptr [rax + rcx + 8], xmm0
142290caf: inc r12
142290cb2: mov qword ptr [r8 + 0x30], r12
142290cb6: inc qword ptr [r8 + 0x40]
142290cba: mov rdi, qword ptr [rbp + 0xf8]
142290cc1: mov rdx, qword ptr [rbp + 0x120]
142290cc8: mov eax, dword ptr [r15 + 0x10]
142290ccc: lea ecx, [rax - 4]
142290ccf: cmp ecx, 7
142290cd2: jae 0x142290d0b
142290cd4: mov rsi, qword ptr [r13]
142290cd8: mov eax, dword ptr [rbp + 0x114]
142290cde: mov rcx, qword ptr [rbp + 0x28]
142290ce2: mov dword ptr [rcx], eax
142290ce4: mov r13, qword ptr [rbp + 0x30]
142290ce8: mov r12, qword ptr [r13]
142290cec: mov r15, qword ptr [rdx]
142290cef: mov rcx, qword ptr [rbp + 0x38]
142290cf3: cmp r15, qword ptr [rcx]
142290cf6: je 0x142290a80
142290cfc: mov rax, qword ptr [rbp + 0xf0]
142290d03: mov rcx, rdx
142290d06: jmp 0x142290a9a
142290d0b: cmp eax, 0xc
142290d0e: je 0x142290cd4
142290d10: lea ecx, [rsi - 4]
142290d13: cmp ecx, 7
142290d16: jae 0x142290d6b
142290d18: cmp eax, 0xc
142290d1b: je 0x142290ab7
142290d21: mov rsi, qword ptr [r13]
142290d25: mov eax, dword ptr [rbp + 0x11c]
142290d2b: mov rcx, qword ptr [rbp + 0x58]
142290d2f: mov dword ptr [rcx], eax
142290d31: mov r13, qword ptr [rbp + 0x60]
142290d35: mov r12, qword ptr [r13]
142290d39: mov rdx, qword ptr [rbp + 0x100]
142290d40: mov r15, qword ptr [rdx]
142290d43: mov rcx, qword ptr [rbp + 0x68]
142290d47: cmp r15, qword ptr [rcx]
142290d4a: jne 0x142290d76
142290d4c: lea rdx, [rip + 0xdaa575]
142290d53: call 0x140562630
142290d58: mov rax, qword ptr [rbp + 0x108]
142290d5f: mov rcx, qword ptr [rbp + 0x100]
142290d66: jmp 0x142290a9a
142290d6b: cmp esi, 0xc
142290d6e: jne 0x142290ab7
142290d74: jmp 0x142290d18
142290d76: mov rax, qword ptr [rbp + 0x108]
142290d7d: mov rcx, rdx
142290d80: jmp 0x142290a9a
142290d85: mov rax, qword ptr [rbp + 0x70]
142290d89: mov rcx, qword ptr [rax]
142290d8c: call 0x140987ca0
142290d91: mov rcx, qword ptr [r14 + 0x10]
142290d95: test rcx, rcx
142290d98: je 0x142290e05
142290d9a: shl rcx, 2
142290d9e: mov r8, qword ptr [rbp + 0x78]
142290da2: cmp rcx, qword ptr [r8 + 0x18]
142290da6: ja 0x142290ea1
142290dac: mov r9, qword ptr [r14 + 8]
142290db0: mov qword ptr [rbp + 0xd0], 0
142290dbb: mov qword ptr [rbp + 0xd8], rcx
142290dc2: lea rdx, [rbp + 0xd0]
142290dc9: mov qword ptr [rsp + 0x28], rdx
142290dce: mov qword ptr [rsp + 0x20], rcx
142290dd3: mov qword ptr [rsp + 0x38], 0
142290ddc: mov qword ptr [rsp + 0x30], 1
142290de5: lea rcx, [rbp + 0x80]
142290dec: mov rdx, rax
142290def: call 0x140beefa0
142290df4: mov rax, qword ptr [rbp + 0x80]
142290dfb: cmp rax, 0xb
142290dff: jne 0x142290ea8
142290e05: movaps xmm6, xmmword ptr [rbp + 0x130]
142290e0c: movaps xmm7, xmmword ptr [rbp + 0x140]
142290e13: movaps xmm8, xmmword ptr [rbp + 0x150]
142290e1b: add rsp, 0x1e8
142290e22: pop rbx
142290e23: pop rdi
142290e24: pop rsi
142290e25: pop r12
142290e27: pop r13
142290e29: pop r14
142290e2b: pop r15
142290e2d: pop rbp
142290e2e: ret 
142290e2f: lea rcx, [rip + 0xdaa32a]
142290e36: lea r8, [rip + 0xdaa35b]
142290e3d: mov edx, 0x32
142290e42: call 0x1428d9430
142290e47: mov qword ptr [rbp + 0x80], 0
142290e52: add rdi, 0x10
142290e56: lea rax, [rip + 0xea58bb]
142290e5d: mov qword ptr [rsp + 0x20], rax
142290e62: lea rdx, [rbp - 0x40]
142290e66: lea r9, [rbp + 0x80]
142290e6d: xor ecx, ecx
142290e6f: mov r8, rdi
142290e72: call 0x1428cab41
142290e77: lea r8, [rip + 0xdaa332]
142290e7e: mov rcx, rax
142290e81: call 0x1428d9518
142290e86: lea r8, [rip + 0xdaa33b]
142290e8d: mov rcx, rax
142290e90: call 0x1428d9518
142290e95: lea rcx, [rip + 0xdaa294]
142290e9c: call 0x1428d9310
142290ea1: mov eax, 0xb
142290ea6: jmp 0x142290edf
142290ea8: mov rcx, qword ptr [rbp + 0xc8]
142290eaf: mov qword ptr [rbp], rcx
142290eb3: movups xmm0, xmmword ptr [rbp + 0x88]
142290eba: movups xmm1, xmmword ptr [rbp + 0x98]
142290ec1: movups xmm2, xmmword ptr [rbp + 0xa8]
142290ec8: movups xmm3, xmmword ptr [rbp + 0xb8]
142290ecf: movaps xmmword ptr [rbp - 0x10], xmm3
142290ed3: movaps xmmword ptr [rbp - 0x20], xmm2
142290ed7: movaps xmmword ptr [rbp - 0x30], xmm1
142290edb: movaps xmmword ptr [rbp - 0x40], xmm0
142290edf: mov qword ptr [rbp + 0x80], rax
142290ee6: movaps xmm0, xmmword ptr [rbp - 0x40]
142290eea: movaps xmm1, xmmword ptr [rbp - 0x30]
142290eee: movaps xmm2, xmmword ptr [rbp - 0x20]
142290ef2: movaps xmm3, xmmword ptr [rbp - 0x10]
142290ef6: movups xmmword ptr [rbp + 0x88], xmm0
142290efd: movups xmmword ptr [rbp + 0x98], xmm1
142290f04: movups xmmword ptr [rbp + 0xa8], xmm2
142290f0b: movups xmmword ptr [rbp + 0xb8], xmm3
142290f12: mov rax, qword ptr [rbp]
142290f16: mov qword ptr [rbp + 0xc8], rax
142290f1d: lea rax, [rip + 0xdaa224]
142290f24: mov qword ptr [rsp + 0x20], rax
142290f29: lea rcx, [rip + 0xdaa120]
142290f30: lea r9, [rip + 0xdaa0f9]
