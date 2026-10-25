00000001409938b0: 4157                             push r15
00000001409938b2: 4156                             push r14
00000001409938b4: 4155                             push r13
00000001409938b6: 4154                             push r12
00000001409938b8: 56                               push rsi
00000001409938b9: 57                               push rdi
00000001409938ba: 55                               push rbp
00000001409938bb: 53                               push rbx
00000001409938bc: 4881ec08010000                   sub rsp, 0x108
00000001409938c3: 440f29b424f0000000               movaps xmmword ptr [rsp + 0xf0], xmm14
00000001409938cc: 440f29ac24e0000000               movaps xmmword ptr [rsp + 0xe0], xmm13
00000001409938d5: 440f29a424d0000000               movaps xmmword ptr [rsp + 0xd0], xmm12
00000001409938de: 440f299c24c0000000               movaps xmmword ptr [rsp + 0xc0], xmm11
00000001409938e7: 440f299424b0000000               movaps xmmword ptr [rsp + 0xb0], xmm10
00000001409938f0: 440f298c24a0000000               movaps xmmword ptr [rsp + 0xa0], xmm9
00000001409938f9: 440f29842490000000               movaps xmmword ptr [rsp + 0x90], xmm8
0000000140993902: 0f29bc2480000000                 movaps xmmword ptr [rsp + 0x80], xmm7
000000014099390a: 0f29742470                       movaps xmmword ptr [rsp + 0x70], xmm6
000000014099390f: 4c89ce                           mov rsi, r9
0000000140993912: 0f28f2                           movaps xmm6, xmm2
0000000140993915: 440f104918                       movups xmm9, xmmword ptr [rcx + 0x18]
000000014099391a: f20f104a04                       movsd xmm1, qword ptr [rdx + 4]
000000014099391f: f30f101a                         movss xmm3, dword ptr [rdx]
0000000140993923: 0f16d9                           movlhps xmm3, xmm1
0000000140993926: 0f28c3                           movaps xmm0, xmm3
0000000140993929: 0fc6c1d8                         shufps xmm0, xmm1, 0xd8
000000014099392d: 0fc6d958                         shufps xmm3, xmm1, 0x58
0000000140993931: 410f28d1                         movaps xmm2, xmm9
0000000140993935: 410f59d1                         mulps xmm2, xmm9
0000000140993939: f30f16ca                         movshdup xmm1, xmm2
000000014099393d: f30f58ca                         addss xmm1, xmm2
0000000140993941: 0f28e2                           movaps xmm4, xmm2
0000000140993944: 660f15e2                         unpckhpd xmm4, xmm2
0000000140993948: f30f58e1                         addss xmm4, xmm1
000000014099394c: 0fc6e400                         shufps xmm4, xmm4, 0
0000000140993950: 0fc6d2ff                         shufps xmm2, xmm2, 0xff
0000000140993954: 0f5cd4                           subps xmm2, xmm4
0000000140993957: 0f59d3                           mulps xmm2, xmm3
000000014099395a: 410f28c9                         movaps xmm1, xmm9
000000014099395e: 0f59c8                           mulps xmm1, xmm0
0000000140993961: f30f16d9                         movshdup xmm3, xmm1
0000000140993965: f30f58d9                         addss xmm3, xmm1
0000000140993969: 0f12c9                           movhlps xmm1, xmm1
000000014099396c: f30f58cb                         addss xmm1, xmm3
0000000140993970: f30f58c9                         addss xmm1, xmm1
0000000140993974: 0fc6c900                         shufps xmm1, xmm1, 0
0000000140993978: 410f59c9                         mulps xmm1, xmm9
000000014099397c: 0f58ca                           addps xmm1, xmm2
000000014099397f: 410f28d1                         movaps xmm2, xmm9
0000000140993983: 410fc6d1d2                       shufps xmm2, xmm9, 0xd2
0000000140993988: 0f59d0                           mulps xmm2, xmm0
000000014099398b: 0fc6c0d2                         shufps xmm0, xmm0, 0xd2
000000014099398f: 410f59c1                         mulps xmm0, xmm9
0000000140993993: 0f5cd0                           subps xmm2, xmm0
0000000140993996: 0fc6d2d2                         shufps xmm2, xmm2, 0xd2
000000014099399a: 450f58c9                         addps xmm9, xmm9
000000014099399e: 450fc6c9ff                       shufps xmm9, xmm9, 0xff
00000001409939a3: 440f59ca                         mulps xmm9, xmm2
00000001409939a7: 440f58c9                         addps xmm9, xmm1
00000001409939ab: 4c8b6108                         mov r12, qword ptr [rcx + 8]
00000001409939af: 488b5910                         mov rbx, qword ptr [rcx + 0x10]
00000001409939b3: 4989dd                           mov r13, rbx
00000001409939b6: 49c1e504                         shl r13, 4
00000001409939ba: 49be661afc7a5763846b             movabs r14, 0x6b8463577afc1a66
00000001409939c4: 49bfe3d76841299f4b0e             movabs r15, 0xe4b9f294168d7e3
00000001409939ce: 4d89e0                           mov r8, r12
00000001409939d1: 6666666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
00000001409939e0: 4d85ed                           test r13, r13
00000001409939e3: 0f8466020000                     je 0x140993c4f
00000001409939e9: 498d6810                         lea rbp, [r8 + 0x10]
00000001409939ed: 498b08                           mov rcx, qword ptr [r8]
00000001409939f0: 498b4008                         mov rax, qword ptr [r8 + 8]
00000001409939f4: ff5038                           call qword ptr [rax + 0x38]
00000001409939f7: 4889c7                           mov rdi, rax
00000001409939fa: 4889c1                           mov rcx, rax
00000001409939fd: ff5218                           call qword ptr [rdx + 0x18]
0000000140993a00: 4983c5f0                         add r13, -0x10
0000000140993a04: 4989e8                           mov r8, rbp
0000000140993a07: 4885ff                           test rdi, rdi
0000000140993a0a: 74d4                             je 0x1409939e0
0000000140993a0c: 4989e8                           mov r8, rbp
0000000140993a0f: 4c39f0                           cmp rax, r14
0000000140993a12: 75cc                             jne 0x1409939e0
0000000140993a14: 4989e8                           mov r8, rbp
0000000140993a17: 4c39fa                           cmp rdx, r15
0000000140993a1a: 75c4                             jne 0x1409939e0
0000000140993a1c: f3440f105f04                     movss xmm11, dword ptr [rdi + 4]
0000000140993a22: f3440f1007                       movss xmm8, dword ptr [rdi]
0000000140993a27: f3450f58c1                       addss xmm8, xmm9
0000000140993a2c: 450f28e9                         movaps xmm13, xmm9
0000000140993a30: 66450f15e9                       unpckhpd xmm13, xmm9
0000000140993a35: f3440f586f08                     addss xmm13, dword ptr [rdi + 8]
0000000140993a3b: f30f593531960302                 mulss xmm6, dword ptr [rip + 0x2039631]
0000000140993a43: 0f28c6                           movaps xmm0, xmm6
0000000140993a46: e87504f901                       call 0x142923ec0 ; 
0000000140993a4b: f3440f1015b01ef901               movss xmm10, dword ptr [rip + 0x1f91eb0]
0000000140993a54: f3440f5cd0                       subss xmm10, xmm0
0000000140993a59: f30f1005536f1202                 movss xmm0, dword ptr [rip + 0x2126f53]
0000000140993a61: 0f28c8                           movaps xmm1, xmm0
0000000140993a64: f3410f5fc8                       maxss xmm1, xmm8
0000000140993a69: f3440f1025466f1202               movss xmm12, dword ptr [rip + 0x2126f46]
0000000140993a72: 410f28f4                         movaps xmm6, xmm12
0000000140993a76: f30f5df1                         minss xmm6, xmm1
0000000140993a7a: f3410f5fc5                       maxss xmm0, xmm13
0000000140993a7f: 410f28fc                         movaps xmm7, xmm12
0000000140993a83: f30f5df8                         minss xmm7, xmm0
0000000140993a87: f3410f5cf0                       subss xmm6, xmm8
0000000140993a8c: f3410f5cfd                       subss xmm7, xmm13
0000000140993a91: f3410f59f2                       mulss xmm6, xmm10
0000000140993a96: f3410f59fa                       mulss xmm7, xmm10
0000000140993a9b: f3410f58f0                       addss xmm6, xmm8
0000000140993aa0: f3410f58fd                       addss xmm7, xmm13
0000000140993aa5: 488b4e20                         mov rcx, qword ptr [rsi + 0x20]
0000000140993aa9: 0f28ce                           movaps xmm1, xmm6
0000000140993aac: 0f28d7                           movaps xmm2, xmm7
0000000140993aaf: e88c5b0500                       call 0x1409e9640 ; country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>
0000000140993ab4: 440f28c0                         movaps xmm8, xmm0
0000000140993ab8: 4889df                           mov rdi, rbx
0000000140993abb: 48c1e704                         shl rdi, 4
0000000140993abf: 4d89e0                           mov r8, r12
0000000140993ac2: 66666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
0000000140993ad0: 4885ff                           test rdi, rdi
0000000140993ad3: 0f8476010000                     je 0x140993c4f
0000000140993ad9: 4d8d6810                         lea r13, [r8 + 0x10]
0000000140993add: 498b08                           mov rcx, qword ptr [r8]
0000000140993ae0: 498b4008                         mov rax, qword ptr [r8 + 8]
0000000140993ae4: ff5038                           call qword ptr [rax + 0x38]
0000000140993ae7: 4889c6                           mov rsi, rax
0000000140993aea: 4889c1                           mov rcx, rax
0000000140993aed: ff5218                           call qword ptr [rdx + 0x18]
0000000140993af0: 4883c7f0                         add rdi, -0x10
0000000140993af4: 4d89e8                           mov r8, r13
0000000140993af7: 4885f6                           test rsi, rsi
0000000140993afa: 74d4                             je 0x140993ad0
0000000140993afc: 4d89e8                           mov r8, r13
0000000140993aff: 4c39f0                           cmp rax, r14
0000000140993b02: 75cc                             jne 0x140993ad0
0000000140993b04: 4d89e8                           mov r8, r13
0000000140993b07: 4c39fa                           cmp rdx, r15
0000000140993b0a: 75c4                             jne 0x140993ad0
0000000140993b0c: f2440f102e                       movsd xmm13, qword ptr [rsi]
0000000140993b11: f3440f107608                     movss xmm14, dword ptr [rsi + 8]
0000000140993b17: 48c1e304                         shl rbx, 4
0000000140993b1b: 0f1f440000                       nop dword ptr [rax + rax]
0000000140993b20: 4885db                           test rbx, rbx
0000000140993b23: 0f8493010000                     je 0x140993cbc
0000000140993b29: 498d7c2410                       lea rdi, [r12 + 0x10]
0000000140993b2e: 498b0c24                         mov rcx, qword ptr [r12]
0000000140993b32: 498b442408                       mov rax, qword ptr [r12 + 8]
0000000140993b37: ff5040                           call qword ptr [rax + 0x40]
0000000140993b3a: 4889c6                           mov rsi, rax
0000000140993b3d: 4889c1                           mov rcx, rax
0000000140993b40: ff5218                           call qword ptr [rdx + 0x18]
0000000140993b43: 4883c3f0                         add rbx, -0x10
0000000140993b47: 4989fc                           mov r12, rdi
0000000140993b4a: 4885f6                           test rsi, rsi
0000000140993b4d: 74d1                             je 0x140993b20
0000000140993b4f: 4989fc                           mov r12, rdi
0000000140993b52: 4c39f0                           cmp rax, r14
0000000140993b55: 75c9                             jne 0x140993b20
0000000140993b57: 4989fc                           mov r12, rdi
0000000140993b5a: 4c39fa                           cmp rdx, r15
0000000140993b5d: 75c1                             jne 0x140993b20
0000000140993b5f: f3410f5cfe                       subss xmm7, xmm14
0000000140993b64: f3410f16c1                       movshdup xmm0, xmm9
0000000140993b69: f3410f58c3                       addss xmm0, xmm11
0000000140993b6e: 0f57c9                           xorps xmm1, xmm1
0000000140993b71: f3440f5fc1                       maxss xmm8, xmm1
0000000140993b76: f30f5fc8                         maxss xmm1, xmm0
0000000140993b7a: f3440f5de1                       minss xmm12, xmm1
0000000140993b7f: f3440f5ce0                       subss xmm12, xmm0
0000000140993b84: f3450f59d4                       mulss xmm10, xmm12
0000000140993b89: f3440f5805fe9f0502               addss xmm8, dword ptr [rip + 0x2059ffe]
0000000140993b92: f3440f58d0                       addss xmm10, xmm0
0000000140993b97: 410f28c0                         movaps xmm0, xmm8
0000000140993b9b: f3410f5fc2                       maxss xmm0, xmm10
0000000140993ba0: f3450fc2d203                     cmpunordss xmm10, xmm10
0000000140993ba6: 410f28ca                         movaps xmm1, xmm10
0000000140993baa: 0f55c8                           andnps xmm1, xmm0
0000000140993bad: 450f54d0                         andps xmm10, xmm8
0000000140993bb1: 440f56d1                         orps xmm10, xmm1
0000000140993bb5: f30f593d73be0e02                 mulss xmm7, dword ptr [rip + 0x20ebe73]
0000000140993bbd: f30f5e3d07dd0302                 divss xmm7, dword ptr [rip + 0x203dd07]
0000000140993bc5: f30f587e08                       addss xmm7, dword ptr [rsi + 8]
0000000140993bca: 410f14f2                         unpcklps xmm6, xmm10
0000000140993bce: 410f5cf5                         subps xmm6, xmm13
0000000140993bd2: 0f5935e76d1202                   mulps xmm6, xmmword ptr [rip + 0x2126de7]
0000000140993bd9: 0f5e35f06d1202                   divps xmm6, xmmword ptr [rip + 0x2126df0]
0000000140993be0: f20f1006                         movsd xmm0, qword ptr [rsi]
0000000140993be4: 0f58c6                           addps xmm0, xmm6
0000000140993be7: 0f1306                           movlps qword ptr [rsi], xmm0
0000000140993bea: f30f117e08                       movss dword ptr [rsi + 8], xmm7
0000000140993bef: 0f28742470                       movaps xmm6, xmmword ptr [rsp + 0x70]
0000000140993bf4: 0f28bc2480000000                 movaps xmm7, xmmword ptr [rsp + 0x80]
0000000140993bfc: 440f28842490000000               movaps xmm8, xmmword ptr [rsp + 0x90]
0000000140993c05: 440f288c24a0000000               movaps xmm9, xmmword ptr [rsp + 0xa0]
0000000140993c0e: 440f289424b0000000               movaps xmm10, xmmword ptr [rsp + 0xb0]
0000000140993c17: 440f289c24c0000000               movaps xmm11, xmmword ptr [rsp + 0xc0]
0000000140993c20: 440f28a424d0000000               movaps xmm12, xmmword ptr [rsp + 0xd0]
0000000140993c29: 440f28ac24e0000000               movaps xmm13, xmmword ptr [rsp + 0xe0]
0000000140993c32: 440f28b424f0000000               movaps xmm14, xmmword ptr [rsp + 0xf0]
0000000140993c3b: 4881c408010000                   add rsp, 0x108
0000000140993c42: 5b                               pop rbx
0000000140993c43: 5d                               pop rbp
0000000140993c44: 5f                               pop rdi
0000000140993c45: 5e                               pop rsi
0000000140993c46: 415c                             pop r12
0000000140993c48: 415d                             pop r13
0000000140993c4a: 415e                             pop r14
0000000140993c4c: 415f                             pop r15
0000000140993c4e: c3                               ret 
0000000140993c4f: 488d0592901502                   lea rax, [rip + 0x2159092]
0000000140993c56: 4889442430                       mov qword ptr [rsp + 0x30], rax
0000000140993c5b: 48c744243822000000               mov qword ptr [rsp + 0x38], 0x22
0000000140993c64: 488d442430                       lea rax, [rsp + 0x30]
0000000140993c69: 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000140993c6e: 488d051b6277ff                   lea rax, [rip - 0x889de5]
0000000140993c75: 4889442428                       mov qword ptr [rsp + 0x28], rax
0000000140993c7a: 488d05af901502                   lea rax, [rip + 0x21590af]
0000000140993c81: 4889442440                       mov qword ptr [rsp + 0x40], rax
0000000140993c86: 48c744244802000000               mov qword ptr [rsp + 0x48], 2
0000000140993c8f: 48c744246000000000               mov qword ptr [rsp + 0x60], 0
0000000140993c98: 488d442420                       lea rax, [rsp + 0x20]
0000000140993c9d: 4889442450                       mov qword ptr [rsp + 0x50], rax
0000000140993ca2: 48c744245801000000               mov qword ptr [rsp + 0x58], 1
0000000140993cab: 488d1576911502                   lea rdx, [rip + 0x2159176]
0000000140993cb2: 488d4c2440                       lea rcx, [rsp + 0x40]
0000000140993cb7: e8d456f401                       call 0x1428d9390 ; core::panicking::panic_fmt
0000000140993cbc: 488d0525901502                   lea rax, [rip + 0x2159025]
0000000140993cc3: 4889442430                       mov qword ptr [rsp + 0x30], rax
0000000140993cc8: 48c744243822000000               mov qword ptr [rsp + 0x38], 0x22
0000000140993cd1: 488d442430                       lea rax, [rsp + 0x30]
0000000140993cd6: 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000140993cdb: 488d05ae6177ff                   lea rax, [rip - 0x889e52]
0000000140993ce2: 4889442428                       mov qword ptr [rsp + 0x28], rax
0000000140993ce7: 488d0542901502                   lea rax, [rip + 0x2159042]
0000000140993cee: 4889442440                       mov qword ptr [rsp + 0x40], rax
0000000140993cf3: 48c744244802000000               mov qword ptr [rsp + 0x48], 2
0000000140993cfc: 48c744246000000000               mov qword ptr [rsp + 0x60], 0
0000000140993d05: 488d442420                       lea rax, [rsp + 0x20]
0000000140993d0a: 4889442450                       mov qword ptr [rsp + 0x50], rax
0000000140993d0f: 48c744245801000000               mov qword ptr [rsp + 0x58], 1
0000000140993d18: 488d1589901502                   lea rdx, [rip + 0x2159089]
0000000140993d1f: 488d4c2440                       lea rcx, [rsp + 0x40]
0000000140993d24: e86756f401                       call 0x1428d9390 ; core::panicking::panic_fmt
0000000140993d29: cc                               int3 
