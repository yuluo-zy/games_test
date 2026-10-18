1416289d0: push rbp
1416289d1: push r15
1416289d3: push r14
1416289d5: push r13
1416289d7: push r12
1416289d9: push rsi
1416289da: push rdi
1416289db: push rbx
1416289dc: sub rsp, 0x238
1416289e3: lea rbp, [rsp + 0x80]
1416289eb: mov qword ptr [rbp + 0x1b0], 0xfffffffffffffffe
1416289f6: mov qword ptr [rbp + 0x1a0], rdx
1416289fd: mov qword ptr [rbp + 0x198], rcx
141628a04: cmp qword ptr [r8 + 0x88], 0
141628a0c: movabs rbx, 0x65cfda6a55a841ac
141628a16: movabs r14, 0x8226cc786e679355
141628a20: je 0x141628aba
141628a26: mov rax, qword ptr [r8 + 0x70]
141628a2a: mov rcx, qword ptr [r8 + 0x78]
141628a2e: mov rdx, rcx
141628a31: and rdx, rbx
141628a34: xor r9d, r9d
141628a37: movdqa xmm0, xmmword ptr [rip + 0x13279d1]
141628a3f: pcmpeqd xmm1, xmm1
141628a43: movdqu xmm2, xmmword ptr [rax + rdx]
141628a48: movdqa xmm3, xmm2
141628a4c: pcmpeqb xmm3, xmm0
141628a50: pmovmskb r10d, xmm3
141628a55: test r10d, r10d
141628a58: jne 0x141628a8d
141628a5a: pcmpeqb xmm2, xmm1
141628a5e: pmovmskb r10d, xmm2
141628a63: test r10d, r10d
141628a66: jne 0x141628aba
141628a68: add rdx, r9
141628a6b: add rdx, 0x10
141628a6f: add r9, 0x10
141628a73: and rdx, rcx
141628a76: jmp 0x141628a43
141628a78: nop dword ptr [rax + rax]
141628a80: lea r11d, [r10 - 1]
141628a84: and r11w, r10w
141628a88: mov r10d, r11d
141628a8b: je 0x141628a5a
141628a8d: tzcnt r11d, r10d
141628a92: add r11, rdx
141628a95: and r11, rcx
141628a98: neg r11
141628a9b: lea r11, [r11 + r11*2]
141628a9f: cmp qword ptr [rax + r11*8 - 0x18], r14
141628aa4: jne 0x141628a80
141628aa6: cmp qword ptr [rax + r11*8 - 0x10], rbx
141628aab: jne 0x141628a80
141628aad: lea rax, [rax + r11*8]
141628ab1: mov rsi, qword ptr [rax - 8]
141628ab5: jmp 0x141628bbb
141628aba: lea rdi, [r8 + 0x90]
141628ac1: mov eax, dword ptr [r8 + 0x90]
141628ac8: cmp eax, 0x3ffffffd
141628acd: ja 0x1416295c4
141628ad3: lea ecx, [rax + 1]
141628ad6: lock cmpxchg dword ptr [rdi], ecx
141628ada: jne 0x1416295c4
141628ae0: movzx eax, byte ptr [r8 + 0x98]
141628ae8: cmp qword ptr [r8 + 0xf0], 0
141628af0: je 0x141629804
141628af6: mov rax, qword ptr [r8 + 0xd8]
141628afd: mov rcx, qword ptr [r8 + 0xe0]
141628b04: mov rdx, rcx
141628b07: and rdx, rbx
141628b0a: xor r9d, r9d
141628b0d: movdqa xmm0, xmmword ptr [rip + 0x13278fb]
141628b15: pcmpeqd xmm1, xmm1
141628b19: movdqu xmm2, xmmword ptr [rax + rdx]
141628b1e: movdqa xmm3, xmm2
141628b22: pcmpeqb xmm3, xmm0
141628b26: pmovmskb r10d, xmm3
141628b2b: test r10d, r10d
141628b2e: jne 0x141628b6d
141628b30: pcmpeqb xmm2, xmm1
141628b34: pmovmskb r10d, xmm2
141628b39: test r10d, r10d
141628b3c: jne 0x141629804
141628b42: add rdx, r9
141628b45: add rdx, 0x10
141628b49: add r9, 0x10
141628b4d: and rdx, rcx
141628b50: jmp 0x141628b19
141628b52: nop word ptr cs:[rax + rax]
141628b60: lea r11d, [r10 - 1]
141628b64: and r11w, r10w
141628b68: mov r10d, r11d
141628b6b: je 0x141628b30
141628b6d: tzcnt esi, r10d
141628b72: add rsi, rdx
141628b75: and rsi, rcx
141628b78: mov r11, rsi
141628b7b: neg r11
141628b7e: shl rsi, 3
141628b82: shl r11, 7
141628b86: sub r11, rsi
141628b89: cmp qword ptr [rax + r11 - 0x88], r14
141628b91: jne 0x141628b60
141628b93: cmp qword ptr [rax + r11 - 0x80], rbx
141628b98: jne 0x141628b60
141628b9a: add rax, r11
141628b9d: mov rsi, qword ptr [rax - 8]
141628ba1: mov edx, 0xffffffff
141628ba6: lock xadd dword ptr [rdi], edx
141628baa: dec edx
141628bac: mov eax, edx
141628bae: and eax, 0xbfffffff
141628bb3: neg eax
141628bb5: jo 0x1416297a4
141628bbb: cmp qword ptr [r8 + 0x218], rsi
141628bc2: jbe 0x14162981a
141628bc8: mov rax, qword ptr [r8 + 0x210]
141628bcf: mov rcx, qword ptr [rax + rsi*8]
141628bd3: test rcx, rcx
141628bd6: je 0x14162981a
141628bdc: mov rax, qword ptr [r8 + 0x1e0]
141628be3: not rcx
141628be6: lea rcx, [rcx + rcx*2]
141628bea: shl rcx, 5
141628bee: cmp qword ptr [rax + rcx + 0x20], 0
141628bf4: je 0x14162981a
141628bfa: add rax, rcx
141628bfd: mov rdi, qword ptr [rax + 0x10]
141628c01: mov byte ptr [rbp + 0x1af], 1
141628c08: lea rdx, [rip + 0x16057c9]
141628c0f: mov rcx, rdi
141628c12: call 0x140969720
141628c17: mov rbx, rax
141628c1a: test rax, rax
141628c1d: je 0x1416297da
141628c23: lea rax, [rip + 0x1475006]
141628c2a: mov qword ptr [rbp + 0xa0], rax
141628c31: mov qword ptr [rbp + 0xa8], 0x2c
141628c3c: mov r14, rbx
141628c3f: add r14, 0x10
141628c43: cmp qword ptr [rbx + 0x18], 0x2c
141628c48: jne 0x1416295de
141628c4e: mov rdx, qword ptr [r14]
141628c51: lea rcx, [rip + 0x1474fd8]
141628c58: mov r8d, 0x2c
141628c5e: call 0x142923bf0
141628c63: test eax, eax
141628c65: jne 0x1416295de
141628c6b: mov rcx, qword ptr [rbx]
141628c6e: mov rax, qword ptr [rbx + 8]
141628c72: mov rax, qword ptr [rax + 0x18]
141628c76: mov byte ptr [rbp + 0x1af], 1
141628c7d: call rax
141628c7f: mov rbx, rax
141628c82: test rax, rax
141628c85: je 0x1416297da
141628c8b: mov byte ptr [rbp + 0x1af], 1
141628c92: lea rdx, [rip + 0x16056b7]
141628c99: mov rcx, rdi
141628c9c: call 0x140969720
141628ca1: mov rdi, rax
141628ca4: test rax, rax
141628ca7: je 0x1416297ef
141628cad: lea rax, [rip + 0x1475a72]
141628cb4: mov qword ptr [rbp + 0xa0], rax
141628cbb: mov qword ptr [rbp + 0xa8], 0x26
141628cc6: mov r14, rdi
141628cc9: add r14, 0x10
141628ccd: cmp qword ptr [rdi + 0x18], 0x26
141628cd2: jne 0x141629613
141628cd8: mov rdx, qword ptr [r14]
141628cdb: lea rcx, [rip + 0x1475a44]
141628ce2: mov r8d, 0x26
141628ce8: call 0x142923bf0
141628ced: test eax, eax
141628cef: jne 0x141629613
141628cf5: mov rcx, qword ptr [rdi]
141628cf8: mov rax, qword ptr [rdi + 8]
141628cfc: mov rax, qword ptr [rax + 0x18]
141628d00: mov byte ptr [rbp + 0x1af], 1
141628d07: call rax
141628d09: mov rdi, rax
141628d0c: test rax, rax
141628d0f: je 0x1416297ef
141628d15: lea rcx, [rbp - 0x58]
141628d19: mov r8d, 0xf8
141628d1f: mov rdx, qword ptr [rbp + 0x1a0]
141628d26: call 0x142923c00
141628d2b: mov r14, qword ptr [rbp + 0x88]
141628d32: lea rdx, [r14 + 0x60]
141628d36: lea r8, [rip + 0x165882c]
141628d3d: lea rcx, [rbp + 0xa0]
141628d44: mov r9d, 0xb
141628d4a: call 0x141524340
141628d4f: mov r15, qword ptr [rbp + 0xa0]
141628d56: test r15, r15
141628d59: je 0x141628e2c
141628d5f: mov r13, qword ptr [rbp + 0xa8]
141628d66: mov r12, qword ptr [rbp + 0xb0]
141628d6d: mov rsi, qword ptr [rbp + 0xb8]
141628d74: lea r9, [r14 + 0x18]
141628d78: lea r8, [r14 + 0x90]
141628d7f: mov rdx, qword ptr [r14 + 0x1e0]
141628d86: add r14, 0x1a8
141628d8d: mov byte ptr [rsp + 0x20], 0x1f
141628d92: mov rcx, r14
141628d95: call 0x140972c20
141628d9a: mov rcx, qword ptr [r12]
141628d9e: mov rdx, qword ptr [r12 + 8]
141628da3: mov r8, rdx
141628da6: and r8, rsi
141628da9: movdqu xmm0, xmmword ptr [rcx + r8]
141628daf: pmovmskb r9d, xmm0
141628db4: test r9d, r9d
141628db7: je 0x141629648
141628dbd: tzcnt r9d, r9d
141628dc2: add r9, r8
141628dc5: and r9, rdx
141628dc8: movzx r8d, byte ptr [rcx + r9]
141628dcd: test r8b, r8b
141628dd0: jns 0x1416296ba
141628dd6: shr rsi, 0x39
141628dda: lea r10, [r9 - 0x10]
141628dde: and r10, rdx
141628de1: mov byte ptr [rcx + r9], sil
141628de5: mov byte ptr [rcx + r10 + 0x10], sil
141628dea: neg r9
141628ded: and r8b, 1
141628df1: movzx edx, r8b
141628df5: sub qword ptr [r12 + 0x10], rdx
141628dfa: lea r8, [r9 + r9*2]
141628dfe: lea rdx, [rcx + r8*8]
141628e02: mov qword ptr [rcx + r8*8 - 0x18], r15
141628e07: mov qword ptr [rcx + r8*8 - 0x10], r13
141628e0c: mov byte ptr [rcx + r8*8 - 8], al
141628e11: inc qword ptr [r12 + 0x18]
141628e16: mov rcx, qword ptr [rbp + 0x90]
141628e1d: cmp qword ptr [rcx + 0xb0], 0
141628e25: jne 0x141628e48
141628e27: jmp 0x141628f39
141628e2c: mov rdx, qword ptr [rbp + 0xa8]
141628e33: mov rcx, qword ptr [rbp + 0x90]
141628e3a: cmp qword ptr [rcx + 0xb0], 0
141628e42: je 0x141628f39
141628e48: movzx r14d, byte ptr [rdx - 8]
141628e4d: mov rax, qword ptr [rcx + 0x98]
141628e54: mov rcx, qword ptr [rcx + 0xa0]
141628e5b: movabs rdx, 0x168ddb2c3b5f38a9
141628e65: mov r8, rcx
141628e68: and r8, rdx
141628e6b: lea r9, [rax - 0x10]
141628e6f: xor r10d, r10d
141628e72: movdqa xmm0, xmmword ptr [rip + 0x137bd26]
141628e7a: pcmpeqd xmm1, xmm1
141628e7e: movdqu xmm2, xmmword ptr [rax + r8]
141628e84: movdqa xmm3, xmm2
141628e88: pcmpeqb xmm3, xmm0
141628e8c: pmovmskb r11d, xmm3
141628e91: test r11d, r11d
141628e94: je 0x141628ed0
141628e96: tzcnt esi, r11d
141628e9b: add rsi, r8
141628e9e: and rsi, rcx
141628ea1: mov r15, rsi
141628ea4: shl r15, 4
141628ea8: mov r12, r9
141628eab: sub r12, r15
141628eae: cmp qword ptr [r12], rdx
141628eb2: je 0x141628eee
141628eb4: lea esi, [r11 - 1]
141628eb8: and si, r11w
141628ebc: mov r11d, esi
141628ebf: jne 0x141628e96
141628ec1: nop word ptr cs:[rax + rax]
141628ed0: pcmpeqb xmm2, xmm1
141628ed4: pmovmskb r11d, xmm2
141628ed9: test r11d, r11d
141628edc: jne 0x141628f39
141628ede: add r8, r10
141628ee1: add r8, 0x10
141628ee5: add r10, 0x10
141628ee9: and r8, rcx
141628eec: jmp 0x141628e7e
141628eee: neg rsi
141628ef1: shl rsi, 4
141628ef5: mov ecx, dword ptr [rax + rsi - 8]
141628ef9: mov esi, dword ptr [rax + rsi - 4]
141628efd: mov dword ptr [rbp + 0x1a8], ecx
141628f03: test ecx, ecx
141628f05: jne 0x141629702
141628f0b: mov r15, qword ptr [rbp - 0x38]
141628f0f: cmp r15, qword ptr [rbp - 0x48]
141628f13: jne 0x141628f25
141628f15: lea rcx, [rbp - 0x48]
141628f19: lea rdx, [rip + 0x16584b0]
141628f20: call 0x140562e70
141628f25: mov rax, qword ptr [rbp - 0x40]
141628f29: mov dword ptr [rax + r15*8], esi
141628f2d: mov byte ptr [rax + r15*8 + 4], r14b
141628f32: inc r15
141628f35: mov qword ptr [rbp - 0x38], r15
141628f39: lea rcx, [rbp + 0xa0]
141628f40: lea rdx, [rbp - 0x58]
141628f44: mov r8d, 0xf8
141628f4a: call 0x142923c00
141628f4f: mov r14, qword ptr [rbp + 0x180]
141628f56: lea rdx, [r14 + 0x60]
141628f5a: lea r8, [rip + 0x16585fa]
141628f61: lea rcx, [rbp - 0x58]
141628f65: mov r9d, 0xe
141628f6b: call 0x141524340
141628f70: mov r15, qword ptr [rbp - 0x58]
141628f74: test r15, r15
141628f77: je 0x141629041
141628f7d: mov r13, qword ptr [rbp - 0x50]
141628f81: mov r12, qword ptr [rbp - 0x48]
141628f85: mov rsi, qword ptr [rbp - 0x40]
141628f89: lea r9, [r14 + 0x18]
141628f8d: lea r8, [r14 + 0x90]
141628f94: mov rdx, qword ptr [r14 + 0x1e0]
141628f9b: add r14, 0x1a8
141628fa2: mov byte ptr [rsp + 0x20], 0x1f
141628fa7: mov rcx, r14
141628faa: call 0x140971d20
141628faf: mov rcx, qword ptr [r12]
141628fb3: mov rdx, qword ptr [r12 + 8]
141628fb8: mov r8, rdx
141628fbb: and r8, rsi
141628fbe: movdqu xmm0, xmmword ptr [rcx + r8]
141628fc4: pmovmskb r9d, xmm0
141628fc9: test r9d, r9d
141628fcc: je 0x14162966e
141628fd2: tzcnt r9d, r9d
141628fd7: add r9, r8
141628fda: and r9, rdx
141628fdd: movzx r8d, byte ptr [rcx + r9]
141628fe2: test r8b, r8b
141628fe5: jns 0x1416296d2
141628feb: shr rsi, 0x39
141628fef: lea r10, [r9 - 0x10]
141628ff3: and r10, rdx
141628ff6: mov byte ptr [rcx + r9], sil
141628ffa: mov byte ptr [rcx + r10 + 0x10], sil
141628fff: neg r9
141629002: and r8b, 1
141629006: movzx edx, r8b
14162900a: sub qword ptr [r12 + 0x10], rdx
14162900f: lea r8, [r9 + r9*2]
141629013: lea rdx, [rcx + r8*8]
141629017: mov qword ptr [rcx + r8*8 - 0x18], r15
14162901c: mov qword ptr [rcx + r8*8 - 0x10], r13
141629021: mov byte ptr [rcx + r8*8 - 8], al
141629026: inc qword ptr [r12 + 0x18]
14162902b: mov rcx, qword ptr [rbp + 0x188]
141629032: cmp qword ptr [rcx + 0xb0], 0
14162903a: jne 0x14162905a
14162903c: jmp 0x141629158
141629041: mov rdx, qword ptr [rbp - 0x50]
141629045: mov rcx, qword ptr [rbp + 0x188]
14162904c: cmp qword ptr [rcx + 0xb0], 0
141629054: je 0x141629158
14162905a: movzx r14d, byte ptr [rdx - 8]
14162905f: mov rax, qword ptr [rcx + 0x98]
141629066: mov rcx, qword ptr [rcx + 0xa0]
14162906d: movabs rdx, 0x47ddebfb264383d0
141629077: mov r8, rcx
14162907a: and r8, rdx
14162907d: lea r9, [rax - 0x10]
141629081: xor r10d, r10d
141629084: movdqa xmm0, xmmword ptr [rip + 0x13275a4]
14162908c: pcmpeqd xmm1, xmm1
141629090: movdqu xmm2, xmmword ptr [rax + r8]
141629096: movdqa xmm3, xmm2
14162909a: pcmpeqb xmm3, xmm0
14162909e: pmovmskb r11d, xmm3
1416290a3: test r11d, r11d
1416290a6: je 0x1416290e0
1416290a8: tzcnt esi, r11d
1416290ad: add rsi, r8
1416290b0: and rsi, rcx
1416290b3: mov r15, rsi
1416290b6: shl r15, 4
1416290ba: mov r12, r9
1416290bd: sub r12, r15
1416290c0: cmp qword ptr [r12], rdx
1416290c4: je 0x1416290fe
1416290c6: lea esi, [r11 - 1]
1416290ca: and si, r11w
1416290ce: mov r11d, esi
1416290d1: jne 0x1416290a8
1416290d3: nop word ptr cs:[rax + rax]
1416290e0: pcmpeqb xmm2, xmm1
1416290e4: pmovmskb r11d, xmm2
1416290e9: test r11d, r11d
1416290ec: jne 0x141629158
1416290ee: add r8, r10
1416290f1: add r8, 0x10
1416290f5: add r10, 0x10
1416290f9: and r8, rcx
1416290fc: jmp 0x141629090
1416290fe: neg rsi
141629101: shl rsi, 4
141629105: mov ecx, dword ptr [rax + rsi - 8]
141629109: mov esi, dword ptr [rax + rsi - 4]
14162910d: mov dword ptr [rbp + 0x1a8], ecx
141629113: test ecx, ecx
141629115: jne 0x14162973a
14162911b: mov r15, qword ptr [rbp + 0xc0]
141629122: cmp r15, qword ptr [rbp + 0xb0]
141629129: jne 0x14162913e
14162912b: lea rcx, [rbp + 0xb0]
141629132: lea rdx, [rip + 0x1658297]
141629139: call 0x140562e70
14162913e: mov rax, qword ptr [rbp + 0xb8]
141629145: mov dword ptr [rax + r15*8], esi
141629149: mov byte ptr [rax + r15*8 + 4], r14b
14162914e: inc r15
141629151: mov qword ptr [rbp + 0xc0], r15
141629158: lea rcx, [rbp - 0x58]
14162915c: lea rdx, [rbp + 0xa0]
141629163: mov r8d, 0xf8
141629169: call 0x142923c00
14162916e: mov r14, qword ptr [rbp + 0x88]
141629175: lea rdx, [r14 + 0x60]
141629179: lea r8, [rip + 0x16582db]
141629180: lea rcx, [rbp + 0xa0]
141629187: mov r9d, 5
14162918d: call 0x141524340
141629192: mov r15, qword ptr [rbp + 0xa0]
141629199: test r15, r15
14162919c: je 0x14162926f
1416291a2: mov r13, qword ptr [rbp + 0xa8]
1416291a9: mov r12, qword ptr [rbp + 0xb0]
1416291b0: mov rsi, qword ptr [rbp + 0xb8]
1416291b7: lea r9, [r14 + 0x18]
1416291bb: lea r8, [r14 + 0x90]
1416291c2: mov rdx, qword ptr [r14 + 0x1e0]
1416291c9: add r14, 0x1a8
1416291d0: mov byte ptr [rsp + 0x20], 0x1f
1416291d5: mov rcx, r14
1416291d8: call 0x140970120
1416291dd: mov rcx, qword ptr [r12]
1416291e1: mov rdx, qword ptr [r12 + 8]
1416291e6: mov r8, rdx
1416291e9: and r8, rsi
1416291ec: movdqu xmm0, xmmword ptr [rcx + r8]
1416291f2: pmovmskb r9d, xmm0
1416291f7: test r9d, r9d
1416291fa: je 0x141629694
141629200: tzcnt r9d, r9d
141629205: add r9, r8
141629208: and r9, rdx
14162920b: movzx r8d, byte ptr [rcx + r9]
141629210: test r8b, r8b
141629213: jns 0x1416296ea
141629219: shr rsi, 0x39
14162921d: lea r10, [r9 - 0x10]
141629221: and r10, rdx
141629224: mov byte ptr [rcx + r9], sil
141629228: mov byte ptr [rcx + r10 + 0x10], sil
14162922d: neg r9
141629230: and r8b, 1
141629234: movzx edx, r8b
141629238: sub qword ptr [r12 + 0x10], rdx
14162923d: lea r8, [r9 + r9*2]
141629241: lea rdx, [rcx + r8*8]
141629245: mov qword ptr [rcx + r8*8 - 0x18], r15
14162924a: mov qword ptr [rcx + r8*8 - 0x10], r13
14162924f: mov byte ptr [rcx + r8*8 - 8], al
141629254: inc qword ptr [r12 + 0x18]
141629259: mov rcx, qword ptr [rbp + 0x90]
141629260: cmp qword ptr [rcx + 0xb0], 0
141629268: jne 0x14162928b
14162926a: jmp 0x141629379
14162926f: mov rdx, qword ptr [rbp + 0xa8]
141629276: mov rcx, qword ptr [rbp + 0x90]
14162927d: cmp qword ptr [rcx + 0xb0], 0
141629285: je 0x141629379
14162928b: movzx r14d, byte ptr [rdx - 8]
141629290: mov rax, qword ptr [rcx + 0x98]
141629297: mov rcx, qword ptr [rcx + 0xa0]
14162929e: movabs rdx, 0x339e76846042a271
1416292a8: mov r8, rcx
1416292ab: and r8, rdx
1416292ae: lea r9, [rax - 0x10]
1416292b2: xor r10d, r10d
1416292b5: movdqa xmm0, xmmword ptr [rip + 0x1327413]
1416292bd: pcmpeqd xmm1, xmm1
1416292c1: movdqu xmm2, xmmword ptr [rax + r8]
1416292c7: movdqa xmm3, xmm2
1416292cb: pcmpeqb xmm3, xmm0
1416292cf: pmovmskb r11d, xmm3
1416292d4: test r11d, r11d
1416292d7: je 0x141629310
1416292d9: tzcnt esi, r11d
1416292de: add rsi, r8
1416292e1: and rsi, rcx
1416292e4: mov r15, rsi
1416292e7: shl r15, 4
1416292eb: mov r12, r9
1416292ee: sub r12, r15
1416292f1: cmp qword ptr [r12], rdx
1416292f5: je 0x14162932e
1416292f7: lea esi, [r11 - 1]
1416292fb: and si, r11w
1416292ff: mov r11d, esi
141629302: jne 0x1416292d9
141629304: nop word ptr cs:[rax + rax]
141629310: pcmpeqb xmm2, xmm1
141629314: pmovmskb r11d, xmm2
141629319: test r11d, r11d
14162931c: jne 0x141629379
14162931e: add r8, r10
141629321: add r8, 0x10
141629325: add r10, 0x10
141629329: and r8, rcx
14162932c: jmp 0x1416292c1
14162932e: neg rsi
141629331: shl rsi, 4
141629335: mov ecx, dword ptr [rax + rsi - 8]
141629339: mov esi, dword ptr [rax + rsi - 4]
14162933d: mov dword ptr [rbp + 0x1a8], ecx
141629343: test ecx, ecx
141629345: jne 0x14162976c
14162934b: mov r15, qword ptr [rbp - 0x38]
14162934f: cmp r15, qword ptr [rbp - 0x48]
141629353: jne 0x141629365
141629355: lea rcx, [rbp - 0x48]
141629359: lea rdx, [rip + 0x1658070]
141629360: call 0x140562e70
141629365: mov rax, qword ptr [rbp - 0x40]
141629369: mov dword ptr [rax + r15*8], esi
14162936d: mov byte ptr [rax + r15*8 + 4], r14b
141629372: inc r15
141629375: mov qword ptr [rbp - 0x38], r15
141629379: lea r14, [rbp + 0xa0]
141629380: lea rdx, [rbp - 0x58]
141629384: mov r8d, 0xf8
14162938a: mov rcx, r14
14162938d: call 0x142923c00
141629392: mov byte ptr [rbp + 0x1af], 0
141629399: lea rcx, [rbp - 0x58]
14162939d: movabs r8, 0xe9701a257aa8828c
1416293a7: mov rdx, r14
1416293aa: call 0x14162c290
1416293af: mov rax, qword ptr [rbx]
1416293b2: cmp byte ptr [rbp + 0x86], 8
1416293b9: je 0x141629494
1416293bf: mov r9, qword ptr [rbp + 0x90]
1416293c6: cmp qword ptr [r9 + 0xe8], 0
1416293ce: je 0x141629494
1416293d4: cmp qword ptr [r9 + 0x130], 0
1416293dc: je 0x141629494
1416293e2: mov r8, qword ptr [rbp + 0x60]
1416293e6: mov rdx, qword ptr [rbp + 0x68]
1416293ea: mov rcx, qword ptr [r9 + 0x118]
1416293f1: mov r9, qword ptr [r9 + 0x120]
1416293f8: movabs r10, 0x1c9f2eef0ac739ac
141629402: mov r11, r9
141629405: and r11, r10
141629408: lea rsi, [rcx - 0x10]
14162940c: xor ebx, ebx
14162940e: movdqa xmm0, xmmword ptr [rip + 0x13270ca]
141629416: pcmpeqd xmm1, xmm1
14162941a: movdqu xmm2, xmmword ptr [rcx + r11]
141629420: movdqa xmm3, xmm2
141629424: pcmpeqb xmm3, xmm0
141629428: pmovmskb r14d, xmm3
14162942d: test r14d, r14d
141629430: je 0x14162945d
141629432: tzcnt r15d, r14d
141629437: add r15, r11
14162943a: and r15, r9
14162943d: mov r12, r15
141629440: shl r12, 4
141629444: mov r13, rsi
141629447: sub r13, r12
14162944a: cmp qword ptr [r13], r10
14162944e: je 0x14162947b
141629450: lea r15d, [r14 - 1]
141629454: and r15w, r14w
141629458: mov r14d, r15d
14162945b: jne 0x141629432
14162945d: pcmpeqb xmm2, xmm1
141629461: pmovmskb r14d, xmm2
141629466: test r14d, r14d
141629469: jne 0x141629494
14162946b: add r11, rbx
14162946e: add r11, 0x10
141629472: add rbx, 0x10
141629476: and r11, r9
141629479: jmp 0x14162941a
14162947b: neg r15
14162947e: shl r15, 4
141629482: mov ecx, dword ptr [rcx + r15 - 8]
141629487: cmp rdx, rcx
14162948a: jb 0x1416297be
141629490: mov dword ptr [r8 + rcx], eax
141629494: lea rcx, [rbp + 0xa0]
14162949b: lea rdx, [rbp - 0x58]
14162949f: mov r8d, 0xf8
1416294a5: call 0x142923c00
1416294aa: mov rax, qword ptr [rdi]
1416294ad: cmp byte ptr [rbp + 0x17e], 8
1416294b4: je 0x141629591
1416294ba: mov r9, qword ptr [rbp + 0x188]
1416294c1: cmp qword ptr [r9 + 0xe8], 0
1416294c9: je 0x141629591
1416294cf: cmp qword ptr [r9 + 0x130], 0
1416294d7: je 0x141629591
1416294dd: mov r8, qword ptr [rbp + 0x158]
1416294e4: mov rdx, qword ptr [rbp + 0x160]
1416294eb: mov rcx, qword ptr [r9 + 0x118]
1416294f2: mov r9, qword ptr [r9 + 0x120]
1416294f9: movabs r10, 0xc2b7f2fdf5e21521
141629503: mov r11, r9
141629506: and r11, r10
141629509: lea rsi, [rcx - 0x10]
14162950d: xor edi, edi
14162950f: movdqa xmm0, xmmword ptr [rip + 0x1326ee9]
141629517: pcmpeqd xmm1, xmm1
14162951b: movdqu xmm2, xmmword ptr [rcx + r11]
141629521: movdqa xmm3, xmm2
141629525: pcmpeqb xmm3, xmm0
141629529: pmovmskb ebx, xmm3
14162952d: test ebx, ebx
14162952f: je 0x14162955c
141629531: tzcnt r14d, ebx
141629536: add r14, r11
141629539: and r14, r9
14162953c: mov r15, r14
14162953f: shl r15, 4
141629543: mov r12, rsi
141629546: sub r12, r15
141629549: cmp qword ptr [r12], r10
14162954d: je 0x141629578
14162954f: lea r14d, [rbx - 1]
141629553: and r14w, bx
141629557: mov ebx, r14d
14162955a: jne 0x141629531
14162955c: pcmpeqb xmm2, xmm1
141629560: pmovmskb ebx, xmm2
141629564: test ebx, ebx
141629566: jne 0x141629591
141629568: add r11, rdi
14162956b: add r11, 0x10
14162956f: add rdi, 0x10
141629573: and r11, r9
141629576: jmp 0x14162951b
141629578: neg r14
14162957b: shl r14, 4
14162957f: mov ecx, dword ptr [rcx + r14 - 8]
141629584: cmp rdx, rcx
141629587: jb 0x1416297cc
14162958d: mov dword ptr [r8 + rcx], eax
141629591: lea rdx, [rbp + 0xa0]
141629598: mov r8d, 0xf8
14162959e: mov rsi, qword ptr [rbp + 0x198]
1416295a5: mov rcx, rsi
1416295a8: call 0x142923c00
1416295ad: mov rax, rsi
1416295b0: add rsp, 0x238
1416295b7: pop rbx
1416295b8: pop rdi
1416295b9: pop rsi
1416295ba: pop r12
1416295bc: pop r13
1416295be: pop r14
1416295c0: pop r15
1416295c2: pop rbp
1416295c3: ret 
1416295c4: mov byte ptr [rbp + 0x1af], 1
1416295cb: mov rcx, rdi
1416295ce: mov rsi, r8
1416295d1: call 0x1428d8a30
1416295d6: mov r8, rsi
1416295d9: jmp 0x141628ae0
1416295de: mov qword ptr [rbp - 0x58], 0
1416295e6: mov byte ptr [rbp + 0x1af], 1
1416295ed: lea rax, [rip + 0x1604b8c]
1416295f4: mov qword ptr [rsp + 0x20], rax
1416295f9: lea rdx, [rbp + 0xa0]
141629600: lea r9, [rbp - 0x58]
141629604: xor ecx, ecx
141629606: mov r8, r14
141629609: call 0x1428cab41
14162960e: jmp 0x14162982d
141629613: mov qword ptr [rbp - 0x58], 0
14162961b: mov byte ptr [rbp + 0x1af], 1
141629622: lea rax, [rip + 0x1604b57]
141629629: mov qword ptr [rsp + 0x20], rax
14162962e: lea rdx, [rbp + 0xa0]
141629635: lea r9, [rbp - 0x58]
141629639: xor ecx, ecx
14162963b: mov r8, r14
14162963e: call 0x1428cab41
141629643: jmp 0x14162982d
141629648: mov r10d, 0x10
14162964e: add r8, r10
141629651: and r8, rdx
141629654: movdqu xmm0, xmmword ptr [rcx + r8]
14162965a: pmovmskb r9d, xmm0
14162965f: add r10, 0x10
141629663: test r9d, r9d
141629666: jne 0x141628dbd
14162966c: jmp 0x14162964e
14162966e: mov r10d, 0x10
141629674: add r8, r10
141629677: and r8, rdx
14162967a: movdqu xmm0, xmmword ptr [rcx + r8]
141629680: pmovmskb r9d, xmm0
141629685: add r10, 0x10
141629689: test r9d, r9d
14162968c: jne 0x141628fd2
141629692: jmp 0x141629674
141629694: mov r10d, 0x10
14162969a: add r8, r10
14162969d: and r8, rdx
1416296a0: movdqu xmm0, xmmword ptr [rcx + r8]
1416296a6: pmovmskb r9d, xmm0
1416296ab: add r10, 0x10
1416296af: test r9d, r9d
1416296b2: jne 0x141629200
1416296b8: jmp 0x14162969a
1416296ba: movdqa xmm0, xmmword ptr [rcx]
1416296be: pmovmskb r8d, xmm0
1416296c3: tzcnt r9d, r8d
1416296c8: movzx r8d, byte ptr [rcx + r9]
1416296cd: jmp 0x141628dd6
1416296d2: movdqa xmm0, xmmword ptr [rcx]
1416296d6: pmovmskb r8d, xmm0
1416296db: tzcnt r9d, r8d
1416296e0: movzx r8d, byte ptr [rcx + r9]
1416296e5: jmp 0x141628feb
1416296ea: movdqa xmm0, xmmword ptr [rcx]
1416296ee: pmovmskb r8d, xmm0
1416296f3: tzcnt r9d, r8d
1416296f8: movzx r8d, byte ptr [rcx + r9]
1416296fd: jmp 0x141629219
141629702: mov qword ptr [rbp + 0xa0], 0
14162970d: lea rax, [rip + 0x1657ca4]
141629714: mov qword ptr [rsp + 0x20], rax
141629719: lea r8, [rip + 0x1657c54]
141629720: lea rdx, [rbp + 0x1a8]
141629727: lea r9, [rbp + 0xa0]
14162972e: xor ecx, ecx
141629730: call 0x1428a994f
141629735: jmp 0x14162982d
14162973a: mov qword ptr [rbp - 0x58], 0
141629742: lea rax, [rip + 0x1657c6f]
141629749: mov qword ptr [rsp + 0x20], rax
14162974e: lea r8, [rip + 0x1657c1f]
141629755: lea rdx, [rbp + 0x1a8]
14162975c: lea r9, [rbp - 0x58]
141629760: xor ecx, ecx
141629762: call 0x1428a994f
141629767: jmp 0x14162982d
14162976c: mov qword ptr [rbp + 0xa0], 0
141629777: lea rax, [rip + 0x1657c3a]
14162977e: mov qword ptr [rsp + 0x20], rax
141629783: lea r8, [rip + 0x1657bea]
14162978a: lea rdx, [rbp + 0x1a8]
141629791: lea r9, [rbp + 0xa0]
141629798: xor ecx, ecx
14162979a: call 0x1428a994f
14162979f: jmp 0x14162982d
1416297a4: mov byte ptr [rbp + 0x1af], 1
1416297ab: mov rcx, rdi
1416297ae: mov rdi, r8
1416297b1: call 0x1428d8d40
1416297b6: mov r8, rdi
1416297b9: jmp 0x141628bbb
1416297be: lea r8, [rip + 0x148c3fb]
1416297c5: call 0x1428d9830
1416297ca: jmp 0x14162982d
1416297cc: lea r8, [rip + 0x148c3ed]
1416297d3: call 0x1428d9830
1416297d8: jmp 0x14162982d
1416297da: mov byte ptr [rbp + 0x1af], 1
1416297e1: lea rcx, [rip + 0x1657b50]
1416297e8: call 0x1428d9310
1416297ed: jmp 0x14162982d
1416297ef: mov byte ptr [rbp + 0x1af], 1
1416297f6: lea rcx, [rip + 0x1657b53]
1416297fd: call 0x1428d9310
141629802: jmp 0x14162982d
141629804: mov edx, 0xffffffff
141629809: lock xadd dword ptr [rdi], edx
14162980d: dec edx
14162980f: mov eax, edx
141629811: and eax, 0xbfffffff
141629816: neg eax
141629818: jo 0x14162982f
14162981a: mov byte ptr [rbp + 0x1af], 1
141629821: lea rcx, [rip + 0x1657af8]
141629828: call 0x1428d9310
14162982d: ud2 
14162982f: mov byte ptr [rbp + 0x1af], 1
141629836: mov rcx, rdi
141629839: call 0x1428d8d40
14162983e: jmp 0x14162981a
141629840: mov qword ptr [rsp + 0x10], rdx
141629845: push rbp
141629846: push r15
141629848: push r14
14162984a: push r13
14162984c: push r12
14162984e: push rsi
14162984f: push rdi
141629850: push rbx
141629851: sub rsp, 0x28
141629855: lea rbp, [rdx + 0x80]
14162985c: cmp byte ptr [rbp + 0x1af], 0
141629863: jne 0x141629876
141629865: add rsp, 0x28
141629869: pop rbx
14162986a: pop rdi
14162986b: pop rsi
14162986c: pop r12
14162986e: pop r13
141629870: pop r14
141629872: pop r15
141629874: pop rbp
141629875: ret 
141629876: mov rcx, qword ptr [rbp + 0x1a0]
14162987d: call 0x1408935c0
141629882: jmp 0x141629865
141629884: nop word ptr cs:[rax + rax]
141629890: mov qword ptr [rsp + 0x10], rdx
141629895: push rbp
141629896: push r15
141629898: push r14
14162989a: push r13
14162989c: push r12
14162989e: push rsi
14162989f: push rdi
1416298a0: push rbx
1416298a1: sub rsp, 0x28
1416298a5: lea rbp, [rdx + 0x80]
1416298ac: mov byte ptr [rbp + 0x1af], 0
1416298b3: lea rcx, [rbp - 0x58]
1416298b7: call 0x1408935c0
1416298bc: mov byte ptr [rbp + 0x1af], 0
1416298c3: add rsp, 0x28
1416298c7: pop rbx
1416298c8: pop rdi
1416298c9: pop rsi
1416298ca: pop r12
1416298cc: pop r13
1416298ce: pop r14
1416298d0: pop r15
1416298d2: pop rbp
1416298d3: ret 
1416298d4: nop word ptr cs:[rax + rax]
1416298e0: mov qword ptr [rsp + 0x10], rdx
1416298e5: push rbp
1416298e6: push r15
1416298e8: push r14
1416298ea: push r13
1416298ec: push r12
1416298ee: push rsi
1416298ef: push rdi
1416298f0: push rbx
1416298f1: sub rsp, 0x28
1416298f5: lea rbp, [rdx + 0x80]
1416298fc: mov byte ptr [rbp + 0x1af], 0
141629903: lea rcx, [rbp + 0xa0]
14162990a: call 0x1408935c0
14162990f: mov byte ptr [rbp + 0x1af], 0
141629916: add rsp, 0x28
14162991a: pop rbx
14162991b: pop rdi
14162991c: pop rsi
14162991d: pop r12
14162991f: pop r13
141629921: pop r14
141629923: pop r15
141629925: pop rbp
141629926: ret 
141629927: nop word ptr [rax + rax]
141629930: mov qword ptr [rsp + 0x10], rdx
141629935: push rbp
141629936: push r15
141629938: push r14
14162993a: push r13
14162993c: push r12
14162993e: push rsi
14162993f: push rdi
141629940: push rbx
141629941: sub rsp, 0x28
141629945: lea rbp, [rdx + 0x80]
14162994c: mov byte ptr [rbp + 0x1af], 0
141629953: lea rcx, [rbp - 0x58]
141629957: call 0x1408935c0
14162995c: mov byte ptr [rbp + 0x1af], 0
141629963: add rsp, 0x28
141629967: pop rbx
141629968: pop rdi
141629969: pop rsi
14162996a: pop r12
14162996c: pop r13
14162996e: pop r14
141629970: pop r15
141629972: pop rbp
141629973: ret 
141629974: nop word ptr cs:[rax + rax]
141629980: mov qword ptr [rsp + 0x10], rdx
141629985: push rbp
141629986: push r15
141629988: push r14
14162998a: push r13
14162998c: push r12
14162998e: push rsi
14162998f: push rdi
141629990: push rbx
141629991: sub rsp, 0x28
141629995: lea rbp, [rdx + 0x80]
14162999c: mov byte ptr [rbp + 0x1af], 0
1416299a3: lea rcx, [rbp - 0x58]
1416299a7: call 0x1408935c0
1416299ac: mov byte ptr [rbp + 0x1af], 0
1416299b3: add rsp, 0x28
1416299b7: pop rbx
1416299b8: pop rdi
1416299b9: pop rsi
1416299ba: pop r12
1416299bc: pop r13
1416299be: pop r14
1416299c0: pop r15
1416299c2: pop rbp
1416299c3: ret 
1416299c4: nop word ptr cs:[rax + rax]
1416299d0: mov qword ptr [rsp + 0x10], rdx
1416299d5: push rbp
1416299d6: push r15
1416299d8: push r14
1416299da: push r13
1416299dc: push r12
1416299de: push rsi
1416299df: push rdi
1416299e0: push rbx
1416299e1: sub rsp, 0x28
1416299e5: lea rbp, [rdx + 0x80]
1416299ec: mov byte ptr [rbp + 0x1af], 0
1416299f3: lea rcx, [rbp + 0xa0]
1416299fa: call 0x1408935c0
1416299ff: mov byte ptr [rbp + 0x1af], 0
141629a06: add rsp, 0x28
141629a0a: pop rbx
141629a0b: pop rdi
141629a0c: pop rsi
141629a0d: pop r12
141629a0f: pop r13
141629a11: pop r14
141629a13: pop r15
141629a15: pop rbp
141629a16: ret 
