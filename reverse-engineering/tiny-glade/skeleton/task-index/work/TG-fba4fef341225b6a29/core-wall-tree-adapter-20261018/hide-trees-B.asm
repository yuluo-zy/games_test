000000014226a260: 55                             push rbp
000000014226a261: 4157                           push r15
000000014226a263: 4156                           push r14
000000014226a265: 4155                           push r13
000000014226a267: 4154                           push r12
000000014226a269: 56                             push rsi
000000014226a26a: 57                             push rdi
000000014226a26b: 53                             push rbx
000000014226a26c: 4881ece8010000                 sub rsp, 0x1e8
000000014226a273: 488dac2480000000               lea rbp, [rsp + 0x80]
000000014226a27b: 440f298d50010000               movaps xmmword ptr [rbp + 0x150], xmm9
000000014226a283: 440f298540010000               movaps xmmword ptr [rbp + 0x140], xmm8
000000014226a28b: 0f29bd30010000                 movaps xmmword ptr [rbp + 0x130], xmm7
000000014226a292: 0f29b520010000                 movaps xmmword ptr [rbp + 0x120], xmm6
000000014226a299: 48c78518010000feffffff         mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
000000014226a2a4: 4c8945e8                       mov qword ptr [rbp - 0x18], r8
000000014226a2a8: 4d8b31                         mov r14, qword ptr [r9]
000000014226a2ab: 4d8b4908                       mov r9, qword ptr [r9 + 8]
000000014226a2af: 4d8b1e                         mov r11, qword ptr [r14]
000000014226a2b2: 4531d2                         xor r10d, r10d
000000014226a2b5: 4c89d8                         mov rax, r11
000000014226a2b8: 492b4118                       sub rax, qword ptr [r9 + 0x18]
000000014226a2bc: 490f42c2                       cmovb rax, r10
000000014226a2c0: 4d2b5938                       sub r11, qword ptr [r9 + 0x38]
000000014226a2c4: 4c8b85e0010000                 mov r8, qword ptr [rbp + 0x1e0]
000000014226a2cb: 4d0f42da                       cmovb r11, r10
000000014226a2cf: 4c8d3c40                       lea r15, [rax + rax*2]
000000014226a2d3: 49c1e703                       shl r15, 3
000000014226a2d7: 4d037908                       add r15, qword ptr [r9 + 8]
000000014226a2db: 498b7110                       mov rsi, qword ptr [r9 + 0x10]
000000014226a2df: 498b7930                       mov rdi, qword ptr [r9 + 0x30]
000000014226a2e3: 4829c6                         sub rsi, rax
000000014226a2e6: 490f42f2                       cmovb rsi, r10
000000014226a2ea: bb08000000                     mov ebx, 8
000000014226a2ef: 4c0f42fb                       cmovb r15, rbx
000000014226a2f3: 4b8d045b                       lea rax, [r11 + r11*2]
000000014226a2f7: 48c1e003                       shl rax, 3
000000014226a2fb: 49034128                       add rax, qword ptr [r9 + 0x28]
000000014226a2ff: 4c29df                         sub rdi, r11
000000014226a302: 490f42fa                       cmovb rdi, r10
000000014226a306: 480f42c3                       cmovb rax, rbx
000000014226a30a: 4c8b95d0010000                 mov r10, qword ptr [rbp + 0x1d0]
000000014226a311: 4d8b4940                       mov r9, qword ptr [r9 + 0x40]
000000014226a315: 4929f9                         sub r9, rdi
000000014226a318: 4929f1                         sub r9, rsi
000000014226a31b: 4c89b5b0000000                 mov qword ptr [rbp + 0xb0], r14
000000014226a322: 4d890e                         mov qword ptr [r14], r9
000000014226a325: 4c8d0c76                       lea r9, [rsi + rsi*2]
000000014226a329: 4c89fe                         mov rsi, r15
000000014226a32c: 4f8d0ccf                       lea r9, [r15 + r9*8]
000000014226a330: 4c894d78                       mov qword ptr [rbp + 0x78], r9
000000014226a334: 4c8d0c7f                       lea r9, [rdi + rdi*2]
000000014226a338: 4e8d0cc8                       lea r9, [rax + r9*8]
000000014226a33c: 4c894de0                       mov qword ptr [rbp - 0x20], r9
000000014226a340: 488b12                         mov rdx, qword ptr [rdx]
000000014226a343: 48895530                       mov qword ptr [rbp + 0x30], rdx
000000014226a347: 498b12                         mov rdx, qword ptr [r10]
000000014226a34a: 48899580000000                 mov qword ptr [rbp + 0x80], rdx
000000014226a351: 418b501c                       mov edx, dword ptr [r8 + 0x1c]
000000014226a355: 8995c0000000                   mov dword ptr [rbp + 0xc0], edx
000000014226a35b: 498b10                         mov rdx, qword ptr [r8]
000000014226a35e: 48895548                       mov qword ptr [rbp + 0x48], rdx
000000014226a362: 498b5010                       mov rdx, qword ptr [r8 + 0x10]
000000014226a366: 48895538                       mov qword ptr [rbp + 0x38], rdx
000000014226a36a: 4c8b31                         mov r14, qword ptr [rcx]
000000014226a36d: 4c8b6908                       mov r13, qword ptr [rcx + 8]
000000014226a371: 8b4914                         mov ecx, dword ptr [rcx + 0x14]
000000014226a374: 898dec000000                   mov dword ptr [rbp + 0xec], ecx
000000014226a37a: 488b8dd8010000                 mov rcx, qword ptr [rbp + 0x1d8]
000000014226a381: 8b511c                         mov edx, dword ptr [rcx + 0x1c]
000000014226a384: 8995c4000000                   mov dword ptr [rbp + 0xc4], edx
000000014226a38a: 488b11                         mov rdx, qword ptr [rcx]
000000014226a38d: 488b4910                       mov rcx, qword ptr [rcx + 0x10]
000000014226a391: 48894d40                       mov qword ptr [rbp + 0x40], rcx
000000014226a395: f30f1035d3b56b00               movss xmm6, dword ptr [rip + 0x6bb5d3]
000000014226a39d: 488995b8000000                 mov qword ptr [rbp + 0xb8], rdx
000000014226a3a4: 488d4a20                       lea rcx, [rdx + 0x20]
000000014226a3a8: 48894d50                       mov qword ptr [rbp + 0x50], rcx
000000014226a3ac: 41bcffffffff                   mov r12d, 0xffffffff
000000014226a3b2: 4c897568                       mov qword ptr [rbp + 0x68], r14
000000014226a3b6: 4c89ade0000000                 mov qword ptr [rbp + 0xe0], r13
000000014226a3bd: eb0f                           jmp 0x14226a3ce
000000014226a3bf: 90                             nop 
000000014226a3c0: 488b85f8000000                 mov rax, qword ptr [rbp + 0xf8]
000000014226a3c7: 488bb5c8000000                 mov rsi, qword ptr [rbp + 0xc8]
000000014226a3ce: 4885f6                         test rsi, rsi
000000014226a3d1: 741d                           je 0x14226a3f0
000000014226a3d3: 483b7578                       cmp rsi, qword ptr [rbp + 0x78]
000000014226a3d7: 7417                           je 0x14226a3f0
000000014226a3d9: 488d4e18                       lea rcx, [rsi + 0x18]
000000014226a3dd: 48898dc8000000                 mov qword ptr [rbp + 0xc8], rcx
000000014226a3e4: 488985f8000000                 mov qword ptr [rbp + 0xf8], rax
000000014226a3eb: eb2f                           jmp 0x14226a41c
000000014226a3ed: 0f1f00                         nop dword ptr [rax]
000000014226a3f0: 4885c0                         test rax, rax
000000014226a3f3: 0f84c3040000                   je 0x14226a8bc
000000014226a3f9: 483b45e0                       cmp rax, qword ptr [rbp - 0x20]
000000014226a3fd: 0f84b9040000                   je 0x14226a8bc
000000014226a403: 488d4818                       lea rcx, [rax + 0x18]
000000014226a407: 48898df8000000                 mov qword ptr [rbp + 0xf8], rcx
000000014226a40e: 48c785c800000000000000         mov qword ptr [rbp + 0xc8], 0
000000014226a419: 4889c6                         mov rsi, rax
000000014226a41c: 488b85b0000000                 mov rax, qword ptr [rbp + 0xb0]
000000014226a423: 48ff00                         inc qword ptr [rax]
000000014226a426: 488d4db8                       lea rcx, [rbp - 0x48]
000000014226a42a: 488b5530                       mov rdx, qword ptr [rbp + 0x30]
000000014226a42e: e8cd0c7bfe                     call 0x140a1b100
000000014226a433: 4c8b7dd8                       mov r15, qword ptr [rbp - 0x28]
000000014226a437: 4d85ff                         test r15, r15
000000014226a43a: 7484                           je 0x14226a3c0
000000014226a43c: 48897570                       mov qword ptr [rbp + 0x70], rsi
000000014226a440: 488d4608                       lea rax, [rsi + 8]
000000014226a444: 488985a8000000                 mov qword ptr [rbp + 0xa8], rax
000000014226a44b: 0fb775d0                       movzx esi, word ptr [rbp - 0x30]
000000014226a44f: 488b5db8                       mov rbx, qword ptr [rbp - 0x48]
000000014226a453: 488b7dc0                       mov rdi, qword ptr [rbp - 0x40]
000000014226a457: eb20                           jmp 0x14226a479
000000014226a459: 4c8bade0000000                 mov r13, qword ptr [rbp + 0xe0]
000000014226a460: 4c8b7568                       mov r14, qword ptr [rbp + 0x68]
000000014226a464: 41bcffffffff                   mov r12d, 0xffffffff
000000014226a46a: 660f1f440000                   nop word ptr [rax + rax]
000000014226a470: 49ffcf                         dec r15
000000014226a473: 0f8447ffffff                   je 0x14226a3c0
000000014226a479: 6685f6                         test si, si
000000014226a47c: 7412                           je 0x14226a490
000000014226a47e: 4885db                         test rbx, rbx
000000014226a481: 0f8439ffffff                   je 0x14226a3c0
000000014226a487: 8d46ff                         lea eax, [rsi - 1]
000000014226a48a: 21f0                           and eax, esi
000000014226a48c: eb2a                           jmp 0x14226a4b8
000000014226a48e: 6690                           nop 
000000014226a490: 660f6f07                       movdqa xmm0, xmmword ptr [rdi]
000000014226a494: 660fd7f0                       pmovmskb esi, xmm0
000000014226a498: 4881c300ffffff                 add rbx, -0x100
000000014226a49f: 4883c710                       add rdi, 0x10
000000014226a4a3: 81feffff0000                   cmp esi, 0xffff
000000014226a4a9: 74e5                           je 0x14226a490
000000014226a4ab: b9feffffff                     mov ecx, 0xfffffffe
000000014226a4b0: 29f1                           sub ecx, esi
000000014226a4b2: f7d6                           not esi
000000014226a4b4: 89f0                           mov eax, esi
000000014226a4b6: 21c8                           and eax, ecx
000000014226a4b8: 89f1                           mov ecx, esi
000000014226a4ba: f30fbcc9                       tzcnt ecx, ecx
000000014226a4be: 89c6                           mov esi, eax
000000014226a4c0: c1e104                         shl ecx, 4
000000014226a4c3: 4889d8                         mov rax, rbx
000000014226a4c6: 4829c8                         sub rax, rcx
000000014226a4c9: 488b48f8                       mov rcx, qword ptr [rax - 8]
000000014226a4cd: 89ca                           mov edx, ecx
000000014226a4cf: 493b5510                       cmp rdx, qword ptr [r13 + 0x10]
000000014226a4d3: 739b                           jae 0x14226a470
000000014226a4d5: 4d8b4508                       mov r8, qword ptr [r13 + 8]
000000014226a4d9: 48c1e920                       shr rcx, 0x20
000000014226a4dd: 488d1492                       lea rdx, [rdx + rdx*4]
000000014226a4e1: 41390c90                       cmp dword ptr [r8 + rdx*4], ecx
000000014226a4e5: 7589                           jne 0x14226a470
000000014226a4e7: 498d0c90                       lea rcx, [r8 + rdx*4]
000000014226a4eb: 8b5104                         mov edx, dword ptr [rcx + 4]
000000014226a4ee: 4c39e2                         cmp rdx, r12
000000014226a4f1: 0f8479ffffff                   je 0x14226a470
000000014226a4f7: 49395638                       cmp qword ptr [r14 + 0x38], rdx
000000014226a4fb: 0f866fffffff                   jbe 0x14226a470
000000014226a501: 4d8b4628                       mov r8, qword ptr [r14 + 0x28]
000000014226a505: 4189d1                         mov r9d, edx
000000014226a508: 41c1e906                       shr r9d, 6
000000014226a50c: 4f8b04c8                       mov r8, qword ptr [r8 + r9*8]
000000014226a510: 490fa3d0                       bt r8, rdx
000000014226a514: 0f8356ffffff                   jae 0x14226a470
000000014226a51a: 8b510c                         mov edx, dword ptr [rcx + 0xc]
000000014226a51d: 4d8b85a8010000                 mov r8, qword ptr [r13 + 0x1a8]
000000014226a524: 4c8d0cd2                       lea r9, [rdx + rdx*8]
000000014226a528: 4f8b54c840                     mov r10, qword ptr [r8 + r9*8 + 0x40]
000000014226a52d: 4b8b54c818                     mov rdx, qword ptr [r8 + r9*8 + 0x18]
000000014226a532: 4f8b44c838                     mov r8, qword ptr [r8 + r9*8 + 0x38]
000000014226a537: 4d8b8e18010000                 mov r9, qword ptr [r14 + 0x118]
000000014226a53e: 4d39ca                         cmp r10, r9
000000014226a541: 7627                           jbe 0x14226a56a
000000014226a543: 4f8b0cc8                       mov r9, qword ptr [r8 + r9*8]
000000014226a547: 4d85c9                         test r9, r9
000000014226a54a: 741e                           je 0x14226a56a
000000014226a54c: 49f7d1                         not r9
000000014226a54f: 4f8d0c49                       lea r9, [r9 + r9*2]
000000014226a553: 49c1e104                       shl r9, 4
000000014226a557: 4e8b4c0a10                     mov r9, qword ptr [rdx + r9 + 0x10]
000000014226a55c: 4d8b9e20010000                 mov r11, qword ptr [r14 + 0x120]
000000014226a563: 4d39da                         cmp r10, r11
000000014226a566: 7711                           ja 0x14226a579
000000014226a568: eb3d                           jmp 0x14226a5a7
000000014226a56a: 4531c9                         xor r9d, r9d
000000014226a56d: 4d8b9e20010000                 mov r11, qword ptr [r14 + 0x120]
000000014226a574: 4d39da                         cmp r10, r11
000000014226a577: 762e                           jbe 0x14226a5a7
000000014226a579: 4f8b1cd8                       mov r11, qword ptr [r8 + r11*8]
000000014226a57d: 4d85db                         test r11, r11
000000014226a580: 7425                           je 0x14226a5a7
000000014226a582: 49f7d3                         not r11
000000014226a585: 4f8d1c5b                       lea r11, [r11 + r11*2]
000000014226a589: 49c1e304                       shl r11, 4
000000014226a58d: 4e8b5c1a10                     mov r11, qword ptr [rdx + r11 + 0x10]
000000014226a592: 4c899df0000000                 mov qword ptr [rbp + 0xf0], r11
000000014226a599: 4d8b9e28010000                 mov r11, qword ptr [r14 + 0x128]
000000014226a5a0: 4d39da                         cmp r10, r11
000000014226a5a3: 763b                           jbe 0x14226a5e0
000000014226a5a5: eb17                           jmp 0x14226a5be
000000014226a5a7: 48c785f000000000000000         mov qword ptr [rbp + 0xf0], 0
000000014226a5b2: 4d8b9e28010000                 mov r11, qword ptr [r14 + 0x128]
000000014226a5b9: 4d39da                         cmp r10, r11
000000014226a5bc: 7622                           jbe 0x14226a5e0
000000014226a5be: 4f8b14d8                       mov r10, qword ptr [r8 + r11*8]
000000014226a5c2: 4d85d2                         test r10, r10
000000014226a5c5: 7419                           je 0x14226a5e0
000000014226a5c7: 49f7d2                         not r10
000000014226a5ca: 4f8d1452                       lea r10, [r10 + r10*2]
000000014226a5ce: 49c1e204                       shl r10, 4
000000014226a5d2: 4e8b541210                     mov r10, qword ptr [rdx + r10 + 0x10]
000000014226a5d7: 4c8995d8000000                 mov qword ptr [rbp + 0xd8], r10
000000014226a5de: eb0b                           jmp 0x14226a5eb
000000014226a5e0: 48c785d800000000000000         mov qword ptr [rbp + 0xd8], 0
000000014226a5eb: 488b40f0                       mov rax, qword ptr [rax - 0x10]
000000014226a5ef: 488985d0000000                 mov qword ptr [rbp + 0xd0], rax
000000014226a5f6: 448b6910                       mov r13d, dword ptr [rcx + 0x10]
000000014226a5fa: 498b8610010000                 mov rax, qword ptr [r14 + 0x110]
000000014226a601: 498b04c0                       mov rax, qword ptr [r8 + rax*8]
000000014226a605: 48f7d0                         not rax
000000014226a608: 488d0440                       lea rax, [rax + rax*2]
000000014226a60c: 48c1e004                       shl rax, 4
000000014226a610: 488b4c0210                     mov rcx, qword ptr [rdx + rax + 0x10]
000000014226a615: 488b540228                     mov rdx, qword ptr [rdx + rax + 0x28]
000000014226a61a: 4a8d046d00000000               lea rax, [r13*2]
000000014226a622: 4c01e8                         add rax, r13
000000014226a625: 48c1e004                       shl rax, 4
000000014226a629: 48899508010000                 mov qword ptr [rbp + 0x108], rdx
000000014226a630: 448b85ec000000                 mov r8d, dword ptr [rbp + 0xec]
000000014226a637: 468904aa                       mov dword ptr [rdx + r13*4], r8d
000000014226a63b: 4d89ec                         mov r12, r13
000000014226a63e: 49c1e404                       shl r12, 4
000000014226a642: 48898d00010000                 mov qword ptr [rbp + 0x100], rcx
000000014226a649: 42c644210c00                   mov byte ptr [rcx + r12 + 0xc], 0
000000014226a64f: f3410f6f0401                   movdqu xmm0, xmmword ptr [r9 + rax]
000000014226a655: 410f104c0110                   movups xmm1, xmmword ptr [r9 + rax + 0x10]
000000014226a65b: 410f10540120                   movups xmm2, xmmword ptr [r9 + rax + 0x20]
000000014226a661: 0f295510                       movaps xmmword ptr [rbp + 0x10], xmm2
000000014226a665: 0f294d00                       movaps xmmword ptr [rbp], xmm1
000000014226a669: 660f7f45f0                     movdqa xmmword ptr [rbp - 0x10], xmm0
000000014226a66e: 488d4d24                       lea rcx, [rbp + 0x24]
000000014226a672: 488d55f0                       lea rdx, [rbp - 0x10]
000000014226a676: e825ab86fe                     call 0x140ad51a0
000000014226a67b: c6851701000001                 mov byte ptr [rbp + 0x117], 1
000000014226a682: f30f104d24                     movss xmm1, dword ptr [rbp + 0x24]
000000014226a687: f30f10552c                     movss xmm2, dword ptr [rbp + 0x2c]
000000014226a68c: 4c8d75a8                       lea r14, [rbp - 0x58]
000000014226a690: 4c89f1                         mov rcx, r14
000000014226a693: 0f28de                         movaps xmm3, xmm6
000000014226a696: e8a5cea2fe                     call 0x140c97540
000000014226a69b: 488b85a8000000                 mov rax, qword ptr [rbp + 0xa8]
000000014226a6a2: 48898590000000                 mov qword ptr [rbp + 0x90], rax
000000014226a6a9: 488d4524                       lea rax, [rbp + 0x24]
000000014226a6ad: 48898598000000                 mov qword ptr [rbp + 0x98], rax
000000014226a6b4: 488d8517010000                 lea rax, [rbp + 0x117]
000000014226a6bb: 488985a0000000                 mov qword ptr [rbp + 0xa0], rax
000000014226a6c2: 488b8d80000000                 mov rcx, qword ptr [rbp + 0x80]
000000014226a6c9: 4c89f2                         mov rdx, r14
000000014226a6cc: 4c8d8590000000                 lea r8, [rbp + 0x90]
000000014226a6d3: e8484c3700                     call 0x1425df320
000000014226a6d8: 80bd1701000000                 cmp byte ptr [rbp + 0x117], 0
000000014226a6df: 0f8574fdffff                   jne 0x14226a459
000000014226a6e5: 488b8500010000                 mov rax, qword ptr [rbp + 0x100]
000000014226a6ec: 4c01e0                         add rax, r12
000000014226a6ef: 4883c00c                       add rax, 0xc
000000014226a6f3: 488b8d08010000                 mov rcx, qword ptr [rbp + 0x108]
000000014226a6fa: 8b95ec000000                   mov edx, dword ptr [rbp + 0xec]
000000014226a700: 428914a9                       mov dword ptr [rcx + r13*4], edx
000000014226a704: c60001                         mov byte ptr [rax], 1
000000014226a707: 488b4570                       mov rax, qword ptr [rbp + 0x70]
000000014226a70b: 80781000                       cmp byte ptr [rax + 0x10], 0
000000014226a70f: 0f8444fdffff                   je 0x14226a459
000000014226a715: 4c89a500010000                 mov qword ptr [rbp + 0x100], r12
000000014226a71c: 8b85c4000000                   mov eax, dword ptr [rbp + 0xc4]
000000014226a722: 488b4d40                       mov rcx, qword ptr [rbp + 0x40]
000000014226a726: 8901                           mov dword ptr [rcx], eax
000000014226a728: 488b85b8000000                 mov rax, qword ptr [rbp + 0xb8]
000000014226a72f: 4c8b7030                       mov r14, qword ptr [rax + 0x30]
000000014226a733: 4c8b6040                       mov r12, qword ptr [rax + 0x40]
000000014226a737: 4c3b7020                       cmp r14, qword ptr [rax + 0x20]
000000014226a73b: 7510                           jne 0x14226a74d
000000014226a73d: 488b4d50                       mov rcx, qword ptr [rbp + 0x50]
000000014226a741: 488d15c8a5dc00                 lea rdx, [rip + 0xdca5c8]
000000014226a748: e8e37e2ffe                     call 0x140562630
000000014226a74d: 488b95b8000000                 mov rdx, qword ptr [rbp + 0xb8]
000000014226a754: 488b4228                       mov rax, qword ptr [rdx + 0x28]
000000014226a758: 4c89f1                         mov rcx, r14
000000014226a75b: 48c1e104                       shl rcx, 4
000000014226a75f: 4c892408                       mov qword ptr [rax + rcx], r12
000000014226a763: 4c8b85d0000000                 mov r8, qword ptr [rbp + 0xd0]
000000014226a76a: 4c89440808                     mov qword ptr [rax + rcx + 8], r8
000000014226a76f: 49ffc6                         inc r14
000000014226a772: 4c897230                       mov qword ptr [rdx + 0x30], r14
000000014226a776: 48ff4240                       inc qword ptr [rdx + 0x40]
000000014226a77a: 8b85c0000000                   mov eax, dword ptr [rbp + 0xc0]
000000014226a780: 488b4d38                       mov rcx, qword ptr [rbp + 0x38]
000000014226a784: 8901                           mov dword ptr [rcx], eax
000000014226a786: 0f287df0                       movaps xmm7, xmmword ptr [rbp - 0x10]
000000014226a78a: f3440f104500                   movss xmm8, dword ptr [rbp]
000000014226a790: 448b6504                       mov r12d, dword ptr [rbp + 4]
000000014226a794: 496bc558                       imul rax, r13, 0x58
000000014226a798: 488b8df0000000                 mov rcx, qword ptr [rbp + 0xf0]
000000014226a79f: 4801c1                         add rcx, rax
000000014226a7a2: 4883c148                       add rcx, 0x48
000000014226a7a6: e885b92eff                     call 0x141556130
000000014226a7ab: 4885d2                         test rdx, rdx
000000014226a7ae: 0f885f070000                   js 0x14226af13
000000014226a7b4: 4989c6                         mov r14, rax
000000014226a7b7: 4989d0                         mov r8, rdx
000000014226a7ba: 4489a508010000                 mov dword ptr [rbp + 0x108], r12d
000000014226a7c1: 4c8bade0000000                 mov r13, qword ptr [rbp + 0xe0]
000000014226a7c8: 7425                           je 0x14226a7ef
000000014226a7ca: 0fb60538a43c01                 movzx eax, byte ptr [rip + 0x13ca438]
000000014226a7d1: ba01000000                     mov edx, 1
000000014226a7d6: 4d89c4                         mov r12, r8
000000014226a7d9: 4c89c1                         mov rcx, r8
000000014226a7dc: e82f943afe                     call 0x140613c10
000000014226a7e1: 4885c0                         test rax, rax
000000014226a7e4: 0f846b070000                   je 0x14226af55
000000014226a7ea: 4d89e0                         mov r8, r12
000000014226a7ed: eb05                           jmp 0x14226a7f4
000000014226a7ef: b801000000                     mov eax, 1
000000014226a7f4: 4989c4                         mov r12, rax
000000014226a7f7: 4889c1                         mov rcx, rax
000000014226a7fa: 4c89f2                         mov rdx, r14
000000014226a7fd: 4d89c6                         mov r14, r8
000000014226a800: e8fb936b00                     call 0x142923c00
000000014226a805: 488b85d8000000                 mov rax, qword ptr [rbp + 0xd8]
000000014226a80c: 488b8d00010000                 mov rcx, qword ptr [rbp + 0x100]
000000014226a813: 0fb6440808                     movzx eax, byte ptr [rax + rcx + 8]
000000014226a818: 888500010000                   mov byte ptr [rbp + 0x100], al
000000014226a81e: 0fb605e4a33c01                 movzx eax, byte ptr [rip + 0x13ca3e4]
000000014226a825: b958000000                     mov ecx, 0x58
000000014226a82a: ba08000000                     mov edx, 8
000000014226a82f: e8dc933afe                     call 0x140613c10
000000014226a834: 4885c0                         test rax, rax
000000014226a837: 4c89a588000000                 mov qword ptr [rbp + 0x88], r12
000000014226a83e: 0f84b7060000                   je 0x14226aefb
000000014226a844: 48c70001000000                 mov qword ptr [rax], 1
000000014226a84b: 48c7400801000000               mov qword ptr [rax + 8], 1
000000014226a853: c7401002000000                 mov dword ptr [rax + 0x10], 2
000000014226a85a: 488b8dd0000000                 mov rcx, qword ptr [rbp + 0xd0]
000000014226a861: 48894818                       mov qword ptr [rax + 0x18], rcx
000000014226a865: 4c897020                       mov qword ptr [rax + 0x20], r14
000000014226a869: 4c896028                       mov qword ptr [rax + 0x28], r12
000000014226a86d: 4c897030                       mov qword ptr [rax + 0x30], r14
000000014226a871: 0f117838                       movups xmmword ptr [rax + 0x38], xmm7
000000014226a875: f3440f114048                   movss dword ptr [rax + 0x48], xmm8
000000014226a87b: 8b8d08010000                   mov ecx, dword ptr [rbp + 0x108]
000000014226a881: 89484c                         mov dword ptr [rax + 0x4c], ecx
000000014226a884: 0fb68d00010000                 movzx ecx, byte ptr [rbp + 0x100]
000000014226a88b: 884850                         mov byte ptr [rax + 0x50], cl
000000014226a88e: 48898598000000                 mov qword ptr [rbp + 0x98], rax
000000014226a895: 48c785900000000c000000         mov qword ptr [rbp + 0x90], 0xc
000000014226a8a0: 488b4d48                       mov rcx, qword ptr [rbp + 0x48]
000000014226a8a4: 488d9590000000                 lea rdx, [rbp + 0x90]
000000014226a8ab: 4c8d058eaadc00                 lea r8, [rip + 0xdcaa8e]
000000014226a8b2: e809a681fe                     call 0x140a84ec0
000000014226a8b7: e9a4fbffff                     jmp 0x14226a460
000000014226a8bc: 488b45e8                       mov rax, qword ptr [rbp - 0x18]
000000014226a8c0: 488b30                         mov rsi, qword ptr [rax]
000000014226a8c3: 488b4808                       mov rcx, qword ptr [rax + 8]
000000014226a8c7: 4c8b06                         mov r8, qword ptr [rsi]
000000014226a8ca: 31d2                           xor edx, edx
000000014226a8cc: 4c89c0                         mov rax, r8
000000014226a8cf: 482b4118                       sub rax, qword ptr [rcx + 0x18]
000000014226a8d3: 480f42c2                       cmovb rax, rdx
000000014226a8d7: 4c2b4138                       sub r8, qword ptr [rcx + 0x38]
000000014226a8db: 4c0f42c2                       cmovb r8, rdx
000000014226a8df: 488d3c40                       lea rdi, [rax + rax*2]
000000014226a8e3: 48c1e704                       shl rdi, 4
000000014226a8e7: 48037908                       add rdi, qword ptr [rcx + 8]
000000014226a8eb: 4c8b5110                       mov r10, qword ptr [rcx + 0x10]
000000014226a8ef: 4c8b4930                       mov r9, qword ptr [rcx + 0x30]
000000014226a8f3: 4929c2                         sub r10, rax
000000014226a8f6: 4c0f42d2                       cmovb r10, rdx
000000014226a8fa: 41bb08000000                   mov r11d, 8
000000014226a900: 490f42fb                       cmovb rdi, r11
000000014226a904: 4b8d0440                       lea rax, [r8 + r8*2]
000000014226a908: 48c1e004                       shl rax, 4
000000014226a90c: 48034128                       add rax, qword ptr [rcx + 0x28]
000000014226a910: 4d29c1                         sub r9, r8
000000014226a913: 4c0f42ca                       cmovb r9, rdx
000000014226a917: 490f42c3                       cmovb rax, r11
000000014226a91b: 4989f8                         mov r8, rdi
000000014226a91e: 488b4940                       mov rcx, qword ptr [rcx + 0x40]
000000014226a922: 4c29c9                         sub rcx, r9
000000014226a925: 4c29d1                         sub rcx, r10
000000014226a928: 48897570                       mov qword ptr [rbp + 0x70], rsi
000000014226a92c: 48890e                         mov qword ptr [rsi], rcx
000000014226a92f: 4b8d0c52                       lea rcx, [r10 + r10*2]
000000014226a933: 48c1e104                       shl rcx, 4
000000014226a937: 4801f9                         add rcx, rdi
000000014226a93a: 48894d78                       mov qword ptr [rbp + 0x78], rcx
000000014226a93e: 4b8d0c49                       lea rcx, [r9 + r9*2]
000000014226a942: 48c1e104                       shl rcx, 4
000000014226a946: 4801c1                         add rcx, rax
000000014226a949: 48898d80000000                 mov qword ptr [rbp + 0x80], rcx
000000014226a950: 41bcffffffff                   mov r12d, 0xffffffff
000000014226a956: 0f57f6                         xorps xmm6, xmm6
000000014226a959: f30f103d339d6c00               movss xmm7, dword ptr [rip + 0x6c9d33]
000000014226a961: eb1b                           jmp 0x14226a97e
000000014226a963: 666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
000000014226a970: 488b85c8000000                 mov rax, qword ptr [rbp + 0xc8]
000000014226a977: 4c8b85a8000000                 mov r8, qword ptr [rbp + 0xa8]
000000014226a97e: 4d85c0                         test r8, r8
000000014226a981: 742d                           je 0x14226a9b0
000000014226a983: 4c3b4578                       cmp r8, qword ptr [rbp + 0x78]
000000014226a987: 7427                           je 0x14226a9b0
000000014226a989: 4c8985f8000000                 mov qword ptr [rbp + 0xf8], r8
000000014226a990: 498d4830                       lea rcx, [r8 + 0x30]
000000014226a994: 48898da8000000                 mov qword ptr [rbp + 0xa8], rcx
000000014226a99b: 488985c8000000                 mov qword ptr [rbp + 0xc8], rax
000000014226a9a2: eb3f                           jmp 0x14226a9e3
000000014226a9a4: 6666662e0f1f840000000000       nop word ptr cs:[rax + rax]
000000014226a9b0: 4885c0                         test rax, rax
000000014226a9b3: 0f8410050000                   je 0x14226aec9
000000014226a9b9: 483b8580000000                 cmp rax, qword ptr [rbp + 0x80]
000000014226a9c0: 0f8403050000                   je 0x14226aec9
000000014226a9c6: 488d4830                       lea rcx, [rax + 0x30]
000000014226a9ca: 48898dc8000000                 mov qword ptr [rbp + 0xc8], rcx
000000014226a9d1: 48c785a800000000000000         mov qword ptr [rbp + 0xa8], 0
000000014226a9dc: 488985f8000000                 mov qword ptr [rbp + 0xf8], rax
000000014226a9e3: 488b4570                       mov rax, qword ptr [rbp + 0x70]
000000014226a9e7: 48ff00                         inc qword ptr [rax]
000000014226a9ea: 488d4db8                       lea rcx, [rbp - 0x48]
000000014226a9ee: 488b5530                       mov rdx, qword ptr [rbp + 0x30]
000000014226a9f2: e809077bfe                     call 0x140a1b100
000000014226a9f7: 488b7dd8                       mov rdi, qword ptr [rbp - 0x28]
000000014226a9fb: 4885ff                         test rdi, rdi
000000014226a9fe: 0f846cffffff                   je 0x14226a970
000000014226aa04: 488b85f8000000                 mov rax, qword ptr [rbp + 0xf8]
000000014226aa0b: 4883c004                       add rax, 4
000000014226aa0f: 488985b0000000                 mov qword ptr [rbp + 0xb0], rax
000000014226aa16: 0fb75dd0                       movzx ebx, word ptr [rbp - 0x30]
000000014226aa1a: 488b75b8                       mov rsi, qword ptr [rbp - 0x48]
000000014226aa1e: 4c8b7dc0                       mov r15, qword ptr [rbp - 0x40]
000000014226aa22: e9d2000000                     jmp 0x14226aaf9
000000014226aa27: b801000000                     mov eax, 1
000000014226aa2c: 4889c1                         mov rcx, rax
000000014226aa2f: 488985e0000000                 mov qword ptr [rbp + 0xe0], rax
000000014226aa36: 4c89c2                         mov rdx, r8
000000014226aa39: 4d89e0                         mov r8, r12
000000014226aa3c: e8bf916b00                     call 0x142923c00
000000014226aa41: 488b85d8000000                 mov rax, qword ptr [rbp + 0xd8]
000000014226aa48: 460fb6743008                   movzx r14d, byte ptr [rax + r14 + 8]
000000014226aa4e: 0fb605b4a13c01                 movzx eax, byte ptr [rip + 0x13ca1b4]
000000014226aa55: b958000000                     mov ecx, 0x58
000000014226aa5a: ba08000000                     mov edx, 8
000000014226aa5f: e8ac913afe                     call 0x140613c10
000000014226aa64: 4885c0                         test rax, rax
000000014226aa67: 0f84bb040000                   je 0x14226af28
000000014226aa6d: 48c70001000000                 mov qword ptr [rax], 1
000000014226aa74: 48c7400801000000               mov qword ptr [rax + 8], 1
000000014226aa7c: c7401002000000                 mov dword ptr [rax + 0x10], 2
000000014226aa83: 488b8dd0000000                 mov rcx, qword ptr [rbp + 0xd0]
000000014226aa8a: 48894818                       mov qword ptr [rax + 0x18], rcx
000000014226aa8e: 4c896020                       mov qword ptr [rax + 0x20], r12
000000014226aa92: 488b8de0000000                 mov rcx, qword ptr [rbp + 0xe0]
000000014226aa99: 48894828                       mov qword ptr [rax + 0x28], rcx
000000014226aa9d: 4c896030                       mov qword ptr [rax + 0x30], r12
000000014226aaa1: 440f114038                     movups xmmword ptr [rax + 0x38], xmm8
000000014226aaa6: f3440f114848                   movss dword ptr [rax + 0x48], xmm9
000000014226aaac: 8b8d00010000                   mov ecx, dword ptr [rbp + 0x100]
000000014226aab2: 89484c                         mov dword ptr [rax + 0x4c], ecx
000000014226aab5: 44887050                       mov byte ptr [rax + 0x50], r14b
000000014226aab9: 48898598000000                 mov qword ptr [rbp + 0x98], rax
000000014226aac0: 48c785900000000c000000         mov qword ptr [rbp + 0x90], 0xc
000000014226aacb: 488b4d48                       mov rcx, qword ptr [rbp + 0x48]
000000014226aacf: 488d9590000000                 lea rdx, [rbp + 0x90]
000000014226aad6: 4c8d054ba8dc00                 lea r8, [rip + 0xdca84b]
000000014226aadd: e8dea381fe                     call 0x140a84ec0
000000014226aae2: 4c8b7568                       mov r14, qword ptr [rbp + 0x68]
000000014226aae6: 41bcffffffff                   mov r12d, 0xffffffff
000000014226aaec: 0f1f4000                       nop dword ptr [rax]
000000014226aaf0: 48ffcf                         dec rdi
000000014226aaf3: 0f8477feffff                   je 0x14226a970
000000014226aaf9: 6685db                         test bx, bx
000000014226aafc: 7412                           je 0x14226ab10
000000014226aafe: 4885f6                         test rsi, rsi
000000014226ab01: 0f8469feffff                   je 0x14226a970
000000014226ab07: 8d43ff                         lea eax, [rbx - 1]
000000014226ab0a: 21d8                           and eax, ebx
000000014226ab0c: eb2b                           jmp 0x14226ab39
000000014226ab0e: 6690                           nop 
000000014226ab10: 66410f6f07                     movdqa xmm0, xmmword ptr [r15]
000000014226ab15: 660fd7d8                       pmovmskb ebx, xmm0
000000014226ab19: 4881c600ffffff                 add rsi, -0x100
000000014226ab20: 4983c710                       add r15, 0x10
000000014226ab24: 81fbffff0000                   cmp ebx, 0xffff
000000014226ab2a: 74e4                           je 0x14226ab10
000000014226ab2c: b9feffffff                     mov ecx, 0xfffffffe
000000014226ab31: 29d9                           sub ecx, ebx
000000014226ab33: f7d3                           not ebx
000000014226ab35: 89d8                           mov eax, ebx
000000014226ab37: 21c8                           and eax, ecx
000000014226ab39: 89d9                           mov ecx, ebx
000000014226ab3b: f30fbcc9                       tzcnt ecx, ecx
000000014226ab3f: 89c3                           mov ebx, eax
000000014226ab41: c1e104                         shl ecx, 4
000000014226ab44: 4889f0                         mov rax, rsi
000000014226ab47: 4829c8                         sub rax, rcx
000000014226ab4a: 488b48f8                       mov rcx, qword ptr [rax - 8]
000000014226ab4e: 89ca                           mov edx, ecx
000000014226ab50: 493b5510                       cmp rdx, qword ptr [r13 + 0x10]
000000014226ab54: 739a                           jae 0x14226aaf0
000000014226ab56: 4d8b4508                       mov r8, qword ptr [r13 + 8]
000000014226ab5a: 48c1e920                       shr rcx, 0x20
000000014226ab5e: 488d1492                       lea rdx, [rdx + rdx*4]
000000014226ab62: 41390c90                       cmp dword ptr [r8 + rdx*4], ecx
000000014226ab66: 7588                           jne 0x14226aaf0
000000014226ab68: 498d0c90                       lea rcx, [r8 + rdx*4]
000000014226ab6c: 8b5104                         mov edx, dword ptr [rcx + 4]
000000014226ab6f: 4c39e2                         cmp rdx, r12
000000014226ab72: 0f8478ffffff                   je 0x14226aaf0
000000014226ab78: 49395638                       cmp qword ptr [r14 + 0x38], rdx
000000014226ab7c: 0f866effffff                   jbe 0x14226aaf0
000000014226ab82: 4d8b4628                       mov r8, qword ptr [r14 + 0x28]
000000014226ab86: 4189d1                         mov r9d, edx
000000014226ab89: 41c1e906                       shr r9d, 6
000000014226ab8d: 4f8b04c8                       mov r8, qword ptr [r8 + r9*8]
000000014226ab91: 490fa3d0                       bt r8, rdx
000000014226ab95: 0f8355ffffff                   jae 0x14226aaf0
000000014226ab9b: 8b510c                         mov edx, dword ptr [rcx + 0xc]
000000014226ab9e: 4d8b85a8010000                 mov r8, qword ptr [r13 + 0x1a8]
000000014226aba5: 4c8d0cd2                       lea r9, [rdx + rdx*8]
000000014226aba9: 4f8b54c840                     mov r10, qword ptr [r8 + r9*8 + 0x40]
000000014226abae: 4b8b54c818                     mov rdx, qword ptr [r8 + r9*8 + 0x18]
000000014226abb3: 4f8b44c838                     mov r8, qword ptr [r8 + r9*8 + 0x38]
000000014226abb8: 4d8b8e18010000                 mov r9, qword ptr [r14 + 0x118]
000000014226abbf: 4d39ca                         cmp r10, r9
000000014226abc2: 7627                           jbe 0x14226abeb
000000014226abc4: 4f8b0cc8                       mov r9, qword ptr [r8 + r9*8]
000000014226abc8: 4d85c9                         test r9, r9
000000014226abcb: 741e                           je 0x14226abeb
000000014226abcd: 49f7d1                         not r9
000000014226abd0: 4f8d0c49                       lea r9, [r9 + r9*2]
000000014226abd4: 49c1e104                       shl r9, 4
000000014226abd8: 4e8b4c0a10                     mov r9, qword ptr [rdx + r9 + 0x10]
000000014226abdd: 4d8b9e20010000                 mov r11, qword ptr [r14 + 0x120]
000000014226abe4: 4d39da                         cmp r10, r11
000000014226abe7: 7711                           ja 0x14226abfa
000000014226abe9: eb3d                           jmp 0x14226ac28
000000014226abeb: 4531c9                         xor r9d, r9d
000000014226abee: 4d8b9e20010000                 mov r11, qword ptr [r14 + 0x120]
000000014226abf5: 4d39da                         cmp r10, r11
000000014226abf8: 762e                           jbe 0x14226ac28
000000014226abfa: 4f8b1cd8                       mov r11, qword ptr [r8 + r11*8]
000000014226abfe: 4d85db                         test r11, r11
000000014226ac01: 7425                           je 0x14226ac28
000000014226ac03: 49f7d3                         not r11
000000014226ac06: 4f8d1c5b                       lea r11, [r11 + r11*2]
000000014226ac0a: 49c1e304                       shl r11, 4
000000014226ac0e: 4e8b5c1a10                     mov r11, qword ptr [rdx + r11 + 0x10]
000000014226ac13: 4c899df0000000                 mov qword ptr [rbp + 0xf0], r11
000000014226ac1a: 4d8b9e28010000                 mov r11, qword ptr [r14 + 0x128]
000000014226ac21: 4d39da                         cmp r10, r11
000000014226ac24: 763b                           jbe 0x14226ac61
000000014226ac26: eb17                           jmp 0x14226ac3f
000000014226ac28: 48c785f000000000000000         mov qword ptr [rbp + 0xf0], 0
000000014226ac33: 4d8b9e28010000                 mov r11, qword ptr [r14 + 0x128]
000000014226ac3a: 4d39da                         cmp r10, r11
000000014226ac3d: 7622                           jbe 0x14226ac61
000000014226ac3f: 4f8b14d8                       mov r10, qword ptr [r8 + r11*8]
000000014226ac43: 4d85d2                         test r10, r10
000000014226ac46: 7419                           je 0x14226ac61
000000014226ac48: 49f7d2                         not r10
000000014226ac4b: 4f8d1452                       lea r10, [r10 + r10*2]
000000014226ac4f: 49c1e204                       shl r10, 4
000000014226ac53: 4e8b541210                     mov r10, qword ptr [rdx + r10 + 0x10]
000000014226ac58: 4c8995d8000000                 mov qword ptr [rbp + 0xd8], r10
000000014226ac5f: eb0b                           jmp 0x14226ac6c
000000014226ac61: 48c785d800000000000000         mov qword ptr [rbp + 0xd8], 0
000000014226ac6c: 488b40f0                       mov rax, qword ptr [rax - 0x10]
000000014226ac70: 488985d0000000                 mov qword ptr [rbp + 0xd0], rax
000000014226ac77: 448b6110                       mov r12d, dword ptr [rcx + 0x10]
000000014226ac7b: 498b8610010000                 mov rax, qword ptr [r14 + 0x110]
000000014226ac82: 498b04c0                       mov rax, qword ptr [r8 + rax*8]
000000014226ac86: 48f7d0                         not rax
000000014226ac89: 488d0440                       lea rax, [rax + rax*2]
000000014226ac8d: 48c1e004                       shl rax, 4
000000014226ac91: 4c8b440210                     mov r8, qword ptr [rdx + rax + 0x10]
000000014226ac96: 488b540228                     mov rdx, qword ptr [rdx + rax + 0x28]
000000014226ac9b: 4b8d0464                       lea rax, [r12 + r12*2]
000000014226ac9f: 48c1e004                       shl rax, 4
000000014226aca3: 8b8dec000000                   mov ecx, dword ptr [rbp + 0xec]
000000014226aca9: 48899508010000                 mov qword ptr [rbp + 0x108], rdx
000000014226acb0: 42890ca2                       mov dword ptr [rdx + r12*4], ecx
000000014226acb4: 4d89e6                         mov r14, r12
000000014226acb7: 49c1e604                       shl r14, 4
000000014226acbb: 4c898500010000                 mov qword ptr [rbp + 0x100], r8
000000014226acc2: 43c644300c00                   mov byte ptr [r8 + r14 + 0xc], 0
000000014226acc8: 410f100401                     movups xmm0, xmmword ptr [r9 + rax]
000000014226accd: 410f104c0110                   movups xmm1, xmmword ptr [r9 + rax + 0x10]
000000014226acd3: 410f10540120                   movups xmm2, xmmword ptr [r9 + rax + 0x20]
000000014226acd9: 0f295510                       movaps xmmword ptr [rbp + 0x10], xmm2
000000014226acdd: 0f294d00                       movaps xmmword ptr [rbp], xmm1
000000014226ace1: 0f2945f0                       movaps xmmword ptr [rbp - 0x10], xmm0
000000014226ace5: 488d4d5c                       lea rcx, [rbp + 0x5c]
000000014226ace9: 488d55f0                       lea rdx, [rbp - 0x10]
000000014226aced: e8aea486fe                     call 0x140ad51a0
000000014226acf2: 488b85f8000000                 mov rax, qword ptr [rbp + 0xf8]
000000014226acf9: f30f10401c                     movss xmm0, dword ptr [rax + 0x1c]
000000014226acfe: 0f2ec6                         ucomiss xmm0, xmm6
000000014226ad01: 760e                           jbe 0x14226ad11
000000014226ad03: f30f5c4560                     subss xmm0, dword ptr [rbp + 0x60]
000000014226ad08: 0f2ec7                         ucomiss xmm0, xmm7
000000014226ad0b: 0f87d1fdffff                   ja 0x14226aae2
000000014226ad11: f60001                         test byte ptr [rax], 1
000000014226ad14: 488985f8000000                 mov qword ptr [rbp + 0xf8], rax
000000014226ad1b: 743b                           je 0x14226ad58
000000014226ad1d: f30f105020                     movss xmm2, dword ptr [rax + 0x20]
000000014226ad22: 488d8d90000000                 lea rcx, [rbp + 0x90]
000000014226ad29: 488b95b0000000                 mov rdx, qword ptr [rbp + 0xb0]
000000014226ad30: 0f28da                         movaps xmm3, xmm2
000000014226ad33: e8f8f9a2fe                     call 0x140c9a730
000000014226ad38: f30f104d5c                     movss xmm1, dword ptr [rbp + 0x5c]
000000014226ad3d: f30f105564                     movss xmm2, dword ptr [rbp + 0x64]
000000014226ad42: 488d8d90000000                 lea rcx, [rbp + 0x90]
000000014226ad49: e8e2f8a2fe                     call 0x140c9a630
000000014226ad4e: 84c0                           test al, al
000000014226ad50: 0f848cfdffff                   je 0x14226aae2
000000014226ad56: eb3a                           jmp 0x14226ad92
000000014226ad58: f30f105020                     movss xmm2, dword ptr [rax + 0x20]
000000014226ad5d: f30f58d2                       addss xmm2, xmm2
000000014226ad61: 488d8d90000000                 lea rcx, [rbp + 0x90]
000000014226ad68: 488b95b0000000                 mov rdx, qword ptr [rbp + 0xb0]
000000014226ad6f: e84cd3a2fe                     call 0x140c980c0
000000014226ad74: f30f104d5c                     movss xmm1, dword ptr [rbp + 0x5c]
000000014226ad79: f30f105564                     movss xmm2, dword ptr [rbp + 0x64]
000000014226ad7e: 488d8d90000000                 lea rcx, [rbp + 0x90]
000000014226ad85: e866d3a2fe                     call 0x140c980f0
000000014226ad8a: 84c0                           test al, al
000000014226ad8c: 0f8450fdffff                   je 0x14226aae2
000000014226ad92: 488b8500010000                 mov rax, qword ptr [rbp + 0x100]
000000014226ad99: 4c01f0                         add rax, r14
000000014226ad9c: 4883c00c                       add rax, 0xc
000000014226ada0: 8b8dec000000                   mov ecx, dword ptr [rbp + 0xec]
000000014226ada6: 488b9508010000                 mov rdx, qword ptr [rbp + 0x108]
000000014226adad: 42890ca2                       mov dword ptr [rdx + r12*4], ecx
000000014226adb1: c60001                         mov byte ptr [rax], 1
000000014226adb4: 488b85f8000000                 mov rax, qword ptr [rbp + 0xf8]
000000014226adbb: 80782401                       cmp byte ptr [rax + 0x24], 1
000000014226adbf: 0f851dfdffff                   jne 0x14226aae2
000000014226adc5: 8b85c4000000                   mov eax, dword ptr [rbp + 0xc4]
000000014226adcb: 488b4d40                       mov rcx, qword ptr [rbp + 0x40]
000000014226adcf: 8901                           mov dword ptr [rcx], eax
000000014226add1: 488b85b8000000                 mov rax, qword ptr [rbp + 0xb8]
000000014226add8: 4c8b4830                       mov r9, qword ptr [rax + 0x30]
000000014226addc: 4c8b4040                       mov r8, qword ptr [rax + 0x40]
000000014226ade0: 4c3b4820                       cmp r9, qword ptr [rax + 0x20]
000000014226ade4: 752c                           jne 0x14226ae12
000000014226ade6: 488b4d50                       mov rcx, qword ptr [rbp + 0x50]
000000014226adea: 488d151f9fdc00                 lea rdx, [rip + 0xdc9f1f]
000000014226adf1: 4c898d00010000                 mov qword ptr [rbp + 0x100], r9
000000014226adf8: 4c898508010000                 mov qword ptr [rbp + 0x108], r8
000000014226adff: e82c782ffe                     call 0x140562630
000000014226ae04: 4c8b8508010000                 mov r8, qword ptr [rbp + 0x108]
000000014226ae0b: 4c8b8d00010000                 mov r9, qword ptr [rbp + 0x100]
000000014226ae12: 488b95b8000000                 mov rdx, qword ptr [rbp + 0xb8]
000000014226ae19: 488b4228                       mov rax, qword ptr [rdx + 0x28]
000000014226ae1d: 4c89c9                         mov rcx, r9
000000014226ae20: 48c1e104                       shl rcx, 4
000000014226ae24: 4c890408                       mov qword ptr [rax + rcx], r8
000000014226ae28: 4c8b85d0000000                 mov r8, qword ptr [rbp + 0xd0]
000000014226ae2f: 4c89440808                     mov qword ptr [rax + rcx + 8], r8
000000014226ae34: 49ffc1                         inc r9
000000014226ae37: 4c894a30                       mov qword ptr [rdx + 0x30], r9
000000014226ae3b: 48ff4240                       inc qword ptr [rdx + 0x40]
000000014226ae3f: 8b85c0000000                   mov eax, dword ptr [rbp + 0xc0]
000000014226ae45: 488b4d38                       mov rcx, qword ptr [rbp + 0x38]
000000014226ae49: 8901                           mov dword ptr [rcx], eax
000000014226ae4b: 440f2845f0                     movaps xmm8, xmmword ptr [rbp - 0x10]
000000014226ae50: f3440f104d00                   movss xmm9, dword ptr [rbp]
000000014226ae56: 8b4504                         mov eax, dword ptr [rbp + 4]
000000014226ae59: 898500010000                   mov dword ptr [rbp + 0x100], eax
000000014226ae5f: 496bc458                       imul rax, r12, 0x58
000000014226ae63: 488b8df0000000                 mov rcx, qword ptr [rbp + 0xf0]
000000014226ae6a: 4801c1                         add rcx, rax
000000014226ae6d: 4883c148                       add rcx, 0x48
000000014226ae71: e8bab22eff                     call 0x141556130
000000014226ae76: 4885d2                         test rdx, rdx
000000014226ae79: 0f88c1000000                   js 0x14226af40
000000014226ae7f: 4989c0                         mov r8, rax
000000014226ae82: 4989d4                         mov r12, rdx
000000014226ae85: 0f849cfbffff                   je 0x14226aa27
000000014226ae8b: 4c8985f0000000                 mov qword ptr [rbp + 0xf0], r8
000000014226ae92: 0fb605709d3c01                 movzx eax, byte ptr [rip + 0x13c9d70]
000000014226ae99: ba01000000                     mov edx, 1
000000014226ae9e: 4c89a508010000                 mov qword ptr [rbp + 0x108], r12
000000014226aea5: 4c89e1                         mov rcx, r12
000000014226aea8: e8638d3afe                     call 0x140613c10
000000014226aead: 4885c0                         test rax, rax
000000014226aeb0: 0f84be000000                   je 0x14226af74
000000014226aeb6: 4c8ba508010000                 mov r12, qword ptr [rbp + 0x108]
000000014226aebd: 4c8b85f0000000                 mov r8, qword ptr [rbp + 0xf0]
000000014226aec4: e963fbffff                     jmp 0x14226aa2c
000000014226aec9: 0f28b520010000                 movaps xmm6, xmmword ptr [rbp + 0x120]
000000014226aed0: 0f28bd30010000                 movaps xmm7, xmmword ptr [rbp + 0x130]
000000014226aed7: 440f288540010000               movaps xmm8, xmmword ptr [rbp + 0x140]
000000014226aedf: 440f288d50010000               movaps xmm9, xmmword ptr [rbp + 0x150]
000000014226aee7: 4881c4e8010000                 add rsp, 0x1e8
000000014226aeee: 5b                             pop rbx
000000014226aeef: 5f                             pop rdi
000000014226aef0: 5e                             pop rsi
000000014226aef1: 415c                           pop r12
000000014226aef3: 415d                           pop r13
000000014226aef5: 415e                           pop r14
000000014226aef7: 415f                           pop r15
000000014226aef9: 5d                             pop rbp
000000014226aefa: c3                             ret 
000000014226aefb: 4c89b5f8000000                 mov qword ptr [rbp + 0xf8], r14
000000014226af02: b908000000                     mov ecx, 8
000000014226af07: ba58000000                     mov edx, 0x58
000000014226af0c: e8efe06600                     call 0x1428d9000
000000014226af11: eb2b                           jmp 0x14226af3e
000000014226af13: 31c9                           xor ecx, ecx
000000014226af15: 4c8d05c4a3dc00                 lea r8, [rip + 0xdca3c4]
000000014226af1c: 488b9588000000                 mov rdx, qword ptr [rbp + 0x88]
000000014226af23: e8bbe06600                     call 0x1428d8fe3
000000014226af28: 4c89a508010000                 mov qword ptr [rbp + 0x108], r12
000000014226af2f: b908000000                     mov ecx, 8
000000014226af34: ba58000000                     mov edx, 0x58
000000014226af39: e8c2e06600                     call 0x1428d9000
000000014226af3e: 0f0b                           ud2 
000000014226af40: 31c9                           xor ecx, ecx
000000014226af42: 4c8d0597a3dc00                 lea r8, [rip + 0xdca397]
000000014226af49: 488b95e0000000                 mov rdx, qword ptr [rbp + 0xe0]
000000014226af50: e88ee06600                     call 0x1428d8fe3
000000014226af55: b901000000                     mov ecx, 1
000000014226af5a: 4c89a588000000                 mov qword ptr [rbp + 0x88], r12
000000014226af61: 4c8d0578a3dc00                 lea r8, [rip + 0xdca378]
000000014226af68: 488b9588000000                 mov rdx, qword ptr [rbp + 0x88]
000000014226af6f: e86fe06600                     call 0x1428d8fe3
000000014226af74: b901000000                     mov ecx, 1
000000014226af79: 488b8508010000                 mov rax, qword ptr [rbp + 0x108]
000000014226af80: 488985e0000000                 mov qword ptr [rbp + 0xe0], rax
000000014226af87: 4c8d0552a3dc00                 lea r8, [rip + 0xdca352]
000000014226af8e: 488b95e0000000                 mov rdx, qword ptr [rbp + 0xe0]
000000014226af95: e849e06600                     call 0x1428d8fe3
000000014226af9a: cc                             int3 
000000014226af9b: 0f1f440000                     nop dword ptr [rax + rax]
000000014226afa0: 4889542410                     mov qword ptr [rsp + 0x10], rdx
000000014226afa5: 55                             push rbp
000000014226afa6: 4157                           push r15
000000014226afa8: 4156                           push r14
000000014226afaa: 4155                           push r13
000000014226afac: 4154                           push r12
000000014226afae: 56                             push rsi
000000014226afaf: 57                             push rdi
000000014226afb0: 53                             push rbx
000000014226afb1: 4883ec68                       sub rsp, 0x68
000000014226afb5: 488daa80000000                 lea rbp, [rdx + 0x80]
000000014226afbc: 440f294c2420                   movaps xmmword ptr [rsp + 0x20], xmm9
000000014226afc2: 440f29442430                   movaps xmmword ptr [rsp + 0x30], xmm8
000000014226afc8: 0f297c2440                     movaps xmmword ptr [rsp + 0x40], xmm7
000000014226afcd: 0f29742450                     movaps xmmword ptr [rsp + 0x50], xmm6
000000014226afd2: 4883bd0801000000               cmp qword ptr [rbp + 0x108], 0
000000014226afda: 7419                           je 0x14226aff5
000000014226afdc: 41b801000000                   mov r8d, 1
000000014226afe2: 488b8de0000000                 mov rcx, qword ptr [rbp + 0xe0]
000000014226afe9: 488b9508010000                 mov rdx, qword ptr [rbp + 0x108]
000000014226aff0: e82b8c3afe                     call 0x140613c20
000000014226aff5: 0f28742450                     movaps xmm6, xmmword ptr [rsp + 0x50]
000000014226affa: 0f287c2440                     movaps xmm7, xmmword ptr [rsp + 0x40]
000000014226afff: 440f28442430                   movaps xmm8, xmmword ptr [rsp + 0x30]
000000014226b005: 440f284c2420                   movaps xmm9, xmmword ptr [rsp + 0x20]
000000014226b00b: 4883c468                       add rsp, 0x68
000000014226b00f: 5b                             pop rbx
000000014226b010: 5f                             pop rdi
000000014226b011: 5e                             pop rsi
000000014226b012: 415c                           pop r12
000000014226b014: 415d                           pop r13
000000014226b016: 415e                           pop r14
000000014226b018: 415f                           pop r15
000000014226b01a: 5d                             pop rbp
000000014226b01b: c3                             ret 
000000014226b01c: 0f1f4000                       nop dword ptr [rax]
000000014226b020: 4889542410                     mov qword ptr [rsp + 0x10], rdx
000000014226b025: 55                             push rbp
000000014226b026: 4157                           push r15
000000014226b028: 4156                           push r14
000000014226b02a: 4155                           push r13
000000014226b02c: 4154                           push r12
000000014226b02e: 56                             push rsi
000000014226b02f: 57                             push rdi
000000014226b030: 53                             push rbx
000000014226b031: 4883ec68                       sub rsp, 0x68
000000014226b035: 488daa80000000                 lea rbp, [rdx + 0x80]
000000014226b03c: 440f294c2420                   movaps xmmword ptr [rsp + 0x20], xmm9
000000014226b042: 440f29442430                   movaps xmmword ptr [rsp + 0x30], xmm8
000000014226b048: 0f297c2440                     movaps xmmword ptr [rsp + 0x40], xmm7
000000014226b04d: 0f29742450                     movaps xmmword ptr [rsp + 0x50], xmm6
000000014226b052: 4883bdf800000000               cmp qword ptr [rbp + 0xf8], 0
000000014226b05a: 7419                           je 0x14226b075
000000014226b05c: 41b801000000                   mov r8d, 1
000000014226b062: 488b8d88000000                 mov rcx, qword ptr [rbp + 0x88]
000000014226b069: 488b95f8000000                 mov rdx, qword ptr [rbp + 0xf8]
000000014226b070: e8ab8b3afe                     call 0x140613c20
000000014226b075: 0f28742450                     movaps xmm6, xmmword ptr [rsp + 0x50]
000000014226b07a: 0f287c2440                     movaps xmm7, xmmword ptr [rsp + 0x40]
000000014226b07f: 440f28442430                   movaps xmm8, xmmword ptr [rsp + 0x30]
000000014226b085: 440f284c2420                   movaps xmm9, xmmword ptr [rsp + 0x20]
000000014226b08b: 4883c468                       add rsp, 0x68
000000014226b08f: 5b                             pop rbx
000000014226b090: 5f                             pop rdi
000000014226b091: 5e                             pop rsi
000000014226b092: 415c                           pop r12
000000014226b094: 415d                           pop r13
000000014226b096: 415e                           pop r14
000000014226b098: 415f                           pop r15
000000014226b09a: 5d                             pop rbp
000000014226b09b: c3                             ret 
