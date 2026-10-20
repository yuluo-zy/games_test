140a889b0: 4157                      push r15
140a889b2: 4156                      push r14
140a889b4: 4155                      push r13
140a889b6: 4154                      push r12
140a889b8: 56                        push rsi
140a889b9: 57                        push rdi
140a889ba: 55                        push rbp
140a889bb: 53                        push rbx
140a889bc: 4881ec18010000            sub rsp, 0x118
140a889c3: 440f299c2400010000        movaps xmmword ptr [rsp + 0x100], xmm11
140a889cc: 440f299424f0000000        movaps xmmword ptr [rsp + 0xf0], xmm10
140a889d5: 440f298c24e0000000        movaps xmmword ptr [rsp + 0xe0], xmm9
140a889de: 440f298424d0000000        movaps xmmword ptr [rsp + 0xd0], xmm8
140a889e7: 0f29bc24c0000000          movaps xmmword ptr [rsp + 0xc0], xmm7
140a889ef: 0f29b424b0000000          movaps xmmword ptr [rsp + 0xb0], xmm6
140a889f7: 4c89cb                    mov rbx, r9
140a889fa: 4c89c6                    mov rsi, r8
140a889fd: 4c8b39                    mov r15, qword ptr [rcx]
140a88a00: 488b3a                    mov rdi, qword ptr [rdx]
140a88a03: 0fb687d0040000            movzx eax, byte ptr [rdi + 0x4d0]
140a88a0a: 498b0f                    mov rcx, qword ptr [r15]
140a88a0d: 4885c9                    test rcx, rcx
140a88a10: 742e                      je 0x140a88a40
140a88a12: 3c03                      cmp al, 3
140a88a14: 7405                      je 0x140a88a1b
140a88a16: 0f57db                    xorps xmm3, xmm3
140a88a19: eb08                      jmp 0x140a88a23
140a88a1b: f30f101de1cee901          movss xmm3, dword ptr [rip + 0x1e9cee1]
140a88a23: 4883c110                  add rcx, 0x10
140a88a27: 488d1582040502            lea rdx, [rip + 0x2050482]
140a88a2e: 41b80b000000              mov r8d, 0xb
140a88a34: e877b3fcff                call 0x140a53db0
140a88a39: 0fb687d0040000            movzx eax, byte ptr [rdi + 0x4d0]
140a88a40: 4c8ba42480010000          mov r12, qword ptr [rsp + 0x180]
140a88a48: 3c03                      cmp al, 3
140a88a4a: 0f858b040000              jne 0x140a88edb
140a88a50: 488b842498010000          mov rax, qword ptr [rsp + 0x198]
140a88a58: 488b00                    mov rax, qword ptr [rax]
140a88a5b: 83781803                  cmp dword ptr [rax + 0x18], 3
140a88a5f: 7226                      jb 0x140a88a87
140a88a61: f20f108050010000          movsd xmm0, qword ptr [rax + 0x150]
140a88a69: 0f59c0                    mulps xmm0, xmm0
140a88a6c: f30f16c8                  movshdup xmm1, xmm0
140a88a70: f30f58c8                  addss xmm1, xmm0
140a88a74: 0f57c0                    xorps xmm0, xmm0
140a88a77: f30f51c1                  sqrtss xmm0, xmm1
140a88a7b: 0f57c9                    xorps xmm1, xmm1
140a88a7e: 0f2ec1                    ucomiss xmm0, xmm1
140a88a81: 0f8654040000              jbe 0x140a88edb
140a88a87: 0fb6af44030000            movzx ebp, byte ptr [rdi + 0x344]
140a88a8e: f30f10355a5cea01          movss xmm6, dword ptr [rip + 0x1ea5c5a]
140a88a96: 80bf280300000b            cmp byte ptr [rdi + 0x328], 0xb
140a88a9d: 0f8512040000              jne 0x140a88eb5
140a88aa3: 488b0e                    mov rcx, qword ptr [rsi]
140a88aa6: 488d15fb571a02            lea rdx, [rip + 0x21a57fb]
140a88aad: e86e0ceeff                call 0x140969720
140a88ab2: 4885c0                    test rax, rax
140a88ab5: 0f84c1040000              je 0x140a88f7c
140a88abb: 4889c6                    mov rsi, rax
140a88abe: 488d05fc490102            lea rax, [rip + 0x20149fc]
140a88ac5: 4889442430                mov qword ptr [rsp + 0x30], rax
140a88aca: 48c74424383f000000        mov qword ptr [rsp + 0x38], 0x3f
140a88ad3: 48837e183f                cmp qword ptr [rsi + 0x18], 0x3f
140a88ad8: 0f8571040000              jne 0x140a88f4f
140a88ade: 488b5610                  mov rdx, qword ptr [rsi + 0x10]
140a88ae2: 488d0dd8490102            lea rcx, [rip + 0x20149d8]
140a88ae9: 41b83f000000              mov r8d, 0x3f
140a88aef: e8fcb0e901                call 0x142923bf0
140a88af4: 85c0                      test eax, eax
140a88af6: 0f8553040000              jne 0x140a88f4f
140a88afc: 488b0e                    mov rcx, qword ptr [rsi]
140a88aff: 488b4608                  mov rax, qword ptr [rsi + 8]
140a88b03: ff5018                    call qword ptr [rax + 0x18]
140a88b06: 4885c0                    test rax, rax
140a88b09: 0f846d040000              je 0x140a88f7c
140a88b0f: 488b8c2490010000          mov rcx, qword ptr [rsp + 0x190]
140a88b17: 488b942488010000          mov rdx, qword ptr [rsp + 0x188]
140a88b1f: 4c8b00                    mov r8, qword ptr [rax]
140a88b22: 4c89442440                mov qword ptr [rsp + 0x40], r8
140a88b27: 488b12                    mov rdx, qword ptr [rdx]
140a88b2a: 4c8d4208                  lea r8, [rdx + 8]
140a88b2e: 4c8d4a20                  lea r9, [rdx + 0x20]
140a88b32: 4c8d5210                  lea r10, [rdx + 0x10]
140a88b36: 4c8d5a28                  lea r11, [rdx + 0x28]
140a88b3a: 80ba9802000000            cmp byte ptr [rdx + 0x298], 0
140a88b41: 4d0f45c1                  cmovne r8, r9
140a88b45: 4d0f44da                  cmove r11, r10
140a88b49: 488b4008                  mov rax, qword ptr [rax + 8]
140a88b4d: 4889442448                mov qword ptr [rsp + 0x48], rax
140a88b52: 498b03                    mov rax, qword ptr [r11]
140a88b55: 498b10                    mov rdx, qword ptr [r8]
140a88b58: 48899424a0000000          mov qword ptr [rsp + 0xa0], rdx
140a88b60: 48898424a8000000          mov qword ptr [rsp + 0xa8], rax
140a88b68: 488b03                    mov rax, qword ptr [rbx]
140a88b6b: 488d5008                  lea rdx, [rax + 8]
140a88b6f: 4c8d4020                  lea r8, [rax + 0x20]
140a88b73: 4c8d4810                  lea r9, [rax + 0x10]
140a88b77: 4c8d5028                  lea r10, [rax + 0x28]
140a88b7b: 80b89802000000            cmp byte ptr [rax + 0x298], 0
140a88b82: 490f45d0                  cmovne rdx, r8
140a88b86: 4d0f44d1                  cmove r10, r9
140a88b8a: 4c8b4320                  mov r8, qword ptr [rbx + 0x20]
140a88b8e: 498b02                    mov rax, qword ptr [r10]
140a88b91: 4d8d4808                  lea r9, [r8 + 8]
140a88b95: 4d8d5020                  lea r10, [r8 + 0x20]
140a88b99: 4d8d5810                  lea r11, [r8 + 0x10]
140a88b9d: 4d8d7028                  lea r14, [r8 + 0x28]
140a88ba1: 4180b89802000000          cmp byte ptr [r8 + 0x298], 0
140a88ba9: 4d0f45ca                  cmovne r9, r10
140a88bad: 4d0f44f3                  cmove r14, r11
140a88bb1: 488b12                    mov rdx, qword ptr [rdx]
140a88bb4: 4d8b06                    mov r8, qword ptr [r14]
140a88bb7: 4d8b09                    mov r9, qword ptr [r9]
140a88bba: 4c8b5340                  mov r10, qword ptr [rbx + 0x40]
140a88bbe: 4d8d5a08                  lea r11, [r10 + 8]
140a88bc2: 4d8d7220                  lea r14, [r10 + 0x20]
140a88bc6: 4d8d6a10                  lea r13, [r10 + 0x10]
140a88bca: 498d7228                  lea rsi, [r10 + 0x28]
140a88bce: 4180ba9802000000          cmp byte ptr [r10 + 0x298], 0
140a88bd6: 4d0f45de                  cmovne r11, r14
140a88bda: 490f44f5                  cmove rsi, r13
140a88bde: 4c8b16                    mov r10, qword ptr [rsi]
140a88be1: 4d8b1b                    mov r11, qword ptr [r11]
140a88be4: 488b7360                  mov rsi, qword ptr [rbx + 0x60]
140a88be8: 488d5e08                  lea rbx, [rsi + 8]
140a88bec: 4c8d7620                  lea r14, [rsi + 0x20]
140a88bf0: 4c8d6e10                  lea r13, [rsi + 0x10]
140a88bf4: 80be9802000000            cmp byte ptr [rsi + 0x298], 0
140a88bfb: 490f45de                  cmovne rbx, r14
140a88bff: 488d7628                  lea rsi, [rsi + 0x28]
140a88c03: 490f44f5                  cmove rsi, r13
140a88c07: 488b36                    mov rsi, qword ptr [rsi]
140a88c0a: 488b1b                    mov rbx, qword ptr [rbx]
140a88c0d: 4889542460                mov qword ptr [rsp + 0x60], rdx
140a88c12: 4889442468                mov qword ptr [rsp + 0x68], rax
140a88c17: 4c894c2470                mov qword ptr [rsp + 0x70], r9
140a88c1c: 4c89442478                mov qword ptr [rsp + 0x78], r8
140a88c21: 4c899c2480000000          mov qword ptr [rsp + 0x80], r11
140a88c29: 4c89942488000000          mov qword ptr [rsp + 0x88], r10
140a88c31: 48899c2490000000          mov qword ptr [rsp + 0x90], rbx
140a88c39: 4889b42498000000          mov qword ptr [rsp + 0x98], rsi
140a88c41: 488b01                    mov rax, qword ptr [rcx]
140a88c44: 488d4808                  lea rcx, [rax + 8]
140a88c48: 488d5020                  lea rdx, [rax + 0x20]
140a88c4c: 4c8d4010                  lea r8, [rax + 0x10]
140a88c50: 4c8d4828                  lea r9, [rax + 0x28]
140a88c54: 80b89802000000            cmp byte ptr [rax + 0x298], 0
140a88c5b: 480f45ca                  cmovne rcx, rdx
140a88c5f: 4d0f44c8                  cmove r9, r8
140a88c63: 498b01                    mov rax, qword ptr [r9]
140a88c66: 488b09                    mov rcx, qword ptr [rcx]
140a88c69: 48894c2430                mov qword ptr [rsp + 0x30], rcx
140a88c6e: 4889442438                mov qword ptr [rsp + 0x38], rax
140a88c73: 488d4c2454                lea rcx, [rsp + 0x54]
140a88c78: 4889fa                    mov rdx, rdi
140a88c7b: e8c0e02000                call 0x140c96d40
140a88c80: f3440f10442454            movss xmm8, dword ptr [rsp + 0x54]
140a88c87: f30f107c245c              movss xmm7, dword ptr [rsp + 0x5c]
140a88c8d: c744242000000243          mov dword ptr [rsp + 0x20], 0x43020000
140a88c95: 488d8c24a0000000          lea rcx, [rsp + 0xa0]
140a88c9d: f30f101db329ff01          movss xmm3, dword ptr [rip + 0x1ff29b3]
140a88ca5: 410f28c8                  movaps xmm1, xmm8
140a88ca9: 0f28d7                    movaps xmm2, xmm7
140a88cac: e83fc1f8ff                call 0x140a14df0
140a88cb1: 0f2e05cccce901            ucomiss xmm0, dword ptr [rip + 0x1e9cccc]
140a88cb8: 760d                      jbe 0x140a88cc7
140a88cba: f30f1035b643f401          movss xmm6, dword ptr [rip + 0x1f443b6]
140a88cc2: e9ee010000                jmp 0x140a88eb5
140a88cc7: 488d5c2470                lea rbx, [rsp + 0x70]
140a88ccc: 4c8db42480000000          lea r14, [rsp + 0x80]
140a88cd4: 488dbc2490000000          lea rdi, [rsp + 0x90]
140a88cdc: c744242000000243          mov dword ptr [rsp + 0x20], 0x43020000
140a88ce4: 488d4c2460                lea rcx, [rsp + 0x60]
140a88ce9: f3440f100d6629ff01        movss xmm9, dword ptr [rip + 0x1ff2966]
140a88cf2: 410f28c8                  movaps xmm1, xmm8
140a88cf6: 0f28d7                    movaps xmm2, xmm7
140a88cf9: 410f28d9                  movaps xmm3, xmm9
140a88cfd: e8eec0f8ff                call 0x140a14df0
140a88d02: 440f28d0                  movaps xmm10, xmm0
140a88d06: c744242000000243          mov dword ptr [rsp + 0x20], 0x43020000
140a88d0e: 4889d9                    mov rcx, rbx
140a88d11: 410f28c8                  movaps xmm1, xmm8
140a88d15: 0f28d7                    movaps xmm2, xmm7
140a88d18: 410f28d9                  movaps xmm3, xmm9
140a88d1c: e8cfc0f8ff                call 0x140a14df0
140a88d21: 440f28d8                  movaps xmm11, xmm0
140a88d25: f3450f58da                addss xmm11, xmm10
140a88d2a: c744242000000243          mov dword ptr [rsp + 0x20], 0x43020000
140a88d32: 4c89f1                    mov rcx, r14
140a88d35: 410f28c8                  movaps xmm1, xmm8
140a88d39: 0f28d7                    movaps xmm2, xmm7
140a88d3c: 410f28d9                  movaps xmm3, xmm9
140a88d40: e8abc0f8ff                call 0x140a14df0
140a88d45: 440f28d0                  movaps xmm10, xmm0
140a88d49: f3450f58d3                addss xmm10, xmm11
140a88d4e: c744242000000243          mov dword ptr [rsp + 0x20], 0x43020000
140a88d56: 4889f9                    mov rcx, rdi
140a88d59: 410f28c8                  movaps xmm1, xmm8
140a88d5d: 0f28d7                    movaps xmm2, xmm7
140a88d60: 410f28d9                  movaps xmm3, xmm9
140a88d64: e887c0f8ff                call 0x140a14df0
140a88d69: f3410f58c2                addss xmm0, xmm10
140a88d6e: 0f57c9                    xorps xmm1, xmm1
140a88d71: 0f57db                    xorps xmm3, xmm3
140a88d74: f30f5fd8                  maxss xmm3, xmm0
140a88d78: f30f101584cbe901          movss xmm2, dword ptr [rip + 0x1e9cb84]
140a88d80: 0f28c2                    movaps xmm0, xmm2
140a88d83: f30f5dc3                  minss xmm0, xmm3
140a88d87: 0f2e05f6cbe901            ucomiss xmm0, dword ptr [rip + 0x1e9cbf6]
140a88d8e: 763e                      jbe 0x140a88dce
140a88d90: c744242000000243          mov dword ptr [rsp + 0x20], 0x43020000
140a88d98: 488d4c2460                lea rcx, [rsp + 0x60]
140a88d9d: f30f101db328ff01          movss xmm3, dword ptr [rip + 0x1ff28b3]
140a88da5: 410f28c8                  movaps xmm1, xmm8
140a88da9: 0f28d7                    movaps xmm2, xmm7
140a88dac: e83fc0f8ff                call 0x140a14df0
140a88db1: 31c0                      xor eax, eax
140a88db3: 0f2e05cacbe901            ucomiss xmm0, dword ptr [rip + 0x1e9cbca]
140a88dba: 0f97c0                    seta al
140a88dbd: 488d0d5cfc0402            lea rcx, [rip + 0x204fc5c]
140a88dc4: f30f103481                movss xmm6, dword ptr [rcx + rax*4]
140a88dc9: e9e7000000                jmp 0x140a88eb5
140a88dce: 410f28d8                  movaps xmm3, xmm8
140a88dd2: f3410f5ed9                divss xmm3, xmm9
140a88dd7: 0f28c7                    movaps xmm0, xmm7
140a88dda: f3410f5ec1                divss xmm0, xmm9
140a88ddf: f30f102589cae901          movss xmm4, dword ptr [rip + 0x1e9ca89]
140a88de7: f30f58dc                  addss xmm3, xmm4
140a88deb: f30f58c4                  addss xmm0, xmm4
140a88def: f30f5fd9                  maxss xmm3, xmm1
140a88df3: f30f5fc1                  maxss xmm0, xmm1
140a88df7: f30f5dda                  minss xmm3, xmm2
140a88dfb: f30f5dc2                  minss xmm0, xmm2
140a88dff: f30f1015a9aef101          movss xmm2, dword ptr [rip + 0x1f1aea9]
140a88e07: f30f59da                  mulss xmm3, xmm2
140a88e0b: f3480f2cc3                cvttss2si rax, xmm3
140a88e10: 31d2                      xor edx, edx
140a88e12: 0f2ed9                    ucomiss xmm3, xmm1
140a88e15: 0f42c2                    cmovb eax, edx
140a88e18: f30f59c2                  mulss xmm0, xmm2
140a88e1c: f30f1015ac79f101          movss xmm2, dword ptr [rip + 0x1f179ac]
140a88e24: 0f2eda                    ucomiss xmm3, xmm2
140a88e27: 41b8ffffffff              mov r8d, 0xffffffff
140a88e2d: 410f47c0                  cmova eax, r8d
140a88e31: f3480f2cc8                cvttss2si rcx, xmm0
140a88e36: 0f2ec1                    ucomiss xmm0, xmm1
140a88e39: 0f42ca                    cmovb ecx, edx
140a88e3c: 0f2ec2                    ucomiss xmm0, xmm2
140a88e3f: 410f47c8                  cmova ecx, r8d
140a88e43: 3dff010000                cmp eax, 0x1ff
140a88e48: baff010000                mov edx, 0x1ff
140a88e4d: 0f43c2                    cmovae eax, edx
140a88e50: 81f9ff010000              cmp ecx, 0x1ff
140a88e56: 0f43ca                    cmovae ecx, edx
140a88e59: c1e109                    shl ecx, 9
140a88e5c: 09c1                      or ecx, eax
140a88e5e: c1e905                    shr ecx, 5
140a88e61: 488b542448                mov rdx, qword ptr [rsp + 0x48]
140a88e66: 4839ca                    cmp rdx, rcx
140a88e69: 0f8619010000              jbe 0x140a88f88
140a88e6f: 488b542440                mov rdx, qword ptr [rsp + 0x40]
140a88e74: 8b0c8a                    mov ecx, dword ptr [rdx + rcx*4]
140a88e77: 0fa3c1                    bt ecx, eax
140a88e7a: 7239                      jb 0x140a88eb5
140a88e7c: c744242000000243          mov dword ptr [rsp + 0x20], 0x43020000
140a88e84: 488d4c2430                lea rcx, [rsp + 0x30]
140a88e89: f30f101dc727ff01          movss xmm3, dword ptr [rip + 0x1ff27c7]
140a88e91: 410f28c8                  movaps xmm1, xmm8
140a88e95: 0f28d7                    movaps xmm2, xmm7
140a88e98: e853bff8ff                call 0x140a14df0
140a88e9d: f30f100dcbcae901          movss xmm1, dword ptr [rip + 0x1e9cacb]
140a88ea5: f30fc2c801                cmpltss xmm1, xmm0
140a88eaa: f30f1035e2b7ea01          movss xmm6, dword ptr [rip + 0x1eab7e2]
140a88eb2: 0f54f1                    andps xmm6, xmm1
140a88eb5: 498b0f                    mov rcx, qword ptr [r15]
140a88eb8: 4885c9                    test rcx, rcx
140a88ebb: 7419                      je 0x140a88ed6
140a88ebd: 4883c110                  add rcx, 0x10
140a88ec1: 488d1510000502            lea rdx, [rip + 0x2050010]
140a88ec8: 41b80a000000              mov r8d, 0xa
140a88ece: 0f28de                    movaps xmm3, xmm6
140a88ed1: e8daaefcff                call 0x140a53db0
140a88ed6: 4084ed                    test bpl, bpl
140a88ed9: 745d                      je 0x140a88f38
140a88edb: 418b44241c                mov eax, dword ptr [r12 + 0x1c]
140a88ee0: 498b4c2410                mov rcx, qword ptr [r12 + 0x10]
140a88ee5: 8901                      mov dword ptr [rcx], eax
140a88ee7: 498b0c24                  mov rcx, qword ptr [r12]
140a88eeb: e84005fdff                call 0x140a59430
140a88ef0: 0f28b424b0000000          movaps xmm6, xmmword ptr [rsp + 0xb0]
140a88ef8: 0f28bc24c0000000          movaps xmm7, xmmword ptr [rsp + 0xc0]
140a88f00: 440f288424d0000000        movaps xmm8, xmmword ptr [rsp + 0xd0]
140a88f09: 440f288c24e0000000        movaps xmm9, xmmword ptr [rsp + 0xe0]
140a88f12: 440f289424f0000000        movaps xmm10, xmmword ptr [rsp + 0xf0]
140a88f1b: 440f289c2400010000        movaps xmm11, xmmword ptr [rsp + 0x100]
140a88f24: 4881c418010000            add rsp, 0x118
140a88f2b: 5b                        pop rbx
140a88f2c: 5d                        pop rbp
140a88f2d: 5f                        pop rdi
140a88f2e: 5e                        pop rsi
140a88f2f: 415c                      pop r12
140a88f31: 415d                      pop r13
140a88f33: 415e                      pop r14
140a88f35: 415f                      pop r15
140a88f37: c3                        ret 
140a88f38: 418b44241c                mov eax, dword ptr [r12 + 0x1c]
140a88f3d: 498b4c2410                mov rcx, qword ptr [r12 + 0x10]
140a88f42: 8901                      mov dword ptr [rcx], eax
140a88f44: 498b0c24                  mov rcx, qword ptr [r12]
140a88f48: e86307fdff                call 0x140a596b0
140a88f4d: eba1                      jmp 0x140a88ef0
140a88f4f: 48c744246000000000        mov qword ptr [rsp + 0x60], 0
140a88f58: 4883c610                  add rsi, 0x10
140a88f5c: 488d051d521a02            lea rax, [rip + 0x21a521d]
140a88f63: 4889442420                mov qword ptr [rsp + 0x20], rax
140a88f68: 488d542430                lea rdx, [rsp + 0x30]
140a88f6d: 4c8d4c2460                lea r9, [rsp + 0x60]
140a88f72: 31c9                      xor ecx, ecx
140a88f74: 4989f0                    mov r8, rsi
140a88f77: e8c51be401                call 0x1428cab41
140a88f7c: 488d0d3dff0402            lea rcx, [rip + 0x204ff3d]
140a88f83: e88803e501                call 0x1428d9310
140a88f88: 4c8d0501c10302            lea r8, [rip + 0x203c101]
140a88f8f: e88405e501                call 0x1428d9518
140a88f94: cc                        int3 
