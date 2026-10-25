142315e40: 4157                     push r15
142315e42: 4156                     push r14
142315e44: 4155                     push r13
142315e46: 4154                     push r12
142315e48: 56                       push rsi
142315e49: 57                       push rdi
142315e4a: 55                       push rbp
142315e4b: 53                       push rbx
142315e4c: 4881ec98010000           sub rsp, 0x198
142315e53: 440f29842480010000       movaps xmmword ptr [rsp + 0x180], xmm8
142315e5c: 0f29bc2470010000         movaps xmmword ptr [rsp + 0x170], xmm7
142315e64: 0f29b42460010000         movaps xmmword ptr [rsp + 0x160], xmm6
142315e6c: 488b19                   mov rbx, qword ptr [rcx]
142315e6f: 4c8b7908                 mov r15, qword ptr [rcx + 8]
142315e73: 448b4910                 mov r9d, dword ptr [rcx + 0x10]
142315e77: 8b4114                   mov eax, dword ptr [rcx + 0x14]
142315e7a: 498db7a0010000           lea rsi, [r15 + 0x1a0]
142315e81: 498dbff8000000           lea rdi, [r15 + 0xf8]
142315e88: 488d8c24c0000000         lea rcx, [rsp + 0xc0]
142315e90: 89442420                 mov dword ptr [rsp + 0x20], eax
142315e94: 4c89fa                   mov rdx, r15
142315e97: 4989d8                   mov r8, rbx
142315e9a: e8518dffff               call 0x14230ebf0
142315e9f: 4989da                   mov r10, rbx
142315ea2: 4c89bc24a0000000         mov qword ptr [rsp + 0xa0], r15
142315eaa: 4889b424a8000000         mov qword ptr [rsp + 0xa8], rsi
142315eb2: 4889bc24b0000000         mov qword ptr [rsp + 0xb0], rdi
142315eba: 48899c24b8000000         mov qword ptr [rsp + 0xb8], rbx
142315ec2: 488b9c24c0000000         mov rbx, qword ptr [rsp + 0xc0]
142315eca: 4c8bb424c8000000         mov r14, qword ptr [rsp + 0xc8]
142315ed2: 4c8b8424e0000000         mov r8, qword ptr [rsp + 0xe0]
142315eda: 488b8424f0000000         mov rax, qword ptr [rsp + 0xf0]
142315ee2: 4889442460               mov qword ptr [rsp + 0x60], rax
142315ee7: 488b842400010000         mov rax, qword ptr [rsp + 0x100]
142315eef: 4889442458               mov qword ptr [rsp + 0x58], rax
142315ef4: 448b9c240c010000         mov r11d, dword ptr [rsp + 0x10c]
142315efc: 4c8b8c2410010000         mov r9, qword ptr [rsp + 0x110]
142315f04: 488b842418010000         mov rax, qword ptr [rsp + 0x118]
142315f0c: 4889442440               mov qword ptr [rsp + 0x40], rax
142315f11: 488b842420010000         mov rax, qword ptr [rsp + 0x120]
142315f19: 4889442438               mov qword ptr [rsp + 0x38], rax
142315f1e: 488b842428010000         mov rax, qword ptr [rsp + 0x128]
142315f26: 4889442450               mov qword ptr [rsp + 0x50], rax
142315f2b: 488b842438010000         mov rax, qword ptr [rsp + 0x138]
142315f33: 4889442448               mov qword ptr [rsp + 0x48], rax
142315f38: 8bb42444010000           mov esi, dword ptr [rsp + 0x144]
142315f3f: 4c8bac2448010000         mov r13, qword ptr [rsp + 0x148]
142315f47: 488b8c2450010000         mov rcx, qword ptr [rsp + 0x150]
142315f4f: 440fb6a42458010000       movzx r12d, byte ptr [rsp + 0x158]
142315f58: f30f1035a4f96000         movss xmm6, dword ptr [rip + 0x60f9a4]
142315f60: eb65                     jmp 0x142315fc7
142315f62: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142315f70: 450f57c0                 xorps xmm8, xmm8
142315f74: 488b4c2468               mov rcx, qword ptr [rsp + 0x68]
142315f79: 48ffc1                   inc rcx
142315f7c: 488b542478               mov rdx, qword ptr [rsp + 0x78]
142315f81: 44891a                   mov dword ptr [rdx], r11d
142315f84: f30f12c0                 movsldup xmm0, xmm0
142315f88: 0f59f8                   mulps xmm7, xmm0
142315f8b: 0fb6c0                   movzx eax, al
142315f8e: 660f6ec0                 movd xmm0, eax
142315f92: 660f70c050               pshufd xmm0, xmm0, 0x50
142315f97: 660f72f01f               pslld xmm0, 0x1f
142315f9c: 660f72e01f               psrad xmm0, 0x1f
142315fa1: 660fdbc7                 pand xmm0, xmm7
142315fa5: 660fd607                 movq qword ptr [rdi], xmm0
142315fa9: f3440f114708             movss dword ptr [rdi + 8], xmm8
142315faf: 488b842480000000         mov rax, qword ptr [rsp + 0x80]
142315fb7: 8930                     mov dword ptr [rax], esi
142315fb9: 488b842488000000         mov rax, qword ptr [rsp + 0x88]
142315fc1: c7000000803f             mov dword ptr [rax], 0x3f800000
142315fc7: 41f6c401                 test r12b, 1
142315fcb: 7443                     je 0x142316010
142315fcd: 4c39e9                   cmp rcx, r13
142315fd0: 0f8522020000             jne 0x1423161f8
142315fd6: 4c39f3                   cmp rbx, r14
142315fd9: 0f847d040000             je 0x14231645c
142315fdf: 498b87a8010000           mov rax, qword ptr [r15 + 0x1a8]
142315fe6: 662e0f1f840000000000     nop word ptr cs:[rax + rax]
142315ff0: 8b0b                     mov ecx, dword ptr [rbx]
142315ff2: 488d0cc9                 lea rcx, [rcx + rcx*8]
142315ff6: 4c8b6cc810               mov r13, qword ptr [rax + rcx*8 + 0x10]
142315ffb: 4d85ed                   test r13, r13
142315ffe: 7552                     jne 0x142316052
142316000: 4883c304                 add rbx, 4
142316004: 4c39f3                   cmp rbx, r14
142316007: 75e7                     jne 0x142315ff0
142316009: e94e040000               jmp 0x14231645c
14231600e: 6690                     nop 
142316010: 4c39e9                   cmp rcx, r13
142316013: 0f85dc020000             jne 0x1423162f5
142316019: 4c39f3                   cmp rbx, r14
14231601c: 0f843a040000             je 0x14231645c
142316022: 498b8700010000           mov rax, qword ptr [r15 + 0x100]
142316029: 0f1f8000000000           nop dword ptr [rax]
142316030: 8b0b                     mov ecx, dword ptr [rbx]
142316032: 488d0c89                 lea rcx, [rcx + rcx*4]
142316036: 48c1e105                 shl rcx, 5
14231603a: 4c8b6c0858               mov r13, qword ptr [rax + rcx + 0x58]
14231603f: 4d85ed                   test r13, r13
142316042: 755f                     jne 0x1423160a3
142316044: 4883c304                 add rbx, 4
142316048: 4c39f3                   cmp rbx, r14
14231604b: 75e3                     jne 0x142316030
14231604d: e90a040000               jmp 0x14231645c
142316052: 488d04c8                 lea rax, [rax + rcx*8]
142316056: 498b9210010000           mov rdx, qword ptr [r10 + 0x110]
14231605d: 488b4840                 mov rcx, qword ptr [rax + 0x40]
142316061: 4c89c7                   mov rdi, r8
142316064: 4839d1                   cmp rcx, rdx
142316067: 0f869d000000             jbe 0x14231610a
14231606d: 4c8b4038                 mov r8, qword ptr [rax + 0x38]
142316071: 498b14d0                 mov rdx, qword ptr [r8 + rdx*8]
142316075: 4885d2                   test rdx, rdx
142316078: 0f848c000000             je 0x14231610a
14231607e: 4c8b4018                 mov r8, qword ptr [rax + 0x18]
142316082: 48f7d2                   not rdx
142316085: 488d1452                 lea rdx, [rdx + rdx*2]
142316089: 48c1e204                 shl rdx, 4
14231608d: 4d8b4c1010               mov r9, qword ptr [r8 + rdx + 0x10]
142316092: 498b9218010000           mov rdx, qword ptr [r10 + 0x118]
142316099: 4839d1                   cmp rcx, rdx
14231609c: 777b                     ja 0x142316119
14231609e: e9b2000000               jmp 0x142316155
1423160a3: 4801c8                   add rax, rcx
1423160a6: 8b8894000000             mov ecx, dword ptr [rax + 0x94]
1423160ac: 498b97a8010000           mov rdx, qword ptr [r15 + 0x1a8]
1423160b3: 4c8d0cc9                 lea r9, [rcx + rcx*8]
1423160b7: 4a8d0cca                 lea rcx, [rdx + r9*8]
1423160bb: 4d8b8210010000           mov r8, qword ptr [r10 + 0x110]
1423160c2: 4a8b54ca40               mov rdx, qword ptr [rdx + r9*8 + 0x40]
1423160c7: 4c39c2                   cmp rdx, r8
1423160ca: 0f8634010000             jbe 0x142316204
1423160d0: 4c8b4938                 mov r9, qword ptr [rcx + 0x38]
1423160d4: 4f8b04c1                 mov r8, qword ptr [r9 + r8*8]
1423160d8: 4d85c0                   test r8, r8
1423160db: 0f8423010000             je 0x142316204
1423160e1: 4c8b4918                 mov r9, qword ptr [rcx + 0x18]
1423160e5: 49f7d0                   not r8
1423160e8: 4f8d0440                 lea r8, [r8 + r8*2]
1423160ec: 49c1e004                 shl r8, 4
1423160f0: 4b8b7c0110               mov rdi, qword ptr [r9 + r8 + 0x10]
1423160f5: 4d8b8218010000           mov r8, qword ptr [r10 + 0x118]
1423160fc: 4c39c2                   cmp rdx, r8
1423160ff: 0f870d010000             ja 0x142316212
142316105: e944010000               jmp 0x14231624e
14231610a: 4531c9                   xor r9d, r9d
14231610d: 498b9218010000           mov rdx, qword ptr [r10 + 0x118]
142316114: 4839d1                   cmp rcx, rdx
142316117: 763c                     jbe 0x142316155
142316119: 4c8b4038                 mov r8, qword ptr [rax + 0x38]
14231611d: 498b14d0                 mov rdx, qword ptr [r8 + rdx*8]
142316121: 4885d2                   test rdx, rdx
142316124: 742f                     je 0x142316155
142316126: 4c8b4018                 mov r8, qword ptr [rax + 0x18]
14231612a: 48f7d2                   not rdx
14231612d: 488d1452                 lea rdx, [rdx + rdx*2]
142316131: 48c1e204                 shl rdx, 4
142316135: 498b541010               mov rdx, qword ptr [r8 + rdx + 0x10]
14231613a: 4889542440               mov qword ptr [rsp + 0x40], rdx
14231613f: 4d8b8220010000           mov r8, qword ptr [r10 + 0x120]
142316146: 488b5038                 mov rdx, qword ptr [rax + 0x38]
14231614a: 4c39c1                   cmp rcx, r8
14231614d: 771f                     ja 0x14231616e
14231614f: 488b4018                 mov rax, qword ptr [rax + 0x18]
142316153: eb3d                     jmp 0x142316192
142316155: 48c744244000000000       mov qword ptr [rsp + 0x40], 0
14231615e: 4d8b8220010000           mov r8, qword ptr [r10 + 0x120]
142316165: 488b5038                 mov rdx, qword ptr [rax + 0x38]
142316169: 4c39c1                   cmp rcx, r8
14231616c: 76e1                     jbe 0x14231614f
14231616e: 4a8b0cc2                 mov rcx, qword ptr [rdx + r8*8]
142316172: 488b4018                 mov rax, qword ptr [rax + 0x18]
142316176: 4885c9                   test rcx, rcx
142316179: 7417                     je 0x142316192
14231617b: 48f7d1                   not rcx
14231617e: 488d0c49                 lea rcx, [rcx + rcx*2]
142316182: 48c1e104                 shl rcx, 4
142316186: 488b4c0810               mov rcx, qword ptr [rax + rcx + 0x10]
14231618b: 48894c2438               mov qword ptr [rsp + 0x38], rcx
142316190: eb09                     jmp 0x14231619b
142316192: 48c744243800000000       mov qword ptr [rsp + 0x38], 0
14231619b: 4883c304                 add rbx, 4
14231619f: 498b8a28010000           mov rcx, qword ptr [r10 + 0x128]
1423161a6: 4d8b8230010000           mov r8, qword ptr [r10 + 0x130]
1423161ad: 488b0cca                 mov rcx, qword ptr [rdx + rcx*8]
1423161b1: 48f7d1                   not rcx
1423161b4: 488d0c49                 lea rcx, [rcx + rcx*2]
1423161b8: 48c1e104                 shl rcx, 4
1423161bc: 488b6c0810               mov rbp, qword ptr [rax + rcx + 0x10]
1423161c1: 48896c2460               mov qword ptr [rsp + 0x60], rbp
1423161c6: 488b4c0828               mov rcx, qword ptr [rax + rcx + 0x28]
1423161cb: 48894c2458               mov qword ptr [rsp + 0x58], rcx
1423161d0: 4a8b0cc2                 mov rcx, qword ptr [rdx + r8*8]
1423161d4: 48f7d1                   not rcx
1423161d7: 488d0c49                 lea rcx, [rcx + rcx*2]
1423161db: 48c1e104                 shl rcx, 4
1423161df: 488b540810               mov rdx, qword ptr [rax + rcx + 0x10]
1423161e4: 4889542450               mov qword ptr [rsp + 0x50], rdx
1423161e9: 488b440828               mov rax, qword ptr [rax + rcx + 0x28]
1423161ee: 4889442448               mov qword ptr [rsp + 0x48], rax
1423161f3: 31c9                     xor ecx, ecx
1423161f5: 4989f8                   mov r8, rdi
1423161f8: 48894c2468               mov qword ptr [rsp + 0x68], rcx
1423161fd: 89cf                     mov edi, ecx
1423161ff: e902010000               jmp 0x142316306
142316204: 31ff                     xor edi, edi
142316206: 4d8b8218010000           mov r8, qword ptr [r10 + 0x118]
14231620d: 4c39c2                   cmp rdx, r8
142316210: 763c                     jbe 0x14231624e
142316212: 4c8b4938                 mov r9, qword ptr [rcx + 0x38]
142316216: 4f8b04c1                 mov r8, qword ptr [r9 + r8*8]
14231621a: 4d85c0                   test r8, r8
14231621d: 742f                     je 0x14231624e
14231621f: 4c8b4918                 mov r9, qword ptr [rcx + 0x18]
142316223: 49f7d0                   not r8
142316226: 4f8d0440                 lea r8, [r8 + r8*2]
14231622a: 49c1e004                 shl r8, 4
14231622e: 4f8b440110               mov r8, qword ptr [r9 + r8 + 0x10]
142316233: 4c89442440               mov qword ptr [rsp + 0x40], r8
142316238: 4d8b8a20010000           mov r9, qword ptr [r10 + 0x120]
14231623f: 4c8b4138                 mov r8, qword ptr [rcx + 0x38]
142316243: 4c39ca                   cmp rdx, r9
142316246: 771f                     ja 0x142316267
142316248: 488b4918                 mov rcx, qword ptr [rcx + 0x18]
14231624c: eb3d                     jmp 0x14231628b
14231624e: 48c744244000000000       mov qword ptr [rsp + 0x40], 0
142316257: 4d8b8a20010000           mov r9, qword ptr [r10 + 0x120]
14231625e: 4c8b4138                 mov r8, qword ptr [rcx + 0x38]
142316262: 4c39ca                   cmp rdx, r9
142316265: 76e1                     jbe 0x142316248
142316267: 4b8b14c8                 mov rdx, qword ptr [r8 + r9*8]
14231626b: 488b4918                 mov rcx, qword ptr [rcx + 0x18]
14231626f: 4885d2                   test rdx, rdx
142316272: 7417                     je 0x14231628b
142316274: 48f7d2                   not rdx
142316277: 488d1452                 lea rdx, [rdx + rdx*2]
14231627b: 48c1e204                 shl rdx, 4
14231627f: 488b541110               mov rdx, qword ptr [rcx + rdx + 0x10]
142316284: 4889542438               mov qword ptr [rsp + 0x38], rdx
142316289: eb09                     jmp 0x142316294
14231628b: 48c744243800000000       mov qword ptr [rsp + 0x38], 0
142316294: 4883c304                 add rbx, 4
142316298: 498b9228010000           mov rdx, qword ptr [r10 + 0x128]
14231629f: 4d8b8a30010000           mov r9, qword ptr [r10 + 0x130]
1423162a6: 498b14d0                 mov rdx, qword ptr [r8 + rdx*8]
1423162aa: 48f7d2                   not rdx
1423162ad: 488d1452                 lea rdx, [rdx + rdx*2]
1423162b1: 48c1e204                 shl rdx, 4
1423162b5: 488b6c1110               mov rbp, qword ptr [rcx + rdx + 0x10]
1423162ba: 48896c2460               mov qword ptr [rsp + 0x60], rbp
1423162bf: 488b541128               mov rdx, qword ptr [rcx + rdx + 0x28]
1423162c4: 4889542458               mov qword ptr [rsp + 0x58], rdx
1423162c9: 4b8b14c8                 mov rdx, qword ptr [r8 + r9*8]
1423162cd: 48f7d2                   not rdx
1423162d0: 488d1452                 lea rdx, [rdx + rdx*2]
1423162d4: 48c1e204                 shl rdx, 4
1423162d8: 4c8b441110               mov r8, qword ptr [rcx + rdx + 0x10]
1423162dd: 4c89442450               mov qword ptr [rsp + 0x50], r8
1423162e2: 488b4c1128               mov rcx, qword ptr [rcx + rdx + 0x28]
1423162e7: 48894c2448               mov qword ptr [rsp + 0x48], rcx
1423162ec: 4c8b4050                 mov r8, qword ptr [rax + 0x50]
1423162f0: 31c9                     xor ecx, ecx
1423162f2: 4989f9                   mov r9, rdi
1423162f5: 48894c2468               mov qword ptr [rsp + 0x68], rcx
1423162fa: 4889c8                   mov rax, rcx
1423162fd: 48c1e004                 shl rax, 4
142316301: 418b7c0008               mov edi, dword ptr [r8 + rax + 8]
142316306: 488d2c7f                 lea rbp, [rdi + rdi*2]
14231630a: 48c1e504                 shl rbp, 4
14231630e: 4c01cd                   add rbp, r9
142316311: 486bcf68                 imul rcx, rdi, 0x68
142316315: 48034c2440               add rcx, qword ptr [rsp + 0x40]
14231631a: 488b442458               mov rax, qword ptr [rsp + 0x58]
14231631f: 488d04b8                 lea rax, [rax + rdi*4]
142316323: 4889442478               mov qword ptr [rsp + 0x78], rax
142316328: 488b442450               mov rax, qword ptr [rsp + 0x50]
14231632d: 488d04b8                 lea rax, [rax + rdi*4]
142316331: 4889842488000000         mov qword ptr [rsp + 0x88], rax
142316339: 488b442448               mov rax, qword ptr [rsp + 0x48]
14231633e: 488d04b8                 lea rax, [rax + rdi*4]
142316342: 4889842480000000         mov qword ptr [rsp + 0x80], rax
14231634a: 48c1e704                 shl rdi, 4
14231634e: 488b442438               mov rax, qword ptr [rsp + 0x38]
142316353: 4801f8                   add rax, rdi
142316356: 48037c2460               add rdi, qword ptr [rsp + 0x60]
14231635b: f60001                   test byte ptr [rax], 1
14231635e: 7470                     je 0x1423163d0
142316360: 80793001                 cmp byte ptr [rcx + 0x30], 1
142316364: 756a                     jne 0x1423163d0
142316366: f20f107804               movsd xmm7, qword ptr [rax + 4]
14231636b: f3440f10400c             movss xmm8, dword ptr [rax + 0xc]
142316371: 4488642437               mov byte ptr [rsp + 0x37], r12b
142316376: 4c89bc2490000000         mov qword ptr [rsp + 0x90], r15
14231637e: 4d89d7                   mov r15, r10
142316381: 4c89b42498000000         mov qword ptr [rsp + 0x98], r14
142316389: 89742474                 mov dword ptr [rsp + 0x74], esi
14231638d: 4489de                   mov esi, r11d
142316390: 4d89ce                   mov r14, r9
142316393: 4d89c4                   mov r12, r8
142316396: e8257158fe               call 0x14089d4c0
14231639b: 4d89e0                   mov r8, r12
14231639e: 4d89f1                   mov r9, r14
1423163a1: 4189f3                   mov r11d, esi
1423163a4: 8b742474                 mov esi, dword ptr [rsp + 0x74]
1423163a8: 4c8bb42498000000         mov r14, qword ptr [rsp + 0x98]
1423163b0: 4d89fa                   mov r10, r15
1423163b3: 4c8bbc2490000000         mov r15, qword ptr [rsp + 0x90]
1423163bb: 440fb6642437             movzx r12d, byte ptr [rsp + 0x37]
1423163c1: 84c0                     test al, al
1423163c3: 0f8487000000             je 0x142316450
1423163c9: 0f1f8000000000           nop dword ptr [rax]
1423163d0: f20f107d10               movsd xmm7, qword ptr [rbp + 0x10]
1423163d5: f30f104518               movss xmm0, dword ptr [rbp + 0x18]
1423163da: 440f28c0                 movaps xmm8, xmm0
1423163de: 0f28cf                   movaps xmm1, xmm7
1423163e1: f3440f5cc0               subss xmm8, xmm0
1423163e6: 0f5cf9                   subps xmm7, xmm1
1423163e9: 0f28c7                   movaps xmm0, xmm7
1423163ec: 0f59c7                   mulps xmm0, xmm7
1423163ef: f30f16c8                 movshdup xmm1, xmm0
1423163f3: f30f58c8                 addss xmm1, xmm0
1423163f7: 410f28c0                 movaps xmm0, xmm8
1423163fb: f3410f59c0               mulss xmm0, xmm8
142316400: f30f58c1                 addss xmm0, xmm1
142316404: 0f57c9                   xorps xmm1, xmm1
142316407: f30f51c8                 sqrtss xmm1, xmm0
14231640b: 0f28c6                   movaps xmm0, xmm6
14231640e: f30f5ec1                 divss xmm0, xmm1
142316412: 660f7ec0                 movd eax, xmm0
142316416: 85c0                     test eax, eax
142316418: 0f99c1                   setns cl
14231641b: 89c2                     mov edx, eax
14231641d: 81e2ffffff7f             and edx, 0x7fffffff
142316423: 81c2000080ff             add edx, 0xff800000
142316429: 81fa0000007f             cmp edx, 0x7f000000
14231642f: 0f92c2                   setb dl
142316432: 20ca                     and dl, cl
142316434: ffc8                     dec eax
142316436: 3dffff7f00               cmp eax, 0x7fffff
14231643b: 0f92c0                   setb al
14231643e: 08d0                     or al, dl
142316440: 0f842afbffff             je 0x142315f70
142316446: f3440f59c0               mulss xmm8, xmm0
14231644b: e924fbffff               jmp 0x142315f74
142316450: f20f104d10               movsd xmm1, qword ptr [rbp + 0x10]
142316455: f30f104518               movss xmm0, dword ptr [rbp + 0x18]
14231645a: eb85                     jmp 0x1423163e1
14231645c: 0f28b42460010000         movaps xmm6, xmmword ptr [rsp + 0x160]
142316464: 0f28bc2470010000         movaps xmm7, xmmword ptr [rsp + 0x170]
14231646c: 440f28842480010000       movaps xmm8, xmmword ptr [rsp + 0x180]
142316475: 4881c498010000           add rsp, 0x198
14231647c: 5b                       pop rbx
14231647d: 5d                       pop rbp
14231647e: 5f                       pop rdi
14231647f: 5e                       pop rsi
142316480: 415c                     pop r12
142316482: 415d                     pop r13
142316484: 415e                     pop r14
142316486: 415f                     pop r15
142316488: c3                       ret 
