0000000140841b00: 55                             push rbp
0000000140841b01: 4157                           push r15
0000000140841b03: 4156                           push r14
0000000140841b05: 4155                           push r13
0000000140841b07: 4154                           push r12
0000000140841b09: 56                             push rsi
0000000140841b0a: 57                             push rdi
0000000140841b0b: 53                             push rbx
0000000140841b0c: 4881ec38010000                 sub rsp, 0x138
0000000140841b13: 488dac2480000000               lea rbp, [rsp + 0x80]
0000000140841b1b: 0f29b5a0000000                 movaps xmmword ptr [rbp + 0xa0], xmm6
0000000140841b22: 48c78598000000feffffff         mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
0000000140841b2d: 48894d70                       mov qword ptr [rbp + 0x70], rcx
0000000140841b31: 4c894558                       mov qword ptr [rbp + 0x58], r8
0000000140841b35: 4c894d60                       mov qword ptr [rbp + 0x60], r9
0000000140841b39: 4889d1                         mov rcx, rdx
0000000140841b3c: 4c89c2                         mov rdx, r8
0000000140841b3f: 4d89c8                         mov r8, r9
0000000140841b42: e889671f00                     call 0x140a382d0
0000000140841b47: 4889c7                         mov rdi, rax
0000000140841b4a: 4889d3                         mov rbx, rdx
0000000140841b4d: 48c7457800000000               mov qword ptr [rbp + 0x78], 0
0000000140841b55: 48c7858000000004000000         mov qword ptr [rbp + 0x80], 4
0000000140841b60: 48c7858800000000000000         mov qword ptr [rbp + 0x88], 0
0000000140841b6b: 488b0536a8de02                 mov rax, qword ptr [rip + 0x2dea836]
0000000140841b72: 488b00                         mov rax, qword ptr [rax]
0000000140841b75: 4883f804                       cmp rax, 4
0000000140841b79: 0f86bc000000                   jbe 0x140841c3b
0000000140841b7f: 48895d68                       mov qword ptr [rbp + 0x68], rbx
0000000140841b83: 488d4568                       lea rax, [rbp + 0x68]
0000000140841b87: 488945b8                       mov qword ptr [rbp - 0x48], rax
0000000140841b8b: 488d052ebb5400                 lea rax, [rip + 0x54bb2e]
0000000140841b92: 488945c0                       mov qword ptr [rbp - 0x40], rax
0000000140841b96: 488d4558                       lea rax, [rbp + 0x58]
0000000140841b9a: 488945c8                       mov qword ptr [rbp - 0x38], rax
0000000140841b9e: 488d051b7cd100                 lea rax, [rip + 0xd17c1b]
0000000140841ba5: 488945d0                       mov qword ptr [rbp - 0x30], rax
0000000140841ba9: 488d0d60272502                 lea rcx, [rip + 0x2252760]
0000000140841bb0: e8cbc1e3ff                     call 0x14067dd80
0000000140841bb5: 0f1000                         movups xmm0, xmmword ptr [rax]
0000000140841bb8: 8b4010                         mov eax, dword ptr [rax + 0x10]
0000000140841bbb: 48c7450805000000               mov qword ptr [rbp + 8], 5
0000000140841bc3: 488d0d56262502                 lea rcx, [rip + 0x2252656]
0000000140841bca: 48894d10                       mov qword ptr [rbp + 0x10], rcx
0000000140841bce: 48c745181d000000               mov qword ptr [rbp + 0x18], 0x1d
0000000140841bd6: 488d1513272502                 lea rdx, [rip + 0x2252713]
0000000140841bdd: 48895528                       mov qword ptr [rbp + 0x28], rdx
0000000140841be1: 48c7453002000000               mov qword ptr [rbp + 0x30], 2
0000000140841be9: 488d55b8                       lea rdx, [rbp - 0x48]
0000000140841bed: 48895538                       mov qword ptr [rbp + 0x38], rdx
0000000140841bf1: 48c7454002000000               mov qword ptr [rbp + 0x40], 2
0000000140841bf9: 48c7454800000000               mov qword ptr [rbp + 0x48], 0
0000000140841c01: 48c745d800000000               mov qword ptr [rbp - 0x28], 0
0000000140841c09: 48894de0                       mov qword ptr [rbp - 0x20], rcx
0000000140841c0d: 48c745e81d000000               mov qword ptr [rbp - 0x18], 0x1d
0000000140841c15: 48c745f000000000               mov qword ptr [rbp - 0x10], 0
0000000140841c1d: 0f1145f8                       movups xmmword ptr [rbp - 8], xmm0
0000000140841c21: c7452001000000                 mov dword ptr [rbp + 0x20], 1
0000000140841c28: 894524                         mov dword ptr [rbp + 0x24], eax
0000000140841c2b: 488d8d97000000                 lea rcx, [rbp + 0x97]
0000000140841c32: 488d55d8                       lea rdx, [rbp - 0x28]
0000000140841c36: e8f5865000                     call 0x140d4a330
0000000140841c3b: 488b8528010000                 mov rax, qword ptr [rbp + 0x128]
0000000140841c42: 4c8bbd20010000                 mov r15, qword ptr [rbp + 0x120]
0000000140841c49: 4c8d24df                       lea r12, [rdi + rbx*8]
0000000140841c4d: f3410f10770c                   movss xmm6, dword ptr [r15 + 0xc]
0000000140841c53: 448b681c                       mov r13d, dword ptr [rax + 0x1c]
0000000140841c57: 488b18                         mov rbx, qword ptr [rax]
0000000140841c5a: 488b7010                       mov rsi, qword ptr [rax + 0x10]
0000000140841c5e: 4c39e7                         cmp rdi, r12
0000000140841c61: 7468                           je 0x140841ccb
0000000140841c63: 666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
0000000140841c70: 448b37                         mov r14d, dword ptr [rdi]
0000000140841c73: 0f28c6                         movaps xmm0, xmm6
0000000140841c76: e8a5820700                     call 0x1408b9f20
0000000140841c7b: 44892e                         mov dword ptr [rsi], r13d
0000000140841c7e: 0fb65704                       movzx edx, byte ptr [rdi + 4]
0000000140841c82: 418b4f08                       mov ecx, dword ptr [r15 + 8]
0000000140841c86: 894de0                         mov dword ptr [rbp - 0x20], ecx
0000000140841c89: 498b0f                         mov rcx, qword ptr [r15]
0000000140841c8c: 48894dd8                       mov qword ptr [rbp - 0x28], rcx
0000000140841c90: 668945e4                       mov word ptr [rbp - 0x1c], ax
0000000140841c94: 66448975e6                     mov word ptr [rbp - 0x1a], r14w
0000000140841c99: 488d4578                       lea rax, [rbp + 0x78]
0000000140841c9d: 4889442430                     mov qword ptr [rsp + 0x30], rax
0000000140841ca2: 4c897c2420                     mov qword ptr [rsp + 0x20], r15
0000000140841ca7: 48c744242801000000             mov qword ptr [rsp + 0x28], 1
0000000140841cb0: 41b901000000                   mov r9d, 1
0000000140841cb6: 4889d9                         mov rcx, rbx
0000000140841cb9: 4c8d45d8                       lea r8, [rbp - 0x28]
0000000140841cbd: e80eb4d200                     call 0x14156d0d0
0000000140841cc2: 4883c708                       add rdi, 8
0000000140841cc6: 4c39e7                         cmp rdi, r12
0000000140841cc9: 75a5                           jne 0x140841c70
0000000140841ccb: 488b8588000000                 mov rax, qword ptr [rbp + 0x88]
0000000140841cd2: 488b4d70                       mov rcx, qword ptr [rbp + 0x70]
0000000140841cd6: 48894110                       mov qword ptr [rcx + 0x10], rax
0000000140841cda: 0f104578                       movups xmm0, xmmword ptr [rbp + 0x78]
0000000140841cde: 0f1101                         movups xmmword ptr [rcx], xmm0
0000000140841ce1: 0f28b5a0000000                 movaps xmm6, xmmword ptr [rbp + 0xa0]
0000000140841ce8: 4881c438010000                 add rsp, 0x138
0000000140841cef: 5b                             pop rbx
0000000140841cf0: 5f                             pop rdi
0000000140841cf1: 5e                             pop rsi
0000000140841cf2: 415c                           pop r12
0000000140841cf4: 415d                           pop r13
0000000140841cf6: 415e                           pop r14
0000000140841cf8: 415f                           pop r15
0000000140841cfa: 5d                             pop rbp
0000000140841cfb: c3                             ret 
0000000140841cfc: 0f1f4000                       nop dword ptr [rax]
0000000140841d00: 4889542410                     mov qword ptr [rsp + 0x10], rdx
0000000140841d05: 55                             push rbp
0000000140841d06: 4157                           push r15
0000000140841d08: 4156                           push r14
0000000140841d0a: 4155                           push r13
0000000140841d0c: 4154                           push r12
0000000140841d0e: 56                             push rsi
0000000140841d0f: 57                             push rdi
0000000140841d10: 53                             push rbx
0000000140841d11: 4883ec48                       sub rsp, 0x48
0000000140841d15: 488daa80000000                 lea rbp, [rdx + 0x80]
0000000140841d1c: 0f29742430                     movaps xmmword ptr [rsp + 0x30], xmm6
0000000140841d21: 488b4578                       mov rax, qword ptr [rbp + 0x78]
0000000140841d25: 4885c0                         test rax, rax
0000000140841d28: 741a                           je 0x140841d44
0000000140841d2a: 488b8d80000000                 mov rcx, qword ptr [rbp + 0x80]
0000000140841d31: 48c1e002                       shl rax, 2
0000000140841d35: 488d1440                       lea rdx, [rax + rax*2]
0000000140841d39: 41b804000000                   mov r8d, 4
0000000140841d3f: e8dc1eddff                     call 0x140613c20
0000000140841d44: 0f28742430                     movaps xmm6, xmmword ptr [rsp + 0x30]
0000000140841d49: 4883c448                       add rsp, 0x48
0000000140841d4d: 5b                             pop rbx
0000000140841d4e: 5f                             pop rdi
0000000140841d4f: 5e                             pop rsi
0000000140841d50: 415c                           pop r12
0000000140841d52: 415d                           pop r13
0000000140841d54: 415e                           pop r14
0000000140841d56: 415f                           pop r15
0000000140841d58: 5d                             pop rbp
0000000140841d59: c3                             ret 
