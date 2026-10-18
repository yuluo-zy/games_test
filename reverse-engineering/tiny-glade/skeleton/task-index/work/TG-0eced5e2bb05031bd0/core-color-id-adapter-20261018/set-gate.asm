142299840: push rbp
142299841: push r15
142299843: push r14
142299845: push r13
142299847: push r12
142299849: push rsi
14229984a: push rdi
14229984b: push rbx
14229984c: sub rsp, 0x178
142299853: lea rbp, [rsp + 0x80]
14229985b: movaps xmmword ptr [rbp + 0xe0], xmm8
142299863: movaps xmmword ptr [rbp + 0xd0], xmm7
14229986a: movaps xmmword ptr [rbp + 0xc0], xmm6
142299871: mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
14229987c: mov r14, r9
14229987f: mov rsi, rcx
142299882: mov r9, qword ptr [r8]
142299885: mov rbx, qword ptr [r8 + 8]
142299889: mov rax, qword ptr [rbx + 0x30]
14229988d: mov rcx, qword ptr [rbx + 0x40]
142299891: xor r8d, r8d
142299894: mov qword ptr [rbp + 0xa8], r9
14229989b: sub rcx, qword ptr [r9]
14229989e: cmovae r8, rcx
1422998a2: add rax, qword ptr [rbx + 0x10]
1422998a6: cmp rax, r8
1422998a9: cmovae rax, r8
1422998ad: test rax, rax
1422998b0: je 0x142299c55
1422998b6: mov rcx, qword ptr [rdx]
1422998b9: lea rdx, [rip + 0xe9ce80]
1422998c0: call 0x1425f18b0
1422998c5: test rax, rax
1422998c8: je 0x142299cd0
1422998ce: mov rdi, rax
1422998d1: lea rax, [rip + 0xe9cfc2]
1422998d8: mov qword ptr [rbp - 0x40], rax
1422998dc: mov qword ptr [rbp - 0x38], 0x32
1422998e4: cmp qword ptr [rdi + 0x18], 0x32
1422998e9: jne 0x142299c97
1422998ef: mov rdx, qword ptr [rdi + 0x10]
1422998f3: lea rcx, [rip + 0xe9cfa0]
1422998fa: mov r8d, 0x32
142299900: call 0x142923bf0
142299905: test eax, eax
142299907: jne 0x142299c97
14229990d: mov rcx, qword ptr [rdi]
142299910: mov rax, qword ptr [rdi + 8]
142299914: call qword ptr [rax + 0x18]
142299917: mov qword ptr [rbp + 0x40], rax
14229991b: test rax, rax
14229991e: je 0x142299cd0
142299924: mov qword ptr [rbp + 0x38], rsi
142299928: mov rcx, qword ptr [rbp + 0x168]
14229992f: mov r15, qword ptr [rbp + 0xa8]
142299936: mov r9, qword ptr [r15]
142299939: xor r8d, r8d
14229993c: mov r10, r9
14229993f: sub r10, qword ptr [rbx + 0x18]
142299943: cmovb r10, r8
142299947: sub r9, qword ptr [rbx + 0x38]
14229994b: mov rdx, qword ptr [rbp + 0x160]
142299952: cmovb r9, r8
142299956: lea rax, [r10 + r10*2]
14229995a: shl rax, 3
14229995e: add rax, qword ptr [rbx + 8]
142299962: mov r11, qword ptr [rbx + 0x10]
142299966: mov rdi, qword ptr [rbx + 0x30]
14229996a: sub r11, r10
14229996d: cmovb r11, r8
142299971: mov r10d, 8
142299977: cmovb rax, r10
14229997b: lea rsi, [r9 + r9*2]
14229997f: shl rsi, 3
142299983: add rsi, qword ptr [rbx + 0x28]
142299987: sub rdi, r9
14229998a: cmovb rdi, r8
14229998e: cmovb rsi, r10
142299992: mov r8, qword ptr [rbx + 0x40]
142299996: sub r8, rdi
142299999: sub r8, r11
14229999c: mov qword ptr [r15], r8
14229999f: lea r8, [r11 + r11*2]
1422999a3: lea r8, [rax + r8*8]
1422999a7: mov qword ptr [rbp + 0x10], r8
1422999ab: lea r8, [rdi + rdi*2]
1422999af: lea r8, [rsi + r8*8]
1422999b3: mov qword ptr [rbp + 0x20], r8
1422999b7: mov r8, qword ptr [r14]
1422999ba: mov qword ptr [rbp + 8], r8
1422999be: mov r8d, dword ptr [rcx + 0x1c]
1422999c2: mov dword ptr [rbp + 0xb0], r8d
1422999c9: mov r15, qword ptr [rcx]
1422999cc: mov rcx, qword ptr [rcx + 0x10]
1422999d0: mov qword ptr [rbp + 0x18], rcx
1422999d4: mov ecx, dword ptr [rdx + 0x1c]
1422999d7: mov dword ptr [rbp + 0xb4], ecx
1422999dd: mov r12, qword ptr [rdx]
1422999e0: mov rcx, qword ptr [rdx + 0x10]
1422999e4: mov qword ptr [rbp + 0x28], rcx
1422999e8: lea rcx, [r12 + 0x20]
1422999ed: mov qword ptr [rbp + 0x30], rcx
1422999f1: movaps xmm6, xmmword ptr [rip + 0x829428]
1422999f8: movaps xmm7, xmmword ptr [rip + 0x829431]
1422999ff: movss xmm8, dword ptr [rip + 0x68be68]
142299a08: jmp 0x142299a43
142299a0a: nop word ptr [rax + rax]
142299a10: mov rax, qword ptr [r12 + 0x28]
142299a15: mov rcx, rsi
142299a18: shl rcx, 5
142299a1c: mov qword ptr [rax + rcx], rdi
142299a20: mov edx, dword ptr [rbp + 0x58]
142299a23: mov dword ptr [rax + rcx + 0x18], edx
142299a27: movups xmm0, xmmword ptr [rbp + 0x48]
142299a2b: movups xmmword ptr [rax + rcx + 8], xmm0
142299a30: inc rsi
142299a33: mov qword ptr [r12 + 0x30], rsi
142299a38: inc qword ptr [r12 + 0x40]
142299a3d: mov rsi, r14
142299a40: mov rax, rbx
142299a43: test rax, rax
142299a46: je 0x142299a60
142299a48: cmp rax, qword ptr [rbp + 0x10]
142299a4c: je 0x142299a60
142299a4e: lea rbx, [rax + 0x18]
142299a52: mov r14, rsi
142299a55: mov r13, rax
142299a58: jmp 0x142299a7f
142299a5a: nop word ptr [rax + rax]
142299a60: test rsi, rsi
142299a63: je 0x142299bdb
142299a69: cmp rsi, qword ptr [rbp + 0x20]
142299a6d: je 0x142299bdb
142299a73: lea r14, [rsi + 0x18]
142299a77: xor ebx, ebx
142299a79: mov r13, rsi
142299a7c: mov rsi, r14
142299a7f: mov rax, qword ptr [rbp + 0xa8]
142299a86: inc qword ptr [rax]
142299a89: mov rdx, qword ptr [r13 + 8]
142299a8d: mov rcx, qword ptr [rbp + 8]
142299a91: lea r8, [rip + 0xda2478]
142299a98: call 0x140a91ef0
142299a9d: mov rdi, rax
142299aa0: mov rax, rbx
142299aa3: test rdi, rdi
142299aa6: je 0x142299a43
142299aa8: lea rcx, [r13 + 8]
142299aac: call 0x1400885d0
142299ab1: cmp rax, 0x5000
142299ab7: jae 0x142299c7f
142299abd: mov ecx, dword ptr [r13 + 0x10]
142299ac1: mov edx, dword ptr [rbp + 0xb0]
142299ac7: mov r8, qword ptr [rbp + 0x18]
142299acb: mov dword ptr [r8], edx
142299ace: mov rdx, qword ptr [r15 + 0x10]
142299ad2: cmp rax, rdx
142299ad5: jae 0x142299cc1
142299adb: mov rdx, qword ptr [r15 + 8]
142299adf: mov dword ptr [rdx + rax*4], ecx
142299ae2: cmp byte ptr [r13 + 0x14], 1
142299ae7: mov rsi, r14
142299aea: mov rax, rbx
142299aed: jne 0x142299a43
142299af3: mov rcx, qword ptr [rdi + 0x48]
142299af7: movaps xmm0, xmm7
142299afa: movaps xmm1, xmm6
142299afd: test rcx, rcx
142299b00: je 0x142299b61
142299b02: mov rax, qword ptr [rdi + 0x40]
142299b06: lea rcx, [rcx + rcx*2]
142299b0a: lea rcx, [rax + rcx*4]
142299b0e: movaps xmm0, xmm7
142299b11: movaps xmm2, xmm6
142299b14: movaps xmm1, xmm6
142299b17: nop word ptr [rax + rax]
142299b20: movss xmm3, dword ptr [rax]
142299b24: movss xmm4, dword ptr [rax + 8]
142299b29: unpcklps xmm3, xmm4
142299b2c: movaps xmm4, xmm3
142299b2f: minps xmm4, xmm0
142299b32: cmpunordps xmm0, xmm0
142299b36: movaps xmm5, xmm3
142299b39: andps xmm5, xmm0
142299b3c: andnps xmm0, xmm4
142299b3f: orps xmm0, xmm5
142299b42: cmpunordps xmm1, xmm1
142299b46: movaps xmm4, xmm3
142299b49: andps xmm4, xmm1
142299b4c: maxps xmm3, xmm2
142299b4f: andnps xmm1, xmm3
142299b52: orps xmm1, xmm4
142299b55: add rax, 0xc
142299b59: movaps xmm2, xmm1
142299b5c: cmp rax, rcx
142299b5f: jne 0x142299b20
142299b61: movlhps xmm0, xmm1
142299b64: movups xmmword ptr [rbp - 0x40], xmm0
142299b68: lea rcx, [rbp - 0x40]
142299b6c: call 0x1412400c0
142299b71: movss dword ptr [rbp + 0x98], xmm0
142299b79: movss dword ptr [rbp + 0x9c], xmm1
142299b81: mov rcx, rdi
142299b84: call 0x140b2a620
142299b89: mulss xmm0, xmm8
142299b8e: lea rcx, [rbp + 0x50]
142299b92: lea rdx, [rbp + 0x98]
142299b99: movaps xmm2, xmm0
142299b9c: call 0x140c9cde0
142299ba1: mov byte ptr [rbp + 0x48], 7
142299ba5: mov eax, dword ptr [rbp + 0xb4]
142299bab: mov rcx, qword ptr [rbp + 0x28]
142299baf: mov dword ptr [rcx], eax
142299bb1: mov rsi, qword ptr [r12 + 0x30]
142299bb6: mov rdi, qword ptr [r12 + 0x40]
142299bbb: cmp rsi, qword ptr [r12 + 0x20]
142299bc0: jne 0x142299a10
142299bc6: mov rcx, qword ptr [rbp + 0x30]
142299bca: lea rdx, [rip + 0xda16f7]
142299bd1: call 0x140562c30
142299bd6: jmp 0x142299a10
142299bdb: mov rax, qword ptr [rbp + 0x38]
142299bdf: mov rcx, qword ptr [rax]
142299be2: call 0x140987ca0
142299be7: mov rcx, qword ptr [r15 + 0x10]
142299beb: test rcx, rcx
142299bee: je 0x142299c55
142299bf0: shl rcx, 2
142299bf4: mov r8, qword ptr [rbp + 0x40]
142299bf8: cmp rcx, qword ptr [r8 + 0x18]
142299bfc: ja 0x142299cdc
142299c02: mov r9, qword ptr [r15 + 8]
142299c06: mov qword ptr [rbp + 0x98], 0
142299c11: mov qword ptr [rbp + 0xa0], rcx
142299c18: lea rdx, [rbp + 0x98]
142299c1f: mov qword ptr [rsp + 0x28], rdx
142299c24: mov qword ptr [rsp + 0x20], rcx
142299c29: mov qword ptr [rsp + 0x38], 0
142299c32: mov qword ptr [rsp + 0x30], 1
142299c3b: lea rcx, [rbp + 0x48]
142299c3f: mov rdx, rax
142299c42: call 0x140beefa0
142299c47: mov rax, qword ptr [rbp + 0x48]
142299c4b: cmp rax, 0xb
142299c4f: jne 0x142299ce3
142299c55: movaps xmm6, xmmword ptr [rbp + 0xc0]
142299c5c: movaps xmm7, xmmword ptr [rbp + 0xd0]
142299c63: movaps xmm8, xmmword ptr [rbp + 0xe0]
142299c6b: add rsp, 0x178
142299c72: pop rbx
142299c73: pop rdi
142299c74: pop rsi
142299c75: pop r12
142299c77: pop r13
142299c79: pop r14
142299c7b: pop r15
142299c7d: pop rbp
142299c7e: ret 
142299c7f: lea rcx, [rip + 0xda22a2]
142299c86: lea r8, [rip + 0xda22d3]
142299c8d: mov edx, 0x32
142299c92: call 0x1428d9430
142299c97: mov qword ptr [rbp + 0x48], 0
142299c9f: add rdi, 0x10
142299ca3: lea rax, [rip + 0xe9ca6e]
142299caa: mov qword ptr [rsp + 0x20], rax
142299caf: lea rdx, [rbp - 0x40]
142299cb3: lea r9, [rbp + 0x48]
142299cb7: xor ecx, ecx
142299cb9: mov r8, rdi
142299cbc: call 0x1428cab41
142299cc1: lea r8, [rip + 0xda22b0]
142299cc8: mov rcx, rax
142299ccb: call 0x1428d9518
142299cd0: lea rcx, [rip + 0xda2209]
142299cd7: call 0x1428d9310
142299cdc: mov eax, 0xb
142299ce1: jmp 0x142299d11
142299ce3: mov rcx, qword ptr [rbp + 0x90]
142299cea: mov qword ptr [rbp], rcx
142299cee: movups xmm0, xmmword ptr [rbp + 0x50]
142299cf2: movups xmm1, xmmword ptr [rbp + 0x60]
142299cf6: movups xmm2, xmmword ptr [rbp + 0x70]
142299cfa: movups xmm3, xmmword ptr [rbp + 0x80]
142299d01: movaps xmmword ptr [rbp - 0x10], xmm3
142299d05: movaps xmmword ptr [rbp - 0x20], xmm2
142299d09: movaps xmmword ptr [rbp - 0x30], xmm1
142299d0d: movaps xmmword ptr [rbp - 0x40], xmm0
142299d11: mov qword ptr [rbp + 0x48], rax
142299d15: movaps xmm0, xmmword ptr [rbp - 0x40]
142299d19: movaps xmm1, xmmword ptr [rbp - 0x30]
142299d1d: movaps xmm2, xmmword ptr [rbp - 0x20]
142299d21: movaps xmm3, xmmword ptr [rbp - 0x10]
142299d25: movups xmmword ptr [rbp + 0x50], xmm0
142299d29: movups xmmword ptr [rbp + 0x60], xmm1
142299d2d: movups xmmword ptr [rbp + 0x70], xmm2
142299d31: movups xmmword ptr [rbp + 0x80], xmm3
142299d38: mov rax, qword ptr [rbp]
142299d3c: mov qword ptr [rbp + 0x90], rax
142299d43: lea rax, [rip + 0xda21ae]
142299d4a: mov qword ptr [rsp + 0x20], rax
142299d4f: lea rcx, [rip + 0xda20f2]
142299d56: lea r9, [rip + 0xda20cb]
