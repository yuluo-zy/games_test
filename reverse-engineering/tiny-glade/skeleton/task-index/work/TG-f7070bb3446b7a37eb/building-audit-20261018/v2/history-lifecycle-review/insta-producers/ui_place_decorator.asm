1407a9c10: 55                           push rbp
1407a9c11: 4157                         push r15
1407a9c13: 4156                         push r14
1407a9c15: 4155                         push r13
1407a9c17: 4154                         push r12
1407a9c19: 56                           push rsi
1407a9c1a: 57                           push rdi
1407a9c1b: 53                           push rbx
1407a9c1c: 4881ec28080000               sub rsp, 0x828
1407a9c23: 488dac2480000000             lea rbp, [rsp + 0x80]
1407a9c2b: 440f29bd90070000             movaps xmmword ptr [rbp + 0x790], xmm15
1407a9c33: 440f29b580070000             movaps xmmword ptr [rbp + 0x780], xmm14
1407a9c3b: 440f29ad70070000             movaps xmmword ptr [rbp + 0x770], xmm13
1407a9c43: 440f29a560070000             movaps xmmword ptr [rbp + 0x760], xmm12
1407a9c4b: 440f299d50070000             movaps xmmword ptr [rbp + 0x750], xmm11
1407a9c53: 440f299540070000             movaps xmmword ptr [rbp + 0x740], xmm10
1407a9c5b: 440f298d30070000             movaps xmmword ptr [rbp + 0x730], xmm9
1407a9c63: 440f298520070000             movaps xmmword ptr [rbp + 0x720], xmm8
1407a9c6b: 0f29bd10070000               movaps xmmword ptr [rbp + 0x710], xmm7
1407a9c72: 0f29b500070000               movaps xmmword ptr [rbp + 0x700], xmm6
1407a9c79: 48c785f8060000feffffff       mov qword ptr [rbp + 0x6f8], 0xfffffffffffffffe
1407a9c84: 4c89ce                       mov rsi, r9
1407a9c87: 48899568040000               mov qword ptr [rbp + 0x468], rdx
1407a9c8e: 488b01                       mov rax, qword ptr [rcx]
1407a9c91: 488985c0060000               mov qword ptr [rbp + 0x6c0], rax
1407a9c98: 488b4110                     mov rax, qword ptr [rcx + 0x10]
1407a9c9c: 488985a8050000               mov qword ptr [rbp + 0x5a8], rax
1407a9ca3: 8b411c                       mov eax, dword ptr [rcx + 0x1c]
1407a9ca6: 8985ac060000                 mov dword ptr [rbp + 0x6ac], eax
1407a9cac: 488b4120                     mov rax, qword ptr [rcx + 0x20]
1407a9cb0: 48898510050000               mov qword ptr [rbp + 0x510], rax
1407a9cb7: 488b4130                     mov rax, qword ptr [rcx + 0x30]
1407a9cbb: 48898520050000               mov qword ptr [rbp + 0x520], rax
1407a9cc2: 8b413c                       mov eax, dword ptr [rcx + 0x3c]
1407a9cc5: 8985a8060000                 mov dword ptr [rbp + 0x6a8], eax
1407a9ccb: 488b4140                     mov rax, qword ptr [rcx + 0x40]
1407a9ccf: 48898518050000               mov qword ptr [rbp + 0x518], rax
1407a9cd6: 0f104160                     movups xmm0, xmmword ptr [rcx + 0x60]
1407a9cda: 0f104970                     movups xmm1, xmmword ptr [rcx + 0x70]
1407a9cde: 0f298d10030000               movaps xmmword ptr [rbp + 0x310], xmm1
1407a9ce5: 0f298500030000               movaps xmmword ptr [rbp + 0x300], xmm0
1407a9cec: 0f108180000000               movups xmm0, xmmword ptr [rcx + 0x80]
1407a9cf3: 0f108990000000               movups xmm1, xmmword ptr [rcx + 0x90]
1407a9cfa: 0f1091a0000000               movups xmm2, xmmword ptr [rcx + 0xa0]
1407a9d01: 0f1099b0000000               movups xmm3, xmmword ptr [rcx + 0xb0]
1407a9d08: 0f299d00010000               movaps xmmword ptr [rbp + 0x100], xmm3
1407a9d0f: 0f2995f0000000               movaps xmmword ptr [rbp + 0xf0], xmm2
1407a9d16: 0f298de0000000               movaps xmmword ptr [rbp + 0xe0], xmm1
1407a9d1d: 0f2985d0000000               movaps xmmword ptr [rbp + 0xd0], xmm0
1407a9d24: 0f1081c0000000               movups xmm0, xmmword ptr [rcx + 0xc0]
1407a9d2b: 0f1089d0000000               movups xmm1, xmmword ptr [rcx + 0xd0]
1407a9d32: f30f6f91e0000000             movdqu xmm2, xmmword ptr [rcx + 0xe0]
1407a9d3a: 0f1099f0000000               movups xmm3, xmmword ptr [rcx + 0xf0]
1407a9d41: 0f299df0010000               movaps xmmword ptr [rbp + 0x1f0], xmm3
1407a9d48: 660f7f95e0010000             movdqa xmmword ptr [rbp + 0x1e0], xmm2
1407a9d50: 0f298dd0010000               movaps xmmword ptr [rbp + 0x1d0], xmm1
1407a9d57: 0f2985c0010000               movaps xmmword ptr [rbp + 0x1c0], xmm0
1407a9d5e: 4c8ba900010000               mov r13, qword ptr [rcx + 0x100]
1407a9d65: 498b4030                     mov rax, qword ptr [r8 + 0x30]
1407a9d69: 488985a0010000               mov qword ptr [rbp + 0x1a0], rax
1407a9d70: 488b8110010000               mov rax, qword ptr [rcx + 0x110]
1407a9d77: 488985c0030000               mov qword ptr [rbp + 0x3c0], rax
1407a9d7e: 0f1081a0010000               movups xmm0, xmmword ptr [rcx + 0x1a0]
1407a9d85: 0f1089b0010000               movups xmm1, xmmword ptr [rcx + 0x1b0]
1407a9d8c: 0f298dc0050000               movaps xmmword ptr [rbp + 0x5c0], xmm1
1407a9d93: 0f2985b0050000               movaps xmmword ptr [rbp + 0x5b0], xmm0
1407a9d9a: 448bb11c010000               mov r14d, dword ptr [rcx + 0x11c]
1407a9da1: 488b8120010000               mov rax, qword ptr [rcx + 0x120]
1407a9da8: 48898500050000               mov qword ptr [rbp + 0x500], rax
1407a9daf: 488b8130010000               mov rax, qword ptr [rcx + 0x130]
1407a9db6: 488985a0040000               mov qword ptr [rbp + 0x4a0], rax
1407a9dbd: 8b813c010000                 mov eax, dword ptr [rcx + 0x13c]
1407a9dc3: 898584060000                 mov dword ptr [rbp + 0x684], eax
1407a9dc9: 488b8140010000               mov rax, qword ptr [rcx + 0x140]
1407a9dd0: 488985e8060000               mov qword ptr [rbp + 0x6e8], rax
1407a9dd7: 488b8160010000               mov rax, qword ptr [rcx + 0x160]
1407a9dde: 48898590040000               mov qword ptr [rbp + 0x490], rax
1407a9de5: 488b8170010000               mov rax, qword ptr [rcx + 0x170]
1407a9dec: 48898598040000               mov qword ptr [rbp + 0x498], rax
1407a9df3: 8b817c010000                 mov eax, dword ptr [rcx + 0x17c]
1407a9df9: 898580060000                 mov dword ptr [rbp + 0x680], eax
1407a9dff: 488b8180010000               mov rax, qword ptr [rcx + 0x180]
1407a9e06: 488985a0060000               mov qword ptr [rbp + 0x6a0], rax
1407a9e0d: 488b81c0010000               mov rax, qword ptr [rcx + 0x1c0]
1407a9e14: 488985d8060000               mov qword ptr [rbp + 0x6d8], rax
1407a9e1b: 488b99c8010000               mov rbx, qword ptr [rcx + 0x1c8]
1407a9e22: 410f104020                   movups xmm0, xmmword ptr [r8 + 0x20]
1407a9e27: 0f298590010000               movaps xmmword ptr [rbp + 0x190], xmm0
1407a9e2e: 498b00                       mov rax, qword ptr [r8]
1407a9e31: 488985b8040000               mov qword ptr [rbp + 0x4b8], rax
1407a9e38: 498b4038                     mov rax, qword ptr [r8 + 0x38]
1407a9e3c: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
1407a9e43: 498b4058                     mov rax, qword ptr [r8 + 0x58]
1407a9e47: 48898578020000               mov qword ptr [rbp + 0x278], rax
1407a9e4e: 498b4078                     mov rax, qword ptr [r8 + 0x78]
1407a9e52: 48898560040000               mov qword ptr [rbp + 0x460], rax
1407a9e59: 498b8098000000               mov rax, qword ptr [r8 + 0x98]
1407a9e60: 488985b8030000               mov qword ptr [rbp + 0x3b8], rax
1407a9e67: 488bbd20080000               mov rdi, qword ptr [rbp + 0x820]
1407a9e6e: 4c8bbd30080000               mov r15, qword ptr [rbp + 0x830]
1407a9e75: 4d8b21                       mov r12, qword ptr [r9]
1407a9e78: 498b4120                     mov rax, qword ptr [r9 + 0x20]
1407a9e7c: 488985d8030000               mov qword ptr [rbp + 0x3d8], rax
1407a9e83: 498b4140                     mov rax, qword ptr [r9 + 0x40]
1407a9e87: 48898598060000               mov qword ptr [rbp + 0x698], rax
1407a9e8e: 498b4160                     mov rax, qword ptr [r9 + 0x60]
1407a9e92: 48898580020000               mov qword ptr [rbp + 0x280], rax
1407a9e99: 498d9180000000               lea rdx, [r9 + 0x80]
1407a9ea0: 488d4d30                     lea rcx, [rbp + 0x30]
1407a9ea4: 41b8a0000000                 mov r8d, 0xa0
1407a9eaa: e8519d1702                   call 0x142923c00
1407a9eaf: 488b8620010000               mov rax, qword ptr [rsi + 0x120]
1407a9eb6: 488985b0040000               mov qword ptr [rbp + 0x4b0], rax
1407a9ebd: 488b8640010000               mov rax, qword ptr [rsi + 0x140]
1407a9ec4: 488985a8040000               mov qword ptr [rbp + 0x4a8], rax
1407a9ecb: 0f104720                     movups xmm0, xmmword ptr [rdi + 0x20]
1407a9ecf: 0f104f30                     movups xmm1, xmmword ptr [rdi + 0x30]
1407a9ed3: 0f298d80010000               movaps xmmword ptr [rbp + 0x180], xmm1
1407a9eda: 0f298570010000               movaps xmmword ptr [rbp + 0x170], xmm0
1407a9ee1: 0f104740                     movups xmm0, xmmword ptr [rdi + 0x40]
1407a9ee5: 0f104f50                     movups xmm1, xmmword ptr [rdi + 0x50]
1407a9ee9: 0f298d20020000               movaps xmmword ptr [rbp + 0x220], xmm1
1407a9ef0: 0f298510020000               movaps xmmword ptr [rbp + 0x210], xmm0
1407a9ef7: 488b4760                     mov rax, qword ptr [rdi + 0x60]
1407a9efb: 488985e0060000               mov qword ptr [rbp + 0x6e0], rax
1407a9f02: 488b7768                     mov rsi, qword ptr [rdi + 0x68]
1407a9f06: 488b8788000000               mov rax, qword ptr [rdi + 0x88]
1407a9f0d: 488985f8040000               mov qword ptr [rbp + 0x4f8], rax
1407a9f14: 488b8790000000               mov rax, qword ptr [rdi + 0x90]
1407a9f1b: 48898508050000               mov qword ptr [rbp + 0x508], rax
1407a9f22: 0f1087b0000000               movups xmm0, xmmword ptr [rdi + 0xb0]
1407a9f29: 0f108fc0000000               movups xmm1, xmmword ptr [rdi + 0xc0]
1407a9f30: 0f298d60010000               movaps xmmword ptr [rbp + 0x160], xmm1
1407a9f37: 0f298550010000               movaps xmmword ptr [rbp + 0x150], xmm0
1407a9f3e: 498b07                       mov rax, qword ptr [r15]
1407a9f41: 488985d0030000               mov qword ptr [rbp + 0x3d0], rax
1407a9f48: 410f104708                   movups xmm0, xmmword ptr [r15 + 8]
1407a9f4d: 410f104f18                   movups xmm1, xmmword ptr [r15 + 0x18]
1407a9f52: 0f298d40010000               movaps xmmword ptr [rbp + 0x140], xmm1
1407a9f59: 0f298530010000               movaps xmmword ptr [rbp + 0x130], xmm0
1407a9f60: 488b9510080000               mov rdx, qword ptr [rbp + 0x810]
1407a9f67: 8b421c                       mov eax, dword ptr [rdx + 0x1c]
1407a9f6a: 488b4a10                     mov rcx, qword ptr [rdx + 0x10]
1407a9f6e: 8901                         mov dword ptr [rcx], eax
1407a9f70: 488b3a                       mov rdi, qword ptr [rdx]
1407a9f73: 4489b548040000               mov dword ptr [rbp + 0x448], r14d
1407a9f7a: 488b85c0030000               mov rax, qword ptr [rbp + 0x3c0]
1407a9f81: 448930                       mov dword ptr [rax], r14d
1407a9f84: 418b4510                     mov eax, dword ptr [r13 + 0x10]
1407a9f88: 89854c040000                 mov dword ptr [rbp + 0x44c], eax
1407a9f8e: f3410f107514                 movss xmm6, dword ptr [r13 + 0x14]
1407a9f94: f3450f104d18                 movss xmm9, dword ptr [r13 + 0x18]
1407a9f9a: f3450f105d1c                 movss xmm11, dword ptr [r13 + 0x1c]
1407a9fa0: 4c89adc8030000               mov qword ptr [rbp + 0x3c8], r13
1407a9fa7: 41c7451000000000             mov dword ptr [r13 + 0x10], 0
1407a9faf: 488d8dd0000000               lea rcx, [rbp + 0xd0]
1407a9fb6: e8b5ca3700                   call 0x140b26a70
1407a9fbb: 48898500020000               mov qword ptr [rbp + 0x200], rax
1407a9fc2: 48899508020000               mov qword ptr [rbp + 0x208], rdx
1407a9fc9: 4c89a5e0030000               mov qword ptr [rbp + 0x3e0], r12
1407a9fd0: 41833c240b                   cmp dword ptr [r12], 0xb
1407a9fd5: 7431                         je 0x1407aa008
1407a9fd7: c785eb05000000000000         mov dword ptr [rbp + 0x5eb], 0
1407a9fe1: 48c785e405000000000000       mov qword ptr [rbp + 0x5e4], 0
1407a9fec: c70700000000                 mov dword ptr [rdi], 0
1407a9ff2: 488b85e0050000               mov rax, qword ptr [rbp + 0x5e0]
1407a9ff9: 488b8de7050000               mov rcx, qword ptr [rbp + 0x5e7]
1407aa000: 48894704                     mov qword ptr [rdi + 4], rax
1407aa004: 48894f0b                     mov qword ptr [rdi + 0xb], rcx
1407aa008: 4889bdc8060000               mov qword ptr [rbp + 0x6c8], rdi
1407aa00f: 4c8b8dd8060000               mov r9, qword ptr [rbp + 0x6d8]
1407aa016: 498b09                       mov rcx, qword ptr [r9]
1407aa019: 31c0                         xor eax, eax
1407aa01b: 4889ca                       mov rdx, rcx
1407aa01e: 482b5318                     sub rdx, qword ptr [rbx + 0x18]
1407aa022: 480f42d0                     cmovb rdx, rax
1407aa026: 482b4b38                     sub rcx, qword ptr [rbx + 0x38]
1407aa02a: 480f42c8                     cmovb rcx, rax
1407aa02e: 4c8b5b10                     mov r11, qword ptr [rbx + 0x10]
1407aa032: 4c8b4330                     mov r8, qword ptr [rbx + 0x30]
1407aa036: 4c8985b0060000               mov qword ptr [rbp + 0x6b0], r8
1407aa03d: 4929c8                       sub r8, rcx
1407aa040: 4c0f42c0                     cmovb r8, rax
1407aa044: 410f96c4                     setbe r12b
1407aa048: 4d89de                       mov r14, r11
1407aa04b: 4929d6                       sub r14, rdx
1407aa04e: 4c0f42f0                     cmovb r14, rax
1407aa052: 488b7b40                     mov rdi, qword ptr [rbx + 0x40]
1407aa056: 410f96c5                     setbe r13b
1407aa05a: 4989fa                       mov r10, rdi
1407aa05d: 4d29c2                       sub r10, r8
1407aa060: 4d29f2                       sub r10, r14
1407aa063: 4c8b7b08                     mov r15, qword ptr [rbx + 8]
1407aa067: 488b5b28                     mov rbx, qword ptr [rbx + 0x28]
1407aa06b: 4d8911                       mov qword ptr [r9], r10
1407aa06e: 488b8518080000               mov rax, qword ptr [rbp + 0x818]
1407aa075: 4584e5                       test r13b, r12b
1407aa078: 757e                         jne 0x1407aa0f8
1407aa07a: 4989ca                       mov r10, rcx
1407aa07d: 49c1e205                     shl r10, 5
1407aa081: 4c01d3                       add rbx, r10
1407aa084: 49c1e005                     shl r8, 5
1407aa088: 4989d2                       mov r10, rdx
1407aa08b: 49c1e205                     shl r10, 5
1407aa08f: 4d01d7                       add r15, r10
1407aa092: 49c1e605                     shl r14, 5
1407aa096: 4531d2                       xor r10d, r10d
1407aa099: 4939d3                       cmp r11, rdx
1407aa09c: ba08000000                   mov edx, 8
1407aa0a1: 4c0f42fa                     cmovb r15, rdx
1407aa0a5: 4f8d5c37e0                   lea r11, [r15 + r14 - 0x20]
1407aa0aa: 4d0f47d3                     cmova r10, r11
1407aa0ae: 48398db0060000               cmp qword ptr [rbp + 0x6b0], rcx
1407aa0b5: 480f42da                     cmovb rbx, rdx
1407aa0b9: 4a8d4c03e0                   lea rcx, [rbx + r8 - 0x20]
1407aa0be: 490f46ca                     cmovbe rcx, r10
1407aa0c2: 498939                       mov qword ptr [r9], rdi
1407aa0c5: 0fb6511a                     movzx edx, byte ptr [rcx + 0x1a]
1407aa0c9: 440fb74118                   movzx r8d, word ptr [rcx + 0x18]
1407aa0ce: 448b4908                     mov r9d, dword ptr [rcx + 8]
1407aa0d2: f30f104114                   movss xmm0, dword ptr [rcx + 0x14]
1407aa0d7: f20f10490c                   movsd xmm1, qword ptr [rcx + 0xc]
1407aa0dc: 488b8dc8060000               mov rcx, qword ptr [rbp + 0x6c8]
1407aa0e3: 448909                       mov dword ptr [rcx], r9d
1407aa0e6: f20f114904                   movsd qword ptr [rcx + 4], xmm1
1407aa0eb: f30f11410c                   movss dword ptr [rcx + 0xc], xmm0
1407aa0f0: 6644894110                   mov word ptr [rcx + 0x10], r8w
1407aa0f5: 885112                       mov byte ptr [rcx + 0x12], dl
1407aa0f8: 4c8bbde0060000               mov r15, qword ptr [rbp + 0x6e0]
1407aa0ff: 498b0f                       mov rcx, qword ptr [r15]
1407aa102: 4531c0                       xor r8d, r8d
1407aa105: 4889ca                       mov rdx, rcx
1407aa108: 482b5618                     sub rdx, qword ptr [rsi + 0x18]
1407aa10c: 490f42d0                     cmovb rdx, r8
1407aa110: 482b4e38                     sub rcx, qword ptr [rsi + 0x38]
1407aa114: 490f42c8                     cmovb rcx, r8
1407aa118: 488d3cd2                     lea rdi, [rdx + rdx*8]
1407aa11c: 48c1e703                     shl rdi, 3
1407aa120: 48037e08                     add rdi, qword ptr [rsi + 8]
1407aa124: 4c8b4e10                     mov r9, qword ptr [rsi + 0x10]
1407aa128: 4d89cc                       mov r12, r9
1407aa12b: 4929d4                       sub r12, rdx
1407aa12e: 4d0f42e0                     cmovb r12, r8
1407aa132: 4c8b5630                     mov r10, qword ptr [rsi + 0x30]
1407aa136: 41bb08000000                 mov r11d, 8
1407aa13c: 490f42fb                     cmovb rdi, r11
1407aa140: 4c8d34c9                     lea r14, [rcx + rcx*8]
1407aa144: 49c1e603                     shl r14, 3
1407aa148: 4c037628                     add r14, qword ptr [rsi + 0x28]
1407aa14c: 4c89d3                       mov rbx, r10
1407aa14f: 4829cb                       sub rbx, rcx
1407aa152: 490f42d8                     cmovb rbx, r8
1407aa156: 4d0f42f3                     cmovb r14, r11
1407aa15a: 4c8b4640                     mov r8, qword ptr [rsi + 0x40]
1407aa15e: 4929d8                       sub r8, rbx
1407aa161: 4d29e0                       sub r8, r12
1407aa164: 4d8907                       mov qword ptr [r15], r8
1407aa167: 488bb510020000               mov rsi, qword ptr [rbp + 0x210]
1407aa16e: 448b581c                     mov r11d, dword ptr [rax + 0x1c]
1407aa172: 44899dd4060000               mov dword ptr [rbp + 0x6d4], r11d
1407aa179: 4c8b18                       mov r11, qword ptr [rax]
1407aa17c: 4c899db0060000               mov qword ptr [rbp + 0x6b0], r11
1407aa183: 488b4010                     mov rax, qword ptr [rax + 0x10]
1407aa187: 48898588060000               mov qword ptr [rbp + 0x688], rax
1407aa18e: 488b8520020000               mov rax, qword ptr [rbp + 0x220]
1407aa195: 48898550040000               mov qword ptr [rbp + 0x450], rax
1407aa19c: 4889b528050000               mov qword ptr [rbp + 0x528], rsi
1407aa1a3: 488d4620                     lea rax, [rsi + 0x20]
1407aa1a7: 48898558040000               mov qword ptr [rbp + 0x458], rax
1407aa1ae: 8b852c020000                 mov eax, dword ptr [rbp + 0x22c]
1407aa1b4: 89859c050000                 mov dword ptr [rbp + 0x59c], eax
1407aa1ba: 4939d1                       cmp r9, rdx
1407aa1bd: 0f29b5b0010000               movaps xmmword ptr [rbp + 0x1b0], xmm6
1407aa1c4: 7613                         jbe 0x1407aa1d9
1407aa1c6: 4c89a5d8060000               mov qword ptr [rbp + 0x6d8], r12
1407aa1cd: 4c8d7f48                     lea r15, [rdi + 0x48]
1407aa1d1: 4d89f5                       mov r13, r14
1407aa1d4: 4989fc                       mov r12, rdi
1407aa1d7: eb16                         jmp 0x1407aa1ef
1407aa1d9: 4939ca                       cmp r10, rcx
1407aa1dc: 7672                         jbe 0x1407aa250
1407aa1de: 4c89a5d8060000               mov qword ptr [rbp + 0x6d8], r12
1407aa1e5: 4d8d6e48                     lea r13, [r14 + 0x48]
1407aa1e9: 4531ff                       xor r15d, r15d
1407aa1ec: 4d89f4                       mov r12, r14
1407aa1ef: 49ffc0                       inc r8
1407aa1f2: 488b85e0060000               mov rax, qword ptr [rbp + 0x6e0]
1407aa1f9: 4c8900                       mov qword ptr [rax], r8
1407aa1fc: 41833c2400                   cmp dword ptr [r12], 0
1407aa201: 7429                         je 0x1407aa22c
1407aa203: f3410f10742430               movss xmm6, dword ptr [r12 + 0x30]
1407aa20a: f3410f10442438               movss xmm0, dword ptr [r12 + 0x38]
1407aa211: 0f14f0                       unpcklps xmm6, xmm0
1407aa214: c685b806000003               mov byte ptr [rbp + 0x6b8], 3
1407aa21b: b801000000                   mov eax, 1
1407aa220: 488985a0050000               mov qword ptr [rbp + 0x5a0], rax
1407aa227: e9a5100000                   jmp 0x1407ab2d1
1407aa22c: 41f644240401                 test byte ptr [r12 + 4], 1
1407aa232: 0f84690d0000                 je 0x1407aafa1
1407aa238: 498b442410                   mov rax, qword ptr [r12 + 0x10]
1407aa23d: 488985d8050000               mov qword ptr [rbp + 0x5d8], rax
1407aa244: f2410f10742408               movsd xmm6, qword ptr [r12 + 8]
1407aa24b: e96f100000                   jmp 0x1407ab2bf
1407aa250: 4531d2                       xor r10d, r10d
1407aa253: 48c7857004000000000000       mov qword ptr [rbp + 0x470], 0
1407aa25e: 48c7857804000008000000       mov qword ptr [rbp + 0x478], 8
1407aa269: 48c7858004000000000000       mov qword ptr [rbp + 0x480], 0
1407aa274: 488bb5f8040000               mov rsi, qword ptr [rbp + 0x4f8]
1407aa27b: 488b16                       mov rdx, qword ptr [rsi]
1407aa27e: 31c9                         xor ecx, ecx
1407aa280: 4889d0                       mov rax, rdx
1407aa283: 4c8b9d08050000               mov r11, qword ptr [rbp + 0x508]
1407aa28a: 492b4318                     sub rax, qword ptr [r11 + 0x18]
1407aa28e: 480f42c1                     cmovb rax, rcx
1407aa292: 492b5338                     sub rdx, qword ptr [r11 + 0x38]
1407aa296: 480f42d1                     cmovb rdx, rcx
1407aa29a: 4c8b85c8030000               mov r8, qword ptr [rbp + 0x3c8]
1407aa2a1: 4983c014                     add r8, 0x14
1407aa2a5: 4c898588020000               mov qword ptr [rbp + 0x288], r8
1407aa2ac: 486bd878                     imul rbx, rax, 0x78
1407aa2b0: 49035b08                     add rbx, qword ptr [r11 + 8]
1407aa2b4: 4d8b4310                     mov r8, qword ptr [r11 + 0x10]
1407aa2b8: 4929c0                       sub r8, rax
1407aa2bb: 4c0f42c1                     cmovb r8, rcx
1407aa2bf: 4d8b4b30                     mov r9, qword ptr [r11 + 0x30]
1407aa2c3: bf08000000                   mov edi, 8
1407aa2c8: 480f42df                     cmovb rbx, rdi
1407aa2cc: 486bc278                     imul rax, rdx, 0x78
1407aa2d0: 49034328                     add rax, qword ptr [r11 + 0x28]
1407aa2d4: 4929d1                       sub r9, rdx
1407aa2d7: 4c0f42c9                     cmovb r9, rcx
1407aa2db: 480f42c7                     cmovb rax, rdi
1407aa2df: 498b4b40                     mov rcx, qword ptr [r11 + 0x40]
1407aa2e3: 4c29c9                       sub rcx, r9
1407aa2e6: 4c29c1                       sub rcx, r8
1407aa2e9: 48890e                       mov qword ptr [rsi], rcx
1407aa2ec: 496bf878                     imul rdi, r8, 0x78
1407aa2f0: 4801df                       add rdi, rbx
1407aa2f3: 4d6bf178                     imul r14, r9, 0x78
1407aa2f7: 4901c6                       add r14, rax
1407aa2fa: 4c8dbdf8050000               lea r15, [rbp + 0x5f8]
1407aa301: 4c8da540030000               lea r12, [rbp + 0x340]
1407aa308: 4c8dade0050000               lea r13, [rbp + 0x5e0]
1407aa30f: 4889bd68020000               mov qword ptr [rbp + 0x268], rdi
1407aa316: 4c89b570020000               mov qword ptr [rbp + 0x270], r14
1407aa31d: eb42                         jmp 0x1407aa361
1407aa31f: 488bb5f8040000               mov rsi, qword ptr [rbp + 0x4f8]
1407aa326: 4c8bb570020000               mov r14, qword ptr [rbp + 0x270]
1407aa32d: 4c8dbdf8050000               lea r15, [rbp + 0x5f8]
1407aa334: 4c8da540030000               lea r12, [rbp + 0x340]
1407aa33b: 4c8dade0050000               lea r13, [rbp + 0x5e0]
1407aa342: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407aa350: 41b201                       mov r10b, 1
1407aa353: 488b9dd0050000               mov rbx, qword ptr [rbp + 0x5d0]
1407aa35a: 488b85e0060000               mov rax, qword ptr [rbp + 0x6e0]
1407aa361: 4885db                       test rbx, rbx
1407aa364: 741a                         je 0x1407aa380
1407aa366: 4839fb                       cmp rbx, rdi
1407aa369: 7415                         je 0x1407aa380
1407aa36b: 488d4b78                     lea rcx, [rbx + 0x78]
1407aa36f: 48898dd0050000               mov qword ptr [rbp + 0x5d0], rcx
1407aa376: 488985e0060000               mov qword ptr [rbp + 0x6e0], rax
1407aa37d: eb2c                         jmp 0x1407aa3ab
1407aa37f: 90                           nop 
1407aa380: 4885c0                       test rax, rax
1407aa383: 0f8479040000                 je 0x1407aa802
1407aa389: 4c39f0                       cmp rax, r14
1407aa38c: 0f8470040000                 je 0x1407aa802
1407aa392: 488d4878                     lea rcx, [rax + 0x78]
1407aa396: 48898de0060000               mov qword ptr [rbp + 0x6e0], rcx
1407aa39d: 48c785d005000000000000       mov qword ptr [rbp + 0x5d0], 0
1407aa3a8: 4889c3                       mov rbx, rax
1407aa3ab: 48ff06                       inc qword ptr [rsi]
1407aa3ae: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407aa3b4: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407aa3bb: 8901                         mov dword ptr [rcx], eax
1407aa3bd: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407aa3c4: c6404801                     mov byte ptr [rax + 0x48], 1
1407aa3c8: 488b8598040000               mov rax, qword ptr [rbp + 0x498]
1407aa3cf: 8b8d80060000                 mov ecx, dword ptr [rbp + 0x680]
1407aa3d5: 8908                         mov dword ptr [rax], ecx
1407aa3d7: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa3de: 4c89e1                       mov rcx, r12
1407aa3e1: 488b9590040000               mov rdx, qword ptr [rbp + 0x490]
1407aa3e8: 41b001                       mov r8b, 1
1407aa3eb: e8d0950800                   call 0x1408339c0
1407aa3f0: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa3f7: 4c89e1                       mov rcx, r12
1407aa3fa: e8e1950800                   call 0x1408339e0
1407aa3ff: 8985d8060000                 mov dword ptr [rbp + 0x6d8], eax
1407aa405: 807b683f                     cmp byte ptr [rbx + 0x68], 0x3f
1407aa409: 0f85e2140000                 jne 0x1407ab8f1
1407aa40f: 0f1033                       movups xmm6, xmmword ptr [rbx]
1407aa412: 488b4330                     mov rax, qword ptr [rbx + 0x30]
1407aa416: 48898550050000               mov qword ptr [rbp + 0x550], rax
1407aa41d: 0f104310                     movups xmm0, xmmword ptr [rbx + 0x10]
1407aa421: 0f104b20                     movups xmm1, xmmword ptr [rbx + 0x20]
1407aa425: 0f298d40050000               movaps xmmword ptr [rbp + 0x540], xmm1
1407aa42c: 0f298530050000               movaps xmmword ptr [rbp + 0x530], xmm0
1407aa433: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa43a: e8311789ff                   call 0x14003bb70
1407aa43f: 8b4b64                       mov ecx, dword ptr [rbx + 0x64]
1407aa442: 8b5354                       mov edx, dword ptr [rbx + 0x54]
1407aa445: 4c8d8520060000               lea r8, [rbp + 0x620]
1407aa44c: 41895010                     mov dword ptr [r8 + 0x10], edx
1407aa450: 0f104344                     movups xmm0, xmmword ptr [rbx + 0x44]
1407aa454: 410f1100                     movups xmmword ptr [r8], xmm0
1407aa458: 488b9550050000               mov rdx, qword ptr [rbp + 0x550]
1407aa45f: 49895720                     mov qword ptr [r15 + 0x20], rdx
1407aa463: 660f6f8530050000             movdqa xmm0, xmmword ptr [rbp + 0x530]
1407aa46b: 0f288d40050000               movaps xmm1, xmmword ptr [rbp + 0x540]
1407aa472: 410f114f10                   movups xmmword ptr [r15 + 0x10], xmm1
1407aa477: f3410f7f07                   movdqu xmmword ptr [r15], xmm0
1407aa47c: 0f11b5e8050000               movups xmmword ptr [rbp + 0x5e8], xmm6
1407aa483: 8b95d8060000                 mov edx, dword ptr [rbp + 0x6d8]
1407aa489: 899534060000                 mov dword ptr [rbp + 0x634], edx
1407aa48f: 898d38060000                 mov dword ptr [rbp + 0x638], ecx
1407aa495: 488d4b58                     lea rcx, [rbx + 0x58]
1407aa499: 66c7853c060000013f           mov word ptr [rbp + 0x63c], 0x3f01
1407aa4a2: 88853e060000                 mov byte ptr [rbp + 0x63e], al
1407aa4a8: 48c785e005000003000000       mov qword ptr [rbp + 0x5e0], 3
1407aa4b3: 8b4108                       mov eax, dword ptr [rcx + 8]
1407aa4b6: 898538050000                 mov dword ptr [rbp + 0x538], eax
1407aa4bc: 48898d90060000               mov qword ptr [rbp + 0x690], rcx
1407aa4c3: 488b01                       mov rax, qword ptr [rcx]
1407aa4c6: 48898530050000               mov qword ptr [rbp + 0x530], rax
1407aa4cd: c6853c05000000               mov byte ptr [rbp + 0x53c], 0
1407aa4d4: 488b85a8050000               mov rax, qword ptr [rbp + 0x5a8]
1407aa4db: 8b8dac060000                 mov ecx, dword ptr [rbp + 0x6ac]
1407aa4e1: 8908                         mov dword ptr [rax], ecx
1407aa4e3: 488b8520050000               mov rax, qword ptr [rbp + 0x520]
1407aa4ea: 8b8da8060000                 mov ecx, dword ptr [rbp + 0x6a8]
1407aa4f0: 8908                         mov dword ptr [rax], ecx
1407aa4f2: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407aa4f8: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407aa4ff: 8901                         mov dword ptr [rcx], eax
1407aa501: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa508: 488b8518050000               mov rax, qword ptr [rbp + 0x518]
1407aa50f: 4889442438                   mov qword ptr [rsp + 0x38], rax
1407aa514: 488d85c0010000               lea rax, [rbp + 0x1c0]
1407aa51b: 4889442430                   mov qword ptr [rsp + 0x30], rax
1407aa520: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407aa527: 4889442428                   mov qword ptr [rsp + 0x28], rax
1407aa52c: 488b8510050000               mov rax, qword ptr [rbp + 0x510]
1407aa533: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407aa538: 4c89e9                       mov rcx, r13
1407aa53b: 31d2                         xor edx, edx
1407aa53d: 4c8d8530050000               lea r8, [rbp + 0x530]
1407aa544: 4c8b8dc0060000               mov r9, qword ptr [rbp + 0x6c0]
1407aa54b: e8e0c8ac00                   call 0x141276e30
1407aa550: f6433801                     test byte ptr [rbx + 0x38], 1
1407aa554: 0f84f6fdffff                 je 0x1407aa350
1407aa55a: 8b85cc050000                 mov eax, dword ptr [rbp + 0x5cc]
1407aa560: 488b8dc0050000               mov rcx, qword ptr [rbp + 0x5c0]
1407aa567: 8901                         mov dword ptr [rcx], eax
1407aa569: 488b8db0050000               mov rcx, qword ptr [rbp + 0x5b0]
1407aa570: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa577: e804431500                   call 0x1408fe880
1407aa57c: 8b85cc050000                 mov eax, dword ptr [rbp + 0x5cc]
1407aa582: 488b8dc0050000               mov rcx, qword ptr [rbp + 0x5c0]
1407aa589: 8901                         mov dword ptr [rcx], eax
1407aa58b: 488b8db0050000               mov rcx, qword ptr [rbp + 0x5b0]
1407aa592: 8b533c                       mov edx, dword ptr [rbx + 0x3c]
1407aa595: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa59c: 448b85d8060000               mov r8d, dword ptr [rbp + 0x6d8]
1407aa5a3: e8c8571500                   call 0x1408ffd70
1407aa5a8: 8b85cc050000                 mov eax, dword ptr [rbp + 0x5cc]
1407aa5ae: 488b8dc0050000               mov rcx, qword ptr [rbp + 0x5c0]
1407aa5b5: 8901                         mov dword ptr [rcx], eax
1407aa5b7: 488b8db0050000               mov rcx, qword ptr [rbp + 0x5b0]
1407aa5be: 448b4340                     mov r8d, dword ptr [rbx + 0x40]
1407aa5c2: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa5c9: 8b95d8060000                 mov edx, dword ptr [rbp + 0x6d8]
1407aa5cf: e89c571500                   call 0x1408ffd70
1407aa5d4: 8b85cc050000                 mov eax, dword ptr [rbp + 0x5cc]
1407aa5da: 488b8dc0050000               mov rcx, qword ptr [rbp + 0x5c0]
1407aa5e1: 8901                         mov dword ptr [rcx], eax
1407aa5e3: 488b8db0050000               mov rcx, qword ptr [rbp + 0x5b0]
1407aa5ea: 8b533c                       mov edx, dword ptr [rbx + 0x3c]
1407aa5ed: 448b4340                     mov r8d, dword ptr [rbx + 0x40]
1407aa5f1: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa5f8: e883581500                   call 0x1408ffe80
1407aa5fd: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa604: ba02000000                   mov edx, 2
1407aa609: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1407aa610: e8ab1c2700                   call 0x140a1c2c0
1407aa615: 4885c0                       test rax, rax
1407aa618: 0f8432fdffff                 je 0x1407aa350
1407aa61e: 4889c2                       mov rdx, rax
1407aa621: 4883c230                     add rdx, 0x30
1407aa625: 0f107050                     movups xmm6, xmmword ptr [rax + 0x50]
1407aa629: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407aa630: 4c89e9                       mov rcx, r13
1407aa633: e88895ab00                   call 0x141263bc0
1407aa638: 0f11b500060000               movups xmmword ptr [rbp + 0x600], xmm6
1407aa63f: 488b85e0050000               mov rax, qword ptr [rbp + 0x5e0]
1407aa646: 488985d8050000               mov qword ptr [rbp + 0x5d8], rax
1407aa64d: 4885c0                       test rax, rax
1407aa650: 0f84fafcffff                 je 0x1407aa350
1407aa656: 48899db8060000               mov qword ptr [rbp + 0x6b8], rbx
1407aa65d: 488bbdf8050000               mov rdi, qword ptr [rbp + 0x5f8]
1407aa664: 4885ff                       test rdi, rdi
1407aa667: 0f8448010000                 je 0x1407aa7b5
1407aa66d: 4c8bbdd8050000               mov r15, qword ptr [rbp + 0x5d8]
1407aa674: 4d8d6f10                     lea r13, [r15 + 0x10]
1407aa678: 66410f6f07                   movdqa xmm0, xmmword ptr [r15]
1407aa67d: 660fd7d8                     pmovmskb ebx, xmm0
1407aa681: f7d3                         not ebx
1407aa683: eb67                         jmp 0x1407aa6ec
1407aa685: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
1407aa690: 488b8578040000               mov rax, qword ptr [rbp + 0x478]
1407aa697: 4b8d0cb6                     lea rcx, [r14 + r14*4]
1407aa69b: 488b9590060000               mov rdx, qword ptr [rbp + 0x690]
1407aa6a2: 488914c8                     mov qword ptr [rax + rcx*8], rdx
1407aa6a6: 8974c808                     mov dword ptr [rax + rcx*8 + 8], esi
1407aa6aa: 448964c80c                   mov dword ptr [rax + rcx*8 + 0xc], r12d
1407aa6af: 0f1374c810                   movlps qword ptr [rax + rcx*8 + 0x10], xmm6
1407aa6b4: f30f117cc818                 movss dword ptr [rax + rcx*8 + 0x18], xmm7
1407aa6ba: 8b95d8060000                 mov edx, dword ptr [rbp + 0x6d8]
1407aa6c0: 8954c81c                     mov dword ptr [rax + rcx*8 + 0x1c], edx
1407aa6c4: 8b9508050000                 mov edx, dword ptr [rbp + 0x508]
1407aa6ca: 8954c820                     mov dword ptr [rax + rcx*8 + 0x20], edx
1407aa6ce: 0fb695a0050000               movzx edx, byte ptr [rbp + 0x5a0]
1407aa6d5: 8854c824                     mov byte ptr [rax + rcx*8 + 0x24], dl
1407aa6d9: 49ffc6                       inc r14
1407aa6dc: 4c89b580040000               mov qword ptr [rbp + 0x480], r14
1407aa6e3: 48ffcf                       dec rdi
1407aa6e6: 0f84c9000000                 je 0x1407aa7b5
1407aa6ec: 6685db                       test bx, bx
1407aa6ef: 740f                         je 0x1407aa700
1407aa6f1: 8d43ff                       lea eax, [rbx - 1]
1407aa6f4: 21d8                         and eax, ebx
1407aa6f6: eb32                         jmp 0x1407aa72a
1407aa6f8: 0f1f840000000000             nop dword ptr [rax + rax]
1407aa700: 66410f6f4500                 movdqa xmm0, xmmword ptr [r13]
1407aa706: 660fd7d8                     pmovmskb ebx, xmm0
1407aa70a: 4981c700fbffff               add r15, -0x500
1407aa711: 4983c510                     add r13, 0x10
1407aa715: 81fbffff0000                 cmp ebx, 0xffff
1407aa71b: 74e3                         je 0x1407aa700
1407aa71d: b9feffffff                   mov ecx, 0xfffffffe
1407aa722: 29d9                         sub ecx, ebx
1407aa724: f7d3                         not ebx
1407aa726: 89d8                         mov eax, ebx
1407aa728: 21c8                         and eax, ecx
1407aa72a: 89d9                         mov ecx, ebx
1407aa72c: f30fbcc9                     tzcnt ecx, ecx
1407aa730: 89c3                         mov ebx, eax
1407aa732: 48f7d9                       neg rcx
1407aa735: 488d0489                     lea rax, [rcx + rcx*4]
1407aa739: 48c1e004                     shl rax, 4
1407aa73d: 41807c07da04                 cmp byte ptr [r15 + rax - 0x26], 4
1407aa743: 759e                         jne 0x1407aa6e3
1407aa745: 4c01f8                       add rax, r15
1407aa748: 8b70b8                       mov esi, dword ptr [rax - 0x48]
1407aa74b: 488b8db8060000               mov rcx, qword ptr [rbp + 0x6b8]
1407aa752: 3b713c                       cmp esi, dword ptr [rcx + 0x3c]
1407aa755: 758c                         jne 0x1407aa6e3
1407aa757: 448b60bc                     mov r12d, dword ptr [rax - 0x44]
1407aa75b: 488b8db8060000               mov rcx, qword ptr [rbp + 0x6b8]
1407aa762: 443b6140                     cmp r12d, dword ptr [rcx + 0x40]
1407aa766: 0f8577ffffff                 jne 0x1407aa6e3
1407aa76c: f20f1070c0                   movsd xmm6, qword ptr [rax - 0x40]
1407aa771: f30f1078c8                   movss xmm7, dword ptr [rax - 0x38]
1407aa776: 8b48b0                       mov ecx, dword ptr [rax - 0x50]
1407aa779: 898d08050000                 mov dword ptr [rbp + 0x508], ecx
1407aa77f: 0fb640fe                     movzx eax, byte ptr [rax - 2]
1407aa783: 8885a0050000                 mov byte ptr [rbp + 0x5a0], al
1407aa789: 4c8bb580040000               mov r14, qword ptr [rbp + 0x480]
1407aa790: 4c3bb570040000               cmp r14, qword ptr [rbp + 0x470]
1407aa797: 0f85f3feffff                 jne 0x1407aa690
1407aa79d: 488d8d70040000               lea rcx, [rbp + 0x470]
1407aa7a4: 488d1535872d02               lea rdx, [rip + 0x22d8735]
1407aa7ab: e80083dbff                   call 0x140562ab0
1407aa7b0: e9dbfeffff                   jmp 0x1407aa690
1407aa7b5: 4c8b8de8050000               mov r9, qword ptr [rbp + 0x5e8]
1407aa7bc: 4d85c9                       test r9, r9
1407aa7bf: 488bbd68020000               mov rdi, qword ptr [rbp + 0x268]
1407aa7c6: 0f8453fbffff                 je 0x1407aa31f
1407aa7cc: 498d4101                     lea rax, [r9 + 1]
1407aa7d0: b950000000                   mov ecx, 0x50
1407aa7d5: 48f7e1                       mul rcx
1407aa7d8: 4901c1                       add r9, rax
1407aa7db: 4983c111                     add r9, 0x11
1407aa7df: 0f843afbffff                 je 0x1407aa31f
1407aa7e5: 488b8dd8050000               mov rcx, qword ptr [rbp + 0x5d8]
1407aa7ec: 4829c1                       sub rcx, rax
1407aa7ef: 41b810000000                 mov r8d, 0x10
1407aa7f5: 4c89ca                       mov rdx, r9
1407aa7f8: e82394e6ff                   call 0x140613c20
1407aa7fd: e91dfbffff                   jmp 0x1407aa31f
1407aa802: 488bb570040000               mov rsi, qword ptr [rbp + 0x470]
1407aa809: 4c8bad78040000               mov r13, qword ptr [rbp + 0x478]
1407aa810: 488b8580040000               mov rax, qword ptr [rbp + 0x480]
1407aa817: 488d0480                     lea rax, [rax + rax*4]
1407aa81b: 4c8d3cc500000000             lea r15, [rax*8]
1407aa823: 4d01ef                       add r15, r13
1407aa826: 66450f76e4                   pcmpeqd xmm12, xmm12
1407aa82b: f30f1035d1b01702             movss xmm6, dword ptr [rip + 0x217b0d1]
1407aa833: 440f282db5b01702             movaps xmm13, xmmword ptr [rip + 0x217b0b5]
1407aa83b: f3440f103560332402           movss xmm14, dword ptr [rip + 0x2243360]
1407aa844: 4c89adf8040000               mov qword ptr [rbp + 0x4f8], r13
1407aa84b: 4489d3                       mov ebx, r10d
1407aa84e: 4889b590060000               mov qword ptr [rbp + 0x690], rsi
1407aa855: 4c89bdd8050000               mov qword ptr [rbp + 0x5d8], r15
1407aa85c: e97f030000                   jmp 0x1407aabe0
1407aa861: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407aa870: f3420f6f0c01                 movdqu xmm1, xmmword ptr [rcx + r8]
1407aa876: 660f6fd1                     movdqa xmm2, xmm1
1407aa87a: 660f74d0                     pcmpeqb xmm2, xmm0
1407aa87e: 66440fd7d2                   pmovmskb r10d, xmm2
1407aa883: 4585d2                       test r10d, r10d
1407aa886: 7428                         je 0x1407aa8b0
1407aa888: f3450fbcda                   tzcnt r11d, r10d
1407aa88d: 4d01c3                       add r11, r8
1407aa890: 4921d3                       and r11, rdx
1407aa893: 49f7db                       neg r11
1407aa896: 4d6bdb78                     imul r11, r11, 0x78
1407aa89a: 4e3b3418                     cmp r14, qword ptr [rax + r11]
1407aa89e: 7440                         je 0x1407aa8e0
1407aa8a0: 458d5aff                     lea r11d, [r10 - 1]
1407aa8a4: 664521d3                     and r11w, r10w
1407aa8a8: 4589da                       mov r10d, r11d
1407aa8ab: 75db                         jne 0x1407aa888
1407aa8ad: 0f1f00                       nop dword ptr [rax]
1407aa8b0: 66410f74cc                   pcmpeqb xmm1, xmm12
1407aa8b5: 66440fd7d1                   pmovmskb r10d, xmm1
1407aa8ba: 4585d2                       test r10d, r10d
1407aa8bd: 0f85ed030000                 jne 0x1407aacb0
1407aa8c3: 4d01c8                       add r8, r9
1407aa8c6: 4983c010                     add r8, 0x10
1407aa8ca: 4983c110                     add r9, 0x10
1407aa8ce: 4921d0                       and r8, rdx
1407aa8d1: eb9d                         jmp 0x1407aa870
1407aa8d3: 666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
1407aa8e0: 4889b508050000               mov qword ptr [rbp + 0x508], rsi
1407aa8e7: 4488a5a0050000               mov byte ptr [rbp + 0x5a0], r12b
1407aa8ee: 4e8b7c19a0                   mov r15, qword ptr [rcx + r11 - 0x60]
1407aa8f3: 4c89f8                       mov rax, r15
1407aa8f6: ba0c000000                   mov edx, 0xc
1407aa8fb: 48f7e2                       mul rdx
1407aa8fe: 4989c6                       mov r14, rax
1407aa901: 0f80e6290000                 jo 0x1407ad2ed
1407aa907: bf00000000                   mov edi, 0
1407aa90c: 48b8fcffffffffffff7f         movabs rax, 0x7ffffffffffffffc
1407aa916: 4939c6                       cmp r14, rax
1407aa919: 0f87d0290000                 ja 0x1407ad2ef
1407aa91f: 89de                         mov esi, ebx
1407aa921: 4e8b641998                   mov r12, qword ptr [rcx + r11 - 0x68]
1407aa926: 4d85f6                       test r14, r14
1407aa929: 742a                         je 0x1407aa955
1407aa92b: 0fb605d7a2e802               movzx eax, byte ptr [rip + 0x2e8a2d7]
1407aa932: bf04000000                   mov edi, 4
1407aa937: ba04000000                   mov edx, 4
1407aa93c: 4c89f1                       mov rcx, r14
1407aa93f: e8cc92e6ff                   call 0x140613c10
1407aa944: 4885c0                       test rax, rax
1407aa947: 0f84a2290000                 je 0x1407ad2ef
1407aa94d: 4889c3                       mov rbx, rax
1407aa950: 4c89ff                       mov rdi, r15
1407aa953: eb07                         jmp 0x1407aa95c
1407aa955: bb04000000                   mov ebx, 4
1407aa95a: 31ff                         xor edi, edi
1407aa95c: 4889d9                       mov rcx, rbx
1407aa95f: 4c89e2                       mov rdx, r12
1407aa962: 4d89f0                       mov r8, r14
1407aa965: e896921702                   call 0x142923c00
1407aa96a: 4889bd30050000               mov qword ptr [rbp + 0x530], rdi
1407aa971: 48899d38050000               mov qword ptr [rbp + 0x538], rbx
1407aa978: 4c89bd40050000               mov qword ptr [rbp + 0x540], r15
1407aa97f: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407aa986: 488d9530050000               lea rdx, [rbp + 0x530]
1407aa98d: 41b801000000                 mov r8d, 1
1407aa993: e848f3ab00                   call 0x141269ce0
1407aa998: 4c8bbdd8050000               mov r15, qword ptr [rbp + 0x5d8]
1407aa99f: 448aa5a0050000               mov r12b, byte ptr [rbp + 0x5a0]
1407aa9a6: 488b8d08050000               mov rcx, qword ptr [rbp + 0x508]
1407aa9ad: 89f3                         mov ebx, esi
1407aa9af: 488bbde0050000               mov rdi, qword ptr [rbp + 0x5e0]
1407aa9b6: 4889f8                       mov rax, rdi
1407aa9b9: 48f7d8                       neg rax
1407aa9bc: 0f80ee020000                 jo 0x1407aacb0
1407aa9c2: 488d85e8050000               lea rax, [rbp + 0x5e8]
1407aa9c9: 0f1000                       movups xmm0, xmmword ptr [rax]
1407aa9cc: 0f104810                     movups xmm1, xmmword ptr [rax + 0x10]
1407aa9d0: 0f105020                     movups xmm2, xmmword ptr [rax + 0x20]
1407aa9d4: 488d8538050000               lea rax, [rbp + 0x538]
1407aa9db: 0f115020                     movups xmmword ptr [rax + 0x20], xmm2
1407aa9df: 0f114810                     movups xmmword ptr [rax + 0x10], xmm1
1407aa9e3: 0f1100                       movups xmmword ptr [rax], xmm0
1407aa9e6: 4889bd30050000               mov qword ptr [rbp + 0x530], rdi
1407aa9ed: f30f1009                     movss xmm1, dword ptr [rcx]
1407aa9f1: f30f105108                   movss xmm2, dword ptr [rcx + 8]
1407aa9f6: 488d8d30050000               lea rcx, [rbp + 0x530]
1407aa9fd: 4c8d0dc4842d02               lea r9, [rip + 0x22d84c4]
1407aaa04: e807f7ab00                   call 0x14126a110
1407aaa09: 440f28c0                     movaps xmm8, xmm0
1407aaa0d: 0f2ec7                       ucomiss xmm0, xmm7
1407aaa10: 762a                         jbe 0x1407aaa3c
1407aaa12: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
1407aaa18: 8b95e0060000                 mov edx, dword ptr [rbp + 0x6e0]
1407aaa1e: e83d681500                   call 0x140901260
1407aaa23: 89c6                         mov esi, eax
1407aaa25: 4189d6                       mov r14d, edx
1407aaa28: 0f57c0                       xorps xmm0, xmm0
1407aaa2b: 410f28c8                     movaps xmm1, xmm8
1407aaa2f: 0f28d7                       movaps xmm2, xmm7
1407aaa32: e8695a4e00                   call 0x140c904a0
1407aaa37: 0f28f8                       movaps xmm7, xmm0
1407aaa3a: eb31                         jmp 0x1407aaa6d
1407aaa3c: 8b8de0060000                 mov ecx, dword ptr [rbp + 0x6e0]
1407aaa42: 8b95b8060000                 mov edx, dword ptr [rbp + 0x6b8]
1407aaa48: e813681500                   call 0x140901260
1407aaa4d: 89c6                         mov esi, eax
1407aaa4f: 4189d6                       mov r14d, edx
1407aaa52: 0f28c6                       movaps xmm0, xmm6
1407aaa55: 410f28c8                     movaps xmm1, xmm8
1407aaa59: 0f28d7                       movaps xmm2, xmm7
1407aaa5c: e83f5a4e00                   call 0x140c904a0
1407aaa61: 0f28f8                       movaps xmm7, xmm0
1407aaa64: 450f57fd                     xorps xmm15, xmm13
1407aaa68: f3450f58d6                   addss xmm10, xmm14
1407aaa6d: e8fe1089ff                   call 0x14003bb70
1407aaa72: 48c785e005000002000000       mov qword ptr [rbp + 0x5e0], 2
1407aaa7d: 48c785f005000002000000       mov qword ptr [rbp + 0x5f0], 2
1407aaa88: 89b500060000                 mov dword ptr [rbp + 0x600], esi
1407aaa8e: 4489b504060000               mov dword ptr [rbp + 0x604], r14d
1407aaa95: f3440f11bd08060000           movss dword ptr [rbp + 0x608], xmm15
1407aaa9e: f30f11bd0c060000             movss dword ptr [rbp + 0x60c], xmm7
1407aaaa6: f3440f119510060000           movss dword ptr [rbp + 0x610], xmm10
1407aaaaf: c6852206000004               mov byte ptr [rbp + 0x622], 4
1407aaab6: 888528060000                 mov byte ptr [rbp + 0x628], al
1407aaabc: c7853006000000000000         mov dword ptr [rbp + 0x630], 0
1407aaac6: c7853806000000000000         mov dword ptr [rbp + 0x638], 0
1407aaad0: c6854006000000               mov byte ptr [rbp + 0x640], 0
1407aaad7: c6854206000000               mov byte ptr [rbp + 0x642], 0
1407aaade: 66c785440600000240           mov word ptr [rbp + 0x644], 0x4002
1407aaae7: 8b85d0050000                 mov eax, dword ptr [rbp + 0x5d0]
1407aaaed: 898548060000                 mov dword ptr [rbp + 0x648], eax
1407aaaf3: 4488a54c060000               mov byte ptr [rbp + 0x64c], r12b
1407aaafa: 488b8d08050000               mov rcx, qword ptr [rbp + 0x508]
1407aab01: 8b4108                       mov eax, dword ptr [rcx + 8]
1407aab04: 898548030000                 mov dword ptr [rbp + 0x348], eax
1407aab0a: 488b01                       mov rax, qword ptr [rcx]
1407aab0d: 48898540030000               mov qword ptr [rbp + 0x340], rax
1407aab14: c6854c03000000               mov byte ptr [rbp + 0x34c], 0
1407aab1b: 488b85a8050000               mov rax, qword ptr [rbp + 0x5a8]
1407aab22: 8b8dac060000                 mov ecx, dword ptr [rbp + 0x6ac]
1407aab28: 8908                         mov dword ptr [rax], ecx
1407aab2a: 488b8520050000               mov rax, qword ptr [rbp + 0x520]
1407aab31: 8b8da8060000                 mov ecx, dword ptr [rbp + 0x6a8]
1407aab37: 8908                         mov dword ptr [rax], ecx
1407aab39: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407aab3f: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407aab46: 8901                         mov dword ptr [rcx], eax
1407aab48: 488b8518050000               mov rax, qword ptr [rbp + 0x518]
1407aab4f: 4889442438                   mov qword ptr [rsp + 0x38], rax
1407aab54: 488d85c0010000               lea rax, [rbp + 0x1c0]
1407aab5b: 4889442430                   mov qword ptr [rsp + 0x30], rax
1407aab60: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407aab67: 4889442428                   mov qword ptr [rsp + 0x28], rax
1407aab6c: 488b8510050000               mov rax, qword ptr [rbp + 0x510]
1407aab73: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407aab78: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407aab7f: 31d2                         xor edx, edx
1407aab81: 4c8d8540030000               lea r8, [rbp + 0x340]
1407aab88: 4c8b8dc0060000               mov r9, qword ptr [rbp + 0x6c0]
1407aab8f: e89cc2ac00                   call 0x141276e30
1407aab94: 4885ff                       test rdi, rdi
1407aab97: 488bb590060000               mov rsi, qword ptr [rbp + 0x690]
1407aab9e: 741a                         je 0x1407aabba
1407aaba0: 488b8d38050000               mov rcx, qword ptr [rbp + 0x538]
1407aaba7: 48c1e702                     shl rdi, 2
1407aabab: 488d147f                     lea rdx, [rdi + rdi*2]
1407aabaf: 41b804000000                 mov r8d, 4
1407aabb5: e86690e6ff                   call 0x140613c20
1407aabba: 488b9548050000               mov rdx, qword ptr [rbp + 0x548]
1407aabc1: 4885d2                       test rdx, rdx
1407aabc4: 741a                         je 0x1407aabe0
1407aabc6: 488b8d50050000               mov rcx, qword ptr [rbp + 0x550]
1407aabcd: 48c1e202                     shl rdx, 2
1407aabd1: 41b804000000                 mov r8d, 4
1407aabd7: e84490e6ff                   call 0x140613c20
1407aabdc: 0f1f4000                     nop dword ptr [rax]
1407aabe0: 4d39fd                       cmp r13, r15
1407aabe3: 0f84d3000000                 je 0x1407aacbc
1407aabe9: 450fb66524                   movzx r12d, byte ptr [r13 + 0x24]
1407aabee: 4180fc40                     cmp r12b, 0x40
1407aabf2: 0f84c4000000                 je 0x1407aacbc
1407aabf8: 498b7500                     mov rsi, qword ptr [r13]
1407aabfc: 418b4508                     mov eax, dword ptr [r13 + 8]
1407aac00: 8985d8060000                 mov dword ptr [rbp + 0x6d8], eax
1407aac06: 418b450c                     mov eax, dword ptr [r13 + 0xc]
1407aac0a: 8985b8060000                 mov dword ptr [rbp + 0x6b8], eax
1407aac10: f3450f107d10                 movss xmm15, dword ptr [r13 + 0x10]
1407aac16: f3410f107d14                 movss xmm7, dword ptr [r13 + 0x14]
1407aac1c: f3450f105518                 movss xmm10, dword ptr [r13 + 0x18]
1407aac22: 418b451c                     mov eax, dword ptr [r13 + 0x1c]
1407aac26: 8985e0060000                 mov dword ptr [rbp + 0x6e0], eax
1407aac2c: 418b4520                     mov eax, dword ptr [r13 + 0x20]
1407aac30: 8985d0050000                 mov dword ptr [rbp + 0x5d0], eax
1407aac36: 488b8db8030000               mov rcx, qword ptr [rbp + 0x3b8]
1407aac3d: e83e98acff                   call 0x140274480
1407aac42: 4889c7                       mov rdi, rax
1407aac45: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
1407aac4b: 8b95b8060000                 mov edx, dword ptr [rbp + 0x6b8]
1407aac51: e81af21a00                   call 0x140959e70
1407aac56: 4989c6                       mov r14, rax
1407aac59: 4983c528                     add r13, 0x28
1407aac5d: 48898590020000               mov qword ptr [rbp + 0x290], rax
1407aac64: 48837f1800                   cmp qword ptr [rdi + 0x18], 0
1407aac69: 7445                         je 0x1407aacb0
1407aac6b: 4889f9                       mov rcx, rdi
1407aac6e: 4883c120                     add rcx, 0x20
1407aac72: 488d9590020000               lea rdx, [rbp + 0x290]
1407aac79: e8f260e0ff                   call 0x1405b0d70
1407aac7e: 488b0f                       mov rcx, qword ptr [rdi]
1407aac81: 488b5708                     mov rdx, qword ptr [rdi + 8]
1407aac85: 4989d0                       mov r8, rdx
1407aac88: 4921c0                       and r8, rax
1407aac8b: 48c1e839                     shr rax, 0x39
1407aac8f: 660f6ec0                     movd xmm0, eax
1407aac93: 660f60c0                     punpcklbw xmm0, xmm0
1407aac97: f20f70c000                   pshuflw xmm0, xmm0, 0
1407aac9c: 660f70c000                   pshufd xmm0, xmm0, 0
1407aaca1: 488d4188                     lea rax, [rcx - 0x78]
1407aaca5: 4531c9                       xor r9d, r9d
1407aaca8: e9c3fbffff                   jmp 0x1407aa870
1407aacad: 0f1f00                       nop dword ptr [rax]
1407aacb0: 488bb590060000               mov rsi, qword ptr [rbp + 0x690]
1407aacb7: e924ffffff                   jmp 0x1407aabe0
1407aacbc: 4885f6                       test rsi, rsi
1407aacbf: 741a                         je 0x1407aacdb
1407aacc1: 48c1e603                     shl rsi, 3
1407aacc5: 488d14b6                     lea rdx, [rsi + rsi*4]
1407aacc9: 41b808000000                 mov r8d, 8
1407aaccf: 488b8df8040000               mov rcx, qword ptr [rbp + 0x4f8]
1407aacd6: e8458fe6ff                   call 0x140613c20
1407aacdb: f6c301                       test bl, 1
1407aacde: 0f85e5240000                 jne 0x1407ad1c9
1407aace4: 4c8b8de8060000               mov r9, qword ptr [rbp + 0x6e8]
1407aaceb: 498b01                       mov rax, qword ptr [r9]
1407aacee: 498b4908                     mov rcx, qword ptr [r9 + 8]
1407aacf2: 4883c0fe                     add rax, -2
1407aacf6: 4883d1ff                     adc rcx, -1
1407aacfa: 4883f805                     cmp rax, 5
1407aacfe: 4889ca                       mov rdx, rcx
1407aad01: 4883da00                     sbb rdx, 0
1407aad05: 0f92c2                       setb dl
1407aad08: 4883f803                     cmp rax, 3
1407aad0c: 410f94c0                     sete r8b
1407aad10: 4120d0                       and r8b, dl
1407aad13: 4180f801                     cmp r8b, 1
1407aad17: 0f85f7000000                 jne 0x1407aae14
1407aad1d: 498b5110                     mov rdx, qword ptr [r9 + 0x10]
1407aad21: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aad28: 488b8dd8030000               mov rcx, qword ptr [rbp + 0x3d8]
1407aad2f: e84c30edff                   call 0x14067dd80
1407aad34: 48898500040000               mov qword ptr [rbp + 0x400], rax
1407aad3b: 48899508040000               mov qword ptr [rbp + 0x408], rdx
1407aad42: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aad49: 488d8d00040000               lea rcx, [rbp + 0x400]
1407aad50: e8bb311800                   call 0x14092df10
1407aad55: 88c3                         mov bl, al
1407aad57: 488b95b0050000               mov rdx, qword ptr [rbp + 0x5b0]
1407aad5e: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aad65: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407aad6c: e81f3b1500                   call 0x1408fe890
1407aad71: 83bde005000003               cmp dword ptr [rbp + 0x5e0], 3
1407aad78: 0f855a030000                 jne 0x1407ab0d8
1407aad7e: 31ff                         xor edi, edi
1407aad80: 80fb02                       cmp bl, 2
1407aad83: 0f8336240000                 jae 0x1407ad1bf
1407aad89: 8b85cc050000                 mov eax, dword ptr [rbp + 0x5cc]
1407aad8f: 488b8dc0050000               mov rcx, qword ptr [rbp + 0x5c0]
1407aad96: 8901                         mov dword ptr [rcx], eax
1407aad98: 488b8db0050000               mov rcx, qword ptr [rbp + 0x5b0]
1407aad9f: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aada6: e8d53a1500                   call 0x1408fe880
1407aadab: 488b8550040000               mov rax, qword ptr [rbp + 0x450]
1407aadb2: 8b8d9c050000                 mov ecx, dword ptr [rbp + 0x59c]
1407aadb8: 8908                         mov dword ptr [rax], ecx
1407aadba: 488b8528050000               mov rax, qword ptr [rbp + 0x528]
1407aadc1: 488b7030                     mov rsi, qword ptr [rax + 0x30]
1407aadc5: 488b5840                     mov rbx, qword ptr [rax + 0x40]
1407aadc9: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
1407aadcd: 751a                         jne 0x1407aade9
1407aadcf: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aadd6: 488d15a3714702               lea rdx, [rip + 0x24771a3]
1407aaddd: 488b8d58040000               mov rcx, qword ptr [rbp + 0x458]
1407aade4: e8c77fdbff                   call 0x140562db0
1407aade9: 488b9528050000               mov rdx, qword ptr [rbp + 0x528]
1407aadf0: 488b4228                     mov rax, qword ptr [rdx + 0x28]
1407aadf4: 488d0c76                     lea rcx, [rsi + rsi*2]
1407aadf8: c704c800000000               mov dword ptr [rax + rcx*8], 0
1407aadff: 48895cc810                   mov qword ptr [rax + rcx*8 + 0x10], rbx
1407aae04: 48ffc6                       inc rsi
1407aae07: 48897230                     mov qword ptr [rdx + 0x30], rsi
1407aae0b: 48ff4240                     inc qword ptr [rdx + 0x40]
1407aae0f: e9ab230000                   jmp 0x1407ad1bf
1407aae14: 31d2                         xor edx, edx
1407aae16: 41b804000000                 mov r8d, 4
1407aae1c: 4939c0                       cmp r8, rax
1407aae1f: 4819ca                       sbb rdx, rcx
1407aae22: 0f92c1                       setb cl
1407aae25: 4883f8ff                     cmp rax, -1
1407aae29: 0f94c0                       sete al
1407aae2c: 08c8                         or al, cl
1407aae2e: 3c01                         cmp al, 1
1407aae30: 0f8513020000                 jne 0x1407ab049
1407aae36: 450fb67169                   movzx r14d, byte ptr [r9 + 0x69]
1407aae3b: 488b8500050000               mov rax, qword ptr [rbp + 0x500]
1407aae42: 80b88102000000               cmp byte ptr [rax + 0x281], 0
1407aae49: 753a                         jne 0x1407aae85
1407aae4b: 488b85a0040000               mov rax, qword ptr [rbp + 0x4a0]
1407aae52: 8b8d84060000                 mov ecx, dword ptr [rbp + 0x684]
1407aae58: 8908                         mov dword ptr [rax], ecx
1407aae5a: 488b8500050000               mov rax, qword ptr [rbp + 0x500]
1407aae61: c6808202000001               mov byte ptr [rax + 0x282], 1
1407aae68: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407aae6f: 0fb64068                     movzx eax, byte ptr [rax + 0x68]
1407aae73: 488b8dc8060000               mov rcx, qword ptr [rbp + 0x6c8]
1407aae7a: 884110                       mov byte ptr [rcx + 0x10], al
1407aae7d: 3c01                         cmp al, 1
1407aae7f: 0f8544230000                 jne 0x1407ad1c9
1407aae85: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407aae8c: 488b5060                     mov rdx, qword ptr [rax + 0x60]
1407aae90: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aae97: 488b8dd8030000               mov rcx, qword ptr [rbp + 0x3d8]
1407aae9e: e8dd2eedff                   call 0x14067dd80
1407aaea3: 48898520030000               mov qword ptr [rbp + 0x320], rax
1407aaeaa: 48899528030000               mov qword ptr [rbp + 0x328], rdx
1407aaeb1: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aaeb8: 488d8d20030000               lea rcx, [rbp + 0x320]
1407aaebf: e8bc371800                   call 0x14092e680
1407aaec4: 84c0                         test al, al
1407aaec6: 0f84fd220000                 je 0x1407ad1c9
1407aaecc: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aaed3: 488d8d20030000               lea rcx, [rbp + 0x320]
1407aaeda: e831301800                   call 0x14092df10
1407aaedf: 4189c5                       mov r13d, eax
1407aaee2: 4180fd02                     cmp r13b, 2
1407aaee6: 0f92c1                       setb cl
1407aaee9: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407aaef0: 0a4868                       or cl, byte ptr [rax + 0x68]
1407aaef3: 888db8060000                 mov byte ptr [rbp + 0x6b8], cl
1407aaef9: 4180fd02                     cmp r13b, 2
1407aaefd: 7337                         jae 0x1407aaf36
1407aaeff: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407aaf05: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407aaf0c: 8901                         mov dword ptr [rcx], eax
1407aaf0e: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407aaf15: c6404801                     mov byte ptr [rax + 0x48], 1
1407aaf19: 488b85a0040000               mov rax, qword ptr [rbp + 0x4a0]
1407aaf20: 8b8d84060000                 mov ecx, dword ptr [rbp + 0x684]
1407aaf26: 8908                         mov dword ptr [rax], ecx
1407aaf28: 488b8500050000               mov rax, qword ptr [rbp + 0x500]
1407aaf2f: c6808002000001               mov byte ptr [rax + 0x280], 1
1407aaf36: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aaf3d: 488d8d20030000               lea rcx, [rbp + 0x320]
1407aaf44: e8c72f1800                   call 0x14092df10
1407aaf49: 3c02                         cmp al, 2
1407aaf4b: 7510                         jne 0x1407aaf5d
1407aaf4d: 488b85e0030000               mov rax, qword ptr [rbp + 0x3e0]
1407aaf54: 833814                       cmp dword ptr [rax], 0x14
1407aaf57: 0f85730a0000                 jne 0x1407ab9d0
1407aaf5d: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407aaf64: 80781201                     cmp byte ptr [rax + 0x12], 1
1407aaf68: 0f845b220000                 je 0x1407ad1c9
1407aaf6e: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407aaf75: 80781001                     cmp byte ptr [rax + 0x10], 1
1407aaf79: 0f855c0a0000                 jne 0x1407ab9db
1407aaf7f: 488b85a0040000               mov rax, qword ptr [rbp + 0x4a0]
1407aaf86: 8b8d84060000                 mov ecx, dword ptr [rbp + 0x684]
1407aaf8c: 8908                         mov dword ptr [rax], ecx
1407aaf8e: 488b8500050000               mov rax, qword ptr [rbp + 0x500]
1407aaf95: c6808202000001               mov byte ptr [rax + 0x282], 1
1407aaf9c: e93a0a0000                   jmp 0x1407ab9db
1407aafa1: 0f57c0                       xorps xmm0, xmm0
1407aafa4: f3410f58442438               addss xmm0, dword ptr [r12 + 0x38]
1407aafab: f2410f104c2430               movsd xmm1, qword ptr [r12 + 0x30]
1407aafb2: 0f580d477c2d02               addps xmm1, xmmword ptr [rip + 0x22d7c47]
1407aafb9: 0f138d30050000               movlps qword ptr [rbp + 0x530], xmm1
1407aafc0: f30f118538050000             movss dword ptr [rbp + 0x538], xmm0
1407aafc8: f20f1005302d2302             movsd xmm0, qword ptr [rip + 0x2232d30]
1407aafd0: f20f11853c050000             movsd qword ptr [rbp + 0x53c], xmm0
1407aafd8: c7854405000000000080         mov dword ptr [rbp + 0x544], 0x80000000
1407aafe2: c744242002000000             mov dword ptr [rsp + 0x20], 2
1407aafea: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407aaff1: 4c8d8530050000               lea r8, [rbp + 0x530]
1407aaff8: f30f101d80a91702             movss xmm3, dword ptr [rip + 0x217a980]
1407ab000: 488d5570                     lea rdx, [rbp + 0x70]
1407ab004: e8d7121200                   call 0x1408cc2e0
1407ab009: 498b4c2410                   mov rcx, qword ptr [r12 + 0x10]
1407ab00e: 8b8514060000                 mov eax, dword ptr [rbp + 0x614]
1407ab014: 83f802                       cmp eax, 2
1407ab017: 0f8494020000                 je 0x1407ab2b1
1407ab01d: a801                         test al, 1
1407ab01f: 0f848c020000                 je 0x1407ab2b1
1407ab025: 80bde005000002               cmp byte ptr [rbp + 0x5e0], 2
1407ab02c: 7507                         jne 0x1407ab035
1407ab02e: 488b8de8050000               mov rcx, qword ptr [rbp + 0x5e8]
1407ab035: 48898dd8050000               mov qword ptr [rbp + 0x5d8], rcx
1407ab03c: f20f10b518060000             movsd xmm6, qword ptr [rbp + 0x618]
1407ab044: e976020000                   jmp 0x1407ab2bf
1407ab049: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab050: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407ab057: e894eb3400                   call 0x140af9bf0
1407ab05c: 84c0                         test al, al
1407ab05e: 0f8459210000                 je 0x1407ad1bd
1407ab064: 488b95b0050000               mov rdx, qword ptr [rbp + 0x5b0]
1407ab06b: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab072: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407ab079: e812381500                   call 0x1408fe890
1407ab07e: 83bde005000003               cmp dword ptr [rbp + 0x5e0], 3
1407ab085: 0f8432210000                 je 0x1407ad1bd
1407ab08b: 488b9500030000               mov rdx, qword ptr [rbp + 0x300]
1407ab092: 448b85f0050000               mov r8d, dword ptr [rbp + 0x5f0]
1407ab099: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab0a0: 488d8d30050000               lea rcx, [rbp + 0x530]
1407ab0a7: e8d4fa2600                   call 0x140a1ab80
1407ab0ac: f6853005000001               test byte ptr [rbp + 0x530], 1
1407ab0b3: 0f8404210000                 je 0x1407ad1bd
1407ab0b9: f30f108538050000             movss xmm0, dword ptr [rbp + 0x538]
1407ab0c1: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ab0c8: c70001000000                 mov dword ptr [rax], 1
1407ab0ce: f30f114004                   movss dword ptr [rax + 4], xmm0
1407ab0d3: e9e5200000                   jmp 0x1407ad1bd
1407ab0d8: 8bb5f0050000                 mov esi, dword ptr [rbp + 0x5f0]
1407ab0de: 8b85cc050000                 mov eax, dword ptr [rbp + 0x5cc]
1407ab0e4: 488b8dc0050000               mov rcx, qword ptr [rbp + 0x5c0]
1407ab0eb: 8901                         mov dword ptr [rcx], eax
1407ab0ed: 488b8db0050000               mov rcx, qword ptr [rbp + 0x5b0]
1407ab0f4: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407ab0fb: 448b4058                     mov r8d, dword ptr [rax + 0x58]
1407ab0ff: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab106: 89f2                         mov edx, esi
1407ab108: e8634c1500                   call 0x1408ffd70
1407ab10d: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407ab114: 8b5058                       mov edx, dword ptr [rax + 0x58]
1407ab117: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab11e: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1407ab125: e806182700                   call 0x140a1c930
1407ab12a: 4885c0                       test rax, rax
1407ab12d: 0f846c070000                 je 0x1407ab89f
1407ab133: 8b4840                       mov ecx, dword ptr [rax + 0x40]
1407ab136: 898da0020000                 mov dword ptr [rbp + 0x2a0], ecx
1407ab13c: 0f104030                     movups xmm0, xmmword ptr [rax + 0x30]
1407ab140: 0f298590020000               movaps xmmword ptr [rbp + 0x290], xmm0
1407ab147: 0f1000                       movups xmm0, xmmword ptr [rax]
1407ab14a: 0f104810                     movups xmm1, xmmword ptr [rax + 0x10]
1407ab14e: 0f298530050000               movaps xmmword ptr [rbp + 0x530], xmm0
1407ab155: 0f298d40050000               movaps xmmword ptr [rbp + 0x540], xmm1
1407ab15c: 488b4820                     mov rcx, qword ptr [rax + 0x20]
1407ab160: 48898d50050000               mov qword ptr [rbp + 0x550], rcx
1407ab167: 8b5044                       mov edx, dword ptr [rax + 0x44]
1407ab16a: 488b4028                     mov rax, qword ptr [rax + 0x28]
1407ab16e: 48898d00060000               mov qword ptr [rbp + 0x600], rcx
1407ab175: 0f298df0050000               movaps xmmword ptr [rbp + 0x5f0], xmm1
1407ab17c: 0f2985e0050000               movaps xmmword ptr [rbp + 0x5e0], xmm0
1407ab183: 48898508060000               mov qword ptr [rbp + 0x608], rax
1407ab18a: 0f288590020000               movaps xmm0, xmmword ptr [rbp + 0x290]
1407ab191: 0f298510060000               movaps xmmword ptr [rbp + 0x610], xmm0
1407ab198: 8b85a0020000                 mov eax, dword ptr [rbp + 0x2a0]
1407ab19e: 898520060000                 mov dword ptr [rbp + 0x620], eax
1407ab1a4: 899524060000                 mov dword ptr [rbp + 0x624], edx
1407ab1aa: 488b95b0050000               mov rdx, qword ptr [rbp + 0x5b0]
1407ab1b1: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab1b8: 488d8d40030000               lea rcx, [rbp + 0x340]
1407ab1bf: 4189f0                       mov r8d, esi
1407ab1c2: e8f9361500                   call 0x1408fe8c0
1407ab1c7: 889dd4060000                 mov byte ptr [rbp + 0x6d4], bl
1407ab1cd: 488b8540030000               mov rax, qword ptr [rbp + 0x340]
1407ab1d4: 488985b0060000               mov qword ptr [rbp + 0x6b0], rax
1407ab1db: 488b8548030000               mov rax, qword ptr [rbp + 0x348]
1407ab1e2: 488985d8060000               mov qword ptr [rbp + 0x6d8], rax
1407ab1e9: 488b9d50030000               mov rbx, qword ptr [rbp + 0x350]
1407ab1f0: 4885db                       test rbx, rbx
1407ab1f3: 0f847d060000                 je 0x1407ab876
1407ab1f9: 488db510060000               lea rsi, [rbp + 0x610]
1407ab200: 48c1e302                     shl rbx, 2
1407ab204: 31ff                         xor edi, edi
1407ab206: 4c8d3d937a2d02               lea r15, [rip + 0x22d7a93]
1407ab20d: 4c8d25a47a2d02               lea r12, [rip + 0x22d7aa4]
1407ab214: eb1b                         jmp 0x1407ab231
1407ab216: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1407ab220: 44887010                     mov byte ptr [rax + 0x10], r14b
1407ab224: 4883c704                     add rdi, 4
1407ab228: 4839fb                       cmp rbx, rdi
1407ab22b: 0f8445060000                 je 0x1407ab876
1407ab231: 488b85d8060000               mov rax, qword ptr [rbp + 0x6d8]
1407ab238: 8b1438                       mov edx, dword ptr [rax + rdi]
1407ab23b: 488b85a8050000               mov rax, qword ptr [rbp + 0x5a8]
1407ab242: 8b8dac060000                 mov ecx, dword ptr [rbp + 0x6ac]
1407ab248: 8908                         mov dword ptr [rax], ecx
1407ab24a: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1407ab251: e89a192700                   call 0x140a1cbf0
1407ab256: 4989c5                       mov r13, rax
1407ab259: 4885c0                       test rax, rax
1407ab25c: 74c6                         je 0x1407ab224
1407ab25e: 8b8508060000                 mov eax, dword ptr [rbp + 0x608]
1407ab264: 41894528                     mov dword ptr [r13 + 0x28], eax
1407ab268: 4889f1                       mov rcx, rsi
1407ab26b: 488d15fe792d02               lea rdx, [rip + 0x22d79fe]
1407ab272: e839e20800                   call 0x1408394b0
1407ab277: 448b7008                     mov r14d, dword ptr [rax + 8]
1407ab27b: 4983c530                     add r13, 0x30
1407ab27f: 4c89e9                       mov rcx, r13
1407ab282: 488d15ff792d02               lea rdx, [rip + 0x22d79ff]
1407ab289: e822e20800                   call 0x1408394b0
1407ab28e: 44897008                     mov dword ptr [rax + 8], r14d
1407ab292: 4889f1                       mov rcx, rsi
1407ab295: 4c89fa                       mov rdx, r15
1407ab298: e813e20800                   call 0x1408394b0
1407ab29d: 448a7010                     mov r14b, byte ptr [rax + 0x10]
1407ab2a1: 4c89e9                       mov rcx, r13
1407ab2a4: 4c89e2                       mov rdx, r12
1407ab2a7: e804e20800                   call 0x1408394b0
1407ab2ac: e96fffffff                   jmp 0x1407ab220
1407ab2b1: 48898dd8050000               mov qword ptr [rbp + 0x5d8], rcx
1407ab2b8: 0f283521a61702               movaps xmm6, xmmword ptr [rip + 0x217a621]
1407ab2bf: c685b806000002               mov byte ptr [rbp + 0x6b8], 2
1407ab2c6: 48c785a005000000000000       mov qword ptr [rbp + 0x5a0], 0
1407ab2d1: f3410f107c242c               movss xmm7, dword ptr [r12 + 0x2c]
1407ab2d8: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407ab2de: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407ab2e5: 8901                         mov dword ptr [rcx], eax
1407ab2e7: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407ab2ee: c6404801                     mov byte ptr [rax + 0x48], 1
1407ab2f2: 488b8598040000               mov rax, qword ptr [rbp + 0x498]
1407ab2f9: 8b8d80060000                 mov ecx, dword ptr [rbp + 0x680]
1407ab2ff: 8908                         mov dword ptr [rax], ecx
1407ab301: 488db530050000               lea rsi, [rbp + 0x530]
1407ab308: 4889f1                       mov rcx, rsi
1407ab30b: 488b9590040000               mov rdx, qword ptr [rbp + 0x490]
1407ab312: 41b001                       mov r8b, 1
1407ab315: e8a6860800                   call 0x1408339c0
1407ab31a: 4889f1                       mov rcx, rsi
1407ab31d: e8be860800                   call 0x1408339e0
1407ab322: 4189c0                       mov r8d, eax
1407ab325: 410fb644243c                 movzx eax, byte ptr [r12 + 0x3c]
1407ab32b: 04c8                         add al, 0xc8
1407ab32d: 3c08                         cmp al, 8
1407ab32f: 0f92c1                       setb cl
1407ab332: 3c06                         cmp al, 6
1407ab334: 0f95c0                       setne al
1407ab337: 84c1                         test cl, al
1407ab339: 0f85d6050000                 jne 0x1407ab915
1407ab33f: 488b85d8060000               mov rax, qword ptr [rbp + 0x6d8]
1407ab346: 488d34c0                     lea rsi, [rax + rax*8]
1407ab34a: 488d14db                     lea rdx, [rbx + rbx*8]
1407ab34e: 488b8550040000               mov rax, qword ptr [rbp + 0x450]
1407ab355: 8b8d9c050000                 mov ecx, dword ptr [rbp + 0x59c]
1407ab35b: 8908                         mov dword ptr [rax], ecx
1407ab35d: 488b8528050000               mov rax, qword ptr [rbp + 0x528]
1407ab364: 488b5830                     mov rbx, qword ptr [rax + 0x30]
1407ab368: 4c8b4840                     mov r9, qword ptr [rax + 0x40]
1407ab36c: 483b5820                     cmp rbx, qword ptr [rax + 0x20]
1407ab370: 753d                         jne 0x1407ab3af
1407ab372: 48899590060000               mov qword ptr [rbp + 0x690], rdx
1407ab379: 488d15006c4702               lea rdx, [rip + 0x2476c00]
1407ab380: 488b8d58040000               mov rcx, qword ptr [rbp + 0x458]
1407ab387: 448985d8060000               mov dword ptr [rbp + 0x6d8], r8d
1407ab38e: 4c898dd0050000               mov qword ptr [rbp + 0x5d0], r9
1407ab395: e8167adbff                   call 0x140562db0
1407ab39a: 488b9590060000               mov rdx, qword ptr [rbp + 0x690]
1407ab3a1: 4c8b8dd0050000               mov r9, qword ptr [rbp + 0x5d0]
1407ab3a8: 448b85d8060000               mov r8d, dword ptr [rbp + 0x6d8]
1407ab3af: 488d04f7                     lea rax, [rdi + rsi*8]
1407ab3b3: 488985d0050000               mov qword ptr [rbp + 0x5d0], rax
1407ab3ba: 498d04d6                     lea rax, [r14 + rdx*8]
1407ab3be: 48898590060000               mov qword ptr [rbp + 0x690], rax
1407ab3c5: 488b9528050000               mov rdx, qword ptr [rbp + 0x528]
1407ab3cc: 488b4228                     mov rax, qword ptr [rdx + 0x28]
1407ab3d0: 488d0c5b                     lea rcx, [rbx + rbx*2]
1407ab3d4: c704c803000000               mov dword ptr [rax + rcx*8], 3
1407ab3db: 448944c804                   mov dword ptr [rax + rcx*8 + 4], r8d
1407ab3e0: 4c894cc810                   mov qword ptr [rax + rcx*8 + 0x10], r9
1407ab3e5: 48ffc3                       inc rbx
1407ab3e8: 48895a30                     mov qword ptr [rdx + 0x30], rbx
1407ab3ec: 48ff4240                     inc qword ptr [rdx + 0x40]
1407ab3f0: 410fb65c243c                 movzx ebx, byte ptr [r12 + 0x3c]
1407ab3f6: 4489c6                       mov esi, r8d
1407ab3f9: e8720789ff                   call 0x14003bb70
1407ab3fe: 410f10442418                 movups xmm0, xmmword ptr [r12 + 0x18]
1407ab404: 488d9520060000               lea rdx, [rbp + 0x620]
1407ab40b: 0f1102                       movups xmmword ptr [rdx], xmm0
1407ab40e: 418b4c2428                   mov ecx, dword ptr [r12 + 0x28]
1407ab413: 894a10                       mov dword ptr [rdx + 0x10], ecx
1407ab416: 488b8da0050000               mov rcx, qword ptr [rbp + 0x5a0]
1407ab41d: 48898de8050000               mov qword ptr [rbp + 0x5e8], rcx
1407ab424: 488b8dd8050000               mov rcx, qword ptr [rbp + 0x5d8]
1407ab42b: 48898df0050000               mov qword ptr [rbp + 0x5f0], rcx
1407ab432: 0f13b5f8050000               movlps qword ptr [rbp + 0x5f8], xmm6
1407ab439: f30f11bd00060000             movss dword ptr [rbp + 0x600], xmm7
1407ab441: 0fb68db8060000               movzx ecx, byte ptr [rbp + 0x6b8]
1407ab448: 888d1a060000                 mov byte ptr [rbp + 0x61a], cl
1407ab44e: 89b534060000                 mov dword ptr [rbp + 0x634], esi
1407ab454: c7853806000000000000         mov dword ptr [rbp + 0x638], 0
1407ab45e: c6853c06000001               mov byte ptr [rbp + 0x63c], 1
1407ab465: 889d3d060000                 mov byte ptr [rbp + 0x63d], bl
1407ab46b: 88853e060000                 mov byte ptr [rbp + 0x63e], al
1407ab471: 48c785e005000003000000       mov qword ptr [rbp + 0x5e0], 3
1407ab47c: 418b442438                   mov eax, dword ptr [r12 + 0x38]
1407ab481: 898548030000                 mov dword ptr [rbp + 0x348], eax
1407ab487: 498b442430                   mov rax, qword ptr [r12 + 0x30]
1407ab48c: 48898540030000               mov qword ptr [rbp + 0x340], rax
1407ab493: c6854c03000000               mov byte ptr [rbp + 0x34c], 0
1407ab49a: 488b85a8050000               mov rax, qword ptr [rbp + 0x5a8]
1407ab4a1: 8b8dac060000                 mov ecx, dword ptr [rbp + 0x6ac]
1407ab4a7: 8908                         mov dword ptr [rax], ecx
1407ab4a9: 488b8520050000               mov rax, qword ptr [rbp + 0x520]
1407ab4b0: 8b8da8060000                 mov ecx, dword ptr [rbp + 0x6a8]
1407ab4b6: 8908                         mov dword ptr [rax], ecx
1407ab4b8: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407ab4be: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407ab4c5: 8901                         mov dword ptr [rcx], eax
1407ab4c7: 488b8518050000               mov rax, qword ptr [rbp + 0x518]
1407ab4ce: 4889442438                   mov qword ptr [rsp + 0x38], rax
1407ab4d3: 488d85c0010000               lea rax, [rbp + 0x1c0]
1407ab4da: 4889442430                   mov qword ptr [rsp + 0x30], rax
1407ab4df: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407ab4e6: 4889442428                   mov qword ptr [rsp + 0x28], rax
1407ab4eb: 488b8510050000               mov rax, qword ptr [rbp + 0x510]
1407ab4f2: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407ab4f7: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407ab4fe: 4c8d8540030000               lea r8, [rbp + 0x340]
1407ab505: 31d2                         xor edx, edx
1407ab507: 4c8b8dc0060000               mov r9, qword ptr [rbp + 0x6c0]
1407ab50e: e81db9ac00                   call 0x141276e30
1407ab513: 0f57ff                       xorps xmm7, xmm7
1407ab516: f2440f1005e1762d02           movsd xmm8, qword ptr [rip + 0x22d76e1]
1407ab51f: f2440f1015d8272302           movsd xmm10, qword ptr [rip + 0x22327d8]
1407ab528: f30f103550a41702             movss xmm6, dword ptr [rip + 0x217a450]
1407ab530: 440f2825a8a31702             movaps xmm12, xmmword ptr [rip + 0x217a3a8]
1407ab538: e94a010000                   jmp 0x1407ab687
1407ab53d: 0f1f00                       nop dword ptr [rax]
1407ab540: 488b9528050000               mov rdx, qword ptr [rbp + 0x528]
1407ab547: 488b4228                     mov rax, qword ptr [rdx + 0x28]
1407ab54b: 4b8d0c64                     lea rcx, [r12 + r12*2]
1407ab54f: c704c803000000               mov dword ptr [rax + rcx*8], 3
1407ab556: 44896cc804                   mov dword ptr [rax + rcx*8 + 4], r13d
1407ab55b: 48897cc810                   mov qword ptr [rax + rcx*8 + 0x10], rdi
1407ab560: 49ffc4                       inc r12
1407ab563: 4c896230                     mov qword ptr [rdx + 0x30], r12
1407ab567: 48ff4240                     inc qword ptr [rdx + 0x40]
1407ab56b: 410fb67f3c                   movzx edi, byte ptr [r15 + 0x3c]
1407ab570: e8fb0589ff                   call 0x14003bb70
1407ab575: 410f104718                   movups xmm0, xmmword ptr [r15 + 0x18]
1407ab57a: 488d9520060000               lea rdx, [rbp + 0x620]
1407ab581: 0f1102                       movups xmmword ptr [rdx], xmm0
1407ab584: 418b4f28                     mov ecx, dword ptr [r15 + 0x28]
1407ab588: 894a10                       mov dword ptr [rdx + 0x10], ecx
1407ab58b: 4c89b5e8050000               mov qword ptr [rbp + 0x5e8], r14
1407ab592: 488b8db8060000               mov rcx, qword ptr [rbp + 0x6b8]
1407ab599: 48898df0050000               mov qword ptr [rbp + 0x5f0], rcx
1407ab5a0: 440f13adf8050000             movlps qword ptr [rbp + 0x5f8], xmm13
1407ab5a8: f3440f11b500060000           movss dword ptr [rbp + 0x600], xmm14
1407ab5b1: 4088b51a060000               mov byte ptr [rbp + 0x61a], sil
1407ab5b8: 4489ad34060000               mov dword ptr [rbp + 0x634], r13d
1407ab5bf: c7853806000000000000         mov dword ptr [rbp + 0x638], 0
1407ab5c9: c6853c06000001               mov byte ptr [rbp + 0x63c], 1
1407ab5d0: 4088bd3d060000               mov byte ptr [rbp + 0x63d], dil
1407ab5d7: 88853e060000                 mov byte ptr [rbp + 0x63e], al
1407ab5dd: 48c785e005000003000000       mov qword ptr [rbp + 0x5e0], 3
1407ab5e8: 418b4738                     mov eax, dword ptr [r15 + 0x38]
1407ab5ec: 898548030000                 mov dword ptr [rbp + 0x348], eax
1407ab5f2: 498b4730                     mov rax, qword ptr [r15 + 0x30]
1407ab5f6: 48898540030000               mov qword ptr [rbp + 0x340], rax
1407ab5fd: c6854c03000000               mov byte ptr [rbp + 0x34c], 0
1407ab604: 488b85a8050000               mov rax, qword ptr [rbp + 0x5a8]
1407ab60b: 8b8dac060000                 mov ecx, dword ptr [rbp + 0x6ac]
1407ab611: 8908                         mov dword ptr [rax], ecx
1407ab613: 488b8520050000               mov rax, qword ptr [rbp + 0x520]
1407ab61a: 8b8da8060000                 mov ecx, dword ptr [rbp + 0x6a8]
1407ab620: 8908                         mov dword ptr [rax], ecx
1407ab622: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407ab628: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407ab62f: 8901                         mov dword ptr [rcx], eax
1407ab631: 488b8518050000               mov rax, qword ptr [rbp + 0x518]
1407ab638: 4889442438                   mov qword ptr [rsp + 0x38], rax
1407ab63d: 488d85c0010000               lea rax, [rbp + 0x1c0]
1407ab644: 4889442430                   mov qword ptr [rsp + 0x30], rax
1407ab649: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407ab650: 4889442428                   mov qword ptr [rsp + 0x28], rax
1407ab655: 488b8510050000               mov rax, qword ptr [rbp + 0x510]
1407ab65c: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407ab661: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407ab668: 31d2                         xor edx, edx
1407ab66a: 4c8d8540030000               lea r8, [rbp + 0x340]
1407ab671: 4c8b8dc0060000               mov r9, qword ptr [rbp + 0x6c0]
1407ab678: e8b3b7ac00                   call 0x141276e30
1407ab67d: 4989df                       mov r15, rbx
1407ab680: 4c8badd8060000               mov r13, qword ptr [rbp + 0x6d8]
1407ab687: 4d85ff                       test r15, r15
1407ab68a: 7444                         je 0x1407ab6d0
1407ab68c: 4c3bbdd0050000               cmp r15, qword ptr [rbp + 0x5d0]
1407ab693: 743b                         je 0x1407ab6d0
1407ab695: 498d5f48                     lea rbx, [r15 + 0x48]
1407ab699: 4c89ee                       mov rsi, r13
1407ab69c: 488b85e0060000               mov rax, qword ptr [rbp + 0x6e0]
1407ab6a3: 48ff00                       inc qword ptr [rax]
1407ab6a6: 41833f00                     cmp dword ptr [r15], 0
1407ab6aa: 7456                         je 0x1407ab702
1407ab6ac: f3450f106f30                 movss xmm13, dword ptr [r15 + 0x30]
1407ab6b2: f3410f104738                 movss xmm0, dword ptr [r15 + 0x38]
1407ab6b8: 440f14e8                     unpcklps xmm13, xmm0
1407ab6bc: 41b403                       mov r12b, 3
1407ab6bf: 41be01000000                 mov r14d, 1
1407ab6c5: e9f3000000                   jmp 0x1407ab7bd
1407ab6ca: 660f1f440000                 nop word ptr [rax + rax]
1407ab6d0: 41b201                       mov r10b, 1
1407ab6d3: 4d85ed                       test r13, r13
1407ab6d6: 0f8477ebffff                 je 0x1407aa253
1407ab6dc: 4c3bad90060000               cmp r13, qword ptr [rbp + 0x690]
1407ab6e3: 0f846aebffff                 je 0x1407aa253
1407ab6e9: 498d7548                     lea rsi, [r13 + 0x48]
1407ab6ed: 31db                         xor ebx, ebx
1407ab6ef: 4d89ef                       mov r15, r13
1407ab6f2: 488b85e0060000               mov rax, qword ptr [rbp + 0x6e0]
1407ab6f9: 48ff00                       inc qword ptr [rax]
1407ab6fc: 41833f00                     cmp dword ptr [r15], 0
1407ab700: 75aa                         jne 0x1407ab6ac
1407ab702: 41f6470401                   test byte ptr [r15 + 4], 1
1407ab707: 7416                         je 0x1407ab71f
1407ab709: 498b4710                     mov rax, qword ptr [r15 + 0x10]
1407ab70d: 488985b8060000               mov qword ptr [rbp + 0x6b8], rax
1407ab714: f2450f106f08                 movsd xmm13, qword ptr [r15 + 8]
1407ab71a: e998000000                   jmp 0x1407ab7b7
1407ab71f: f3410f104738                 movss xmm0, dword ptr [r15 + 0x38]
1407ab725: f30f58c7                     addss xmm0, xmm7
1407ab729: f2410f104f30                 movsd xmm1, qword ptr [r15 + 0x30]
1407ab72f: 410f58c8                     addps xmm1, xmm8
1407ab733: 0f138d30050000               movlps qword ptr [rbp + 0x530], xmm1
1407ab73a: f30f118538050000             movss dword ptr [rbp + 0x538], xmm0
1407ab742: f2440f11953c050000           movsd qword ptr [rbp + 0x53c], xmm10
1407ab74b: c7854405000000000080         mov dword ptr [rbp + 0x544], 0x80000000
1407ab755: c744242002000000             mov dword ptr [rsp + 0x20], 2
1407ab75d: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407ab764: 488d5570                     lea rdx, [rbp + 0x70]
1407ab768: 4c8d8530050000               lea r8, [rbp + 0x530]
1407ab76f: 0f28de                       movaps xmm3, xmm6
1407ab772: e8690b1200                   call 0x1408cc2e0
1407ab777: 498b4f10                     mov rcx, qword ptr [r15 + 0x10]
1407ab77b: 8b8514060000                 mov eax, dword ptr [rbp + 0x614]
1407ab781: 83f802                       cmp eax, 2
1407ab784: 7426                         je 0x1407ab7ac
1407ab786: a801                         test al, 1
1407ab788: 7422                         je 0x1407ab7ac
1407ab78a: 80bde005000002               cmp byte ptr [rbp + 0x5e0], 2
1407ab791: 7507                         jne 0x1407ab79a
1407ab793: 488b8de8050000               mov rcx, qword ptr [rbp + 0x5e8]
1407ab79a: 48898db8060000               mov qword ptr [rbp + 0x6b8], rcx
1407ab7a1: f2440f10ad18060000           movsd xmm13, qword ptr [rbp + 0x618]
1407ab7aa: eb0b                         jmp 0x1407ab7b7
1407ab7ac: 48898db8060000               mov qword ptr [rbp + 0x6b8], rcx
1407ab7b3: 450f28ec                     movaps xmm13, xmm12
1407ab7b7: 41b402                       mov r12b, 2
1407ab7ba: 4531f6                       xor r14d, r14d
1407ab7bd: f3450f10772c                 movss xmm14, dword ptr [r15 + 0x2c]
1407ab7c3: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407ab7c9: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407ab7d0: 8901                         mov dword ptr [rcx], eax
1407ab7d2: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407ab7d9: c6404801                     mov byte ptr [rax + 0x48], 1
1407ab7dd: 488b8598040000               mov rax, qword ptr [rbp + 0x498]
1407ab7e4: 8b8d80060000                 mov ecx, dword ptr [rbp + 0x680]
1407ab7ea: 8908                         mov dword ptr [rax], ecx
1407ab7ec: 488dbd30050000               lea rdi, [rbp + 0x530]
1407ab7f3: 4889f9                       mov rcx, rdi
1407ab7f6: 488b9590040000               mov rdx, qword ptr [rbp + 0x490]
1407ab7fd: 41b001                       mov r8b, 1
1407ab800: e8bb810800                   call 0x1408339c0
1407ab805: 4889f9                       mov rcx, rdi
1407ab808: e8d3810800                   call 0x1408339e0
1407ab80d: 4189c5                       mov r13d, eax
1407ab810: 410fb6473c                   movzx eax, byte ptr [r15 + 0x3c]
1407ab815: 04c8                         add al, 0xc8
1407ab817: 3c08                         cmp al, 8
1407ab819: 0f92c1                       setb cl
1407ab81c: 3c06                         cmp al, 6
1407ab81e: 0f95c0                       setne al
1407ab821: 84c1                         test cl, al
1407ab823: 0f85ec000000                 jne 0x1407ab915
1407ab829: 4889f0                       mov rax, rsi
1407ab82c: 4489e6                       mov esi, r12d
1407ab82f: 488985d8060000               mov qword ptr [rbp + 0x6d8], rax
1407ab836: 488b8550040000               mov rax, qword ptr [rbp + 0x450]
1407ab83d: 8b8d9c050000                 mov ecx, dword ptr [rbp + 0x59c]
1407ab843: 8908                         mov dword ptr [rax], ecx
1407ab845: 488b8528050000               mov rax, qword ptr [rbp + 0x528]
1407ab84c: 4c8b6030                     mov r12, qword ptr [rax + 0x30]
1407ab850: 488b7840                     mov rdi, qword ptr [rax + 0x40]
1407ab854: 4c3b6020                     cmp r12, qword ptr [rax + 0x20]
1407ab858: 0f85e2fcffff                 jne 0x1407ab540
1407ab85e: 488b8d58040000               mov rcx, qword ptr [rbp + 0x458]
1407ab865: 488d1514674702               lea rdx, [rip + 0x2476714]
1407ab86c: e83f75dbff                   call 0x140562db0
1407ab871: e9cafcffff                   jmp 0x1407ab540
1407ab876: 488b95b0060000               mov rdx, qword ptr [rbp + 0x6b0]
1407ab87d: 4885d2                       test rdx, rdx
1407ab880: 7416                         je 0x1407ab898
1407ab882: 48c1e202                     shl rdx, 2
1407ab886: 41b804000000                 mov r8d, 4
1407ab88c: 488b8dd8060000               mov rcx, qword ptr [rbp + 0x6d8]
1407ab893: e88883e6ff                   call 0x140613c20
1407ab898: 0fb69dd4060000               movzx ebx, byte ptr [rbp + 0x6d4]
1407ab89f: 80fb02                       cmp bl, 2
1407ab8a2: 488b8de8060000               mov rcx, qword ptr [rbp + 0x6e8]
1407ab8a9: 7213                         jb 0x1407ab8be
1407ab8ab: 40b701                       mov dil, 1
1407ab8ae: 488b85d0030000               mov rax, qword ptr [rbp + 0x3d0]
1407ab8b5: 803800                       cmp byte ptr [rax], 0
1407ab8b8: 0f8501190000                 jne 0x1407ad1bf
1407ab8be: 8b5158                       mov edx, dword ptr [rcx + 0x58]
1407ab8c1: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab8c8: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1407ab8cf: e85c102700                   call 0x140a1c930
1407ab8d4: 4885c0                       test rax, rax
1407ab8d7: 7454                         je 0x1407ab92d
1407ab8d9: 4883c030                     add rax, 0x30
1407ab8dd: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab8e4: 4889c1                       mov rcx, rax
1407ab8e7: e8a4db0800                   call 0x140839490
1407ab8ec: 4889c6                       mov rsi, rax
1407ab8ef: eb3e                         jmp 0x1407ab92f
1407ab8f1: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407ab8f8: 488d0df9752d02               lea rcx, [rip + 0x22d75f9]
1407ab8ff: 4c8d050a762d02               lea r8, [rip + 0x22d760a]
1407ab906: ba13000000                   mov edx, 0x13
1407ab90b: e820db1202                   call 0x1428d9430
1407ab910: e9461a0000                   jmp 0x1407ad35b
1407ab915: 488d0ddc752d02               lea rcx, [rip + 0x22d75dc]
1407ab91c: 4c8d0505762d02               lea r8, [rip + 0x22d7605]
1407ab923: ba13000000                   mov edx, 0x13
1407ab928: e803db1202                   call 0x1428d9430
1407ab92d: 31f6                         xor esi, esi
1407ab92f: 488b9500030000               mov rdx, qword ptr [rbp + 0x300]
1407ab936: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407ab93d: 448b4058                     mov r8d, dword ptr [rax + 0x58]
1407ab941: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab948: 488d8d58020000               lea rcx, [rbp + 0x258]
1407ab94f: e82cf22600                   call 0x140a1ab80
1407ab954: 40b701                       mov dil, 1
1407ab957: 4885f6                       test rsi, rsi
1407ab95a: 0f8420f4ffff                 je 0x1407aad80
1407ab960: 83bd5802000001               cmp dword ptr [rbp + 0x258], 1
1407ab967: 0f8513f4ffff                 jne 0x1407aad80
1407ab96d: 31c0                         xor eax, eax
1407ab96f: 80fb02                       cmp bl, 2
1407ab972: 8b8d64020000                 mov ecx, dword ptr [rbp + 0x264]
1407ab978: 898dfc050000                 mov dword ptr [rbp + 0x5fc], ecx
1407ab97e: 488b8d5c020000               mov rcx, qword ptr [rbp + 0x25c]
1407ab985: 48898df4050000               mov qword ptr [rbp + 0x5f4], rcx
1407ab98c: 1405                         adc al, 5
1407ab98e: 8885e0050000                 mov byte ptr [rbp + 0x5e0], al
1407ab994: c6850806000000               mov byte ptr [rbp + 0x608], 0
1407ab99b: f20f1006                     movsd xmm0, qword ptr [rsi]
1407ab99f: 0fc6c0e1                     shufps xmm0, xmm0, 0xe1
1407ab9a3: 0f138500060000               movlps qword ptr [rbp + 0x600], xmm0
1407ab9aa: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab9b1: 4c8d0518732d02               lea r8, [rip + 0x22d7318]
1407ab9b8: 488d8d30010000               lea rcx, [rbp + 0x130]
1407ab9bf: 488d95e0050000               lea rdx, [rbp + 0x5e0]
1407ab9c6: e8552cab00                   call 0x14125e620
1407ab9cb: e9b0f3ffff                   jmp 0x1407aad80
1407ab9d0: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ab9d7: c6401200                     mov byte ptr [rax + 0x12], 0
1407ab9db: 4180fd02                     cmp r13b, 2
1407ab9df: 410f92c0                     setb r8b
1407ab9e3: 488b8598040000               mov rax, qword ptr [rbp + 0x498]
1407ab9ea: 8b8d80060000                 mov ecx, dword ptr [rbp + 0x680]
1407ab9f0: 8908                         mov dword ptr [rax], ecx
1407ab9f2: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ab9f9: 488d8d18010000               lea rcx, [rbp + 0x118]
1407aba00: 488b9590040000               mov rdx, qword ptr [rbp + 0x490]
1407aba07: e8b47f0800                   call 0x1408339c0
1407aba0c: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aba13: 488d8d18010000               lea rcx, [rbp + 0x118]
1407aba1a: e8c17f0800                   call 0x1408339e0
1407aba1f: 8985d8060000                 mov dword ptr [rbp + 0x6d8], eax
1407aba25: 8b85d8060000                 mov eax, dword ptr [rbp + 0x6d8]
1407aba2b: 8985f4040000                 mov dword ptr [rbp + 0x4f4], eax
1407aba31: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407aba38: 408a7869                     mov dil, byte ptr [rax + 0x69]
1407aba3c: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aba43: e8280189ff                   call 0x14003bb70
1407aba48: 88c3                         mov bl, al
1407aba4a: 488b054706e802               mov rax, qword ptr [rip + 0x2e80647]
1407aba51: 4531ff                       xor r15d, r15d
1407aba54: 803800                       cmp byte ptr [rax], 0
1407aba57: 410f94c4                     sete r12b
1407aba5b: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aba62: e8096985ff                   call 0x140002370
1407aba67: 4088c6                       mov sil, al
1407aba6a: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aba71: e8fa6885ff                   call 0x140002370
1407aba76: 4588e7                       mov r15b, r12b
1407aba79: 438d0cbf                     lea ecx, [r15 + r15*4]
1407aba7d: 48c7859002000000000000       mov qword ptr [rbp + 0x290], 0
1407aba88: 48898598020000               mov qword ptr [rbp + 0x298], rax
1407aba8f: 0f57c0                       xorps xmm0, xmm0
1407aba92: 0f1185a0020000               movups xmmword ptr [rbp + 0x2a0], xmm0
1407aba99: 4088b5b0020000               mov byte ptr [rbp + 0x2b0], sil
1407abaa0: 66c785b10200000001           mov word ptr [rbp + 0x2b1], 0x100
1407abaa9: 4088bdd6020000               mov byte ptr [rbp + 0x2d6], dil
1407abab0: c685d702000000               mov byte ptr [rbp + 0x2d7], 0
1407abab7: 889dd4020000                 mov byte ptr [rbp + 0x2d4], bl
1407ababd: 48c785b802000000000000       mov qword ptr [rbp + 0x2b8], 0
1407abac8: 888dd5020000                 mov byte ptr [rbp + 0x2d5], cl
1407abace: c685d102000002               mov byte ptr [rbp + 0x2d1], 2
1407abad5: 488b5530                     mov rdx, qword ptr [rbp + 0x30]
1407abad9: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abae0: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407abae7: e8e4621600                   call 0x140911dd0
1407abaec: f30f1085e0050000             movss xmm0, dword ptr [rbp + 0x5e0]
1407abaf4: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407abafb: f30f5c4038                   subss xmm0, dword ptr [rax + 0x38]
1407abb00: f30f108de8050000             movss xmm1, dword ptr [rbp + 0x5e8]
1407abb08: f30f5c4840                   subss xmm1, dword ptr [rax + 0x40]
1407abb0d: 0f28d0                       movaps xmm2, xmm0
1407abb10: f30f59d0                     mulss xmm2, xmm0
1407abb14: 0f28d9                       movaps xmm3, xmm1
1407abb17: f30f59d9                     mulss xmm3, xmm1
1407abb1b: f30f58da                     addss xmm3, xmm2
1407abb1f: 0f57d2                       xorps xmm2, xmm2
1407abb22: f30f51d3                     sqrtss xmm2, xmm3
1407abb26: f30f5ef2                     divss xmm6, xmm2
1407abb2a: f30f59c6                     mulss xmm0, xmm6
1407abb2e: f30f59f1                     mulss xmm6, xmm1
1407abb32: f30f594044                   mulss xmm0, dword ptr [rax + 0x44]
1407abb37: f30f59704c                   mulss xmm6, dword ptr [rax + 0x4c]
1407abb3c: f30f58f0                     addss xmm6, xmm0
1407abb40: 0f57c0                       xorps xmm0, xmm0
1407abb43: 0f2ec6                       ucomiss xmm0, xmm6
1407abb46: 0f93c1                       setae cl
1407abb49: 898de0060000                 mov dword ptr [rbp + 0x6e0], ecx
1407abb4f: 0fb64069                     movzx eax, byte ptr [rax + 0x69]
1407abb53: 04c8                         add al, 0xc8
1407abb55: 3c08                         cmp al, 8
1407abb57: 0fb6c0                       movzx eax, al
1407abb5a: b906000000                   mov ecx, 6
1407abb5f: 0f42c8                       cmovb ecx, eax
1407abb62: 0fb6c1                       movzx eax, cl
1407abb65: 488d0ddc752d02               lea rcx, [rip + 0x22d75dc]
1407abb6c: 48630481                     movsxd rax, dword ptr [rcx + rax*4]
1407abb70: 4801c8                       add rax, rcx
1407abb73: ffe0                         jmp rax
1407abb75: 488b85a0060000               mov rax, qword ptr [rbp + 0x6a0]
1407abb7c: 8b5030                       mov edx, dword ptr [rax + 0x30]
1407abb7f: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abb86: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407abb8d: e8aed90800                   call 0x140839540
1407abb92: e91f010000                   jmp 0x1407abcb6
1407abb97: 488b85a0060000               mov rax, qword ptr [rbp + 0x6a0]
1407abb9e: f30f10483c                   movss xmm1, dword ptr [rax + 0x3c]
1407abba3: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abbaa: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407abbb1: e89ad90800                   call 0x140839550
1407abbb6: e9fb000000                   jmp 0x1407abcb6
1407abbbb: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abbc2: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407abbc9: 0f57c9                       xorps xmm1, xmm1
1407abbcc: 41b001                       mov r8b, 1
1407abbcf: e84cd90800                   call 0x140839520
1407abbd4: e9dd000000                   jmp 0x1407abcb6
1407abbd9: 488b95b0050000               mov rdx, qword ptr [rbp + 0x5b0]
1407abbe0: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abbe7: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407abbee: e89d2c1500                   call 0x1408fe890
1407abbf3: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abbfa: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407abc01: 488b95c0060000               mov rdx, qword ptr [rbp + 0x6c0]
1407abc08: e813dcffff                   call 0x1407a9820
1407abc0d: 4885c0                       test rax, rax
1407abc10: 7432                         je 0x1407abc44
1407abc12: f30f104004                   movss xmm0, dword ptr [rax + 4]
1407abc17: 488d5008                     lea rdx, [rax + 8]
1407abc1b: 4c8d4010                     lea r8, [rax + 0x10]
1407abc1f: 8b480c                       mov ecx, dword ptr [rax + 0xc]
1407abc22: eb37                         jmp 0x1407abc5b
1407abc24: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abc2b: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407abc32: b201                         mov dl, 1
1407abc34: e8f7d80800                   call 0x140839530
1407abc39: eb7b                         jmp 0x1407abcb6
1407abc3b: c685f104000003               mov byte ptr [rbp + 0x4f1], 3
1407abc42: eb72                         jmp 0x1407abcb6
1407abc44: 488b8da0060000               mov rcx, qword ptr [rbp + 0x6a0]
1407abc4b: 488d510c                     lea rdx, [rcx + 0xc]
1407abc4f: 4c8d4114                     lea r8, [rcx + 0x14]
1407abc53: f30f1005759c1702             movss xmm0, dword ptr [rip + 0x2179c75]
1407abc5b: 8b12                         mov edx, dword ptr [rdx]
1407abc5d: 450fb600                     movzx r8d, byte ptr [r8]
1407abc61: 4885c0                       test rax, rax
1407abc64: 750a                         jne 0x1407abc70
1407abc66: 488b85a0060000               mov rax, qword ptr [rbp + 0x6a0]
1407abc6d: 8b4810                       mov ecx, dword ptr [rax + 0x10]
1407abc70: c785e005000000000000         mov dword ptr [rbp + 0x5e0], 0
1407abc7a: f30f1185e4050000             movss dword ptr [rbp + 0x5e4], xmm0
1407abc82: 8995e8050000                 mov dword ptr [rbp + 0x5e8], edx
1407abc88: 448885f0050000               mov byte ptr [rbp + 0x5f0], r8b
1407abc8f: 898dec050000                 mov dword ptr [rbp + 0x5ec], ecx
1407abc95: c685f105000000               mov byte ptr [rbp + 0x5f1], 0
1407abc9c: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abca3: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407abcaa: 488d95e0050000               lea rdx, [rbp + 0x5e0]
1407abcb1: e8aad80800                   call 0x140839560
1407abcb6: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407abcbd: 4c8d4820                     lea r9, [rax + 0x20]
1407abcc1: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abcc8: 488d8d30050000               lea rcx, [rbp + 0x530]
1407abccf: 488d95f4040000               lea rdx, [rbp + 0x4f4]
1407abcd6: 4c8d8590020000               lea r8, [rbp + 0x290]
1407abcdd: e8ae0cad00                   call 0x14127c990
1407abce2: 4c8b85e8060000               mov r8, qword ptr [rbp + 0x6e8]
1407abce9: 498d4838                     lea rcx, [r8 + 0x38]
1407abced: 0fb6b530050000               movzx esi, byte ptr [rbp + 0x530]
1407abcf4: 83fe04                       cmp esi, 4
1407abcf7: 7512                         jne 0x1407abd0b
1407abcf9: 4889cb                       mov rbx, rcx
1407abcfc: c7852c04000002000000         mov dword ptr [rbp + 0x42c], 2
1407abd06: e979010000                   jmp 0x1407abe84
1407abd0b: 8b4108                       mov eax, dword ptr [rcx + 8]
1407abd0e: 8985c8040000                 mov dword ptr [rbp + 0x4c8], eax
1407abd14: 4889cb                       mov rbx, rcx
1407abd17: 488b01                       mov rax, qword ptr [rcx]
1407abd1a: 488985c0040000               mov qword ptr [rbp + 0x4c0], rax
1407abd21: 418d46c8                     lea eax, [r14 - 0x38]
1407abd25: 3c08                         cmp al, 8
1407abd27: 0f93c1                       setae cl
1407abd2a: 3c06                         cmp al, 6
1407abd2c: 0f94c2                       sete dl
1407abd2f: 08ca                         or dl, cl
1407abd31: 0f848e000000                 je 0x1407abdc5
1407abd37: 4d8b38                       mov r15, qword ptr [r8]
1407abd3a: 410f287010                   movaps xmm6, xmmword ptr [r8 + 0x10]
1407abd3f: 488b5530                     mov rdx, qword ptr [rbp + 0x30]
1407abd43: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abd4a: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407abd51: e8ba601600                   call 0x140911e10
1407abd56: 41f6c701                     test r15b, 1
1407abd5a: 7507                         jne 0x1407abd63
1407abd5c: 0f28355d231902               movaps xmm6, xmmword ptr [rip + 0x219235d]
1407abd63: 0f2885e0050000               movaps xmm0, xmmword ptr [rbp + 0x5e0]
1407abd6a: 0f28ce                       movaps xmm1, xmm6
1407abd6d: 0fc6ce00                     shufps xmm1, xmm6, 0
1407abd71: 0f28d6                       movaps xmm2, xmm6
1407abd74: 0fc6d655                     shufps xmm2, xmm6, 0x55
1407abd78: 0f28de                       movaps xmm3, xmm6
1407abd7b: 0fc6deaa                     shufps xmm3, xmm6, 0xaa
1407abd7f: 0fc6f6ff                     shufps xmm6, xmm6, 0xff
1407abd83: 0f59f0                       mulps xmm6, xmm0
1407abd86: 0f28e0                       movaps xmm4, xmm0
1407abd89: 0fc6e01b                     shufps xmm4, xmm0, 0x1b
1407abd8d: 0f59cc                       mulps xmm1, xmm4
1407abd90: 0fc6e4b1                     shufps xmm4, xmm4, 0xb1
1407abd94: 0f59e2                       mulps xmm4, xmm2
1407abd97: 0f590dd2132202               mulps xmm1, xmmword ptr [rip + 0x22213d2]
1407abd9e: 0fc6c0b1                     shufps xmm0, xmm0, 0xb1
1407abda2: 0f59c3                       mulps xmm0, xmm3
1407abda5: 0f5925d4132202               mulps xmm4, xmmword ptr [rip + 0x22213d4]
1407abdac: 0f5905dd132202               mulps xmm0, xmmword ptr [rip + 0x22213dd]
1407abdb3: 0f58ce                       addps xmm1, xmm6
1407abdb6: 0f58c4                       addps xmm0, xmm4
1407abdb9: 0f58c1                       addps xmm0, xmm1
1407abdbc: 0f2985e0050000               movaps xmmword ptr [rbp + 0x5e0], xmm0
1407abdc3: eb58                         jmp 0x1407abe1d
1407abdc5: 83fe01                       cmp esi, 1
1407abdc8: 741e                         je 0x1407abde8
1407abdca: 83fe02                       cmp esi, 2
1407abdcd: 751d                         jne 0x1407abdec
1407abdcf: 488b5530                     mov rdx, qword ptr [rbp + 0x30]
1407abdd3: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abdda: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407abde1: e82a601600                   call 0x140911e10
1407abde6: eb35                         jmp 0x1407abe1d
1407abde8: 3c07                         cmp al, 7
1407abdea: 74e3                         je 0x1407abdcf
1407abdec: 498d4044                     lea rax, [r8 + 0x44]
1407abdf0: 8b4808                       mov ecx, dword ptr [rax + 8]
1407abdf3: 898d48030000                 mov dword ptr [rbp + 0x348], ecx
1407abdf9: 488b00                       mov rax, qword ptr [rax]
1407abdfc: 48898540030000               mov qword ptr [rbp + 0x340], rax
1407abe03: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abe0a: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407abe11: 488d9540030000               lea rdx, [rbp + 0x340]
1407abe18: e823584e00                   call 0x140c91640
1407abe1d: 488d8531050000               lea rax, [rbp + 0x531]
1407abe24: 4c8b85e8060000               mov r8, qword ptr [rbp + 0x6e8]
1407abe2b: 418b4858                     mov ecx, dword ptr [r8 + 0x58]
1407abe2f: 898d34040000                 mov dword ptr [rbp + 0x434], ecx
1407abe35: 498b4850                     mov rcx, qword ptr [r8 + 0x50]
1407abe39: 48898d2c040000               mov qword ptr [rbp + 0x42c], rcx
1407abe40: 488b8dc0040000               mov rcx, qword ptr [rbp + 0x4c0]
1407abe47: 48898d20040000               mov qword ptr [rbp + 0x420], rcx
1407abe4e: 8b8dc8040000                 mov ecx, dword ptr [rbp + 0x4c8]
1407abe54: 898d28040000                 mov dword ptr [rbp + 0x428], ecx
1407abe5a: 0f2885e0050000               movaps xmm0, xmmword ptr [rbp + 0x5e0]
1407abe61: 488b08                       mov rcx, qword ptr [rax]
1407abe64: 488b4007                     mov rax, qword ptr [rax + 7]
1407abe68: 48898d11040000               mov qword ptr [rbp + 0x411], rcx
1407abe6f: 48898518040000               mov qword ptr [rbp + 0x418], rax
1407abe76: 0f298500040000               movaps xmmword ptr [rbp + 0x400], xmm0
1407abe7d: 4088b510040000               mov byte ptr [rbp + 0x410], sil
1407abe84: 418078693f                   cmp byte ptr [r8 + 0x69], 0x3f
1407abe89: 0f8534020000                 jne 0x1407ac0c3
1407abe8f: 488b95b0050000               mov rdx, qword ptr [rbp + 0x5b0]
1407abe96: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abe9d: 488d8d30050000               lea rcx, [rbp + 0x530]
1407abea4: e8e7291500                   call 0x1408fe890
1407abea9: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abeb0: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407abeb7: 488d9530050000               lea rdx, [rbp + 0x530]
1407abebe: 4c8b85c0060000               mov r8, qword ptr [rbp + 0x6c0]
1407abec5: e8b6daffff                   call 0x1407a9980
1407abeca: 83bde005000003               cmp dword ptr [rbp + 0x5e0], 3
1407abed1: 0f84e5010000                 je 0x1407ac0bc
1407abed7: 448bbdf0050000               mov r15d, dword ptr [rbp + 0x5f0]
1407abede: 488bb5f8050000               mov rsi, qword ptr [rbp + 0x5f8]
1407abee5: 807e2202                     cmp byte ptr [rsi + 0x22], 2
1407abee9: 7348                         jae 0x1407abf33
1407abeeb: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407abef2: c6401101                     mov byte ptr [rax + 0x11], 1
1407abef6: 488b9500030000               mov rdx, qword ptr [rbp + 0x300]
1407abefd: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abf04: 4c8d0ddd6d2d02               lea r9, [rip + 0x22d6ddd]
1407abf0b: 488d8d30050000               lea rcx, [rbp + 0x530]
1407abf12: 4589f8                       mov r8d, r15d
1407abf15: e8d6ea2600                   call 0x140a1a9f0
1407abf1a: f30f108534050000             movss xmm0, dword ptr [rbp + 0x534]
1407abf22: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407abf29: f30f11400c                   movss dword ptr [rax + 0xc], xmm0
1407abf2e: e989010000                   jmp 0x1407ac0bc
1407abf33: 4883c630                     add rsi, 0x30
1407abf37: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abf3e: 488d15bb6d2d02               lea rdx, [rip + 0x22d6dbb]
1407abf45: 4889f1                       mov rcx, rsi
1407abf48: e863d50800                   call 0x1408394b0
1407abf4d: f30f1000                     movss xmm0, dword ptr [rax]
1407abf51: 0f57c9                       xorps xmm1, xmm1
1407abf54: 0f2ec1                       ucomiss xmm0, xmm1
1407abf57: 0f85cb000000                 jne 0x1407ac028
1407abf5d: 0f8ac5000000                 jp 0x1407ac028
1407abf63: 488b8db0050000               mov rcx, qword ptr [rbp + 0x5b0]
1407abf6a: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abf71: e83a291500                   call 0x1408fe8b0
1407abf76: 4989c4                       mov r12, rax
1407abf79: 488b95b0050000               mov rdx, qword ptr [rbp + 0x5b0]
1407abf80: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abf87: 488d8d30050000               lea rcx, [rbp + 0x530]
1407abf8e: e8fd281500                   call 0x1408fe890
1407abf93: 83bd3005000003               cmp dword ptr [rbp + 0x530], 3
1407abf9a: 0f8463130000                 je 0x1407ad303
1407abfa0: 8b8540050000                 mov eax, dword ptr [rbp + 0x540]
1407abfa6: 898530050000                 mov dword ptr [rbp + 0x530], eax
1407abfac: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abfb3: 488d9530050000               lea rdx, [rbp + 0x530]
1407abfba: 4c89e1                       mov rcx, r12
1407abfbd: e88e1eab00                   call 0x14125de50
1407abfc2: 4885c0                       test rax, rax
1407abfc5: 740e                         je 0x1407abfd5
1407abfc7: 488b4808                     mov rcx, qword ptr [rax + 8]
1407abfcb: 488b4010                     mov rax, qword ptr [rax + 0x10]
1407abfcf: 488d04c1                     lea rax, [rcx + rax*8]
1407abfd3: eb0a                         jmp 0x1407abfdf
1407abfd5: b804000000                   mov eax, 4
1407abfda: b904000000                   mov ecx, 4
1407abfdf: 48898d30050000               mov qword ptr [rbp + 0x530], rcx
1407abfe6: 48898538050000               mov qword ptr [rbp + 0x538], rax
1407abfed: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407abff4: 488d8d30050000               lea rcx, [rbp + 0x530]
1407abffb: 488b95c0060000               mov rdx, qword ptr [rbp + 0x6c0]
1407ac002: e879d7ffff                   call 0x1407a9780
1407ac007: 84c0                         test al, al
1407ac009: 741d                         je 0x1407ac028
1407ac00b: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac012: c6401100                     mov byte ptr [rax + 0x11], 0
1407ac016: c7400c00000000               mov dword ptr [rax + 0xc], 0
1407ac01d: c70000000000                 mov dword ptr [rax], 0
1407ac023: e994000000                   jmp 0x1407ac0bc
1407ac028: 488b9500030000               mov rdx, qword ptr [rbp + 0x300]
1407ac02f: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac036: 4c8d0df36c2d02               lea r9, [rip + 0x22d6cf3]
1407ac03d: 488d8d30050000               lea rcx, [rbp + 0x530]
1407ac044: 4589f8                       mov r8d, r15d
1407ac047: e8a4e92600                   call 0x140a1a9f0
1407ac04c: f30f108534050000             movss xmm0, dword ptr [rbp + 0x534]
1407ac054: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac05b: f30f11400c                   movss dword ptr [rax + 0xc], xmm0
1407ac060: f60001                       test byte ptr [rax], 1
1407ac063: 7428                         je 0x1407ac08d
1407ac065: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac06c: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407ac073: e808961100                   call 0x1408c5680
1407ac078: 84c0                         test al, al
1407ac07a: 7411                         je 0x1407ac08d
1407ac07c: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac083: f30f10400c                   movss xmm0, dword ptr [rax + 0xc]
1407ac088: f30f114004                   movss dword ptr [rax + 4], xmm0
1407ac08d: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac094: 488d15ad6c2d02               lea rdx, [rip + 0x22d6cad]
1407ac09b: 4889f1                       mov rcx, rsi
1407ac09e: e80dd40800                   call 0x1408394b0
1407ac0a3: f30f1000                     movss xmm0, dword ptr [rax]
1407ac0a7: 0f57c9                       xorps xmm1, xmm1
1407ac0aa: 0f2ec1                       ucomiss xmm0, xmm1
1407ac0ad: 750d                         jne 0x1407ac0bc
1407ac0af: 7a0b                         jp 0x1407ac0bc
1407ac0b1: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac0b8: c6401100                     mov byte ptr [rax + 0x11], 0
1407ac0bc: 4c8b85e8060000               mov r8, qword ptr [rbp + 0x6e8]
1407ac0c3: 4080ff3f                     cmp dil, 0x3f
1407ac0c7: 0f858f000000                 jne 0x1407ac15c
1407ac0cd: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac0d4: 80781100                     cmp byte ptr [rax + 0x11], 0
1407ac0d8: 7447                         je 0x1407ac121
1407ac0da: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac0e1: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407ac0e8: e893951100                   call 0x1408c5680
1407ac0ed: 84c0                         test al, al
1407ac0ef: 7530                         jne 0x1407ac121
1407ac0f1: 83bd2c04000002               cmp dword ptr [rbp + 0x42c], 2
1407ac0f8: 7427                         je 0x1407ac121
1407ac0fa: 80bd1004000000               cmp byte ptr [rbp + 0x410], 0
1407ac101: 751e                         jne 0x1407ac121
1407ac103: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407ac10a: f30f10403c                   movss xmm0, dword ptr [rax + 0x3c]
1407ac10f: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac116: c70001000000                 mov dword ptr [rax], 1
1407ac11c: f30f114004                   movss dword ptr [rax + 4], xmm0
1407ac121: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac128: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407ac12f: e84c951100                   call 0x1408c5680
1407ac134: 488b95c8060000               mov rdx, qword ptr [rbp + 0x6c8]
1407ac13b: 0fb64a11                     movzx ecx, byte ptr [rdx + 0x11]
1407ac13f: 84c0                         test al, al
1407ac141: 7420                         je 0x1407ac163
1407ac143: f6c101                       test cl, 1
1407ac146: 742a                         je 0x1407ac172
1407ac148: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac14f: f3440f10400c                 movss xmm8, dword ptr [rax + 0xc]
1407ac155: bf01000000                   mov edi, 1
1407ac15a: eb40                         jmp 0x1407ac19c
1407ac15c: 31f6                         xor esi, esi
1407ac15e: e944020000                   jmp 0x1407ac3a7
1407ac163: 8b3a                         mov edi, dword ptr [rdx]
1407ac165: f3440f104204                 movss xmm8, dword ptr [rdx + 4]
1407ac16b: f6c101                       test cl, 1
1407ac16e: 7411                         je 0x1407ac181
1407ac170: eb2a                         jmp 0x1407ac19c
1407ac172: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac179: 8b38                         mov edi, dword ptr [rax]
1407ac17b: f3440f104004                 movss xmm8, dword ptr [rax + 4]
1407ac181: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac188: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407ac18f: e8ec941100                   call 0x1408c5680
1407ac194: 84c0                         test al, al
1407ac196: 0f8402020000                 je 0x1407ac39e
1407ac19c: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac1a3: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407ac1aa: e8d1941100                   call 0x1408c5680
1407ac1af: 84c0                         test al, al
1407ac1b1: 0f84cd010000                 je 0x1407ac384
1407ac1b7: 488b85b0040000               mov rax, qword ptr [rbp + 0x4b0]
1407ac1be: f30f1030                     movss xmm6, dword ptr [rax]
1407ac1c2: f30f107804                   movss xmm7, dword ptr [rax + 4]
1407ac1c7: 4c8b7d30                     mov r15, qword ptr [rbp + 0x30]
1407ac1cb: 488b4d50                     mov rcx, qword ptr [rbp + 0x50]
1407ac1cf: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac1d6: e835da1900                   call 0x140949c10
1407ac1db: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac1e2: f30f114c2428                 movss dword ptr [rsp + 0x28], xmm1
1407ac1e8: f30f11442420                 movss dword ptr [rsp + 0x20], xmm0
1407ac1ee: 488d7518                     lea rsi, [rbp + 0x18]
1407ac1f2: 4889f1                       mov rcx, rsi
1407ac1f5: 0f28ce                       movaps xmm1, xmm6
1407ac1f8: 0f28d7                       movaps xmm2, xmm7
1407ac1fb: 4d89f9                       mov r9, r15
1407ac1fe: e8ad08ad00                   call 0x14127cab0
1407ac203: 85ff                         test edi, edi
1407ac205: 0f8493010000                 je 0x1407ac39e
1407ac20b: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac212: f30f107008                   movss xmm6, dword ptr [rax + 8]
1407ac217: c7854803000000000000         mov dword ptr [rbp + 0x348], 0
1407ac221: 48c7854003000000000000       mov qword ptr [rbp + 0x340], 0
1407ac22c: 8bbd2c040000                 mov edi, dword ptr [rbp + 0x42c]
1407ac232: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac239: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407ac240: e83b941100                   call 0x1408c5680
1407ac245: 4188c7                       mov r15b, al
1407ac248: 31c0                         xor eax, eax
1407ac24a: 83ff02                       cmp edi, 2
1407ac24d: 488dbd00040000               lea rdi, [rbp + 0x400]
1407ac254: 480f44f8                     cmove rdi, rax
1407ac258: f3440f58c6                   addss xmm8, xmm6
1407ac25d: 4c8da5e0050000               lea r12, [rbp + 0x5e0]
1407ac264: 488d5530                     lea rdx, [rbp + 0x30]
1407ac268: 41b8a0000000                 mov r8d, 0xa0
1407ac26e: 4c89e1                       mov rcx, r12
1407ac271: e88a791702                   call 0x142923c00
1407ac276: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac27d: 488d8540030000               lea rax, [rbp + 0x340]
1407ac284: 4889442448                   mov qword ptr [rsp + 0x48], rax
1407ac289: f3440f11442440               movss dword ptr [rsp + 0x40], xmm8
1407ac290: 4889742438                   mov qword ptr [rsp + 0x38], rsi
1407ac295: 44887c2430                   mov byte ptr [rsp + 0x30], r15b
1407ac29a: 48897c2428                   mov qword ptr [rsp + 0x28], rdi
1407ac29f: f30f11742420                 movss dword ptr [rsp + 0x20], xmm6
1407ac2a5: 488d8d30050000               lea rcx, [rbp + 0x530]
1407ac2ac: 4c89e2                       mov rdx, r12
1407ac2af: 4c8b85b8040000               mov r8, qword ptr [rbp + 0x4b8]
1407ac2b6: 4c8b8d60040000               mov r9, qword ptr [rbp + 0x460]
1407ac2bd: e8bebbffff                   call 0x1407a7e80
1407ac2c2: 83bd5c05000002               cmp dword ptr [rbp + 0x55c], 2
1407ac2c9: 400f95c6                     setne sil
1407ac2cd: 0f84cd000000                 je 0x1407ac3a0
1407ac2d3: f30f10bd54050000             movss xmm7, dword ptr [rbp + 0x554]
1407ac2db: f30f10b570050000             movss xmm6, dword ptr [rbp + 0x570]
1407ac2e3: f30f1005ed951702             movss xmm0, dword ptr [rip + 0x21795ed]
1407ac2eb: 0f2ec6                       ucomiss xmm0, xmm6
1407ac2ee: 761e                         jbe 0x1407ac30e
1407ac2f0: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac2f7: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407ac2fe: e87d931100                   call 0x1408c5680
1407ac303: 84c0                         test al, al
1407ac305: 7507                         jne 0x1407ac30e
1407ac307: f30f5cfe                     subss xmm7, xmm6
1407ac30b: 0f57f6                       xorps xmm6, xmm6
1407ac30e: 488d8558050000               lea rax, [rbp + 0x558]
1407ac315: 8b8d50050000                 mov ecx, dword ptr [rbp + 0x550]
1407ac31b: 898d20040000                 mov dword ptr [rbp + 0x420], ecx
1407ac321: 0f288530050000               movaps xmm0, xmmword ptr [rbp + 0x530]
1407ac328: 0f288d40050000               movaps xmm1, xmmword ptr [rbp + 0x540]
1407ac32f: 0f298d10040000               movaps xmmword ptr [rbp + 0x410], xmm1
1407ac336: 0f298500040000               movaps xmmword ptr [rbp + 0x400], xmm0
1407ac33d: 0f1000                       movups xmm0, xmmword ptr [rax]
1407ac340: 0f118528040000               movups xmmword ptr [rbp + 0x428], xmm0
1407ac347: 488b4010                     mov rax, qword ptr [rax + 0x10]
1407ac34b: 48898538040000               mov qword ptr [rbp + 0x438], rax
1407ac352: f30f11bd24040000             movss dword ptr [rbp + 0x424], xmm7
1407ac35a: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac361: 488d15f8692d02               lea rdx, [rip + 0x22d69f8]
1407ac368: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407ac36f: e83cd10800                   call 0x1408394b0
1407ac374: f30f1130                     movss dword ptr [rax], xmm6
1407ac378: c785e006000000000000         mov dword ptr [rbp + 0x6e0], 0
1407ac382: eb1c                         jmp 0x1407ac3a0
1407ac384: 83bd2c04000002               cmp dword ptr [rbp + 0x42c], 2
1407ac38b: 7411                         je 0x1407ac39e
1407ac38d: 0fb68510040000               movzx eax, byte ptr [rbp + 0x410]
1407ac394: fec8                         dec al
1407ac396: 3c01                         cmp al, 1
1407ac398: 0f8619feffff                 jbe 0x1407ac1b7
1407ac39e: 31f6                         xor esi, esi
1407ac3a0: 4c8b85e8060000               mov r8, qword ptr [rbp + 0x6e8]
1407ac3a7: 83bd2c04000002               cmp dword ptr [rbp + 0x42c], 2
1407ac3ae: 7528                         jne 0x1407ac3d8
1407ac3b0: 8b4308                       mov eax, dword ptr [rbx + 8]
1407ac3b3: 898550030000                 mov dword ptr [rbp + 0x350], eax
1407ac3b9: 488b03                       mov rax, qword ptr [rbx]
1407ac3bc: 48898548030000               mov qword ptr [rbp + 0x348], rax
1407ac3c3: 48c7854003000003000000       mov qword ptr [rbp + 0x340], 3
1407ac3ce: b803000000                   mov eax, 3
1407ac3d3: e942010000                   jmp 0x1407ac51a
1407ac3d8: 0f288500040000               movaps xmm0, xmmword ptr [rbp + 0x400]
1407ac3df: 0f288d10040000               movaps xmm1, xmmword ptr [rbp + 0x410]
1407ac3e6: 0f289520040000               movaps xmm2, xmmword ptr [rbp + 0x420]
1407ac3ed: 0f289d30040000               movaps xmm3, xmmword ptr [rbp + 0x430]
1407ac3f4: 0f298df0050000               movaps xmmword ptr [rbp + 0x5f0], xmm1
1407ac3fb: 0f299d10060000               movaps xmmword ptr [rbp + 0x610], xmm3
1407ac402: 0f299500060000               movaps xmmword ptr [rbp + 0x600], xmm2
1407ac409: 0f2985e0050000               movaps xmmword ptr [rbp + 0x5e0], xmm0
1407ac410: 0fb685f0050000               movzx eax, byte ptr [rbp + 0x5f0]
1407ac417: 488d0d4a6d2d02               lea rcx, [rip + 0x22d6d4a]
1407ac41e: 48630481                     movsxd rax, dword ptr [rcx + rax*4]
1407ac422: 4801c8                       add rax, rcx
1407ac425: ffe0                         jmp rax
1407ac427: 4c8bbdf8050000               mov r15, qword ptr [rbp + 0x5f8]
1407ac42e: 31ff                         xor edi, edi
1407ac430: eb0c                         jmp 0x1407ac43e
1407ac432: bf01000000                   mov edi, 1
1407ac437: eb05                         jmp 0x1407ac43e
1407ac439: bf02000000                   mov edi, 2
1407ac43e: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac445: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407ac44c: e82f921100                   call 0x1408c5680
1407ac451: 3401                         xor al, 1
1407ac453: 488b8d00030000               mov rcx, qword ptr [rbp + 0x300]
1407ac45a: 488b9528080000               mov rdx, qword ptr [rbp + 0x828]
1407ac461: 488b12                       mov rdx, qword ptr [rdx]
1407ac464: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac46b: 4889942488000000             mov qword ptr [rsp + 0x88], rdx
1407ac473: 488b95a8040000               mov rdx, qword ptr [rbp + 0x4a8]
1407ac47a: 4889942480000000             mov qword ptr [rsp + 0x80], rdx
1407ac482: 8b95e0060000                 mov edx, dword ptr [rbp + 0x6e0]
1407ac488: 88542478                     mov byte ptr [rsp + 0x78], dl
1407ac48c: 48894c2470                   mov qword ptr [rsp + 0x70], rcx
1407ac491: 88442468                     mov byte ptr [rsp + 0x68], al
1407ac495: 4c897c2460                   mov qword ptr [rsp + 0x60], r15
1407ac49a: 48897c2458                   mov qword ptr [rsp + 0x58], rdi
1407ac49f: 488d8500020000               lea rax, [rbp + 0x200]
1407ac4a6: 4889442450                   mov qword ptr [rsp + 0x50], rax
1407ac4ab: 488b85c0060000               mov rax, qword ptr [rbp + 0x6c0]
1407ac4b2: 4889442448                   mov qword ptr [rsp + 0x48], rax
1407ac4b7: 488d8590020000               lea rax, [rbp + 0x290]
1407ac4be: 4889442430                   mov qword ptr [rsp + 0x30], rax
1407ac4c3: 8b85d8060000                 mov eax, dword ptr [rbp + 0x6d8]
1407ac4c9: 89442428                     mov dword ptr [rsp + 0x28], eax
1407ac4cd: 488d85e0050000               lea rax, [rbp + 0x5e0]
1407ac4d4: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407ac4d9: 48c744244000000000           mov qword ptr [rsp + 0x40], 0
1407ac4e2: 48c744243804000000           mov qword ptr [rsp + 0x38], 4
1407ac4eb: 488d8d40030000               lea rcx, [rbp + 0x340]
1407ac4f2: 4c8d8590010000               lea r8, [rbp + 0x190]
1407ac4f9: 488b95b8040000               mov rdx, qword ptr [rbp + 0x4b8]
1407ac500: 4c8b8db8030000               mov r9, qword ptr [rbp + 0x3b8]
1407ac507: e8640aad00                   call 0x14127cf70
1407ac50c: 488b8540030000               mov rax, qword ptr [rbp + 0x340]
1407ac513: 4c8b85e8060000               mov r8, qword ptr [rbp + 0x6e8]
1407ac51a: 488d50fd                     lea rdx, [rax - 3]
1407ac51e: 4883c0fe                     add rax, -2
1407ac522: 31c9                         xor ecx, ecx
1407ac524: 4883fa02                     cmp rdx, 2
1407ac528: 480f42c8                     cmovb rcx, rax
1407ac52c: 4885c9                       test rcx, rcx
1407ac52f: 7435                         je 0x1407ac566
1407ac531: 4883f901                     cmp rcx, 1
1407ac535: 0f8501030000                 jne 0x1407ac83c
1407ac53b: 488b85b0040000               mov rax, qword ptr [rbp + 0x4b0]
1407ac542: f30f1008                     movss xmm1, dword ptr [rax]
1407ac546: 660f6e5004                   movd xmm2, dword ptr [rax + 4]
1407ac54b: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac552: 4489f1                       mov ecx, r14d
1407ac555: 4c8b8da8040000               mov r9, qword ptr [rbp + 0x4a8]
1407ac55c: e8bf150000                   call 0x1407adb20
1407ac561: e9570c0000                   jmp 0x1407ad1bd
1407ac566: 4989dc                       mov r12, rbx
1407ac569: 0f288540030000               movaps xmm0, xmmword ptr [rbp + 0x340]
1407ac570: 0f288d50030000               movaps xmm1, xmmword ptr [rbp + 0x350]
1407ac577: 0f289560030000               movaps xmm2, xmmword ptr [rbp + 0x360]
1407ac57e: 0f289d70030000               movaps xmm3, xmmword ptr [rbp + 0x370]
1407ac585: 0f299550050000               movaps xmmword ptr [rbp + 0x550], xmm2
1407ac58c: 0f289590030000               movaps xmm2, xmmword ptr [rbp + 0x390]
1407ac593: 0f299580050000               movaps xmmword ptr [rbp + 0x580], xmm2
1407ac59a: 0f289580030000               movaps xmm2, xmmword ptr [rbp + 0x380]
1407ac5a1: 0f299570050000               movaps xmmword ptr [rbp + 0x570], xmm2
1407ac5a8: 0f299d60050000               movaps xmmword ptr [rbp + 0x560], xmm3
1407ac5af: 0f298d40050000               movaps xmmword ptr [rbp + 0x540], xmm1
1407ac5b6: 0f298530050000               movaps xmmword ptr [rbp + 0x530], xmm0
1407ac5bd: 8bbd50050000                 mov edi, dword ptr [rbp + 0x550]
1407ac5c3: 83ff02                       cmp edi, 2
1407ac5c6: 752b                         jne 0x1407ac5f3
1407ac5c8: f30f108d58050000             movss xmm1, dword ptr [rbp + 0x558]
1407ac5d0: f30f10955c050000             movss xmm2, dword ptr [rbp + 0x55c]
1407ac5d8: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac5df: 488b8d78020000               mov rcx, qword ptr [rbp + 0x278]
1407ac5e6: e865633200                   call 0x140ad2950
1407ac5eb: 84c0                         test al, al
1407ac5ed: 0f84f8020000                 je 0x1407ac8eb
1407ac5f3: 0f57c0                       xorps xmm0, xmm0
1407ac5f6: f6854c04000001               test byte ptr [rbp + 0x44c], 1
1407ac5fd: 7454                         je 0x1407ac653
1407ac5ff: 0f288580050000               movaps xmm0, xmmword ptr [rbp + 0x580]
1407ac606: f30f108d84050000             movss xmm1, dword ptr [rbp + 0x584]
1407ac60e: f3410f5cc9                   subss xmm1, xmm9
1407ac613: f30f59c9                     mulss xmm1, xmm1
1407ac617: f30f109588050000             movss xmm2, dword ptr [rbp + 0x588]
1407ac61f: 0f16d0                       movlhps xmm2, xmm0
1407ac622: 0fc6d0e2                     shufps xmm2, xmm0, 0xe2
1407ac626: 0f2885b0010000               movaps xmm0, xmmword ptr [rbp + 0x1b0]
1407ac62d: 410f14c3                     unpcklps xmm0, xmm11
1407ac631: 0f5cd0                       subps xmm2, xmm0
1407ac634: 0f59d2                       mulps xmm2, xmm2
1407ac637: f30f58ca                     addss xmm1, xmm2
1407ac63b: f30f16c2                     movshdup xmm0, xmm2
1407ac63f: f30f58c1                     addss xmm0, xmm1
1407ac643: f30f51c0                     sqrtss xmm0, xmm0
1407ac647: 488b8580020000               mov rax, qword ptr [rbp + 0x280]
1407ac64e: f30f5e4058                   divss xmm0, dword ptr [rax + 0x58]
1407ac653: 488b95c8030000               mov rdx, qword ptr [rbp + 0x3c8]
1407ac65a: f30f114224                   movss dword ptr [rdx + 0x24], xmm0
1407ac65f: 488b8580050000               mov rax, qword ptr [rbp + 0x580]
1407ac666: 48898530030000               mov qword ptr [rbp + 0x330], rax
1407ac66d: 8b8d88050000                 mov ecx, dword ptr [rbp + 0x588]
1407ac673: 898d38030000                 mov dword ptr [rbp + 0x338], ecx
1407ac679: 488985e0050000               mov qword ptr [rbp + 0x5e0], rax
1407ac680: 898de8050000                 mov dword ptr [rbp + 0x5e8], ecx
1407ac686: 488b85c0030000               mov rax, qword ptr [rbp + 0x3c0]
1407ac68d: 8b8d48040000                 mov ecx, dword ptr [rbp + 0x448]
1407ac693: 8908                         mov dword ptr [rax], ecx
1407ac695: c7421001000000               mov dword ptr [rdx + 0x10], 1
1407ac69c: 8b85e8050000                 mov eax, dword ptr [rbp + 0x5e8]
1407ac6a2: 488b8d88020000               mov rcx, qword ptr [rbp + 0x288]
1407ac6a9: 894108                       mov dword ptr [rcx + 8], eax
1407ac6ac: 488b85e0050000               mov rax, qword ptr [rbp + 0x5e0]
1407ac6b3: 488901                       mov qword ptr [rcx], rax
1407ac6b6: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac6bd: 488d8d30020000               lea rcx, [rbp + 0x230]
1407ac6c4: 488d9530050000               lea rdx, [rbp + 0x530]
1407ac6cb: 4c8b85b8040000               mov r8, qword ptr [rbp + 0x4b8]
1407ac6d2: e8193bad00                   call 0x1412801f0
1407ac6d7: 440fb68db8060000             movzx r9d, byte ptr [rbp + 0x6b8]
1407ac6df: 4180f101                     xor r9b, 1
1407ac6e3: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407ac6ea: c6401202                     mov byte ptr [rax + 0x12], 2
1407ac6ee: f6857005000001               test byte ptr [rbp + 0x570], 1
1407ac6f5: 0f849d010000                 je 0x1407ac898
1407ac6fb: 8b8574050000                 mov eax, dword ptr [rbp + 0x574]
1407ac701: 0fb68d78050000               movzx ecx, byte ptr [rbp + 0x578]
1407ac708: 488b95e8060000               mov rdx, qword ptr [rbp + 0x6e8]
1407ac70f: 0fb65269                     movzx edx, byte ptr [rdx + 0x69]
1407ac713: 4c8b8550020000               mov r8, qword ptr [rbp + 0x250]
1407ac71a: 4c898520060000               mov qword ptr [rbp + 0x620], r8
1407ac721: 0f108530020000               movups xmm0, xmmword ptr [rbp + 0x230]
1407ac728: 0f108d40020000               movups xmm1, xmmword ptr [rbp + 0x240]
1407ac72f: 0f118d10060000               movups xmmword ptr [rbp + 0x610], xmm1
1407ac736: 0f118500060000               movups xmmword ptr [rbp + 0x600], xmm0
1407ac73d: 0f288540050000               movaps xmm0, xmmword ptr [rbp + 0x540]
1407ac744: 0f1185f0050000               movups xmmword ptr [rbp + 0x5f0], xmm0
1407ac74b: 0f1185e0050000               movups xmmword ptr [rbp + 0x5e0], xmm0
1407ac752: 888d28060000                 mov byte ptr [rbp + 0x628], cl
1407ac758: c7853006000000000000         mov dword ptr [rbp + 0x630], 0
1407ac762: c7853806000000000000         mov dword ptr [rbp + 0x638], 0
1407ac76c: c6854006000001               mov byte ptr [rbp + 0x640], 1
1407ac773: 888d41060000                 mov byte ptr [rbp + 0x641], cl
1407ac779: c6854206000000               mov byte ptr [rbp + 0x642], 0
1407ac780: 66c785440600000240           mov word ptr [rbp + 0x644], 0x4002
1407ac789: 898548060000                 mov dword ptr [rbp + 0x648], eax
1407ac78f: 88954c060000                 mov byte ptr [rbp + 0x64c], dl
1407ac795: 8b8538030000                 mov eax, dword ptr [rbp + 0x338]
1407ac79b: 8985c8040000                 mov dword ptr [rbp + 0x4c8], eax
1407ac7a1: 488b8530030000               mov rax, qword ptr [rbp + 0x330]
1407ac7a8: 488985c0040000               mov qword ptr [rbp + 0x4c0], rax
1407ac7af: c685cc04000000               mov byte ptr [rbp + 0x4cc], 0
1407ac7b6: 488b85a8050000               mov rax, qword ptr [rbp + 0x5a8]
1407ac7bd: 8b8dac060000                 mov ecx, dword ptr [rbp + 0x6ac]
1407ac7c3: 8908                         mov dword ptr [rax], ecx
1407ac7c5: 488b8520050000               mov rax, qword ptr [rbp + 0x520]
1407ac7cc: 8b8da8060000                 mov ecx, dword ptr [rbp + 0x6a8]
1407ac7d2: 8908                         mov dword ptr [rax], ecx
1407ac7d4: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407ac7da: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407ac7e1: 8901                         mov dword ptr [rcx], eax
1407ac7e3: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac7ea: 488b8518050000               mov rax, qword ptr [rbp + 0x518]
1407ac7f1: 4889442438                   mov qword ptr [rsp + 0x38], rax
1407ac7f6: 488d85c0010000               lea rax, [rbp + 0x1c0]
1407ac7fd: 4889442430                   mov qword ptr [rsp + 0x30], rax
1407ac802: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407ac809: 4889442428                   mov qword ptr [rsp + 0x28], rax
1407ac80e: 488b8510050000               mov rax, qword ptr [rbp + 0x510]
1407ac815: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407ac81a: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407ac821: 4c8d85c0040000               lea r8, [rbp + 0x4c0]
1407ac828: 4489ca                       mov edx, r9d
1407ac82b: 4c8b8dc0060000               mov r9, qword ptr [rbp + 0x6c0]
1407ac832: e8f9a5ac00                   call 0x141276e30
1407ac837: e981090000                   jmp 0x1407ad1bd
1407ac83c: 488b85a0040000               mov rax, qword ptr [rbp + 0x4a0]
1407ac843: 8b8d84060000                 mov ecx, dword ptr [rbp + 0x684]
1407ac849: 8908                         mov dword ptr [rax], ecx
1407ac84b: 488b8500050000               mov rax, qword ptr [rbp + 0x500]
1407ac852: c6808202000002               mov byte ptr [rax + 0x282], 2
1407ac859: 488b8dc8060000               mov rcx, qword ptr [rbp + 0x6c8]
1407ac860: 80791202                     cmp byte ptr [rcx + 0x12], 2
1407ac864: b002                         mov al, 2
1407ac866: 1c00                         sbb al, 0
1407ac868: 884112                       mov byte ptr [rcx + 0x12], al
1407ac86b: 410fb64869                   movzx ecx, byte ptr [r8 + 0x69]
1407ac870: 488b85b0040000               mov rax, qword ptr [rbp + 0x4b0]
1407ac877: f30f1008                     movss xmm1, dword ptr [rax]
1407ac87b: 660f6e5004                   movd xmm2, dword ptr [rax + 4]
1407ac880: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac887: 4c8b8da8040000               mov r9, qword ptr [rbp + 0x4a8]
1407ac88e: e88d120000                   call 0x1407adb20
1407ac893: e925090000                   jmp 0x1407ad1bd
1407ac898: 44888db8060000               mov byte ptr [rbp + 0x6b8], r9b
1407ac89f: 488b95e8060000               mov rdx, qword ptr [rbp + 0x6e8]
1407ac8a6: 0fb64269                     movzx eax, byte ptr [rdx + 0x69]
1407ac8aa: 04c8                         add al, 0xc8
1407ac8ac: 3c08                         cmp al, 8
1407ac8ae: 0fb6c0                       movzx eax, al
1407ac8b1: b906000000                   mov ecx, 6
1407ac8b6: 0f42c8                       cmovb ecx, eax
1407ac8b9: 0fb6c1                       movzx eax, cl
1407ac8bc: 488d0db5682d02               lea rcx, [rip + 0x22d68b5]
1407ac8c3: 48630481                     movsxd rax, dword ptr [rcx + rax*4]
1407ac8c7: 4801c8                       add rax, rcx
1407ac8ca: ffe0                         jmp rax
1407ac8cc: 488b85a0060000               mov rax, qword ptr [rbp + 0x6a0]
1407ac8d3: 803800                       cmp byte ptr [rax], 0
1407ac8d6: c785e0060000e7030000         mov dword ptr [rbp + 0x6e0], 0x3e7
1407ac8e0: 0f859b000000                 jne 0x1407ac981
1407ac8e6: e9cc010000                   jmp 0x1407acab7
1407ac8eb: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac8f2: 488d157f642d02               lea rdx, [rip + 0x22d647f]
1407ac8f9: 488b8d20080000               mov rcx, qword ptr [rbp + 0x820]
1407ac900: e85b1cab00                   call 0x14125e560
1407ac905: 4180fd01                     cmp r13b, 1
1407ac909: 771a                         ja 0x1407ac925
1407ac90b: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac912: 488d1577642d02               lea rdx, [rip + 0x22d6477]
1407ac919: 488d8d70010000               lea rcx, [rbp + 0x170]
1407ac920: e83b1cab00                   call 0x14125e560
1407ac925: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407ac92c: 0fb64869                     movzx ecx, byte ptr [rax + 0x69]
1407ac930: 488b85b0040000               mov rax, qword ptr [rbp + 0x4b0]
1407ac937: f30f1008                     movss xmm1, dword ptr [rax]
1407ac93b: 660f6e5004                   movd xmm2, dword ptr [rax + 4]
1407ac940: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac947: 4c8b8da8040000               mov r9, qword ptr [rbp + 0x4a8]
1407ac94e: e8cd110000                   call 0x1407adb20
1407ac953: e965080000                   jmp 0x1407ad1bd
1407ac958: 488b85a0060000               mov rax, qword ptr [rbp + 0x6a0]
1407ac95f: 8b4028                       mov eax, dword ptr [rax + 0x28]
1407ac962: e94a010000                   jmp 0x1407acab1
1407ac967: c785e006000000000000         mov dword ptr [rbp + 0x6e0], 0
1407ac971: 488b85a0060000               mov rax, qword ptr [rbp + 0x6a0]
1407ac978: 803800                       cmp byte ptr [rax], 0
1407ac97b: 0f8436010000                 je 0x1407acab7
1407ac981: 8b4004                       mov eax, dword ptr [rax + 4]
1407ac984: e928010000                   jmp 0x1407acab1
1407ac989: 488d8d40050000               lea rcx, [rbp + 0x540]
1407ac990: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac997: e8a4f82600                   call 0x140a1c240
1407ac99c: 488985e0050000               mov qword ptr [rbp + 0x5e0], rax
1407ac9a3: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ac9aa: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407ac9b1: e81abc8dff                   call 0x1400885d0
1407ac9b6: 488b8de8030000               mov rcx, qword ptr [rbp + 0x3e8]
1407ac9bd: 488b5110                     mov rdx, qword ptr [rcx + 0x10]
1407ac9c1: 4839d0                       cmp rax, rdx
1407ac9c4: 0f834e090000                 jae 0x1407ad318
1407ac9ca: 488b4908                     mov rcx, qword ptr [rcx + 8]
1407ac9ce: 8b0481                       mov eax, dword ptr [rcx + rax*4]
1407ac9d1: ffc8                         dec eax
1407ac9d3: c785e0060000a0860100         mov dword ptr [rbp + 0x6e0], 0x186a0
1407ac9dd: 83f80b                       cmp eax, 0xb
1407ac9e0: 488b95e8060000               mov rdx, qword ptr [rbp + 0x6e8]
1407ac9e7: 0f83ca000000                 jae 0x1407acab7
1407ac9ed: 89c0                         mov eax, eax
1407ac9ef: 488d0d8a662d02               lea rcx, [rip + 0x22d668a]
1407ac9f6: 8b0481                       mov eax, dword ptr [rcx + rax*4]
1407ac9f9: e9b3000000                   jmp 0x1407acab1
1407ac9fe: 488b95b0050000               mov rdx, qword ptr [rbp + 0x5b0]
1407aca05: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aca0c: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407aca13: e8781e1500                   call 0x1408fe890
1407aca18: 488b95e0050000               mov rdx, qword ptr [rbp + 0x5e0]
1407aca1f: 4883fa03                     cmp rdx, 3
1407aca23: 0f85b1040000                 jne 0x1407aceda
1407aca29: 488d8d40050000               lea rcx, [rbp + 0x540]
1407aca30: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aca37: e854f82600                   call 0x140a1c290
1407aca3c: 4883f801                     cmp rax, 1
1407aca40: 0f85e5070000                 jne 0x1407ad22b
1407aca46: 488995e0050000               mov qword ptr [rbp + 0x5e0], rdx
1407aca4d: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aca54: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407aca5b: e870bb8dff                   call 0x1400885d0
1407aca60: 488b8de8030000               mov rcx, qword ptr [rbp + 0x3e8]
1407aca67: 488b5110                     mov rdx, qword ptr [rcx + 0x10]
1407aca6b: 4839d0                       cmp rax, rdx
1407aca6e: 0f83d1080000                 jae 0x1407ad345
1407aca74: 488b4908                     mov rcx, qword ptr [rcx + 8]
1407aca78: 8b0481                       mov eax, dword ptr [rcx + rax*4]
1407aca7b: ffc8                         dec eax
1407aca7d: c785e0060000a0860100         mov dword ptr [rbp + 0x6e0], 0x186a0
1407aca87: 83f80a                       cmp eax, 0xa
1407aca8a: 7712                         ja 0x1407aca9e
1407aca8c: 89c0                         mov eax, eax
1407aca8e: 488d0deb652d02               lea rcx, [rip + 0x22d65eb]
1407aca95: 8b0481                       mov eax, dword ptr [rcx + rax*4]
1407aca98: 8985e0060000                 mov dword ptr [rbp + 0x6e0], eax
1407aca9e: 488b95e8060000               mov rdx, qword ptr [rbp + 0x6e8]
1407acaa5: eb10                         jmp 0x1407acab7
1407acaa7: 488b85a0060000               mov rax, qword ptr [rbp + 0x6a0]
1407acaae: 8b4034                       mov eax, dword ptr [rax + 0x34]
1407acab1: 8985e0060000                 mov dword ptr [rbp + 0x6e0], eax
1407acab7: 4180fd02                     cmp r13b, 2
1407acabb: 733e                         jae 0x1407acafb
1407acabd: 488b85e0030000               mov rax, qword ptr [rbp + 0x3e0]
1407acac4: 833814                       cmp dword ptr [rax], 0x14
1407acac7: 7532                         jne 0x1407acafb
1407acac9: c785e005000000000000         mov dword ptr [rbp + 0x5e0], 0
1407acad3: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acada: 4c8d05f7622d02               lea r8, [rip + 0x22d62f7]
1407acae1: 488d8d10020000               lea rcx, [rbp + 0x210]
1407acae8: 488d95e0050000               lea rdx, [rbp + 0x5e0]
1407acaef: e8ac1bab00                   call 0x14125e6a0
1407acaf4: 488b95e8060000               mov rdx, qword ptr [rbp + 0x6e8]
1407acafb: 807a693f                     cmp byte ptr [rdx + 0x69], 0x3f
1407acaff: 0f95c0                       setne al
1407acb02: 4008c6                       or sil, al
1407acb05: 0f85ea020000                 jne 0x1407acdf5
1407acb0b: 83ff02                       cmp edi, 2
1407acb0e: 7530                         jne 0x1407acb40
1407acb10: f30f10bd58050000             movss xmm7, dword ptr [rbp + 0x558]
1407acb18: f3440f10855c050000           movss xmm8, dword ptr [rbp + 0x55c]
1407acb21: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acb28: 488b8d60040000               mov rcx, qword ptr [rbp + 0x460]
1407acb2f: 0f28cf                       movaps xmm1, xmm7
1407acb32: 410f28d0                     movaps xmm2, xmm8
1407acb36: e825aeaaff                   call 0x140257960
1407acb3b: 0f28f0                       movaps xmm6, xmm0
1407acb3e: eb17                         jmp 0x1407acb57
1407acb40: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407acb47: f30f107838                   movss xmm7, dword ptr [rax + 0x38]
1407acb4c: f30f10703c                   movss xmm6, dword ptr [rax + 0x3c]
1407acb51: f3440f104040                 movss xmm8, dword ptr [rax + 0x40]
1407acb57: 488b95b0050000               mov rdx, qword ptr [rbp + 0x5b0]
1407acb5e: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acb65: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407acb6c: e81f1d1500                   call 0x1408fe890
1407acb71: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acb78: 488d8de0020000               lea rcx, [rbp + 0x2e0]
1407acb7f: 488d95e0050000               lea rdx, [rbp + 0x5e0]
1407acb86: 4c8b85c0060000               mov r8, qword ptr [rbp + 0x6c0]
1407acb8d: e8ceccffff                   call 0x1407a9860
1407acb92: 488b85e0020000               mov rax, qword ptr [rbp + 0x2e0]
1407acb99: 4883f803                     cmp rax, 3
1407acb9d: 0f8452020000                 je 0x1407acdf5
1407acba3: 488b8de8020000               mov rcx, qword ptr [rbp + 0x2e8]
1407acbaa: 488bb5f0020000               mov rsi, qword ptr [rbp + 0x2f0]
1407acbb1: 488985a8030000               mov qword ptr [rbp + 0x3a8], rax
1407acbb8: 48898db0030000               mov qword ptr [rbp + 0x3b0], rcx
1407acbbf: 448b85f8020000               mov r8d, dword ptr [rbp + 0x2f8]
1407acbc6: 488b9500030000               mov rdx, qword ptr [rbp + 0x300]
1407acbcd: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acbd4: 4c8d0d15622d02               lea r9, [rip + 0x22d6215]
1407acbdb: 488d8df4030000               lea rcx, [rbp + 0x3f4]
1407acbe2: e809de2600                   call 0x140a1a9f0
1407acbe7: f30f108df4030000             movss xmm1, dword ptr [rbp + 0x3f4]
1407acbef: f30f1095fc030000             movss xmm2, dword ptr [rbp + 0x3fc]
1407acbf7: f30f118d88040000             movss dword ptr [rbp + 0x488], xmm1
1407acbff: f30f11958c040000             movss dword ptr [rbp + 0x48c], xmm2
1407acc07: 807e2202                     cmp byte ptr [rsi + 0x22], 2
1407acc0b: 7559                         jne 0x1407acc66
1407acc0d: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acc14: 488d8da8030000               lea rcx, [rbp + 0x3a8]
1407acc1b: e820f62600                   call 0x140a1c240
1407acc20: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acc27: 4c8d05da612d02               lea r8, [rip + 0x22d61da]
1407acc2e: 488b8db8040000               mov rcx, qword ptr [rbp + 0x4b8]
1407acc35: 4889c2                       mov rdx, rax
1407acc38: e8b3522e00                   call 0x140a91ef0
1407acc3d: 4885c0                       test rax, rax
1407acc40: 0f84ea060000                 je 0x1407ad330
1407acc46: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acc4d: 4889c1                       mov rcx, rax
1407acc50: e88bda3700                   call 0x140b2a6e0
1407acc55: eb22                         jmp 0x1407acc79
1407acc57: c785e006000000000000         mov dword ptr [rbp + 0x6e0], 0
1407acc61: e951feffff                   jmp 0x1407acab7
1407acc66: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acc6d: 488b8d60040000               mov rcx, qword ptr [rbp + 0x460]
1407acc74: e8e7acaaff                   call 0x140257960
1407acc79: 440f28c8                     movaps xmm9, xmm0
1407acc7d: 488d85f8030000               lea rax, [rbp + 0x3f8]
1407acc84: f3440f10a5f8030000           movss xmm12, dword ptr [rbp + 0x3f8]
1407acc8d: f30f5cbd88040000             subss xmm7, dword ptr [rbp + 0x488]
1407acc95: f3440f5c858c040000           subss xmm8, dword ptr [rbp + 0x48c]
1407acc9e: 488d8db0050000               lea rcx, [rbp + 0x5b0]
1407acca5: 48898dc0040000               mov qword ptr [rbp + 0x4c0], rcx
1407accac: 488d9500030000               lea rdx, [rbp + 0x300]
1407accb3: 488995c8040000               mov qword ptr [rbp + 0x4c8], rdx
1407accba: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
1407accc1: 4c8d8588040000               lea r8, [rbp + 0x488]
1407accc8: 4c8985d8040000               mov qword ptr [rbp + 0x4d8], r8
1407acccf: 48898de0050000               mov qword ptr [rbp + 0x5e0], rcx
1407accd6: 488995e8050000               mov qword ptr [rbp + 0x5e8], rdx
1407accdd: 488985f0050000               mov qword ptr [rbp + 0x5f0], rax
1407acce4: 4c8985f8050000               mov qword ptr [rbp + 0x5f8], r8
1407acceb: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407accf2: 488b8d98060000               mov rcx, qword ptr [rbp + 0x698]
1407accf9: e882891100                   call 0x1408c5680
1407accfe: f3450f5ce1                   subss xmm12, xmm9
1407acd03: 450f28d9                     movaps xmm11, xmm9
1407acd07: f3450f58dc                   addss xmm11, xmm12
1407acd0c: 450f28d3                     movaps xmm10, xmm11
1407acd10: f3440f5cd6                   subss xmm10, xmm6
1407acd15: 84c0                         test al, al
1407acd17: 741f                         je 0x1407acd38
1407acd19: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acd20: 488d1571612d02               lea rdx, [rip + 0x22d6171]
1407acd27: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407acd2e: e87dc70800                   call 0x1408394b0
1407acd33: e9a5000000                   jmp 0x1407acddd
1407acd38: f30f59ff                     mulss xmm7, xmm7
1407acd3c: f3450f59c0                   mulss xmm8, xmm8
1407acd41: f3440f58c7                   addss xmm8, xmm7
1407acd46: 0f57ff                       xorps xmm7, xmm7
1407acd49: f3410f51f8                   sqrtss xmm7, xmm8
1407acd4e: 440f2e25ae8b1702             ucomiss xmm12, dword ptr [rip + 0x2178bae]
1407acd56: 771b                         ja 0x1407acd73
1407acd58: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acd5f: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407acd66: e8850c0000                   call 0x1407ad9f0
1407acd6b: 84c0                         test al, al
1407acd6d: 0f84d4040000                 je 0x1407ad247
1407acd73: 0f57c0                       xorps xmm0, xmm0
1407acd76: 440f2ed0                     ucomiss xmm10, xmm0
1407acd7a: 7647                         jbe 0x1407acdc3
1407acd7c: 410f28c2                     movaps xmm0, xmm10
1407acd80: f3410f58c2                   addss xmm0, xmm10
1407acd85: f30f100de38b1702             movss xmm1, dword ptr [rip + 0x2178be3]
1407acd8d: f30f59c8                     mulss xmm1, xmm0
1407acd91: 0f2ef9                       ucomiss xmm7, xmm1
1407acd94: 762d                         jbe 0x1407acdc3
1407acd96: 0f2ec7                       ucomiss xmm0, xmm7
1407acd99: 7628                         jbe 0x1407acdc3
1407acd9b: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acda2: 488d8dc0040000               lea rcx, [rbp + 0x4c0]
1407acda9: e8120b0000                   call 0x1407ad8c0
1407acdae: 84c0                         test al, al
1407acdb0: 7511                         jne 0x1407acdc3
1407acdb2: 488b85c8060000               mov rax, qword ptr [rbp + 0x6c8]
1407acdb9: 80781100                     cmp byte ptr [rax + 0x11], 0
1407acdbd: 0f8405050000                 je 0x1407ad2c8
1407acdc3: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acdca: 488d15af602d02               lea rdx, [rip + 0x22d60af]
1407acdd1: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407acdd8: e8d3c60800                   call 0x1408394b0
1407acddd: 0f57c0                       xorps xmm0, xmm0
1407acde0: f3410f5fc2                   maxss xmm0, xmm10
1407acde5: f30f100d678b1702             movss xmm1, dword ptr [rip + 0x2178b67]
1407acded: f30f5dc8                     minss xmm1, xmm0
1407acdf1: f30f1108                     movss dword ptr [rax], xmm1
1407acdf5: 488b95b0050000               mov rdx, qword ptr [rbp + 0x5b0]
1407acdfc: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ace03: 488d8dc0040000               lea rcx, [rbp + 0x4c0]
1407ace0a: e8811a1500                   call 0x1408fe890
1407ace0f: 488bbdc0040000               mov rdi, qword ptr [rbp + 0x4c0]
1407ace16: 4883ff03                     cmp rdi, 3
1407ace1a: 0f8410020000                 je 0x1407ad030
1407ace20: 488b9dc8040000               mov rbx, qword ptr [rbp + 0x4c8]
1407ace27: 8bb5d0040000                 mov esi, dword ptr [rbp + 0x4d0]
1407ace2d: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ace34: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1407ace3b: 4889fa                       mov rdx, rdi
1407ace3e: 4989d8                       mov r8, rbx
1407ace41: 4189f1                       mov r9d, esi
1407ace44: e8a7f72600                   call 0x140a1c5f0
1407ace49: 4885c0                       test rax, rax
1407ace4c: 0f84de010000                 je 0x1407ad030
1407ace52: 488d8d30020000               lea rcx, [rbp + 0x230]
1407ace59: 4889c2                       mov rdx, rax
1407ace5c: e84fccffff                   call 0x1407a9ab0
1407ace61: 84c0                         test al, al
1407ace63: 741f                         je 0x1407ace84
1407ace65: 483bbd40050000               cmp rdi, qword ptr [rbp + 0x540]
1407ace6c: 7516                         jne 0x1407ace84
1407ace6e: 4885ff                       test rdi, rdi
1407ace71: 0f8552030000                 jne 0x1407ad1c9
1407ace77: 483b9d48050000               cmp rbx, qword ptr [rbp + 0x548]
1407ace7e: 0f8445030000                 je 0x1407ad1c9
1407ace84: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407ace8b: 8078693f                     cmp byte ptr [rax + 0x69], 0x3f
1407ace8f: 0f859b010000                 jne 0x1407ad030
1407ace95: 488b8db0050000               mov rcx, qword ptr [rbp + 0x5b0]
1407ace9c: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acea3: e8081a1500                   call 0x1408fe8b0
1407acea8: 4889c7                       mov rdi, rax
1407aceab: 89b5e0050000                 mov dword ptr [rbp + 0x5e0], esi
1407aceb1: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aceb8: 488d95e0050000               lea rdx, [rbp + 0x5e0]
1407acebf: 4889c1                       mov rcx, rax
1407acec2: e8890fab00                   call 0x14125de50
1407acec7: 4885c0                       test rax, rax
1407aceca: 7446                         je 0x1407acf12
1407acecc: 488b4808                     mov rcx, qword ptr [rax + 8]
1407aced0: 488b4010                     mov rax, qword ptr [rax + 0x10]
1407aced4: 488d04c1                     lea rax, [rcx + rax*8]
1407aced8: eb42                         jmp 0x1407acf1c
1407aceda: 4c8b85e8050000               mov r8, qword ptr [rbp + 0x5e8]
1407acee1: 448b8df0050000               mov r9d, dword ptr [rbp + 0x5f0]
1407acee8: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407aceef: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1407acef6: e8f5f62600                   call 0x140a1c5f0
1407acefb: 4885c0                       test rax, rax
1407acefe: 0f8474030000                 je 0x1407ad278
1407acf04: 8b4828                       mov ecx, dword ptr [rax + 0x28]
1407acf07: 898de0060000                 mov dword ptr [rbp + 0x6e0], ecx
1407acf0d: e966030000                   jmp 0x1407ad278
1407acf12: b804000000                   mov eax, 4
1407acf17: b904000000                   mov ecx, 4
1407acf1c: 4883c748                     add rdi, 0x48
1407acf20: 4889bde0050000               mov qword ptr [rbp + 0x5e0], rdi
1407acf27: 48898de8050000               mov qword ptr [rbp + 0x5e8], rcx
1407acf2e: 488985f0050000               mov qword ptr [rbp + 0x5f0], rax
1407acf35: 89b5f8050000                 mov dword ptr [rbp + 0x5f8], esi
1407acf3b: 488b8568040000               mov rax, qword ptr [rbp + 0x468]
1407acf42: 488b38                       mov rdi, qword ptr [rax]
1407acf45: 488d9de0020000               lea rbx, [rbp + 0x2e0]
1407acf4c: 4c8db5e0050000               lea r14, [rbp + 0x5e0]
1407acf53: 4c8dbda8030000               lea r15, [rbp + 0x3a8]
1407acf5a: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acf61: 4889d9                       mov rcx, rbx
1407acf64: 4c89f2                       mov rdx, r14
1407acf67: e8f4e1ab00                   call 0x14126b160
1407acf6c: 4883bde802000000             cmp qword ptr [rbp + 0x2e8], 0
1407acf74: 7474                         je 0x1407acfea
1407acf76: 8b8de0020000                 mov ecx, dword ptr [rbp + 0x2e0]
1407acf7c: 8b95e4020000                 mov edx, dword ptr [rbp + 0x2e4]
1407acf82: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acf89: e8d2421500                   call 0x140901260
1407acf8e: 8985a8030000                 mov dword ptr [rbp + 0x3a8], eax
1407acf94: 8995ac030000                 mov dword ptr [rbp + 0x3ac], edx
1407acf9a: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acfa1: 4889f9                       mov rcx, rdi
1407acfa4: 4c89fa                       mov rdx, r15
1407acfa7: e814c6ffff                   call 0x1407a95c0
1407acfac: 84c0                         test al, al
1407acfae: 74aa                         je 0x1407acf5a
1407acfb0: 488b8d68040000               mov rcx, qword ptr [rbp + 0x468]
1407acfb7: 8b411c                       mov eax, dword ptr [rcx + 0x1c]
1407acfba: 488b4910                     mov rcx, qword ptr [rcx + 0x10]
1407acfbe: 8901                         mov dword ptr [rcx], eax
1407acfc0: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acfc7: 89f1                         mov ecx, esi
1407acfc9: 8b95d8060000                 mov edx, dword ptr [rbp + 0x6d8]
1407acfcf: e88c421500                   call 0x140901260
1407acfd4: 4189d0                       mov r8d, edx
1407acfd7: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407acfde: 4889f9                       mov rcx, rdi
1407acfe1: 89c2                         mov edx, eax
1407acfe3: e858b6b1ff                   call 0x1402c8640
1407acfe8: eb46                         jmp 0x1407ad030
1407acfea: 488b8d68040000               mov rcx, qword ptr [rbp + 0x468]
1407acff1: 8b411c                       mov eax, dword ptr [rcx + 0x1c]
1407acff4: 488b4910                     mov rcx, qword ptr [rcx + 0x10]
1407acff8: 8901                         mov dword ptr [rcx], eax
1407acffa: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad001: 89f1                         mov ecx, esi
1407ad003: 8b95d8060000                 mov edx, dword ptr [rbp + 0x6d8]
1407ad009: e852421500                   call 0x140901260
1407ad00e: 8985e0050000                 mov dword ptr [rbp + 0x5e0], eax
1407ad014: 8995e4050000                 mov dword ptr [rbp + 0x5e4], edx
1407ad01a: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad021: 488d95e0050000               lea rdx, [rbp + 0x5e0]
1407ad028: 4889f9                       mov rcx, rdi
1407ad02b: e870acab00                   call 0x141267ca0
1407ad030: 4180fd02                     cmp r13b, 2
1407ad034: 0f92c0                       setb al
1407ad037: 488b8de8060000               mov rcx, qword ptr [rbp + 0x6e8]
1407ad03e: 0fb65969                     movzx ebx, byte ptr [rcx + 0x69]
1407ad042: 80fb3f                       cmp bl, 0x3f
1407ad045: 0f94c1                       sete cl
1407ad048: 84c8                         test al, cl
1407ad04a: 7425                         je 0x1407ad071
1407ad04c: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad053: 488d15565e2d02               lea rdx, [rip + 0x22d5e56]
1407ad05a: 488d8d50010000               lea rcx, [rbp + 0x150]
1407ad061: e8fa14ab00                   call 0x14125e560
1407ad066: 488b85e8060000               mov rax, qword ptr [rbp + 0x6e8]
1407ad06d: 0fb65869                     movzx ebx, byte ptr [rax + 0x69]
1407ad071: 0f28b540050000               movaps xmm6, xmmword ptr [rbp + 0x540]
1407ad078: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad07f: e8ecea88ff                   call 0x14003bb70
1407ad084: 4180fd02                     cmp r13b, 2
1407ad088: 0f92c1                       setb cl
1407ad08b: 8b95f0040000                 mov edx, dword ptr [rbp + 0x4f0]
1407ad091: 899530060000                 mov dword ptr [rbp + 0x630], edx
1407ad097: 0f1085e0040000               movups xmm0, xmmword ptr [rbp + 0x4e0]
1407ad09e: 0f118520060000               movups xmmword ptr [rbp + 0x620], xmm0
1407ad0a5: 488b95e8060000               mov rdx, qword ptr [rbp + 0x6e8]
1407ad0ac: 0fb65268                     movzx edx, byte ptr [rdx + 0x68]
1407ad0b0: 80f201                       xor dl, 1
1407ad0b3: 08ca                         or dl, cl
1407ad0b5: 8b8df4040000                 mov ecx, dword ptr [rbp + 0x4f4]
1407ad0bb: 4c8b8550020000               mov r8, qword ptr [rbp + 0x250]
1407ad0c2: 4c898518060000               mov qword ptr [rbp + 0x618], r8
1407ad0c9: 0f108530020000               movups xmm0, xmmword ptr [rbp + 0x230]
1407ad0d0: 0f108d40020000               movups xmm1, xmmword ptr [rbp + 0x240]
1407ad0d7: 0f118d08060000               movups xmmword ptr [rbp + 0x608], xmm1
1407ad0de: 0f1185f8050000               movups xmmword ptr [rbp + 0x5f8], xmm0
1407ad0e5: 0f11b5e8050000               movups xmmword ptr [rbp + 0x5e8], xmm6
1407ad0ec: 898d34060000                 mov dword ptr [rbp + 0x634], ecx
1407ad0f2: 8b8de0060000                 mov ecx, dword ptr [rbp + 0x6e0]
1407ad0f8: 898d38060000                 mov dword ptr [rbp + 0x638], ecx
1407ad0fe: 88953c060000                 mov byte ptr [rbp + 0x63c], dl
1407ad104: 889d3d060000                 mov byte ptr [rbp + 0x63d], bl
1407ad10a: 88853e060000                 mov byte ptr [rbp + 0x63e], al
1407ad110: 48c785e005000003000000       mov qword ptr [rbp + 0x5e0], 3
1407ad11b: 418b442408                   mov eax, dword ptr [r12 + 8]
1407ad120: 8985c8040000                 mov dword ptr [rbp + 0x4c8], eax
1407ad126: 498b0424                     mov rax, qword ptr [r12]
1407ad12a: 488985c0040000               mov qword ptr [rbp + 0x4c0], rax
1407ad131: c685cc04000000               mov byte ptr [rbp + 0x4cc], 0
1407ad138: 488b85a8050000               mov rax, qword ptr [rbp + 0x5a8]
1407ad13f: 8b8dac060000                 mov ecx, dword ptr [rbp + 0x6ac]
1407ad145: 8908                         mov dword ptr [rax], ecx
1407ad147: 488b8520050000               mov rax, qword ptr [rbp + 0x520]
1407ad14e: 8b8da8060000                 mov ecx, dword ptr [rbp + 0x6a8]
1407ad154: 8908                         mov dword ptr [rax], ecx
1407ad156: 8b85d4060000                 mov eax, dword ptr [rbp + 0x6d4]
1407ad15c: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1407ad163: 8901                         mov dword ptr [rcx], eax
1407ad165: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad16c: 488b8518050000               mov rax, qword ptr [rbp + 0x518]
1407ad173: 4889442438                   mov qword ptr [rsp + 0x38], rax
1407ad178: 488d85c0010000               lea rax, [rbp + 0x1c0]
1407ad17f: 4889442430                   mov qword ptr [rsp + 0x30], rax
1407ad184: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1407ad18b: 4889442428                   mov qword ptr [rsp + 0x28], rax
1407ad190: 488b8510050000               mov rax, qword ptr [rbp + 0x510]
1407ad197: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407ad19c: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407ad1a3: 4c8d85c0040000               lea r8, [rbp + 0x4c0]
1407ad1aa: 0fb695b8060000               movzx edx, byte ptr [rbp + 0x6b8]
1407ad1b1: 4c8b8dc0060000               mov r9, qword ptr [rbp + 0x6c0]
1407ad1b8: e8739cac00                   call 0x141276e30
1407ad1bd: 31ff                         xor edi, edi
1407ad1bf: 488b85d0030000               mov rax, qword ptr [rbp + 0x3d0]
1407ad1c6: 408838                       mov byte ptr [rax], dil
1407ad1c9: 0f28b500070000               movaps xmm6, xmmword ptr [rbp + 0x700]
1407ad1d0: 0f28bd10070000               movaps xmm7, xmmword ptr [rbp + 0x710]
1407ad1d7: 440f288520070000             movaps xmm8, xmmword ptr [rbp + 0x720]
1407ad1df: 440f288d30070000             movaps xmm9, xmmword ptr [rbp + 0x730]
1407ad1e7: 440f289540070000             movaps xmm10, xmmword ptr [rbp + 0x740]
1407ad1ef: 440f289d50070000             movaps xmm11, xmmword ptr [rbp + 0x750]
1407ad1f7: 440f28a560070000             movaps xmm12, xmmword ptr [rbp + 0x760]
1407ad1ff: 440f28ad70070000             movaps xmm13, xmmword ptr [rbp + 0x770]
1407ad207: 440f28b580070000             movaps xmm14, xmmword ptr [rbp + 0x780]
1407ad20f: 440f28bd90070000             movaps xmm15, xmmword ptr [rbp + 0x790]
1407ad217: 4881c428080000               add rsp, 0x828
1407ad21e: 5b                           pop rbx
1407ad21f: 5f                           pop rdi
1407ad220: 5e                           pop rsi
1407ad221: 415c                         pop r12
1407ad223: 415d                         pop r13
1407ad225: 415e                         pop r14
1407ad227: 415f                         pop r15
1407ad229: 5d                           pop rbp
1407ad22a: c3                           ret 
1407ad22b: 488b85a0060000               mov rax, qword ptr [rbp + 0x6a0]
1407ad232: 8b4008                       mov eax, dword ptr [rax + 8]
1407ad235: 8985e0060000                 mov dword ptr [rbp + 0x6e0], eax
1407ad23b: 488b95e8060000               mov rdx, qword ptr [rbp + 0x6e8]
1407ad242: e970f8ffff                   jmp 0x1407acab7
1407ad247: f30f593d21861702             mulss xmm7, dword ptr [rip + 0x2178621]
1407ad24f: 0f28c6                       movaps xmm0, xmm6
1407ad252: f30f58c7                     addss xmm0, xmm7
1407ad256: 440f2ec8                     ucomiss xmm9, xmm0
1407ad25a: 763b                         jbe 0x1407ad297
1407ad25c: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad263: 488d15e65b2d02               lea rdx, [rip + 0x22d5be6]
1407ad26a: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407ad271: e83ac20800                   call 0x1408394b0
1407ad276: eb6a                         jmp 0x1407ad2e2
1407ad278: 488b95e8060000               mov rdx, qword ptr [rbp + 0x6e8]
1407ad27f: 4885c0                       test rax, rax
1407ad282: 0f852ff8ffff                 jne 0x1407acab7
1407ad288: 488b85a0060000               mov rax, qword ptr [rbp + 0x6a0]
1407ad28f: 8b4008                       mov eax, dword ptr [rax + 8]
1407ad292: e91af8ffff                   jmp 0x1407acab1
1407ad297: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad29e: 488d15935b2d02               lea rdx, [rip + 0x22d5b93]
1407ad2a5: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407ad2ac: e8ffc10800                   call 0x1408394b0
1407ad2b1: f3440f58df                   addss xmm11, xmm7
1407ad2b6: f3440f5cde                   subss xmm11, xmm6
1407ad2bb: 0f57c0                       xorps xmm0, xmm0
1407ad2be: f3410f5fc3                   maxss xmm0, xmm11
1407ad2c3: e91dfbffff                   jmp 0x1407acde5
1407ad2c8: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad2cf: 488d15925b2d02               lea rdx, [rip + 0x22d5b92]
1407ad2d6: 488d8de0040000               lea rcx, [rbp + 0x4e0]
1407ad2dd: e8cec10800                   call 0x1408394b0
1407ad2e2: c70000000000                 mov dword ptr [rax], 0
1407ad2e8: e908fbffff                   jmp 0x1407acdf5
1407ad2ed: 31ff                         xor edi, edi
1407ad2ef: 4c8d051a4d4702               lea r8, [rip + 0x2474d1a]
1407ad2f6: 4889f9                       mov rcx, rdi
1407ad2f9: 4c89f2                       mov rdx, r14
1407ad2fc: e8e2bc1202                   call 0x1428d8fe3
1407ad301: eb58                         jmp 0x1407ad35b
1407ad303: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad30a: 488d0d075a2d02               lea rcx, [rip + 0x22d5a07]
1407ad311: e8fabf1202                   call 0x1428d9310
1407ad316: eb43                         jmp 0x1407ad35b
1407ad318: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad31f: 4c8d05825a2d02               lea r8, [rip + 0x22d5a82]
1407ad326: 4889c1                       mov rcx, rax
1407ad329: e8eac11202                   call 0x1428d9518
1407ad32e: eb2b                         jmp 0x1407ad35b
1407ad330: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad337: 488d0de25a2d02               lea rcx, [rip + 0x22d5ae2]
1407ad33e: e8cdbf1202                   call 0x1428d9310
1407ad343: eb16                         jmp 0x1407ad35b
1407ad345: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad34c: 4c8d056d5a2d02               lea r8, [rip + 0x22d5a6d]
1407ad353: 4889c1                       mov rcx, rax
1407ad356: e8bdc11202                   call 0x1428d9518
1407ad35b: 0f0b                         ud2 
1407ad35d: 0f1f00                       nop dword ptr [rax]
1407ad360: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407ad365: 55                           push rbp
1407ad366: 4157                         push r15
1407ad368: 4156                         push r14
1407ad36a: 4155                         push r13
1407ad36c: 4154                         push r12
1407ad36e: 56                           push rsi
1407ad36f: 57                           push rdi
1407ad370: 53                           push rbx
1407ad371: 4881ec38010000               sub rsp, 0x138
1407ad378: 488daa80000000               lea rbp, [rdx + 0x80]
1407ad37f: 440f29bc2490000000           movaps xmmword ptr [rsp + 0x90], xmm15
1407ad388: 440f29b424a0000000           movaps xmmword ptr [rsp + 0xa0], xmm14
1407ad391: 440f29ac24b0000000           movaps xmmword ptr [rsp + 0xb0], xmm13
1407ad39a: 66440f7fa424c0000000         movdqa xmmword ptr [rsp + 0xc0], xmm12
1407ad3a4: 440f299c24d0000000           movaps xmmword ptr [rsp + 0xd0], xmm11
1407ad3ad: 440f299424e0000000           movaps xmmword ptr [rsp + 0xe0], xmm10
1407ad3b6: 440f298c24f0000000           movaps xmmword ptr [rsp + 0xf0], xmm9
1407ad3bf: 440f29842400010000           movaps xmmword ptr [rsp + 0x100], xmm8
1407ad3c8: 0f29bc2410010000             movaps xmmword ptr [rsp + 0x110], xmm7
1407ad3d0: 0f29b42420010000             movaps xmmword ptr [rsp + 0x120], xmm6
1407ad3d8: 4883bdb006000000             cmp qword ptr [rbp + 0x6b0], 0
1407ad3e0: 741d                         je 0x1407ad3ff
1407ad3e2: 488b95b0060000               mov rdx, qword ptr [rbp + 0x6b0]
1407ad3e9: 48c1e202                     shl rdx, 2
1407ad3ed: 41b804000000                 mov r8d, 4
1407ad3f3: 488b8dd8060000               mov rcx, qword ptr [rbp + 0x6d8]
1407ad3fa: e82168e6ff                   call 0x140613c20
1407ad3ff: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad406: 0f28b42420010000             movaps xmm6, xmmword ptr [rsp + 0x120]
1407ad40e: 0f28bc2410010000             movaps xmm7, xmmword ptr [rsp + 0x110]
1407ad416: 440f28842400010000           movaps xmm8, xmmword ptr [rsp + 0x100]
1407ad41f: 440f288c24f0000000           movaps xmm9, xmmword ptr [rsp + 0xf0]
1407ad428: 440f289424e0000000           movaps xmm10, xmmword ptr [rsp + 0xe0]
1407ad431: 440f289c24d0000000           movaps xmm11, xmmword ptr [rsp + 0xd0]
1407ad43a: 440f28a424c0000000           movaps xmm12, xmmword ptr [rsp + 0xc0]
1407ad443: 440f28ac24b0000000           movaps xmm13, xmmword ptr [rsp + 0xb0]
1407ad44c: 440f28b424a0000000           movaps xmm14, xmmword ptr [rsp + 0xa0]
1407ad455: 440f28bc2490000000           movaps xmm15, xmmword ptr [rsp + 0x90]
1407ad45e: 4881c438010000               add rsp, 0x138
1407ad465: 5b                           pop rbx
1407ad466: 5f                           pop rdi
1407ad467: 5e                           pop rsi
1407ad468: 415c                         pop r12
1407ad46a: 415d                         pop r13
1407ad46c: 415e                         pop r14
1407ad46e: 415f                         pop r15
1407ad470: 5d                           pop rbp
1407ad471: c3                           ret 
1407ad472: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ad480: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407ad485: 55                           push rbp
1407ad486: 4157                         push r15
1407ad488: 4156                         push r14
1407ad48a: 4155                         push r13
1407ad48c: 4154                         push r12
1407ad48e: 56                           push rsi
1407ad48f: 57                           push rdi
1407ad490: 53                           push rbx
1407ad491: 4881ec38010000               sub rsp, 0x138
1407ad498: 488daa80000000               lea rbp, [rdx + 0x80]
1407ad49f: 440f29bc2490000000           movaps xmmword ptr [rsp + 0x90], xmm15
1407ad4a8: 440f29b424a0000000           movaps xmmword ptr [rsp + 0xa0], xmm14
1407ad4b1: 440f29ac24b0000000           movaps xmmword ptr [rsp + 0xb0], xmm13
1407ad4ba: 66440f7fa424c0000000         movdqa xmmword ptr [rsp + 0xc0], xmm12
1407ad4c4: 440f299c24d0000000           movaps xmmword ptr [rsp + 0xd0], xmm11
1407ad4cd: 440f299424e0000000           movaps xmmword ptr [rsp + 0xe0], xmm10
1407ad4d6: 440f298c24f0000000           movaps xmmword ptr [rsp + 0xf0], xmm9
1407ad4df: 440f29842400010000           movaps xmmword ptr [rsp + 0x100], xmm8
1407ad4e8: 0f29bc2410010000             movaps xmmword ptr [rsp + 0x110], xmm7
1407ad4f0: 0f29b42420010000             movaps xmmword ptr [rsp + 0x120], xmm6
1407ad4f8: 488d8d30050000               lea rcx, [rbp + 0x530]
1407ad4ff: e81ce599ff                   call 0x14014ba20
1407ad504: 0f28b42420010000             movaps xmm6, xmmword ptr [rsp + 0x120]
1407ad50c: 0f28bc2410010000             movaps xmm7, xmmword ptr [rsp + 0x110]
1407ad514: 440f28842400010000           movaps xmm8, xmmword ptr [rsp + 0x100]
1407ad51d: 440f288c24f0000000           movaps xmm9, xmmword ptr [rsp + 0xf0]
1407ad526: 440f289424e0000000           movaps xmm10, xmmword ptr [rsp + 0xe0]
1407ad52f: 440f289c24d0000000           movaps xmm11, xmmword ptr [rsp + 0xd0]
1407ad538: 66440f6fa424c0000000         movdqa xmm12, xmmword ptr [rsp + 0xc0]
1407ad542: 440f28ac24b0000000           movaps xmm13, xmmword ptr [rsp + 0xb0]
1407ad54b: 440f28b424a0000000           movaps xmm14, xmmword ptr [rsp + 0xa0]
1407ad554: 440f28bc2490000000           movaps xmm15, xmmword ptr [rsp + 0x90]
1407ad55d: 4881c438010000               add rsp, 0x138
1407ad564: 5b                           pop rbx
1407ad565: 5f                           pop rdi
1407ad566: 5e                           pop rsi
1407ad567: 415c                         pop r12
1407ad569: 415d                         pop r13
1407ad56b: 415e                         pop r14
1407ad56d: 415f                         pop r15
1407ad56f: 5d                           pop rbp
1407ad570: c3                           ret 
1407ad571: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ad580: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407ad585: 55                           push rbp
1407ad586: 4157                         push r15
1407ad588: 4156                         push r14
1407ad58a: 4155                         push r13
1407ad58c: 4154                         push r12
1407ad58e: 56                           push rsi
1407ad58f: 57                           push rdi
1407ad590: 53                           push rbx
1407ad591: 4881ec38010000               sub rsp, 0x138
1407ad598: 488daa80000000               lea rbp, [rdx + 0x80]
1407ad59f: 440f29bc2490000000           movaps xmmword ptr [rsp + 0x90], xmm15
1407ad5a8: 440f29b424a0000000           movaps xmmword ptr [rsp + 0xa0], xmm14
1407ad5b1: 440f29ac24b0000000           movaps xmmword ptr [rsp + 0xb0], xmm13
1407ad5ba: 440f29a424c0000000           movaps xmmword ptr [rsp + 0xc0], xmm12
1407ad5c3: 440f299c24d0000000           movaps xmmword ptr [rsp + 0xd0], xmm11
1407ad5cc: 440f299424e0000000           movaps xmmword ptr [rsp + 0xe0], xmm10
1407ad5d5: 440f298c24f0000000           movaps xmmword ptr [rsp + 0xf0], xmm9
1407ad5de: 440f29842400010000           movaps xmmword ptr [rsp + 0x100], xmm8
1407ad5e7: 0f29bc2410010000             movaps xmmword ptr [rsp + 0x110], xmm7
1407ad5ef: 0f29b42420010000             movaps xmmword ptr [rsp + 0x120], xmm6
1407ad5f7: 488d8de0050000               lea rcx, [rbp + 0x5e0]
1407ad5fe: e88d450d00                   call 0x140881b90
1407ad603: c685f706000001               mov byte ptr [rbp + 0x6f7], 1
1407ad60a: 0f28b42420010000             movaps xmm6, xmmword ptr [rsp + 0x120]
1407ad612: 0f28bc2410010000             movaps xmm7, xmmword ptr [rsp + 0x110]
1407ad61a: 440f28842400010000           movaps xmm8, xmmword ptr [rsp + 0x100]
1407ad623: 440f288c24f0000000           movaps xmm9, xmmword ptr [rsp + 0xf0]
1407ad62c: 440f289424e0000000           movaps xmm10, xmmword ptr [rsp + 0xe0]
1407ad635: 440f289c24d0000000           movaps xmm11, xmmword ptr [rsp + 0xd0]
1407ad63e: 440f28a424c0000000           movaps xmm12, xmmword ptr [rsp + 0xc0]
1407ad647: 440f28ac24b0000000           movaps xmm13, xmmword ptr [rsp + 0xb0]
1407ad650: 440f28b424a0000000           movaps xmm14, xmmword ptr [rsp + 0xa0]
1407ad659: 440f28bc2490000000           movaps xmm15, xmmword ptr [rsp + 0x90]
1407ad662: 4881c438010000               add rsp, 0x138
1407ad669: 5b                           pop rbx
1407ad66a: 5f                           pop rdi
1407ad66b: 5e                           pop rsi
1407ad66c: 415c                         pop r12
1407ad66e: 415d                         pop r13
1407ad670: 415e                         pop r14
1407ad672: 415f                         pop r15
1407ad674: 5d                           pop rbp
1407ad675: c3                           ret 
1407ad676: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1407ad680: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407ad685: 55                           push rbp
1407ad686: 4157                         push r15
1407ad688: 4156                         push r14
1407ad68a: 4155                         push r13
1407ad68c: 4154                         push r12
1407ad68e: 56                           push rsi
1407ad68f: 57                           push rdi
1407ad690: 53                           push rbx
1407ad691: 4881ec38010000               sub rsp, 0x138
1407ad698: 488daa80000000               lea rbp, [rdx + 0x80]
1407ad69f: 440f29bc2490000000           movaps xmmword ptr [rsp + 0x90], xmm15
1407ad6a8: 440f29b424a0000000           movaps xmmword ptr [rsp + 0xa0], xmm14
1407ad6b1: 440f29ac24b0000000           movaps xmmword ptr [rsp + 0xb0], xmm13
1407ad6ba: 66440f7fa424c0000000         movdqa xmmword ptr [rsp + 0xc0], xmm12
1407ad6c4: 440f299c24d0000000           movaps xmmword ptr [rsp + 0xd0], xmm11
1407ad6cd: 440f299424e0000000           movaps xmmword ptr [rsp + 0xe0], xmm10
1407ad6d6: 440f298c24f0000000           movaps xmmword ptr [rsp + 0xf0], xmm9
1407ad6df: 440f29842400010000           movaps xmmword ptr [rsp + 0x100], xmm8
1407ad6e8: 0f29bc2410010000             movaps xmmword ptr [rsp + 0x110], xmm7
1407ad6f0: 0f29b42420010000             movaps xmmword ptr [rsp + 0x120], xmm6
1407ad6f8: 4883bd9006000000             cmp qword ptr [rbp + 0x690], 0
1407ad700: 7421                         je 0x1407ad723
1407ad702: 488b8590060000               mov rax, qword ptr [rbp + 0x690]
1407ad709: 48c1e003                     shl rax, 3
1407ad70d: 488d1480                     lea rdx, [rax + rax*4]
1407ad711: 41b808000000                 mov r8d, 8
1407ad717: 488b8df8040000               mov rcx, qword ptr [rbp + 0x4f8]
1407ad71e: e8fd64e6ff                   call 0x140613c20
1407ad723: c685f706000000               mov byte ptr [rbp + 0x6f7], 0
1407ad72a: 0f28b42420010000             movaps xmm6, xmmword ptr [rsp + 0x120]
1407ad732: 0f28bc2410010000             movaps xmm7, xmmword ptr [rsp + 0x110]
1407ad73a: 440f28842400010000           movaps xmm8, xmmword ptr [rsp + 0x100]
1407ad743: 440f288c24f0000000           movaps xmm9, xmmword ptr [rsp + 0xf0]
1407ad74c: 440f289424e0000000           movaps xmm10, xmmword ptr [rsp + 0xe0]
1407ad755: 440f289c24d0000000           movaps xmm11, xmmword ptr [rsp + 0xd0]
1407ad75e: 440f28a424c0000000           movaps xmm12, xmmword ptr [rsp + 0xc0]
1407ad767: 440f28ac24b0000000           movaps xmm13, xmmword ptr [rsp + 0xb0]
1407ad770: 440f28b424a0000000           movaps xmm14, xmmword ptr [rsp + 0xa0]
1407ad779: 440f28bc2490000000           movaps xmm15, xmmword ptr [rsp + 0x90]
1407ad782: 4881c438010000               add rsp, 0x138
1407ad789: 5b                           pop rbx
1407ad78a: 5f                           pop rdi
1407ad78b: 5e                           pop rsi
1407ad78c: 415c                         pop r12
1407ad78e: 415d                         pop r13
1407ad790: 415e                         pop r14
1407ad792: 415f                         pop r15
1407ad794: 5d                           pop rbp
1407ad795: c3                           ret 
1407ad796: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1407ad7a0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407ad7a5: 55                           push rbp
1407ad7a6: 4157                         push r15
1407ad7a8: 4156                         push r14
1407ad7aa: 4155                         push r13
1407ad7ac: 4154                         push r12
1407ad7ae: 56                           push rsi
1407ad7af: 57                           push rdi
1407ad7b0: 53                           push rbx
1407ad7b1: 4881ec38010000               sub rsp, 0x138
1407ad7b8: 488daa80000000               lea rbp, [rdx + 0x80]
1407ad7bf: 440f29bc2490000000           movaps xmmword ptr [rsp + 0x90], xmm15
1407ad7c8: 440f29b424a0000000           movaps xmmword ptr [rsp + 0xa0], xmm14
1407ad7d1: 440f29ac24b0000000           movaps xmmword ptr [rsp + 0xb0], xmm13
1407ad7da: 440f29a424c0000000           movaps xmmword ptr [rsp + 0xc0], xmm12
1407ad7e3: 440f299c24d0000000           movaps xmmword ptr [rsp + 0xd0], xmm11
1407ad7ec: 440f299424e0000000           movaps xmmword ptr [rsp + 0xe0], xmm10
1407ad7f5: 440f298c24f0000000           movaps xmmword ptr [rsp + 0xf0], xmm9
1407ad7fe: 440f29842400010000           movaps xmmword ptr [rsp + 0x100], xmm8
1407ad807: 0f29bc2410010000             movaps xmmword ptr [rsp + 0x110], xmm7
1407ad80f: 0f29b42420010000             movaps xmmword ptr [rsp + 0x120], xmm6
1407ad817: 80bdf706000000               cmp byte ptr [rbp + 0x6f7], 0
1407ad81e: 7426                         je 0x1407ad846
1407ad820: 488b8570040000               mov rax, qword ptr [rbp + 0x470]
1407ad827: 4885c0                       test rax, rax
1407ad82a: 741a                         je 0x1407ad846
1407ad82c: 488b8d78040000               mov rcx, qword ptr [rbp + 0x478]
1407ad833: 48c1e003                     shl rax, 3
1407ad837: 488d1480                     lea rdx, [rax + rax*4]
1407ad83b: 41b808000000                 mov r8d, 8
1407ad841: e8da63e6ff                   call 0x140613c20
1407ad846: 0f28b42420010000             movaps xmm6, xmmword ptr [rsp + 0x120]
1407ad84e: 0f28bc2410010000             movaps xmm7, xmmword ptr [rsp + 0x110]
1407ad856: 440f28842400010000           movaps xmm8, xmmword ptr [rsp + 0x100]
1407ad85f: 440f288c24f0000000           movaps xmm9, xmmword ptr [rsp + 0xf0]
1407ad868: 440f289424e0000000           movaps xmm10, xmmword ptr [rsp + 0xe0]
1407ad871: 440f289c24d0000000           movaps xmm11, xmmword ptr [rsp + 0xd0]
1407ad87a: 440f28a424c0000000           movaps xmm12, xmmword ptr [rsp + 0xc0]
1407ad883: 440f28ac24b0000000           movaps xmm13, xmmword ptr [rsp + 0xb0]
1407ad88c: 440f28b424a0000000           movaps xmm14, xmmword ptr [rsp + 0xa0]
1407ad895: 440f28bc2490000000           movaps xmm15, xmmword ptr [rsp + 0x90]
1407ad89e: 4881c438010000               add rsp, 0x138
1407ad8a5: 5b                           pop rbx
1407ad8a6: 5f                           pop rdi
1407ad8a7: 5e                           pop rsi
1407ad8a8: 415c                         pop r12
1407ad8aa: 415d                         pop r13
1407ad8ac: 415e                         pop r14
1407ad8ae: 415f                         pop r15
1407ad8b0: 5d                           pop rbp
1407ad8b1: c3                           ret 
