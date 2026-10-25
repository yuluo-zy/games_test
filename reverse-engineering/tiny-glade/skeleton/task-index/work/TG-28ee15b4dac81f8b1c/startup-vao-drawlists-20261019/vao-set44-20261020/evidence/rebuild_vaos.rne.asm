0000000140864bf0: 55                       push rbp
0000000140864bf1: 4157                     push r15
0000000140864bf3: 4156                     push r14
0000000140864bf5: 4155                     push r13
0000000140864bf7: 4154                     push r12
0000000140864bf9: 56                       push rsi
0000000140864bfa: 57                       push rdi
0000000140864bfb: 53                       push rbx
0000000140864bfc: 4881ec38010000           sub rsp, 0x138
0000000140864c03: 488dac2480000000         lea rbp, [rsp + 0x80]
0000000140864c0b: 660f7fbda0000000         movdqa xmmword ptr [rbp + 0xa0], xmm7
0000000140864c13: 0f29b590000000           movaps xmmword ptr [rbp + 0x90], xmm6
0000000140864c1a: 48c78588000000feffffff   mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
0000000140864c25: 4c8b09                   mov r9, qword ptr [rcx]
0000000140864c28: 498b4110                 mov rax, qword ptr [r9 + 0x10]
0000000140864c2c: 4885c0                   test rax, rax
0000000140864c2f: 0f8444030000             je 0x140864f79
0000000140864c35: 48894dc8                 mov qword ptr [rbp - 0x38], rcx
0000000140864c39: 48c7456800000000         mov qword ptr [rbp + 0x68], 0
0000000140864c41: 48c7457008000000         mov qword ptr [rbp + 0x70], 8
0000000140864c49: 48c7457800000000         mov qword ptr [rbp + 0x78], 0
0000000140864c51: 4c894d50                 mov qword ptr [rbp + 0x50], r9
0000000140864c55: 498b7908                 mov rdi, qword ptr [r9 + 8]
0000000140864c59: 4c8d3cc7                 lea r15, [rdi + rax*8]
0000000140864c5d: 488b1a                   mov rbx, qword ptr [rdx]
0000000140864c60: 488b4210                 mov rax, qword ptr [rdx + 0x10]
0000000140864c64: 488945b8                 mov qword ptr [rbp - 0x48], rax
0000000140864c68: 8b421c                   mov eax, dword ptr [rdx + 0x1c]
0000000140864c6b: 89455c                   mov dword ptr [rbp + 0x5c], eax
0000000140864c6e: 498b00                   mov rax, qword ptr [r8]
0000000140864c71: 488945c0                 mov qword ptr [rbp - 0x40], rax
0000000140864c75: 4c8d7350                 lea r14, [rbx + 0x50]
0000000140864c79: 0f57f6                   xorps xmm6, xmm6
0000000140864c7c: 660f76ff                 pcmpeqd xmm7, xmm7
0000000140864c80: eb17                     jmp 0x140864c99
0000000140864c82: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0000000140864c90: 488d4d60                 lea rcx, [rbp + 0x60]
0000000140864c94: e8977f4e00               call 0x140d4cc30
0000000140864c99: 4c39ff                   cmp rdi, r15
0000000140864c9c: 0f848c020000             je 0x140864f2e
0000000140864ca2: 4989fc                   mov r12, rdi
0000000140864ca5: 4883c708                 add rdi, 8
0000000140864ca9: 498b3424                 mov rsi, qword ptr [r12]
0000000140864cad: 488975d0                 mov qword ptr [rbp - 0x30], rsi
0000000140864cb1: 48837b4800               cmp qword ptr [rbx + 0x48], 0
0000000140864cb6: 0f84a4000000             je 0x140864d60
0000000140864cbc: 4c89f1                   mov rcx, r14
0000000140864cbf: 488d55d0                 lea rdx, [rbp - 0x30]
0000000140864cc3: e8a8c0d4ff               call 0x1405b0d70
0000000140864cc8: 488b4b30                 mov rcx, qword ptr [rbx + 0x30]
0000000140864ccc: 488b5338                 mov rdx, qword ptr [rbx + 0x38]
0000000140864cd0: 4989d0                   mov r8, rdx
0000000140864cd3: 4921c0                   and r8, rax
0000000140864cd6: 48c1e839                 shr rax, 0x39
0000000140864cda: 660f6ec0                 movd xmm0, eax
0000000140864cde: 660f60c0                 punpcklbw xmm0, xmm0
0000000140864ce2: f20f70c000               pshuflw xmm0, xmm0, 0
0000000140864ce7: 660f70c000               pshufd xmm0, xmm0, 0
0000000140864cec: 4c8d69f0                 lea r13, [rcx - 0x10]
0000000140864cf0: 4531c9                   xor r9d, r9d
0000000140864cf3: f3420f6f0c01             movdqu xmm1, xmmword ptr [rcx + r8]
0000000140864cf9: 660f6fd1                 movdqa xmm2, xmm1
0000000140864cfd: 660f74d0                 pcmpeqb xmm2, xmm0
0000000140864d01: 66440fd7d2               pmovmskb r10d, xmm2
0000000140864d06: 4585d2                   test r10d, r10d
0000000140864d09: 7435                     je 0x140864d40
0000000140864d0b: f3450fbcda               tzcnt r11d, r10d
0000000140864d10: 4d01c3                   add r11, r8
0000000140864d13: 4921d3                   and r11, rdx
0000000140864d16: 49c1e304                 shl r11, 4
0000000140864d1a: 4c89e8                   mov rax, r13
0000000140864d1d: 4c29d8                   sub rax, r11
0000000140864d20: 483930                   cmp qword ptr [rax], rsi
0000000140864d23: 0f84e7000000             je 0x140864e10
0000000140864d29: 418d42ff                 lea eax, [r10 - 1]
0000000140864d2d: 664421d0                 and ax, r10w
0000000140864d31: 4189c2                   mov r10d, eax
0000000140864d34: 75d5                     jne 0x140864d0b
0000000140864d36: 662e0f1f840000000000     nop word ptr cs:[rax + rax]
0000000140864d40: 660f74cf                 pcmpeqb xmm1, xmm7
0000000140864d44: 660fd7c1                 pmovmskb eax, xmm1
0000000140864d48: 85c0                     test eax, eax
0000000140864d4a: 7514                     jne 0x140864d60
0000000140864d4c: 4d01c8                   add r8, r9
0000000140864d4f: 4983c010                 add r8, 0x10
0000000140864d53: 4983c110                 add r9, 0x10
0000000140864d57: 4921d0                   and r8, rdx
0000000140864d5a: eb97                     jmp 0x140864cf3
0000000140864d5c: 0f1f4000                 nop dword ptr [rax]
0000000140864d60: 488b054176dc02           mov rax, qword ptr [rip + 0x2dc7641]
0000000140864d67: 488b00                   mov rax, qword ptr [rax]
0000000140864d6a: 4883f802                 cmp rax, 2
0000000140864d6e: 0f8225ffffff             jb 0x140864c99
0000000140864d74: 488d0dfd172302           lea rcx, [rip + 0x22317fd]
0000000140864d7b: e80090e1ff               call 0x14067dd80
0000000140864d80: f30f6f00                 movdqu xmm0, xmmword ptr [rax]
0000000140864d84: 8b4010                   mov eax, dword ptr [rax + 0x10]
0000000140864d87: 48c7450002000000         mov qword ptr [rbp], 2
0000000140864d8f: 488d1522172302           lea rdx, [rip + 0x2231722]
0000000140864d96: 48895508                 mov qword ptr [rbp + 8], rdx
0000000140864d9a: 48c745102a000000         mov qword ptr [rbp + 0x10], 0x2a
0000000140864da2: 488d0dbf172302           lea rcx, [rip + 0x22317bf]
0000000140864da9: 48894d20                 mov qword ptr [rbp + 0x20], rcx
0000000140864dad: 48c7452801000000         mov qword ptr [rbp + 0x28], 1
0000000140864db5: 48c7453008000000         mov qword ptr [rbp + 0x30], 8
0000000140864dbd: 48c745d000000000         mov qword ptr [rbp - 0x30], 0
0000000140864dc5: 488d4d38                 lea rcx, [rbp + 0x38]
0000000140864dc9: 0f1131                   movups xmmword ptr [rcx], xmm6
0000000140864dcc: 488955d8                 mov qword ptr [rbp - 0x28], rdx
0000000140864dd0: 48c745e02a000000         mov qword ptr [rbp - 0x20], 0x2a
0000000140864dd8: 48c745e800000000         mov qword ptr [rbp - 0x18], 0
0000000140864de0: f30f7f45f0               movdqu xmmword ptr [rbp - 0x10], xmm0
0000000140864de5: c7451801000000           mov dword ptr [rbp + 0x18], 1
0000000140864dec: 89451c                   mov dword ptr [rbp + 0x1c], eax
0000000140864def: 488d8d87000000           lea rcx, [rbp + 0x87]
0000000140864df6: 488d55d0                 lea rdx, [rbp - 0x30]
0000000140864dfa: e831554e00               call 0x140d4a330 ; log dispatch
0000000140864dff: e995feffff               jmp 0x140864c99
0000000140864e04: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0000000140864e10: 488b45b8                 mov rax, qword ptr [rbp - 0x48]
0000000140864e14: 8b4d5c                   mov ecx, dword ptr [rbp + 0x5c]
0000000140864e17: 8908                     mov dword ptr [rax], ecx
0000000140864e19: 488b45c0                 mov rax, qword ptr [rbp - 0x40]
0000000140864e1d: 488b10                   mov rdx, qword ptr [rax]
0000000140864e20: 4883c210                 add rdx, 0x10
0000000140864e24: 4d8b0c24                 mov r9, qword ptr [r12]
0000000140864e28: 4889d9                   mov rcx, rbx
0000000140864e2b: 4c8b4550                 mov r8, qword ptr [rbp + 0x50]
0000000140864e2f: e8bcf50b00               call 0x1409243f0 ; AssetVAOLibrary::rebuild_vao
0000000140864e34: 4885c0                   test rax, rax
0000000140864e37: 0f845cfeffff             je 0x140864c99
0000000140864e3d: 48894560                 mov qword ptr [rbp + 0x60], rax
0000000140864e41: 4d8b2424                 mov r12, qword ptr [r12]
0000000140864e45: 488b7578                 mov rsi, qword ptr [rbp + 0x78]
0000000140864e49: 483b7568                 cmp rsi, qword ptr [rbp + 0x68]
0000000140864e4d: 7510                     jne 0x140864e5f
0000000140864e4f: 488d4d68                 lea rcx, [rbp + 0x68]
0000000140864e53: 488d1536172302           lea rdx, [rip + 0x2231736]
0000000140864e5a: e811dacfff               call 0x140562870
0000000140864e5f: 488b4570                 mov rax, qword ptr [rbp + 0x70]
0000000140864e63: 4c8924f0                 mov qword ptr [rax + rsi*8], r12
0000000140864e67: 48ffc6                   inc rsi
0000000140864e6a: 48897578                 mov qword ptr [rbp + 0x78], rsi
0000000140864e6e: 488b053375dc02           mov rax, qword ptr [rip + 0x2dc7533]
0000000140864e75: 488b00                   mov rax, qword ptr [rax]
0000000140864e78: 4883f802                 cmp rax, 2
0000000140864e7c: 0f820efeffff             jb 0x140864c90
0000000140864e82: 488d4560                 lea rax, [rbp + 0x60]
0000000140864e86: 488945a8                 mov qword ptr [rbp - 0x58], rax
0000000140864e8a: 488d057f7d4e00           lea rax, [rip + 0x4e7d7f]
0000000140864e91: 488945b0                 mov qword ptr [rbp - 0x50], rax
0000000140864e95: 488d0d34172302           lea rcx, [rip + 0x2231734]
0000000140864e9c: e8df8ee1ff               call 0x14067dd80
0000000140864ea1: f30f6f00                 movdqu xmm0, xmmword ptr [rax]
0000000140864ea5: 8b4010                   mov eax, dword ptr [rax + 0x10]
0000000140864ea8: 48c7450002000000         mov qword ptr [rbp], 2
0000000140864eb0: 488d0d01162302           lea rcx, [rip + 0x2231601]
0000000140864eb7: 48894d08                 mov qword ptr [rbp + 8], rcx
0000000140864ebb: 48c745102a000000         mov qword ptr [rbp + 0x10], 0x2a
0000000140864ec3: 488d15f6162302           lea rdx, [rip + 0x22316f6]
0000000140864eca: 48895520                 mov qword ptr [rbp + 0x20], rdx
0000000140864ece: 48c7452801000000         mov qword ptr [rbp + 0x28], 1
0000000140864ed6: 488d55a8                 lea rdx, [rbp - 0x58]
0000000140864eda: 48895530                 mov qword ptr [rbp + 0x30], rdx
0000000140864ede: 48c7453801000000         mov qword ptr [rbp + 0x38], 1
0000000140864ee6: 48c7454000000000         mov qword ptr [rbp + 0x40], 0
0000000140864eee: 48c745d000000000         mov qword ptr [rbp - 0x30], 0
0000000140864ef6: 48894dd8                 mov qword ptr [rbp - 0x28], rcx
0000000140864efa: 48c745e02a000000         mov qword ptr [rbp - 0x20], 0x2a
0000000140864f02: 48c745e800000000         mov qword ptr [rbp - 0x18], 0
0000000140864f0a: f30f7f45f0               movdqu xmmword ptr [rbp - 0x10], xmm0
0000000140864f0f: c7451801000000           mov dword ptr [rbp + 0x18], 1
0000000140864f16: 89451c                   mov dword ptr [rbp + 0x1c], eax
0000000140864f19: 488d8d87000000           lea rcx, [rbp + 0x87]
0000000140864f20: 488d55d0                 lea rdx, [rbp - 0x30]
0000000140864f24: e807544e00               call 0x140d4a330 ; log dispatch
0000000140864f29: e962fdffff               jmp 0x140864c90
0000000140864f2e: 488b4578                 mov rax, qword ptr [rbp + 0x78]
0000000140864f32: 488945e0                 mov qword ptr [rbp - 0x20], rax
0000000140864f36: 0f104568                 movups xmm0, xmmword ptr [rbp + 0x68]
0000000140864f3a: 0f2945d0                 movaps xmmword ptr [rbp - 0x30], xmm0
0000000140864f3e: 488b4dc8                 mov rcx, qword ptr [rbp - 0x38]
0000000140864f42: 8b411c                   mov eax, dword ptr [rcx + 0x1c]
0000000140864f45: 488b4910                 mov rcx, qword ptr [rcx + 0x10]
0000000140864f49: 8901                     mov dword ptr [rcx], eax
0000000140864f4b: 488b7550                 mov rsi, qword ptr [rbp + 0x50]
0000000140864f4f: 488b16                   mov rdx, qword ptr [rsi]
0000000140864f52: 4885d2                   test rdx, rdx
0000000140864f55: 7413                     je 0x140864f6a
0000000140864f57: 488b4e08                 mov rcx, qword ptr [rsi + 8]
0000000140864f5b: 48c1e203                 shl rdx, 3
0000000140864f5f: 41b808000000             mov r8d, 8
0000000140864f65: e8b6ecdaff               call 0x140613c20
0000000140864f6a: 488b45e0                 mov rax, qword ptr [rbp - 0x20]
0000000140864f6e: 48894610                 mov qword ptr [rsi + 0x10], rax
0000000140864f72: 0f2845d0                 movaps xmm0, xmmword ptr [rbp - 0x30]
0000000140864f76: 0f1106                   movups xmmword ptr [rsi], xmm0
0000000140864f79: 0f28b590000000           movaps xmm6, xmmword ptr [rbp + 0x90]
0000000140864f80: 0f28bda0000000           movaps xmm7, xmmword ptr [rbp + 0xa0]
0000000140864f87: 4881c438010000           add rsp, 0x138
0000000140864f8e: 5b                       pop rbx
0000000140864f8f: 5f                       pop rdi
0000000140864f90: 5e                       pop rsi
0000000140864f91: 415c                     pop r12
0000000140864f93: 415d                     pop r13
0000000140864f95: 415e                     pop r14
0000000140864f97: 415f                     pop r15
0000000140864f99: 5d                       pop rbp
0000000140864f9a: c3                       ret 
0000000140864f9b: 0f1f440000               nop dword ptr [rax + rax]
0000000140864fa0: 4889542410               mov qword ptr [rsp + 0x10], rdx
0000000140864fa5: 55                       push rbp
0000000140864fa6: 4157                     push r15
0000000140864fa8: 4156                     push r14
0000000140864faa: 4155                     push r13
0000000140864fac: 4154                     push r12
0000000140864fae: 56                       push rsi
0000000140864faf: 57                       push rdi
0000000140864fb0: 53                       push rbx
0000000140864fb1: 4883ec48                 sub rsp, 0x48
0000000140864fb5: 488daa80000000           lea rbp, [rdx + 0x80]
0000000140864fbc: 660f7f7c2420             movdqa xmmword ptr [rsp + 0x20], xmm7
0000000140864fc2: 0f29742430               movaps xmmword ptr [rsp + 0x30], xmm6
0000000140864fc7: 488b5568                 mov rdx, qword ptr [rbp + 0x68]
0000000140864fcb: 4885d2                   test rdx, rdx
0000000140864fce: 7413                     je 0x140864fe3
0000000140864fd0: 488b4d70                 mov rcx, qword ptr [rbp + 0x70]
0000000140864fd4: 48c1e203                 shl rdx, 3
0000000140864fd8: 41b808000000             mov r8d, 8
0000000140864fde: e83decdaff               call 0x140613c20
0000000140864fe3: 0f28742430               movaps xmm6, xmmword ptr [rsp + 0x30]
0000000140864fe8: 0f287c2420               movaps xmm7, xmmword ptr [rsp + 0x20]
0000000140864fed: 4883c448                 add rsp, 0x48
0000000140864ff1: 5b                       pop rbx
0000000140864ff2: 5f                       pop rdi
0000000140864ff3: 5e                       pop rsi
0000000140864ff4: 415c                     pop r12
0000000140864ff6: 415d                     pop r13
0000000140864ff8: 415e                     pop r14
0000000140864ffa: 415f                     pop r15
0000000140864ffc: 5d                       pop rbp
0000000140864ffd: c3                       ret 
0000000140864ffe: 6690                     nop 
0000000140865000: 4889542410               mov qword ptr [rsp + 0x10], rdx
0000000140865005: 55                       push rbp
0000000140865006: 4157                     push r15
0000000140865008: 4156                     push r14
000000014086500a: 4155                     push r13
000000014086500c: 4154                     push r12
000000014086500e: 56                       push rsi
000000014086500f: 57                       push rdi
0000000140865010: 53                       push rbx
0000000140865011: 4883ec48                 sub rsp, 0x48
0000000140865015: 488daa80000000           lea rbp, [rdx + 0x80]
000000014086501c: 660f7f7c2420             movdqa xmmword ptr [rsp + 0x20], xmm7
0000000140865022: 0f29742430               movaps xmmword ptr [rsp + 0x30], xmm6
0000000140865027: 488d4d60                 lea rcx, [rbp + 0x60]
000000014086502b: e8007c4e00               call 0x140d4cc30
0000000140865030: 0f28742430               movaps xmm6, xmmword ptr [rsp + 0x30]
0000000140865035: 0f287c2420               movaps xmm7, xmmword ptr [rsp + 0x20]
000000014086503a: 4883c448                 add rsp, 0x48
000000014086503e: 5b                       pop rbx
000000014086503f: 5f                       pop rdi
0000000140865040: 5e                       pop rsi
0000000140865041: 415c                     pop r12
0000000140865043: 415d                     pop r13
0000000140865045: 415e                     pop r14
0000000140865047: 415f                     pop r15
0000000140865049: 5d                       pop rbp
000000014086504a: c3                       ret 
