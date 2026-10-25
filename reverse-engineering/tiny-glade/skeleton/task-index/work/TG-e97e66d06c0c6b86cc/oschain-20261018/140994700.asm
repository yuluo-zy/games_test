0000000140994700: 4157                             push r15
0000000140994702: 4156                             push r14
0000000140994704: 4155                             push r13
0000000140994706: 4154                             push r12
0000000140994708: 56                               push rsi
0000000140994709: 57                               push rdi
000000014099470a: 55                               push rbp
000000014099470b: 53                               push rbx
000000014099470c: 4881ec58010000                   sub rsp, 0x158
0000000140994713: 440f29bc2440010000               movaps xmmword ptr [rsp + 0x140], xmm15
000000014099471c: 440f29b42430010000               movaps xmmword ptr [rsp + 0x130], xmm14
0000000140994725: 440f29ac2420010000               movaps xmmword ptr [rsp + 0x120], xmm13
000000014099472e: 440f29a42410010000               movaps xmmword ptr [rsp + 0x110], xmm12
0000000140994737: 440f299c2400010000               movaps xmmword ptr [rsp + 0x100], xmm11
0000000140994740: 440f299424f0000000               movaps xmmword ptr [rsp + 0xf0], xmm10
0000000140994749: 440f298c24e0000000               movaps xmmword ptr [rsp + 0xe0], xmm9
0000000140994752: 440f298424d0000000               movaps xmmword ptr [rsp + 0xd0], xmm8
000000014099475b: 0f29bc24c0000000                 movaps xmmword ptr [rsp + 0xc0], xmm7
0000000140994763: 0f29b424b0000000                 movaps xmmword ptr [rsp + 0xb0], xmm6
000000014099476b: 0f28f2                           movaps xmm6, xmm2
000000014099476e: f30f115150                       movss dword ptr [rcx + 0x50], xmm2
0000000140994773: f30f101a                         movss xmm3, dword ptr [rdx]
0000000140994777: f20f104a04                       movsd xmm1, qword ptr [rdx + 4]
000000014099477c: 0f28d3                           movaps xmm2, xmm3
000000014099477f: f30f59d3                         mulss xmm2, xmm3
0000000140994783: 0f28c1                           movaps xmm0, xmm1
0000000140994786: 0f59c1                           mulps xmm0, xmm1
0000000140994789: f30f58d0                         addss xmm2, xmm0
000000014099478d: f30f16c0                         movshdup xmm0, xmm0
0000000140994791: f30f58c2                         addss xmm0, xmm2
0000000140994795: f30f101563621202                 movss xmm2, dword ptr [rip + 0x2126263]
000000014099479d: 0f2ed0                           ucomiss xmm2, xmm0
00000001409947a0: 0f871a060000                     ja 0x140994dc0
00000001409947a6: 4c89cf                           mov rdi, r9
00000001409947a9: 4889ce                           mov rsi, rcx
00000001409947ac: 0fb66954                         movzx ebp, byte ptr [rcx + 0x54]
00000001409947b0: 440f105118                       movups xmm10, xmmword ptr [rcx + 0x18]
00000001409947b5: 0f16d9                           movlhps xmm3, xmm1
00000001409947b8: 0f28d3                           movaps xmm2, xmm3
00000001409947bb: 0fc6d1d8                         shufps xmm2, xmm1, 0xd8
00000001409947bf: 0fc6d958                         shufps xmm3, xmm1, 0x58
00000001409947c3: 410f28ca                         movaps xmm1, xmm10
00000001409947c7: 410f59ca                         mulps xmm1, xmm10
00000001409947cb: f30f16e1                         movshdup xmm4, xmm1
00000001409947cf: f30f58e1                         addss xmm4, xmm1
00000001409947d3: 0f28e9                           movaps xmm5, xmm1
00000001409947d6: 660f15e9                         unpckhpd xmm5, xmm1
00000001409947da: f30f58ec                         addss xmm5, xmm4
00000001409947de: 0fc6ed00                         shufps xmm5, xmm5, 0
00000001409947e2: 0fc6c9ff                         shufps xmm1, xmm1, 0xff
00000001409947e6: 0f5ccd                           subps xmm1, xmm5
00000001409947e9: 0f59cb                           mulps xmm1, xmm3
00000001409947ec: 410f28da                         movaps xmm3, xmm10
00000001409947f0: 410fc6dad2                       shufps xmm3, xmm10, 0xd2
00000001409947f5: 0f28e2                           movaps xmm4, xmm2
00000001409947f8: 0f59da                           mulps xmm3, xmm2
00000001409947fb: 410f59d2                         mulps xmm2, xmm10
00000001409947ff: f30f16ea                         movshdup xmm5, xmm2
0000000140994803: f30f58ea                         addss xmm5, xmm2
0000000140994807: 0f12d2                           movhlps xmm2, xmm2
000000014099480a: f30f58d5                         addss xmm2, xmm5
000000014099480e: f30f58d2                         addss xmm2, xmm2
0000000140994812: 0fc6d200                         shufps xmm2, xmm2, 0
0000000140994816: 410f59d2                         mulps xmm2, xmm10
000000014099481a: 0f58d1                           addps xmm2, xmm1
000000014099481d: 0fc6e4d2                         shufps xmm4, xmm4, 0xd2
0000000140994821: 410f59e2                         mulps xmm4, xmm10
0000000140994825: 0f5cdc                           subps xmm3, xmm4
0000000140994828: 0fc6dbd2                         shufps xmm3, xmm3, 0xd2
000000014099482c: 450f58d2                         addps xmm10, xmm10
0000000140994830: 450fc6d2ff                       shufps xmm10, xmm10, 0xff
0000000140994835: 440f59d3                         mulps xmm10, xmm3
0000000140994839: 440f58d2                         addps xmm10, xmm2
000000014099483d: 410f28ca                         movaps xmm1, xmm10
0000000140994841: 4084ed                           test bpl, bpl
0000000140994844: 750c                             jne 0x140994852
0000000140994846: 0f57c9                           xorps xmm1, xmm1
0000000140994849: 410f16ca                         movlhps xmm1, xmm10
000000014099484d: 410fc6cae2                       shufps xmm1, xmm10, 0xe2
0000000140994852: 0f28d9                           movaps xmm3, xmm1
0000000140994855: 0f59d9                           mulps xmm3, xmm1
0000000140994858: 0f28d1                           movaps xmm2, xmm1
000000014099485b: 0fc6d1e9                         shufps xmm2, xmm1, 0xe9
000000014099485f: f30f16e3                         movshdup xmm4, xmm3
0000000140994863: f30f58e3                         addss xmm4, xmm3
0000000140994867: 0f12db                           movhlps xmm3, xmm3
000000014099486a: f30f58dc                         addss xmm3, xmm4
000000014099486e: 0f57e4                           xorps xmm4, xmm4
0000000140994871: f30f51e3                         sqrtss xmm4, xmm3
0000000140994875: f3440f10058610f901               movss xmm8, dword ptr [rip + 0x1f91086]
000000014099487e: 410f28d8                         movaps xmm3, xmm8
0000000140994882: f30f5edc                         divss xmm3, xmm4
0000000140994886: 660f7ed8                         movd eax, xmm3
000000014099488a: 85c0                             test eax, eax
000000014099488c: 0f99c1                           setns cl
000000014099488f: 89c2                             mov edx, eax
0000000140994891: 81e2ffffff7f                     and edx, 0x7fffffff
0000000140994897: 81c2000080ff                     add edx, 0xff800000
000000014099489d: 81fa0000007f                     cmp edx, 0x7f000000
00000001409948a3: 0f92c2                           setb dl
00000001409948a6: 20ca                             and dl, cl
00000001409948a8: ffc8                             dec eax
00000001409948aa: 3dffff7f00                       cmp eax, 0x7fffff
00000001409948af: 0f92c0                           setb al
00000001409948b2: 08d0                             or al, dl
00000001409948b4: 7505                             jne 0x1409948bb
00000001409948b6: 0f57c9                           xorps xmm1, xmm1
00000001409948b9: eb04                             jmp 0x1409948bf
00000001409948bb: f30f59cb                         mulss xmm1, xmm3
00000001409948bf: f3440f51c8                       sqrtss xmm9, xmm0
00000001409948c4: f3410f59c9                       mulss xmm1, xmm9
00000001409948c9: f30f590d33fdf901                 mulss xmm1, dword ptr [rip + 0x1f9fd33]
00000001409948d1: f30f5e0df3cf0302                 divss xmm1, dword ptr [rip + 0x203cff3]
00000001409948d9: f30f12c3                         movsldup xmm0, xmm3
00000001409948dd: 0f59d0                           mulps xmm2, xmm0
00000001409948e0: 0fb6c0                           movzx eax, al
00000001409948e3: 660f6ec0                         movd xmm0, eax
00000001409948e7: 660f70c050                       pshufd xmm0, xmm0, 0x50
00000001409948ec: 660f72f01f                       pslld xmm0, 0x1f
00000001409948f1: 660f72e01f                       psrad xmm0, 0x1f
00000001409948f6: 660fdbc2                         pand xmm0, xmm2
00000001409948fa: 660f70c0d0                       pshufd xmm0, xmm0, 0xd0
00000001409948ff: 450fc6c980                       shufps xmm9, xmm9, 0x80
0000000140994904: 440f59c8                         mulps xmm9, xmm0
0000000140994908: 440f590d00611202                 mulps xmm9, xmmword ptr [rip + 0x2126100]
0000000140994910: 440f5e0d08611202                 divps xmm9, xmmword ptr [rip + 0x2126108]
0000000140994918: f3440f10c9                       movss xmm9, xmm1
000000014099491d: 4c8b4608                         mov r8, qword ptr [rsi + 8]
0000000140994921: 4c8b7610                         mov r14, qword ptr [rsi + 0x10]
0000000140994925: 49c1e604                         shl r14, 4
0000000140994929: 49bf661afc7a5763846b             movabs r15, 0x6b8463577afc1a66
0000000140994933: 49bce3d76841299f4b0e             movabs r12, 0xe4b9f294168d7e3
000000014099493d: 0f1f00                           nop dword ptr [rax]
0000000140994940: 4d85f6                           test r14, r14
0000000140994943: 0f84e3040000                     je 0x140994e2c
0000000140994949: 4d8d6810                         lea r13, [r8 + 0x10]
000000014099494d: 498b08                           mov rcx, qword ptr [r8]
0000000140994950: 498b4008                         mov rax, qword ptr [r8 + 8]
0000000140994954: ff5038                           call qword ptr [rax + 0x38]
0000000140994957: 4889c3                           mov rbx, rax
000000014099495a: 4889c1                           mov rcx, rax
000000014099495d: ff5218                           call qword ptr [rdx + 0x18]
0000000140994960: 4983c6f0                         add r14, -0x10
0000000140994964: 4d89e8                           mov r8, r13
0000000140994967: 4885db                           test rbx, rbx
000000014099496a: 74d4                             je 0x140994940
000000014099496c: 4d89e8                           mov r8, r13
000000014099496f: 4c39f8                           cmp rax, r15
0000000140994972: 75cc                             jne 0x140994940
0000000140994974: 4d89e8                           mov r8, r13
0000000140994977: 4c39e2                           cmp rdx, r12
000000014099497a: 75c4                             jne 0x140994940
000000014099497c: f30f1174245c                     movss dword ptr [rsp + 0x5c], xmm6
0000000140994982: f30f104304                       movss xmm0, dword ptr [rbx + 4]
0000000140994987: f30f58055957fc01                 addss xmm0, dword ptr [rip + 0x1fc5759]
000000014099498f: 4084ed                           test bpl, bpl
0000000140994992: b820000000                       mov eax, 0x20
0000000140994997: bdef300000                       mov ebp, 0x30ef
000000014099499c: 0f45e8                           cmovne ebp, eax
000000014099499f: f3440f106b08                     movss xmm13, dword ptr [rbx + 8]
00000001409949a5: f3450f16f2                       movshdup xmm14, xmm10
00000001409949aa: f3440f101b                       movss xmm11, dword ptr [rbx]
00000001409949af: 440f14d8                         unpcklps xmm11, xmm0
00000001409949b3: 450fc6d2e2                       shufps xmm10, xmm10, 0xe2
00000001409949b8: 41bd08000000                     mov r13d, 8
00000001409949be: 4c8d742460                       lea r14, [rsp + 0x60]
00000001409949c3: 488d5c2440                       lea rbx, [rsp + 0x40]
00000001409949c8: f30f103da00ef901                 movss xmm7, dword ptr [rip + 0x1f90ea0]
00000001409949d0: 0f283579601202                   movaps xmm6, xmmword ptr [rip + 0x2126079]
00000001409949d7: 660f1f840000000000               nop word ptr [rax + rax]
00000001409949e0: 440f135c2440                     movlps qword ptr [rsp + 0x40], xmm11
00000001409949e6: f3440f116c2448                   movss dword ptr [rsp + 0x48], xmm13
00000001409949ed: 440f134c244c                     movlps qword ptr [rsp + 0x4c], xmm9
00000001409949f3: 450f28e1                         movaps xmm12, xmm9
00000001409949f7: 66450f15e1                       unpckhpd xmm12, xmm9
00000001409949fc: f3440f11642454                   movss dword ptr [rsp + 0x54], xmm12
0000000140994a03: f30f1005f90ef901                 movss xmm0, dword ptr [rip + 0x1f90ef9]
0000000140994a0b: 0f13442430                       movlps qword ptr [rsp + 0x30], xmm0
0000000140994a10: 66c74424380000                   mov word ptr [rsp + 0x38], 0
0000000140994a17: 896c2428                         mov dword ptr [rsp + 0x28], ebp
0000000140994a1b: 488d442430                       lea rax, [rsp + 0x30]
0000000140994a20: 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000140994a25: 4c89f1                           mov rcx, r14
0000000140994a28: 4889fa                           mov rdx, rdi
0000000140994a2b: 4989d8                           mov r8, rbx
0000000140994a2e: 0f28df                           movaps xmm3, xmm7
0000000140994a31: e85a79f3ff                       call 0x1408cc390 ; country_core::systems::collision::world::Raycasting::spherecast
0000000140994a36: 83bc249400000002                 cmp dword ptr [rsp + 0x94], 2
0000000140994a3e: 0f8469010000                     je 0x140994bad
0000000140994a44: 0f57c0                           xorps xmm0, xmm0
0000000140994a47: f30f5f842490000000               maxss xmm0, dword ptr [rsp + 0x90]
0000000140994a50: f30f10942484000000               movss xmm2, dword ptr [rsp + 0x84]
0000000140994a59: 410f28d8                         movaps xmm3, xmm8
0000000140994a5d: f30f5dd8                         minss xmm3, xmm0
0000000140994a61: f3410f16c1                       movshdup xmm0, xmm9
0000000140994a66: f3440f59e3                       mulss xmm12, xmm3
0000000140994a6b: f30f12cb                         movsldup xmm1, xmm3
0000000140994a6f: 410f59c9                         mulps xmm1, xmm9
0000000140994a73: 0f28e1                           movaps xmm4, xmm1
0000000140994a76: 0f5925b35f1202                   mulps xmm4, xmmword ptr [rip + 0x2125fb3]
0000000140994a7d: 410f28ec                         movaps xmm5, xmm12
0000000140994a81: f30f592d037e0e02                 mulss xmm5, dword ptr [rip + 0x20e7e03]
0000000140994a89: 440f58dc                         addps xmm11, xmm4
0000000140994a8d: f3440f58ed                       addss xmm13, xmm5
0000000140994a92: 410f28e0                         movaps xmm4, xmm8
0000000140994a96: f30f5ce3                         subss xmm4, xmm3
0000000140994a9a: f30f59c4                         mulss xmm0, xmm4
0000000140994a9e: 0f28ea                           movaps xmm5, xmm2
0000000140994aa1: f30f59ea                         mulss xmm5, xmm2
0000000140994aa5: f2440f10bc2488000000             movsd xmm15, qword ptr [rsp + 0x88]
0000000140994aaf: 410f28df                         movaps xmm3, xmm15
0000000140994ab3: 410fc6d7d4                       shufps xmm2, xmm15, 0xd4
0000000140994ab8: 410fc6d7f2                       shufps xmm2, xmm15, 0xf2
0000000140994abd: 450f59ff                         mulps xmm15, xmm15
0000000140994ac1: f3410f58ef                       addss xmm5, xmm15
0000000140994ac6: f3450f16ff                       movshdup xmm15, xmm15
0000000140994acb: f3440f58fd                       addss xmm15, xmm5
0000000140994ad0: 0f57ed                           xorps xmm5, xmm5
0000000140994ad3: f3410f51ef                       sqrtss xmm5, xmm15
0000000140994ad8: 450f28f8                         movaps xmm15, xmm8
0000000140994adc: f3440f5efd                       divss xmm15, xmm5
0000000140994ae1: f3410f59df                       mulss xmm3, xmm15
0000000140994ae6: 0f28eb                           movaps xmm5, xmm3
0000000140994ae9: f30f59e8                         mulss xmm5, xmm0
0000000140994aed: 450fc6c9e2                       shufps xmm9, xmm9, 0xe2
0000000140994af2: f30f12e4                         movsldup xmm4, xmm4
0000000140994af6: 410f59e1                         mulps xmm4, xmm9
0000000140994afa: f3450f12cf                       movsldup xmm9, xmm15
0000000140994aff: 440f59ca                         mulps xmm9, xmm2
0000000140994b03: 410f28d1                         movaps xmm2, xmm9
0000000140994b07: 0f59d4                           mulps xmm2, xmm4
0000000140994b0a: f3440f16fa                       movshdup xmm15, xmm2
0000000140994b0f: f3440f58fd                       addss xmm15, xmm5
0000000140994b14: f3440f58fa                       addss xmm15, xmm2
0000000140994b19: f3410f59df                       mulss xmm3, xmm15
0000000140994b1e: f3410f12d7                       movsldup xmm2, xmm15
0000000140994b23: 410f59d1                         mulps xmm2, xmm9
0000000140994b27: f30f5cc3                         subss xmm0, xmm3
0000000140994b2b: 0f5ce2                           subps xmm4, xmm2
0000000140994b2e: f30f16d1                         movshdup xmm2, xmm1
0000000140994b32: f30f12c9                         movsldup xmm1, xmm1
0000000140994b36: f30f5915025f1202                 mulss xmm2, dword ptr [rip + 0x2125f02]
0000000140994b3e: f3410f10cc                       movss xmm1, xmm12
0000000140994b43: 0f59ce                           mulps xmm1, xmm6
0000000140994b46: 0f58cc                           addps xmm1, xmm4
0000000140994b49: f30f58c2                         addss xmm0, xmm2
0000000140994b4d: 410f28d6                         movaps xmm2, xmm14
0000000140994b51: f30f59d0                         mulss xmm2, xmm0
0000000140994b55: 410f28da                         movaps xmm3, xmm10
0000000140994b59: 0f59d9                           mulps xmm3, xmm1
0000000140994b5c: f30f16e3                         movshdup xmm4, xmm3
0000000140994b60: f30f58e2                         addss xmm4, xmm2
0000000140994b64: f30f58e3                         addss xmm4, xmm3
0000000140994b68: 0f57d2                           xorps xmm2, xmm2
0000000140994b6b: 0f2ed4                           ucomiss xmm2, xmm4
0000000140994b6e: 7346                             jae 0x140994bb6
0000000140994b70: 0f28d1                           movaps xmm2, xmm1
0000000140994b73: 0f59d1                           mulps xmm2, xmm1
0000000140994b76: 0f28d8                           movaps xmm3, xmm0
0000000140994b79: f30f59d8                         mulss xmm3, xmm0
0000000140994b7d: f30f16e2                         movshdup xmm4, xmm2
0000000140994b81: f30f58e3                         addss xmm4, xmm3
0000000140994b85: f30f58e2                         addss xmm4, xmm2
0000000140994b89: f30f10156f5e1202                 movss xmm2, dword ptr [rip + 0x2125e6f]
0000000140994b91: 0f2ed4                           ucomiss xmm2, xmm4
0000000140994b94: 7720                             ja 0x140994bb6
0000000140994b96: 0fc6c1d4                         shufps xmm0, xmm1, 0xd4
0000000140994b9a: 0fc6c1c2                         shufps xmm0, xmm1, 0xc2
0000000140994b9e: 440f28c8                         movaps xmm9, xmm0
0000000140994ba2: 41ffcd                           dec r13d
0000000140994ba5: 0f8535feffff                     jne 0x1409949e0
0000000140994bab: eb09                             jmp 0x140994bb6
0000000140994bad: 450f58d9                         addps xmm11, xmm9
0000000140994bb1: f3450f58ec                       addss xmm13, xmm12
0000000140994bb6: 440f581d22c10e02                 addps xmm11, xmmword ptr [rip + 0x20ec122]
0000000140994bbe: 450f57c9                         xorps xmm9, xmm9
0000000140994bc2: f3450f58cd                       addss xmm9, xmm13
0000000140994bc7: f30f1044245c                     movss xmm0, dword ptr [rsp + 0x5c]
0000000140994bcd: f30f59059f840302                 mulss xmm0, dword ptr [rip + 0x203849f]
0000000140994bd5: e8e6f2f801                       call 0x142923ec0 ; 
0000000140994bda: f3440f5cc0                       subss xmm8, xmm0
0000000140994bdf: 803daa01ca0200                   cmp byte ptr [rip + 0x2ca01aa], 0
0000000140994be6: 7447                             je 0x140994c2f
0000000140994be8: 410f28c3                         movaps xmm0, xmm11
0000000140994bec: f3410f59c3                       mulss xmm0, xmm11
0000000140994bf1: 410f28c9                         movaps xmm1, xmm9
0000000140994bf5: f3410f59c9                       mulss xmm1, xmm9
0000000140994bfa: f30f58c8                         addss xmm1, xmm0
0000000140994bfe: 0f2e0d835e1202                   ucomiss xmm1, dword ptr [rip + 0x2125e83]
0000000140994c05: 410f28c1                         movaps xmm0, xmm9
0000000140994c09: 410f16c3                         movlhps xmm0, xmm11
0000000140994c0d: 410fc6c3e2                       shufps xmm0, xmm11, 0xe2
0000000140994c12: 0f8698000000                     jbe 0x140994cb0
0000000140994c18: f30f51c9                         sqrtss xmm1, xmm1
0000000140994c1c: f30f12c9                         movsldup xmm1, xmm1
0000000140994c20: 0f5ec1                           divps xmm0, xmm1
0000000140994c23: 0f5905665e1202                   mulps xmm0, xmmword ptr [rip + 0x2125e66]
0000000140994c2a: e981000000                       jmp 0x140994cb0
0000000140994c2f: 410f28c1                         movaps xmm0, xmm9
0000000140994c33: 410f16c3                         movlhps xmm0, xmm11
0000000140994c37: 410fc6c3e2                       shufps xmm0, xmm11, 0xe2
0000000140994c3c: 0f28151d5e1202                   movaps xmm2, xmmword ptr [rip + 0x2125e1d]
0000000140994c43: 0f28da                           movaps xmm3, xmm2
0000000140994c46: 0f5fd8                           maxps xmm3, xmm0
0000000140994c49: 0f28c8                           movaps xmm1, xmm0
0000000140994c4c: 0fc2c803                         cmpunordps xmm1, xmm0
0000000140994c50: 0f54d1                           andps xmm2, xmm1
0000000140994c53: 0f55cb                           andnps xmm1, xmm3
0000000140994c56: 0f56ca                           orps xmm1, xmm2
0000000140994c59: 0f2815105e1202                   movaps xmm2, xmmword ptr [rip + 0x2125e10]
0000000140994c60: 0f28da                           movaps xmm3, xmm2
0000000140994c63: 0f5dd9                           minps xmm3, xmm1
0000000140994c66: 0fc2c903                         cmpunordps xmm1, xmm1
0000000140994c6a: 0f54d1                           andps xmm2, xmm1
0000000140994c6d: 0f55cb                           andnps xmm1, xmm3
0000000140994c70: 0f56ca                           orps xmm1, xmm2
0000000140994c73: 0f28d0                           movaps xmm2, xmm0
0000000140994c76: 0f5cd1                           subps xmm2, xmm1
0000000140994c79: 0f28e2                           movaps xmm4, xmm2
0000000140994c7c: 0f59e2                           mulps xmm4, xmm2
0000000140994c7f: f30f16dc                         movshdup xmm3, xmm4
0000000140994c83: f30f58dc                         addss xmm3, xmm4
0000000140994c87: 0f2e1df25d1202                   ucomiss xmm3, dword ptr [rip + 0x2125df2]
0000000140994c8e: 7620                             jbe 0x140994cb0
0000000140994c90: 0f57c0                           xorps xmm0, xmm0
0000000140994c93: f30f51c3                         sqrtss xmm0, xmm3
0000000140994c97: f30f101de55d1202                 movss xmm3, dword ptr [rip + 0x2125de5]
0000000140994c9f: f30f5ed8                         divss xmm3, xmm0
0000000140994ca3: f30f12c3                         movsldup xmm0, xmm3
0000000140994ca7: 0f59d0                           mulps xmm2, xmm0
0000000140994caa: 0f58ca                           addps xmm1, xmm2
0000000140994cad: 0f28c1                           movaps xmm0, xmm1
0000000140994cb0: 0f13442460                       movlps qword ptr [rsp + 0x60], xmm0
0000000140994cb5: f3410f16d3                       movshdup xmm2, xmm11
0000000140994cba: 488d8c24a0000000                 lea rcx, [rsp + 0xa0]
0000000140994cc2: 488d542460                       lea rdx, [rsp + 0x60]
0000000140994cc7: e814813000                       call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
0000000140994ccc: f30f10b424a8000000               movss xmm6, dword ptr [rsp + 0xa8]
0000000140994cd5: f3410f5cf1                       subss xmm6, xmm9
0000000140994cda: f20f108424a0000000               movsd xmm0, qword ptr [rsp + 0xa0]
0000000140994ce3: 410f5cc3                         subps xmm0, xmm11
0000000140994ce7: f3410f12f8                       movsldup xmm7, xmm8
0000000140994cec: 0f59f8                           mulps xmm7, xmm0
0000000140994cef: f3410f59f0                       mulss xmm6, xmm8
0000000140994cf4: 410f58fb                         addps xmm7, xmm11
0000000140994cf8: f3410f58f1                       addss xmm6, xmm9
0000000140994cfd: 4c8b4608                         mov r8, qword ptr [rsi + 8]
0000000140994d01: 4c8b7610                         mov r14, qword ptr [rsi + 0x10]
0000000140994d05: 49c1e604                         shl r14, 4
0000000140994d09: 0f1f8000000000                   nop dword ptr [rax]
0000000140994d10: 4d85f6                           test r14, r14
0000000140994d13: 0f8434010000                     je 0x140994e4d
0000000140994d19: 4d8d6810                         lea r13, [r8 + 0x10]
0000000140994d1d: 498b08                           mov rcx, qword ptr [r8]
0000000140994d20: 498b4008                         mov rax, qword ptr [r8 + 8]
0000000140994d24: ff5038                           call qword ptr [rax + 0x38]
0000000140994d27: 4889c7                           mov rdi, rax
0000000140994d2a: 4889c1                           mov rcx, rax
0000000140994d2d: ff5218                           call qword ptr [rdx + 0x18]
0000000140994d30: 4983c6f0                         add r14, -0x10
0000000140994d34: 4d89e8                           mov r8, r13
0000000140994d37: 4885ff                           test rdi, rdi
0000000140994d3a: 74d4                             je 0x140994d10
0000000140994d3c: 4d89e8                           mov r8, r13
0000000140994d3f: 4c39f8                           cmp rax, r15
0000000140994d42: 75cc                             jne 0x140994d10
0000000140994d44: 4d89e8                           mov r8, r13
0000000140994d47: 4c39e2                           cmp rdx, r12
0000000140994d4a: 75c4                             jne 0x140994d10
0000000140994d4c: f20f1007                         movsd xmm0, qword ptr [rdi]
0000000140994d50: f30f5c7708                       subss xmm6, dword ptr [rdi + 8]
0000000140994d55: 0f5cf8                           subps xmm7, xmm0
0000000140994d58: 4c8b4608                         mov r8, qword ptr [rsi + 8]
0000000140994d5c: 488b7e10                         mov rdi, qword ptr [rsi + 0x10]
0000000140994d60: 48c1e704                         shl rdi, 4
0000000140994d64: 6666662e0f1f840000000000         nop word ptr cs:[rax + rax]
0000000140994d70: 4885ff                           test rdi, rdi
0000000140994d73: 0f843f010000                     je 0x140994eb8
0000000140994d79: 4d8d7010                         lea r14, [r8 + 0x10]
0000000140994d7d: 498b08                           mov rcx, qword ptr [r8]
0000000140994d80: 498b4008                         mov rax, qword ptr [r8 + 8]
0000000140994d84: ff5040                           call qword ptr [rax + 0x40]
0000000140994d87: 4889c6                           mov rsi, rax
0000000140994d8a: 4889c1                           mov rcx, rax
0000000140994d8d: ff5218                           call qword ptr [rdx + 0x18]
0000000140994d90: 4883c7f0                         add rdi, -0x10
0000000140994d94: 4d89f0                           mov r8, r14
0000000140994d97: 4885f6                           test rsi, rsi
0000000140994d9a: 74d4                             je 0x140994d70
0000000140994d9c: 4d89f0                           mov r8, r14
0000000140994d9f: 4c39f8                           cmp rax, r15
0000000140994da2: 75cc                             jne 0x140994d70
0000000140994da4: 4d89f0                           mov r8, r14
0000000140994da7: 4c39e2                           cmp rdx, r12
0000000140994daa: 75c4                             jne 0x140994d70
0000000140994dac: f30f587608                       addss xmm6, dword ptr [rsi + 8]
0000000140994db1: f20f1006                         movsd xmm0, qword ptr [rsi]
0000000140994db5: 0f58f8                           addps xmm7, xmm0
0000000140994db8: 0f133e                           movlps qword ptr [rsi], xmm7
0000000140994dbb: f30f117608                       movss dword ptr [rsi + 8], xmm6
0000000140994dc0: 0f28b424b0000000                 movaps xmm6, xmmword ptr [rsp + 0xb0]
0000000140994dc8: 0f28bc24c0000000                 movaps xmm7, xmmword ptr [rsp + 0xc0]
0000000140994dd0: 440f288424d0000000               movaps xmm8, xmmword ptr [rsp + 0xd0]
0000000140994dd9: 440f288c24e0000000               movaps xmm9, xmmword ptr [rsp + 0xe0]
0000000140994de2: 440f289424f0000000               movaps xmm10, xmmword ptr [rsp + 0xf0]
0000000140994deb: 440f289c2400010000               movaps xmm11, xmmword ptr [rsp + 0x100]
0000000140994df4: 440f28a42410010000               movaps xmm12, xmmword ptr [rsp + 0x110]
0000000140994dfd: 440f28ac2420010000               movaps xmm13, xmmword ptr [rsp + 0x120]
0000000140994e06: 440f28b42430010000               movaps xmm14, xmmword ptr [rsp + 0x130]
0000000140994e0f: 440f28bc2440010000               movaps xmm15, xmmword ptr [rsp + 0x140]
0000000140994e18: 4881c458010000                   add rsp, 0x158
0000000140994e1f: 5b                               pop rbx
0000000140994e20: 5d                               pop rbp
0000000140994e21: 5f                               pop rdi
0000000140994e22: 5e                               pop rsi
0000000140994e23: 415c                             pop r12
0000000140994e25: 415d                             pop r13
0000000140994e27: 415e                             pop r14
0000000140994e29: 415f                             pop r15
0000000140994e2b: c3                               ret 
0000000140994e2c: 488d05b57e1502                   lea rax, [rip + 0x2157eb5]
0000000140994e33: 4889442440                       mov qword ptr [rsp + 0x40], rax
0000000140994e38: 48c744244822000000               mov qword ptr [rsp + 0x48], 0x22
0000000140994e41: 488d442440                       lea rax, [rsp + 0x40]
0000000140994e46: 4889442430                       mov qword ptr [rsp + 0x30], rax
0000000140994e4b: eb1a                             jmp 0x140994e67
0000000140994e4d: 488d05947e1502                   lea rax, [rip + 0x2157e94]
0000000140994e54: 4889442440                       mov qword ptr [rsp + 0x40], rax
0000000140994e59: 48c744244822000000               mov qword ptr [rsp + 0x48], 0x22
0000000140994e62: 48895c2430                       mov qword ptr [rsp + 0x30], rbx
0000000140994e67: 488d05225077ff                   lea rax, [rip - 0x88afde]
0000000140994e6e: 4889442438                       mov qword ptr [rsp + 0x38], rax
0000000140994e73: 488d05b67e1502                   lea rax, [rip + 0x2157eb6]
0000000140994e7a: 4889442460                       mov qword ptr [rsp + 0x60], rax
0000000140994e7f: 48c744246802000000               mov qword ptr [rsp + 0x68], 2
0000000140994e88: 48c784248000000000000000         mov qword ptr [rsp + 0x80], 0
0000000140994e94: 488d442430                       lea rax, [rsp + 0x30]
0000000140994e99: 4889442470                       mov qword ptr [rsp + 0x70], rax
0000000140994e9e: 48c744247801000000               mov qword ptr [rsp + 0x78], 1
0000000140994ea7: 488d157a7f1502                   lea rdx, [rip + 0x2157f7a]
0000000140994eae: 488d4c2460                       lea rcx, [rsp + 0x60]
0000000140994eb3: e8d844f401                       call 0x1428d9390 ; core::panicking::panic_fmt
0000000140994eb8: 488d05297e1502                   lea rax, [rip + 0x2157e29]
0000000140994ebf: 4889442440                       mov qword ptr [rsp + 0x40], rax
0000000140994ec4: 48c744244822000000               mov qword ptr [rsp + 0x48], 0x22
0000000140994ecd: 48895c2430                       mov qword ptr [rsp + 0x30], rbx
0000000140994ed2: 488d05b74f77ff                   lea rax, [rip - 0x88b049]
0000000140994ed9: 4889442438                       mov qword ptr [rsp + 0x38], rax
0000000140994ede: 488d054b7e1502                   lea rax, [rip + 0x2157e4b]
0000000140994ee5: 4889442460                       mov qword ptr [rsp + 0x60], rax
0000000140994eea: 48c744246802000000               mov qword ptr [rsp + 0x68], 2
0000000140994ef3: 48c784248000000000000000         mov qword ptr [rsp + 0x80], 0
0000000140994eff: 488d442430                       lea rax, [rsp + 0x30]
0000000140994f04: 4889442470                       mov qword ptr [rsp + 0x70], rax
0000000140994f09: 48c744247801000000               mov qword ptr [rsp + 0x78], 1
0000000140994f12: 488d158f7e1502                   lea rdx, [rip + 0x2157e8f]
0000000140994f19: 488d4c2460                       lea rcx, [rsp + 0x60]
0000000140994f1e: e86d44f401                       call 0x1428d9390 ; core::panicking::panic_fmt
0000000140994f23: cc                               int3 
