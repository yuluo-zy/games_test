14079c6d0: 55                           push rbp
14079c6d1: 4157                         push r15
14079c6d3: 4156                         push r14
14079c6d5: 4155                         push r13
14079c6d7: 4154                         push r12
14079c6d9: 56                           push rsi
14079c6da: 57                           push rdi
14079c6db: 53                           push rbx
14079c6dc: 4881eca8070000               sub rsp, 0x7a8
14079c6e3: 488dac2480000000             lea rbp, [rsp + 0x80]
14079c6eb: 440f29bd10070000             movaps xmmword ptr [rbp + 0x710], xmm15
14079c6f3: 440f29b500070000             movaps xmmword ptr [rbp + 0x700], xmm14
14079c6fb: 440f29adf0060000             movaps xmmword ptr [rbp + 0x6f0], xmm13
14079c703: 440f29a5e0060000             movaps xmmword ptr [rbp + 0x6e0], xmm12
14079c70b: 440f299dd0060000             movaps xmmword ptr [rbp + 0x6d0], xmm11
14079c713: 440f2995c0060000             movaps xmmword ptr [rbp + 0x6c0], xmm10
14079c71b: 440f298db0060000             movaps xmmword ptr [rbp + 0x6b0], xmm9
14079c723: 440f2985a0060000             movaps xmmword ptr [rbp + 0x6a0], xmm8
14079c72b: 0f29bd90060000               movaps xmmword ptr [rbp + 0x690], xmm7
14079c732: 0f29b580060000               movaps xmmword ptr [rbp + 0x680], xmm6
14079c739: 48c78578060000feffffff       mov qword ptr [rbp + 0x678], 0xfffffffffffffffe
14079c744: 48898db0010000               mov qword ptr [rbp + 0x1b0], rcx
14079c74b: 410f1001                     movups xmm0, xmmword ptr [r9]
14079c74f: 410f104910                   movups xmm1, xmmword ptr [r9 + 0x10]
14079c754: 410f105120                   movups xmm2, xmmword ptr [r9 + 0x20]
14079c759: 410f105930                   movups xmm3, xmmword ptr [r9 + 0x30]
14079c75e: 0f295df0                     movaps xmmword ptr [rbp - 0x10], xmm3
14079c762: 0f2955e0                     movaps xmmword ptr [rbp - 0x20], xmm2
14079c766: 0f294dd0                     movaps xmmword ptr [rbp - 0x30], xmm1
14079c76a: 0f2945c0                     movaps xmmword ptr [rbp - 0x40], xmm0
14079c76e: 498b4140                     mov rax, qword ptr [r9 + 0x40]
14079c772: 48898538060000               mov qword ptr [rbp + 0x638], rax
14079c779: 498b4150                     mov rax, qword ptr [r9 + 0x50]
14079c77d: 48898588020000               mov qword ptr [rbp + 0x288], rax
14079c784: 418b415c                     mov eax, dword ptr [r9 + 0x5c]
14079c788: 898538050000                 mov dword ptr [rbp + 0x538], eax
14079c78e: 498b4160                     mov rax, qword ptr [r9 + 0x60]
14079c792: 48898590020000               mov qword ptr [rbp + 0x290], rax
14079c799: 498b4170                     mov rax, qword ptr [r9 + 0x70]
14079c79d: 48898598020000               mov qword ptr [rbp + 0x298], rax
14079c7a4: 418b417c                     mov eax, dword ptr [r9 + 0x7c]
14079c7a8: 89853c050000                 mov dword ptr [rbp + 0x53c], eax
14079c7ae: 498b8180000000               mov rax, qword ptr [r9 + 0x80]
14079c7b5: 488985a0020000               mov qword ptr [rbp + 0x2a0], rax
14079c7bc: 498b81a0000000               mov rax, qword ptr [r9 + 0xa0]
14079c7c3: 48898520020000               mov qword ptr [rbp + 0x220], rax
14079c7ca: 498b81c0000000               mov rax, qword ptr [r9 + 0xc0]
14079c7d1: 488985b8030000               mov qword ptr [rbp + 0x3b8], rax
14079c7d8: 4d8b91e0000000               mov r10, qword ptr [r9 + 0xe0]
14079c7df: 498b81f0000000               mov rax, qword ptr [r9 + 0xf0]
14079c7e6: 488985b0030000               mov qword ptr [rbp + 0x3b0], rax
14079c7ed: 418b81fc000000               mov eax, dword ptr [r9 + 0xfc]
14079c7f4: 898518030000                 mov dword ptr [rbp + 0x318], eax
14079c7fa: 498b8100010000               mov rax, qword ptr [r9 + 0x100]
14079c801: 488985b8010000               mov qword ptr [rbp + 0x1b8], rax
14079c808: 498b8110010000               mov rax, qword ptr [r9 + 0x110]
14079c80f: 48898580020000               mov qword ptr [rbp + 0x280], rax
14079c816: 418b811c010000               mov eax, dword ptr [r9 + 0x11c]
14079c81d: 8985f4020000                 mov dword ptr [rbp + 0x2f4], eax
14079c823: 4d8b9920010000               mov r11, qword ptr [r9 + 0x120]
14079c82a: 488b85b0070000               mov rax, qword ptr [rbp + 0x7b0]
14079c831: 0f104040                     movups xmm0, xmmword ptr [rax + 0x40]
14079c835: 0f104850                     movups xmm1, xmmword ptr [rax + 0x50]
14079c839: 0f298de0020000               movaps xmmword ptr [rbp + 0x2e0], xmm1
14079c840: 0f2985d0020000               movaps xmmword ptr [rbp + 0x2d0], xmm0
14079c847: 0f104060                     movups xmm0, xmmword ptr [rax + 0x60]
14079c84b: f30f6f4870                   movdqu xmm1, xmmword ptr [rax + 0x70]
14079c850: 660f7f8d80030000             movdqa xmmword ptr [rbp + 0x380], xmm1
14079c858: 0f298570030000               movaps xmmword ptr [rbp + 0x370], xmm0
14079c85f: 488b08                       mov rcx, qword ptr [rax]
14079c862: 4c8b4820                     mov r9, qword ptr [rax + 0x20]
14079c866: 4c898d58050000               mov qword ptr [rbp + 0x558], r9
14079c86d: 4c8b4830                     mov r9, qword ptr [rax + 0x30]
14079c871: 4c898d88040000               mov qword ptr [rbp + 0x488], r9
14079c878: 448b483c                     mov r9d, dword ptr [rax + 0x3c]
14079c87c: 44898d7c050000               mov dword ptr [rbp + 0x57c], r9d
14079c883: 4c8b8880000000               mov r9, qword ptr [rax + 0x80]
14079c88a: 4c898d08010000               mov qword ptr [rbp + 0x108], r9
14079c891: 4c8b8890000000               mov r9, qword ptr [rax + 0x90]
14079c898: 4c898d10010000               mov qword ptr [rbp + 0x110], r9
14079c89f: 8b809c000000                 mov eax, dword ptr [rax + 0x9c]
14079c8a5: 898500020000                 mov dword ptr [rbp + 0x200], eax
14079c8ab: 488b85c8070000               mov rax, qword ptr [rbp + 0x7c8]
14079c8b2: 4c8b08                       mov r9, qword ptr [rax]
14079c8b5: 4c898df0000000               mov qword ptr [rbp + 0xf0], r9
14079c8bc: 4c8b4820                     mov r9, qword ptr [rax + 0x20]
14079c8c0: 4c898d28020000               mov qword ptr [rbp + 0x228], r9
14079c8c7: 4c8b4830                     mov r9, qword ptr [rax + 0x30]
14079c8cb: 4c898df8000000               mov qword ptr [rbp + 0xf8], r9
14079c8d2: 448b483c                     mov r9d, dword ptr [rax + 0x3c]
14079c8d6: 44898df8010000               mov dword ptr [rbp + 0x1f8], r9d
14079c8dd: 4c8b4840                     mov r9, qword ptr [rax + 0x40]
14079c8e1: 4c898d30020000               mov qword ptr [rbp + 0x230], r9
14079c8e8: 4c8b4850                     mov r9, qword ptr [rax + 0x50]
14079c8ec: 4c898d00010000               mov qword ptr [rbp + 0x100], r9
14079c8f3: 448b485c                     mov r9d, dword ptr [rax + 0x5c]
14079c8f7: 44898dfc010000               mov dword ptr [rbp + 0x1fc], r9d
14079c8fe: 4c8b4860                     mov r9, qword ptr [rax + 0x60]
14079c902: 4c898d38020000               mov qword ptr [rbp + 0x238], r9
14079c909: 4c8b4870                     mov r9, qword ptr [rax + 0x70]
14079c90d: 4c898d18010000               mov qword ptr [rbp + 0x118], r9
14079c914: 8b407c                       mov eax, dword ptr [rax + 0x7c]
14079c917: 898504020000                 mov dword ptr [rbp + 0x204], eax
14079c91d: 4c8bb160010000               mov r14, qword ptr [rcx + 0x160]
14079c924: 66410f6f06                   movdqa xmm0, xmmword ptr [r14]
14079c929: 66440fd7e0                   pmovmskb r12d, xmm0
14079c92e: 41f7d4                       not r12d
14079c931: 498b00                       mov rax, qword ptr [r8]
14079c934: 48898568050000               mov qword ptr [rbp + 0x568], rax
14079c93b: 488b02                       mov rax, qword ptr [rdx]
14079c93e: 48898578040000               mov qword ptr [rbp + 0x478], rax
14079c945: 488b85e8070000               mov rax, qword ptr [rbp + 0x7e8]
14079c94c: 8b501c                       mov edx, dword ptr [rax + 0x1c]
14079c94f: 899508020000                 mov dword ptr [rbp + 0x208], edx
14079c955: 488b10                       mov rdx, qword ptr [rax]
14079c958: 488b4010                     mov rax, qword ptr [rax + 0x10]
14079c95c: 48898510020000               mov qword ptr [rbp + 0x210], rax
14079c963: f3440f101d307d1902           movss xmm11, dword ptr [rip + 0x2197d30]
14079c96c: f3440f1015f3062302           movss xmm10, dword ptr [rip + 0x22306f3]
14079c975: f3440f100d868f1802           movss xmm9, dword ptr [rip + 0x2188f86]
14079c97e: 450f57ff                     xorps xmm15, xmm15
14079c982: f30f10354e8f1802             movss xmm6, dword ptr [rip + 0x2188f4e]
14079c98a: f3440f1035c18f1802           movss xmm14, dword ptr [rip + 0x2188fc1]
14079c993: f3440f10254cd71b02           movss xmm12, dword ptr [rip + 0x21bd74c]
14079c99c: 440f282ddc1f1902             movaps xmm13, xmmword ptr [rip + 0x2191fdc]
14079c9a4: 48898d30060000               mov qword ptr [rbp + 0x630], rcx
14079c9ab: 488b8178010000               mov rax, qword ptr [rcx + 0x178]
14079c9b2: 4d8d7e10                     lea r15, [r14 + 0x10]
14079c9b6: 4c899d50050000               mov qword ptr [rbp + 0x550], r11
14079c9bd: 498d8b58010000               lea rcx, [r11 + 0x158]
14079c9c4: 48898d28030000               mov qword ptr [rbp + 0x328], rcx
14079c9cb: 4c899510030000               mov qword ptr [rbp + 0x310], r10
14079c9d2: 498d4a20                     lea rcx, [r10 + 0x20]
14079c9d6: 48898d20030000               mov qword ptr [rbp + 0x320], rcx
14079c9dd: 48899580040000               mov qword ptr [rbp + 0x480], rdx
14079c9e4: 488d4a20                     lea rcx, [rdx + 0x20]
14079c9e8: 48898de0000000               mov qword ptr [rbp + 0xe0], rcx
14079c9ef: 488db580050000               lea rsi, [rbp + 0x580]
14079c9f6: e965080000                   jmp 0x14079d260
14079c9fb: 0f1f440000                   nop dword ptr [rax + rax]
14079ca00: f3420f6f0c01                 movdqu xmm1, xmmword ptr [rcx + r8]
14079ca06: 660f6fd1                     movdqa xmm2, xmm1
14079ca0a: 660f74d0                     pcmpeqb xmm2, xmm0
14079ca0e: 66440fd7d2                   pmovmskb r10d, xmm2
14079ca13: 4585d2                       test r10d, r10d
14079ca16: 7428                         je 0x14079ca40
14079ca18: f3450fbcda                   tzcnt r11d, r10d
14079ca1d: 4d01c3                       add r11, r8
14079ca20: 4921d3                       and r11, rdx
14079ca23: 49f7db                       neg r11
14079ca26: 4d6bdb78                     imul r11, r11, 0x78
14079ca2a: 4a3b3c18                     cmp rdi, qword ptr [rax + r11]
14079ca2e: 7436                         je 0x14079ca66
14079ca30: 458d5aff                     lea r11d, [r10 - 1]
14079ca34: 664521d3                     and r11w, r10w
14079ca38: 4589da                       mov r10d, r11d
14079ca3b: 75db                         jne 0x14079ca18
14079ca3d: 0f1f00                       nop dword ptr [rax]
14079ca40: 660f740d488f1802             pcmpeqb xmm1, xmmword ptr [rip + 0x2188f48]
14079ca48: 66440fd7d1                   pmovmskb r10d, xmm1
14079ca4d: 4585d2                       test r10d, r10d
14079ca50: 0f85ea0c0000                 jne 0x14079d740
14079ca56: 4d01c8                       add r8, r9
14079ca59: 4983c010                     add r8, 0x10
14079ca5d: 4983c110                     add r9, 0x10
14079ca61: 4921d0                       and r8, rdx
14079ca64: eb9a                         jmp 0x14079ca00
14079ca66: 4a8b5c19a0                   mov rbx, qword ptr [rcx + r11 - 0x60]
14079ca6b: 4889d8                       mov rax, rbx
14079ca6e: ba0c000000                   mov edx, 0xc
14079ca73: 48f7e2                       mul rdx
14079ca76: 4889c7                       mov rdi, rax
14079ca79: 0f803b520000                 jo 0x1407a1cba
14079ca7f: be00000000                   mov esi, 0
14079ca84: 48b8fcffffffffffff7f         movabs rax, 0x7ffffffffffffffc
14079ca8e: 4839c7                       cmp rdi, rax
14079ca91: 0f8725520000                 ja 0x1407a1cbc
14079ca97: 4a8b541998                   mov rdx, qword ptr [rcx + r11 - 0x68]
14079ca9c: 4885ff                       test rdi, rdi
14079ca9f: 7438                         je 0x14079cad9
14079caa1: 48899568060000               mov qword ptr [rbp + 0x668], rdx
14079caa8: 0fb6055a81e902               movzx eax, byte ptr [rip + 0x2e9815a]
14079caaf: be04000000                   mov esi, 4
14079cab4: ba04000000                   mov edx, 4
14079cab9: 4889f9                       mov rcx, rdi
14079cabc: e84f71e7ff                   call 0x140613c10
14079cac1: 4885c0                       test rax, rax
14079cac4: 0f84f2510000                 je 0x1407a1cbc
14079caca: 4989c5                       mov r13, rax
14079cacd: 4889de                       mov rsi, rbx
14079cad0: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
14079cad7: eb08                         jmp 0x14079cae1
14079cad9: 41bd04000000                 mov r13d, 4
14079cadf: 31f6                         xor esi, esi
14079cae1: 4c89e9                       mov rcx, r13
14079cae4: 4989f8                       mov r8, rdi
14079cae7: e814711802                   call 0x142923c00
14079caec: 4889b580050000               mov qword ptr [rbp + 0x580], rsi
14079caf3: 4c89ad88050000               mov qword ptr [rbp + 0x588], r13
14079cafa: 48899d90050000               mov qword ptr [rbp + 0x590], rbx
14079cb01: 488d8d48010000               lea rcx, [rbp + 0x148]
14079cb08: 488db580050000               lea rsi, [rbp + 0x580]
14079cb0f: 4889f2                       mov rdx, rsi
14079cb12: 41b801000000                 mov r8d, 1
14079cb18: e8c3d1ac00                   call 0x141269ce0
14079cb1d: 31ff                         xor edi, edi
14079cb1f: 483bbd48010000               cmp rdi, qword ptr [rbp + 0x148]
14079cb26: 0f80140c0000                 jo 0x14079d740
14079cb2c: 488b8578010000               mov rax, qword ptr [rbp + 0x178]
14079cb33: 48898560030000               mov qword ptr [rbp + 0x360], rax
14079cb3a: 0f108548010000               movups xmm0, xmmword ptr [rbp + 0x148]
14079cb41: 0f108d58010000               movups xmm1, xmmword ptr [rbp + 0x158]
14079cb48: 0f109568010000               movups xmm2, xmmword ptr [rbp + 0x168]
14079cb4f: 0f299550030000               movaps xmmword ptr [rbp + 0x350], xmm2
14079cb56: 0f298d40030000               movaps xmmword ptr [rbp + 0x340], xmm1
14079cb5d: 0f298530030000               movaps xmmword ptr [rbp + 0x330], xmm0
14079cb64: 488b9538030000               mov rdx, qword ptr [rbp + 0x338]
14079cb6b: 488b8540030000               mov rax, qword ptr [rbp + 0x340]
14079cb72: 488d0440                     lea rax, [rax + rax*2]
14079cb76: 4c8d0482                     lea r8, [rdx + rax*4]
14079cb7a: 4889f1                       mov rcx, rsi
14079cb7d: 4c8d0dac574802               lea r9, [rip + 0x24857ac]
14079cb84: e857deb1ff                   call 0x1402ba9e0
14079cb89: 488b9d58030000               mov rbx, qword ptr [rbp + 0x358]
14079cb90: 4885db                       test rbx, rbx
14079cb93: 7450                         je 0x14079cbe5
14079cb95: 488bbd50030000               mov rdi, qword ptr [rbp + 0x350]
14079cb9c: 488d349d00000000             lea rsi, [rbx*4]
14079cba4: 0fb6055e80e902               movzx eax, byte ptr [rip + 0x2e9805e]
14079cbab: ba04000000                   mov edx, 4
14079cbb0: 4889f1                       mov rcx, rsi
14079cbb3: e85870e7ff                   call 0x140613c10
14079cbb8: 4885c0                       test rax, rax
14079cbbb: 0f840d510000                 je 0x1407a1cce
14079cbc1: 4883fb08                     cmp rbx, 8
14079cbc5: 0f93c2                       setae dl
14079cbc8: 4889c1                       mov rcx, rax
14079cbcb: 4829f9                       sub rcx, rdi
14079cbce: 4883f920                     cmp rcx, 0x20
14079cbd2: 0f93c1                       setae cl
14079cbd5: 84ca                         test dl, cl
14079cbd7: 7516                         jne 0x14079cbef
14079cbd9: 4531c0                       xor r8d, r8d
14079cbdc: 488db580050000               lea rsi, [rbp + 0x580]
14079cbe3: eb3b                         jmp 0x14079cc20
14079cbe5: b804000000                   mov eax, 4
14079cbea: e99a000000                   jmp 0x14079cc89
14079cbef: 4989d8                       mov r8, rbx
14079cbf2: 4983e0f8                     and r8, 0xfffffffffffffff8
14079cbf6: 31c9                         xor ecx, ecx
14079cbf8: 488db580050000               lea rsi, [rbp + 0x580]
14079cbff: 90                           nop 
14079cc00: 0f10048f                     movups xmm0, xmmword ptr [rdi + rcx*4]
14079cc04: 0f104c8f10                   movups xmm1, xmmword ptr [rdi + rcx*4 + 0x10]
14079cc09: 0f110488                     movups xmmword ptr [rax + rcx*4], xmm0
14079cc0d: 0f114c8810                   movups xmmword ptr [rax + rcx*4 + 0x10], xmm1
14079cc12: 4883c108                     add rcx, 8
14079cc16: 4939c8                       cmp r8, rcx
14079cc19: 75e5                         jne 0x14079cc00
14079cc1b: 4c39c3                       cmp rbx, r8
14079cc1e: 7467                         je 0x14079cc87
14079cc20: 4889da                       mov rdx, rbx
14079cc23: 4c89c1                       mov rcx, r8
14079cc26: 4883e203                     and rdx, 3
14079cc2a: 7416                         je 0x14079cc42
14079cc2c: 4c89c1                       mov rcx, r8
14079cc2f: 90                           nop 
14079cc30: f30f10048f                   movss xmm0, dword ptr [rdi + rcx*4]
14079cc35: f30f110488                   movss dword ptr [rax + rcx*4], xmm0
14079cc3a: 48ffc1                       inc rcx
14079cc3d: 48ffca                       dec rdx
14079cc40: 75ee                         jne 0x14079cc30
14079cc42: 4929d8                       sub r8, rbx
14079cc45: 4983f8fc                     cmp r8, -4
14079cc49: 773c                         ja 0x14079cc87
14079cc4b: 0f1f440000                   nop dword ptr [rax + rax]
14079cc50: f30f10048f                   movss xmm0, dword ptr [rdi + rcx*4]
14079cc55: f30f110488                   movss dword ptr [rax + rcx*4], xmm0
14079cc5a: f30f10448f04                 movss xmm0, dword ptr [rdi + rcx*4 + 4]
14079cc60: f30f11448804                 movss dword ptr [rax + rcx*4 + 4], xmm0
14079cc66: f30f10448f08                 movss xmm0, dword ptr [rdi + rcx*4 + 8]
14079cc6c: f30f11448808                 movss dword ptr [rax + rcx*4 + 8], xmm0
14079cc72: f30f10448f0c                 movss xmm0, dword ptr [rdi + rcx*4 + 0xc]
14079cc78: f30f1144880c                 movss dword ptr [rax + rcx*4 + 0xc], xmm0
14079cc7e: 4883c104                     add rcx, 4
14079cc82: 4839cb                       cmp rbx, rcx
14079cc85: 75c9                         jne 0x14079cc50
14079cc87: 31ff                         xor edi, edi
14079cc89: f30f108560030000             movss xmm0, dword ptr [rbp + 0x360]
14079cc91: 0f108d80050000               movups xmm1, xmmword ptr [rbp + 0x580]
14079cc98: 0f298d40020000               movaps xmmword ptr [rbp + 0x240], xmm1
14079cc9f: 488b8d90050000               mov rcx, qword ptr [rbp + 0x590]
14079cca6: 48898d50020000               mov qword ptr [rbp + 0x250], rcx
14079ccad: 48899d58020000               mov qword ptr [rbp + 0x258], rbx
14079ccb4: 488985e8000000               mov qword ptr [rbp + 0xe8], rax
14079ccbb: 48898560020000               mov qword ptr [rbp + 0x260], rax
14079ccc2: 48899d68020000               mov qword ptr [rbp + 0x268], rbx
14079ccc9: f30f118570020000             movss dword ptr [rbp + 0x270], xmm0
14079ccd1: f30f5905778c1802             mulss xmm0, dword ptr [rip + 0x2188c77]
14079ccd9: e8c2721802                   call 0x142923fa0
14079ccde: f3480f2cc0                   cvttss2si rax, xmm0
14079cce3: 4889c1                       mov rcx, rax
14079cce6: 48c1f93f                     sar rcx, 0x3f
14079ccea: 0f28c8                       movaps xmm1, xmm0
14079cced: f30f5c0de7d31b02             subss xmm1, dword ptr [rip + 0x21bd3e7]
14079ccf5: f3480f2cd1                   cvttss2si rdx, xmm1
14079ccfa: 4821ca                       and rdx, rcx
14079ccfd: 4809c2                       or rdx, rax
14079cd00: 0f57d2                       xorps xmm2, xmm2
14079cd03: 0f2ec2                       ucomiss xmm0, xmm2
14079cd06: 480f42d7                     cmovb rdx, rdi
14079cd0a: 0f2e05cfd31b02               ucomiss xmm0, dword ptr [rip + 0x21bd3cf]
14079cd11: 48c7c0ffffffff               mov rax, 0xffffffffffffffff
14079cd18: 480f47d0                     cmova rdx, rax
14079cd1c: 4883fa01                     cmp rdx, 1
14079cd20: 4883d200                     adc rdx, 0
14079cd24: 48899d18020000               mov qword ptr [rbp + 0x218], rbx
14079cd2b: 48899548060000               mov qword ptr [rbp + 0x648], rdx
14079cd32: 780a                         js 0x14079cd3e
14079cd34: 0f57c0                       xorps xmm0, xmm0
14079cd37: f3480f2ac2                   cvtsi2ss xmm0, rdx
14079cd3c: eb1a                         jmp 0x14079cd58
14079cd3e: 4889d0                       mov rax, rdx
14079cd41: 48d1e8                       shr rax, 1
14079cd44: 89d1                         mov ecx, edx
14079cd46: 83e101                       and ecx, 1
14079cd49: 4809c1                       or rcx, rax
14079cd4c: 0f57c0                       xorps xmm0, xmm0
14079cd4f: f3480f2ac1                   cvtsi2ss xmm0, rcx
14079cd54: f30f58c0                     addss xmm0, xmm0
14079cd58: 450f28d9                     movaps xmm11, xmm9
14079cd5c: f3440f5ed8                   divss xmm11, xmm0
14079cd61: 31db                         xor ebx, ebx
14079cd63: 483b9d48060000               cmp rbx, qword ptr [rbp + 0x648]
14079cd6a: 7225                         jb 0x14079cd91
14079cd6c: e946040000                   jmp 0x14079d1b7
14079cd71: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14079cd80: 410f28d0                     movaps xmm2, xmm8
14079cd84: 483b9d48060000               cmp rbx, qword ptr [rbp + 0x648]
14079cd8b: 0f8326040000                 jae 0x14079d1b7
14079cd91: f30f1195b8020000             movss dword ptr [rbp + 0x2b8], xmm2
14079cd99: 410f2ed7                     ucomiss xmm2, xmm15
14079cd9d: 0f8247320000                 jb 0x14079ffea
14079cda3: 440f2eca                     ucomiss xmm9, xmm2
14079cda7: 0f823d320000                 jb 0x14079ffea
14079cdad: 0f2ed2                       ucomiss xmm2, xmm2
14079cdb0: 0f8aa5320000                 jp 0x1407a005b
14079cdb6: 410f28c3                     movaps xmm0, xmm11
14079cdba: f30f58c2                     addss xmm0, xmm2
14079cdbe: 450f28c1                     movaps xmm8, xmm9
14079cdc2: f3440f5dc0                   minss xmm8, xmm0
14079cdc7: f3440f1185b8020000           movss dword ptr [rbp + 0x2b8], xmm8
14079cdd0: 450f2ec7                     ucomiss xmm8, xmm15
14079cdd4: 0f829e320000                 jb 0x1407a0078
14079cdda: 450f2ec8                     ucomiss xmm9, xmm8
14079cdde: 0f8294320000                 jb 0x1407a0078
14079cde4: 450f2ec0                     ucomiss xmm8, xmm8
14079cde8: 0f8afb320000                 jp 0x1407a00e9
14079cdee: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
14079cdf5: 488d9530030000               lea rdx, [rbp + 0x330]
14079cdfc: 410f28d8                     movaps xmm3, xmm8
14079ce00: e8ebdaac00                   call 0x14126a8f0
14079ce05: 48ffc3                       inc rbx
14079ce08: 483bbdc0030000               cmp rdi, qword ptr [rbp + 0x3c0]
14079ce0f: 0f806bffffff                 jo 0x14079cd80
14079ce15: 488b85f0030000               mov rax, qword ptr [rbp + 0x3f0]
14079ce1c: 488985b0050000               mov qword ptr [rbp + 0x5b0], rax
14079ce23: 0f1085c0030000               movups xmm0, xmmword ptr [rbp + 0x3c0]
14079ce2a: 0f108dd0030000               movups xmm1, xmmword ptr [rbp + 0x3d0]
14079ce31: 0f1095e0030000               movups xmm2, xmmword ptr [rbp + 0x3e0]
14079ce38: 0f2995a0050000               movaps xmmword ptr [rbp + 0x5a0], xmm2
14079ce3f: 0f298d90050000               movaps xmmword ptr [rbp + 0x590], xmm1
14079ce46: 0f298580050000               movaps xmmword ptr [rbp + 0x580], xmm0
14079ce4d: 488d8d90040000               lea rcx, [rbp + 0x490]
14079ce54: 4889f2                       mov rdx, rsi
14079ce57: 0f28d6                       movaps xmm2, xmm6
14079ce5a: 41b901000000                 mov r9d, 1
14079ce60: e82bc0ac00                   call 0x141268e90
14079ce65: 488b8580050000               mov rax, qword ptr [rbp + 0x580]
14079ce6c: 4885c0                       test rax, rax
14079ce6f: 741a                         je 0x14079ce8b
14079ce71: 488b8d88050000               mov rcx, qword ptr [rbp + 0x588]
14079ce78: 48c1e002                     shl rax, 2
14079ce7c: 488d1440                     lea rdx, [rax + rax*2]
14079ce80: 41b804000000                 mov r8d, 4
14079ce86: e8956de7ff                   call 0x140613c20
14079ce8b: 488b9598050000               mov rdx, qword ptr [rbp + 0x598]
14079ce92: 4885d2                       test rdx, rdx
14079ce95: 7416                         je 0x14079cead
14079ce97: 488b8da0050000               mov rcx, qword ptr [rbp + 0x5a0]
14079ce9e: 48c1e202                     shl rdx, 2
14079cea2: 41b804000000                 mov r8d, 4
14079cea8: e8736de7ff                   call 0x140613c20
14079cead: 483bbd90040000               cmp rdi, qword ptr [rbp + 0x490]
14079ceb4: 0f80c6feffff                 jo 0x14079cd80
14079ceba: 0f108590040000               movups xmm0, xmmword ptr [rbp + 0x490]
14079cec1: 0f108da0040000               movups xmm1, xmmword ptr [rbp + 0x4a0]
14079cec8: 0f1095b0040000               movups xmm2, xmmword ptr [rbp + 0x4b0]
14079cecf: 0f2985d0040000               movaps xmmword ptr [rbp + 0x4d0], xmm0
14079ced6: 0f298de0040000               movaps xmmword ptr [rbp + 0x4e0], xmm1
14079cedd: 488b85c0040000               mov rax, qword ptr [rbp + 0x4c0]
14079cee4: 48898500050000               mov qword ptr [rbp + 0x500], rax
14079ceeb: 0f2995f0040000               movaps xmmword ptr [rbp + 0x4f0], xmm2
14079cef2: 488bbdd8040000               mov rdi, qword ptr [rbp + 0x4d8]
14079cef9: 488b85e0040000               mov rax, qword ptr [rbp + 0x4e0]
14079cf00: 410f28c6                     movaps xmm0, xmm14
14079cf04: 4885c0                       test rax, rax
14079cf07: 743e                         je 0x14079cf47
14079cf09: 488d0c8500000000             lea rcx, [rax*4]
14079cf11: 488d0c49                     lea rcx, [rcx + rcx*2]
14079cf15: 31d2                         xor edx, edx
14079cf17: 410f28ce                     movaps xmm1, xmm14
14079cf1b: 410f28c6                     movaps xmm0, xmm14
14079cf1f: 90                           nop 
14079cf20: f30f10541704                 movss xmm2, dword ptr [rdi + rdx + 4]
14079cf26: f30fc2c003                   cmpunordss xmm0, xmm0
14079cf2b: 0f28d8                       movaps xmm3, xmm0
14079cf2e: 0f54da                       andps xmm3, xmm2
14079cf31: f30f5dd1                     minss xmm2, xmm1
14079cf35: 0f55c2                       andnps xmm0, xmm2
14079cf38: 0f56c3                       orps xmm0, xmm3
14079cf3b: 4883c20c                     add rdx, 0xc
14079cf3f: 0f28c8                       movaps xmm1, xmm0
14079cf42: 4839d1                       cmp rcx, rdx
14079cf45: 75d9                         jne 0x14079cf20
14079cf47: 48c785b802000000000000       mov qword ptr [rbp + 0x2b8], 0
14079cf52: 48c785c002000004000000       mov qword ptr [rbp + 0x2c0], 4
14079cf5d: 48c785c802000000000000       mov qword ptr [rbp + 0x2c8], 0
14079cf68: f30f12f8                     movsldup xmm7, xmm0
14079cf6c: 0f583d3d4c2e02               addps xmm7, xmmword ptr [rip + 0x22e4c3d]
14079cf73: 4889c1                       mov rcx, rax
14079cf76: 48ffc9                       dec rcx
14079cf79: 7807                         js 0x14079cf82
14079cf7b: f34c0f2ac9                   cvtsi2ss xmm9, rcx
14079cf80: eb16                         jmp 0x14079cf98
14079cf82: 4889ca                       mov rdx, rcx
14079cf85: 48d1ea                       shr rdx, 1
14079cf88: 83e101                       and ecx, 1
14079cf8b: 4809d1                       or rcx, rdx
14079cf8e: f34c0f2ac9                   cvtsi2ss xmm9, rcx
14079cf93: f3450f58c9                   addss xmm9, xmm9
14079cf98: 488d0440                     lea rax, [rax + rax*2]
14079cf9c: 488d0487                     lea rax, [rdi + rax*4]
14079cfa0: 48898558060000               mov qword ptr [rbp + 0x658], rax
14079cfa7: 4989f8                       mov r8, rdi
14079cfaa: 31f6                         xor esi, esi
14079cfac: eb41                         jmp 0x14079cfef
14079cfae: 6690                         nop 
14079cfb0: 4c8b8568060000               mov r8, qword ptr [rbp + 0x668]
14079cfb7: 4983c00c                     add r8, 0xc
14079cfbb: 48ffc6                       inc rsi
14079cfbe: 488b85c0020000               mov rax, qword ptr [rbp + 0x2c0]
14079cfc5: 4a8d0c6d00000000             lea rcx, [r13*2]
14079cfcd: 4c01e9                       add rcx, r13
14079cfd0: 8b9588050000                 mov edx, dword ptr [rbp + 0x588]
14079cfd6: 89548808                     mov dword ptr [rax + rcx*4 + 8], edx
14079cfda: 488b9580050000               mov rdx, qword ptr [rbp + 0x580]
14079cfe1: 48891488                     mov qword ptr [rax + rcx*4], rdx
14079cfe5: 49ffc5                       inc r13
14079cfe8: 4c89adc8020000               mov qword ptr [rbp + 0x2c8], r13
14079cfef: 4c3b8558060000               cmp r8, qword ptr [rbp + 0x658]
14079cff6: 4c898568060000               mov qword ptr [rbp + 0x668], r8
14079cffd: 0f84dd000000                 je 0x14079d0e0
14079d003: 4885f6                       test rsi, rsi
14079d006: 7818                         js 0x14079d020
14079d008: 0f57c0                       xorps xmm0, xmm0
14079d00b: f3480f2ac6                   cvtsi2ss xmm0, rsi
14079d010: eb28                         jmp 0x14079d03a
14079d012: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14079d020: 4889f0                       mov rax, rsi
14079d023: 48d1e8                       shr rax, 1
14079d026: 89f1                         mov ecx, esi
14079d028: 83e101                       and ecx, 1
14079d02b: 4809c1                       or rcx, rax
14079d02e: 0f57c0                       xorps xmm0, xmm0
14079d031: f3480f2ac1                   cvtsi2ss xmm0, rcx
14079d036: f30f58c0                     addss xmm0, xmm0
14079d03a: f3410f5ec1                   divss xmm0, xmm9
14079d03f: f3410f58c4                   addss xmm0, xmm12
14079d044: 410f54c5                     andps xmm0, xmm13
14079d048: f30f58c0                     addss xmm0, xmm0
14079d04c: 0f28c8                       movaps xmm1, xmm0
14079d04f: f30f59c8                     mulss xmm1, xmm0
14079d053: f30f59c8                     mulss xmm1, xmm0
14079d057: f30f1015a5881802             movss xmm2, dword ptr [rip + 0x21888a5]
14079d05f: 0f28c2                       movaps xmm0, xmm2
14079d062: f30f5cc1                     subss xmm0, xmm1
14079d066: 0f28ca                       movaps xmm1, xmm2
14079d069: f30f5cc8                     subss xmm1, xmm0
14079d06d: 0f14c1                       unpcklps xmm0, xmm1
14079d070: 0f59c7                       mulps xmm0, xmm7
14079d073: f30f16d0                     movshdup xmm2, xmm0
14079d077: f30f58d0                     addss xmm2, xmm0
14079d07b: f3410f1000                   movss xmm0, dword ptr [r8]
14079d080: f3410f104808                 movss xmm1, dword ptr [r8 + 8]
14079d086: f30f1185c0030000             movss dword ptr [rbp + 0x3c0], xmm0
14079d08e: f30f118dc4030000             movss dword ptr [rbp + 0x3c4], xmm1
14079d096: 488d8d80050000               lea rcx, [rbp + 0x580]
14079d09d: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079d0a4: e837fd4f00                   call 0x140c9cde0
14079d0a9: 4c8badc8020000               mov r13, qword ptr [rbp + 0x2c8]
14079d0b0: 4c3badb8020000               cmp r13, qword ptr [rbp + 0x2b8]
14079d0b7: 0f85f3feffff                 jne 0x14079cfb0
14079d0bd: 488d8db8020000               lea rcx, [rbp + 0x2b8]
14079d0c4: 488d15e5522e02               lea rdx, [rip + 0x22e52e5]
14079d0cb: e8b0204000                   call 0x140b9f180
14079d0d0: e9dbfeffff                   jmp 0x14079cfb0
14079d0d5: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
14079d0e0: 488b85c8020000               mov rax, qword ptr [rbp + 0x2c8]
14079d0e7: 488985d0030000               mov qword ptr [rbp + 0x3d0], rax
14079d0ee: 0f1085b8020000               movups xmm0, xmmword ptr [rbp + 0x2b8]
14079d0f5: 0f2985c0030000               movaps xmmword ptr [rbp + 0x3c0], xmm0
14079d0fc: 488d8d80050000               lea rcx, [rbp + 0x580]
14079d103: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079d10a: e8617d3600                   call 0x140b04e70
14079d10f: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
14079d116: 488d9580050000               lea rdx, [rbp + 0x580]
14079d11d: e8ee7d3600                   call 0x140b04f10
14079d122: 488d8d80050000               lea rcx, [rbp + 0x580]
14079d129: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079d130: f30f101540ff2202             movss xmm2, dword ptr [rip + 0x222ff40]
14079d138: e8037e3600                   call 0x140b04f40
14079d13d: 488d8d80050000               lea rcx, [rbp + 0x580]
14079d144: 488b9568050000               mov rdx, qword ptr [rbp + 0x568]
14079d14b: e8207e3600                   call 0x140b04f70
14079d150: 488b85d0040000               mov rax, qword ptr [rbp + 0x4d0]
14079d157: 4885c0                       test rax, rax
14079d15a: 7416                         je 0x14079d172
14079d15c: 48c1e002                     shl rax, 2
14079d160: 488d1440                     lea rdx, [rax + rax*2]
14079d164: 41b804000000                 mov r8d, 4
14079d16a: 4889f9                       mov rcx, rdi
14079d16d: e8ae6ae7ff                   call 0x140613c20
14079d172: 488b95e8040000               mov rdx, qword ptr [rbp + 0x4e8]
14079d179: 4885d2                       test rdx, rdx
14079d17c: f3440f100d7f871802           movss xmm9, dword ptr [rip + 0x218877f]
14079d185: 488db580050000               lea rsi, [rbp + 0x580]
14079d18c: 7416                         je 0x14079d1a4
14079d18e: 488b8df0040000               mov rcx, qword ptr [rbp + 0x4f0]
14079d195: 48c1e202                     shl rdx, 2
14079d199: 41b804000000                 mov r8d, 4
14079d19f: e87c6ae7ff                   call 0x140613c20
14079d1a4: 410f28d0                     movaps xmm2, xmm8
14079d1a8: 31ff                         xor edi, edi
14079d1aa: 483b9d48060000               cmp rbx, qword ptr [rbp + 0x648]
14079d1b1: 0f82dafbffff                 jb 0x14079cd91
14079d1b7: 488b9540020000               mov rdx, qword ptr [rbp + 0x240]
14079d1be: 4885d2                       test rdx, rdx
14079d1c1: 7416                         je 0x14079d1d9
14079d1c3: 488b8d48020000               mov rcx, qword ptr [rbp + 0x248]
14079d1ca: 48c1e203                     shl rdx, 3
14079d1ce: 41b804000000                 mov r8d, 4
14079d1d4: e8476ae7ff                   call 0x140613c20
14079d1d9: 488b9518020000               mov rdx, qword ptr [rbp + 0x218]
14079d1e0: 4885d2                       test rdx, rdx
14079d1e3: f3440f101db0741902           movss xmm11, dword ptr [rip + 0x21974b0]
14079d1ec: 7416                         je 0x14079d204
14079d1ee: 48c1e202                     shl rdx, 2
14079d1f2: 41b804000000                 mov r8d, 4
14079d1f8: 488b8de8000000               mov rcx, qword ptr [rbp + 0xe8]
14079d1ff: e81c6ae7ff                   call 0x140613c20
14079d204: 488b8530030000               mov rax, qword ptr [rbp + 0x330]
14079d20b: 4885c0                       test rax, rax
14079d20e: 741a                         je 0x14079d22a
14079d210: 488b8d38030000               mov rcx, qword ptr [rbp + 0x338]
14079d217: 48c1e002                     shl rax, 2
14079d21b: 488d1440                     lea rdx, [rax + rax*2]
14079d21f: 41b804000000                 mov r8d, 4
14079d225: e8f669e7ff                   call 0x140613c20
14079d22a: 488b9548030000               mov rdx, qword ptr [rbp + 0x348]
14079d231: 4885d2                       test rdx, rdx
14079d234: 0f8406050000                 je 0x14079d740
14079d23a: 488b8d50030000               mov rcx, qword ptr [rbp + 0x350]
14079d241: 48c1e202                     shl rdx, 2
14079d245: 41b804000000                 mov r8d, 4
14079d24b: e8d069e7ff                   call 0x140613c20
14079d250: e9eb040000                   jmp 0x14079d740
14079d255: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
14079d260: 4883e801                     sub rax, 1
14079d264: 0f82e2040000                 jb 0x14079d74c
14079d26a: 664585e4                     test r12w, r12w
14079d26e: 48898560050000               mov qword ptr [rbp + 0x560], rax
14079d275: 7419                         je 0x14079d290
14079d277: 418d4424ff                   lea eax, [r12 - 1]
14079d27c: 4421e0                       and eax, r12d
14079d27f: eb3d                         jmp 0x14079d2be
14079d281: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14079d290: 66410f6f07                   movdqa xmm0, xmmword ptr [r15]
14079d295: 66440fd7e0                   pmovmskb r12d, xmm0
14079d29a: 4981c680feffff               add r14, -0x180
14079d2a1: 4983c710                     add r15, 0x10
14079d2a5: 4181fcffff0000               cmp r12d, 0xffff
14079d2ac: 74e2                         je 0x14079d290
14079d2ae: b9feffffff                   mov ecx, 0xfffffffe
14079d2b3: 4429e1                       sub ecx, r12d
14079d2b6: 41f7d4                       not r12d
14079d2b9: 4489e0                       mov eax, r12d
14079d2bc: 21c8                         and eax, ecx
14079d2be: 4489e1                       mov ecx, r12d
14079d2c1: f30fbcc9                     tzcnt ecx, ecx
14079d2c5: 4189c4                       mov r12d, eax
14079d2c8: 48f7d9                       neg rcx
14079d2cb: 488d3449                     lea rsi, [rcx + rcx*2]
14079d2cf: 498b44f6e8                   mov rax, qword ptr [r14 + rsi*8 - 0x18]
14079d2d4: 48898508030000               mov qword ptr [rbp + 0x308], rax
14079d2db: f3450f1044f6f4               movss xmm8, dword ptr [r14 + rsi*8 - 0xc]
14079d2e2: f3410f107cf6fc               movss xmm7, dword ptr [r14 + rsi*8 - 4]
14079d2e9: f3450f104cf6f8               movss xmm9, dword ptr [r14 + rsi*8 - 8]
14079d2f0: f3450f58cb                   addss xmm9, xmm11
14079d2f5: f3440f1185c0030000           movss dword ptr [rbp + 0x3c0], xmm8
14079d2fe: f3440f118dc4030000           movss dword ptr [rbp + 0x3c4], xmm9
14079d307: f30f11bdc8030000             movss dword ptr [rbp + 0x3c8], xmm7
14079d30f: 488dbd80050000               lea rdi, [rbp + 0x580]
14079d316: 4889f9                       mov rcx, rdi
14079d319: 488d9dc0030000               lea rbx, [rbp + 0x3c0]
14079d320: 4889da                       mov rdx, rbx
14079d323: e868513600                   call 0x140b02490
14079d328: 4889d9                       mov rcx, rbx
14079d32b: 4889fa                       mov rdx, rdi
14079d32e: e8cd513600                   call 0x140b02500
14079d333: f3410f1054f6f0               movss xmm2, dword ptr [r14 + rsi*8 - 0x10]
14079d33a: 4889fe                       mov rsi, rdi
14079d33d: f3410f59d2                   mulss xmm2, xmm10
14079d342: 4889f9                       mov rcx, rdi
14079d345: 4889da                       mov rdx, rbx
14079d348: e893513600                   call 0x140b024e0
14079d34d: 4889f9                       mov rcx, rdi
14079d350: 488b9568050000               mov rdx, qword ptr [rbp + 0x568]
14079d357: e8c4513600                   call 0x140b02520
14079d35c: 488d05d54f2e02               lea rax, [rip + 0x22e4fd5]
14079d363: 488985c0030000               mov qword ptr [rbp + 0x3c0], rax
14079d36a: 488d051fcb96ff               lea rax, [rip - 0x6934e1]
14079d371: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
14079d378: 488d8508030000               lea rax, [rbp + 0x308]
14079d37f: 488985d0030000               mov qword ptr [rbp + 0x3d0], rax
14079d386: 488d05f3f2ffff               lea rax, [rip - 0xd0d]
14079d38d: 488985d8030000               mov qword ptr [rbp + 0x3d8], rax
14079d394: 488d05ad4f2e02               lea rax, [rip + 0x22e4fad]
14079d39b: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079d3a2: 48c7858805000002000000       mov qword ptr [rbp + 0x588], 2
14079d3ad: 48899d90050000               mov qword ptr [rbp + 0x590], rbx
14079d3b4: 48c7859805000002000000       mov qword ptr [rbp + 0x598], 2
14079d3bf: 48c785a005000000000000       mov qword ptr [rbp + 0x5a0], 0
14079d3ca: 488d8d40020000               lea rcx, [rbp + 0x240]
14079d3d1: 4889fa                       mov rdx, rdi
14079d3d4: e8b7cb5d00                   call 0x140d79f90
14079d3d9: c785c4030000cdcc4c3e         mov dword ptr [rbp + 0x3c4], 0x3e4ccccd
14079d3e3: f3440f1185c8030000           movss dword ptr [rbp + 0x3c8], xmm8
14079d3ec: f3440f118dcc030000           movss dword ptr [rbp + 0x3cc], xmm9
14079d3f5: f30f11bdd0030000             movss dword ptr [rbp + 0x3d0], xmm7
14079d3fd: c685c003000000               mov byte ptr [rbp + 0x3c0], 0
14079d404: c685d004000000               mov byte ptr [rbp + 0x4d0], 0
14079d40b: c6856506000001               mov byte ptr [rbp + 0x665], 1
14079d412: 4889f9                       mov rcx, rdi
14079d415: 4889da                       mov rdx, rbx
14079d418: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079d41f: e8ecb92000                   call 0x1409a8e10
14079d424: f3440f100dd7841802           movss xmm9, dword ptr [rip + 0x21884d7]
14079d42d: 808d2906000002               or byte ptr [rbp + 0x629], 2
14079d434: 41b8b0000000                 mov r8d, 0xb0
14079d43a: 4889d9                       mov rcx, rbx
14079d43d: 4889f2                       mov rdx, rsi
14079d440: e8bb671802                   call 0x142923c00
14079d445: c6855306000001               mov byte ptr [rbp + 0x653], 1
14079d44c: 4889f1                       mov rcx, rsi
14079d44f: e85cbb2000                   call 0x1409a8fb0
14079d454: 48c7859804000000000000       mov qword ptr [rbp + 0x498], 0
14079d45f: c7859004000001000000         mov dword ptr [rbp + 0x490], 1
14079d469: c6855306000001               mov byte ptr [rbp + 0x653], 1
14079d470: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
14079d477: 4889f2                       mov rdx, rsi
14079d47a: 4c8d8590040000               lea r8, [rbp + 0x490]
14079d481: e84abb2000                   call 0x1409a8fd0
14079d486: c6855306000000               mov byte ptr [rbp + 0x653], 0
14079d48d: 4889f1                       mov rcx, rsi
14079d490: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079d497: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079d49e: e8ddb92000                   call 0x1409a8e80
14079d4a3: c6856506000001               mov byte ptr [rbp + 0x665], 1
14079d4aa: 488dbdc0030000               lea rdi, [rbp + 0x3c0]
14079d4b1: 4889f9                       mov rcx, rdi
14079d4b4: 4889f2                       mov rdx, rsi
14079d4b7: 4531c0                       xor r8d, r8d
14079d4ba: e821ba2000                   call 0x1409a8ee0
14079d4bf: c6856506000000               mov byte ptr [rbp + 0x665], 0
14079d4c6: 488b8d78040000               mov rcx, qword ptr [rbp + 0x478]
14079d4cd: 488d9540020000               lea rdx, [rbp + 0x240]
14079d4d4: 4989f8                       mov r8, rdi
14079d4d7: e8145d1800                   call 0x1409231f0
14079d4dc: 48898580010000               mov qword ptr [rbp + 0x180], rax
14079d4e3: 48899588010000               mov qword ptr [rbp + 0x188], rdx
14079d4ea: 488d8d80010000               lea rcx, [rbp + 0x180]
14079d4f1: e8fa101900                   call 0x14092e5f0
14079d4f6: 84c0                         test al, al
14079d4f8: 0f84b0010000                 je 0x14079d6ae
14079d4fe: 488b85b0030000               mov rax, qword ptr [rbp + 0x3b0]
14079d505: 8b8d18030000                 mov ecx, dword ptr [rbp + 0x318]
14079d50b: 8908                         mov dword ptr [rax], ecx
14079d50d: 488b8d20030000               mov rcx, qword ptr [rbp + 0x320]
14079d514: 488dbd08030000               lea rdi, [rbp + 0x308]
14079d51b: 4889fa                       mov rdx, rdi
14079d51e: e8bd46e1ff                   call 0x1405b1be0
14079d523: 4889f1                       mov rcx, rsi
14079d526: 488b9510030000               mov rdx, qword ptr [rbp + 0x310]
14079d52d: 4989c0                       mov r8, rax
14079d530: 4989f9                       mov r9, rdi
14079d533: e8a8973d00                   call 0x140b76ce0
14079d538: f6858005000001               test byte ptr [rbp + 0x580], 1
14079d53f: 0f8469010000                 je 0x14079d6ae
14079d545: 488b8580020000               mov rax, qword ptr [rbp + 0x280]
14079d54c: 8b8df4020000                 mov ecx, dword ptr [rbp + 0x2f4]
14079d552: 8908                         mov dword ptr [rax], ecx
14079d554: f20f10bd08030000             movsd xmm7, qword ptr [rbp + 0x308]
14079d55c: 0fb605a676e902               movzx eax, byte ptr [rip + 0x2e976a6]
14079d563: b920000000                   mov ecx, 0x20
14079d568: ba08000000                   mov edx, 8
14079d56d: e89e66e7ff                   call 0x140613c10
14079d572: 4885c0                       test rax, rax
14079d575: 0f84f4460000                 je 0x1407a1c6f
14079d57b: 48c70001000000               mov qword ptr [rax], 1
14079d582: 48c7400801000000             mov qword ptr [rax + 8], 1
14079d58a: c7401001000000               mov dword ptr [rax + 0x10], 1
14079d591: 0f137814                     movlps qword ptr [rax + 0x14], xmm7
14079d595: 48898588050000               mov qword ptr [rbp + 0x588], rax
14079d59c: 48c785800500000e000000       mov qword ptr [rbp + 0x580], 0xe
14079d5a7: 488b8db8010000               mov rcx, qword ptr [rbp + 0x1b8]
14079d5ae: 4889f2                       mov rdx, rsi
14079d5b1: 4c8d05b04d2e02               lea r8, [rip + 0x22e4db0]
14079d5b8: e803792e00                   call 0x140a84ec0
14079d5bd: 4889f1                       mov rcx, rsi
14079d5c0: 488d9580010000               lea rdx, [rbp + 0x180]
14079d5c7: e874101900                   call 0x14092e640
14079d5cc: 83bd8005000002               cmp dword ptr [rbp + 0x580], 2
14079d5d3: 7517                         jne 0x14079d5ec
14079d5d5: 48c785d004000000000000       mov qword ptr [rbp + 0x4d0], 0
14079d5e0: c785d804000000000000         mov dword ptr [rbp + 0x4d8], 0
14079d5ea: eb1a                         jmp 0x14079d606
14079d5ec: 488d8da4050000               lea rcx, [rbp + 0x5a4]
14079d5f3: 8b4108                       mov eax, dword ptr [rcx + 8]
14079d5f6: 8985d8040000                 mov dword ptr [rbp + 0x4d8], eax
14079d5fc: 488b01                       mov rax, qword ptr [rcx]
14079d5ff: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
14079d606: 8b85d8040000                 mov eax, dword ptr [rbp + 0x4d8]
14079d60c: 488d8dd3030000               lea rcx, [rbp + 0x3d3]
14079d613: 894108                       mov dword ptr [rcx + 8], eax
14079d616: 488b85d0040000               mov rax, qword ptr [rbp + 0x4d0]
14079d61d: 488901                       mov qword ptr [rcx], rax
14079d620: 8b8508020000                 mov eax, dword ptr [rbp + 0x208]
14079d626: 488b8d10020000               mov rcx, qword ptr [rbp + 0x210]
14079d62d: 8901                         mov dword ptr [rcx], eax
14079d62f: 488b8580040000               mov rax, qword ptr [rbp + 0x480]
14079d636: 488b7030                     mov rsi, qword ptr [rax + 0x30]
14079d63a: 488b7840                     mov rdi, qword ptr [rax + 0x40]
14079d63e: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
14079d642: 7513                         jne 0x14079d657
14079d644: 488b8de0000000               mov rcx, qword ptr [rbp + 0xe0]
14079d64b: 488d152e494802               lea rdx, [rip + 0x248492e]
14079d652: e87943e9ff                   call 0x1406319d0
14079d657: 488b9580040000               mov rdx, qword ptr [rbp + 0x480]
14079d65e: 488b4228                     mov rax, qword ptr [rdx + 0x28]
14079d662: 486bce38                     imul rcx, rsi, 0x38
14079d666: 48893c08                     mov qword ptr [rax + rcx], rdi
14079d66a: c644080812                   mov byte ptr [rax + rcx + 8], 0x12
14079d66f: 0f1085c0030000               movups xmm0, xmmword ptr [rbp + 0x3c0]
14079d676: f30f6f8dcf030000             movdqu xmm1, xmmword ptr [rbp + 0x3cf]
14079d67e: 0f11440809                   movups xmmword ptr [rax + rcx + 9], xmm0
14079d683: f30f7f4c0818                 movdqu xmmword ptr [rax + rcx + 0x18], xmm1
14079d689: 660f6e05df811802             movd xmm0, dword ptr [rip + 0x21881df]
14079d691: 660fd6440828                 movq qword ptr [rax + rcx + 0x28], xmm0
14079d697: c644083000                   mov byte ptr [rax + rcx + 0x30], 0
14079d69c: 48ffc6                       inc rsi
14079d69f: 48897230                     mov qword ptr [rdx + 0x30], rsi
14079d6a3: 48ff4240                     inc qword ptr [rdx + 0x40]
14079d6a7: 488db580050000               lea rsi, [rbp + 0x580]
14079d6ae: 488d8d80010000               lea rcx, [rbp + 0x180]
14079d6b5: e8c60f1900                   call 0x14092e680
14079d6ba: 84c0                         test al, al
14079d6bc: 0f847e000000                 je 0x14079d740
14079d6c2: 8b8d08030000                 mov ecx, dword ptr [rbp + 0x308]
14079d6c8: 8b950c030000                 mov edx, dword ptr [rbp + 0x30c]
14079d6ce: e89dc71b00                   call 0x140959e70
14079d6d3: 4889c7                       mov rdi, rax
14079d6d6: 488985d8000000               mov qword ptr [rbp + 0xd8], rax
14079d6dd: 488b8550050000               mov rax, qword ptr [rbp + 0x550]
14079d6e4: 4883b85001000000             cmp qword ptr [rax + 0x150], 0
14079d6ec: 7452                         je 0x14079d740
14079d6ee: 488b8d28030000               mov rcx, qword ptr [rbp + 0x328]
14079d6f5: 488d95d8000000               lea rdx, [rbp + 0xd8]
14079d6fc: e86f36e1ff                   call 0x1405b0d70
14079d701: 488b9550050000               mov rdx, qword ptr [rbp + 0x550]
14079d708: 488b8a38010000               mov rcx, qword ptr [rdx + 0x138]
14079d70f: 488b9240010000               mov rdx, qword ptr [rdx + 0x140]
14079d716: 4989d0                       mov r8, rdx
14079d719: 4921c0                       and r8, rax
14079d71c: 48c1e839                     shr rax, 0x39
14079d720: 660f6ec0                     movd xmm0, eax
14079d724: 660f60c0                     punpcklbw xmm0, xmm0
14079d728: f20f70c000                   pshuflw xmm0, xmm0, 0
14079d72d: 660f70c000                   pshufd xmm0, xmm0, 0
14079d732: 488d4188                     lea rax, [rcx - 0x78]
14079d736: 4531c9                       xor r9d, r9d
14079d739: e9c2f2ffff                   jmp 0x14079ca00
14079d73e: 6690                         nop 
14079d740: 488b8560050000               mov rax, qword ptr [rbp + 0x560]
14079d747: e914fbffff                   jmp 0x14079d260
14079d74c: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
14079d753: f3410f104678                 movss xmm0, dword ptr [r14 + 0x78]
14079d759: 0f57f6                       xorps xmm6, xmm6
14079d75c: 0f2ec6                       ucomiss xmm0, xmm6
14079d75f: 0f868d0d0000                 jbe 0x14079e4f2
14079d765: 488b85b8070000               mov rax, qword ptr [rbp + 0x7b8]
14079d76c: 488b10                       mov rdx, qword ptr [rax]
14079d76f: 4889f1                       mov rcx, rsi
14079d772: e879964f00                   call 0x140c96df0
14079d777: f30f108d80050000             movss xmm1, dword ptr [rbp + 0x580]
14079d77f: f30f108588050000             movss xmm0, dword ptr [rbp + 0x588]
14079d787: 0f28d1                       movaps xmm2, xmm1
14079d78a: f30f59d1                     mulss xmm2, xmm1
14079d78e: 0f28d8                       movaps xmm3, xmm0
14079d791: f30f59d8                     mulss xmm3, xmm0
14079d795: f30f58da                     addss xmm3, xmm2
14079d799: f30f51db                     sqrtss xmm3, xmm3
14079d79d: 410f28d1                     movaps xmm2, xmm9
14079d7a1: f30f5ed3                     divss xmm2, xmm3
14079d7a5: f30f59ca                     mulss xmm1, xmm2
14079d7a9: f30f59d0                     mulss xmm2, xmm0
14079d7ad: 488d8d80050000               lea rcx, [rbp + 0x580]
14079d7b4: e817384f00                   call 0x140c90fd0
14079d7b9: 0f288580050000               movaps xmm0, xmmword ptr [rbp + 0x580]
14079d7c0: 0f28c8                       movaps xmm1, xmm0
14079d7c3: 0fc6c800                     shufps xmm1, xmm0, 0
14079d7c7: 0f28d0                       movaps xmm2, xmm0
14079d7ca: 0fc6d055                     shufps xmm2, xmm0, 0x55
14079d7ce: 0f28d8                       movaps xmm3, xmm0
14079d7d1: 0fc6d8aa                     shufps xmm3, xmm0, 0xaa
14079d7d5: 0fc6c0ff                     shufps xmm0, xmm0, 0xff
14079d7d9: f20f1025ef432e02             movsd xmm4, qword ptr [rip + 0x22e43ef]
14079d7e1: 0f59cc                       mulps xmm1, xmm4
14079d7e4: 0f590d85f92202               mulps xmm1, xmmword ptr [rip + 0x222f985]
14079d7eb: 0f282dce432e02               movaps xmm5, xmmword ptr [rip + 0x22e43ce]
14079d7f2: 0f59d4                       mulps xmm2, xmm4
14079d7f5: 0f591584f92202               mulps xmm2, xmmword ptr [rip + 0x222f984]
14079d7fc: 0f59c5                       mulps xmm0, xmm5
14079d7ff: 0f59dd                       mulps xmm3, xmm5
14079d802: 0f591d87f92202               mulps xmm3, xmmword ptr [rip + 0x222f987]
14079d809: 0f58c8                       addps xmm1, xmm0
14079d80c: 0f58da                       addps xmm3, xmm2
14079d80f: 0f58d9                       addps xmm3, xmm1
14079d812: 0f299dc0010000               movaps xmmword ptr [rbp + 0x1c0], xmm3
14079d819: 498b467c                     mov rax, qword ptr [r14 + 0x7c]
14079d81d: 48898540050000               mov qword ptr [rbp + 0x540], rax
14079d824: 418b8684000000               mov eax, dword ptr [r14 + 0x84]
14079d82b: 898548050000                 mov dword ptr [rbp + 0x548], eax
14079d831: f3410f104678                 movss xmm0, dword ptr [r14 + 0x78]
14079d837: f30f5905e9f82202             mulss xmm0, dword ptr [rip + 0x222f8e9]
14079d83f: f30f118548010000             movss dword ptr [rbp + 0x148], xmm0
14079d847: c685c003000000               mov byte ptr [rbp + 0x3c0], 0
14079d84e: 488d05d1452e02               lea rax, [rip + 0x22e45d1]
14079d855: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
14079d85c: 48c785d00300000a000000       mov qword ptr [rbp + 0x3d0], 0xa
14079d867: c685d803000001               mov byte ptr [rbp + 0x3d8], 1
14079d86e: 488d05bb452e02               lea rax, [rip + 0x22e45bb]
14079d875: 488985e0030000               mov qword ptr [rbp + 0x3e0], rax
14079d87c: 48c785e803000012000000       mov qword ptr [rbp + 0x3e8], 0x12
14079d887: 48c785f003000000000000       mov qword ptr [rbp + 0x3f0], 0
14079d892: 31c9                         xor ecx, ecx
14079d894: 488dbdd0040000               lea rdi, [rbp + 0x4d0]
14079d89b: eb0d                         jmp 0x14079d8aa
14079d89d: 0f1f00                       nop dword ptr [rax]
14079d8a0: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
14079d8a7: 48ffc1                       inc rcx
14079d8aa: 4883f902                     cmp rcx, 2
14079d8ae: 0f8423020000                 je 0x14079dad7
14079d8b4: 488d0449                     lea rax, [rcx + rcx*2]
14079d8b8: 0fb69cc5c0030000             movzx ebx, byte ptr [rbp + rax*8 + 0x3c0]
14079d8c0: 80fb02                       cmp bl, 2
14079d8c3: 0f840e020000                 je 0x14079dad7
14079d8c9: 48898d58060000               mov qword ptr [rbp + 0x658], rcx
14079d8d0: 488d04c5c0030000             lea rax, [rax*8 + 0x3c0]
14079d8d8: 4801e8                       add rax, rbp
14079d8db: 0f104008                     movups xmm0, xmmword ptr [rax + 8]
14079d8df: 0f118590040000               movups xmmword ptr [rbp + 0x490], xmm0
14079d8e6: 488d8540050000               lea rax, [rbp + 0x540]
14079d8ed: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079d8f4: 488d0505e15a00               lea rax, [rip + 0x5ae105]
14079d8fb: 48898588050000               mov qword ptr [rbp + 0x588], rax
14079d902: 488d8590040000               lea rax, [rbp + 0x490]
14079d909: 48898590050000               mov qword ptr [rbp + 0x590], rax
14079d910: 488d05895fd9ff               lea rax, [rip - 0x26a077]
14079d917: 48898598050000               mov qword ptr [rbp + 0x598], rax
14079d91e: 488d85c0010000               lea rax, [rbp + 0x1c0]
14079d925: 488985a0050000               mov qword ptr [rbp + 0x5a0], rax
14079d92c: 488d055dd65a00               lea rax, [rip + 0x5ad65d]
14079d933: 488985a8050000               mov qword ptr [rbp + 0x5a8], rax
14079d93a: 488d8548010000               lea rax, [rbp + 0x148]
14079d941: 488985b0050000               mov qword ptr [rbp + 0x5b0], rax
14079d948: 488d05c1ef5e00               lea rax, [rip + 0x5eefc1]
14079d94f: 488985b8050000               mov qword ptr [rbp + 0x5b8], rax
14079d956: 488d0553492e02               lea rax, [rip + 0x22e4953]
14079d95d: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
14079d964: 48c785d804000004000000       mov qword ptr [rbp + 0x4d8], 4
14079d96f: 4c8dad80050000               lea r13, [rbp + 0x580]
14079d976: 4c89ade0040000               mov qword ptr [rbp + 0x4e0], r13
14079d97d: 48c785e804000004000000       mov qword ptr [rbp + 0x4e8], 4
14079d988: 48c785f004000000000000       mov qword ptr [rbp + 0x4f0], 0
14079d993: 488d8da8000000               lea rcx, [rbp + 0xa8]
14079d99a: 4889fe                       mov rsi, rdi
14079d99d: 4889fa                       mov rdx, rdi
14079d9a0: e8ebc55d00                   call 0x140d79f90
14079d9a5: 488b85a8000000               mov rax, qword ptr [rbp + 0xa8]
14079d9ac: 48898568060000               mov qword ptr [rbp + 0x668], rax
14079d9b3: 4c8bb5b0000000               mov r14, qword ptr [rbp + 0xb0]
14079d9ba: 4c8bbdb8000000               mov r15, qword ptr [rbp + 0xb8]
14079d9c1: 4c8ba590040000               mov r12, qword ptr [rbp + 0x490]
14079d9c8: 488bbd98040000               mov rdi, qword ptr [rbp + 0x498]
14079d9cf: f30f10bd48010000             movss xmm7, dword ptr [rbp + 0x148]
14079d9d7: 488b8540050000               mov rax, qword ptr [rbp + 0x540]
14079d9de: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079d9e5: 8b8548050000                 mov eax, dword ptr [rbp + 0x548]
14079d9eb: 898588050000                 mov dword ptr [rbp + 0x588], eax
14079d9f1: 440f2885c0010000             movaps xmm8, xmmword ptr [rbp + 0x1c0]
14079d9f9: 0fb6050972e902               movzx eax, byte ptr [rip + 0x2e97209]
14079da00: b950000000                   mov ecx, 0x50
14079da05: ba10000000                   mov edx, 0x10
14079da0a: e80162e7ff                   call 0x140613c10
14079da0f: 4885c0                       test rax, rax
14079da12: 4c89b548060000               mov qword ptr [rbp + 0x648], r14
14079da19: 0f845f420000                 je 0x1407a1c7e
14079da1f: 48c70001000000               mov qword ptr [rax], 1
14079da26: 48c7400801000000             mov qword ptr [rax + 8], 1
14079da2e: 4c896010                     mov qword ptr [rax + 0x10], r12
14079da32: 48897818                     mov qword ptr [rax + 0x18], rdi
14079da36: 440f294020                   movaps xmmword ptr [rax + 0x20], xmm8
14079da3b: f30f117830                   movss dword ptr [rax + 0x30], xmm7
14079da40: 488b8d80050000               mov rcx, qword ptr [rbp + 0x580]
14079da47: 48894834                     mov qword ptr [rax + 0x34], rcx
14079da4b: 8b8d88050000                 mov ecx, dword ptr [rbp + 0x588]
14079da51: 89483c                       mov dword ptr [rax + 0x3c], ecx
14079da54: 885840                       mov byte ptr [rax + 0x40], bl
14079da57: 488d0d92482e02               lea rcx, [rip + 0x22e4892]
14079da5e: 48894c2420                   mov qword ptr [rsp + 0x20], rcx
14079da63: 4c89e9                       mov rcx, r13
14079da66: 4c89f2                       mov rdx, r14
14079da69: 4d89f8                       mov r8, r15
14079da6c: 4989c1                       mov r9, rax
14079da6f: e83cfe0f00                   call 0x14089d8b0
14079da74: 4889f7                       mov rdi, rsi
14079da77: 488b85a0050000               mov rax, qword ptr [rbp + 0x5a0]
14079da7e: 488985f0040000               mov qword ptr [rbp + 0x4f0], rax
14079da85: 0f108580050000               movups xmm0, xmmword ptr [rbp + 0x580]
14079da8c: 0f108d90050000               movups xmm1, xmmword ptr [rbp + 0x590]
14079da93: 0f298de0040000               movaps xmmword ptr [rbp + 0x4e0], xmm1
14079da9a: 0f2985d0040000               movaps xmmword ptr [rbp + 0x4d0], xmm0
14079daa1: 488b8d68050000               mov rcx, qword ptr [rbp + 0x568]
14079daa8: 4889f2                       mov rdx, rsi
14079daab: e8e0f43800                   call 0x140b2cf90
14079dab0: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
14079dab7: 4885d2                       test rdx, rdx
14079daba: 0f84e0fdffff                 je 0x14079d8a0
14079dac0: 41b801000000                 mov r8d, 1
14079dac6: 488b8d48060000               mov rcx, qword ptr [rbp + 0x648]
14079dacd: e84e61e7ff                   call 0x140613c20
14079dad2: e9c9fdffff                   jmp 0x14079d8a0
14079dad7: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
14079dade: 41837e7001                   cmp dword ptr [r14 + 0x70], 1
14079dae3: 488db580050000               lea rsi, [rbp + 0x580]
14079daea: 0f85020a0000                 jne 0x14079e4f2
14079daf0: 418b7e74                     mov edi, dword ptr [r14 + 0x74]
14079daf4: 0f2885c0010000               movaps xmm0, xmmword ptr [rbp + 0x1c0]
14079dafb: 0f28d0                       movaps xmm2, xmm0
14079dafe: 0f59d0                       mulps xmm2, xmm0
14079db01: f30f16ca                     movshdup xmm1, xmm2
14079db05: f30f58ca                     addss xmm1, xmm2
14079db09: 0f28da                       movaps xmm3, xmm2
14079db0c: 660f15da                     unpckhpd xmm3, xmm2
14079db10: f30f58d9                     addss xmm3, xmm1
14079db14: 0fc6db00                     shufps xmm3, xmm3, 0
14079db18: 0fc6d2ff                     shufps xmm2, xmm2, 0xff
14079db1c: 0f5cd3                       subps xmm2, xmm3
14079db1f: 0f280dba402e02               movaps xmm1, xmmword ptr [rip + 0x22e40ba]
14079db26: 0f59d1                       mulps xmm2, xmm1
14079db29: 0f59c8                       mulps xmm1, xmm0
14079db2c: f30f16d9                     movshdup xmm3, xmm1
14079db30: f30f58d9                     addss xmm3, xmm1
14079db34: 0f12c9                       movhlps xmm1, xmm1
14079db37: f30f58cb                     addss xmm1, xmm3
14079db3b: f30f58c9                     addss xmm1, xmm1
14079db3f: 0fc6c900                     shufps xmm1, xmm1, 0
14079db43: 0f59c8                       mulps xmm1, xmm0
14079db46: 0f58ca                       addps xmm1, xmm2
14079db49: f30f10153f002502             movss xmm2, dword ptr [rip + 0x225003f]
14079db51: 0f28d8                       movaps xmm3, xmm0
14079db54: 0f59da                       mulps xmm3, xmm2
14079db57: 0f28e0                       movaps xmm4, xmm0
14079db5a: 0fc6e0c9                     shufps xmm4, xmm0, 0xc9
14079db5e: 0f59e2                       mulps xmm4, xmm2
14079db61: 0fc6dbd2                     shufps xmm3, xmm3, 0xd2
14079db65: 0f5ce3                       subps xmm4, xmm3
14079db68: 0f28d0                       movaps xmm2, xmm0
14079db6b: 0f58d0                       addps xmm2, xmm0
14079db6e: 0fc6d2ff                     shufps xmm2, xmm2, 0xff
14079db72: 0f59d4                       mulps xmm2, xmm4
14079db75: 0f58d1                       addps xmm2, xmm1
14079db78: f20f108d40050000             movsd xmm1, qword ptr [rbp + 0x540]
14079db80: 0f58ca                       addps xmm1, xmm2
14079db83: 0f12d2                       movhlps xmm2, xmm2
14079db86: f30f589548050000             addss xmm2, dword ptr [rbp + 0x548]
14079db8e: 0f138d90050000               movlps qword ptr [rbp + 0x590], xmm1
14079db95: f30f119598050000             movss dword ptr [rbp + 0x598], xmm2
14079db9d: 0f298580050000               movaps xmmword ptr [rbp + 0x580], xmm0
14079dba4: 48b80000803f0000803f         movabs rax, 0x3f8000003f800000
14079dbae: 4889859c050000               mov qword ptr [rbp + 0x59c], rax
14079dbb5: c785a40500000000803f         mov dword ptr [rbp + 0x5a4], 0x3f800000
14079dbbf: 488d8dc8030000               lea rcx, [rbp + 0x3c8]
14079dbc6: 488d8580050000               lea rax, [rbp + 0x580]
14079dbcd: 4889442420                   mov qword ptr [rsp + 0x20], rax
14079dbd2: f30f100db6ff2402             movss xmm1, dword ptr [rip + 0x224ffb6]
14079dbda: f30f101d0e402e02             movss xmm3, dword ptr [rip + 0x22e400e]
14079dbe2: 0f28d1                       movaps xmm2, xmm1
14079dbe5: e8f6c43300                   call 0x140ada0e0
14079dbea: c685c003000002               mov byte ptr [rbp + 0x3c0], 2
14079dbf1: c685d004000000               mov byte ptr [rbp + 0x4d0], 0
14079dbf8: 488d8d80050000               lea rcx, [rbp + 0x580]
14079dbff: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079dc06: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079dc0d: e8feb12000                   call 0x1409a8e10
14079dc12: c6854706000001               mov byte ptr [rbp + 0x647], 1
14079dc19: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
14079dc20: e85bb32000                   call 0x1409a8f80
14079dc25: c6854706000000               mov byte ptr [rbp + 0x647], 0
14079dc2c: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
14079dc33: 488d9580050000               lea rdx, [rbp + 0x580]
14079dc3a: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079dc41: e83ab22000                   call 0x1409a8e80
14079dc46: 488d9d80050000               lea rbx, [rbp + 0x580]
14079dc4d: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079dc54: 4889d9                       mov rcx, rbx
14079dc57: 4531c0                       xor r8d, r8d
14079dc5a: e881b22000                   call 0x1409a8ee0
14079dc5f: 488d15dc412e02               lea rdx, [rip + 0x22e41dc]
14079dc66: 41b810000000                 mov r8d, 0x10
14079dc6c: 488b8d78040000               mov rcx, qword ptr [rbp + 0x478]
14079dc73: 4989d9                       mov r9, rbx
14079dc76: e8555a1800                   call 0x1409236d0
14079dc7b: 48898530030000               mov qword ptr [rbp + 0x330], rax
14079dc82: 48899538030000               mov qword ptr [rbp + 0x338], rdx
14079dc89: 488d8d30030000               lea rcx, [rbp + 0x330]
14079dc90: e87b021900                   call 0x14092df10
14079dc95: 3c03                         cmp al, 3
14079dc97: 0f852f040000                 jne 0x14079e0cc
14079dc9d: 8b858c030000                 mov eax, dword ptr [rbp + 0x38c]
14079dca3: 488b8d80030000               mov rcx, qword ptr [rbp + 0x380]
14079dcaa: 8901                         mov dword ptr [rcx], eax
14079dcac: 488b8570030000               mov rax, qword ptr [rbp + 0x370]
14079dcb3: 8b8d48050000                 mov ecx, dword ptr [rbp + 0x548]
14079dcb9: 894844                       mov dword ptr [rax + 0x44], ecx
14079dcbc: 488b8d40050000               mov rcx, qword ptr [rbp + 0x540]
14079dcc3: 4889483c                     mov qword ptr [rax + 0x3c], rcx
14079dcc7: 440f2895c0010000             movaps xmm10, xmmword ptr [rbp + 0x1c0]
14079dccf: f3440f108d40050000           movss xmm9, dword ptr [rbp + 0x540]
14079dcd8: f30f10bd44050000             movss xmm7, dword ptr [rbp + 0x544]
14079dce0: f3440f108548050000           movss xmm8, dword ptr [rbp + 0x548]
14079dce9: 488d8d80050000               lea rcx, [rbp + 0x580]
14079dcf0: 488d9530030000               lea rdx, [rbp + 0x330]
14079dcf7: e844091900                   call 0x14092e640
14079dcfc: 83bd8005000002               cmp dword ptr [rbp + 0x580], 2
14079dd03: 0f840c400000                 je 0x1407a1d15
14079dd09: 410f28ca                     movaps xmm1, xmm10
14079dd0d: 410f58ca                     addps xmm1, xmm10
14079dd11: 0fc6c9ff                     shufps xmm1, xmm1, 0xff
14079dd15: f30f1005e77b1802             movss xmm0, dword ptr [rip + 0x2187be7]
14079dd1d: 410f28d2                     movaps xmm2, xmm10
14079dd21: 0f59d0                       mulps xmm2, xmm0
14079dd24: 410f28da                     movaps xmm3, xmm10
14079dd28: 410fc6dac9                   shufps xmm3, xmm10, 0xc9
14079dd2d: 0f59d8                       mulps xmm3, xmm0
14079dd30: 0fc6d2d2                     shufps xmm2, xmm2, 0xd2
14079dd34: 0f5cda                       subps xmm3, xmm2
14079dd37: 0f59d9                       mulps xmm3, xmm1
14079dd3a: 410f28ca                     movaps xmm1, xmm10
14079dd3e: 410f59ca                     mulps xmm1, xmm10
14079dd42: 0f28e1                       movaps xmm4, xmm1
14079dd45: 0fc6e1ff                     shufps xmm4, xmm1, 0xff
14079dd49: f30f16d1                     movshdup xmm2, xmm1
14079dd4d: f30f58d1                     addss xmm2, xmm1
14079dd51: 0f12c9                       movhlps xmm1, xmm1
14079dd54: f30f58ca                     addss xmm1, xmm2
14079dd58: 0fc6c900                     shufps xmm1, xmm1, 0
14079dd5c: 0f5ce1                       subps xmm4, xmm1
14079dd5f: 0f280d5af32202               movaps xmm1, xmmword ptr [rip + 0x222f35a]
14079dd66: 0f59e1                       mulps xmm4, xmm1
14079dd69: 410f28d2                     movaps xmm2, xmm10
14079dd6d: 0f59d1                       mulps xmm2, xmm1
14079dd70: f30f16ea                     movshdup xmm5, xmm2
14079dd74: f30f58ea                     addss xmm5, xmm2
14079dd78: 0f12d2                       movhlps xmm2, xmm2
14079dd7b: f30f58d5                     addss xmm2, xmm5
14079dd7f: f30f58d2                     addss xmm2, xmm2
14079dd83: 0fc6d200                     shufps xmm2, xmm2, 0
14079dd87: 410f59d2                     mulps xmm2, xmm10
14079dd8b: 0f58d4                       addps xmm2, xmm4
14079dd8e: 0f58d3                       addps xmm2, xmm3
14079dd91: f30f16da                     movshdup xmm3, xmm2
14079dd95: f30f58fb                     addss xmm7, xmm3
14079dd99: f3440f1095a8050000           movss xmm10, dword ptr [rbp + 0x5a8]
14079dda2: f30f10adac050000             movss xmm5, dword ptr [rbp + 0x5ac]
14079ddaa: f30f109da4050000             movss xmm3, dword ptr [rbp + 0x5a4]
14079ddb2: 0f14eb                       unpcklps xmm5, xmm3
14079ddb5: f30f109d44050000             movss xmm3, dword ptr [rbp + 0x544]
14079ddbd: f3440f5cd3                   subss xmm10, xmm3
14079ddc2: 440f28df                     movaps xmm11, xmm7
14079ddc6: f3440f5cdb                   subss xmm11, xmm3
14079ddcb: f3450f59d3                   mulss xmm10, xmm11
14079ddd0: f3450f59db                   mulss xmm11, xmm11
14079ddd5: 450f14c1                     unpcklps xmm8, xmm9
14079ddd9: 0fc6d2e2                     shufps xmm2, xmm2, 0xe2
14079dddd: 410f58d0                     addps xmm2, xmm8
14079dde1: f3440f108540050000           movss xmm8, dword ptr [rbp + 0x540]
14079ddea: f30f10a548050000             movss xmm4, dword ptr [rbp + 0x548]
14079ddf2: 410f14e0                     unpcklps xmm4, xmm8
14079ddf6: 0f5cec                       subps xmm5, xmm4
14079ddf9: 440f28c2                     movaps xmm8, xmm2
14079ddfd: 440f5cc4                     subps xmm8, xmm4
14079de01: 410f59e8                     mulps xmm5, xmm8
14079de05: f3440f16cd                   movshdup xmm9, xmm5
14079de0a: f3450f58ca                   addss xmm9, xmm10
14079de0f: f3440f58cd                   addss xmm9, xmm5
14079de14: 450f59c0                     mulps xmm8, xmm8
14079de18: f3410f16e8                   movshdup xmm5, xmm8
14079de1d: f3410f58eb                   addss xmm5, xmm11
14079de22: f3410f58e8                   addss xmm5, xmm8
14079de27: f30f5f2d71dc2d02             maxss xmm5, dword ptr [rip + 0x22ddc71]
14079de2f: f3440f5ecd                   divss xmm9, xmm5
14079de34: f30f102dc87a1802             movss xmm5, dword ptr [rip + 0x2187ac8]
14079de3c: f3410f5ce9                   subss xmm5, xmm9
14079de41: 440f28c3                     movaps xmm8, xmm3
14079de45: f3440f59c5                   mulss xmm8, xmm5
14079de4a: f30f12ed                     movsldup xmm5, xmm5
14079de4e: 0f59ec                       mulps xmm5, xmm4
14079de51: f3410f59f9                   mulss xmm7, xmm9
14079de56: f3410f58f8                   addss xmm7, xmm8
14079de5b: f3450f12c1                   movsldup xmm8, xmm9
14079de60: 440f59c2                     mulps xmm8, xmm2
14079de64: 440f58c5                     addps xmm8, xmm5
14079de68: f30f5cfb                     subss xmm7, xmm3
14079de6c: 440f5cc4                     subps xmm8, xmm4
14079de70: f30f59ff                     mulss xmm7, xmm7
14079de74: 450f59c0                     mulps xmm8, xmm8
14079de78: f3410f16d0                   movshdup xmm2, xmm8
14079de7d: f30f58d7                     addss xmm2, xmm7
14079de81: f3410f58d0                   addss xmm2, xmm8
14079de86: f30f51d2                     sqrtss xmm2, xmm2
14079de8a: 8b8d8c030000                 mov ecx, dword ptr [rbp + 0x38c]
14079de90: 488b8580030000               mov rax, qword ptr [rbp + 0x380]
14079de97: 8908                         mov dword ptr [rax], ecx
14079de99: 488b8570030000               mov rax, qword ptr [rbp + 0x370]
14079dea0: f30f115038                   movss dword ptr [rax + 0x38], xmm2
14079dea5: 0f2895c0010000               movaps xmm2, xmmword ptr [rbp + 0x1c0]
14079deac: 0f28da                       movaps xmm3, xmm2
14079deaf: 0f59da                       mulps xmm3, xmm2
14079deb2: f30f16e3                     movshdup xmm4, xmm3
14079deb6: f30f58e3                     addss xmm4, xmm3
14079deba: 0f28eb                       movaps xmm5, xmm3
14079debd: 660f15eb                     unpckhpd xmm5, xmm3
14079dec1: f30f58ec                     addss xmm5, xmm4
14079dec5: 0fc6ed00                     shufps xmm5, xmm5, 0
14079dec9: 0fc6dbff                     shufps xmm3, xmm3, 0xff
14079decd: 0f5cdd                       subps xmm3, xmm5
14079ded0: 0f59d9                       mulps xmm3, xmm1
14079ded3: 0f59ca                       mulps xmm1, xmm2
14079ded6: f30f16e1                     movshdup xmm4, xmm1
14079deda: f30f58e1                     addss xmm4, xmm1
14079dede: 0f12c9                       movhlps xmm1, xmm1
14079dee1: f30f58cc                     addss xmm1, xmm4
14079dee5: f30f58c9                     addss xmm1, xmm1
14079dee9: 0fc6c900                     shufps xmm1, xmm1, 0
14079deed: 0f59ca                       mulps xmm1, xmm2
14079def0: 0f58cb                       addps xmm1, xmm3
14079def3: 0f28da                       movaps xmm3, xmm2
14079def6: 0f59d8                       mulps xmm3, xmm0
14079def9: 0f28e2                       movaps xmm4, xmm2
14079defc: 0fc6e2c9                     shufps xmm4, xmm2, 0xc9
14079df00: 0f59e0                       mulps xmm4, xmm0
14079df03: 0fc6dbd2                     shufps xmm3, xmm3, 0xd2
14079df07: 0f5ce3                       subps xmm4, xmm3
14079df0a: 0f58d2                       addps xmm2, xmm2
14079df0d: 0fc6d2ff                     shufps xmm2, xmm2, 0xff
14079df11: 0f59d4                       mulps xmm2, xmm4
14079df14: 0f58d1                       addps xmm2, xmm1
14079df17: 488b9580030000               mov rdx, qword ptr [rbp + 0x380]
14079df1e: 890a                         mov dword ptr [rdx], ecx
14079df20: 0f135048                     movlps qword ptr [rax + 0x48], xmm2
14079df24: 0f12d2                       movhlps xmm2, xmm2
14079df27: f30f115050                   movss dword ptr [rax + 0x50], xmm2
14079df2c: e80ff85a00                   call 0x140d4d740
14079df31: 4989c4                       mov r12, rax
14079df34: f60001                       test byte ptr [rax], 1
14079df37: 7413                         je 0x14079df4c
14079df39: 4c89e0                       mov rax, r12
14079df3c: 4883c008                     add rax, 8
14079df40: 4d8b742408                   mov r14, qword ptr [r12 + 8]
14079df45: 4d8b7c2410                   mov r15, qword ptr [r12 + 0x10]
14079df4a: eb24                         jmp 0x14079df70
14079df4c: e8af6e5c00                   call 0x140d64e00
14079df51: 4989c6                       mov r14, rax
14079df54: 4989d7                       mov r15, rdx
14079df57: 4c89e0                       mov rax, r12
14079df5a: 4883c008                     add rax, 8
14079df5e: 49c7042401000000             mov qword ptr [r12], 1
14079df66: 4d89742408                   mov qword ptr [r12 + 8], r14
14079df6b: 4989542410                   mov qword ptr [r12 + 0x10], rdx
14079df70: 498d4e01                     lea rcx, [r14 + 1]
14079df74: 488908                       mov qword ptr [rax], rcx
14079df77: 8b858c030000                 mov eax, dword ptr [rbp + 0x38c]
14079df7d: 488b8d80030000               mov rcx, qword ptr [rbp + 0x380]
14079df84: 8901                         mov dword ptr [rcx], eax
14079df86: 488bb570030000               mov rsi, qword ptr [rbp + 0x370]
14079df8d: 4c8d6608                     lea r12, [rsi + 8]
14079df91: 488b5610                     mov rdx, qword ptr [rsi + 0x10]
14079df95: 4885d2                       test rdx, rdx
14079df98: f3440f100d63791802           movss xmm9, dword ptr [rip + 0x2187963]
14079dfa1: 7427                         je 0x14079dfca
14079dfa3: 488d04d517000000             lea rax, [rdx*8 + 0x17]
14079dfab: 4883e0f0                     and rax, 0xfffffffffffffff0
14079dfaf: 4801c2                       add rdx, rax
14079dfb2: 4883c211                     add rdx, 0x11
14079dfb6: 7412                         je 0x14079dfca
14079dfb8: 498b0c24                     mov rcx, qword ptr [r12]
14079dfbc: 4829c1                       sub rcx, rax
14079dfbf: 41b810000000                 mov r8d, 0x10
14079dfc5: e8565ce7ff                   call 0x140613c20
14079dfca: 0f10055f3d2e02               movups xmm0, xmmword ptr [rip + 0x22e3d5f]
14079dfd1: 410f11442410                 movups xmmword ptr [r12 + 0x10], xmm0
14079dfd7: 0f1005423d2e02               movups xmm0, xmmword ptr [rip + 0x22e3d42]
14079dfde: 410f110424                   movups xmmword ptr [r12], xmm0
14079dfe3: 4c897628                     mov qword ptr [rsi + 0x28], r14
14079dfe7: 4c897e30                     mov qword ptr [rsi + 0x30], r15
14079dfeb: 488d8d80050000               lea rcx, [rbp + 0x580]
14079dff2: 488b95b8030000               mov rdx, qword ptr [rbp + 0x3b8]
14079dff9: 4189f8                       mov r8d, edi
14079dffc: e8bf081600                   call 0x1408fe8c0
14079e001: 488b8580050000               mov rax, qword ptr [rbp + 0x580]
14079e008: 48898558060000               mov qword ptr [rbp + 0x658], rax
14079e00f: 488b8588050000               mov rax, qword ptr [rbp + 0x588]
14079e016: 48898568060000               mov qword ptr [rbp + 0x668], rax
14079e01d: 4c8ba590050000               mov r12, qword ptr [rbp + 0x590]
14079e024: 4d85e4                       test r12, r12
14079e027: 747a                         je 0x14079e0a3
14079e029: 49c1e402                     shl r12, 2
14079e02d: 4531ed                       xor r13d, r13d
14079e030: 4c8d35713e2e02               lea r14, [rip + 0x22e3e71]
14079e037: eb10                         jmp 0x14079e049
14079e039: 0f1f8000000000               nop dword ptr [rax]
14079e040: 4983c504                     add r13, 4
14079e044: 4d39ec                       cmp r12, r13
14079e047: 745a                         je 0x14079e0a3
14079e049: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
14079e050: 468b3c28                     mov r15d, dword ptr [rax + r13]
14079e054: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
14079e05b: 4489fa                       mov edx, r15d
14079e05e: e8cde82700                   call 0x140a1c930
14079e063: 4885c0                       test rax, rax
14079e066: 74d8                         je 0x14079e040
14079e068: 8b8d8c030000                 mov ecx, dword ptr [rbp + 0x38c]
14079e06e: 488b9580030000               mov rdx, qword ptr [rbp + 0x380]
14079e075: 890a                         mov dword ptr [rdx], ecx
14079e077: 488bb570030000               mov rsi, qword ptr [rbp + 0x370]
14079e07e: 4883c030                     add rax, 0x30
14079e082: 4889c1                       mov rcx, rax
14079e085: 4c89f2                       mov rdx, r14
14079e088: e823b40900                   call 0x1408394b0
14079e08d: 4883c608                     add rsi, 8
14079e091: f30f105004                   movss xmm2, dword ptr [rax + 4]
14079e096: 4889f1                       mov rcx, rsi
14079e099: 4489fa                       mov edx, r15d
14079e09c: e86f92ac00                   call 0x141267310
14079e0a1: eb9d                         jmp 0x14079e040
14079e0a3: 488b9558060000               mov rdx, qword ptr [rbp + 0x658]
14079e0aa: 4885d2                       test rdx, rdx
14079e0ad: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
14079e0b4: 7416                         je 0x14079e0cc
14079e0b6: 48c1e202                     shl rdx, 2
14079e0ba: 41b804000000                 mov r8d, 4
14079e0c0: 488b8d68060000               mov rcx, qword ptr [rbp + 0x668]
14079e0c7: e8545be7ff                   call 0x140613c20
14079e0cc: 488db530030000               lea rsi, [rbp + 0x330]
14079e0d3: 4889f1                       mov rcx, rsi
14079e0d6: e835fe1800                   call 0x14092df10
14079e0db: 888568060000                 mov byte ptr [rbp + 0x668], al
14079e0e1: f20f1005173b2e02             movsd xmm0, qword ptr [rip + 0x22e3b17]
14079e0e9: f20f118580050000             movsd qword ptr [rbp + 0x580], xmm0
14079e0f1: c6858805000000               mov byte ptr [rbp + 0x588], 0
14079e0f8: 48895c2420                   mov qword ptr [rsp + 0x20], rbx
14079e0fd: 488d8d90040000               lea rcx, [rbp + 0x490]
14079e104: 4c8d8d70030000               lea r9, [rbp + 0x370]
14079e10b: 488b9598070000               mov rdx, qword ptr [rbp + 0x798]
14079e112: 4989f0                       mov r8, rsi
14079e115: e87688ad00                   call 0x141276990
14079e11a: f6859004000001               test byte ptr [rbp + 0x490], 1
14079e121: 0f8412090000                 je 0x14079ea39
14079e127: f30f10bda0040000             movss xmm7, dword ptr [rbp + 0x4a0]
14079e12f: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
14079e136: 488b95b8030000               mov rdx, qword ptr [rbp + 0x3b8]
14079e13d: 4189f8                       mov r8d, edi
14079e140: e87b071600                   call 0x1408fe8c0
14079e145: 8b858c030000                 mov eax, dword ptr [rbp + 0x38c]
14079e14b: 488b8d80030000               mov rcx, qword ptr [rbp + 0x380]
14079e152: 8901                         mov dword ptr [rcx], eax
14079e154: 488b8570030000               mov rax, qword ptr [rbp + 0x370]
14079e15b: c70001000000                 mov dword ptr [rax], 1
14079e161: f30f117804                   movss dword ptr [rax + 4], xmm7
14079e166: 0fb6059c6ae902               movzx eax, byte ptr [rip + 0x2e96a9c]
14079e16d: b918000000                   mov ecx, 0x18
14079e172: ba04000000                   mov edx, 4
14079e177: e8945ae7ff                   call 0x140613c10
14079e17c: 4885c0                       test rax, rax
14079e17f: 0f849c3b0000                 je 0x1407a1d21
14079e185: 488b8d70030000               mov rcx, qword ptr [rbp + 0x370]
14079e18c: f30f104144                   movss xmm0, dword ptr [rcx + 0x44]
14079e191: f30f104950                   movss xmm1, dword ptr [rcx + 0x50]
14079e196: f30f590df6641902             mulss xmm1, dword ptr [rip + 0x21964f6]
14079e19e: 0f28d0                       movaps xmm2, xmm0
14079e1a1: f30f5cd1                     subss xmm2, xmm1
14079e1a5: f30f58c8                     addss xmm1, xmm0
14079e1a9: f20f104148                   movsd xmm0, qword ptr [rcx + 0x48]
14079e1ae: 0f59055b3a2e02               mulps xmm0, xmmword ptr [rip + 0x22e3a5b]
14079e1b5: f20f10593c                   movsd xmm3, qword ptr [rcx + 0x3c]
14079e1ba: 0f28e3                       movaps xmm4, xmm3
14079e1bd: 0f5ce0                       subps xmm4, xmm0
14079e1c0: 0f1320                       movlps qword ptr [rax], xmm4
14079e1c3: f30f115008                   movss dword ptr [rax + 8], xmm2
14079e1c8: 0f58c3                       addps xmm0, xmm3
14079e1cb: 0f13400c                     movlps qword ptr [rax + 0xc], xmm0
14079e1cf: f30f114814                   movss dword ptr [rax + 0x14], xmm1
14079e1d4: 48c785c003000002000000       mov qword ptr [rbp + 0x3c0], 2
14079e1df: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
14079e1e6: 48c785d003000002000000       mov qword ptr [rbp + 0x3d0], 2
14079e1f1: c6855706000001               mov byte ptr [rbp + 0x657], 1
14079e1f8: 488d8d80050000               lea rcx, [rbp + 0x580]
14079e1ff: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079e206: e8656c3600                   call 0x140b04e70
14079e20b: c6855706000001               mov byte ptr [rbp + 0x657], 1
14079e212: 488d8d80050000               lea rcx, [rbp + 0x580]
14079e219: 488b9568050000               mov rdx, qword ptr [rbp + 0x568]
14079e220: e84b6d3600                   call 0x140b04f70
14079e225: 488b85d0040000               mov rax, qword ptr [rbp + 0x4d0]
14079e22c: 48898548060000               mov qword ptr [rbp + 0x648], rax
14079e233: 488b85d8040000               mov rax, qword ptr [rbp + 0x4d8]
14079e23a: 48898558060000               mov qword ptr [rbp + 0x658], rax
14079e241: 488b85e0040000               mov rax, qword ptr [rbp + 0x4e0]
14079e248: 4885c0                       test rax, rax
14079e24b: 0f8468020000                 je 0x14079e4b9
14079e251: 488bbd58060000               mov rdi, qword ptr [rbp + 0x658]
14079e258: 488d3487                     lea rsi, [rdi + rax*4]
14079e25c: 488b8590070000               mov rax, qword ptr [rbp + 0x790]
14079e263: 448b601c                     mov r12d, dword ptr [rax + 0x1c]
14079e267: 488b18                       mov rbx, qword ptr [rax]
14079e26a: 4c8b7010                     mov r14, qword ptr [rax + 0x10]
14079e26e: 66450f76c0                   pcmpeqd xmm8, xmm8
14079e273: f3440f100d281a2e02           movss xmm9, dword ptr [rip + 0x22e1a28]
14079e27c: f3440f10150f641902           movss xmm10, dword ptr [rip + 0x219640f]
14079e285: e9c6010000                   jmp 0x14079e450
14079e28a: 660f1f440000                 nop word ptr [rax + rax]
14079e290: f3420f6f0c01                 movdqu xmm1, xmmword ptr [rcx + r8]
14079e296: 660f6fd1                     movdqa xmm2, xmm1
14079e29a: 660f74d0                     pcmpeqb xmm2, xmm0
14079e29e: 66440fd7ca                   pmovmskb r9d, xmm2
14079e2a3: 4585c9                       test r9d, r9d
14079e2a6: 7438                         je 0x14079e2e0
14079e2a8: f3450fbcd1                   tzcnt r10d, r9d
14079e2ad: 4d01c2                       add r10, r8
14079e2b0: 4921d2                       and r10, rdx
14079e2b3: 4e8d1cd500000000             lea r11, [r10*8]
14079e2bb: 4989cd                       mov r13, rcx
14079e2be: 4d29dd                       sub r13, r11
14079e2c1: 453b7df8                     cmp r15d, dword ptr [r13 - 8]
14079e2c5: 7449                         je 0x14079e310
14079e2c7: 458d51ff                     lea r10d, [r9 - 1]
14079e2cb: 664521ca                     and r10w, r9w
14079e2cf: 4589d1                       mov r9d, r10d
14079e2d2: 75d4                         jne 0x14079e2a8
14079e2d4: 6666662e0f1f840000000000     nop word ptr cs:[rax + rax]
14079e2e0: 66410f74c8                   pcmpeqb xmm1, xmm8
14079e2e5: 66440fd7c9                   pmovmskb r9d, xmm1
14079e2ea: 4585c9                       test r9d, r9d
14079e2ed: 0f85bd010000                 jne 0x14079e4b0
14079e2f3: 4901c0                       add r8, rax
14079e2f6: 4983c010                     add r8, 0x10
14079e2fa: 4883c010                     add rax, 0x10
14079e2fe: 4921d0                       and r8, rdx
14079e301: eb8d                         jmp 0x14079e290
14079e303: 666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
14079e310: 49f7da                       neg r10
14079e313: f3460f105cd1fc               movss xmm11, dword ptr [rcx + r10*8 - 4]
14079e31a: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
14079e321: 4489fa                       mov edx, r15d
14079e324: e8b7e02700                   call 0x140a1c3e0
14079e329: f3440f58df                   addss xmm11, xmm7
14079e32e: 410f28c1                     movaps xmm0, xmm9
14079e332: f3410f5fc3                   maxss xmm0, xmm11
14079e337: 410f28ca                     movaps xmm1, xmm10
14079e33b: f30f5dc8                     minss xmm1, xmm0
14079e33f: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079e346: 48899588050000               mov qword ptr [rbp + 0x588], rdx
14079e34d: 48c7859005000003000000       mov qword ptr [rbp + 0x590], 3
14079e358: c785d005000000000000         mov dword ptr [rbp + 0x5d0], 0
14079e362: c785d805000001000000         mov dword ptr [rbp + 0x5d8], 1
14079e36c: f30f118ddc050000             movss dword ptr [rbp + 0x5dc], xmm1
14079e374: c685e005000000               mov byte ptr [rbp + 0x5e0], 0
14079e37b: c685e205000000               mov byte ptr [rbp + 0x5e2], 0
14079e382: 66c785e40500000240           mov word ptr [rbp + 0x5e4], 0x4002
14079e38b: 4489bde8050000               mov dword ptr [rbp + 0x5e8], r15d
14079e392: c685ec0500003f               mov byte ptr [rbp + 0x5ec], 0x3f
14079e399: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
14079e3a0: 488b9520020000               mov rdx, qword ptr [rbp + 0x220]
14079e3a7: 4589f8                       mov r8d, r15d
14079e3aa: 4c8d0d0f3b2e02               lea r9, [rip + 0x22e3b0f]
14079e3b1: e86ac62700                   call 0x140a1aa20
14079e3b6: 80bd6806000002               cmp byte ptr [rbp + 0x668], 2
14079e3bd: 0f93c2                       setae dl
14079e3c0: 488d8dd0030000               lea rcx, [rbp + 0x3d0]
14079e3c7: 8b4108                       mov eax, dword ptr [rcx + 8]
14079e3ca: 898548020000                 mov dword ptr [rbp + 0x248], eax
14079e3d0: 488b01                       mov rax, qword ptr [rcx]
14079e3d3: 48898540020000               mov qword ptr [rbp + 0x240], rax
14079e3da: c6854c02000000               mov byte ptr [rbp + 0x24c], 0
14079e3e1: 488b8588020000               mov rax, qword ptr [rbp + 0x288]
14079e3e8: 8b8d38050000                 mov ecx, dword ptr [rbp + 0x538]
14079e3ee: 8908                         mov dword ptr [rax], ecx
14079e3f0: 488b8598020000               mov rax, qword ptr [rbp + 0x298]
14079e3f7: 8b8d3c050000                 mov ecx, dword ptr [rbp + 0x53c]
14079e3fd: 8908                         mov dword ptr [rax], ecx
14079e3ff: 458926                       mov dword ptr [r14], r12d
14079e402: 488b85a0020000               mov rax, qword ptr [rbp + 0x2a0]
14079e409: 4889442438                   mov qword ptr [rsp + 0x38], rax
14079e40e: 488d45c0                     lea rax, [rbp - 0x40]
14079e412: 4889442430                   mov qword ptr [rsp + 0x30], rax
14079e417: 48895c2428                   mov qword ptr [rsp + 0x28], rbx
14079e41c: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
14079e423: 4889442420                   mov qword ptr [rsp + 0x20], rax
14079e428: 488d8d80050000               lea rcx, [rbp + 0x580]
14079e42f: 4c8d8540020000               lea r8, [rbp + 0x240]
14079e436: 4c8b8d38060000               mov r9, qword ptr [rbp + 0x638]
14079e43d: e8ee89ad00                   call 0x141276e30
14079e442: eb6c                         jmp 0x14079e4b0
14079e444: 6666662e0f1f840000000000     nop word ptr cs:[rax + rax]
14079e450: 448b3f                       mov r15d, dword ptr [rdi]
14079e453: 4489bdf4010000               mov dword ptr [rbp + 0x1f4], r15d
14079e45a: 4c8bad70030000               mov r13, qword ptr [rbp + 0x370]
14079e461: 49837d2000                   cmp qword ptr [r13 + 0x20], 0
14079e466: 7448                         je 0x14079e4b0
14079e468: 498d4d28                     lea rcx, [r13 + 0x28]
14079e46c: 488d95f4010000               lea rdx, [rbp + 0x1f4]
14079e473: e8582ae1ff                   call 0x1405b0ed0
14079e478: 498b4d08                     mov rcx, qword ptr [r13 + 8]
14079e47c: 498b5510                     mov rdx, qword ptr [r13 + 0x10]
14079e480: 4989d0                       mov r8, rdx
14079e483: 4921c0                       and r8, rax
14079e486: 48c1e839                     shr rax, 0x39
14079e48a: 660f6ec0                     movd xmm0, eax
14079e48e: 660f60c0                     punpcklbw xmm0, xmm0
14079e492: f20f70c000                   pshuflw xmm0, xmm0, 0
14079e497: 660f70c000                   pshufd xmm0, xmm0, 0
14079e49c: 31c0                         xor eax, eax
14079e49e: e9edfdffff                   jmp 0x14079e290
14079e4a3: 666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
14079e4b0: 4883c704                     add rdi, 4
14079e4b4: 4839f7                       cmp rdi, rsi
14079e4b7: 7597                         jne 0x14079e450
14079e4b9: 488b9548060000               mov rdx, qword ptr [rbp + 0x648]
14079e4c0: 4885d2                       test rdx, rdx
14079e4c3: 7416                         je 0x14079e4db
14079e4c5: 48c1e202                     shl rdx, 2
14079e4c9: 41b804000000                 mov r8d, 4
14079e4cf: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
14079e4d6: e84557e7ff                   call 0x140613c20
14079e4db: f3440f100d20741802           movss xmm9, dword ptr [rip + 0x2187420]
14079e4e4: 488db580050000               lea rsi, [rbp + 0x580]
14079e4eb: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
14079e4f2: f3410f104660                 movss xmm0, dword ptr [r14 + 0x60]
14079e4f8: 0f2ec6                       ucomiss xmm0, xmm6
14079e4fb: 0f866a050000                 jbe 0x14079ea6b
14079e501: 488b85b8070000               mov rax, qword ptr [rbp + 0x7b8]
14079e508: 488b10                       mov rdx, qword ptr [rax]
14079e50b: 4889f1                       mov rcx, rsi
14079e50e: e8dd884f00                   call 0x140c96df0
14079e513: f30f108d80050000             movss xmm1, dword ptr [rbp + 0x580]
14079e51b: f30f108588050000             movss xmm0, dword ptr [rbp + 0x588]
14079e523: 0f28d1                       movaps xmm2, xmm1
14079e526: f30f59d1                     mulss xmm2, xmm1
14079e52a: 0f28d8                       movaps xmm3, xmm0
14079e52d: f30f59d8                     mulss xmm3, xmm0
14079e531: f30f58da                     addss xmm3, xmm2
14079e535: f30f51db                     sqrtss xmm3, xmm3
14079e539: 410f28d1                     movaps xmm2, xmm9
14079e53d: f30f5ed3                     divss xmm2, xmm3
14079e541: f30f59ca                     mulss xmm1, xmm2
14079e545: f30f59d0                     mulss xmm2, xmm0
14079e549: 488d8d80050000               lea rcx, [rbp + 0x580]
14079e550: e87b2a4f00                   call 0x140c90fd0
14079e555: 0f289580050000               movaps xmm2, xmmword ptr [rbp + 0x580]
14079e55c: 0f28da                       movaps xmm3, xmm2
14079e55f: 0fc6da00                     shufps xmm3, xmm2, 0
14079e563: 0f28e2                       movaps xmm4, xmm2
14079e566: 0fc6e255                     shufps xmm4, xmm2, 0x55
14079e56a: 0f28c2                       movaps xmm0, xmm2
14079e56d: 0fc6c2aa                     shufps xmm0, xmm2, 0xaa
14079e571: 0fc6d2ff                     shufps xmm2, xmm2, 0xff
14079e575: 0f282dc4ed2202               movaps xmm5, xmmword ptr [rip + 0x222edc4]
14079e57c: 0f59d5                       mulps xmm2, xmm5
14079e57f: 0f2835caed2202               movaps xmm6, xmmword ptr [rip + 0x222edca]
14079e586: 0f59de                       mulps xmm3, xmm6
14079e589: 0f280de0eb2202               movaps xmm1, xmmword ptr [rip + 0x222ebe0]
14079e590: 0f59d9                       mulps xmm3, xmm1
14079e593: 0f58da                       addps xmm3, xmm2
14079e596: 0f59e5                       mulps xmm4, xmm5
14079e599: 0f2815e0eb2202               movaps xmm2, xmmword ptr [rip + 0x222ebe0]
14079e5a0: 0f59e2                       mulps xmm4, xmm2
14079e5a3: 0f59c6                       mulps xmm0, xmm6
14079e5a6: 0f282de3eb2202               movaps xmm5, xmmword ptr [rip + 0x222ebe3]
14079e5ad: 0f59c5                       mulps xmm0, xmm5
14079e5b0: 0f58c4                       addps xmm0, xmm4
14079e5b3: 0f58c3                       addps xmm0, xmm3
14079e5b6: 0f28d8                       movaps xmm3, xmm0
14079e5b9: 0fc6d800                     shufps xmm3, xmm0, 0
14079e5bd: 0f28e0                       movaps xmm4, xmm0
14079e5c0: 0fc6e055                     shufps xmm4, xmm0, 0x55
14079e5c4: 0f28f0                       movaps xmm6, xmm0
14079e5c7: 0fc6c0ff                     shufps xmm0, xmm0, 0xff
14079e5cb: 0f59054e362e02               mulps xmm0, xmmword ptr [rip + 0x22e364e]
14079e5d2: 0f591d57362e02               mulps xmm3, xmmword ptr [rip + 0x22e3657]
14079e5d9: 0fc6f6aa                     shufps xmm6, xmm6, 0xaa
14079e5dd: 0f59d9                       mulps xmm3, xmm1
14079e5e0: 0f592559362e02               mulps xmm4, xmmword ptr [rip + 0x22e3659]
14079e5e7: 0f58d8                       addps xmm3, xmm0
14079e5ea: 0f59e2                       mulps xmm4, xmm2
14079e5ed: 0f59355c362e02               mulps xmm6, xmmword ptr [rip + 0x22e365c]
14079e5f4: 0f59f5                       mulps xmm6, xmm5
14079e5f7: 0f58f4                       addps xmm6, xmm4
14079e5fa: 0f58f3                       addps xmm6, xmm3
14079e5fd: 0f29b530010000               movaps xmmword ptr [rbp + 0x130], xmm6
14079e604: 498b4664                     mov rax, qword ptr [r14 + 0x64]
14079e608: 488985a0030000               mov qword ptr [rbp + 0x3a0], rax
14079e60f: 418b466c                     mov eax, dword ptr [r14 + 0x6c]
14079e613: 8985a8030000                 mov dword ptr [rbp + 0x3a8], eax
14079e619: f3410f104660                 movss xmm0, dword ptr [r14 + 0x60]
14079e61f: f30f5905a9721802             mulss xmm0, dword ptr [rip + 0x21872a9]
14079e627: f30f118548010000             movss dword ptr [rbp + 0x148], xmm0
14079e62f: c685c003000000               mov byte ptr [rbp + 0x3c0], 0
14079e636: 488d05e9372e02               lea rax, [rip + 0x22e37e9]
14079e63d: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
14079e644: 48c785d00300000a000000       mov qword ptr [rbp + 0x3d0], 0xa
14079e64f: c685d803000001               mov byte ptr [rbp + 0x3d8], 1
14079e656: 488d05d3372e02               lea rax, [rip + 0x22e37d3]
14079e65d: 488985e0030000               mov qword ptr [rbp + 0x3e0], rax
14079e664: 48c785e803000012000000       mov qword ptr [rbp + 0x3e8], 0x12
14079e66f: 48c785f003000000000000       mov qword ptr [rbp + 0x3f0], 0
14079e67a: 31c9                         xor ecx, ecx
14079e67c: 488dbdd0040000               lea rdi, [rbp + 0x4d0]
14079e683: eb15                         jmp 0x14079e69a
14079e685: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
14079e690: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
14079e697: 48ffc1                       inc rcx
14079e69a: 4883f902                     cmp rcx, 2
14079e69e: 0f8421020000                 je 0x14079e8c5
14079e6a4: 488d0449                     lea rax, [rcx + rcx*2]
14079e6a8: 0fb69cc5c0030000             movzx ebx, byte ptr [rbp + rax*8 + 0x3c0]
14079e6b0: 80fb02                       cmp bl, 2
14079e6b3: 0f840c020000                 je 0x14079e8c5
14079e6b9: 48898d58060000               mov qword ptr [rbp + 0x658], rcx
14079e6c0: 488d04c5c0030000             lea rax, [rax*8 + 0x3c0]
14079e6c8: 4801e8                       add rax, rbp
14079e6cb: 0f104008                     movups xmm0, xmmword ptr [rax + 8]
14079e6cf: 0f118590040000               movups xmmword ptr [rbp + 0x490], xmm0
14079e6d6: 488d85a0030000               lea rax, [rbp + 0x3a0]
14079e6dd: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079e6e4: 488d0515d35a00               lea rax, [rip + 0x5ad315]
14079e6eb: 48898588050000               mov qword ptr [rbp + 0x588], rax
14079e6f2: 488d8590040000               lea rax, [rbp + 0x490]
14079e6f9: 48898590050000               mov qword ptr [rbp + 0x590], rax
14079e700: 488d059951d9ff               lea rax, [rip - 0x26ae67]
14079e707: 48898598050000               mov qword ptr [rbp + 0x598], rax
14079e70e: 488d8530010000               lea rax, [rbp + 0x130]
14079e715: 488985a0050000               mov qword ptr [rbp + 0x5a0], rax
14079e71c: 488d056dc85a00               lea rax, [rip + 0x5ac86d]
14079e723: 488985a8050000               mov qword ptr [rbp + 0x5a8], rax
14079e72a: 488d8548010000               lea rax, [rbp + 0x148]
14079e731: 488985b0050000               mov qword ptr [rbp + 0x5b0], rax
14079e738: 488d05d1e15e00               lea rax, [rip + 0x5ee1d1]
14079e73f: 488985b8050000               mov qword ptr [rbp + 0x5b8], rax
14079e746: 488d05e33a2e02               lea rax, [rip + 0x22e3ae3]
14079e74d: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
14079e754: 48c785d804000004000000       mov qword ptr [rbp + 0x4d8], 4
14079e75f: 4c8dad80050000               lea r13, [rbp + 0x580]
14079e766: 4c89ade0040000               mov qword ptr [rbp + 0x4e0], r13
14079e76d: 48c785e804000004000000       mov qword ptr [rbp + 0x4e8], 4
14079e778: 48c785f004000000000000       mov qword ptr [rbp + 0x4f0], 0
14079e783: 488d8d90000000               lea rcx, [rbp + 0x90]
14079e78a: 4889fe                       mov rsi, rdi
14079e78d: 4889fa                       mov rdx, rdi
14079e790: e8fbb75d00                   call 0x140d79f90
14079e795: 488b8590000000               mov rax, qword ptr [rbp + 0x90]
14079e79c: 48898568060000               mov qword ptr [rbp + 0x668], rax
14079e7a3: 4c8bb598000000               mov r14, qword ptr [rbp + 0x98]
14079e7aa: 4c8bbda0000000               mov r15, qword ptr [rbp + 0xa0]
14079e7b1: 4c8ba590040000               mov r12, qword ptr [rbp + 0x490]
14079e7b8: 488bbd98040000               mov rdi, qword ptr [rbp + 0x498]
14079e7bf: f30f10b548010000             movss xmm6, dword ptr [rbp + 0x148]
14079e7c7: 488b85a0030000               mov rax, qword ptr [rbp + 0x3a0]
14079e7ce: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079e7d5: 8b85a8030000                 mov eax, dword ptr [rbp + 0x3a8]
14079e7db: 898588050000                 mov dword ptr [rbp + 0x588], eax
14079e7e1: 0f28bd30010000               movaps xmm7, xmmword ptr [rbp + 0x130]
14079e7e8: 0fb6051a64e902               movzx eax, byte ptr [rip + 0x2e9641a]
14079e7ef: b950000000                   mov ecx, 0x50
14079e7f4: ba10000000                   mov edx, 0x10
14079e7f9: e81254e7ff                   call 0x140613c10
14079e7fe: 4885c0                       test rax, rax
14079e801: 4c89b548060000               mov qword ptr [rbp + 0x648], r14
14079e808: 0f8484340000                 je 0x1407a1c92
14079e80e: 48c70001000000               mov qword ptr [rax], 1
14079e815: 48c7400801000000             mov qword ptr [rax + 8], 1
14079e81d: 4c896010                     mov qword ptr [rax + 0x10], r12
14079e821: 48897818                     mov qword ptr [rax + 0x18], rdi
14079e825: 0f297820                     movaps xmmword ptr [rax + 0x20], xmm7
14079e829: f30f117030                   movss dword ptr [rax + 0x30], xmm6
14079e82e: 488b8d80050000               mov rcx, qword ptr [rbp + 0x580]
14079e835: 48894834                     mov qword ptr [rax + 0x34], rcx
14079e839: 8b8d88050000                 mov ecx, dword ptr [rbp + 0x588]
14079e83f: 89483c                       mov dword ptr [rax + 0x3c], ecx
14079e842: 885840                       mov byte ptr [rax + 0x40], bl
14079e845: 488d0d243a2e02               lea rcx, [rip + 0x22e3a24]
14079e84c: 48894c2420                   mov qword ptr [rsp + 0x20], rcx
14079e851: 4c89e9                       mov rcx, r13
14079e854: 4c89f2                       mov rdx, r14
14079e857: 4d89f8                       mov r8, r15
14079e85a: 4989c1                       mov r9, rax
14079e85d: e84ef00f00                   call 0x14089d8b0
14079e862: 4889f7                       mov rdi, rsi
14079e865: 488b85a0050000               mov rax, qword ptr [rbp + 0x5a0]
14079e86c: 488985f0040000               mov qword ptr [rbp + 0x4f0], rax
14079e873: 0f108580050000               movups xmm0, xmmword ptr [rbp + 0x580]
14079e87a: 0f108d90050000               movups xmm1, xmmword ptr [rbp + 0x590]
14079e881: 0f298de0040000               movaps xmmword ptr [rbp + 0x4e0], xmm1
14079e888: 0f2985d0040000               movaps xmmword ptr [rbp + 0x4d0], xmm0
14079e88f: 488b8d68050000               mov rcx, qword ptr [rbp + 0x568]
14079e896: 4889f2                       mov rdx, rsi
14079e899: e8f2e63800                   call 0x140b2cf90
14079e89e: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
14079e8a5: 4885d2                       test rdx, rdx
14079e8a8: 0f84e2fdffff                 je 0x14079e690
14079e8ae: 41b801000000                 mov r8d, 1
14079e8b4: 488b8d48060000               mov rcx, qword ptr [rbp + 0x648]
14079e8bb: e86053e7ff                   call 0x140613c20
14079e8c0: e9cbfdffff                   jmp 0x14079e690
14079e8c5: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
14079e8cc: 41837e5801                   cmp dword ptr [r14 + 0x58], 1
14079e8d1: 488db580050000               lea rsi, [rbp + 0x580]
14079e8d8: 0f858d010000                 jne 0x14079ea6b
14079e8de: 0f57c0                       xorps xmm0, xmm0
14079e8e1: f30f5885a8030000             addss xmm0, dword ptr [rbp + 0x3a8]
14079e8e9: f20f108da0030000             movsd xmm1, qword ptr [rbp + 0x3a0]
14079e8f1: 0f580d68332e02               addps xmm1, xmmword ptr [rip + 0x22e3368]
14079e8f8: 418b7e5c                     mov edi, dword ptr [r14 + 0x5c]
14079e8fc: 0f138d90050000               movlps qword ptr [rbp + 0x590], xmm1
14079e903: f30f118598050000             movss dword ptr [rbp + 0x598], xmm0
14079e90b: 0f288530010000               movaps xmm0, xmmword ptr [rbp + 0x130]
14079e912: 0f298580050000               movaps xmmword ptr [rbp + 0x580], xmm0
14079e919: 48b80000803f0000803f         movabs rax, 0x3f8000003f800000
14079e923: 4889859c050000               mov qword ptr [rbp + 0x59c], rax
14079e92a: c785a40500000000803f         mov dword ptr [rbp + 0x5a4], 0x3f800000
14079e934: 488d8dc8030000               lea rcx, [rbp + 0x3c8]
14079e93b: 488d8580050000               lea rax, [rbp + 0x580]
14079e942: 4889442420                   mov qword ptr [rsp + 0x20], rax
14079e947: f30f100d41f22402             movss xmm1, dword ptr [rip + 0x224f241]
14079e94f: f30f101d99322e02             movss xmm3, dword ptr [rip + 0x22e3299]
14079e957: 0f28d1                       movaps xmm2, xmm1
14079e95a: e881b73300                   call 0x140ada0e0
14079e95f: c685c003000002               mov byte ptr [rbp + 0x3c0], 2
14079e966: c685d004000000               mov byte ptr [rbp + 0x4d0], 0
14079e96d: 488d8d80050000               lea rcx, [rbp + 0x580]
14079e974: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079e97b: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079e982: e889a42000                   call 0x1409a8e10
14079e987: c6854606000001               mov byte ptr [rbp + 0x646], 1
14079e98e: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
14079e995: e8e6a52000                   call 0x1409a8f80
14079e99a: c6854606000000               mov byte ptr [rbp + 0x646], 0
14079e9a1: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
14079e9a8: 488d9580050000               lea rdx, [rbp + 0x580]
14079e9af: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079e9b6: e8c5a42000                   call 0x1409a8e80
14079e9bb: 488d9d80050000               lea rbx, [rbp + 0x580]
14079e9c2: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079e9c9: 4889d9                       mov rcx, rbx
14079e9cc: 4531c0                       xor r8d, r8d
14079e9cf: e80ca52000                   call 0x1409a8ee0
14079e9d4: 488d15fd342e02               lea rdx, [rip + 0x22e34fd]
14079e9db: 41b811000000                 mov r8d, 0x11
14079e9e1: 488b8d78040000               mov rcx, qword ptr [rbp + 0x478]
14079e9e8: 4989d9                       mov r9, rbx
14079e9eb: e8e04c1800                   call 0x1409236d0
14079e9f0: 48898530030000               mov qword ptr [rbp + 0x330], rax
14079e9f7: 48899538030000               mov qword ptr [rbp + 0x338], rdx
14079e9fe: 488d8d30030000               lea rcx, [rbp + 0x330]
14079ea05: e806f51800                   call 0x14092df10
14079ea0a: 3c03                         cmp al, 3
14079ea0c: 0f8590060000                 jne 0x14079f0a2
14079ea12: e829ed5a00                   call 0x140d4d740
14079ea17: 4989c4                       mov r12, rax
14079ea1a: f60001                       test byte ptr [rax], 1
14079ea1d: 0f8483040000                 je 0x14079eea6
14079ea23: 4c89e0                       mov rax, r12
14079ea26: 4883c008                     add rax, 8
14079ea2a: 4d8b742408                   mov r14, qword ptr [r12 + 8]
14079ea2f: 4d8b7c2410                   mov r15, qword ptr [r12 + 0x10]
14079ea34: e991040000                   jmp 0x14079eeca
14079ea39: 8b858c030000                 mov eax, dword ptr [rbp + 0x38c]
14079ea3f: 488b8d80030000               mov rcx, qword ptr [rbp + 0x380]
14079ea46: 8901                         mov dword ptr [rcx], eax
14079ea48: 488b8570030000               mov rax, qword ptr [rbp + 0x370]
14079ea4f: c70000000000                 mov dword ptr [rax], 0
14079ea55: 488db580050000               lea rsi, [rbp + 0x580]
14079ea5c: f3410f104660                 movss xmm0, dword ptr [r14 + 0x60]
14079ea62: 0f2ec6                       ucomiss xmm0, xmm6
14079ea65: 0f8796faffff                 ja 0x14079e501
14079ea6b: f3410f108690000000           movss xmm0, dword ptr [r14 + 0x90]
14079ea74: 0f57f6                       xorps xmm6, xmm6
14079ea77: 0f2ec6                       ucomiss xmm0, xmm6
14079ea7a: 0f86940a0000                 jbe 0x14079f514
14079ea80: 488b85b8070000               mov rax, qword ptr [rbp + 0x7b8]
14079ea87: 488b10                       mov rdx, qword ptr [rax]
14079ea8a: 488d8d80050000               lea rcx, [rbp + 0x580]
14079ea91: e85a834f00                   call 0x140c96df0
14079ea96: f30f108d80050000             movss xmm1, dword ptr [rbp + 0x580]
14079ea9e: f30f108588050000             movss xmm0, dword ptr [rbp + 0x588]
14079eaa6: 0f28d1                       movaps xmm2, xmm1
14079eaa9: f30f59d1                     mulss xmm2, xmm1
14079eaad: 0f28d8                       movaps xmm3, xmm0
14079eab0: f30f59d8                     mulss xmm3, xmm0
14079eab4: f30f58da                     addss xmm3, xmm2
14079eab8: f30f51db                     sqrtss xmm3, xmm3
14079eabc: 410f28d1                     movaps xmm2, xmm9
14079eac0: f30f5ed3                     divss xmm2, xmm3
14079eac4: f30f59ca                     mulss xmm1, xmm2
14079eac8: f30f59d0                     mulss xmm2, xmm0
14079eacc: 488db5c0000000               lea rsi, [rbp + 0xc0]
14079ead3: 4889f1                       mov rcx, rsi
14079ead6: e8f5244f00                   call 0x140c90fd0
14079eadb: 418b869c000000               mov eax, dword ptr [r14 + 0x9c]
14079eae2: 8985b0020000                 mov dword ptr [rbp + 0x2b0], eax
14079eae8: 498b8694000000               mov rax, qword ptr [r14 + 0x94]
14079eaef: 488985a8020000               mov qword ptr [rbp + 0x2a8], rax
14079eaf6: f3410f108690000000           movss xmm0, dword ptr [r14 + 0x90]
14079eaff: f30f590561e52202             mulss xmm0, dword ptr [rip + 0x222e561]
14079eb07: f30f118590040000             movss dword ptr [rbp + 0x490], xmm0
14079eb0f: 488d85a8020000               lea rax, [rbp + 0x2a8]
14079eb16: 488985c0030000               mov qword ptr [rbp + 0x3c0], rax
14079eb1d: 488d05dcce5a00               lea rax, [rip + 0x5acedc]
14079eb24: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
14079eb2b: 4889b5d0030000               mov qword ptr [rbp + 0x3d0], rsi
14079eb32: 488d0557c45a00               lea rax, [rip + 0x5ac457]
14079eb39: 488985d8030000               mov qword ptr [rbp + 0x3d8], rax
14079eb40: 488d8590040000               lea rax, [rbp + 0x490]
14079eb47: 488985e0030000               mov qword ptr [rbp + 0x3e0], rax
14079eb4e: 488d05bbdd5e00               lea rax, [rip + 0x5eddbb]
14079eb55: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
14079eb5c: 488d05ed332e02               lea rax, [rip + 0x22e33ed]
14079eb63: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079eb6a: 48c7858805000003000000       mov qword ptr [rbp + 0x588], 3
14079eb75: 488d85c0030000               lea rax, [rbp + 0x3c0]
14079eb7c: 48898590050000               mov qword ptr [rbp + 0x590], rax
14079eb83: 48c7859805000003000000       mov qword ptr [rbp + 0x598], 3
14079eb8e: 48c785a005000000000000       mov qword ptr [rbp + 0x5a0], 0
14079eb99: 488d4d78                     lea rcx, [rbp + 0x78]
14079eb9d: 488d9580050000               lea rdx, [rbp + 0x580]
14079eba4: e8e7b35d00                   call 0x140d79f90
14079eba9: 488b4578                     mov rax, qword ptr [rbp + 0x78]
14079ebad: 48898568060000               mov qword ptr [rbp + 0x668], rax
14079ebb4: 488bb580000000               mov rsi, qword ptr [rbp + 0x80]
14079ebbb: 488bbd88000000               mov rdi, qword ptr [rbp + 0x88]
14079ebc2: f30f10bd90040000             movss xmm7, dword ptr [rbp + 0x490]
14079ebca: 488b85a8020000               mov rax, qword ptr [rbp + 0x2a8]
14079ebd1: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079ebd8: 8b85b0020000                 mov eax, dword ptr [rbp + 0x2b0]
14079ebde: 898588050000                 mov dword ptr [rbp + 0x588], eax
14079ebe4: 440f2885c0000000             movaps xmm8, xmmword ptr [rbp + 0xc0]
14079ebec: 0fb6051660e902               movzx eax, byte ptr [rip + 0x2e96016]
14079ebf3: b930000000                   mov ecx, 0x30
14079ebf8: ba10000000                   mov edx, 0x10
14079ebfd: e80e50e7ff                   call 0x140613c10
14079ec02: 4885c0                       test rax, rax
14079ec05: 4889b558060000               mov qword ptr [rbp + 0x658], rsi
14079ec0c: 0f84d2300000                 je 0x1407a1ce4
14079ec12: 48c70001000000               mov qword ptr [rax], 1
14079ec19: 48c7400801000000             mov qword ptr [rax + 8], 1
14079ec21: 440f294010                   movaps xmmword ptr [rax + 0x10], xmm8
14079ec26: f30f117820                   movss dword ptr [rax + 0x20], xmm7
14079ec2b: 488b8d80050000               mov rcx, qword ptr [rbp + 0x580]
14079ec32: 48894824                     mov qword ptr [rax + 0x24], rcx
14079ec36: 8b8d88050000                 mov ecx, dword ptr [rbp + 0x588]
14079ec3c: 89482c                       mov dword ptr [rax + 0x2c], ecx
14079ec3f: 488d0d3a332e02               lea rcx, [rip + 0x22e333a]
14079ec46: 48894c2420                   mov qword ptr [rsp + 0x20], rcx
14079ec4b: 488d9d80050000               lea rbx, [rbp + 0x580]
14079ec52: 4889d9                       mov rcx, rbx
14079ec55: 4889f2                       mov rdx, rsi
14079ec58: 4989f8                       mov r8, rdi
14079ec5b: 4989c1                       mov r9, rax
14079ec5e: e84dec0f00                   call 0x14089d8b0
14079ec63: 488b85a0050000               mov rax, qword ptr [rbp + 0x5a0]
14079ec6a: 488985e0030000               mov qword ptr [rbp + 0x3e0], rax
14079ec71: 0f108580050000               movups xmm0, xmmword ptr [rbp + 0x580]
14079ec78: f30f6f8d90050000             movdqu xmm1, xmmword ptr [rbp + 0x590]
14079ec80: 660f7f8dd0030000             movdqa xmmword ptr [rbp + 0x3d0], xmm1
14079ec88: 0f2985c0030000               movaps xmmword ptr [rbp + 0x3c0], xmm0
14079ec8f: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079ec96: 488b8d68050000               mov rcx, qword ptr [rbp + 0x568]
14079ec9d: e8eee23800                   call 0x140b2cf90
14079eca2: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
14079eca9: 4885d2                       test rdx, rdx
14079ecac: 488db580050000               lea rsi, [rbp + 0x580]
14079ecb3: 7412                         je 0x14079ecc7
14079ecb5: 41b801000000                 mov r8d, 1
14079ecbb: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
14079ecc2: e8594fe7ff                   call 0x140613c20
14079ecc7: 4183be8800000001             cmp dword ptr [r14 + 0x88], 1
14079eccf: 0f853f080000                 jne 0x14079f514
14079ecd5: 418bbe8c000000               mov edi, dword ptr [r14 + 0x8c]
14079ecdc: 8b85b0020000                 mov eax, dword ptr [rbp + 0x2b0]
14079ece2: 898598050000                 mov dword ptr [rbp + 0x598], eax
14079ece8: 488b85a8020000               mov rax, qword ptr [rbp + 0x2a8]
14079ecef: 48898590050000               mov qword ptr [rbp + 0x590], rax
14079ecf6: 0f2885c0000000               movaps xmm0, xmmword ptr [rbp + 0xc0]
14079ecfd: 0f298580050000               movaps xmmword ptr [rbp + 0x580], xmm0
14079ed04: 48b80000803f0000803f         movabs rax, 0x3f8000003f800000
14079ed0e: 4889859c050000               mov qword ptr [rbp + 0x59c], rax
14079ed15: c785a40500000000803f         mov dword ptr [rbp + 0x5a4], 0x3f800000
14079ed1f: 488d8dc8030000               lea rcx, [rbp + 0x3c8]
14079ed26: 48895c2420                   mov qword ptr [rsp + 0x20], rbx
14079ed2b: f30f10153d6b1802             movss xmm2, dword ptr [rip + 0x2186b3d]
14079ed33: 660f6e0d956b1802             movd xmm1, dword ptr [rip + 0x2186b95]
14079ed3b: 660f6fd9                     movdqa xmm3, xmm1
14079ed3f: e89cb33300                   call 0x140ada0e0
14079ed44: c685c003000002               mov byte ptr [rbp + 0x3c0], 2
14079ed4b: c685d004000000               mov byte ptr [rbp + 0x4d0], 0
14079ed52: 488d8d80050000               lea rcx, [rbp + 0x580]
14079ed59: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079ed60: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079ed67: e8a4a02000                   call 0x1409a8e10
14079ed6c: c6854506000001               mov byte ptr [rbp + 0x645], 1
14079ed73: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
14079ed7a: e801a22000                   call 0x1409a8f80
14079ed7f: c6854506000000               mov byte ptr [rbp + 0x645], 0
14079ed86: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
14079ed8d: 488d9580050000               lea rdx, [rbp + 0x580]
14079ed94: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079ed9b: e8e0a02000                   call 0x1409a8e80
14079eda0: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079eda7: 4889f1                       mov rcx, rsi
14079edaa: 4531c0                       xor r8d, r8d
14079edad: e82ea12000                   call 0x1409a8ee0
14079edb2: 488d15f7312e02               lea rdx, [rip + 0x22e31f7]
14079edb9: 41b820000000                 mov r8d, 0x20
14079edbf: 488b8d78040000               mov rcx, qword ptr [rbp + 0x478]
14079edc6: 4989f1                       mov r9, rsi
14079edc9: e802491800                   call 0x1409236d0
14079edce: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
14079edd5: 488995d8040000               mov qword ptr [rbp + 0x4d8], rdx
14079eddc: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
14079ede3: 89fa                         mov edx, edi
14079ede5: e846db2700                   call 0x140a1c930
14079edea: 4885c0                       test rax, rax
14079eded: 0f84c7130000                 je 0x1407a01ba
14079edf3: 4989c6                       mov r14, rax
14079edf6: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
14079edfd: e87ef81800                   call 0x14092e680
14079ee02: 84c0                         test al, al
14079ee04: 0f84b0130000                 je 0x1407a01ba
14079ee0a: 488db5d0040000               lea rsi, [rbp + 0x4d0]
14079ee11: 4889f1                       mov rcx, rsi
14079ee14: e8977aeeff                   call 0x1406868b0
14079ee19: 4889c3                       mov rbx, rax
14079ee1c: 488d8d80050000               lea rcx, [rbp + 0x580]
14079ee23: 4889f2                       mov rdx, rsi
14079ee26: e815f81800                   call 0x14092e640
14079ee2b: 83bd8005000002               cmp dword ptr [rbp + 0x580], 2
14079ee32: 0f84252f0000                 je 0x1407a1d5d
14079ee38: 488b85b0050000               mov rax, qword ptr [rbp + 0x5b0]
14079ee3f: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
14079ee46: 0f108580050000               movups xmm0, xmmword ptr [rbp + 0x580]
14079ee4d: f30f6f8d90050000             movdqu xmm1, xmmword ptr [rbp + 0x590]
14079ee55: 0f1095a0050000               movups xmm2, xmmword ptr [rbp + 0x5a0]
14079ee5c: 0f2995e0030000               movaps xmmword ptr [rbp + 0x3e0], xmm2
14079ee63: 660f7f8dd0030000             movdqa xmmword ptr [rbp + 0x3d0], xmm1
14079ee6b: 0f2985c0030000               movaps xmmword ptr [rbp + 0x3c0], xmm0
14079ee72: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
14079ee79: 89fa                         mov edx, edi
14079ee7b: e860d52700                   call 0x140a1c3e0
14079ee80: 4989c7                       mov r15, rax
14079ee83: 4989d4                       mov r12, rdx
14079ee86: 41807e2202                   cmp byte ptr [r14 + 0x22], 2
14079ee8b: 0f8475120000                 je 0x1407a0106
14079ee91: f30f10bdac020000             movss xmm7, dword ptr [rbp + 0x2ac]
14079ee99: f30f583dff2d2e02             addss xmm7, dword ptr [rip + 0x22e2dff]
14079eea1: e968120000                   jmp 0x1407a010e
14079eea6: e8555f5c00                   call 0x140d64e00
14079eeab: 4989c6                       mov r14, rax
14079eeae: 4989d7                       mov r15, rdx
14079eeb1: 4c89e0                       mov rax, r12
14079eeb4: 4883c008                     add rax, 8
14079eeb8: 49c7042401000000             mov qword ptr [r12], 1
14079eec0: 4d89742408                   mov qword ptr [r12 + 8], r14
14079eec5: 4989542410                   mov qword ptr [r12 + 0x10], rdx
14079eeca: 498d4e01                     lea rcx, [r14 + 1]
14079eece: 488908                       mov qword ptr [rax], rcx
14079eed1: 8b85ec020000                 mov eax, dword ptr [rbp + 0x2ec]
14079eed7: 488b8de0020000               mov rcx, qword ptr [rbp + 0x2e0]
14079eede: 8901                         mov dword ptr [rcx], eax
14079eee0: 488bb5d0020000               mov rsi, qword ptr [rbp + 0x2d0]
14079eee7: 4c8d6608                     lea r12, [rsi + 8]
14079eeeb: 488b5610                     mov rdx, qword ptr [rsi + 0x10]
14079eeef: 4889f0                       mov rax, rsi
14079eef2: 4885d2                       test rdx, rdx
14079eef5: 7431                         je 0x14079ef28
14079eef7: 4c8d04d517000000             lea r8, [rdx*8 + 0x17]
14079eeff: 4983e0f0                     and r8, 0xfffffffffffffff0
14079ef03: 4c01c2                       add rdx, r8
14079ef06: 4889f0                       mov rax, rsi
14079ef09: 4883c211                     add rdx, 0x11
14079ef0d: 7419                         je 0x14079ef28
14079ef0f: 498b0c24                     mov rcx, qword ptr [r12]
14079ef13: 4c29c1                       sub rcx, r8
14079ef16: 41b810000000                 mov r8d, 0x10
14079ef1c: e8ff4ce7ff                   call 0x140613c20
14079ef21: 488b85d0020000               mov rax, qword ptr [rbp + 0x2d0]
14079ef28: 0f1005012e2e02               movups xmm0, xmmword ptr [rip + 0x22e2e01]
14079ef2f: 410f11442410                 movups xmmword ptr [r12 + 0x10], xmm0
14079ef35: 0f1005e42d2e02               movups xmm0, xmmword ptr [rip + 0x22e2de4]
14079ef3c: 410f110424                   movups xmmword ptr [r12], xmm0
14079ef41: 4c897628                     mov qword ptr [rsi + 0x28], r14
14079ef45: 4c897e30                     mov qword ptr [rsi + 0x30], r15
14079ef49: 8b8dec020000                 mov ecx, dword ptr [rbp + 0x2ec]
14079ef4f: 488b95e0020000               mov rdx, qword ptr [rbp + 0x2e0]
14079ef56: 890a                         mov dword ptr [rdx], ecx
14079ef58: 8b8da8030000                 mov ecx, dword ptr [rbp + 0x3a8]
14079ef5e: 894844                       mov dword ptr [rax + 0x44], ecx
14079ef61: 488b8da0030000               mov rcx, qword ptr [rbp + 0x3a0]
14079ef68: 4889483c                     mov qword ptr [rax + 0x3c], rcx
14079ef6c: 488d8d80050000               lea rcx, [rbp + 0x580]
14079ef73: 488d9530030000               lea rdx, [rbp + 0x330]
14079ef7a: e8c1f61800                   call 0x14092e640
14079ef7f: 83bd8005000002               cmp dword ptr [rbp + 0x580], 2
14079ef86: 0f84ad2d0000                 je 0x1407a1d39
14079ef8c: f30f1085a8050000             movss xmm0, dword ptr [rbp + 0x5a8]
14079ef94: 8b85ec020000                 mov eax, dword ptr [rbp + 0x2ec]
14079ef9a: f30f5c85a4030000             subss xmm0, dword ptr [rbp + 0x3a4]
14079efa2: 488b8de0020000               mov rcx, qword ptr [rbp + 0x2e0]
14079efa9: 8901                         mov dword ptr [rcx], eax
14079efab: 488b85d0020000               mov rax, qword ptr [rbp + 0x2d0]
14079efb2: f30f114038                   movss dword ptr [rax + 0x38], xmm0
14079efb7: 488d8d80050000               lea rcx, [rbp + 0x580]
14079efbe: 488b95b8030000               mov rdx, qword ptr [rbp + 0x3b8]
14079efc5: 4189f8                       mov r8d, edi
14079efc8: e8f3f81500                   call 0x1408fe8c0
14079efcd: 488b8580050000               mov rax, qword ptr [rbp + 0x580]
14079efd4: 48898558060000               mov qword ptr [rbp + 0x658], rax
14079efdb: 488b8588050000               mov rax, qword ptr [rbp + 0x588]
14079efe2: 48898568060000               mov qword ptr [rbp + 0x668], rax
14079efe9: 4c8ba590050000               mov r12, qword ptr [rbp + 0x590]
14079eff0: 4d85e4                       test r12, r12
14079eff3: 747d                         je 0x14079f072
14079eff5: 49c1e402                     shl r12, 2
14079eff9: 4531ed                       xor r13d, r13d
14079effc: 4c8d35052f2e02               lea r14, [rip + 0x22e2f05]
14079f003: eb14                         jmp 0x14079f019
14079f005: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
14079f010: 4983c504                     add r13, 4
14079f014: 4d39ec                       cmp r12, r13
14079f017: 7459                         je 0x14079f072
14079f019: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
14079f020: 468b3c28                     mov r15d, dword ptr [rax + r13]
14079f024: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
14079f02b: 4489fa                       mov edx, r15d
14079f02e: e8fdd82700                   call 0x140a1c930
14079f033: 4885c0                       test rax, rax
14079f036: 74d8                         je 0x14079f010
14079f038: 8b8dec020000                 mov ecx, dword ptr [rbp + 0x2ec]
14079f03e: 488b95e0020000               mov rdx, qword ptr [rbp + 0x2e0]
14079f045: 890a                         mov dword ptr [rdx], ecx
14079f047: 488bb5d0020000               mov rsi, qword ptr [rbp + 0x2d0]
14079f04e: 4883c030                     add rax, 0x30
14079f052: 4889c1                       mov rcx, rax
14079f055: 4c89f2                       mov rdx, r14
14079f058: e853a40900                   call 0x1408394b0
14079f05d: 4883c608                     add rsi, 8
14079f061: f30f1010                     movss xmm2, dword ptr [rax]
14079f065: 4889f1                       mov rcx, rsi
14079f068: 4489fa                       mov edx, r15d
14079f06b: e8a082ac00                   call 0x141267310
14079f070: eb9e                         jmp 0x14079f010
14079f072: 488b9558060000               mov rdx, qword ptr [rbp + 0x658]
14079f079: 4885d2                       test rdx, rdx
14079f07c: 488db580050000               lea rsi, [rbp + 0x580]
14079f083: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
14079f08a: 7416                         je 0x14079f0a2
14079f08c: 48c1e202                     shl rdx, 2
14079f090: 41b804000000                 mov r8d, 4
14079f096: 488b8d68060000               mov rcx, qword ptr [rbp + 0x668]
14079f09d: e87e4be7ff                   call 0x140613c20
14079f0a2: f20f1005c62b2e02             movsd xmm0, qword ptr [rip + 0x22e2bc6]
14079f0aa: f20f118580050000             movsd qword ptr [rbp + 0x580], xmm0
14079f0b2: c6858805000000               mov byte ptr [rbp + 0x588], 0
14079f0b9: 48895c2420                   mov qword ptr [rsp + 0x20], rbx
14079f0be: 488d8d90040000               lea rcx, [rbp + 0x490]
14079f0c5: 4c8d8530030000               lea r8, [rbp + 0x330]
14079f0cc: 4c8d8dd0020000               lea r9, [rbp + 0x2d0]
14079f0d3: 488b9598070000               mov rdx, qword ptr [rbp + 0x798]
14079f0da: e82177ad00                   call 0x141276800
14079f0df: 83bd9004000001               cmp dword ptr [rbp + 0x490], 1
14079f0e6: 0f85f7030000                 jne 0x14079f4e3
14079f0ec: f30f10b5a0040000             movss xmm6, dword ptr [rbp + 0x4a0]
14079f0f4: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
14079f0fb: 488b95b8030000               mov rdx, qword ptr [rbp + 0x3b8]
14079f102: 4189f8                       mov r8d, edi
14079f105: e8b6f71500                   call 0x1408fe8c0
14079f10a: 8b85ec020000                 mov eax, dword ptr [rbp + 0x2ec]
14079f110: 488b8de0020000               mov rcx, qword ptr [rbp + 0x2e0]
14079f117: 8901                         mov dword ptr [rcx], eax
14079f119: 488b85d0020000               mov rax, qword ptr [rbp + 0x2d0]
14079f120: c70001000000                 mov dword ptr [rax], 1
14079f126: f30f117004                   movss dword ptr [rax + 4], xmm6
14079f12b: 0fb605d75ae902               movzx eax, byte ptr [rip + 0x2e95ad7]
14079f132: b918000000                   mov ecx, 0x18
14079f137: ba04000000                   mov edx, 4
14079f13c: e8cf4ae7ff                   call 0x140613c10
14079f141: 4885c0                       test rax, rax
14079f144: 0f84fb2b0000                 je 0x1407a1d45
14079f14a: 488b8dd0020000               mov rcx, qword ptr [rbp + 0x2d0]
14079f151: f30f104144                   movss xmm0, dword ptr [rcx + 0x44]
14079f156: 0f57c9                       xorps xmm1, xmm1
14079f159: f20f10513c                   movsd xmm2, qword ptr [rcx + 0x3c]
14079f15e: f30f16da                     movshdup xmm3, xmm2
14079f162: f30f581d162b2e02             addss xmm3, dword ptr [rip + 0x22e2b16]
14079f16a: f30f58c8                     addss xmm1, xmm0
14079f16e: f20f10251a2b2e02             movsd xmm4, qword ptr [rip + 0x22e2b1a]
14079f176: 0f58e2                       addps xmm4, xmm2
14079f179: f30f1110                     movss dword ptr [rax], xmm2
14079f17d: f30f115804                   movss dword ptr [rax + 4], xmm3
14079f182: f30f114008                   movss dword ptr [rax + 8], xmm0
14079f187: 0f13600c                     movlps qword ptr [rax + 0xc], xmm4
14079f18b: f30f114814                   movss dword ptr [rax + 0x14], xmm1
14079f190: 48c785c003000002000000       mov qword ptr [rbp + 0x3c0], 2
14079f19b: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
14079f1a2: 48c785d003000002000000       mov qword ptr [rbp + 0x3d0], 2
14079f1ad: c6856706000001               mov byte ptr [rbp + 0x667], 1
14079f1b4: 488d8d80050000               lea rcx, [rbp + 0x580]
14079f1bb: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079f1c2: e8a95c3600                   call 0x140b04e70
14079f1c7: c6856706000001               mov byte ptr [rbp + 0x667], 1
14079f1ce: 488d8d80050000               lea rcx, [rbp + 0x580]
14079f1d5: 488b9568050000               mov rdx, qword ptr [rbp + 0x568]
14079f1dc: e88f5d3600                   call 0x140b04f70
14079f1e1: c6856706000001               mov byte ptr [rbp + 0x667], 1
14079f1e8: 488d8d30030000               lea rcx, [rbp + 0x330]
14079f1ef: e81ced1800                   call 0x14092df10
14079f1f4: 89c3                         mov ebx, eax
14079f1f6: 488b85d0040000               mov rax, qword ptr [rbp + 0x4d0]
14079f1fd: 48898548060000               mov qword ptr [rbp + 0x648], rax
14079f204: 488b85d8040000               mov rax, qword ptr [rbp + 0x4d8]
14079f20b: 48898558060000               mov qword ptr [rbp + 0x658], rax
14079f212: 488b85e0040000               mov rax, qword ptr [rbp + 0x4e0]
14079f219: 4885c0                       test rax, rax
14079f21c: 0f847b020000                 je 0x14079f49d
14079f222: 4c8bad58060000               mov r13, qword ptr [rbp + 0x658]
14079f229: 488d348500000000             lea rsi, [rax*4]
14079f231: 4c01ee                       add rsi, r13
14079f234: 488b8590070000               mov rax, qword ptr [rbp + 0x790]
14079f23b: 8b481c                       mov ecx, dword ptr [rax + 0x1c]
14079f23e: 898d68060000                 mov dword ptr [rbp + 0x668], ecx
14079f244: 4c8b30                       mov r14, qword ptr [rax]
14079f247: 488b7810                     mov rdi, qword ptr [rax + 0x10]
14079f24b: 660f76ff                     pcmpeqd xmm7, xmm7
14079f24f: f3440f100514d51802           movss xmm8, dword ptr [rip + 0x218d514]
14079f258: e9b3010000                   jmp 0x14079f410
14079f25d: 0f1f00                       nop dword ptr [rax]
14079f260: f3420f6f0c01                 movdqu xmm1, xmmword ptr [rcx + r8]
14079f266: 660f6fd1                     movdqa xmm2, xmm1
14079f26a: 660f74d0                     pcmpeqb xmm2, xmm0
14079f26e: 66440fd7ca                   pmovmskb r9d, xmm2
14079f273: 4585c9                       test r9d, r9d
14079f276: 742d                         je 0x14079f2a5
14079f278: f3450fbcd1                   tzcnt r10d, r9d
14079f27d: 4d01c2                       add r10, r8
14079f280: 4921d2                       and r10, rdx
14079f283: 4e8d1cd500000000             lea r11, [r10*8]
14079f28b: 4989cc                       mov r12, rcx
14079f28e: 4d29dc                       sub r12, r11
14079f291: 453b7c24f8                   cmp r15d, dword ptr [r12 - 8]
14079f296: 742f                         je 0x14079f2c7
14079f298: 458d51ff                     lea r10d, [r9 - 1]
14079f29c: 664521ca                     and r10w, r9w
14079f2a0: 4589d1                       mov r9d, r10d
14079f2a3: 75d3                         jne 0x14079f278
14079f2a5: 660f74cf                     pcmpeqb xmm1, xmm7
14079f2a9: 66440fd7c9                   pmovmskb r9d, xmm1
14079f2ae: 4585c9                       test r9d, r9d
14079f2b1: 0f85d9010000                 jne 0x14079f490
14079f2b7: 4901c0                       add r8, rax
14079f2ba: 4983c010                     add r8, 0x10
14079f2be: 4883c010                     add rax, 0x10
14079f2c2: 4921d0                       and r8, rdx
14079f2c5: eb99                         jmp 0x14079f260
14079f2c7: 49f7da                       neg r10
14079f2ca: f3460f104cd1fc               movss xmm9, dword ptr [rcx + r10*8 - 4]
14079f2d1: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
14079f2d8: 4489fa                       mov edx, r15d
14079f2db: e800d12700                   call 0x140a1c3e0
14079f2e0: f3440f58ce                   addss xmm9, xmm6
14079f2e5: 660fefc0                     pxor xmm0, xmm0
14079f2e9: f3410f5fc1                   maxss xmm0, xmm9
14079f2ee: 410f28c8                     movaps xmm1, xmm8
14079f2f2: f30f5dc8                     minss xmm1, xmm0
14079f2f6: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079f2fd: 48899588050000               mov qword ptr [rbp + 0x588], rdx
14079f304: 48c7859005000003000000       mov qword ptr [rbp + 0x590], 3
14079f30f: c785d005000001000000         mov dword ptr [rbp + 0x5d0], 1
14079f319: f30f118dd4050000             movss dword ptr [rbp + 0x5d4], xmm1
14079f321: c785d805000000000000         mov dword ptr [rbp + 0x5d8], 0
14079f32b: c685e005000000               mov byte ptr [rbp + 0x5e0], 0
14079f332: c685e205000000               mov byte ptr [rbp + 0x5e2], 0
14079f339: 66c785e40500000240           mov word ptr [rbp + 0x5e4], 0x4002
14079f342: 4489bde8050000               mov dword ptr [rbp + 0x5e8], r15d
14079f349: c685ec0500003f               mov byte ptr [rbp + 0x5ec], 0x3f
14079f350: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
14079f357: 488b9520020000               mov rdx, qword ptr [rbp + 0x220]
14079f35e: 4589f8                       mov r8d, r15d
14079f361: 4c8d0db82b2e02               lea r9, [rip + 0x22e2bb8]
14079f368: e8b3b62700                   call 0x140a1aa20
14079f36d: 80fb02                       cmp bl, 2
14079f370: 0f93c2                       setae dl
14079f373: 488d8dd0030000               lea rcx, [rbp + 0x3d0]
14079f37a: 8b4108                       mov eax, dword ptr [rcx + 8]
14079f37d: 898548020000                 mov dword ptr [rbp + 0x248], eax
14079f383: 488b01                       mov rax, qword ptr [rcx]
14079f386: 48898540020000               mov qword ptr [rbp + 0x240], rax
14079f38d: c6854c02000000               mov byte ptr [rbp + 0x24c], 0
14079f394: 488b8588020000               mov rax, qword ptr [rbp + 0x288]
14079f39b: 8b8d38050000                 mov ecx, dword ptr [rbp + 0x538]
14079f3a1: 8908                         mov dword ptr [rax], ecx
14079f3a3: 488b8598020000               mov rax, qword ptr [rbp + 0x298]
14079f3aa: 8b8d3c050000                 mov ecx, dword ptr [rbp + 0x53c]
14079f3b0: 8908                         mov dword ptr [rax], ecx
14079f3b2: 8b8568060000                 mov eax, dword ptr [rbp + 0x668]
14079f3b8: 8907                         mov dword ptr [rdi], eax
14079f3ba: 488b85a0020000               mov rax, qword ptr [rbp + 0x2a0]
14079f3c1: 4889442438                   mov qword ptr [rsp + 0x38], rax
14079f3c6: 488d45c0                     lea rax, [rbp - 0x40]
14079f3ca: 4889442430                   mov qword ptr [rsp + 0x30], rax
14079f3cf: 4c89742428                   mov qword ptr [rsp + 0x28], r14
14079f3d4: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
14079f3db: 4889442420                   mov qword ptr [rsp + 0x20], rax
14079f3e0: 488d8d80050000               lea rcx, [rbp + 0x580]
14079f3e7: 4c8d8540020000               lea r8, [rbp + 0x240]
14079f3ee: 4c8b8d38060000               mov r9, qword ptr [rbp + 0x638]
14079f3f5: e8367aad00                   call 0x141276e30
14079f3fa: f3440f100d01651802           movss xmm9, dword ptr [rip + 0x2186501]
14079f403: e988000000                   jmp 0x14079f490
14079f408: 0f1f840000000000             nop dword ptr [rax + rax]
14079f410: 458b7d00                     mov r15d, dword ptr [r13]
14079f414: 4489bdf0010000               mov dword ptr [rbp + 0x1f0], r15d
14079f41b: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
14079f422: 4489fa                       mov edx, r15d
14079f425: e806d52700                   call 0x140a1c930
14079f42a: 4885c0                       test rax, rax
14079f42d: f3440f100dce641802           movss xmm9, dword ptr [rip + 0x21864ce]
14079f436: 7458                         je 0x14079f490
14079f438: 80782202                     cmp byte ptr [rax + 0x22], 2
14079f43c: 7252                         jb 0x14079f490
14079f43e: 4c8ba5d0020000               mov r12, qword ptr [rbp + 0x2d0]
14079f445: 49837c242000                 cmp qword ptr [r12 + 0x20], 0
14079f44b: 7443                         je 0x14079f490
14079f44d: 498d4c2428                   lea rcx, [r12 + 0x28]
14079f452: 488d95f0010000               lea rdx, [rbp + 0x1f0]
14079f459: e8721ae1ff                   call 0x1405b0ed0
14079f45e: 498b4c2408                   mov rcx, qword ptr [r12 + 8]
14079f463: 498b542410                   mov rdx, qword ptr [r12 + 0x10]
14079f468: 4989d0                       mov r8, rdx
14079f46b: 4921c0                       and r8, rax
14079f46e: 48c1e839                     shr rax, 0x39
14079f472: 660f6ec0                     movd xmm0, eax
14079f476: 660f60c0                     punpcklbw xmm0, xmm0
14079f47a: f20f70c000                   pshuflw xmm0, xmm0, 0
14079f47f: 660f70c000                   pshufd xmm0, xmm0, 0
14079f484: 31c0                         xor eax, eax
14079f486: e9d5fdffff                   jmp 0x14079f260
14079f48b: 0f1f440000                   nop dword ptr [rax + rax]
14079f490: 4983c504                     add r13, 4
14079f494: 4939f5                       cmp r13, rsi
14079f497: 0f8573ffffff                 jne 0x14079f410
14079f49d: 488b9548060000               mov rdx, qword ptr [rbp + 0x648]
14079f4a4: 4885d2                       test rdx, rdx
14079f4a7: 7416                         je 0x14079f4bf
14079f4a9: 48c1e202                     shl rdx, 2
14079f4ad: 41b804000000                 mov r8d, 4
14079f4b3: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
14079f4ba: e86147e7ff                   call 0x140613c20
14079f4bf: 488db580050000               lea rsi, [rbp + 0x580]
14079f4c6: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
14079f4cd: f3410f108690000000           movss xmm0, dword ptr [r14 + 0x90]
14079f4d6: 0f57f6                       xorps xmm6, xmm6
14079f4d9: 0f2ec6                       ucomiss xmm0, xmm6
14079f4dc: 7636                         jbe 0x14079f514
14079f4de: e99df5ffff                   jmp 0x14079ea80
14079f4e3: 8b85ec020000                 mov eax, dword ptr [rbp + 0x2ec]
14079f4e9: 488b8de0020000               mov rcx, qword ptr [rbp + 0x2e0]
14079f4f0: 8901                         mov dword ptr [rcx], eax
14079f4f2: 488b85d0020000               mov rax, qword ptr [rbp + 0x2d0]
14079f4f9: c70000000000                 mov dword ptr [rax], 0
14079f4ff: f3410f108690000000           movss xmm0, dword ptr [r14 + 0x90]
14079f508: 0f57f6                       xorps xmm6, xmm6
14079f50b: 0f2ec6                       ucomiss xmm0, xmm6
14079f50e: 0f876cf5ffff                 ja 0x14079ea80
14079f514: f3410f104648                 movss xmm0, dword ptr [r14 + 0x48]
14079f51a: 0f2ec6                       ucomiss xmm0, xmm6
14079f51d: 0f86b40c0000                 jbe 0x1407a01d7
14079f523: 488b85b8070000               mov rax, qword ptr [rbp + 0x7b8]
14079f52a: 488b10                       mov rdx, qword ptr [rax]
14079f52d: 4889f1                       mov rcx, rsi
14079f530: e8bb784f00                   call 0x140c96df0
14079f535: f30f108d80050000             movss xmm1, dword ptr [rbp + 0x580]
14079f53d: f30f108588050000             movss xmm0, dword ptr [rbp + 0x588]
14079f545: 0f28d1                       movaps xmm2, xmm1
14079f548: f30f59d1                     mulss xmm2, xmm1
14079f54c: 0f28d8                       movaps xmm3, xmm0
14079f54f: f30f59d8                     mulss xmm3, xmm0
14079f553: f30f58da                     addss xmm3, xmm2
14079f557: f30f51db                     sqrtss xmm3, xmm3
14079f55b: 410f28d1                     movaps xmm2, xmm9
14079f55f: f30f5ed3                     divss xmm2, xmm3
14079f563: f30f59ca                     mulss xmm1, xmm2
14079f567: f30f59d0                     mulss xmm2, xmm0
14079f56b: 488d8d20010000               lea rcx, [rbp + 0x120]
14079f572: e8591a4f00                   call 0x140c90fd0
14079f577: 418b4654                     mov eax, dword ptr [r14 + 0x54]
14079f57b: 898598030000                 mov dword ptr [rbp + 0x398], eax
14079f581: 498b464c                     mov rax, qword ptr [r14 + 0x4c]
14079f585: 48898590030000               mov qword ptr [rbp + 0x390], rax
14079f58c: f3410f104648                 movss xmm0, dword ptr [r14 + 0x48]
14079f592: f30f590536631802             mulss xmm0, dword ptr [rip + 0x2186336]
14079f59a: f30f118540020000             movss dword ptr [rbp + 0x240], xmm0
14079f5a2: c685c003000000               mov byte ptr [rbp + 0x3c0], 0
14079f5a9: 488d05382a2e02               lea rax, [rip + 0x22e2a38]
14079f5b0: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
14079f5b7: 48c785d00300000a000000       mov qword ptr [rbp + 0x3d0], 0xa
14079f5c2: c685d803000001               mov byte ptr [rbp + 0x3d8], 1
14079f5c9: 488d05222a2e02               lea rax, [rip + 0x22e2a22]
14079f5d0: 488985e0030000               mov qword ptr [rbp + 0x3e0], rax
14079f5d7: 48c785e803000012000000       mov qword ptr [rbp + 0x3e8], 0x12
14079f5e2: 48c785f003000000000000       mov qword ptr [rbp + 0x3f0], 0
14079f5ed: 31c9                         xor ecx, ecx
14079f5ef: 488dbdd0040000               lea rdi, [rbp + 0x4d0]
14079f5f6: eb12                         jmp 0x14079f60a
14079f5f8: 0f1f840000000000             nop dword ptr [rax + rax]
14079f600: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
14079f607: 48ffc1                       inc rcx
14079f60a: 4883f902                     cmp rcx, 2
14079f60e: 0f8418020000                 je 0x14079f82c
14079f614: 488d0449                     lea rax, [rcx + rcx*2]
14079f618: 440fb6acc5c0030000           movzx r13d, byte ptr [rbp + rax*8 + 0x3c0]
14079f621: 4180fd02                     cmp r13b, 2
14079f625: 0f8401020000                 je 0x14079f82c
14079f62b: 48898d58060000               mov qword ptr [rbp + 0x658], rcx
14079f632: 488d04c5c0030000             lea rax, [rax*8 + 0x3c0]
14079f63a: 4801e8                       add rax, rbp
14079f63d: 0f104008                     movups xmm0, xmmword ptr [rax + 8]
14079f641: 0f118590040000               movups xmmword ptr [rbp + 0x490], xmm0
14079f648: 488d8590030000               lea rax, [rbp + 0x390]
14079f64f: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079f656: 488d05a3c35a00               lea rax, [rip + 0x5ac3a3]
14079f65d: 48898588050000               mov qword ptr [rbp + 0x588], rax
14079f664: 488d8590040000               lea rax, [rbp + 0x490]
14079f66b: 48898590050000               mov qword ptr [rbp + 0x590], rax
14079f672: 488d052742d9ff               lea rax, [rip - 0x26bdd9]
14079f679: 48898598050000               mov qword ptr [rbp + 0x598], rax
14079f680: 488d8520010000               lea rax, [rbp + 0x120]
14079f687: 488985a0050000               mov qword ptr [rbp + 0x5a0], rax
14079f68e: 488d05fbb85a00               lea rax, [rip + 0x5ab8fb]
14079f695: 488985a8050000               mov qword ptr [rbp + 0x5a8], rax
14079f69c: 488d8540020000               lea rax, [rbp + 0x240]
14079f6a3: 488985b0050000               mov qword ptr [rbp + 0x5b0], rax
14079f6aa: 488d055fd25e00               lea rax, [rip + 0x5ed25f]
14079f6b1: 488985b8050000               mov qword ptr [rbp + 0x5b8], rax
14079f6b8: 488d05e92a2e02               lea rax, [rip + 0x22e2ae9]
14079f6bf: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
14079f6c6: 48c785d804000004000000       mov qword ptr [rbp + 0x4d8], 4
14079f6d1: 4c8da580050000               lea r12, [rbp + 0x580]
14079f6d8: 4c89a5e0040000               mov qword ptr [rbp + 0x4e0], r12
14079f6df: 48c785e804000004000000       mov qword ptr [rbp + 0x4e8], 4
14079f6ea: 48c785f004000000000000       mov qword ptr [rbp + 0x4f0], 0
14079f6f5: 488d4d60                     lea rcx, [rbp + 0x60]
14079f6f9: 4889fe                       mov rsi, rdi
14079f6fc: 4889fa                       mov rdx, rdi
14079f6ff: e88ca85d00                   call 0x140d79f90
14079f704: 488b4560                     mov rax, qword ptr [rbp + 0x60]
14079f708: 48898568060000               mov qword ptr [rbp + 0x668], rax
14079f70f: 4c8b7568                     mov r14, qword ptr [rbp + 0x68]
14079f713: 4c8b7d70                     mov r15, qword ptr [rbp + 0x70]
14079f717: 488bbd90040000               mov rdi, qword ptr [rbp + 0x490]
14079f71e: 488b9d98040000               mov rbx, qword ptr [rbp + 0x498]
14079f725: f30f10b540020000             movss xmm6, dword ptr [rbp + 0x240]
14079f72d: 488b8590030000               mov rax, qword ptr [rbp + 0x390]
14079f734: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079f73b: 8b8598030000                 mov eax, dword ptr [rbp + 0x398]
14079f741: 898588050000                 mov dword ptr [rbp + 0x588], eax
14079f747: 0f28bd20010000               movaps xmm7, xmmword ptr [rbp + 0x120]
14079f74e: 0fb605b454e902               movzx eax, byte ptr [rip + 0x2e954b4]
14079f755: b950000000                   mov ecx, 0x50
14079f75a: ba10000000                   mov edx, 0x10
14079f75f: e8ac44e7ff                   call 0x140613c10
14079f764: 4885c0                       test rax, rax
14079f767: 4c89b548060000               mov qword ptr [rbp + 0x648], r14
14079f76e: 0f8432250000                 je 0x1407a1ca6
14079f774: 48c70001000000               mov qword ptr [rax], 1
14079f77b: 48c7400801000000             mov qword ptr [rax + 8], 1
14079f783: 48897810                     mov qword ptr [rax + 0x10], rdi
14079f787: 48895818                     mov qword ptr [rax + 0x18], rbx
14079f78b: 0f297820                     movaps xmmword ptr [rax + 0x20], xmm7
14079f78f: f30f117030                   movss dword ptr [rax + 0x30], xmm6
14079f794: 488b8d80050000               mov rcx, qword ptr [rbp + 0x580]
14079f79b: 48894834                     mov qword ptr [rax + 0x34], rcx
14079f79f: 8b8d88050000                 mov ecx, dword ptr [rbp + 0x588]
14079f7a5: 89483c                       mov dword ptr [rax + 0x3c], ecx
14079f7a8: 44886840                     mov byte ptr [rax + 0x40], r13b
14079f7ac: 488d0d352a2e02               lea rcx, [rip + 0x22e2a35]
14079f7b3: 48894c2420                   mov qword ptr [rsp + 0x20], rcx
14079f7b8: 4c89e1                       mov rcx, r12
14079f7bb: 4c89f2                       mov rdx, r14
14079f7be: 4d89f8                       mov r8, r15
14079f7c1: 4989c1                       mov r9, rax
14079f7c4: e8e7e00f00                   call 0x14089d8b0
14079f7c9: 4889f7                       mov rdi, rsi
14079f7cc: 488b85a0050000               mov rax, qword ptr [rbp + 0x5a0]
14079f7d3: 488985f0040000               mov qword ptr [rbp + 0x4f0], rax
14079f7da: 0f108580050000               movups xmm0, xmmword ptr [rbp + 0x580]
14079f7e1: 0f108d90050000               movups xmm1, xmmword ptr [rbp + 0x590]
14079f7e8: 0f298de0040000               movaps xmmword ptr [rbp + 0x4e0], xmm1
14079f7ef: 0f2985d0040000               movaps xmmword ptr [rbp + 0x4d0], xmm0
14079f7f6: 488b8d68050000               mov rcx, qword ptr [rbp + 0x568]
14079f7fd: 4889f2                       mov rdx, rsi
14079f800: e88bd73800                   call 0x140b2cf90
14079f805: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
14079f80c: 4885d2                       test rdx, rdx
14079f80f: 0f84ebfdffff                 je 0x14079f600
14079f815: 41b801000000                 mov r8d, 1
14079f81b: 488b8d48060000               mov rcx, qword ptr [rbp + 0x648]
14079f822: e8f943e7ff                   call 0x140613c20
14079f827: e9d4fdffff                   jmp 0x14079f600
14079f82c: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
14079f833: 41837e4001                   cmp dword ptr [r14 + 0x40], 1
14079f838: 488d9580050000               lea rdx, [rbp + 0x580]
14079f83f: 0f8592090000                 jne 0x1407a01d7
14079f845: 418b7e44                     mov edi, dword ptr [r14 + 0x44]
14079f849: 8b8598030000                 mov eax, dword ptr [rbp + 0x398]
14079f84f: 898598050000                 mov dword ptr [rbp + 0x598], eax
14079f855: 488b8590030000               mov rax, qword ptr [rbp + 0x390]
14079f85c: 48898590050000               mov qword ptr [rbp + 0x590], rax
14079f863: 0f288520010000               movaps xmm0, xmmword ptr [rbp + 0x120]
14079f86a: 0f298580050000               movaps xmmword ptr [rbp + 0x580], xmm0
14079f871: 48b80000803f0000803f         movabs rax, 0x3f8000003f800000
14079f87b: 4889859c050000               mov qword ptr [rbp + 0x59c], rax
14079f882: c785a40500000000803f         mov dword ptr [rbp + 0x5a4], 0x3f800000
14079f88c: 488d8dc8030000               lea rcx, [rbp + 0x3c8]
14079f893: 4889542420                   mov qword ptr [rsp + 0x20], rdx
14079f898: f30f1015d05f1802             movss xmm2, dword ptr [rip + 0x2185fd0]
14079f8a0: 660f6e0d28601802             movd xmm1, dword ptr [rip + 0x2186028]
14079f8a8: 660f6fd9                     movdqa xmm3, xmm1
14079f8ac: e82fa83300                   call 0x140ada0e0
14079f8b1: c685c003000002               mov byte ptr [rbp + 0x3c0], 2
14079f8b8: c685d004000000               mov byte ptr [rbp + 0x4d0], 0
14079f8bf: 488d8d80050000               lea rcx, [rbp + 0x580]
14079f8c6: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079f8cd: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079f8d4: e837952000                   call 0x1409a8e10
14079f8d9: c6854406000001               mov byte ptr [rbp + 0x644], 1
14079f8e0: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
14079f8e7: e894962000                   call 0x1409a8f80
14079f8ec: c6854406000000               mov byte ptr [rbp + 0x644], 0
14079f8f3: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
14079f8fa: 488d9580050000               lea rdx, [rbp + 0x580]
14079f901: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079f908: e873952000                   call 0x1409a8e80
14079f90d: 488db580050000               lea rsi, [rbp + 0x580]
14079f914: 488d95c0030000               lea rdx, [rbp + 0x3c0]
14079f91b: 4889f1                       mov rcx, rsi
14079f91e: 4531c0                       xor r8d, r8d
14079f921: e8ba952000                   call 0x1409a8ee0
14079f926: 488d15d7262e02               lea rdx, [rip + 0x22e26d7]
14079f92d: 41b80f000000                 mov r8d, 0xf
14079f933: 488b8d78040000               mov rcx, qword ptr [rbp + 0x478]
14079f93a: 4989f1                       mov r9, rsi
14079f93d: e88e3d1800                   call 0x1409236d0
14079f942: 48898590040000               mov qword ptr [rbp + 0x490], rax
14079f949: 48899598040000               mov qword ptr [rbp + 0x498], rdx
14079f950: 488d8d90040000               lea rcx, [rbp + 0x490]
14079f957: e8b4e51800                   call 0x14092df10
14079f95c: 04fe                         add al, 0xfe
14079f95e: 0fb6c0                       movzx eax, al
14079f961: 3c03                         cmp al, 3
14079f963: b903000000                   mov ecx, 3
14079f968: 0f42c8                       cmovb ecx, eax
14079f96b: 0fb6c1                       movzx eax, cl
14079f96e: 8d48fe                       lea ecx, [rax - 2]
14079f971: 83f902                       cmp ecx, 2
14079f974: 0f832a060000                 jae 0x14079ffa4
14079f97a: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
14079f981: 488b95b8030000               mov rdx, qword ptr [rbp + 0x3b8]
14079f988: 4189f8                       mov r8d, edi
14079f98b: e830ef1500                   call 0x1408fe8c0
14079f990: 488b85c0070000               mov rax, qword ptr [rbp + 0x7c0]
14079f997: 488b10                       mov rdx, qword ptr [rax]
14079f99a: 488bb558050000               mov rsi, qword ptr [rbp + 0x558]
14079f9a1: f30f10566c                   movss xmm2, dword ptr [rsi + 0x6c]
14079f9a6: c6855606000001               mov byte ptr [rbp + 0x656], 1
14079f9ad: 488d8de0010000               lea rcx, [rbp + 0x1e0]
14079f9b4: e8c7f71400                   call 0x1408ef180
14079f9b9: f20f107e60                   movsd xmm7, qword ptr [rsi + 0x60]
14079f9be: f20f10b5e0010000             movsd xmm6, qword ptr [rbp + 0x1e0]
14079f9c6: f3440f1085e8010000           movss xmm8, dword ptr [rbp + 0x1e8]
14079f9cf: 450f28c8                     movaps xmm9, xmm8
14079f9d3: f3440f5c4e68                 subss xmm9, dword ptr [rsi + 0x68]
14079f9d9: c6855606000001               mov byte ptr [rbp + 0x656], 1
14079f9e0: 488d8d90040000               lea rcx, [rbp + 0x490]
14079f9e7: e824e51800                   call 0x14092df10
14079f9ec: 888548060000                 mov byte ptr [rbp + 0x648], al
14079f9f2: 440f28d6                     movaps xmm10, xmm6
14079f9f6: 440f5cd7                     subps xmm10, xmm7
14079f9fa: 488b85c0030000               mov rax, qword ptr [rbp + 0x3c0]
14079fa01: 488985b0030000               mov qword ptr [rbp + 0x3b0], rax
14079fa08: 488b8dc8030000               mov rcx, qword ptr [rbp + 0x3c8]
14079fa0f: 488b85d0030000               mov rax, qword ptr [rbp + 0x3d0]
14079fa16: 4c8d2c81                     lea r13, [rcx + rax*4]
14079fa1a: 488d4620                     lea rax, [rsi + 0x20]
14079fa1e: 48898568060000               mov qword ptr [rbp + 0x668], rax
14079fa25: 488d4630                     lea rax, [rsi + 0x30]
14079fa29: 48898518030000               mov qword ptr [rbp + 0x318], rax
14079fa30: 488d4650                     lea rax, [rsi + 0x50]
14079fa34: 48898528030000               mov qword ptr [rbp + 0x328], rax
14079fa3b: 488b85a8070000               mov rax, qword ptr [rbp + 0x7a8]
14079fa42: 488b00                       mov rax, qword ptr [rax]
14079fa45: 48898558060000               mov qword ptr [rbp + 0x658], rax
14079fa4c: 488b8590070000               mov rax, qword ptr [rbp + 0x790]
14079fa53: 8b501c                       mov edx, dword ptr [rax + 0x1c]
14079fa56: 899560050000                 mov dword ptr [rbp + 0x560], edx
14079fa5c: 488b10                       mov rdx, qword ptr [rax]
14079fa5f: 48899550050000               mov qword ptr [rbp + 0x550], rdx
14079fa66: 488b4010                     mov rax, qword ptr [rax + 0x10]
14079fa6a: 48898510030000               mov qword ptr [rbp + 0x310], rax
14079fa71: 488d467c                     lea rax, [rsi + 0x7c]
14079fa75: 48898518020000               mov qword ptr [rbp + 0x218], rax
14079fa7c: 488b85d0070000               mov rax, qword ptr [rbp + 0x7d0]
14079fa83: 488b00                       mov rax, qword ptr [rax]
14079fa86: 48898520030000               mov qword ptr [rbp + 0x320], rax
14079fa8d: 488b8528020000               mov rax, qword ptr [rbp + 0x228]
14079fa94: 488d4020                     lea rax, [rax + 0x20]
14079fa98: 48898508020000               mov qword ptr [rbp + 0x208], rax
14079fa9f: 440f16c6                     movlhps xmm8, xmm6
14079faa3: 440fc6c6e2                   shufps xmm8, xmm6, 0xe2
14079faa8: f3440f16de                   movshdup xmm11, xmm6
14079faad: 66450f76e4                   pcmpeqd xmm12, xmm12
14079fab2: 450f57ed                     xorps xmm13, xmm13
14079fab6: 488b8530020000               mov rax, qword ptr [rbp + 0x230]
14079fabd: 4883c020                     add rax, 0x20
14079fac1: 48898510020000               mov qword ptr [rbp + 0x210], rax
14079fac8: 48898d80040000               mov qword ptr [rbp + 0x480], rcx
14079facf: 4889ce                       mov rsi, rcx
14079fad2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14079fae0: 4c39ee                       cmp rsi, r13
14079fae3: 0f842a0d0000                 je 0x1407a0813
14079fae9: 448b36                       mov r14d, dword ptr [rsi]
14079faec: 4489b5f0020000               mov dword ptr [rbp + 0x2f0], r14d
14079faf3: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
14079fafa: 4489f2                       mov edx, r14d
14079fafd: e82ece2700                   call 0x140a1c930
14079fb02: 4883c604                     add rsi, 4
14079fb06: 4885c0                       test rax, rax
14079fb09: 74d5                         je 0x14079fae0
14079fb0b: 80782202                     cmp byte ptr [rax + 0x22], 2
14079fb0f: 72cf                         jb 0x14079fae0
14079fb11: 488bbd58050000               mov rdi, qword ptr [rbp + 0x558]
14079fb18: 48837f1800                   cmp qword ptr [rdi + 0x18], 0
14079fb1d: 0f84b6040000                 je 0x14079ffd9
14079fb23: 488b8d68060000               mov rcx, qword ptr [rbp + 0x668]
14079fb2a: 488d95f0020000               lea rdx, [rbp + 0x2f0]
14079fb31: e89a13e1ff                   call 0x1405b0ed0
14079fb36: 488b0f                       mov rcx, qword ptr [rdi]
14079fb39: 488b5708                     mov rdx, qword ptr [rdi + 8]
14079fb3d: 4989d0                       mov r8, rdx
14079fb40: 4921c0                       and r8, rax
14079fb43: 48c1e839                     shr rax, 0x39
14079fb47: 660f6ec0                     movd xmm0, eax
14079fb4b: 660f60c0                     punpcklbw xmm0, xmm0
14079fb4f: f20f70c000                   pshuflw xmm0, xmm0, 0
14079fb54: 660f70c000                   pshufd xmm0, xmm0, 0
14079fb59: 488d41f0                     lea rax, [rcx - 0x10]
14079fb5d: 4531c9                       xor r9d, r9d
14079fb60: f3420f6f0c01                 movdqu xmm1, xmmword ptr [rcx + r8]
14079fb66: 660f6fd1                     movdqa xmm2, xmm1
14079fb6a: 660f74d0                     pcmpeqb xmm2, xmm0
14079fb6e: 66440fd7d2                   pmovmskb r10d, xmm2
14079fb73: 4585d2                       test r10d, r10d
14079fb76: 742a                         je 0x14079fba2
14079fb78: f3450fbcda                   tzcnt r11d, r10d
14079fb7d: 4d01c3                       add r11, r8
14079fb80: 4921d3                       and r11, rdx
14079fb83: 4c89df                       mov rdi, r11
14079fb86: 48c1e704                     shl rdi, 4
14079fb8a: 4889c3                       mov rbx, rax
14079fb8d: 4829fb                       sub rbx, rdi
14079fb90: 443b33                       cmp r14d, dword ptr [rbx]
14079fb93: 7430                         je 0x14079fbc5
14079fb95: 458d5aff                     lea r11d, [r10 - 1]
14079fb99: 664521d3                     and r11w, r10w
14079fb9d: 4589da                       mov r10d, r11d
14079fba0: 75d6                         jne 0x14079fb78
14079fba2: 66410f74cc                   pcmpeqb xmm1, xmm12
14079fba7: 66440fd7d1                   pmovmskb r10d, xmm1
14079fbac: 4585d2                       test r10d, r10d
14079fbaf: 0f8524040000                 jne 0x14079ffd9
14079fbb5: 4d01c8                       add r8, r9
14079fbb8: 4983c010                     add r8, 0x10
14079fbbc: 4983c110                     add r9, 0x10
14079fbc0: 4921d0                       and r8, rdx
14079fbc3: eb9b                         jmp 0x14079fb60
14079fbc5: 49f7db                       neg r11
14079fbc8: 49c1e304                     shl r11, 4
14079fbcc: f2420f107419f4               movsd xmm6, qword ptr [rcx + r11 - 0xc]
14079fbd3: 410f58f2                     addps xmm6, xmm10
14079fbd7: f3420f107c19fc               movss xmm7, dword ptr [rcx + r11 - 4]
14079fbde: f3410f58f9                   addss xmm7, xmm9
14079fbe3: 488b8df0000000               mov rcx, qword ptr [rbp + 0xf0]
14079fbea: 0f28ce                       movaps xmm1, xmm6
14079fbed: 0f28d7                       movaps xmm2, xmm7
14079fbf0: e85b2d3300                   call 0x140ad2950
14079fbf5: 4189c4                       mov r12d, eax
14079fbf8: 84c0                         test al, al
14079fbfa: 7440                         je 0x14079fc3c
14079fbfc: 488b8588040000               mov rax, qword ptr [rbp + 0x488]
14079fc03: 8b8d7c050000                 mov ecx, dword ptr [rbp + 0x57c]
14079fc09: 8908                         mov dword ptr [rax], ecx
14079fc0b: 0f13b5d0040000               movlps qword ptr [rbp + 0x4d0], xmm6
14079fc12: f30f11bdd8040000             movss dword ptr [rbp + 0x4d8], xmm7
14079fc1a: 488d8d80050000               lea rcx, [rbp + 0x580]
14079fc21: 488b9518030000               mov rdx, qword ptr [rbp + 0x318]
14079fc28: 4589f0                       mov r8d, r14d
14079fc2b: 4c8d8dd0040000               lea r9, [rbp + 0x4d0]
14079fc32: e8c963ac00                   call 0x141266000
14079fc37: e968010000                   jmp 0x14079fda4
14079fc3c: 488b85f8000000               mov rax, qword ptr [rbp + 0xf8]
14079fc43: 8b8df8010000                 mov ecx, dword ptr [rbp + 0x1f8]
14079fc49: 8908                         mov dword ptr [rax], ecx
14079fc4b: 488b8528020000               mov rax, qword ptr [rbp + 0x228]
14079fc52: 488b7830                     mov rdi, qword ptr [rax + 0x30]
14079fc56: 488b5840                     mov rbx, qword ptr [rax + 0x40]
14079fc5a: 483b7820                     cmp rdi, qword ptr [rax + 0x20]
14079fc5e: 7513                         jne 0x14079fc73
14079fc60: 488b8d08020000               mov rcx, qword ptr [rbp + 0x208]
14079fc67: 488d1512234802               lea rdx, [rip + 0x2482312]
14079fc6e: e8fd2bdcff                   call 0x140562870
14079fc73: 488b8d28020000               mov rcx, qword ptr [rbp + 0x228]
14079fc7a: 488b4128                     mov rax, qword ptr [rcx + 0x28]
14079fc7e: 48891cf8                     mov qword ptr [rax + rdi*8], rbx
14079fc82: 48ffc7                       inc rdi
14079fc85: 48897930                     mov qword ptr [rcx + 0x30], rdi
14079fc89: 48ff4140                     inc qword ptr [rcx + 0x40]
14079fc8d: 488b8500010000               mov rax, qword ptr [rbp + 0x100]
14079fc94: 8b8dfc010000                 mov ecx, dword ptr [rbp + 0x1fc]
14079fc9a: 8908                         mov dword ptr [rax], ecx
14079fc9c: 488b8530020000               mov rax, qword ptr [rbp + 0x230]
14079fca3: 488b7830                     mov rdi, qword ptr [rax + 0x30]
14079fca7: 488b5840                     mov rbx, qword ptr [rax + 0x40]
14079fcab: 483b7820                     cmp rdi, qword ptr [rax + 0x20]
14079fcaf: 7513                         jne 0x14079fcc4
14079fcb1: 488b8d10020000               mov rcx, qword ptr [rbp + 0x210]
14079fcb8: 488d15c1224802               lea rdx, [rip + 0x24822c1]
14079fcbf: e8ac2bdcff                   call 0x140562870
14079fcc4: 488b8d30020000               mov rcx, qword ptr [rbp + 0x230]
14079fccb: 488b4128                     mov rax, qword ptr [rcx + 0x28]
14079fccf: 48891cf8                     mov qword ptr [rax + rdi*8], rbx
14079fcd3: 48ffc7                       inc rdi
14079fcd6: 48897930                     mov qword ptr [rcx + 0x30], rdi
14079fcda: 48ff4140                     inc qword ptr [rcx + 0x40]
14079fcde: 488bbd58050000               mov rdi, qword ptr [rbp + 0x558]
14079fce5: 48837f4800                   cmp qword ptr [rdi + 0x48], 0
14079fcea: 0f84b4000000                 je 0x14079fda4
14079fcf0: 488b8d28030000               mov rcx, qword ptr [rbp + 0x328]
14079fcf7: 488d95f0020000               lea rdx, [rbp + 0x2f0]
14079fcfe: e8cd11e1ff                   call 0x1405b0ed0
14079fd03: 488b4f30                     mov rcx, qword ptr [rdi + 0x30]
14079fd07: 488b5738                     mov rdx, qword ptr [rdi + 0x38]
14079fd0b: 4989d0                       mov r8, rdx
14079fd0e: 4921c0                       and r8, rax
14079fd11: 48c1e839                     shr rax, 0x39
14079fd15: 660f6ec0                     movd xmm0, eax
14079fd19: 660f60c0                     punpcklbw xmm0, xmm0
14079fd1d: f20f70c000                   pshuflw xmm0, xmm0, 0
14079fd22: 660f70c000                   pshufd xmm0, xmm0, 0
14079fd27: 488d41f0                     lea rax, [rcx - 0x10]
14079fd2b: 4531c9                       xor r9d, r9d
14079fd2e: f3420f6f0c01                 movdqu xmm1, xmmword ptr [rcx + r8]
14079fd34: 660f6fd1                     movdqa xmm2, xmm1
14079fd38: 660f74d0                     pcmpeqb xmm2, xmm0
14079fd3c: 66440fd7d2                   pmovmskb r10d, xmm2
14079fd41: 4585d2                       test r10d, r10d
14079fd44: 742a                         je 0x14079fd70
14079fd46: f3450fbcda                   tzcnt r11d, r10d
14079fd4b: 4d01c3                       add r11, r8
14079fd4e: 4921d3                       and r11, rdx
14079fd51: 4c89df                       mov rdi, r11
14079fd54: 48c1e704                     shl rdi, 4
14079fd58: 4889c3                       mov rbx, rax
14079fd5b: 4829fb                       sub rbx, rdi
14079fd5e: 443b33                       cmp r14d, dword ptr [rbx]
14079fd61: 742c                         je 0x14079fd8f
14079fd63: 458d5aff                     lea r11d, [r10 - 1]
14079fd67: 664521d3                     and r11w, r10w
14079fd6b: 4589da                       mov r10d, r11d
14079fd6e: 75d6                         jne 0x14079fd46
14079fd70: 66410f74cc                   pcmpeqb xmm1, xmm12
14079fd75: 66440fd7d1                   pmovmskb r10d, xmm1
14079fd7a: 4585d2                       test r10d, r10d
14079fd7d: 7525                         jne 0x14079fda4
14079fd7f: 4d01c8                       add r8, r9
14079fd82: 4983c010                     add r8, 0x10
14079fd86: 4983c110                     add r9, 0x10
14079fd8a: 4921d0                       and r8, rdx
14079fd8d: eb9f                         jmp 0x14079fd2e
14079fd8f: 49f7db                       neg r11
14079fd92: 49c1e304                     shl r11, 4
14079fd96: f2420f107419f4               movsd xmm6, qword ptr [rcx + r11 - 0xc]
14079fd9d: f3420f107c19fc               movss xmm7, dword ptr [rcx + r11 - 4]
14079fda4: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
14079fdab: 4489f2                       mov edx, r14d
14079fdae: e82dc62700                   call 0x140a1c3e0
14079fdb3: 4989c7                       mov r15, rax
14079fdb6: 4889d3                       mov rbx, rdx
14079fdb9: e8b2bd89ff                   call 0x14003bb70
14079fdbe: 4088c7                       mov dil, al
14079fdc1: 488b9558060000               mov rdx, qword ptr [rbp + 0x658]
14079fdc8: 488b4208                     mov rax, qword ptr [rdx + 8]
14079fdcc: 488b4a10                     mov rcx, qword ptr [rdx + 0x10]
14079fdd0: 4801c9                       add rcx, rcx
14079fdd3: 48898580050000               mov qword ptr [rbp + 0x580], rax
14079fdda: 48898d88050000               mov qword ptr [rbp + 0x588], rcx
14079fde1: f20f104250                   movsd xmm0, qword ptr [rdx + 0x50]
14079fde6: f20f118590050000             movsd qword ptr [rbp + 0x590], xmm0
14079fdee: 488d8d80050000               lea rcx, [rbp + 0x580]
14079fdf5: 0f28ce                       movaps xmm1, xmm6
14079fdf8: 0f28d7                       movaps xmm2, xmm7
14079fdfb: e8a07dabff                   call 0x140257ba0
14079fe00: f30f16ce                     movshdup xmm1, xmm6
14079fe04: f30f5cc8                     subss xmm1, xmm0
14079fe08: f3410f5fcd                   maxss xmm1, xmm13
14079fe0d: 4c89bd80050000               mov qword ptr [rbp + 0x580], r15
14079fe14: 48899d88050000               mov qword ptr [rbp + 0x588], rbx
14079fe1b: 48c7859005000001000000       mov qword ptr [rbp + 0x590], 1
14079fe26: f30f11b5a0050000             movss dword ptr [rbp + 0x5a0], xmm6
14079fe2e: f30f11bda4050000             movss dword ptr [rbp + 0x5a4], xmm7
14079fe36: c785a805000000000000         mov dword ptr [rbp + 0x5a8], 0
14079fe40: c685c205000003               mov byte ptr [rbp + 0x5c2], 3
14079fe47: 4088bdc8050000               mov byte ptr [rbp + 0x5c8], dil
14079fe4e: c785d005000001000000         mov dword ptr [rbp + 0x5d0], 1
14079fe58: f30f118dd4050000             movss dword ptr [rbp + 0x5d4], xmm1
14079fe60: c785d805000000000000         mov dword ptr [rbp + 0x5d8], 0
14079fe6a: c685e005000000               mov byte ptr [rbp + 0x5e0], 0
14079fe71: c685e205000000               mov byte ptr [rbp + 0x5e2], 0
14079fe78: 66c785e40500000240           mov word ptr [rbp + 0x5e4], 0x4002
14079fe81: 4489b5e8050000               mov dword ptr [rbp + 0x5e8], r14d
14079fe88: c685ec0500003f               mov byte ptr [rbp + 0x5ec], 0x3f
14079fe8f: b002                         mov al, 2
14079fe91: 4584e4                       test r12b, r12b
14079fe94: 7411                         je 0x14079fea7
14079fe96: 0f13b5d0040000               movlps qword ptr [rbp + 0x4d0], xmm6
14079fe9d: f30f11bdd8040000             movss dword ptr [rbp + 0x4d8], xmm7
14079fea5: 31c0                         xor eax, eax
14079fea7: 80bd4806000002               cmp byte ptr [rbp + 0x648], 2
14079feae: 0f93c2                       setae dl
14079feb1: 8885dc040000                 mov byte ptr [rbp + 0x4dc], al
14079feb7: 488b8588020000               mov rax, qword ptr [rbp + 0x288]
14079febe: 8b8d38050000                 mov ecx, dword ptr [rbp + 0x538]
14079fec4: 8908                         mov dword ptr [rax], ecx
14079fec6: 488b8598020000               mov rax, qword ptr [rbp + 0x298]
14079fecd: 8b8d3c050000                 mov ecx, dword ptr [rbp + 0x53c]
14079fed3: 8908                         mov dword ptr [rax], ecx
14079fed5: 8b8560050000                 mov eax, dword ptr [rbp + 0x560]
14079fedb: 488b8d10030000               mov rcx, qword ptr [rbp + 0x310]
14079fee2: 8901                         mov dword ptr [rcx], eax
14079fee4: 488b85a0020000               mov rax, qword ptr [rbp + 0x2a0]
14079feeb: 4889442438                   mov qword ptr [rsp + 0x38], rax
14079fef0: 488d45c0                     lea rax, [rbp - 0x40]
14079fef4: 4889442430                   mov qword ptr [rsp + 0x30], rax
14079fef9: 488b8550050000               mov rax, qword ptr [rbp + 0x550]
14079ff00: 4889442428                   mov qword ptr [rsp + 0x28], rax
14079ff05: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
14079ff0c: 4889442420                   mov qword ptr [rsp + 0x20], rax
14079ff11: 488d8d80050000               lea rcx, [rbp + 0x580]
14079ff18: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
14079ff1f: 4c8b8d38060000               mov r9, qword ptr [rbp + 0x638]
14079ff26: e8056fad00                   call 0x141276e30
14079ff2b: 4584e4                       test r12b, r12b
14079ff2e: 0f84acfbffff                 je 0x14079fae0
14079ff34: 410f28c3                     movaps xmm0, xmm11
14079ff38: 488b8558050000               mov rax, qword ptr [rbp + 0x558]
14079ff3f: f30f5c8080000000             subss xmm0, dword ptr [rax + 0x80]
14079ff47: f30f59c0                     mulss xmm0, xmm0
14079ff4b: f30f10487c                   movss xmm1, dword ptr [rax + 0x7c]
14079ff50: f30f109084000000             movss xmm2, dword ptr [rax + 0x84]
14079ff58: 0f14ca                       unpcklps xmm1, xmm2
14079ff5b: 410f28d0                     movaps xmm2, xmm8
14079ff5f: 0f5cd1                       subps xmm2, xmm1
14079ff62: 0f59d2                       mulps xmm2, xmm2
14079ff65: f30f58c2                     addss xmm0, xmm2
14079ff69: f30f16ca                     movshdup xmm1, xmm2
14079ff6d: f30f58c8                     addss xmm1, xmm0
14079ff71: 0f57c0                       xorps xmm0, xmm0
14079ff74: f30f51c1                     sqrtss xmm0, xmm1
14079ff78: 488b8520030000               mov rax, qword ptr [rbp + 0x320]
14079ff7f: f30f5e4058                   divss xmm0, dword ptr [rax + 0x58]
14079ff84: 488b8510010000               mov rax, qword ptr [rbp + 0x110]
14079ff8b: 8b8d00020000                 mov ecx, dword ptr [rbp + 0x200]
14079ff91: 8908                         mov dword ptr [rax], ecx
14079ff93: 488b8508010000               mov rax, qword ptr [rbp + 0x108]
14079ff9a: f30f114020                   movss dword ptr [rax + 0x20], xmm0
14079ff9f: e93cfbffff                   jmp 0x14079fae0
14079ffa4: 83f801                       cmp eax, 1
14079ffa7: 488bb558050000               mov rsi, qword ptr [rbp + 0x558]
14079ffae: 0f8523020000                 jne 0x1407a01d7
14079ffb4: e887d75a00                   call 0x140d4d740
14079ffb9: 4989c7                       mov r15, rax
14079ffbc: f60001                       test byte ptr [rax], 1
14079ffbf: 0f843a060000                 je 0x1407a05ff
14079ffc5: 4c89f8                       mov rax, r15
14079ffc8: 4883c008                     add rax, 8
14079ffcc: 498b5f08                     mov rbx, qword ptr [r15 + 8]
14079ffd0: 4d8b7710                     mov r14, qword ptr [r15 + 0x10]
14079ffd4: e947060000                   jmp 0x1407a0620
14079ffd9: 488d0d50202e02               lea rcx, [rip + 0x22e2050]
14079ffe0: e82b931302                   call 0x1428d9310
14079ffe5: e9711d0000                   jmp 0x1407a1d5b
14079ffea: 488d85b8020000               lea rax, [rbp + 0x2b8]
14079fff1: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
14079fff8: 488d0511c95e00               lea rax, [rip + 0x5ec911]
14079ffff: 488985d8040000               mov qword ptr [rbp + 0x4d8], rax
1407a0006: 488d05c3364802               lea rax, [rip + 0x24836c3]
1407a000d: 48898580050000               mov qword ptr [rbp + 0x580], rax
1407a0014: 48c7858805000001000000       mov qword ptr [rbp + 0x588], 1
1407a001f: 48c785a005000000000000       mov qword ptr [rbp + 0x5a0], 0
1407a002a: 488d85d0040000               lea rax, [rbp + 0x4d0]
1407a0031: 48898590050000               mov qword ptr [rbp + 0x590], rax
1407a0038: 48c7859805000001000000       mov qword ptr [rbp + 0x598], 1
1407a0043: 488d1536232e02               lea rdx, [rip + 0x22e2336]
1407a004a: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a0051: e83a931302                   call 0x1428d9390
1407a0056: e9001d0000                   jmp 0x1407a1d5b
1407a005b: 488d0d7e364802               lea rcx, [rip + 0x248367e]
1407a0062: 4c8d0517232e02               lea r8, [rip + 0x22e2317]
1407a0069: ba1d000000                   mov edx, 0x1d
1407a006e: e8bd931302                   call 0x1428d9430
1407a0073: e9e31c0000                   jmp 0x1407a1d5b
1407a0078: 488d85b8020000               lea rax, [rbp + 0x2b8]
1407a007f: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
1407a0086: 488d0583c85e00               lea rax, [rip + 0x5ec883]
1407a008d: 488985d8040000               mov qword ptr [rbp + 0x4d8], rax
1407a0094: 488d0535364802               lea rax, [rip + 0x2483635]
1407a009b: 48898580050000               mov qword ptr [rbp + 0x580], rax
1407a00a2: 48c7858805000001000000       mov qword ptr [rbp + 0x588], 1
1407a00ad: 48c785a005000000000000       mov qword ptr [rbp + 0x5a0], 0
1407a00b8: 488d85d0040000               lea rax, [rbp + 0x4d0]
1407a00bf: 48898590050000               mov qword ptr [rbp + 0x590], rax
1407a00c6: 48c7859805000001000000       mov qword ptr [rbp + 0x598], 1
1407a00d1: 488d15c0222e02               lea rdx, [rip + 0x22e22c0]
1407a00d8: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a00df: e8ac921302                   call 0x1428d9390
1407a00e4: e9721c0000                   jmp 0x1407a1d5b
1407a00e9: 488d0df0354802               lea rcx, [rip + 0x24835f0]
1407a00f0: 4c8d05a1222e02               lea r8, [rip + 0x22e22a1]
1407a00f7: ba1d000000                   mov edx, 0x1d
1407a00fc: e82f931302                   call 0x1428d9430
1407a0101: e9551c0000                   jmp 0x1407a1d5b
1407a0106: f30f103d360d2702             movss xmm7, dword ptr [rip + 0x2270d36]
1407a010e: 4c8b8538020000               mov r8, qword ptr [rbp + 0x238]
1407a0115: 4c8db5e4030000               lea r14, [rbp + 0x3e4]
1407a011c: 488b8518010000               mov rax, qword ptr [rbp + 0x118]
1407a0123: 8b8d04020000                 mov ecx, dword ptr [rbp + 0x204]
1407a0129: 8908                         mov dword ptr [rax], ecx
1407a012b: 498b7030                     mov rsi, qword ptr [r8 + 0x30]
1407a012f: 4d8b6840                     mov r13, qword ptr [r8 + 0x40]
1407a0133: 493b7020                     cmp rsi, qword ptr [r8 + 0x20]
1407a0137: 7517                         jne 0x1407a0150
1407a0139: 498d4820                     lea rcx, [r8 + 0x20]
1407a013d: 488d153c1e4802               lea rdx, [rip + 0x2481e3c]
1407a0144: e8272adcff                   call 0x140562b70
1407a0149: 4c8b8538020000               mov r8, qword ptr [rbp + 0x238]
1407a0150: 498b4028                     mov rax, qword ptr [r8 + 0x28]
1407a0154: 488d0c76                     lea rcx, [rsi + rsi*2]
1407a0158: 48c1e105                     shl rcx, 5
1407a015c: 4c892c08                     mov qword ptr [rax + rcx], r13
1407a0160: 48895c0808                   mov qword ptr [rax + rcx + 8], rbx
1407a0165: c644081004                   mov byte ptr [rax + rcx + 0x10], 4
1407a016a: 897c0814                     mov dword ptr [rax + rcx + 0x14], edi
1407a016e: 4c897c0818                   mov qword ptr [rax + rcx + 0x18], r15
1407a0173: 4c89640820                   mov qword ptr [rax + rcx + 0x20], r12
1407a0178: 498b16                       mov rdx, qword ptr [r14]
1407a017b: 4889540828                   mov qword ptr [rax + rcx + 0x28], rdx
1407a0180: 418b5608                     mov edx, dword ptr [r14 + 8]
1407a0184: 89540830                     mov dword ptr [rax + rcx + 0x30], edx
1407a0188: 48ba000000000000803f         movabs rdx, 0x3f80000000000000
1407a0192: 4889540834                   mov qword ptr [rax + rcx + 0x34], rdx
1407a0197: 48c744083c00000000           mov qword ptr [rax + rcx + 0x3c], 0
1407a01a0: 897c0850                     mov dword ptr [rax + rcx + 0x50], edi
1407a01a4: f30f117c0854                 movss dword ptr [rax + rcx + 0x54], xmm7
1407a01aa: c64408583f                   mov byte ptr [rax + rcx + 0x58], 0x3f
1407a01af: 48ffc6                       inc rsi
1407a01b2: 49897030                     mov qword ptr [r8 + 0x30], rsi
1407a01b6: 49ff4040                     inc qword ptr [r8 + 0x40]
1407a01ba: 488db580050000               lea rsi, [rbp + 0x580]
1407a01c1: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
1407a01c8: f3410f104648                 movss xmm0, dword ptr [r14 + 0x48]
1407a01ce: 0f2ec6                       ucomiss xmm0, xmm6
1407a01d1: 0f874cf3ffff                 ja 0x14079f523
1407a01d7: f3410f104e3c                 movss xmm1, dword ptr [r14 + 0x3c]
1407a01dd: 0f57c0                       xorps xmm0, xmm0
1407a01e0: 0f2ec8                       ucomiss xmm1, xmm0
1407a01e3: 0f86b20c0000                 jbe 0x1407a0e9b
1407a01e9: 488d0d581e2e02               lea rcx, [rip + 0x22e1e58]
1407a01f0: e8fb5c4f00                   call 0x140c95ef0
1407a01f5: 498b460c                     mov rax, qword ptr [r14 + 0xc]
1407a01f9: 48898570050000               mov qword ptr [rbp + 0x570], rax
1407a0200: 418b4614                     mov eax, dword ptr [r14 + 0x14]
1407a0204: 898578050000                 mov dword ptr [rbp + 0x578], eax
1407a020a: 498b4618                     mov rax, qword ptr [r14 + 0x18]
1407a020e: 488985a0010000               mov qword ptr [rbp + 0x1a0], rax
1407a0215: 418b4620                     mov eax, dword ptr [r14 + 0x20]
1407a0219: 8985a8010000                 mov dword ptr [rbp + 0x1a8], eax
1407a021f: 498b4624                     mov rax, qword ptr [r14 + 0x24]
1407a0223: 48898590010000               mov qword ptr [rbp + 0x190], rax
1407a022a: 418b462c                     mov eax, dword ptr [r14 + 0x2c]
1407a022e: 898598010000                 mov dword ptr [rbp + 0x198], eax
1407a0234: f30f590554d92402             mulss xmm0, dword ptr [rip + 0x224d954]
1407a023c: f30f118530030000             movss dword ptr [rbp + 0x330], xmm0
1407a0244: 488d8570050000               lea rax, [rbp + 0x570]
1407a024b: 488985c0030000               mov qword ptr [rbp + 0x3c0], rax
1407a0252: 488d0577b55a00               lea rax, [rip + 0x5ab577]
1407a0259: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
1407a0260: 488d8530030000               lea rax, [rbp + 0x330]
1407a0267: 488985d0030000               mov qword ptr [rbp + 0x3d0], rax
1407a026e: 488d059bc65e00               lea rax, [rip + 0x5ec69b]
1407a0275: 488985d8030000               mov qword ptr [rbp + 0x3d8], rax
1407a027c: 488d05dd1d2e02               lea rax, [rip + 0x22e1ddd]
1407a0283: 48898580050000               mov qword ptr [rbp + 0x580], rax
1407a028a: 48c7858805000002000000       mov qword ptr [rbp + 0x588], 2
1407a0295: 488d85c0030000               lea rax, [rbp + 0x3c0]
1407a029c: 48898590050000               mov qword ptr [rbp + 0x590], rax
1407a02a3: 48c7859805000002000000       mov qword ptr [rbp + 0x598], 2
1407a02ae: 48c785a005000000000000       mov qword ptr [rbp + 0x5a0], 0
1407a02b9: 488d4d48                     lea rcx, [rbp + 0x48]
1407a02bd: 488d9580050000               lea rdx, [rbp + 0x580]
1407a02c4: e8c79c5d00                   call 0x140d79f90
1407a02c9: 488b4548                     mov rax, qword ptr [rbp + 0x48]
1407a02cd: 48898568060000               mov qword ptr [rbp + 0x668], rax
1407a02d4: 488b7d50                     mov rdi, qword ptr [rbp + 0x50]
1407a02d8: 488b7558                     mov rsi, qword ptr [rbp + 0x58]
1407a02dc: 8b8578050000                 mov eax, dword ptr [rbp + 0x578]
1407a02e2: 898588050000                 mov dword ptr [rbp + 0x588], eax
1407a02e8: 488b8570050000               mov rax, qword ptr [rbp + 0x570]
1407a02ef: 48898580050000               mov qword ptr [rbp + 0x580], rax
1407a02f6: f30f10b530030000             movss xmm6, dword ptr [rbp + 0x330]
1407a02fe: 0fb6050449e902               movzx eax, byte ptr [rip + 0x2e94904]
1407a0305: b920000000                   mov ecx, 0x20
1407a030a: ba08000000                   mov edx, 8
1407a030f: e8fc38e7ff                   call 0x140613c10
1407a0314: 4885c0                       test rax, rax
1407a0317: 4889bd58060000               mov qword ptr [rbp + 0x658], rdi
1407a031e: 0f84d1190000                 je 0x1407a1cf5
1407a0324: 48c70001000000               mov qword ptr [rax], 1
1407a032b: 48c7400801000000             mov qword ptr [rax + 8], 1
1407a0333: 488b8d80050000               mov rcx, qword ptr [rbp + 0x580]
1407a033a: 48894810                     mov qword ptr [rax + 0x10], rcx
1407a033e: 8b8d88050000                 mov ecx, dword ptr [rbp + 0x588]
1407a0344: 894818                       mov dword ptr [rax + 0x18], ecx
1407a0347: f30f11701c                   movss dword ptr [rax + 0x1c], xmm6
1407a034c: 488d0d2d1d2e02               lea rcx, [rip + 0x22e1d2d]
1407a0353: 48894c2420                   mov qword ptr [rsp + 0x20], rcx
1407a0358: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a035f: 4889fa                       mov rdx, rdi
1407a0362: 4989f0                       mov r8, rsi
1407a0365: 4989c1                       mov r9, rax
1407a0368: e843d50f00                   call 0x14089d8b0
1407a036d: 488b85a0050000               mov rax, qword ptr [rbp + 0x5a0]
1407a0374: 488985e0030000               mov qword ptr [rbp + 0x3e0], rax
1407a037b: 0f108580050000               movups xmm0, xmmword ptr [rbp + 0x580]
1407a0382: 0f108d90050000               movups xmm1, xmmword ptr [rbp + 0x590]
1407a0389: 0f298dd0030000               movaps xmmword ptr [rbp + 0x3d0], xmm1
1407a0390: 0f2985c0030000               movaps xmmword ptr [rbp + 0x3c0], xmm0
1407a0397: 488d95c0030000               lea rdx, [rbp + 0x3c0]
1407a039e: 488b8d68050000               mov rcx, qword ptr [rbp + 0x568]
1407a03a5: e8e6cb3800                   call 0x140b2cf90
1407a03aa: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
1407a03b1: 4885d2                       test rdx, rdx
1407a03b4: 7412                         je 0x1407a03c8
1407a03b6: 41b801000000                 mov r8d, 1
1407a03bc: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
1407a03c3: e85838e7ff                   call 0x140613c20
1407a03c8: 0fb6053a48e902               movzx eax, byte ptr [rip + 0x2e9483a]
1407a03cf: b918000000                   mov ecx, 0x18
1407a03d4: ba04000000                   mov edx, 4
1407a03d9: e83238e7ff                   call 0x140613c10
1407a03de: 4885c0                       test rax, rax
1407a03e1: 0f841f190000                 je 0x1407a1d06
1407a03e7: 8b8da8010000                 mov ecx, dword ptr [rbp + 0x1a8]
1407a03ed: 894808                       mov dword ptr [rax + 8], ecx
1407a03f0: 488b8da0010000               mov rcx, qword ptr [rbp + 0x1a0]
1407a03f7: 488908                       mov qword ptr [rax], rcx
1407a03fa: 488b8d90010000               mov rcx, qword ptr [rbp + 0x190]
1407a0401: 4889480c                     mov qword ptr [rax + 0xc], rcx
1407a0405: 8b8d98010000                 mov ecx, dword ptr [rbp + 0x198]
1407a040b: 894814                       mov dword ptr [rax + 0x14], ecx
1407a040e: 48c785c003000002000000       mov qword ptr [rbp + 0x3c0], 2
1407a0419: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
1407a0420: 48c785d003000002000000       mov qword ptr [rbp + 0x3d0], 2
1407a042b: 488db580050000               lea rsi, [rbp + 0x580]
1407a0432: 488d95c0030000               lea rdx, [rbp + 0x3c0]
1407a0439: 4889f1                       mov rcx, rsi
1407a043c: e82f4a3600                   call 0x140b04e70
1407a0441: 488dbdc0030000               lea rdi, [rbp + 0x3c0]
1407a0448: f30f1015a0e21802             movss xmm2, dword ptr [rip + 0x218e2a0]
1407a0450: 4889f9                       mov rcx, rdi
1407a0453: 4889f2                       mov rdx, rsi
1407a0456: e8e54a3600                   call 0x140b04f40
1407a045b: 4889f1                       mov rcx, rsi
1407a045e: 4889fa                       mov rdx, rdi
1407a0461: e8aa4a3600                   call 0x140b04f10
1407a0466: 4889f1                       mov rcx, rsi
1407a0469: 488b9568050000               mov rdx, qword ptr [rbp + 0x568]
1407a0470: e8fb4a3600                   call 0x140b04f70
1407a0475: c785c40300000000003f         mov dword ptr [rbp + 0x3c4], 0x3f000000
1407a047f: 488b8570050000               mov rax, qword ptr [rbp + 0x570]
1407a0486: 488985c8030000               mov qword ptr [rbp + 0x3c8], rax
1407a048d: 8b8578050000                 mov eax, dword ptr [rbp + 0x578]
1407a0493: 8985d0030000                 mov dword ptr [rbp + 0x3d0], eax
1407a0499: c685c003000000               mov byte ptr [rbp + 0x3c0], 0
1407a04a0: c685d004000000               mov byte ptr [rbp + 0x4d0], 0
1407a04a7: 488d95c0030000               lea rdx, [rbp + 0x3c0]
1407a04ae: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
1407a04b5: 4889f1                       mov rcx, rsi
1407a04b8: e853892000                   call 0x1409a8e10
1407a04bd: 808d2906000002               or byte ptr [rbp + 0x629], 2
1407a04c4: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
1407a04cb: 41b8b0000000                 mov r8d, 0xb0
1407a04d1: 4889f2                       mov rdx, rsi
1407a04d4: e827371802                   call 0x142923c00
1407a04d9: c6855506000001               mov byte ptr [rbp + 0x655], 1
1407a04e0: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a04e7: e8c48a2000                   call 0x1409a8fb0
1407a04ec: 48c7859804000000000000       mov qword ptr [rbp + 0x498], 0
1407a04f7: c7859004000001000000         mov dword ptr [rbp + 0x490], 1
1407a0501: c6855506000001               mov byte ptr [rbp + 0x655], 1
1407a0508: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
1407a050f: 488d9580050000               lea rdx, [rbp + 0x580]
1407a0516: 4c8d8590040000               lea r8, [rbp + 0x490]
1407a051d: e8ae8a2000                   call 0x1409a8fd0
1407a0522: c6855506000000               mov byte ptr [rbp + 0x655], 0
1407a0529: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a0530: 488d95c0030000               lea rdx, [rbp + 0x3c0]
1407a0537: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
1407a053e: e83d892000                   call 0x1409a8e80
1407a0543: 488db5c0030000               lea rsi, [rbp + 0x3c0]
1407a054a: 488d9580050000               lea rdx, [rbp + 0x580]
1407a0551: 4889f1                       mov rcx, rsi
1407a0554: 4531c0                       xor r8d, r8d
1407a0557: e884892000                   call 0x1409a8ee0
1407a055c: 488d154d1b2e02               lea rdx, [rip + 0x22e1b4d]
1407a0563: 41b813000000                 mov r8d, 0x13
1407a0569: 488b8d78040000               mov rcx, qword ptr [rbp + 0x478]
1407a0570: 4989f1                       mov r9, rsi
1407a0573: e858311800                   call 0x1409236d0
1407a0578: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
1407a057f: 488995d8040000               mov qword ptr [rbp + 0x4d8], rdx
1407a0586: 41837e3401                   cmp dword ptr [r14 + 0x34], 1
1407a058b: 0f850a090000                 jne 0x1407a0e9b
1407a0591: 418b7638                     mov esi, dword ptr [r14 + 0x38]
1407a0595: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
1407a059c: e86fd91800                   call 0x14092df10
1407a05a1: 888580050000                 mov byte ptr [rbp + 0x580], al
1407a05a7: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a05ae: e80de11800                   call 0x14092e6c0
1407a05b3: 84c0                         test al, al
1407a05b5: 0f84e0080000                 je 0x1407a0e9b
1407a05bb: 4c89f0                       mov rax, r14
1407a05be: 41be03000000                 mov r14d, 3
1407a05c4: 833801                       cmp dword ptr [rax], 1
1407a05c7: 0f85ca050000                 jne 0x1407a0b97
1407a05cd: 8b5804                       mov ebx, dword ptr [rax + 4]
1407a05d0: 8b7808                       mov edi, dword ptr [rax + 8]
1407a05d3: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
1407a05da: 89da                         mov edx, ebx
1407a05dc: e84fc32700                   call 0x140a1c930
1407a05e1: 4885c0                       test rax, rax
1407a05e4: 0f848d020000                 je 0x1407a0877
1407a05ea: 488b4808                     mov rcx, qword ptr [rax + 8]
1407a05ee: 48898d68060000               mov qword ptr [rbp + 0x668], rcx
1407a05f5: 440fb66022                   movzx r12d, byte ptr [rax + 0x22]
1407a05fa: e97b020000                   jmp 0x1407a087a
1407a05ff: e8fc475c00                   call 0x140d64e00
1407a0604: 4889c3                       mov rbx, rax
1407a0607: 4989d6                       mov r14, rdx
1407a060a: 4c89f8                       mov rax, r15
1407a060d: 4883c008                     add rax, 8
1407a0611: 49c70701000000               mov qword ptr [r15], 1
1407a0618: 49895f08                     mov qword ptr [r15 + 8], rbx
1407a061c: 49895710                     mov qword ptr [r15 + 0x10], rdx
1407a0620: 488d4b01                     lea rcx, [rbx + 1]
1407a0624: 488908                       mov qword ptr [rax], rcx
1407a0627: 488b8588040000               mov rax, qword ptr [rbp + 0x488]
1407a062e: 8b8d7c050000                 mov ecx, dword ptr [rbp + 0x57c]
1407a0634: 8908                         mov dword ptr [rax], ecx
1407a0636: 488b5608                     mov rdx, qword ptr [rsi + 8]
1407a063a: 4885d2                       test rdx, rdx
1407a063d: 7428                         je 0x1407a0667
1407a063f: 4889d0                       mov rax, rdx
1407a0642: 48c1e004                     shl rax, 4
1407a0646: 4801c2                       add rdx, rax
1407a0649: 4883c221                     add rdx, 0x21
1407a064d: 7418                         je 0x1407a0667
1407a064f: 488b0e                       mov rcx, qword ptr [rsi]
1407a0652: 48f7d8                       neg rax
1407a0655: 4801c1                       add rcx, rax
1407a0658: 4883c1f0                     add rcx, -0x10
1407a065c: 41b810000000                 mov r8d, 0x10
1407a0662: e8b935e7ff                   call 0x140613c20
1407a0667: 0f1005c2162e02               movups xmm0, xmmword ptr [rip + 0x22e16c2]
1407a066e: 0f114610                     movups xmmword ptr [rsi + 0x10], xmm0
1407a0672: 0f1005a7162e02               movups xmm0, xmmword ptr [rip + 0x22e16a7]
1407a0679: 0f1106                       movups xmmword ptr [rsi], xmm0
1407a067c: 48895e20                     mov qword ptr [rsi + 0x20], rbx
1407a0680: 4c897628                     mov qword ptr [rsi + 0x28], r14
1407a0684: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a068b: 488b95b8030000               mov rdx, qword ptr [rbp + 0x3b8]
1407a0692: 4189f8                       mov r8d, edi
1407a0695: e826e21500                   call 0x1408fe8c0
1407a069a: 488b8580050000               mov rax, qword ptr [rbp + 0x580]
1407a06a1: 48898558060000               mov qword ptr [rbp + 0x658], rax
1407a06a8: 488b8588050000               mov rax, qword ptr [rbp + 0x588]
1407a06af: 48898568060000               mov qword ptr [rbp + 0x668], rax
1407a06b6: 4c8bbd90050000               mov r15, qword ptr [rbp + 0x590]
1407a06bd: 4d85ff                       test r15, r15
1407a06c0: 746b                         je 0x1407a072d
1407a06c2: 49c1e702                     shl r15, 2
1407a06c6: 4531e4                       xor r12d, r12d
1407a06c9: 488d3d48192e02               lea rdi, [rip + 0x22e1948]
1407a06d0: 488d9dc0030000               lea rbx, [rbp + 0x3c0]
1407a06d7: 4c8db580050000               lea r14, [rbp + 0x580]
1407a06de: 6690                         nop 
1407a06e0: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
1407a06e7: 428b3420                     mov esi, dword ptr [rax + r12]
1407a06eb: 488b8588040000               mov rax, qword ptr [rbp + 0x488]
1407a06f2: 8b8d7c050000                 mov ecx, dword ptr [rbp + 0x57c]
1407a06f8: 8908                         mov dword ptr [rax], ecx
1407a06fa: 4889d9                       mov rcx, rbx
1407a06fd: 488b9520020000               mov rdx, qword ptr [rbp + 0x220]
1407a0704: 4189f0                       mov r8d, esi
1407a0707: 4989f9                       mov r9, rdi
1407a070a: e8e1a22700                   call 0x140a1a9f0
1407a070f: 4c89f1                       mov rcx, r14
1407a0712: 488b9558050000               mov rdx, qword ptr [rbp + 0x558]
1407a0719: 4189f0                       mov r8d, esi
1407a071c: 4989d9                       mov r9, rbx
1407a071f: e8dc58ac00                   call 0x141266000
1407a0724: 4983c404                     add r12, 4
1407a0728: 4d39e7                       cmp r15, r12
1407a072b: 75b3                         jne 0x1407a06e0
1407a072d: 488b9558060000               mov rdx, qword ptr [rbp + 0x658]
1407a0734: 4885d2                       test rdx, rdx
1407a0737: 7416                         je 0x1407a074f
1407a0739: 48c1e202                     shl rdx, 2
1407a073d: 41b804000000                 mov r8d, 4
1407a0743: 488b8d68060000               mov rcx, qword ptr [rbp + 0x668]
1407a074a: e8d134e7ff                   call 0x140613c20
1407a074f: f30f109594030000             movss xmm2, dword ptr [rbp + 0x394]
1407a0757: 488bb588040000               mov rsi, qword ptr [rbp + 0x488]
1407a075e: 8bbd7c050000                 mov edi, dword ptr [rbp + 0x57c]
1407a0764: 893e                         mov dword ptr [rsi], edi
1407a0766: 488b9d58050000               mov rbx, qword ptr [rbp + 0x558]
1407a076d: f30f11536c                   movss dword ptr [rbx + 0x6c], xmm2
1407a0772: 488b85c0070000               mov rax, qword ptr [rbp + 0x7c0]
1407a0779: 488b10                       mov rdx, qword ptr [rax]
1407a077c: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a0783: e8f8e91400                   call 0x1408ef180
1407a0788: 893e                         mov dword ptr [rsi], edi
1407a078a: 8b8588050000                 mov eax, dword ptr [rbp + 0x588]
1407a0790: 894368                       mov dword ptr [rbx + 0x68], eax
1407a0793: 488b8580050000               mov rax, qword ptr [rbp + 0x580]
1407a079a: 48894360                     mov qword ptr [rbx + 0x60], rax
1407a079e: f30f104368                   movss xmm0, dword ptr [rbx + 0x68]
1407a07a3: f30f5c8598030000             subss xmm0, dword ptr [rbp + 0x398]
1407a07ab: f20f104b60                   movsd xmm1, qword ptr [rbx + 0x60]
1407a07b0: f20f109590030000             movsd xmm2, qword ptr [rbp + 0x390]
1407a07b8: 0f5cca                       subps xmm1, xmm2
1407a07bb: 893e                         mov dword ptr [rsi], edi
1407a07bd: 0f134b70                     movlps qword ptr [rbx + 0x70], xmm1
1407a07c1: f30f114378                   movss dword ptr [rbx + 0x78], xmm0
1407a07c6: 0f107320                     movups xmm6, xmmword ptr [rbx + 0x20]
1407a07ca: 488b7b08                     mov rdi, qword ptr [rbx + 8]
1407a07ce: 4885ff                       test rdi, rdi
1407a07d1: 0f848c000000                 je 0x1407a0863
1407a07d7: 488d7701                     lea rsi, [rdi + 1]
1407a07db: 4889f0                       mov rax, rsi
1407a07de: 48c1e83c                     shr rax, 0x3c
1407a07e2: 7523                         jne 0x1407a0807
1407a07e4: 48c1e604                     shl rsi, 4
1407a07e8: 4c8d7f11                     lea r15, [rdi + 0x11]
1407a07ec: 4889f3                       mov rbx, rsi
1407a07ef: 4c01fb                       add rbx, r15
1407a07f2: 7213                         jb 0x1407a0807
1407a07f4: 48b8f1ffffffffffff7f         movabs rax, 0x7ffffffffffffff1
1407a07fe: 4839c3                       cmp rbx, rax
1407a0801: 0f82e1010000                 jb 0x1407a09e8
1407a0807: b101                         mov cl, 1
1407a0809: e8728b5d00                   call 0x140d79380
1407a080e: e908020000                   jmp 0x1407a0a1b
1407a0813: 488b95b0030000               mov rdx, qword ptr [rbp + 0x3b0]
1407a081a: 4885d2                       test rdx, rdx
1407a081d: 7416                         je 0x1407a0835
1407a081f: 48c1e202                     shl rdx, 2
1407a0823: 41b804000000                 mov r8d, 4
1407a0829: 488b8d80040000               mov rcx, qword ptr [rbp + 0x480]
1407a0830: e8eb33e7ff                   call 0x140613c20
1407a0835: 488b8588040000               mov rax, qword ptr [rbp + 0x488]
1407a083c: 8b8d7c050000                 mov ecx, dword ptr [rbp + 0x57c]
1407a0842: 8908                         mov dword ptr [rax], ecx
1407a0844: 8b85e8010000                 mov eax, dword ptr [rbp + 0x1e8]
1407a084a: 488b8d18020000               mov rcx, qword ptr [rbp + 0x218]
1407a0851: 894108                       mov dword ptr [rcx + 8], eax
1407a0854: 488b85e0010000               mov rax, qword ptr [rbp + 0x1e0]
1407a085b: 488901                       mov qword ptr [rcx], rax
1407a085e: e9a3020000                   jmp 0x1407a0b06
1407a0863: 4889de                       mov rsi, rbx
1407a0866: 0f57ff                       xorps xmm7, xmm7
1407a0869: 4c8d3500184802               lea r14, [rip + 0x2481800]
1407a0870: 31db                         xor ebx, ebx
1407a0872: e9fe010000                   jmp 0x1407a0a75
1407a0877: 41b405                       mov r12b, 5
1407a087a: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
1407a0881: 89fa                         mov edx, edi
1407a0883: e8a8c02700                   call 0x140a1c930
1407a0888: 4885c0                       test rax, rax
1407a088b: 0f8406030000                 je 0x1407a0b97
1407a0891: 4180fc05                     cmp r12b, 5
1407a0895: 0f84fc020000                 je 0x1407a0b97
1407a089b: 440fb66822                   movzx r13d, byte ptr [rax + 0x22]
1407a08a0: 4180fd05                     cmp r13b, 5
1407a08a4: 0f84ed020000                 je 0x1407a0b97
1407a08aa: 4c8bbd38060000               mov r15, qword ptr [rbp + 0x638]
1407a08b1: 4c89f9                       mov rcx, r15
1407a08b4: 89da                         mov edx, ebx
1407a08b6: e825bb2700                   call 0x140a1c3e0
1407a08bb: 488985f8020000               mov qword ptr [rbp + 0x2f8], rax
1407a08c2: 48899500030000               mov qword ptr [rbp + 0x300], rdx
1407a08c9: 4c89f9                       mov rcx, r15
1407a08cc: 89fa                         mov edx, edi
1407a08ce: e80dbb2700                   call 0x140a1c3e0
1407a08d3: 418d4c24fe                   lea ecx, [r12 - 2]
1407a08d8: 440fb6c1                     movzx r8d, cl
1407a08dc: 4d8d4801                     lea r9, [r8 + 1]
1407a08e0: 31c9                         xor ecx, ecx
1407a08e2: 4180f803                     cmp r8b, 3
1407a08e6: 490f42c9                     cmovb rcx, r9
1407a08ea: 4883f901                     cmp rcx, 1
1407a08ee: 0f8430020000                 je 0x1407a0b24
1407a08f4: 4885c9                       test rcx, rcx
1407a08f7: 0f859a020000                 jne 0x1407a0b97
1407a08fd: 4180c5fb                     add r13b, 0xfb
1407a0901: 4180fdfc                     cmp r13b, 0xfc
1407a0905: 0f878c020000                 ja 0x1407a0b97
1407a090b: 483985f8020000               cmp qword ptr [rbp + 0x2f8], rax
1407a0912: 0f857f020000                 jne 0x1407a0b97
1407a0918: 4885c0                       test rax, rax
1407a091b: 750d                         jne 0x1407a092a
1407a091d: 48399500030000               cmp qword ptr [rbp + 0x300], rdx
1407a0924: 0f856d020000                 jne 0x1407a0b97
1407a092a: 488b85e0070000               mov rax, qword ptr [rbp + 0x7e0]
1407a0931: 488b38                       mov rdi, qword ptr [rax]
1407a0934: 488d8df8020000               lea rcx, [rbp + 0x2f8]
1407a093b: e800b92700                   call 0x140a1c240
1407a0940: 4c8d0581172e02               lea r8, [rip + 0x22e1781]
1407a0947: 4889f9                       mov rcx, rdi
1407a094a: 4889c2                       mov rdx, rax
1407a094d: e89e152f00                   call 0x140a91ef0
1407a0952: 4885c0                       test rax, rax
1407a0955: 0f843c020000                 je 0x1407a0b97
1407a095b: 8b8d78050000                 mov ecx, dword ptr [rbp + 0x578]
1407a0961: 898d88050000                 mov dword ptr [rbp + 0x588], ecx
1407a0967: 488b8d70050000               mov rcx, qword ptr [rbp + 0x570]
1407a096e: 48898d80050000               mov qword ptr [rbp + 0x580], rcx
1407a0975: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a097c: 4889c2                       mov rdx, rax
1407a097f: 4889c7                       mov rdi, rax
1407a0982: e829164f00                   call 0x140c91fb0
1407a0987: 0f28f0                       movaps xmm6, xmm0
1407a098a: 0f28f9                       movaps xmm7, xmm1
1407a098d: 4889f9                       mov rcx, rdi
1407a0990: 0f28c8                       movaps xmm1, xmm0
1407a0993: 0f28d7                       movaps xmm2, xmm7
1407a0996: e8f59e3800                   call 0x140b2a890
1407a099b: 440f28cf                     movaps xmm9, xmm7
1407a099f: f3440f5cc8                   subss xmm9, xmm0
1407a09a4: e8c71986ff                   call 0x140002370
1407a09a9: 488b8d70050000               mov rcx, qword ptr [rbp + 0x570]
1407a09b0: 48898d90040000               mov qword ptr [rbp + 0x490], rcx
1407a09b7: 8b8d78050000                 mov ecx, dword ptr [rbp + 0x578]
1407a09bd: 898d98040000                 mov dword ptr [rbp + 0x498], ecx
1407a09c3: 4180e401                     and r12b, 1
1407a09c7: 4c8bb5f8020000               mov r14, qword ptr [rbp + 0x2f8]
1407a09ce: 488bbd00030000               mov rdi, qword ptr [rbp + 0x300]
1407a09d5: 0f14f7                       unpcklps xmm6, xmm7
1407a09d8: 450f57c0                     xorps xmm8, xmm8
1407a09dc: 488b8d68060000               mov rcx, qword ptr [rbp + 0x668]
1407a09e3: e9af010000                   jmp 0x1407a0b97
1407a09e8: 0fb6051a42e902               movzx eax, byte ptr [rip + 0x2e9421a]
1407a09ef: ba10000000                   mov edx, 0x10
1407a09f4: 4889d9                       mov rcx, rbx
1407a09f7: e81432e7ff                   call 0x140613c10
1407a09fc: 4885c0                       test rax, rax
1407a09ff: 740b                         je 0x1407a0a0c
1407a0a01: 4989c6                       mov r14, rax
1407a0a04: 4901f6                       add r14, rsi
1407a0a07: 4889fb                       mov rbx, rdi
1407a0a0a: eb1c                         jmp 0x1407a0a28
1407a0a0c: ba10000000                   mov edx, 0x10
1407a0a11: b101                         mov cl, 1
1407a0a13: 4989d8                       mov r8, rbx
1407a0a16: e8b5895d00                   call 0x140d793d0
1407a0a1b: 4889c3                       mov rbx, rax
1407a0a1e: 4989c7                       mov r15, rax
1407a0a21: 4983c711                     add r15, 0x11
1407a0a25: 4531f6                       xor r14d, r14d
1407a0a28: 488b8558050000               mov rax, qword ptr [rbp + 0x558]
1407a0a2f: 488b30                       mov rsi, qword ptr [rax]
1407a0a32: 4c89f1                       mov rcx, r14
1407a0a35: 4889f2                       mov rdx, rsi
1407a0a38: 4d89f8                       mov r8, r15
1407a0a3b: e8c0311802                   call 0x142923c00
1407a0a40: 48f7d7                       not rdi
1407a0a43: 48c1e704                     shl rdi, 4
1407a0a47: 4801f7                       add rdi, rsi
1407a0a4a: 488bb558050000               mov rsi, qword ptr [rbp + 0x558]
1407a0a51: 4889d9                       mov rcx, rbx
1407a0a54: 48f7d1                       not rcx
1407a0a57: 48c1e104                     shl rcx, 4
1407a0a5b: 4c01f1                       add rcx, r14
1407a0a5e: 4989d8                       mov r8, rbx
1407a0a61: 49c1e004                     shl r8, 4
1407a0a65: 4983c010                     add r8, 0x10
1407a0a69: 4889fa                       mov rdx, rdi
1407a0a6c: e88f311802                   call 0x142923c00
1407a0a71: 0f107e10                     movups xmm7, xmmword ptr [rsi + 0x10]
1407a0a75: 4889f7                       mov rdi, rsi
1407a0a78: 4883c660                     add rsi, 0x60
1407a0a7c: 488b8588040000               mov rax, qword ptr [rbp + 0x488]
1407a0a83: 8b8d7c050000                 mov ecx, dword ptr [rbp + 0x57c]
1407a0a89: 8908                         mov dword ptr [rax], ecx
1407a0a8b: 488b5738                     mov rdx, qword ptr [rdi + 0x38]
1407a0a8f: 4885d2                       test rdx, rdx
1407a0a92: 7429                         je 0x1407a0abd
1407a0a94: 4889d0                       mov rax, rdx
1407a0a97: 48c1e004                     shl rax, 4
1407a0a9b: 4801c2                       add rdx, rax
1407a0a9e: 4883c221                     add rdx, 0x21
1407a0aa2: 7419                         je 0x1407a0abd
1407a0aa4: 488b4f30                     mov rcx, qword ptr [rdi + 0x30]
1407a0aa8: 48f7d8                       neg rax
1407a0aab: 4801c1                       add rcx, rax
1407a0aae: 4883c1f0                     add rcx, -0x10
1407a0ab2: 41b810000000                 mov r8d, 0x10
1407a0ab8: e86331e7ff                   call 0x140613c20
1407a0abd: 4c897730                     mov qword ptr [rdi + 0x30], r14
1407a0ac1: 48895f38                     mov qword ptr [rdi + 0x38], rbx
1407a0ac5: 0f117f40                     movups xmmword ptr [rdi + 0x40], xmm7
1407a0ac9: 0f117750                     movups xmmword ptr [rdi + 0x50], xmm6
1407a0acd: 8b4608                       mov eax, dword ptr [rsi + 8]
1407a0ad0: 898588050000                 mov dword ptr [rbp + 0x588], eax
1407a0ad6: 488b06                       mov rax, qword ptr [rsi]
1407a0ad9: 48898580050000               mov qword ptr [rbp + 0x580], rax
1407a0ae0: 488b8588040000               mov rax, qword ptr [rbp + 0x488]
1407a0ae7: 8b8d7c050000                 mov ecx, dword ptr [rbp + 0x57c]
1407a0aed: 8908                         mov dword ptr [rax], ecx
1407a0aef: 8b8588050000                 mov eax, dword ptr [rbp + 0x588]
1407a0af5: 898784000000                 mov dword ptr [rdi + 0x84], eax
1407a0afb: 488b8580050000               mov rax, qword ptr [rbp + 0x580]
1407a0b02: 4889477c                     mov qword ptr [rdi + 0x7c], rax
1407a0b06: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
1407a0b0d: f3410f104e3c                 movss xmm1, dword ptr [r14 + 0x3c]
1407a0b13: 0f57c0                       xorps xmm0, xmm0
1407a0b16: 0f2ec8                       ucomiss xmm1, xmm0
1407a0b19: 0f87caf6ffff                 ja 0x1407a01e9
1407a0b1f: e977030000                   jmp 0x1407a0e9b
1407a0b24: 4180fd02                     cmp r13b, 2
1407a0b28: 756d                         jne 0x1407a0b97
1407a0b2a: 483985f8020000               cmp qword ptr [rbp + 0x2f8], rax
1407a0b31: 7564                         jne 0x1407a0b97
1407a0b33: 4885c0                       test rax, rax
1407a0b36: 7509                         jne 0x1407a0b41
1407a0b38: 48399500030000               cmp qword ptr [rbp + 0x300], rdx
1407a0b3f: 7556                         jne 0x1407a0b97
1407a0b41: 488d8df8020000               lea rcx, [rbp + 0x2f8]
1407a0b48: e8f3b62700                   call 0x140a1c240
1407a0b4d: f30f109570050000             movss xmm2, dword ptr [rbp + 0x570]
1407a0b55: f30f109d78050000             movss xmm3, dword ptr [rbp + 0x578]
1407a0b5d: 488b8dd8070000               mov rcx, qword ptr [rbp + 0x7d8]
1407a0b64: 488b09                       mov rcx, qword ptr [rcx]
1407a0b67: 488b95e0070000               mov rdx, qword ptr [rbp + 0x7e0]
1407a0b6e: 488b12                       mov rdx, qword ptr [rdx]
1407a0b71: 4889542428                   mov qword ptr [rsp + 0x28], rdx
1407a0b76: 48894c2420                   mov qword ptr [rsp + 0x20], rcx
1407a0b7b: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a0b82: 4889c2                       mov rdx, rax
1407a0b85: e806351400                   call 0x1408e4090
1407a0b8a: 80bd8005000000               cmp byte ptr [rbp + 0x580], 0
1407a0b91: 0f847c100000                 je 0x1407a1c13
1407a0b97: 4c89b5c0030000               mov qword ptr [rbp + 0x3c0], r14
1407a0b9e: 4889bdc8030000               mov qword ptr [rbp + 0x3c8], rdi
1407a0ba5: 0f13b5d0030000               movlps qword ptr [rbp + 0x3d0], xmm6
1407a0bac: 48898dd8030000               mov qword ptr [rbp + 0x3d8], rcx
1407a0bb3: f3440f118de0030000           movss dword ptr [rbp + 0x3e0], xmm9
1407a0bbc: 488b8d90040000               mov rcx, qword ptr [rbp + 0x490]
1407a0bc3: 48898de4030000               mov qword ptr [rbp + 0x3e4], rcx
1407a0bca: 8b8d98040000                 mov ecx, dword ptr [rbp + 0x498]
1407a0bd0: 898dec030000                 mov dword ptr [rbp + 0x3ec], ecx
1407a0bd6: 8885f0030000                 mov byte ptr [rbp + 0x3f0], al
1407a0bdc: c685f103000000               mov byte ptr [rbp + 0x3f1], 0
1407a0be3: 4488a5f2030000               mov byte ptr [rbp + 0x3f2], r12b
1407a0bea: f30f10b578050000             movss xmm6, dword ptr [rbp + 0x578]
1407a0bf2: f30f10bd70050000             movss xmm7, dword ptr [rbp + 0x570]
1407a0bfa: f3440f108d74050000           movss xmm9, dword ptr [rbp + 0x574]
1407a0c03: 488b85a8070000               mov rax, qword ptr [rbp + 0x7a8]
1407a0c0a: 488b00                       mov rax, qword ptr [rax]
1407a0c0d: 488b4808                     mov rcx, qword ptr [rax + 8]
1407a0c11: 488b5010                     mov rdx, qword ptr [rax + 0x10]
1407a0c15: 4801d2                       add rdx, rdx
1407a0c18: 48898d80050000               mov qword ptr [rbp + 0x580], rcx
1407a0c1f: 48899588050000               mov qword ptr [rbp + 0x588], rdx
1407a0c26: f20f104050                   movsd xmm0, qword ptr [rax + 0x50]
1407a0c2b: f20f118590050000             movsd qword ptr [rbp + 0x590], xmm0
1407a0c33: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a0c3a: 0f28cf                       movaps xmm1, xmm7
1407a0c3d: 0f28d6                       movaps xmm2, xmm6
1407a0c40: e85b6fabff                   call 0x140257ba0
1407a0c45: 48c7858005000001000000       mov qword ptr [rbp + 0x580], 1
1407a0c50: f30f11bd90050000             movss dword ptr [rbp + 0x590], xmm7
1407a0c58: f30f11b594050000             movss dword ptr [rbp + 0x594], xmm6
1407a0c60: c7859805000000000000         mov dword ptr [rbp + 0x598], 0
1407a0c6a: c685b205000003               mov byte ptr [rbp + 0x5b2], 3
1407a0c71: 4983fe03                     cmp r14, 3
1407a0c75: 751c                         jne 0x1407a0c93
1407a0c77: 488d8590050000               lea rax, [rbp + 0x590]
1407a0c7e: b901000000                   mov ecx, 1
1407a0c83: 48898d68060000               mov qword ptr [rbp + 0x668], rcx
1407a0c8a: 4c8bb588050000               mov r14, qword ptr [rbp + 0x588]
1407a0c91: eb11                         jmp 0x1407a0ca4
1407a0c93: 4c89b568060000               mov qword ptr [rbp + 0x668], r14
1407a0c9a: 4989fe                       mov r14, rdi
1407a0c9d: 488d85d0030000               lea rax, [rbp + 0x3d0]
1407a0ca4: 488bbda0070000               mov rdi, qword ptr [rbp + 0x7a0]
1407a0cab: 0f1008                       movups xmm1, xmmword ptr [rax]
1407a0cae: 0f105010                     movups xmm2, xmmword ptr [rax + 0x10]
1407a0cb2: 0f294d00                     movaps xmmword ptr [rbp], xmm1
1407a0cb6: 0f295510                     movaps xmmword ptr [rbp + 0x10], xmm2
1407a0cba: 488b4020                     mov rax, qword ptr [rax + 0x20]
1407a0cbe: 48894520                     mov qword ptr [rbp + 0x20], rax
1407a0cc2: 7509                         jne 0x1407a0ccd
1407a0cc4: f3440f5cc8                   subss xmm9, xmm0
1407a0cc9: 450f28c1                     movaps xmm8, xmm9
1407a0ccd: 488b8580020000               mov rax, qword ptr [rbp + 0x280]
1407a0cd4: 8b8df4020000                 mov ecx, dword ptr [rbp + 0x2f4]
1407a0cda: 8908                         mov dword ptr [rax], ecx
1407a0cdc: 488b8db8010000               mov rcx, qword ptr [rbp + 0x1b8]
1407a0ce3: 488b95b0010000               mov rdx, qword ptr [rbp + 0x1b0]
1407a0cea: e881552e00                   call 0x140a86270
1407a0cef: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
1407a0cf6: 89f2                         mov edx, esi
1407a0cf8: e833bc2700                   call 0x140a1c930
1407a0cfd: 4885c0                       test rax, rax
1407a0d00: 742d                         je 0x1407a0d2f
1407a0d02: 4883c030                     add rax, 0x30
1407a0d06: 4889c1                       mov rcx, rax
1407a0d09: e882870900                   call 0x140839490
1407a0d0e: 488b8d30060000               mov rcx, qword ptr [rbp + 0x630]
1407a0d15: f30f107130                   movss xmm6, dword ptr [rcx + 0x30]
1407a0d1a: 4885c0                       test rax, rax
1407a0d1d: 7426                         je 0x1407a0d45
1407a0d1f: 440fb67810                   movzx r15d, byte ptr [rax + 0x10]
1407a0d24: 0fb65811                     movzx ebx, byte ptr [rax + 0x11]
1407a0d28: f20f107808                   movsd xmm7, qword ptr [rax + 8]
1407a0d2d: eb1e                         jmp 0x1407a0d4d
1407a0d2f: 488b8d30060000               mov rcx, qword ptr [rbp + 0x630]
1407a0d36: f30f107130                   movss xmm6, dword ptr [rcx + 0x30]
1407a0d3b: 0f57ff                       xorps xmm7, xmm7
1407a0d3e: 31db                         xor ebx, ebx
1407a0d40: 4531ff                       xor r15d, r15d
1407a0d43: eb0f                         jmp 0x1407a0d54
1407a0d45: 0f57ff                       xorps xmm7, xmm7
1407a0d48: 31db                         xor ebx, ebx
1407a0d4a: 4531ff                       xor r15d, r15d
1407a0d4d: 488b8d30060000               mov rcx, qword ptr [rbp + 0x630]
1407a0d54: 8b4108                       mov eax, dword ptr [rcx + 8]
1407a0d57: 8985c8030000                 mov dword ptr [rbp + 0x3c8], eax
1407a0d5d: 488b01                       mov rax, qword ptr [rcx]
1407a0d60: 488985c0030000               mov qword ptr [rbp + 0x3c0], rax
1407a0d67: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
1407a0d6e: 89f2                         mov edx, esi
1407a0d70: e8bbbb2700                   call 0x140a1c930
1407a0d75: 4885c0                       test rax, rax
1407a0d78: 7405                         je 0x1407a0d7f
1407a0d7a: 8b7028                       mov esi, dword ptr [rax + 0x28]
1407a0d7d: eb05                         jmp 0x1407a0d84
1407a0d7f: bea3860100                   mov esi, 0x186a3
1407a0d84: 8b8578050000                 mov eax, dword ptr [rbp + 0x578]
1407a0d8a: 89858a050000                 mov dword ptr [rbp + 0x58a], eax
1407a0d90: 488b8570050000               mov rax, qword ptr [rbp + 0x570]
1407a0d97: 48898582050000               mov qword ptr [rbp + 0x582], rax
1407a0d9e: 488b85c0030000               mov rax, qword ptr [rbp + 0x3c0]
1407a0da5: 48898590040000               mov qword ptr [rbp + 0x490], rax
1407a0dac: 8b85c8030000                 mov eax, dword ptr [rbp + 0x3c8]
1407a0db2: 898598040000                 mov dword ptr [rbp + 0x498], eax
1407a0db8: 8b471c                       mov eax, dword ptr [rdi + 0x1c]
1407a0dbb: 488b4f10                     mov rcx, qword ptr [rdi + 0x10]
1407a0dbf: 8901                         mov dword ptr [rcx], eax
1407a0dc1: 4c8b27                       mov r12, qword ptr [rdi]
1407a0dc4: 4d8b6c2430                   mov r13, qword ptr [r12 + 0x30]
1407a0dc9: 498b7c2440                   mov rdi, qword ptr [r12 + 0x40]
1407a0dce: 4d3b6c2420                   cmp r13, qword ptr [r12 + 0x20]
1407a0dd3: 7511                         jne 0x1407a0de6
1407a0dd5: 498d4c2420                   lea rcx, [r12 + 0x20]
1407a0dda: 488d159f114802               lea rdx, [rip + 0x248119f]
1407a0de1: e84aa40800                   call 0x14082b230
1407a0de6: 498b442428                   mov rax, qword ptr [r12 + 0x28]
1407a0deb: 496bcd78                     imul rcx, r13, 0x78
1407a0def: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
1407a0df6: 48891408                     mov qword ptr [rax + rcx], rdx
1407a0dfa: 4c89740808                   mov qword ptr [rax + rcx + 8], r14
1407a0dff: 0f284500                     movaps xmm0, xmmword ptr [rbp]
1407a0e03: 0f284d10                     movaps xmm1, xmmword ptr [rbp + 0x10]
1407a0e07: 0f11440810                   movups xmmword ptr [rax + rcx + 0x10], xmm0
1407a0e0c: 0f114c0820                   movups xmmword ptr [rax + rcx + 0x20], xmm1
1407a0e11: 488b5520                     mov rdx, qword ptr [rbp + 0x20]
1407a0e15: 4889540830                   mov qword ptr [rax + rcx + 0x30], rdx
1407a0e1a: 488b9590040000               mov rdx, qword ptr [rbp + 0x490]
1407a0e21: 4889540838                   mov qword ptr [rax + rcx + 0x38], rdx
1407a0e26: 8b9598040000                 mov edx, dword ptr [rbp + 0x498]
1407a0e2c: 89540840                     mov dword ptr [rax + rcx + 0x40], edx
1407a0e30: f3440f11440844               movss dword ptr [rax + rcx + 0x44], xmm8
1407a0e37: f30f11740848                 movss dword ptr [rax + rcx + 0x48], xmm6
1407a0e3d: 0f137c084c                   movlps qword ptr [rax + rcx + 0x4c], xmm7
1407a0e42: 44887c0854                   mov byte ptr [rax + rcx + 0x54], r15b
1407a0e47: 885c0855                     mov byte ptr [rax + rcx + 0x55], bl
1407a0e4b: 488b9580050000               mov rdx, qword ptr [rbp + 0x580]
1407a0e52: 4c8b8586050000               mov r8, qword ptr [rbp + 0x586]
1407a0e59: 4889540856                   mov qword ptr [rax + rcx + 0x56], rdx
1407a0e5e: 4c8944085c                   mov qword ptr [rax + rcx + 0x5c], r8
1407a0e63: 89740864                     mov dword ptr [rax + rcx + 0x64], esi
1407a0e67: c64408683f                   mov byte ptr [rax + rcx + 0x68], 0x3f
1407a0e6c: 8b9540020000                 mov edx, dword ptr [rbp + 0x240]
1407a0e72: 448b8543020000               mov r8d, dword ptr [rbp + 0x243]
1407a0e79: 89540869                     mov dword ptr [rax + rcx + 0x69], edx
1407a0e7d: 448944086c                   mov dword ptr [rax + rcx + 0x6c], r8d
1407a0e82: 48897c0870                   mov qword ptr [rax + rcx + 0x70], rdi
1407a0e87: 49ffc5                       inc r13
1407a0e8a: 4d896c2430                   mov qword ptr [r12 + 0x30], r13
1407a0e8f: 49ff442440                   inc qword ptr [r12 + 0x40]
1407a0e94: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
1407a0e9b: 4d8bae30010000               mov r13, qword ptr [r14 + 0x130]
1407a0ea2: 498b8e48010000               mov rcx, qword ptr [r14 + 0x148]
1407a0ea9: 66410f6f4500                 movdqa xmm0, xmmword ptr [r13]
1407a0eaf: 66440fd7c0                   pmovmskb r8d, xmm0
1407a0eb4: 41f7d0                       not r8d
1407a0eb7: 498d7510                     lea rsi, [r13 + 0x10]
1407a0ebb: 488b8590070000               mov rax, qword ptr [rbp + 0x790]
1407a0ec2: 8b501c                       mov edx, dword ptr [rax + 0x1c]
1407a0ec5: 899580040000                 mov dword ptr [rbp + 0x480], edx
1407a0ecb: 488b10                       mov rdx, qword ptr [rax]
1407a0ece: 48899528030000               mov qword ptr [rbp + 0x328], rdx
1407a0ed5: 488b4010                     mov rax, qword ptr [rax + 0x10]
1407a0ed9: 48898580020000               mov qword ptr [rbp + 0x280], rax
1407a0ee0: 4c8d35d90d2e02               lea r14, [rip + 0x22e0dd9]
1407a0ee7: 440f283d81c22202             movaps xmm15, xmmword ptr [rip + 0x222c281]
1407a0eef: 440f282d89c22202             movaps xmm13, xmmword ptr [rip + 0x222c289]
1407a0ef7: 0f283d92c22202               movaps xmm7, xmmword ptr [rip + 0x222c292]
1407a0efe: 0f2835cb0d2e02               movaps xmm6, xmmword ptr [rip + 0x22e0dcb]
1407a0f05: 440f2805d30d2e02             movaps xmm8, xmmword ptr [rip + 0x22e0dd3]
1407a0f0d: eb0f                         jmp 0x1407a0f1e
1407a0f0f: 90                           nop 
1407a0f10: 488b8d10030000               mov rcx, qword ptr [rbp + 0x310]
1407a0f17: 4c8b85b0030000               mov r8, qword ptr [rbp + 0x3b0]
1407a0f1e: 4883e901                     sub rcx, 1
1407a0f22: 0f82890c0000                 jb 0x1407a1bb1
1407a0f28: 664585c0                     test r8w, r8w
1407a0f2c: 48898d10030000               mov qword ptr [rbp + 0x310], rcx
1407a0f33: 740b                         je 0x1407a0f40
1407a0f35: 418d40ff                     lea eax, [r8 - 1]
1407a0f39: 4421c0                       and eax, r8d
1407a0f3c: eb2f                         jmp 0x1407a0f6d
1407a0f3e: 6690                         nop 
1407a0f40: 660f6f06                     movdqa xmm0, xmmword ptr [rsi]
1407a0f44: 66440fd7c0                   pmovmskb r8d, xmm0
1407a0f49: 4981c500faffff               add r13, -0x600
1407a0f50: 4883c610                     add rsi, 0x10
1407a0f54: 4181f8ffff0000               cmp r8d, 0xffff
1407a0f5b: 74e3                         je 0x1407a0f40
1407a0f5d: b9feffffff                   mov ecx, 0xfffffffe
1407a0f62: 4429c1                       sub ecx, r8d
1407a0f65: 41f7d0                       not r8d
1407a0f68: 4489c0                       mov eax, r8d
1407a0f6b: 21c8                         and eax, ecx
1407a0f6d: 4489c1                       mov ecx, r8d
1407a0f70: f30fbcc9                     tzcnt ecx, ecx
1407a0f74: 488985b0030000               mov qword ptr [rbp + 0x3b0], rax
1407a0f7b: 48f7d9                       neg rcx
1407a0f7e: 488d3c49                     lea rdi, [rcx + rcx*2]
1407a0f82: 48c1e705                     shl rdi, 5
1407a0f86: 418b443da0                   mov eax, dword ptr [r13 + rdi - 0x60]
1407a0f8b: 48898550050000               mov qword ptr [rbp + 0x550], rax
1407a0f92: f3450f104c3df4               movss xmm9, dword ptr [r13 + rdi - 0xc]
1407a0f99: 0fb605693ce902               movzx eax, byte ptr [rip + 0x2e93c69]
1407a0fa0: b960030000                   mov ecx, 0x360
1407a0fa5: ba04000000                   mov edx, 4
1407a0faa: e8612ce7ff                   call 0x140613c10
1407a0faf: 4885c0                       test rax, rax
1407a0fb2: 0f84a10c0000                 je 0x1407a1c59
1407a0fb8: 4889c1                       mov rcx, rax
1407a0fbb: 4a8d1c2f                     lea rbx, [rdi + r13]
1407a0fbf: 4a8d042f                     lea rax, [rdi + r13]
1407a0fc3: 4883c0c0                     add rax, -0x40
1407a0fc7: 48898518030000               mov qword ptr [rbp + 0x318], rax
1407a0fce: 498d443de8                   lea rax, [r13 + rdi - 0x18]
1407a0fd3: 48898520030000               mov qword ptr [rbp + 0x320], rax
1407a0fda: f3440f590d8d481802           mulss xmm9, dword ptr [rip + 0x218488d]
1407a0fe3: f3440f580d84491802           addss xmm9, dword ptr [rip + 0x2184984]
1407a0fec: 48c7853003000018000000       mov qword ptr [rbp + 0x330], 0x18
1407a0ff7: 48898d58060000               mov qword ptr [rbp + 0x658], rcx
1407a0ffe: 48898d38030000               mov qword ptr [rbp + 0x338], rcx
1407a1005: 48c7854003000000000000       mov qword ptr [rbp + 0x340], 0
1407a1010: 410f28c1                     movaps xmm0, xmm9
1407a1014: f30f590574f61b02             mulss xmm0, dword ptr [rip + 0x21bf674]
1407a101c: 440f28d8                     movaps xmm11, xmm0
1407a1020: 450f14d9                     unpcklps xmm11, xmm9
1407a1024: 450f28e3                     movaps xmm12, xmm11
1407a1028: 440f16e0                     movlhps xmm12, xmm0
1407a102c: 440fc6d804                   shufps xmm11, xmm0, 4
1407a1031: 450fc6e4d2                   shufps xmm12, xmm12, 0xd2
1407a1036: 48c7856806000000000000       mov qword ptr [rbp + 0x668], 0
1407a1041: 31c0                         xor eax, eax
1407a1043: 666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
1407a1050: 83f802                       cmp eax, 2
1407a1053: 0f8317060000                 jae 0x1407a1670
1407a1059: 4189c7                       mov r15d, eax
1407a105c: 8985ec010000                 mov dword ptr [rbp + 0x1ec], eax
1407a1062: 31c0                         xor eax, eax
1407a1064: 4585ff                       test r15d, r15d
1407a1067: 0f94c0                       sete al
1407a106a: f3450f101486                 movss xmm10, dword ptr [r14 + rax*4]
1407a1070: 410f28c2                     movaps xmm0, xmm10
1407a1074: e8572f1802                   call 0x142923fd0
1407a1079: 440f28c8                     movaps xmm9, xmm0
1407a107d: 410f28c2                     movaps xmm0, xmm10
1407a1081: e8fa2d1802                   call 0x142923e80
1407a1086: 0f284bb0                     movaps xmm1, xmmword ptr [rbx - 0x50]
1407a108a: 450fc6c900                   shufps xmm9, xmm9, 0
1407a108f: 410f28d1                     movaps xmm2, xmm9
1407a1093: 0f541556481802               andps xmm2, xmmword ptr [rip + 0x2184856]
1407a109a: 0fc6c000                     shufps xmm0, xmm0, 0
1407a109e: 0f59c1                       mulps xmm0, xmm1
1407a10a1: 0f28d9                       movaps xmm3, xmm1
1407a10a4: 0fc6d91b                     shufps xmm3, xmm1, 0x1b
1407a10a8: 0fc6c9b1                     shufps xmm1, xmm1, 0xb1
1407a10ac: 0f59ca                       mulps xmm1, xmm2
1407a10af: 0f59d3                       mulps xmm2, xmm3
1407a10b2: 0fc6dbb1                     shufps xmm3, xmm3, 0xb1
1407a10b6: 410f59d7                     mulps xmm2, xmm15
1407a10ba: 0f58d0                       addps xmm2, xmm0
1407a10bd: 410f59d9                     mulps xmm3, xmm9
1407a10c1: 410f59dd                     mulps xmm3, xmm13
1407a10c5: 0f59cf                       mulps xmm1, xmm7
1407a10c8: 0f58cb                       addps xmm1, xmm3
1407a10cb: 0f58ca                       addps xmm1, xmm2
1407a10ce: 0f28c1                       movaps xmm0, xmm1
1407a10d1: 0fc6c100                     shufps xmm0, xmm1, 0
1407a10d5: 0f28d1                       movaps xmm2, xmm1
1407a10d8: 0fc6d155                     shufps xmm2, xmm1, 0x55
1407a10dc: 440f28c9                     movaps xmm9, xmm1
1407a10e0: 440fc6c9aa                   shufps xmm9, xmm1, 0xaa
1407a10e5: 0fc6c9ff                     shufps xmm1, xmm1, 0xff
1407a10e9: 0f59ce                       mulps xmm1, xmm6
1407a10ec: 0f59c6                       mulps xmm0, xmm6
1407a10ef: 410f59c7                     mulps xmm0, xmm15
1407a10f3: 0f58c1                       addps xmm0, xmm1
1407a10f6: 410f59d0                     mulps xmm2, xmm8
1407a10fa: 410f59d5                     mulps xmm2, xmm13
1407a10fe: 450f59c8                     mulps xmm9, xmm8
1407a1102: 440f59cf                     mulps xmm9, xmm7
1407a1106: 440f58ca                     addps xmm9, xmm2
1407a110a: 440f58c8                     addps xmm9, xmm0
1407a110e: 410f28c1                     movaps xmm0, xmm9
1407a1112: 410f59c1                     mulps xmm0, xmm9
1407a1116: f30f16c8                     movshdup xmm1, xmm0
1407a111a: f30f58c8                     addss xmm1, xmm0
1407a111e: 0f28d0                       movaps xmm2, xmm0
1407a1121: 660f15d0                     unpckhpd xmm2, xmm0
1407a1125: f30f58d1                     addss xmm2, xmm1
1407a1129: 0fc6d200                     shufps xmm2, xmm2, 0
1407a112d: 0fc6c0ff                     shufps xmm0, xmm0, 0xff
1407a1131: 0f5cc2                       subps xmm0, xmm2
1407a1134: 410f28cb                     movaps xmm1, xmm11
1407a1138: 0f59c8                       mulps xmm1, xmm0
1407a113b: 410f28e3                     movaps xmm4, xmm11
1407a113f: 410f59e1                     mulps xmm4, xmm9
1407a1143: f30f16d4                     movshdup xmm2, xmm4
1407a1147: f30f58d4                     addss xmm2, xmm4
1407a114b: 0f12e4                       movhlps xmm4, xmm4
1407a114e: f30f58e2                     addss xmm4, xmm2
1407a1152: f30f58e4                     addss xmm4, xmm4
1407a1156: 0fc6e400                     shufps xmm4, xmm4, 0
1407a115a: 410f59e1                     mulps xmm4, xmm9
1407a115e: 0f58e1                       addps xmm4, xmm1
1407a1161: 410f28d9                     movaps xmm3, xmm9
1407a1165: 410fc6d9d2                   shufps xmm3, xmm9, 0xd2
1407a116a: 410f28cb                     movaps xmm1, xmm11
1407a116e: 0f59cb                       mulps xmm1, xmm3
1407a1171: 410f28d4                     movaps xmm2, xmm12
1407a1175: 410f59d1                     mulps xmm2, xmm9
1407a1179: 0f5cca                       subps xmm1, xmm2
1407a117c: 0fc6c9d2                     shufps xmm1, xmm1, 0xd2
1407a1180: 410f28d1                     movaps xmm2, xmm9
1407a1184: 410f58d1                     addps xmm2, xmm9
1407a1188: 0fc6d2ff                     shufps xmm2, xmm2, 0xff
1407a118c: 0f59ca                       mulps xmm1, xmm2
1407a118f: 0f58cc                       addps xmm1, xmm4
1407a1192: f20f1063e8                   movsd xmm4, qword ptr [rbx - 0x18]
1407a1197: 0f58e1                       addps xmm4, xmm1
1407a119a: 0f12c9                       movhlps xmm1, xmm1
1407a119d: f30f584bf0                   addss xmm1, dword ptr [rbx - 0x10]
1407a11a2: 0f13a5d0010000               movlps qword ptr [rbp + 0x1d0], xmm4
1407a11a9: f30f118dd8010000             movss dword ptr [rbp + 0x1d8], xmm1
1407a11b1: 837bd801                     cmp dword ptr [rbx - 0x28], 1
1407a11b5: 0f8508010000                 jne 0x1407a12c3
1407a11bb: 0f28e8                       movaps xmm5, xmm0
1407a11be: 0f592ddb471802               mulps xmm5, xmmword ptr [rip + 0x21847db]
1407a11c5: 450f28d1                     movaps xmm10, xmm9
1407a11c9: 440f5915df331902             mulps xmm10, xmmword ptr [rip + 0x21933df]
1407a11d1: f3450f16f2                   movshdup xmm14, xmm10
1407a11d6: f3450f58f2                   addss xmm14, xmm10
1407a11db: 450f12d2                     movhlps xmm10, xmm10
1407a11df: f3450f58d6                   addss xmm10, xmm14
1407a11e4: f3450f58d2                   addss xmm10, xmm10
1407a11e9: 450fc6d200                   shufps xmm10, xmm10, 0
1407a11ee: 450f59d1                     mulps xmm10, xmm9
1407a11f2: 440f58d5                     addps xmm10, xmm5
1407a11f6: 0f28eb                       movaps xmm5, xmm3
1407a11f9: 0f592db0331902               mulps xmm5, xmmword ptr [rip + 0x21933b0]
1407a1200: 450f28f1                     movaps xmm14, xmm9
1407a1204: 440f5935e40a2e02             mulps xmm14, xmmword ptr [rip + 0x22e0ae4]
1407a120c: 440f58f5                     addps xmm14, xmm5
1407a1210: 450fc6f6d6                   shufps xmm14, xmm14, 0xd6
1407a1215: 440f59f2                     mulps xmm14, xmm2
1407a1219: 450f58f2                     addps xmm14, xmm10
1407a121d: 410f28ee                     movaps xmm5, xmm14
1407a1221: 410fc6eee8                   shufps xmm5, xmm14, 0xe8
1407a1226: 450f59f6                     mulps xmm14, xmm14
1407a122a: 450f28d6                     movaps xmm10, xmm14
1407a122e: 66450f15d6                   unpckhpd xmm10, xmm14
1407a1233: f3450f58d6                   addss xmm10, xmm14
1407a1238: f3450f51d2                   sqrtss xmm10, xmm10
1407a123d: f3440f1035be461802           movss xmm14, dword ptr [rip + 0x21846be]
1407a1246: f3450f5ef2                   divss xmm14, xmm10
1407a124b: 66440f7ef0                   movd eax, xmm14
1407a1250: 85c0                         test eax, eax
1407a1252: 0f99c1                       setns cl
1407a1255: 89c2                         mov edx, eax
1407a1257: 81e2ffffff7f                 and edx, 0x7fffffff
1407a125d: 81c2000080ff                 add edx, 0xff800000
1407a1263: 81fa0000007f                 cmp edx, 0x7f000000
1407a1269: 0f92c2                       setb dl
1407a126c: 20ca                         and dl, cl
1407a126e: ffc8                         dec eax
1407a1270: 3dffff7f00                   cmp eax, 0x7fffff
1407a1275: 0f92c0                       setb al
1407a1278: 08d0                         or al, dl
1407a127a: f2440f1053dc                 movsd xmm10, qword ptr [rbx - 0x24]
1407a1280: f3450f12f6                   movsldup xmm14, xmm14
1407a1285: 440f59f5                     mulps xmm14, xmm5
1407a1289: 0fb6c0                       movzx eax, al
1407a128c: 660f6ee8                     movd xmm5, eax
1407a1290: 660f70ed50                   pshufd xmm5, xmm5, 0x50
1407a1295: 660f72f51f                   pslld xmm5, 0x1f
1407a129a: 660f72e51f                   psrad xmm5, 0x1f
1407a129f: 66410fdbee                   pand xmm5, xmm14
1407a12a4: 410f59ea                     mulps xmm5, xmm10
1407a12a8: f3440f16d5                   movshdup xmm10, xmm5
1407a12ad: f3440f58d5                   addss xmm10, xmm5
1407a12b2: 418d4701                     lea eax, [r15 + 1]
1407a12b6: 0f57ed                       xorps xmm5, xmm5
1407a12b9: 410f2eea                     ucomiss xmm5, xmm10
1407a12bd: 0f878dfdffff                 ja 0x1407a1050
1407a12c3: f2440f103514fa2d02           movsd xmm14, qword ptr [rip + 0x22dfa14]
1407a12cc: 410f59c6                     mulps xmm0, xmm14
1407a12d0: 410f28e9                     movaps xmm5, xmm9
1407a12d4: 410f59ee                     mulps xmm5, xmm14
1407a12d8: f3440f16d5                   movshdup xmm10, xmm5
1407a12dd: f3440f58d5                   addss xmm10, xmm5
1407a12e2: 0f12ed                       movhlps xmm5, xmm5
1407a12e5: f3410f58ea                   addss xmm5, xmm10
1407a12ea: f30f58ed                     addss xmm5, xmm5
1407a12ee: 0fc6ed00                     shufps xmm5, xmm5, 0
1407a12f2: 410f59e9                     mulps xmm5, xmm9
1407a12f6: 0f58e8                       addps xmm5, xmm0
1407a12f9: 410f59de                     mulps xmm3, xmm14
1407a12fd: 410f28c1                     movaps xmm0, xmm9
1407a1301: 0f5905f8092e02               mulps xmm0, xmmword ptr [rip + 0x22e09f8]
1407a1308: 0f58c3                       addps xmm0, xmm3
1407a130b: 0fc6c0d2                     shufps xmm0, xmm0, 0xd2
1407a130f: 0f59d0                       mulps xmm2, xmm0
1407a1312: 0f58d5                       addps xmm2, xmm5
1407a1315: 0f28c2                       movaps xmm0, xmm2
1407a1318: 660f15c2                     unpckhpd xmm0, xmm2
1407a131c: f30f58c1                     addss xmm0, xmm1
1407a1320: 0f58d4                       addps xmm2, xmm4
1407a1323: 0f139590050000               movlps qword ptr [rbp + 0x590], xmm2
1407a132a: f30f118598050000             movss dword ptr [rbp + 0x598], xmm0
1407a1332: 440f298d80050000             movaps xmmword ptr [rbp + 0x580], xmm9
1407a133a: 48b80000803f0000803f         movabs rax, 0x3f8000003f800000
1407a1344: 488d8d9c050000               lea rcx, [rbp + 0x59c]
1407a134b: 488901                       mov qword ptr [rcx], rax
1407a134e: c741080000803f               mov dword ptr [rcx + 8], 0x3f800000
1407a1355: c6857706000001               mov byte ptr [rbp + 0x677], 1
1407a135c: 488d8580050000               lea rax, [rbp + 0x580]
1407a1363: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407a1368: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
1407a136f: f30f100d19c82402             movss xmm1, dword ptr [rip + 0x224c819]
1407a1377: f30f1015a9bd2202             movss xmm2, dword ptr [rip + 0x222bda9]
1407a137f: f30f101de9441802             movss xmm3, dword ptr [rip + 0x21844e9]
1407a1387: e8548d3300                   call 0x140ada0e0
1407a138c: 488b85c0030000               mov rax, qword ptr [rbp + 0x3c0]
1407a1393: 48898560050000               mov qword ptr [rbp + 0x560], rax
1407a139a: 488b85c8030000               mov rax, qword ptr [rbp + 0x3c8]
1407a13a1: 48898548060000               mov qword ptr [rbp + 0x648], rax
1407a13a8: 4c8bb5d0030000               mov r14, qword ptr [rbp + 0x3d0]
1407a13af: 488b8530030000               mov rax, qword ptr [rbp + 0x330]
1407a13b6: 482b8568060000               sub rax, qword ptr [rbp + 0x668]
1407a13bd: 4c39f0                       cmp rax, r14
1407a13c0: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
1407a13c7: 0f8263020000                 jb 0x1407a1630
1407a13cd: 4a8d04b500000000             lea rax, [r14*4]
1407a13d5: 4c8d04c0                     lea r8, [rax + rax*8]
1407a13d9: 488bbd68060000               mov rdi, qword ptr [rbp + 0x668]
1407a13e0: 488d04ff                     lea rax, [rdi + rdi*8]
1407a13e4: 48898d58060000               mov qword ptr [rbp + 0x658], rcx
1407a13eb: 488d0c81                     lea rcx, [rcx + rax*4]
1407a13ef: 4c8ba548060000               mov r12, qword ptr [rbp + 0x648]
1407a13f6: 4c89e2                       mov rdx, r12
1407a13f9: e802281802                   call 0x142923c00
1407a13fe: 4c01f7                       add rdi, r14
1407a1401: 4889bd68060000               mov qword ptr [rbp + 0x668], rdi
1407a1408: 4889bd40030000               mov qword ptr [rbp + 0x340], rdi
1407a140f: 488b8560050000               mov rax, qword ptr [rbp + 0x560]
1407a1416: 4885c0                       test rax, rax
1407a1419: 7416                         je 0x1407a1431
1407a141b: 48c1e002                     shl rax, 2
1407a141f: 488d14c0                     lea rdx, [rax + rax*8]
1407a1423: 41b804000000                 mov r8d, 4
1407a1429: 4c89e1                       mov rcx, r12
1407a142c: e8ef27e7ff                   call 0x140613c20
1407a1431: f30f1043e4                   movss xmm0, dword ptr [rbx - 0x1c]
1407a1436: f30f5905eabc2202             mulss xmm0, dword ptr [rip + 0x222bcea]
1407a143e: f30f118590040000             movss dword ptr [rbp + 0x490], xmm0
1407a1446: 488b8550050000               mov rax, qword ptr [rbp + 0x550]
1407a144d: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
1407a1454: 488d85d0040000               lea rax, [rbp + 0x4d0]
1407a145b: 48898580050000               mov qword ptr [rbp + 0x580], rax
1407a1462: 488d05577092ff               lea rax, [rip - 0x6d8fa9]
1407a1469: 48898588050000               mov qword ptr [rbp + 0x588], rax
1407a1470: 488d85d0010000               lea rax, [rbp + 0x1d0]
1407a1477: 48898590050000               mov qword ptr [rbp + 0x590], rax
1407a147e: 488d057ba55a00               lea rax, [rip + 0x5aa57b]
1407a1485: 48898598050000               mov qword ptr [rbp + 0x598], rax
1407a148c: 488d8590040000               lea rax, [rbp + 0x490]
1407a1493: 488985a0050000               mov qword ptr [rbp + 0x5a0], rax
1407a149a: 488d056fb45e00               lea rax, [rip + 0x5eb46f]
1407a14a1: 488985a8050000               mov qword ptr [rbp + 0x5a8], rax
1407a14a8: 488d85ec010000               lea rax, [rbp + 0x1ec]
1407a14af: 488985b0050000               mov qword ptr [rbp + 0x5b0], rax
1407a14b6: 488d05c3c05e00               lea rax, [rip + 0x5ec0c3]
1407a14bd: 488985b8050000               mov qword ptr [rbp + 0x5b8], rax
1407a14c4: 488d055d0c2e02               lea rax, [rip + 0x22e0c5d]
1407a14cb: 488985c0030000               mov qword ptr [rbp + 0x3c0], rax
1407a14d2: 48c785c803000004000000       mov qword ptr [rbp + 0x3c8], 4
1407a14dd: 488d8580050000               lea rax, [rbp + 0x580]
1407a14e4: 488985d0030000               mov qword ptr [rbp + 0x3d0], rax
1407a14eb: 48c785d803000004000000       mov qword ptr [rbp + 0x3d8], 4
1407a14f6: 48c785e003000000000000       mov qword ptr [rbp + 0x3e0], 0
1407a1501: c6857706000001               mov byte ptr [rbp + 0x677], 1
1407a1508: 488d4d30                     lea rcx, [rbp + 0x30]
1407a150c: 488d95c0030000               lea rdx, [rbp + 0x3c0]
1407a1513: e8788a5d00                   call 0x140d79f90
1407a1518: 488b4530                     mov rax, qword ptr [rbp + 0x30]
1407a151c: 48898548060000               mov qword ptr [rbp + 0x648], rax
1407a1523: 488b7d38                     mov rdi, qword ptr [rbp + 0x38]
1407a1527: 4c8b7540                     mov r14, qword ptr [rbp + 0x40]
1407a152b: f3440f109590040000           movss xmm10, dword ptr [rbp + 0x490]
1407a1534: 488b85d0010000               mov rax, qword ptr [rbp + 0x1d0]
1407a153b: 48898580050000               mov qword ptr [rbp + 0x580], rax
1407a1542: 8b85d8010000                 mov eax, dword ptr [rbp + 0x1d8]
1407a1548: 898588050000                 mov dword ptr [rbp + 0x588], eax
1407a154e: 0fb605b436e902               movzx eax, byte ptr [rip + 0x2e936b4]
1407a1555: b930000000                   mov ecx, 0x30
1407a155a: ba10000000                   mov edx, 0x10
1407a155f: e8ac26e7ff                   call 0x140613c10
1407a1564: 4885c0                       test rax, rax
1407a1567: 4889bd60050000               mov qword ptr [rbp + 0x560], rdi
1407a156e: 0f84d1060000                 je 0x1407a1c45
1407a1574: 48c70001000000               mov qword ptr [rax], 1
1407a157b: 48c7400801000000             mov qword ptr [rax + 8], 1
1407a1583: 440f294810                   movaps xmmword ptr [rax + 0x10], xmm9
1407a1588: f3440f115020                 movss dword ptr [rax + 0x20], xmm10
1407a158e: 488b8d80050000               mov rcx, qword ptr [rbp + 0x580]
1407a1595: 48894824                     mov qword ptr [rax + 0x24], rcx
1407a1599: 8b8d88050000                 mov ecx, dword ptr [rbp + 0x588]
1407a159f: 89482c                       mov dword ptr [rax + 0x2c], ecx
1407a15a2: 488d0dbf0b2e02               lea rcx, [rip + 0x22e0bbf]
1407a15a9: 48894c2420                   mov qword ptr [rsp + 0x20], rcx
1407a15ae: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a15b5: 4889fa                       mov rdx, rdi
1407a15b8: 4d89f0                       mov r8, r14
1407a15bb: 4989c1                       mov r9, rax
1407a15be: e8edc20f00                   call 0x14089d8b0
1407a15c3: 488b85a0050000               mov rax, qword ptr [rbp + 0x5a0]
1407a15ca: 488985e0030000               mov qword ptr [rbp + 0x3e0], rax
1407a15d1: 0f108580050000               movups xmm0, xmmword ptr [rbp + 0x580]
1407a15d8: 0f108d90050000               movups xmm1, xmmword ptr [rbp + 0x590]
1407a15df: 0f298dd0030000               movaps xmmword ptr [rbp + 0x3d0], xmm1
1407a15e6: 0f2985c0030000               movaps xmmword ptr [rbp + 0x3c0], xmm0
1407a15ed: 488b8d68050000               mov rcx, qword ptr [rbp + 0x568]
1407a15f4: 488d95c0030000               lea rdx, [rbp + 0x3c0]
1407a15fb: e890b93800                   call 0x140b2cf90
1407a1600: 488b9548060000               mov rdx, qword ptr [rbp + 0x648]
1407a1607: 4885d2                       test rdx, rdx
1407a160a: 4c8d35af062e02               lea r14, [rip + 0x22e06af]
1407a1611: 7412                         je 0x1407a1625
1407a1613: 41b801000000                 mov r8d, 1
1407a1619: 488b8d60050000               mov rcx, qword ptr [rbp + 0x560]
1407a1620: e8fb25e7ff                   call 0x140613c20
1407a1625: 41ffc7                       inc r15d
1407a1628: 4489f8                       mov eax, r15d
1407a162b: e920faffff                   jmp 0x1407a1050
1407a1630: 48c744242024000000           mov qword ptr [rsp + 0x20], 0x24
1407a1639: 41b904000000                 mov r9d, 4
1407a163f: 488d8d30030000               lea rcx, [rbp + 0x330]
1407a1646: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
1407a164d: 4d89f0                       mov r8, r14
1407a1650: e8cb681002                   call 0x1428a7f20
1407a1655: 488b8d38030000               mov rcx, qword ptr [rbp + 0x338]
1407a165c: 488b8540030000               mov rax, qword ptr [rbp + 0x340]
1407a1663: 48898568060000               mov qword ptr [rbp + 0x668], rax
1407a166a: e95efdffff                   jmp 0x1407a13cd
1407a166f: 90                           nop 
1407a1670: 488b85b0010000               mov rax, qword ptr [rbp + 0x1b0]
1407a1677: 488b00                       mov rax, qword ptr [rax]
1407a167a: 833804                       cmp dword ptr [rax], 4
1407a167d: 0f850d020000                 jne 0x1407a1890
1407a1683: c6857706000001               mov byte ptr [rbp + 0x677], 1
1407a168a: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
1407a1691: 488b9550050000               mov rdx, qword ptr [rbp + 0x550]
1407a1698: e893b22700                   call 0x140a1c930
1407a169d: 4889c7                       mov rdi, rax
1407a16a0: 4885c0                       test rax, rax
1407a16a3: 0f84e7010000                 je 0x1407a1890
1407a16a9: c6857706000001               mov byte ptr [rbp + 0x677], 1
1407a16b0: 488d8d40020000               lea rcx, [rbp + 0x240]
1407a16b7: 488b9518030000               mov rdx, qword ptr [rbp + 0x318]
1407a16be: e8cda65d00                   call 0x140d7bd90
1407a16c3: 488b8540030000               mov rax, qword ptr [rbp + 0x340]
1407a16ca: 488d8dc8030000               lea rcx, [rbp + 0x3c8]
1407a16d1: 48894110                     mov qword ptr [rcx + 0x10], rax
1407a16d5: 0f108530030000               movups xmm0, xmmword ptr [rbp + 0x330]
1407a16dc: 0f1101                       movups xmmword ptr [rcx], xmm0
1407a16df: c685c003000002               mov byte ptr [rbp + 0x3c0], 2
1407a16e6: c685d004000000               mov byte ptr [rbp + 0x4d0], 0
1407a16ed: c6856606000001               mov byte ptr [rbp + 0x666], 1
1407a16f4: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a16fb: 4c8dbdc0030000               lea r15, [rbp + 0x3c0]
1407a1702: 4c89fa                       mov rdx, r15
1407a1705: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
1407a170c: e8ff762000                   call 0x1409a8e10
1407a1711: c6855406000001               mov byte ptr [rbp + 0x654], 1
1407a1718: 4c89f9                       mov rcx, r15
1407a171b: e860782000                   call 0x1409a8f80
1407a1720: 48c7859804000002000000       mov qword ptr [rbp + 0x498], 2
1407a172b: c7859004000001000000         mov dword ptr [rbp + 0x490], 1
1407a1735: c6855406000001               mov byte ptr [rbp + 0x654], 1
1407a173c: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
1407a1743: 4c89fa                       mov rdx, r15
1407a1746: 4c8d8590040000               lea r8, [rbp + 0x490]
1407a174d: e87e782000                   call 0x1409a8fd0
1407a1752: c6855406000000               mov byte ptr [rbp + 0x654], 0
1407a1759: 4c89f9                       mov rcx, r15
1407a175c: 488d9580050000               lea rdx, [rbp + 0x580]
1407a1763: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
1407a176a: e811772000                   call 0x1409a8e80
1407a176f: c6856606000001               mov byte ptr [rbp + 0x666], 1
1407a1776: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a177d: 4c89fa                       mov rdx, r15
1407a1780: 4531c0                       xor r8d, r8d
1407a1783: e858772000                   call 0x1409a8ee0
1407a1788: c6856606000000               mov byte ptr [rbp + 0x666], 0
1407a178f: 488b8d78040000               mov rcx, qword ptr [rbp + 0x478]
1407a1796: 488d9540020000               lea rdx, [rbp + 0x240]
1407a179d: 4c8d8580050000               lea r8, [rbp + 0x580]
1407a17a4: e8471a1800                   call 0x1409231f0
1407a17a9: 48898548010000               mov qword ptr [rbp + 0x148], rax
1407a17b0: 48899550010000               mov qword ptr [rbp + 0x150], rdx
1407a17b7: 4883c730                     add rdi, 0x30
1407a17bb: c6857706000000               mov byte ptr [rbp + 0x677], 0
1407a17c2: 4889f9                       mov rcx, rdi
1407a17c5: 488d152c092e02               lea rdx, [rip + 0x22e092c]
1407a17cc: e8df7c0900                   call 0x1408394b0
1407a17d1: 4889c7                       mov rdi, rax
1407a17d4: c6857706000000               mov byte ptr [rbp + 0x677], 0
1407a17db: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a17e2: 488d9548010000               lea rdx, [rbp + 0x148]
1407a17e9: e852ce1800                   call 0x14092e640
1407a17ee: 83bd8005000002               cmp dword ptr [rbp + 0x580], 2
1407a17f5: 0f85c4000000                 jne 0x1407a18bf
1407a17fb: 0f2843b0                     movaps xmm0, xmmword ptr [rbx - 0x50]
1407a17ff: 0f28d0                       movaps xmm2, xmm0
1407a1802: 0f59d0                       mulps xmm2, xmm0
1407a1805: f30f16ca                     movshdup xmm1, xmm2
1407a1809: f30f58ca                     addss xmm1, xmm2
1407a180d: 0f28da                       movaps xmm3, xmm2
1407a1810: 660f15da                     unpckhpd xmm3, xmm2
1407a1814: f30f58d9                     addss xmm3, xmm1
1407a1818: 0fc6db00                     shufps xmm3, xmm3, 0
1407a181c: 0fc6d2ff                     shufps xmm2, xmm2, 0xff
1407a1820: 0f5cd3                       subps xmm2, xmm3
1407a1823: f30f101dd9401802             movss xmm3, dword ptr [rip + 0x21840d9]
1407a182b: 0f59d3                       mulps xmm2, xmm3
1407a182e: 0f28c8                       movaps xmm1, xmm0
1407a1831: 0f59cb                       mulps xmm1, xmm3
1407a1834: f30f16d9                     movshdup xmm3, xmm1
1407a1838: f30f58d9                     addss xmm3, xmm1
1407a183c: 0f12c9                       movhlps xmm1, xmm1
1407a183f: f30f58cb                     addss xmm1, xmm3
1407a1843: f30f58c9                     addss xmm1, xmm1
1407a1847: 0fc6c900                     shufps xmm1, xmm1, 0
1407a184b: 0f59c8                       mulps xmm1, xmm0
1407a184e: 0f58ca                       addps xmm1, xmm2
1407a1851: 0f28d0                       movaps xmm2, xmm0
1407a1854: 0fc6d0d2                     shufps xmm2, xmm0, 0xd2
1407a1858: 0f591561b82202               mulps xmm2, xmmword ptr [rip + 0x222b861]
1407a185f: 0f28d8                       movaps xmm3, xmm0
1407a1862: 0fc6d8c9                     shufps xmm3, xmm0, 0xc9
1407a1866: 0f591d432d1902               mulps xmm3, xmmword ptr [rip + 0x2192d43]
1407a186d: 0f5cda                       subps xmm3, xmm2
1407a1870: 0f58c0                       addps xmm0, xmm0
1407a1873: 0fc6c0ff                     shufps xmm0, xmm0, 0xff
1407a1877: 0f59c3                       mulps xmm0, xmm3
1407a187a: 0f58c1                       addps xmm0, xmm1
1407a187d: e95e010000                   jmp 0x1407a19e0
1407a1882: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a1890: 488b8530030000               mov rax, qword ptr [rbp + 0x330]
1407a1897: 4885c0                       test rax, rax
1407a189a: 0f8470f6ffff                 je 0x1407a0f10
1407a18a0: 48c1e002                     shl rax, 2
1407a18a4: 488d14c0                     lea rdx, [rax + rax*8]
1407a18a8: 41b804000000                 mov r8d, 4
1407a18ae: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
1407a18b5: e86623e7ff                   call 0x140613c20
1407a18ba: e951f6ffff                   jmp 0x1407a0f10
1407a18bf: f30f108da4050000             movss xmm1, dword ptr [rbp + 0x5a4]
1407a18c7: 0f2843b0                     movaps xmm0, xmmword ptr [rbx - 0x50]
1407a18cb: 0f28d8                       movaps xmm3, xmm0
1407a18ce: 0f59d8                       mulps xmm3, xmm0
1407a18d1: f30f16d3                     movshdup xmm2, xmm3
1407a18d5: f30f58d3                     addss xmm2, xmm3
1407a18d9: 0f28e3                       movaps xmm4, xmm3
1407a18dc: 660f15e3                     unpckhpd xmm4, xmm3
1407a18e0: f30f58e2                     addss xmm4, xmm2
1407a18e4: 0fc6e400                     shufps xmm4, xmm4, 0
1407a18e8: 0fc6dbff                     shufps xmm3, xmm3, 0xff
1407a18ec: 0f5cdc                       subps xmm3, xmm4
1407a18ef: f30f10250d401802             movss xmm4, dword ptr [rip + 0x218400d]
1407a18f7: 0f59dc                       mulps xmm3, xmm4
1407a18fa: 0f28d0                       movaps xmm2, xmm0
1407a18fd: 0f59d4                       mulps xmm2, xmm4
1407a1900: f30f16e2                     movshdup xmm4, xmm2
1407a1904: f30f58e2                     addss xmm4, xmm2
1407a1908: 0f12d2                       movhlps xmm2, xmm2
1407a190b: f30f58d4                     addss xmm2, xmm4
1407a190f: f30f58d2                     addss xmm2, xmm2
1407a1913: 0fc6d200                     shufps xmm2, xmm2, 0
1407a1917: 0f59d0                       mulps xmm2, xmm0
1407a191a: 0f58d3                       addps xmm2, xmm3
1407a191d: 0f28d8                       movaps xmm3, xmm0
1407a1920: 0fc6d8d2                     shufps xmm3, xmm0, 0xd2
1407a1924: 0f591d95b72202               mulps xmm3, xmmword ptr [rip + 0x222b795]
1407a192b: 0f28e0                       movaps xmm4, xmm0
1407a192e: 0fc6e0c9                     shufps xmm4, xmm0, 0xc9
1407a1932: 0f5925772c1902               mulps xmm4, xmmword ptr [rip + 0x2192c77]
1407a1939: 0f5ce3                       subps xmm4, xmm3
1407a193c: 0f58c0                       addps xmm0, xmm0
1407a193f: 0fc6c0ff                     shufps xmm0, xmm0, 0xff
1407a1943: 0f59c4                       mulps xmm0, xmm4
1407a1946: 0f58c2                       addps xmm0, xmm2
1407a1949: f30f5c4be8                   subss xmm1, dword ptr [rbx - 0x18]
1407a194e: 0f28d1                       movaps xmm2, xmm1
1407a1951: f30f59d1                     mulss xmm2, xmm1
1407a1955: f20f109da8050000             movsd xmm3, qword ptr [rbp + 0x5a8]
1407a195d: f20f1063ec                   movsd xmm4, qword ptr [rbx - 0x14]
1407a1962: 0f5cdc                       subps xmm3, xmm4
1407a1965: 0f28e3                       movaps xmm4, xmm3
1407a1968: 0f59e3                       mulps xmm4, xmm3
1407a196b: f30f58d4                     addss xmm2, xmm4
1407a196f: f30f16e4                     movshdup xmm4, xmm4
1407a1973: f30f58e2                     addss xmm4, xmm2
1407a1977: 0f57d2                       xorps xmm2, xmm2
1407a197a: f30f51d4                     sqrtss xmm2, xmm4
1407a197e: f30f10257e3f1802             movss xmm4, dword ptr [rip + 0x2183f7e]
1407a1986: f30f5ee2                     divss xmm4, xmm2
1407a198a: f30f59cc                     mulss xmm1, xmm4
1407a198e: f30f12d4                     movsldup xmm2, xmm4
1407a1992: 0f59d3                       mulps xmm2, xmm3
1407a1995: f30f59c8                     mulss xmm1, xmm0
1407a1999: 0f28d8                       movaps xmm3, xmm0
1407a199c: 0fc6d8e9                     shufps xmm3, xmm0, 0xe9
1407a19a0: 0f59da                       mulps xmm3, xmm2
1407a19a3: f30f58cb                     addss xmm1, xmm3
1407a19a7: f30f16d3                     movshdup xmm2, xmm3
1407a19ab: f30f58d1                     addss xmm2, xmm1
1407a19af: 0f57c9                       xorps xmm1, xmm1
1407a19b2: 0f2eca                       ucomiss xmm1, xmm2
1407a19b5: 7629                         jbe 0x1407a19e0
1407a19b7: 0f28c8                       movaps xmm1, xmm0
1407a19ba: 660f15c8                     unpckhpd xmm1, xmm0
1407a19be: 0f28152b3f1802               movaps xmm2, xmmword ptr [rip + 0x2183f2b]
1407a19c5: 0f57c2                       xorps xmm0, xmm2
1407a19c8: 0f57ca                       xorps xmm1, xmm2
1407a19cb: 0f138590040000               movlps qword ptr [rbp + 0x490], xmm0
1407a19d2: f30f118d98040000             movss dword ptr [rbp + 0x498], xmm1
1407a19da: eb16                         jmp 0x1407a19f2
1407a19dc: 0f1f4000                     nop dword ptr [rax]
1407a19e0: 0f138590040000               movlps qword ptr [rbp + 0x490], xmm0
1407a19e7: 0f12c0                       movhlps xmm0, xmm0
1407a19ea: f30f118598040000             movss dword ptr [rbp + 0x498], xmm0
1407a19f2: c6857706000000               mov byte ptr [rbp + 0x677], 0
1407a19f9: 488d8d48010000               lea rcx, [rbp + 0x148]
1407a1a00: e80bc51800                   call 0x14092df10
1407a1a05: 88c3                         mov bl, al
1407a1a07: 488b8520030000               mov rax, qword ptr [rbp + 0x320]
1407a1a0e: 48898580050000               mov qword ptr [rbp + 0x580], rax
1407a1a15: 488d8590040000               lea rax, [rbp + 0x490]
1407a1a1c: 48898588050000               mov qword ptr [rbp + 0x588], rax
1407a1a23: 4889bd90050000               mov qword ptr [rbp + 0x590], rdi
1407a1a2a: f20f10057e022e02             movsd xmm0, qword ptr [rip + 0x22e027e]
1407a1a32: f20f1185d0040000             movsd qword ptr [rbp + 0x4d0], xmm0
1407a1a3a: c685d804000000               mov byte ptr [rbp + 0x4d8], 0
1407a1a41: c6857706000000               mov byte ptr [rbp + 0x677], 0
1407a1a48: 488d85d0040000               lea rax, [rbp + 0x4d0]
1407a1a4f: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407a1a54: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
1407a1a5b: 488b9598070000               mov rdx, qword ptr [rbp + 0x798]
1407a1a62: 4c8d8548010000               lea r8, [rbp + 0x148]
1407a1a69: 4c8d8d80050000               lea r9, [rbp + 0x580]
1407a1a70: e82b52ad00                   call 0x141276ca0
1407a1a75: f685c003000001               test byte ptr [rbp + 0x3c0], 1
1407a1a7c: 0f848ef4ffff                 je 0x1407a0f10
1407a1a82: f3440f108dd0030000           movss xmm9, dword ptr [rbp + 0x3d0]
1407a1a8b: c6857706000000               mov byte ptr [rbp + 0x677], 0
1407a1a92: 488b8d38060000               mov rcx, qword ptr [rbp + 0x638]
1407a1a99: 488b9550050000               mov rdx, qword ptr [rbp + 0x550]
1407a1aa0: e83ba92700                   call 0x140a1c3e0
1407a1aa5: 4889d1                       mov rcx, rdx
1407a1aa8: 80fb02                       cmp bl, 2
1407a1aab: 0f93c2                       setae dl
1407a1aae: 48898580050000               mov qword ptr [rbp + 0x580], rax
1407a1ab5: 48898d88050000               mov qword ptr [rbp + 0x588], rcx
1407a1abc: 48c7859005000003000000       mov qword ptr [rbp + 0x590], 3
1407a1ac7: c785d005000000000000         mov dword ptr [rbp + 0x5d0], 0
1407a1ad1: c785d805000001000000         mov dword ptr [rbp + 0x5d8], 1
1407a1adb: f3440f118ddc050000           movss dword ptr [rbp + 0x5dc], xmm9
1407a1ae4: c685e005000000               mov byte ptr [rbp + 0x5e0], 0
1407a1aeb: c685e205000000               mov byte ptr [rbp + 0x5e2], 0
1407a1af2: 66c785e40500000240           mov word ptr [rbp + 0x5e4], 0x4002
1407a1afb: 488b8550050000               mov rax, qword ptr [rbp + 0x550]
1407a1b02: 8985e8050000                 mov dword ptr [rbp + 0x5e8], eax
1407a1b08: c685ec0500003f               mov byte ptr [rbp + 0x5ec], 0x3f
1407a1b0f: 488d8dc8030000               lea rcx, [rbp + 0x3c8]
1407a1b16: 8b4104                       mov eax, dword ptr [rcx + 4]
1407a1b19: 8985d8040000                 mov dword ptr [rbp + 0x4d8], eax
1407a1b1f: 488b41fc                     mov rax, qword ptr [rcx - 4]
1407a1b23: 488985d0040000               mov qword ptr [rbp + 0x4d0], rax
1407a1b2a: c685dc04000000               mov byte ptr [rbp + 0x4dc], 0
1407a1b31: 488b8588020000               mov rax, qword ptr [rbp + 0x288]
1407a1b38: 8b8d38050000                 mov ecx, dword ptr [rbp + 0x538]
1407a1b3e: 8908                         mov dword ptr [rax], ecx
1407a1b40: 488b8598020000               mov rax, qword ptr [rbp + 0x298]
1407a1b47: 8b8d3c050000                 mov ecx, dword ptr [rbp + 0x53c]
1407a1b4d: 8908                         mov dword ptr [rax], ecx
1407a1b4f: 8b8580040000                 mov eax, dword ptr [rbp + 0x480]
1407a1b55: 488b8d80020000               mov rcx, qword ptr [rbp + 0x280]
1407a1b5c: 8901                         mov dword ptr [rcx], eax
1407a1b5e: c6857706000000               mov byte ptr [rbp + 0x677], 0
1407a1b65: 488b85a0020000               mov rax, qword ptr [rbp + 0x2a0]
1407a1b6c: 4889442438                   mov qword ptr [rsp + 0x38], rax
1407a1b71: 488d45c0                     lea rax, [rbp - 0x40]
1407a1b75: 4889442430                   mov qword ptr [rsp + 0x30], rax
1407a1b7a: 488b8528030000               mov rax, qword ptr [rbp + 0x328]
1407a1b81: 4889442428                   mov qword ptr [rsp + 0x28], rax
1407a1b86: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
1407a1b8d: 4889442420                   mov qword ptr [rsp + 0x20], rax
1407a1b92: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a1b99: 4c8d85d0040000               lea r8, [rbp + 0x4d0]
1407a1ba0: 4c8b8d38060000               mov r9, qword ptr [rbp + 0x638]
1407a1ba7: e88452ad00                   call 0x141276e30
1407a1bac: e95ff3ffff                   jmp 0x1407a0f10
1407a1bb1: 0f28b580060000               movaps xmm6, xmmword ptr [rbp + 0x680]
1407a1bb8: 0f28bd90060000               movaps xmm7, xmmword ptr [rbp + 0x690]
1407a1bbf: 440f2885a0060000             movaps xmm8, xmmword ptr [rbp + 0x6a0]
1407a1bc7: 440f288db0060000             movaps xmm9, xmmword ptr [rbp + 0x6b0]
1407a1bcf: 440f2895c0060000             movaps xmm10, xmmword ptr [rbp + 0x6c0]
1407a1bd7: 440f289dd0060000             movaps xmm11, xmmword ptr [rbp + 0x6d0]
1407a1bdf: 440f28a5e0060000             movaps xmm12, xmmword ptr [rbp + 0x6e0]
1407a1be7: 440f28adf0060000             movaps xmm13, xmmword ptr [rbp + 0x6f0]
1407a1bef: 440f28b500070000             movaps xmm14, xmmword ptr [rbp + 0x700]
1407a1bf7: 440f28bd10070000             movaps xmm15, xmmword ptr [rbp + 0x710]
1407a1bff: 4881c4a8070000               add rsp, 0x7a8
1407a1c06: 5b                           pop rbx
1407a1c07: 5f                           pop rdi
1407a1c08: 5e                           pop rsi
1407a1c09: 415c                         pop r12
1407a1c0b: 415d                         pop r13
1407a1c0d: 415e                         pop r14
1407a1c0f: 415f                         pop r15
1407a1c11: 5d                           pop rbp
1407a1c12: c3                           ret 
1407a1c13: f20f10b584050000             movsd xmm6, qword ptr [rbp + 0x584]
1407a1c1b: f3440f108574050000           movss xmm8, dword ptr [rbp + 0x574]
1407a1c24: f3440f5c8590050000           subss xmm8, dword ptr [rbp + 0x590]
1407a1c2d: 4c8bb5f8020000               mov r14, qword ptr [rbp + 0x2f8]
1407a1c34: 488bbd00030000               mov rdi, qword ptr [rbp + 0x300]
1407a1c3b: 41b402                       mov r12b, 2
1407a1c3e: 31c9                         xor ecx, ecx
1407a1c40: e952efffff                   jmp 0x1407a0b97
1407a1c45: b910000000                   mov ecx, 0x10
1407a1c4a: ba30000000                   mov edx, 0x30
1407a1c4f: e8ac731302                   call 0x1428d9000
1407a1c54: e902010000                   jmp 0x1407a1d5b
1407a1c59: 4c8d0580042e02               lea r8, [rip + 0x22e0480]
1407a1c60: b904000000                   mov ecx, 4
1407a1c65: ba60030000                   mov edx, 0x360
1407a1c6a: e874731302                   call 0x1428d8fe3
1407a1c6f: b908000000                   mov ecx, 8
1407a1c74: ba20000000                   mov edx, 0x20
1407a1c79: e882731302                   call 0x1428d9000
1407a1c7e: b910000000                   mov ecx, 0x10
1407a1c83: ba50000000                   mov edx, 0x50
1407a1c88: e873731302                   call 0x1428d9000
1407a1c8d: e9c9000000                   jmp 0x1407a1d5b
1407a1c92: b910000000                   mov ecx, 0x10
1407a1c97: ba50000000                   mov edx, 0x50
1407a1c9c: e85f731302                   call 0x1428d9000
1407a1ca1: e9b5000000                   jmp 0x1407a1d5b
1407a1ca6: b910000000                   mov ecx, 0x10
1407a1cab: ba50000000                   mov edx, 0x50
1407a1cb0: e84b731302                   call 0x1428d9000
1407a1cb5: e9a1000000                   jmp 0x1407a1d5b
1407a1cba: 31f6                         xor esi, esi
1407a1cbc: 4c8d054d034802               lea r8, [rip + 0x248034d]
1407a1cc3: 4889f1                       mov rcx, rsi
1407a1cc6: 4889fa                       mov rdx, rdi
1407a1cc9: e815731302                   call 0x1428d8fe3
1407a1cce: 4c8d055b064802               lea r8, [rip + 0x248065b]
1407a1cd5: b904000000                   mov ecx, 4
1407a1cda: 4889f2                       mov rdx, rsi
1407a1cdd: e801731302                   call 0x1428d8fe3
1407a1ce2: eb77                         jmp 0x1407a1d5b
1407a1ce4: b910000000                   mov ecx, 0x10
1407a1ce9: ba30000000                   mov edx, 0x30
1407a1cee: e80d731302                   call 0x1428d9000
1407a1cf3: eb66                         jmp 0x1407a1d5b
1407a1cf5: b908000000                   mov ecx, 8
1407a1cfa: ba20000000                   mov edx, 0x20
1407a1cff: e8fc721302                   call 0x1428d9000
1407a1d04: eb55                         jmp 0x1407a1d5b
1407a1d06: b904000000                   mov ecx, 4
1407a1d0b: ba18000000                   mov edx, 0x18
1407a1d10: e8eb721302                   call 0x1428d9000
1407a1d15: 488d0d74012e02               lea rcx, [rip + 0x22e0174]
1407a1d1c: e8ef751302                   call 0x1428d9310
1407a1d21: c6855706000001               mov byte ptr [rbp + 0x657], 1
1407a1d28: b904000000                   mov ecx, 4
1407a1d2d: ba18000000                   mov edx, 0x18
1407a1d32: e8c9721302                   call 0x1428d9000
1407a1d37: eb22                         jmp 0x1407a1d5b
1407a1d39: 488d0db0012e02               lea rcx, [rip + 0x22e01b0]
1407a1d40: e8cb751302                   call 0x1428d9310
1407a1d45: c6856706000001               mov byte ptr [rbp + 0x667], 1
1407a1d4c: b904000000                   mov ecx, 4
1407a1d51: ba18000000                   mov edx, 0x18
1407a1d56: e8a5721302                   call 0x1428d9000
1407a1d5b: 0f0b                         ud2 
1407a1d5d: 488d0d6c022e02               lea rcx, [rip + 0x22e026c]
1407a1d64: e8a7751302                   call 0x1428d9310
1407a1d69: cc                           int3 
1407a1d6a: 660f1f440000                 nop word ptr [rax + rax]
1407a1d70: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a1d75: 55                           push rbp
1407a1d76: 4157                         push r15
1407a1d78: 4156                         push r14
1407a1d7a: 4155                         push r13
1407a1d7c: 4154                         push r12
1407a1d7e: 56                           push rsi
1407a1d7f: 57                           push rdi
1407a1d80: 53                           push rbx
1407a1d81: 4881ece8000000               sub rsp, 0xe8
1407a1d88: 488daa80000000               lea rbp, [rdx + 0x80]
1407a1d8f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a1d95: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a1d9b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a1da1: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a1da7: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a1db0: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a1db9: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a1dc2: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a1dcb: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a1dd3: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a1ddb: 4883bd6005000000             cmp qword ptr [rbp + 0x560], 0
1407a1de3: 7421                         je 0x1407a1e06
1407a1de5: 488b8560050000               mov rax, qword ptr [rbp + 0x560]
1407a1dec: 48c1e002                     shl rax, 2
1407a1df0: 488d14c0                     lea rdx, [rax + rax*8]
1407a1df4: 41b804000000                 mov r8d, 4
1407a1dfa: 488b8d48060000               mov rcx, qword ptr [rbp + 0x648]
1407a1e01: e81a1ee7ff                   call 0x140613c20
1407a1e06: c6857706000001               mov byte ptr [rbp + 0x677], 1
1407a1e0d: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a1e15: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a1e1d: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a1e26: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a1e2f: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a1e38: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a1e41: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a1e47: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a1e4d: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a1e53: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a1e59: 4881c4e8000000               add rsp, 0xe8
1407a1e60: 5b                           pop rbx
1407a1e61: 5f                           pop rdi
1407a1e62: 5e                           pop rsi
1407a1e63: 415c                         pop r12
1407a1e65: 415d                         pop r13
1407a1e67: 415e                         pop r14
1407a1e69: 415f                         pop r15
1407a1e6b: 5d                           pop rbp
1407a1e6c: c3                           ret 
1407a1e6d: 0f1f00                       nop dword ptr [rax]
1407a1e70: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a1e75: 55                           push rbp
1407a1e76: 4157                         push r15
1407a1e78: 4156                         push r14
1407a1e7a: 4155                         push r13
1407a1e7c: 4154                         push r12
1407a1e7e: 56                           push rsi
1407a1e7f: 57                           push rdi
1407a1e80: 53                           push rbx
1407a1e81: 4881ece8000000               sub rsp, 0xe8
1407a1e88: 488daa80000000               lea rbp, [rdx + 0x80]
1407a1e8f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a1e95: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a1e9b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a1ea1: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a1ea7: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a1eb0: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a1eb9: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a1ec2: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a1ecb: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a1ed3: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a1edb: 80bd4406000000               cmp byte ptr [rbp + 0x644], 0
1407a1ee2: 7560                         jne 0x1407a1f44
1407a1ee4: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a1eec: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a1ef4: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a1efd: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a1f06: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a1f0f: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a1f18: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a1f1e: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a1f24: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a1f2a: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a1f30: 4881c4e8000000               add rsp, 0xe8
1407a1f37: 5b                           pop rbx
1407a1f38: 5f                           pop rdi
1407a1f39: 5e                           pop rsi
1407a1f3a: 415c                         pop r12
1407a1f3c: 415d                         pop r13
1407a1f3e: 415e                         pop r14
1407a1f40: 415f                         pop r15
1407a1f42: 5d                           pop rbp
1407a1f43: c3                           ret 
1407a1f44: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a1f4b: e820350f00                   call 0x140895470
1407a1f50: eb92                         jmp 0x1407a1ee4
1407a1f52: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a1f60: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a1f65: 55                           push rbp
1407a1f66: 4157                         push r15
1407a1f68: 4156                         push r14
1407a1f6a: 4155                         push r13
1407a1f6c: 4154                         push r12
1407a1f6e: 56                           push rsi
1407a1f6f: 57                           push rdi
1407a1f70: 53                           push rbx
1407a1f71: 4881ece8000000               sub rsp, 0xe8
1407a1f78: 488daa80000000               lea rbp, [rdx + 0x80]
1407a1f7f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a1f85: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a1f8b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a1f91: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a1f97: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a1fa0: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a1fa9: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a1fb2: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a1fbb: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a1fc3: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a1fcb: 80bd4606000000               cmp byte ptr [rbp + 0x646], 0
1407a1fd2: 7560                         jne 0x1407a2034
1407a1fd4: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a1fdc: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a1fe4: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a1fed: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a1ff6: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a1fff: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a2008: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a200e: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a2014: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a201a: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a2020: 4881c4e8000000               add rsp, 0xe8
1407a2027: 5b                           pop rbx
1407a2028: 5f                           pop rdi
1407a2029: 5e                           pop rsi
1407a202a: 415c                         pop r12
1407a202c: 415d                         pop r13
1407a202e: 415e                         pop r14
1407a2030: 415f                         pop r15
1407a2032: 5d                           pop rbp
1407a2033: c3                           ret 
1407a2034: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a203b: e830340f00                   call 0x140895470
1407a2040: eb92                         jmp 0x1407a1fd4
1407a2042: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a2050: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2055: 55                           push rbp
1407a2056: 4157                         push r15
1407a2058: 4156                         push r14
1407a205a: 4155                         push r13
1407a205c: 4154                         push r12
1407a205e: 56                           push rsi
1407a205f: 57                           push rdi
1407a2060: 53                           push rbx
1407a2061: 4881ece8000000               sub rsp, 0xe8
1407a2068: 488daa80000000               lea rbp, [rdx + 0x80]
1407a206f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2075: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a207b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2081: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2087: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2090: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2099: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a20a2: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a20ab: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a20b3: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a20bb: 80bd4506000000               cmp byte ptr [rbp + 0x645], 0
1407a20c2: 7560                         jne 0x1407a2124
1407a20c4: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a20cc: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a20d4: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a20dd: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a20e6: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a20ef: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a20f8: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a20fe: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a2104: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a210a: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a2110: 4881c4e8000000               add rsp, 0xe8
1407a2117: 5b                           pop rbx
1407a2118: 5f                           pop rdi
1407a2119: 5e                           pop rsi
1407a211a: 415c                         pop r12
1407a211c: 415d                         pop r13
1407a211e: 415e                         pop r14
1407a2120: 415f                         pop r15
1407a2122: 5d                           pop rbp
1407a2123: c3                           ret 
1407a2124: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a212b: e840330f00                   call 0x140895470
1407a2130: eb92                         jmp 0x1407a20c4
1407a2132: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a2140: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2145: 55                           push rbp
1407a2146: 4157                         push r15
1407a2148: 4156                         push r14
1407a214a: 4155                         push r13
1407a214c: 4154                         push r12
1407a214e: 56                           push rsi
1407a214f: 57                           push rdi
1407a2150: 53                           push rbx
1407a2151: 4881ece8000000               sub rsp, 0xe8
1407a2158: 488daa80000000               lea rbp, [rdx + 0x80]
1407a215f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2165: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a216b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2171: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2177: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2180: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2189: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2192: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a219b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a21a3: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a21ab: 80bd4706000000               cmp byte ptr [rbp + 0x647], 0
1407a21b2: 7560                         jne 0x1407a2214
1407a21b4: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a21bc: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a21c4: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a21cd: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a21d6: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a21df: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a21e8: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a21ee: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a21f4: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a21fa: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a2200: 4881c4e8000000               add rsp, 0xe8
1407a2207: 5b                           pop rbx
1407a2208: 5f                           pop rdi
1407a2209: 5e                           pop rsi
1407a220a: 415c                         pop r12
1407a220c: 415d                         pop r13
1407a220e: 415e                         pop r14
1407a2210: 415f                         pop r15
1407a2212: 5d                           pop rbp
1407a2213: c3                           ret 
1407a2214: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a221b: e850320f00                   call 0x140895470
1407a2220: eb92                         jmp 0x1407a21b4
1407a2222: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a2230: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2235: 55                           push rbp
1407a2236: 4157                         push r15
1407a2238: 4156                         push r14
1407a223a: 4155                         push r13
1407a223c: 4154                         push r12
1407a223e: 56                           push rsi
1407a223f: 57                           push rdi
1407a2240: 53                           push rbx
1407a2241: 4881ece8000000               sub rsp, 0xe8
1407a2248: 488daa80000000               lea rbp, [rdx + 0x80]
1407a224f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2255: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a225b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2261: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2267: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2270: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2279: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2282: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a228b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2293: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a229b: 80bd5506000000               cmp byte ptr [rbp + 0x655], 0
1407a22a2: 7560                         jne 0x1407a2304
1407a22a4: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a22ac: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a22b4: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a22bd: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a22c6: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a22cf: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a22d8: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a22de: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a22e4: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a22ea: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a22f0: 4881c4e8000000               add rsp, 0xe8
1407a22f7: 5b                           pop rbx
1407a22f8: 5f                           pop rdi
1407a22f9: 5e                           pop rsi
1407a22fa: 415c                         pop r12
1407a22fc: 415d                         pop r13
1407a22fe: 415e                         pop r14
1407a2300: 415f                         pop r15
1407a2302: 5d                           pop rbp
1407a2303: c3                           ret 
1407a2304: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
1407a230b: e860310f00                   call 0x140895470
1407a2310: eb92                         jmp 0x1407a22a4
1407a2312: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a2320: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2325: 55                           push rbp
1407a2326: 4157                         push r15
1407a2328: 4156                         push r14
1407a232a: 4155                         push r13
1407a232c: 4154                         push r12
1407a232e: 56                           push rsi
1407a232f: 57                           push rdi
1407a2330: 53                           push rbx
1407a2331: 4881ece8000000               sub rsp, 0xe8
1407a2338: 488daa80000000               lea rbp, [rdx + 0x80]
1407a233f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2345: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a234b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2351: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2357: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2360: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2369: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2372: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a237b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2383: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a238b: 4883bd6806000000             cmp qword ptr [rbp + 0x668], 0
1407a2393: 7419                         je 0x1407a23ae
1407a2395: 41b801000000                 mov r8d, 1
1407a239b: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
1407a23a2: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
1407a23a9: e87218e7ff                   call 0x140613c20
1407a23ae: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a23b6: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a23be: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a23c7: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a23d0: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a23d9: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a23e2: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a23e8: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a23ee: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a23f4: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a23fa: 4881c4e8000000               add rsp, 0xe8
1407a2401: 5b                           pop rbx
1407a2402: 5f                           pop rdi
1407a2403: 5e                           pop rsi
1407a2404: 415c                         pop r12
1407a2406: 415d                         pop r13
1407a2408: 415e                         pop r14
1407a240a: 415f                         pop r15
1407a240c: 5d                           pop rbp
1407a240d: c3                           ret 
1407a240e: 6690                         nop 
1407a2410: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2415: 55                           push rbp
1407a2416: 4157                         push r15
1407a2418: 4156                         push r14
1407a241a: 4155                         push r13
1407a241c: 4154                         push r12
1407a241e: 56                           push rsi
1407a241f: 57                           push rdi
1407a2420: 53                           push rbx
1407a2421: 4881ece8000000               sub rsp, 0xe8
1407a2428: 488daa80000000               lea rbp, [rdx + 0x80]
1407a242f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2435: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a243b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2441: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2447: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2450: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2459: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2462: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a246b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2473: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a247b: 4883bd6806000000             cmp qword ptr [rbp + 0x668], 0
1407a2483: 7419                         je 0x1407a249e
1407a2485: 41b801000000                 mov r8d, 1
1407a248b: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
1407a2492: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
1407a2499: e88217e7ff                   call 0x140613c20
1407a249e: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a24a6: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a24ae: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a24b7: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a24c0: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a24c9: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a24d2: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a24d8: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a24de: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a24e4: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a24ea: 4881c4e8000000               add rsp, 0xe8
1407a24f1: 5b                           pop rbx
1407a24f2: 5f                           pop rdi
1407a24f3: 5e                           pop rsi
1407a24f4: 415c                         pop r12
1407a24f6: 415d                         pop r13
1407a24f8: 415e                         pop r14
1407a24fa: 415f                         pop r15
1407a24fc: 5d                           pop rbp
1407a24fd: c3                           ret 
1407a24fe: 6690                         nop 
1407a2500: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2505: 55                           push rbp
1407a2506: 4157                         push r15
1407a2508: 4156                         push r14
1407a250a: 4155                         push r13
1407a250c: 4154                         push r12
1407a250e: 56                           push rsi
1407a250f: 57                           push rdi
1407a2510: 53                           push rbx
1407a2511: 4881ece8000000               sub rsp, 0xe8
1407a2518: 488daa80000000               lea rbp, [rdx + 0x80]
1407a251f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2525: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a252b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2531: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2537: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2540: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2549: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2552: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a255b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2563: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a256b: 488b9580050000               mov rdx, qword ptr [rbp + 0x580]
1407a2572: 4885d2                       test rdx, rdx
1407a2575: 7416                         je 0x1407a258d
1407a2577: 488b8d88050000               mov rcx, qword ptr [rbp + 0x588]
1407a257e: 48c1e203                     shl rdx, 3
1407a2582: 41b804000000                 mov r8d, 4
1407a2588: e89316e7ff                   call 0x140613c20
1407a258d: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a2595: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a259d: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a25a6: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a25af: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a25b8: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a25c1: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a25c7: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a25cd: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a25d3: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a25d9: 4881c4e8000000               add rsp, 0xe8
1407a25e0: 5b                           pop rbx
1407a25e1: 5f                           pop rdi
1407a25e2: 5e                           pop rsi
1407a25e3: 415c                         pop r12
1407a25e5: 415d                         pop r13
1407a25e7: 415e                         pop r14
1407a25e9: 415f                         pop r15
1407a25eb: 5d                           pop rbp
1407a25ec: c3                           ret 
1407a25ed: 0f1f00                       nop dword ptr [rax]
1407a25f0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a25f5: 55                           push rbp
1407a25f6: 4157                         push r15
1407a25f8: 4156                         push r14
1407a25fa: 4155                         push r13
1407a25fc: 4154                         push r12
1407a25fe: 56                           push rsi
1407a25ff: 57                           push rdi
1407a2600: 53                           push rbx
1407a2601: 4881ece8000000               sub rsp, 0xe8
1407a2608: 488daa80000000               lea rbp, [rdx + 0x80]
1407a260f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2615: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a261b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2621: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2627: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2630: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2639: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2642: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a264b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2653: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a265b: 4883bd5806000000             cmp qword ptr [rbp + 0x658], 0
1407a2663: 741d                         je 0x1407a2682
1407a2665: 488b9558060000               mov rdx, qword ptr [rbp + 0x658]
1407a266c: 48c1e202                     shl rdx, 2
1407a2670: 41b804000000                 mov r8d, 4
1407a2676: 488b8d68060000               mov rcx, qword ptr [rbp + 0x668]
1407a267d: e89e15e7ff                   call 0x140613c20
1407a2682: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a268a: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a2692: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a269b: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a26a4: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a26ad: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a26b6: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a26bc: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a26c2: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a26c8: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a26ce: 4881c4e8000000               add rsp, 0xe8
1407a26d5: 5b                           pop rbx
1407a26d6: 5f                           pop rdi
1407a26d7: 5e                           pop rsi
1407a26d8: 415c                         pop r12
1407a26da: 415d                         pop r13
1407a26dc: 415e                         pop r14
1407a26de: 415f                         pop r15
1407a26e0: 5d                           pop rbp
1407a26e1: c3                           ret 
1407a26e2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a26f0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a26f5: 55                           push rbp
1407a26f6: 4157                         push r15
1407a26f8: 4156                         push r14
1407a26fa: 4155                         push r13
1407a26fc: 4154                         push r12
1407a26fe: 56                           push rsi
1407a26ff: 57                           push rdi
1407a2700: 53                           push rbx
1407a2701: 4881ece8000000               sub rsp, 0xe8
1407a2708: 488daa80000000               lea rbp, [rdx + 0x80]
1407a270f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2715: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a271b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2721: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2727: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2730: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2739: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2742: 66440f7f8424b0000000         movdqa xmmword ptr [rsp + 0xb0], xmm8
1407a274c: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2754: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a275c: 4883bd4806000000             cmp qword ptr [rbp + 0x648], 0
1407a2764: 741d                         je 0x1407a2783
1407a2766: 488b9548060000               mov rdx, qword ptr [rbp + 0x648]
1407a276d: 48c1e202                     shl rdx, 2
1407a2771: 41b804000000                 mov r8d, 4
1407a2777: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
1407a277e: e89d14e7ff                   call 0x140613c20
1407a2783: c6855706000000               mov byte ptr [rbp + 0x657], 0
1407a278a: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a2792: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a279a: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a27a3: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a27ac: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a27b5: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a27be: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a27c4: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a27ca: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a27d0: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a27d6: 4881c4e8000000               add rsp, 0xe8
1407a27dd: 5b                           pop rbx
1407a27de: 5f                           pop rdi
1407a27df: 5e                           pop rsi
1407a27e0: 415c                         pop r12
1407a27e2: 415d                         pop r13
1407a27e4: 415e                         pop r14
1407a27e6: 415f                         pop r15
1407a27e8: 5d                           pop rbp
1407a27e9: c3                           ret 
1407a27ea: 660f1f440000                 nop word ptr [rax + rax]
1407a27f0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a27f5: 55                           push rbp
1407a27f6: 4157                         push r15
1407a27f8: 4156                         push r14
1407a27fa: 4155                         push r13
1407a27fc: 4154                         push r12
1407a27fe: 56                           push rsi
1407a27ff: 57                           push rdi
1407a2800: 53                           push rbx
1407a2801: 4881ece8000000               sub rsp, 0xe8
1407a2808: 488daa80000000               lea rbp, [rdx + 0x80]
1407a280f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2815: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a281b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2821: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2827: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2830: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2839: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2842: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a284b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2853: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a285b: 80bd5706000000               cmp byte ptr [rbp + 0x657], 0
1407a2862: 7422                         je 0x1407a2886
1407a2864: 488b95d0040000               mov rdx, qword ptr [rbp + 0x4d0]
1407a286b: 4885d2                       test rdx, rdx
1407a286e: 7416                         je 0x1407a2886
1407a2870: 488b8dd8040000               mov rcx, qword ptr [rbp + 0x4d8]
1407a2877: 48c1e202                     shl rdx, 2
1407a287b: 41b804000000                 mov r8d, 4
1407a2881: e89a13e7ff                   call 0x140613c20
1407a2886: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a288e: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a2896: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a289f: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a28a8: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a28b1: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a28ba: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a28c0: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a28c6: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a28cc: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a28d2: 4881c4e8000000               add rsp, 0xe8
1407a28d9: 5b                           pop rbx
1407a28da: 5f                           pop rdi
1407a28db: 5e                           pop rsi
1407a28dc: 415c                         pop r12
1407a28de: 415d                         pop r13
1407a28e0: 415e                         pop r14
1407a28e2: 415f                         pop r15
1407a28e4: 5d                           pop rbp
1407a28e5: c3                           ret 
1407a28e6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1407a28f0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a28f5: 55                           push rbp
1407a28f6: 4157                         push r15
1407a28f8: 4156                         push r14
1407a28fa: 4155                         push r13
1407a28fc: 4154                         push r12
1407a28fe: 56                           push rsi
1407a28ff: 57                           push rdi
1407a2900: 53                           push rbx
1407a2901: 4881ece8000000               sub rsp, 0xe8
1407a2908: 488daa80000000               lea rbp, [rdx + 0x80]
1407a290f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2915: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a291b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2921: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2927: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2930: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2939: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2942: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a294b: 660f7fbc24c0000000           movdqa xmmword ptr [rsp + 0xc0], xmm7
1407a2954: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a295c: 4883bd4806000000             cmp qword ptr [rbp + 0x648], 0
1407a2964: 741d                         je 0x1407a2983
1407a2966: 488b9548060000               mov rdx, qword ptr [rbp + 0x648]
1407a296d: 48c1e202                     shl rdx, 2
1407a2971: 41b804000000                 mov r8d, 4
1407a2977: 488b8d58060000               mov rcx, qword ptr [rbp + 0x658]
1407a297e: e89d12e7ff                   call 0x140613c20
1407a2983: c6856706000000               mov byte ptr [rbp + 0x667], 0
1407a298a: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a2992: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a299a: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a29a3: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a29ac: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a29b5: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a29be: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a29c4: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a29ca: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a29d0: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a29d6: 4881c4e8000000               add rsp, 0xe8
1407a29dd: 5b                           pop rbx
1407a29de: 5f                           pop rdi
1407a29df: 5e                           pop rsi
1407a29e0: 415c                         pop r12
1407a29e2: 415d                         pop r13
1407a29e4: 415e                         pop r14
1407a29e6: 415f                         pop r15
1407a29e8: 5d                           pop rbp
1407a29e9: c3                           ret 
1407a29ea: 660f1f440000                 nop word ptr [rax + rax]
1407a29f0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a29f5: 55                           push rbp
1407a29f6: 4157                         push r15
1407a29f8: 4156                         push r14
1407a29fa: 4155                         push r13
1407a29fc: 4154                         push r12
1407a29fe: 56                           push rsi
1407a29ff: 57                           push rdi
1407a2a00: 53                           push rbx
1407a2a01: 4881ece8000000               sub rsp, 0xe8
1407a2a08: 488daa80000000               lea rbp, [rdx + 0x80]
1407a2a0f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2a15: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a2a1b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2a21: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2a27: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2a30: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2a39: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2a42: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a2a4b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2a53: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a2a5b: 80bd6706000000               cmp byte ptr [rbp + 0x667], 0
1407a2a62: 7422                         je 0x1407a2a86
1407a2a64: 488b95d0040000               mov rdx, qword ptr [rbp + 0x4d0]
1407a2a6b: 4885d2                       test rdx, rdx
1407a2a6e: 7416                         je 0x1407a2a86
1407a2a70: 488b8dd8040000               mov rcx, qword ptr [rbp + 0x4d8]
1407a2a77: 48c1e202                     shl rdx, 2
1407a2a7b: 41b804000000                 mov r8d, 4
1407a2a81: e89a11e7ff                   call 0x140613c20
1407a2a86: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a2a8e: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a2a96: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a2a9f: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a2aa8: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a2ab1: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a2aba: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a2ac0: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a2ac6: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a2acc: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a2ad2: 4881c4e8000000               add rsp, 0xe8
1407a2ad9: 5b                           pop rbx
1407a2ada: 5f                           pop rdi
1407a2adb: 5e                           pop rsi
1407a2adc: 415c                         pop r12
1407a2ade: 415d                         pop r13
1407a2ae0: 415e                         pop r14
1407a2ae2: 415f                         pop r15
1407a2ae4: 5d                           pop rbp
1407a2ae5: c3                           ret 
1407a2ae6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1407a2af0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2af5: 55                           push rbp
1407a2af6: 4157                         push r15
1407a2af8: 4156                         push r14
1407a2afa: 4155                         push r13
1407a2afc: 4154                         push r12
1407a2afe: 56                           push rsi
1407a2aff: 57                           push rdi
1407a2b00: 53                           push rbx
1407a2b01: 4881ece8000000               sub rsp, 0xe8
1407a2b08: 488daa80000000               lea rbp, [rdx + 0x80]
1407a2b0f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2b15: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a2b1b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2b21: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2b27: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2b30: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2b39: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2b42: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a2b4b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2b53: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a2b5b: 4883bd5806000000             cmp qword ptr [rbp + 0x658], 0
1407a2b63: 741d                         je 0x1407a2b82
1407a2b65: 488b9558060000               mov rdx, qword ptr [rbp + 0x658]
1407a2b6c: 48c1e202                     shl rdx, 2
1407a2b70: 41b804000000                 mov r8d, 4
1407a2b76: 488b8d68060000               mov rcx, qword ptr [rbp + 0x668]
1407a2b7d: e89e10e7ff                   call 0x140613c20
1407a2b82: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a2b8a: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a2b92: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a2b9b: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a2ba4: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a2bad: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a2bb6: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a2bbc: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a2bc2: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a2bc8: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a2bce: 4881c4e8000000               add rsp, 0xe8
1407a2bd5: 5b                           pop rbx
1407a2bd6: 5f                           pop rdi
1407a2bd7: 5e                           pop rsi
1407a2bd8: 415c                         pop r12
1407a2bda: 415d                         pop r13
1407a2bdc: 415e                         pop r14
1407a2bde: 415f                         pop r15
1407a2be0: 5d                           pop rbp
1407a2be1: c3                           ret 
1407a2be2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a2bf0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2bf5: 55                           push rbp
1407a2bf6: 4157                         push r15
1407a2bf8: 4156                         push r14
1407a2bfa: 4155                         push r13
1407a2bfc: 4154                         push r12
1407a2bfe: 56                           push rsi
1407a2bff: 57                           push rdi
1407a2c00: 53                           push rbx
1407a2c01: 4881ece8000000               sub rsp, 0xe8
1407a2c08: 488daa80000000               lea rbp, [rdx + 0x80]
1407a2c0f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2c15: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a2c1b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2c21: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2c27: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2c30: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2c39: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2c42: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a2c4b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2c53: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a2c5b: 4883bd5806000000             cmp qword ptr [rbp + 0x658], 0
1407a2c63: 741d                         je 0x1407a2c82
1407a2c65: 488b9558060000               mov rdx, qword ptr [rbp + 0x658]
1407a2c6c: 48c1e202                     shl rdx, 2
1407a2c70: 41b804000000                 mov r8d, 4
1407a2c76: 488b8d68060000               mov rcx, qword ptr [rbp + 0x668]
1407a2c7d: e89e0fe7ff                   call 0x140613c20
1407a2c82: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a2c8a: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a2c92: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a2c9b: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a2ca4: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a2cad: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a2cb6: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a2cbc: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a2cc2: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a2cc8: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a2cce: 4881c4e8000000               add rsp, 0xe8
1407a2cd5: 5b                           pop rbx
1407a2cd6: 5f                           pop rdi
1407a2cd7: 5e                           pop rsi
1407a2cd8: 415c                         pop r12
1407a2cda: 415d                         pop r13
1407a2cdc: 415e                         pop r14
1407a2cde: 415f                         pop r15
1407a2ce0: 5d                           pop rbp
1407a2ce1: c3                           ret 
1407a2ce2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a2cf0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2cf5: 55                           push rbp
1407a2cf6: 4157                         push r15
1407a2cf8: 4156                         push r14
1407a2cfa: 4155                         push r13
1407a2cfc: 4154                         push r12
1407a2cfe: 56                           push rsi
1407a2cff: 57                           push rdi
1407a2d00: 53                           push rbx
1407a2d01: 4881ece8000000               sub rsp, 0xe8
1407a2d08: 488daa80000000               lea rbp, [rdx + 0x80]
1407a2d0f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2d15: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a2d1b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2d21: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2d27: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2d30: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2d39: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2d42: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a2d4b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2d53: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a2d5b: 4883bd6806000000             cmp qword ptr [rbp + 0x668], 0
1407a2d63: 7419                         je 0x1407a2d7e
1407a2d65: 41b801000000                 mov r8d, 1
1407a2d6b: 488b8d48060000               mov rcx, qword ptr [rbp + 0x648]
1407a2d72: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
1407a2d79: e8a20ee7ff                   call 0x140613c20
1407a2d7e: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a2d86: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a2d8e: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a2d97: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a2da0: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a2da9: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a2db2: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a2db8: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a2dbe: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a2dc4: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a2dca: 4881c4e8000000               add rsp, 0xe8
1407a2dd1: 5b                           pop rbx
1407a2dd2: 5f                           pop rdi
1407a2dd3: 5e                           pop rsi
1407a2dd4: 415c                         pop r12
1407a2dd6: 415d                         pop r13
1407a2dd8: 415e                         pop r14
1407a2dda: 415f                         pop r15
1407a2ddc: 5d                           pop rbp
1407a2ddd: c3                           ret 
1407a2dde: 6690                         nop 
1407a2de0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2de5: 55                           push rbp
1407a2de6: 4157                         push r15
1407a2de8: 4156                         push r14
1407a2dea: 4155                         push r13
1407a2dec: 4154                         push r12
1407a2dee: 56                           push rsi
1407a2def: 57                           push rdi
1407a2df0: 53                           push rbx
1407a2df1: 4881ece8000000               sub rsp, 0xe8
1407a2df8: 488daa80000000               lea rbp, [rdx + 0x80]
1407a2dff: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2e05: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a2e0b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2e11: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2e17: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2e20: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2e29: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2e32: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a2e3b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2e43: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a2e4b: 4883bd6806000000             cmp qword ptr [rbp + 0x668], 0
1407a2e53: 7419                         je 0x1407a2e6e
1407a2e55: 41b801000000                 mov r8d, 1
1407a2e5b: 488b8d48060000               mov rcx, qword ptr [rbp + 0x648]
1407a2e62: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
1407a2e69: e8b20de7ff                   call 0x140613c20
1407a2e6e: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a2e76: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a2e7e: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a2e87: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a2e90: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a2e99: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a2ea2: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a2ea8: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a2eae: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a2eb4: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a2eba: 4881c4e8000000               add rsp, 0xe8
1407a2ec1: 5b                           pop rbx
1407a2ec2: 5f                           pop rdi
1407a2ec3: 5e                           pop rsi
1407a2ec4: 415c                         pop r12
1407a2ec6: 415d                         pop r13
1407a2ec8: 415e                         pop r14
1407a2eca: 415f                         pop r15
1407a2ecc: 5d                           pop rbp
1407a2ecd: c3                           ret 
1407a2ece: 6690                         nop 
1407a2ed0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2ed5: 55                           push rbp
1407a2ed6: 4157                         push r15
1407a2ed8: 4156                         push r14
1407a2eda: 4155                         push r13
1407a2edc: 4154                         push r12
1407a2ede: 56                           push rsi
1407a2edf: 57                           push rdi
1407a2ee0: 53                           push rbx
1407a2ee1: 4881ece8000000               sub rsp, 0xe8
1407a2ee8: 488daa80000000               lea rbp, [rdx + 0x80]
1407a2eef: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2ef5: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a2efb: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2f01: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2f07: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a2f10: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a2f19: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a2f22: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a2f2b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a2f33: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a2f3b: 4883bd6806000000             cmp qword ptr [rbp + 0x668], 0
1407a2f43: 7419                         je 0x1407a2f5e
1407a2f45: 41b801000000                 mov r8d, 1
1407a2f4b: 488b8d48060000               mov rcx, qword ptr [rbp + 0x648]
1407a2f52: 488b9568060000               mov rdx, qword ptr [rbp + 0x668]
1407a2f59: e8c20ce7ff                   call 0x140613c20
1407a2f5e: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a2f66: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a2f6e: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a2f77: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a2f80: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a2f89: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a2f92: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a2f98: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a2f9e: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a2fa4: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a2faa: 4881c4e8000000               add rsp, 0xe8
1407a2fb1: 5b                           pop rbx
1407a2fb2: 5f                           pop rdi
1407a2fb3: 5e                           pop rsi
1407a2fb4: 415c                         pop r12
1407a2fb6: 415d                         pop r13
1407a2fb8: 415e                         pop r14
1407a2fba: 415f                         pop r15
1407a2fbc: 5d                           pop rbp
1407a2fbd: c3                           ret 
1407a2fbe: 6690                         nop 
1407a2fc0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a2fc5: 55                           push rbp
1407a2fc6: 4157                         push r15
1407a2fc8: 4156                         push r14
1407a2fca: 4155                         push r13
1407a2fcc: 4154                         push r12
1407a2fce: 56                           push rsi
1407a2fcf: 57                           push rdi
1407a2fd0: 53                           push rbx
1407a2fd1: 4881ece8000000               sub rsp, 0xe8
1407a2fd8: 488daa80000000               lea rbp, [rdx + 0x80]
1407a2fdf: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a2fe5: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a2feb: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a2ff1: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a2ff7: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a3000: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a3009: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a3012: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a301b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3023: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a302b: 80bd5406000000               cmp byte ptr [rbp + 0x654], 0
1407a3032: 7473                         je 0x1407a30a7
1407a3034: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a303b: e830240f00                   call 0x140895470
1407a3040: c6856606000001               mov byte ptr [rbp + 0x666], 1
1407a3047: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a304f: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3057: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3060: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3069: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a3072: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a307b: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a3081: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a3087: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a308d: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a3093: 4881c4e8000000               add rsp, 0xe8
1407a309a: 5b                           pop rbx
1407a309b: 5f                           pop rdi
1407a309c: 5e                           pop rsi
1407a309d: 415c                         pop r12
1407a309f: 415d                         pop r13
1407a30a1: 415e                         pop r14
1407a30a3: 415f                         pop r15
1407a30a5: 5d                           pop rbp
1407a30a6: c3                           ret 
1407a30a7: c6856606000001               mov byte ptr [rbp + 0x666], 1
1407a30ae: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a30b6: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a30be: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a30c7: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a30d0: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a30d9: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a30e2: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a30e8: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a30ee: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a30f4: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a30fa: 4881c4e8000000               add rsp, 0xe8
1407a3101: 5b                           pop rbx
1407a3102: 5f                           pop rdi
1407a3103: 5e                           pop rsi
1407a3104: 415c                         pop r12
1407a3106: 415d                         pop r13
1407a3108: 415e                         pop r14
1407a310a: 415f                         pop r15
1407a310c: 5d                           pop rbp
1407a310d: c3                           ret 
1407a310e: 6690                         nop 
1407a3110: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a3115: 55                           push rbp
1407a3116: 4157                         push r15
1407a3118: 4156                         push r14
1407a311a: 4155                         push r13
1407a311c: 4154                         push r12
1407a311e: 56                           push rsi
1407a311f: 57                           push rdi
1407a3120: 53                           push rbx
1407a3121: 4881ece8000000               sub rsp, 0xe8
1407a3128: 488daa80000000               lea rbp, [rdx + 0x80]
1407a312f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a3135: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a313b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a3141: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a3147: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a3150: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a3159: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a3162: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a316b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3173: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a317b: 488d8d80050000               lea rcx, [rbp + 0x580]
1407a3182: e899889aff                   call 0x14014ba20
1407a3187: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a318f: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3197: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a31a0: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a31a9: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a31b2: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a31bb: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a31c1: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a31c7: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a31cd: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a31d3: 4881c4e8000000               add rsp, 0xe8
1407a31da: 5b                           pop rbx
1407a31db: 5f                           pop rdi
1407a31dc: 5e                           pop rsi
1407a31dd: 415c                         pop r12
1407a31df: 415d                         pop r13
1407a31e1: 415e                         pop r14
1407a31e3: 415f                         pop r15
1407a31e5: 5d                           pop rbp
1407a31e6: c3                           ret 
1407a31e7: 660f1f840000000000           nop word ptr [rax + rax]
1407a31f0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a31f5: 55                           push rbp
1407a31f6: 4157                         push r15
1407a31f8: 4156                         push r14
1407a31fa: 4155                         push r13
1407a31fc: 4154                         push r12
1407a31fe: 56                           push rsi
1407a31ff: 57                           push rdi
1407a3200: 53                           push rbx
1407a3201: 4881ece8000000               sub rsp, 0xe8
1407a3208: 488daa80000000               lea rbp, [rdx + 0x80]
1407a320f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a3215: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a321b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a3221: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a3227: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a3230: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a3239: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a3242: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a324b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3253: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a325b: 80bd6606000000               cmp byte ptr [rbp + 0x666], 0
1407a3262: 0f8485000000                 je 0x1407a32ed
1407a3268: 488b9540020000               mov rdx, qword ptr [rbp + 0x240]
1407a326f: 4885d2                       test rdx, rdx
1407a3272: 7412                         je 0x1407a3286
1407a3274: 488b8d48020000               mov rcx, qword ptr [rbp + 0x248]
1407a327b: 41b801000000                 mov r8d, 1
1407a3281: e89a09e7ff                   call 0x140613c20
1407a3286: c6857706000000               mov byte ptr [rbp + 0x677], 0
1407a328d: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a3295: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a329d: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a32a6: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a32af: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a32b8: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a32c1: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a32c7: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a32cd: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a32d3: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a32d9: 4881c4e8000000               add rsp, 0xe8
1407a32e0: 5b                           pop rbx
1407a32e1: 5f                           pop rdi
1407a32e2: 5e                           pop rsi
1407a32e3: 415c                         pop r12
1407a32e5: 415d                         pop r13
1407a32e7: 415e                         pop r14
1407a32e9: 415f                         pop r15
1407a32eb: 5d                           pop rbp
1407a32ec: c3                           ret 
1407a32ed: c6857706000000               mov byte ptr [rbp + 0x677], 0
1407a32f4: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a32fc: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3304: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a330d: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3316: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a331f: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a3328: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a332e: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a3334: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a333a: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a3340: 4881c4e8000000               add rsp, 0xe8
1407a3347: 5b                           pop rbx
1407a3348: 5f                           pop rdi
1407a3349: 5e                           pop rsi
1407a334a: 415c                         pop r12
1407a334c: 415d                         pop r13
1407a334e: 415e                         pop r14
1407a3350: 415f                         pop r15
1407a3352: 5d                           pop rbp
1407a3353: c3                           ret 
1407a3354: 6666662e0f1f840000000000     nop word ptr cs:[rax + rax]
1407a3360: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a3365: 55                           push rbp
1407a3366: 4157                         push r15
1407a3368: 4156                         push r14
1407a336a: 4155                         push r13
1407a336c: 4154                         push r12
1407a336e: 56                           push rsi
1407a336f: 57                           push rdi
1407a3370: 53                           push rbx
1407a3371: 4881ece8000000               sub rsp, 0xe8
1407a3378: 488daa80000000               lea rbp, [rdx + 0x80]
1407a337f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a3385: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a338b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a3391: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a3397: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a33a0: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a33a9: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a33b2: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a33bb: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a33c3: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a33cb: 80bd5306000000               cmp byte ptr [rbp + 0x653], 0
1407a33d2: 7473                         je 0x1407a3447
1407a33d4: 488d8dc0030000               lea rcx, [rbp + 0x3c0]
1407a33db: e890200f00                   call 0x140895470
1407a33e0: c6856506000001               mov byte ptr [rbp + 0x665], 1
1407a33e7: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a33ef: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a33f7: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3400: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3409: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a3412: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a341b: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a3421: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a3427: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a342d: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a3433: 4881c4e8000000               add rsp, 0xe8
1407a343a: 5b                           pop rbx
1407a343b: 5f                           pop rdi
1407a343c: 5e                           pop rsi
1407a343d: 415c                         pop r12
1407a343f: 415d                         pop r13
1407a3441: 415e                         pop r14
1407a3443: 415f                         pop r15
1407a3445: 5d                           pop rbp
1407a3446: c3                           ret 
1407a3447: c6856506000001               mov byte ptr [rbp + 0x665], 1
1407a344e: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a3456: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a345e: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3467: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3470: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a3479: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a3482: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a3488: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a348e: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a3494: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a349a: 4881c4e8000000               add rsp, 0xe8
1407a34a1: 5b                           pop rbx
1407a34a2: 5f                           pop rdi
1407a34a3: 5e                           pop rsi
1407a34a4: 415c                         pop r12
1407a34a6: 415d                         pop r13
1407a34a8: 415e                         pop r14
1407a34aa: 415f                         pop r15
1407a34ac: 5d                           pop rbp
1407a34ad: c3                           ret 
1407a34ae: 6690                         nop 
1407a34b0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a34b5: 55                           push rbp
1407a34b6: 4157                         push r15
1407a34b8: 4156                         push r14
1407a34ba: 4155                         push r13
1407a34bc: 4154                         push r12
1407a34be: 56                           push rsi
1407a34bf: 57                           push rdi
1407a34c0: 53                           push rbx
1407a34c1: 4881ece8000000               sub rsp, 0xe8
1407a34c8: 488daa80000000               lea rbp, [rdx + 0x80]
1407a34cf: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a34d5: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a34db: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a34e1: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a34e7: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a34f0: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a34f9: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a3502: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a350b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3513: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a351b: 488b9548060000               mov rdx, qword ptr [rbp + 0x648]
1407a3522: 4885d2                       test rdx, rdx
1407a3525: 7412                         je 0x1407a3539
1407a3527: 41b801000000                 mov r8d, 1
1407a352d: 488b8d60050000               mov rcx, qword ptr [rbp + 0x560]
1407a3534: e8e706e7ff                   call 0x140613c20
1407a3539: c6857706000001               mov byte ptr [rbp + 0x677], 1
1407a3540: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a3548: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3550: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3559: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3562: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a356b: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a3574: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a357a: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a3580: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a3586: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a358c: 4881c4e8000000               add rsp, 0xe8
1407a3593: 5b                           pop rbx
1407a3594: 5f                           pop rdi
1407a3595: 5e                           pop rsi
1407a3596: 415c                         pop r12
1407a3598: 415d                         pop r13
1407a359a: 415e                         pop r14
1407a359c: 415f                         pop r15
1407a359e: 5d                           pop rbp
1407a359f: c3                           ret 
1407a35a0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a35a5: 55                           push rbp
1407a35a6: 4157                         push r15
1407a35a8: 4156                         push r14
1407a35aa: 4155                         push r13
1407a35ac: 4154                         push r12
1407a35ae: 56                           push rsi
1407a35af: 57                           push rdi
1407a35b0: 53                           push rbx
1407a35b1: 4881ece8000000               sub rsp, 0xe8
1407a35b8: 488daa80000000               lea rbp, [rdx + 0x80]
1407a35bf: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a35c5: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a35cb: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a35d1: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a35d7: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a35e0: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a35e9: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a35f2: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a35fb: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3603: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a360b: 80bd6506000000               cmp byte ptr [rbp + 0x665], 0
1407a3612: 741e                         je 0x1407a3632
1407a3614: 488b9540020000               mov rdx, qword ptr [rbp + 0x240]
1407a361b: 4885d2                       test rdx, rdx
1407a361e: 7412                         je 0x1407a3632
1407a3620: 488b8d48020000               mov rcx, qword ptr [rbp + 0x248]
1407a3627: 41b801000000                 mov r8d, 1
1407a362d: e8ee05e7ff                   call 0x140613c20
1407a3632: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a363a: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3642: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a364b: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3654: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a365d: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a3666: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a366c: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a3672: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a3678: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a367e: 4881c4e8000000               add rsp, 0xe8
1407a3685: 5b                           pop rbx
1407a3686: 5f                           pop rdi
1407a3687: 5e                           pop rsi
1407a3688: 415c                         pop r12
1407a368a: 415d                         pop r13
1407a368c: 415e                         pop r14
1407a368e: 415f                         pop r15
1407a3690: 5d                           pop rbp
1407a3691: c3                           ret 
1407a3692: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a36a0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a36a5: 55                           push rbp
1407a36a6: 4157                         push r15
1407a36a8: 4156                         push r14
1407a36aa: 4155                         push r13
1407a36ac: 4154                         push r12
1407a36ae: 56                           push rsi
1407a36af: 57                           push rdi
1407a36b0: 53                           push rbx
1407a36b1: 4881ece8000000               sub rsp, 0xe8
1407a36b8: 488daa80000000               lea rbp, [rdx + 0x80]
1407a36bf: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a36c5: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a36cb: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a36d1: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a36d7: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a36e0: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a36e9: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a36f2: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a36fb: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3703: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a370b: 80bd7706000000               cmp byte ptr [rbp + 0x677], 0
1407a3712: 7426                         je 0x1407a373a
1407a3714: 488b8530030000               mov rax, qword ptr [rbp + 0x330]
1407a371b: 4885c0                       test rax, rax
1407a371e: 741a                         je 0x1407a373a
1407a3720: 488b8d38030000               mov rcx, qword ptr [rbp + 0x338]
1407a3727: 48c1e002                     shl rax, 2
1407a372b: 488d14c0                     lea rdx, [rax + rax*8]
1407a372f: 41b804000000                 mov r8d, 4
1407a3735: e8e604e7ff                   call 0x140613c20
1407a373a: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a3742: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a374a: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3753: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a375c: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a3765: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a376e: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a3774: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a377a: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a3780: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a3786: 4881c4e8000000               add rsp, 0xe8
1407a378d: 5b                           pop rbx
1407a378e: 5f                           pop rdi
1407a378f: 5e                           pop rsi
1407a3790: 415c                         pop r12
1407a3792: 415d                         pop r13
1407a3794: 415e                         pop r14
1407a3796: 415f                         pop r15
1407a3798: 5d                           pop rbp
1407a3799: c3                           ret 
1407a379a: 660f1f440000                 nop word ptr [rax + rax]
1407a37a0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a37a5: 55                           push rbp
1407a37a6: 4157                         push r15
1407a37a8: 4156                         push r14
1407a37aa: 4155                         push r13
1407a37ac: 4154                         push r12
1407a37ae: 56                           push rsi
1407a37af: 57                           push rdi
1407a37b0: 53                           push rbx
1407a37b1: 4881ece8000000               sub rsp, 0xe8
1407a37b8: 488daa80000000               lea rbp, [rdx + 0x80]
1407a37bf: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a37c5: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a37cb: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a37d1: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a37d7: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a37e0: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a37e9: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a37f2: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a37fb: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3803: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a380b: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
1407a3812: 483b8558060000               cmp rax, qword ptr [rbp + 0x658]
1407a3819: 7560                         jne 0x1407a387b
1407a381b: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a3823: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a382b: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3834: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a383d: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a3846: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a384f: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a3855: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a385b: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a3861: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a3867: 4881c4e8000000               add rsp, 0xe8
1407a386e: 5b                           pop rbx
1407a386f: 5f                           pop rdi
1407a3870: 5e                           pop rsi
1407a3871: 415c                         pop r12
1407a3873: 415d                         pop r13
1407a3875: 415e                         pop r14
1407a3877: 415f                         pop r15
1407a3879: 5d                           pop rbp
1407a387a: c3                           ret 
1407a387b: 488b85b8020000               mov rax, qword ptr [rbp + 0x2b8]
1407a3882: 4885c0                       test rax, rax
1407a3885: 741a                         je 0x1407a38a1
1407a3887: 488b8dc0020000               mov rcx, qword ptr [rbp + 0x2c0]
1407a388e: 48c1e002                     shl rax, 2
1407a3892: 488d1440                     lea rdx, [rax + rax*2]
1407a3896: 41b804000000                 mov r8d, 4
1407a389c: e87f03e7ff                   call 0x140613c20
1407a38a1: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a38a9: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a38b1: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a38ba: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a38c3: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a38cc: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a38d5: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a38db: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a38e1: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a38e7: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a38ed: 4881c4e8000000               add rsp, 0xe8
1407a38f4: 5b                           pop rbx
1407a38f5: 5f                           pop rdi
1407a38f6: 5e                           pop rsi
1407a38f7: 415c                         pop r12
1407a38f9: 415d                         pop r13
1407a38fb: 415e                         pop r14
1407a38fd: 415f                         pop r15
1407a38ff: 5d                           pop rbp
1407a3900: c3                           ret 
1407a3901: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407a3910: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a3915: 55                           push rbp
1407a3916: 4157                         push r15
1407a3918: 4156                         push r14
1407a391a: 4155                         push r13
1407a391c: 4154                         push r12
1407a391e: 56                           push rsi
1407a391f: 57                           push rdi
1407a3920: 53                           push rbx
1407a3921: 4881ece8000000               sub rsp, 0xe8
1407a3928: 488daa80000000               lea rbp, [rdx + 0x80]
1407a392f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a3935: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a393b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a3941: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a3947: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a3950: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a3959: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a3962: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a396b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3973: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a397b: 488d8dd0040000               lea rcx, [rbp + 0x4d0]
1407a3982: e899809aff                   call 0x14014ba20
1407a3987: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a398f: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3997: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a39a0: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a39a9: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a39b2: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a39bb: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a39c1: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a39c7: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a39cd: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a39d3: 4881c4e8000000               add rsp, 0xe8
1407a39da: 5b                           pop rbx
1407a39db: 5f                           pop rdi
1407a39dc: 5e                           pop rsi
1407a39dd: 415c                         pop r12
1407a39df: 415d                         pop r13
1407a39e1: 415e                         pop r14
1407a39e3: 415f                         pop r15
1407a39e5: 5d                           pop rbp
1407a39e6: c3                           ret 
1407a39e7: 660f1f840000000000           nop word ptr [rax + rax]
1407a39f0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a39f5: 55                           push rbp
1407a39f6: 4157                         push r15
1407a39f8: 4156                         push r14
1407a39fa: 4155                         push r13
1407a39fc: 4154                         push r12
1407a39fe: 56                           push rsi
1407a39ff: 57                           push rdi
1407a3a00: 53                           push rbx
1407a3a01: 4881ece8000000               sub rsp, 0xe8
1407a3a08: 488daa80000000               lea rbp, [rdx + 0x80]
1407a3a0f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a3a15: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a3a1b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a3a21: 66440f7f642470               movdqa xmmword ptr [rsp + 0x70], xmm12
1407a3a28: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a3a31: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a3a3a: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a3a43: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a3a4c: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3a54: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a3a5c: 488b95b0030000               mov rdx, qword ptr [rbp + 0x3b0]
1407a3a63: 4885d2                       test rdx, rdx
1407a3a66: 7416                         je 0x1407a3a7e
1407a3a68: 48c1e202                     shl rdx, 2
1407a3a6c: 41b804000000                 mov r8d, 4
1407a3a72: 488b8d80040000               mov rcx, qword ptr [rbp + 0x480]
1407a3a79: e8a201e7ff                   call 0x140613c20
1407a3a7e: c6855606000000               mov byte ptr [rbp + 0x656], 0
1407a3a85: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a3a8d: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3a95: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3a9e: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3aa7: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a3ab0: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a3ab9: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a3abf: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a3ac5: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a3acb: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a3ad1: 4881c4e8000000               add rsp, 0xe8
1407a3ad8: 5b                           pop rbx
1407a3ad9: 5f                           pop rdi
1407a3ada: 5e                           pop rsi
1407a3adb: 415c                         pop r12
1407a3add: 415d                         pop r13
1407a3adf: 415e                         pop r14
1407a3ae1: 415f                         pop r15
1407a3ae3: 5d                           pop rbp
1407a3ae4: c3                           ret 
1407a3ae5: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
1407a3af0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a3af5: 55                           push rbp
1407a3af6: 4157                         push r15
1407a3af8: 4156                         push r14
1407a3afa: 4155                         push r13
1407a3afc: 4154                         push r12
1407a3afe: 56                           push rsi
1407a3aff: 57                           push rdi
1407a3b00: 53                           push rbx
1407a3b01: 4881ece8000000               sub rsp, 0xe8
1407a3b08: 488daa80000000               lea rbp, [rdx + 0x80]
1407a3b0f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a3b15: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a3b1b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a3b21: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a3b27: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a3b30: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a3b39: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a3b42: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a3b4b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3b53: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a3b5b: 80bd5606000000               cmp byte ptr [rbp + 0x656], 0
1407a3b62: 7422                         je 0x1407a3b86
1407a3b64: 488b95c0030000               mov rdx, qword ptr [rbp + 0x3c0]
1407a3b6b: 4885d2                       test rdx, rdx
1407a3b6e: 7416                         je 0x1407a3b86
1407a3b70: 488b8dc8030000               mov rcx, qword ptr [rbp + 0x3c8]
1407a3b77: 48c1e202                     shl rdx, 2
1407a3b7b: 41b804000000                 mov r8d, 4
1407a3b81: e89a00e7ff                   call 0x140613c20
1407a3b86: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a3b8e: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3b96: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3b9f: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3ba8: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a3bb1: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a3bba: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a3bc0: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a3bc6: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a3bcc: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a3bd2: 4881c4e8000000               add rsp, 0xe8
1407a3bd9: 5b                           pop rbx
1407a3bda: 5f                           pop rdi
1407a3bdb: 5e                           pop rsi
1407a3bdc: 415c                         pop r12
1407a3bde: 415d                         pop r13
1407a3be0: 415e                         pop r14
1407a3be2: 415f                         pop r15
1407a3be4: 5d                           pop rbp
1407a3be5: c3                           ret 
1407a3be6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1407a3bf0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a3bf5: 55                           push rbp
1407a3bf6: 4157                         push r15
1407a3bf8: 4156                         push r14
1407a3bfa: 4155                         push r13
1407a3bfc: 4154                         push r12
1407a3bfe: 56                           push rsi
1407a3bff: 57                           push rdi
1407a3c00: 53                           push rbx
1407a3c01: 4881ece8000000               sub rsp, 0xe8
1407a3c08: 488daa80000000               lea rbp, [rdx + 0x80]
1407a3c0f: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a3c15: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a3c1b: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a3c21: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a3c27: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a3c30: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a3c39: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a3c42: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a3c4b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3c53: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a3c5b: 488d8d40020000               lea rcx, [rbp + 0x240]
1407a3c62: e889e8e2ff                   call 0x1405d24f0
1407a3c67: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a3c6f: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3c77: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3c80: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3c89: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a3c92: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a3c9b: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a3ca1: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a3ca7: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a3cad: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a3cb3: 4881c4e8000000               add rsp, 0xe8
1407a3cba: 5b                           pop rbx
1407a3cbb: 5f                           pop rdi
1407a3cbc: 5e                           pop rsi
1407a3cbd: 415c                         pop r12
1407a3cbf: 415d                         pop r13
1407a3cc1: 415e                         pop r14
1407a3cc3: 415f                         pop r15
1407a3cc5: 5d                           pop rbp
1407a3cc6: c3                           ret 
1407a3cc7: 660f1f840000000000           nop word ptr [rax + rax]
1407a3cd0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1407a3cd5: 55                           push rbp
1407a3cd6: 4157                         push r15
1407a3cd8: 4156                         push r14
1407a3cda: 4155                         push r13
1407a3cdc: 4154                         push r12
1407a3cde: 56                           push rsi
1407a3cdf: 57                           push rdi
1407a3ce0: 53                           push rbx
1407a3ce1: 4881ece8000000               sub rsp, 0xe8
1407a3ce8: 488daa80000000               lea rbp, [rdx + 0x80]
1407a3cef: 440f297c2440                 movaps xmmword ptr [rsp + 0x40], xmm15
1407a3cf5: 440f29742450                 movaps xmmword ptr [rsp + 0x50], xmm14
1407a3cfb: 440f296c2460                 movaps xmmword ptr [rsp + 0x60], xmm13
1407a3d01: 440f29642470                 movaps xmmword ptr [rsp + 0x70], xmm12
1407a3d07: 440f299c2480000000           movaps xmmword ptr [rsp + 0x80], xmm11
1407a3d10: 440f29942490000000           movaps xmmword ptr [rsp + 0x90], xmm10
1407a3d19: 440f298c24a0000000           movaps xmmword ptr [rsp + 0xa0], xmm9
1407a3d22: 440f298424b0000000           movaps xmmword ptr [rsp + 0xb0], xmm8
1407a3d2b: 0f29bc24c0000000             movaps xmmword ptr [rsp + 0xc0], xmm7
1407a3d33: 0f29b424d0000000             movaps xmmword ptr [rsp + 0xd0], xmm6
1407a3d3b: 488d8d30030000               lea rcx, [rbp + 0x330]
1407a3d42: e8d97c9aff                   call 0x14014ba20
1407a3d47: 0f28b424d0000000             movaps xmm6, xmmword ptr [rsp + 0xd0]
1407a3d4f: 0f28bc24c0000000             movaps xmm7, xmmword ptr [rsp + 0xc0]
1407a3d57: 440f288424b0000000           movaps xmm8, xmmword ptr [rsp + 0xb0]
1407a3d60: 440f288c24a0000000           movaps xmm9, xmmword ptr [rsp + 0xa0]
1407a3d69: 440f28942490000000           movaps xmm10, xmmword ptr [rsp + 0x90]
1407a3d72: 440f289c2480000000           movaps xmm11, xmmword ptr [rsp + 0x80]
1407a3d7b: 440f28642470                 movaps xmm12, xmmword ptr [rsp + 0x70]
1407a3d81: 440f286c2460                 movaps xmm13, xmmword ptr [rsp + 0x60]
1407a3d87: 440f28742450                 movaps xmm14, xmmword ptr [rsp + 0x50]
1407a3d8d: 440f287c2440                 movaps xmm15, xmmword ptr [rsp + 0x40]
1407a3d93: 4881c4e8000000               add rsp, 0xe8
1407a3d9a: 5b                           pop rbx
1407a3d9b: 5f                           pop rdi
1407a3d9c: 5e                           pop rsi
1407a3d9d: 415c                         pop r12
1407a3d9f: 415d                         pop r13
1407a3da1: 415e                         pop r14
1407a3da3: 415f                         pop r15
1407a3da5: 5d                           pop rbp
1407a3da6: c3                           ret 
