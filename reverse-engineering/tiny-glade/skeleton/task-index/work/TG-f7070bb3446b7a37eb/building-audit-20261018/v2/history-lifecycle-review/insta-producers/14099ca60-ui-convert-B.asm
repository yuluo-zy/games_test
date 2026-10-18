14099ca60: 55                           push rbp
14099ca61: 4157                         push r15
14099ca63: 4156                         push r14
14099ca65: 4155                         push r13
14099ca67: 4154                         push r12
14099ca69: 56                           push rsi
14099ca6a: 57                           push rdi
14099ca6b: 53                           push rbx
14099ca6c: 4881ec78030000               sub rsp, 0x378
14099ca73: 488dac2480000000             lea rbp, [rsp + 0x80]
14099ca7b: 440f2985e0020000             movaps xmmword ptr [rbp + 0x2e0], xmm8
14099ca83: 0f29bdd0020000               movaps xmmword ptr [rbp + 0x2d0], xmm7
14099ca8a: 660f7fb5c0020000             movdqa xmmword ptr [rbp + 0x2c0], xmm6
14099ca92: 48c785b8020000feffffff       mov qword ptr [rbp + 0x2b8], 0xfffffffffffffffe
14099ca9d: 4c898d90010000               mov qword ptr [rbp + 0x190], r9
14099caa4: 4c898598010000               mov qword ptr [rbp + 0x198], r8
14099caab: 488995e0000000               mov qword ptr [rbp + 0xe0], rdx
14099cab2: 48898d88020000               mov qword ptr [rbp + 0x288], rcx
14099cab9: 4c8b8d68030000               mov r9, qword ptr [rbp + 0x368]
14099cac0: 488b8590030000               mov rax, qword ptr [rbp + 0x390]
14099cac7: 488b9570030000               mov rdx, qword ptr [rbp + 0x370]
14099cace: 410f104148                   movups xmm0, xmmword ptr [r9 + 0x48]
14099cad3: 410f104958                   movups xmm1, xmmword ptr [r9 + 0x58]
14099cad8: 0f294d20                     movaps xmmword ptr [rbp + 0x20], xmm1
14099cadc: 0f294510                     movaps xmmword ptr [rbp + 0x10], xmm0
14099cae0: 488b3a                       mov rdi, qword ptr [rdx]
14099cae3: 488b5a10                     mov rbx, qword ptr [rdx + 0x10]
14099cae7: 8b4a1c                       mov ecx, dword ptr [rdx + 0x1c]
14099caea: 898d64020000                 mov dword ptr [rbp + 0x264], ecx
14099caf0: 4c8b6220                     mov r12, qword ptr [rdx + 0x20]
14099caf4: 488b4a30                     mov rcx, qword ptr [rdx + 0x30]
14099caf8: 4c8b4070                     mov r8, qword ptr [rax + 0x70]
14099cafc: 4c894540                     mov qword ptr [rbp + 0x40], r8
14099cb00: 448b423c                     mov r8d, dword ptr [rdx + 0x3c]
14099cb04: 4c8b5240                     mov r10, qword ptr [rdx + 0x40]
14099cb08: 4c8995a8020000               mov qword ptr [rbp + 0x2a8], r10
14099cb0f: 488b7248                     mov rsi, qword ptr [rdx + 0x48]
14099cb13: 488b5258                     mov rdx, qword ptr [rdx + 0x58]
14099cb17: 48899580000000               mov qword ptr [rbp + 0x80], rdx
14099cb1e: 0f104040                     movups xmm0, xmmword ptr [rax + 0x40]
14099cb22: 0f104850                     movups xmm1, xmmword ptr [rax + 0x50]
14099cb26: 0f294d00                     movaps xmmword ptr [rbp], xmm1
14099cb2a: 0f2945f0                     movaps xmmword ptr [rbp - 0x10], xmm0
14099cb2e: 0f104060                     movups xmm0, xmmword ptr [rax + 0x60]
14099cb32: 0f294530                     movaps xmmword ptr [rbp + 0x30], xmm0
14099cb36: f30f6f4078                   movdqu xmm0, xmmword ptr [rax + 0x78]
14099cb3b: f30f6f8888000000             movdqu xmm1, xmmword ptr [rax + 0x88]
14099cb43: 660f7f4de0                   movdqa xmmword ptr [rbp - 0x20], xmm1
14099cb48: 660f7f45d0                   movdqa xmmword ptr [rbp - 0x30], xmm0
14099cb4d: 488b10                       mov rdx, qword ptr [rax]
14099cb50: 488995c0000000               mov qword ptr [rbp + 0xc0], rdx
14099cb57: 488b5020                     mov rdx, qword ptr [rax + 0x20]
14099cb5b: 488995c8000000               mov qword ptr [rbp + 0xc8], rdx
14099cb62: 488b5030                     mov rdx, qword ptr [rax + 0x30]
14099cb66: 488995d0000000               mov qword ptr [rbp + 0xd0], rdx
14099cb6d: 8b503c                       mov edx, dword ptr [rax + 0x3c]
14099cb70: 8995d8000000                 mov dword ptr [rbp + 0xd8], edx
14099cb76: 4c8bb098000000               mov r14, qword ptr [rax + 0x98]
14099cb7d: 498b4120                     mov rax, qword ptr [r9 + 0x20]
14099cb81: 48898590020000               mov qword ptr [rbp + 0x290], rax
14099cb88: 498b4128                     mov rax, qword ptr [r9 + 0x28]
14099cb8c: 48898588010000               mov qword ptr [rbp + 0x188], rax
14099cb93: 448901                       mov dword ptr [rcx], r8d
14099cb96: e8659e2700                   call 0x140c16a00
14099cb9b: 888598020000                 mov byte ptr [rbp + 0x298], al
14099cba1: 84c0                         test al, al
14099cba3: 4c897560                     mov qword ptr [rbp + 0x60], r14
14099cba7: 4889bd88000000               mov qword ptr [rbp + 0x88], rdi
14099cbae: 48899d90000000               mov qword ptr [rbp + 0x90], rbx
14099cbb5: 745d                         je 0x14099cc14
14099cbb7: 41bf56000000                 mov r15d, 0x56
14099cbbd: b956000000                   mov ecx, 0x56
14099cbc2: e829f4ffff                   call 0x14099bff0
14099cbc7: 4c8d2db2ed1102               lea r13, [rip + 0x211edb2]
14099cbce: 4883f801                     cmp rax, 1
14099cbd2: 0f85a8000000                 jne 0x14099cc80
14099cbd8: 4989d1                       mov r9, rdx
14099cbdb: 4885d2                       test rdx, rdx
14099cbde: 743d                         je 0x14099cc1d
14099cbe0: 4983f956                     cmp r9, 0x56
14099cbe4: 7335                         jae 0x14099cc1b
14099cbe6: 488d0593ed1102               lea rax, [rip + 0x211ed93]
14099cbed: 41803c01bf                   cmp byte ptr [r9 + rax], 0xbf
14099cbf2: 7f29                         jg 0x14099cc1d
14099cbf4: 488d0565ee1102               lea rax, [rip + 0x211ee65]
14099cbfb: 4889442420                   mov qword ptr [rsp + 0x20], rax
14099cc00: 488d0d79ed1102               lea rcx, [rip + 0x211ed79]
14099cc07: ba56000000                   mov edx, 0x56
14099cc0c: 4531c0                       xor r8d, r8d
14099cc0f: e82ccdf301                   call 0x1428d9940
14099cc14: 31c0                         xor eax, eax
14099cc16: e9b6010000                   jmp 0x14099cdd1
14099cc1b: 75d7                         jne 0x14099cbf4
14099cc1d: 4c89c9                       mov rcx, r9
14099cc20: e8cbf3ffff                   call 0x14099bff0
14099cc25: 4883f801                     cmp rax, 1
14099cc29: 7555                         jne 0x14099cc80
14099cc2b: 4989d0                       mov r8, rdx
14099cc2e: 4983c002                     add r8, 2
14099cc32: 7439                         je 0x14099cc6d
14099cc34: 4983f856                     cmp r8, 0x56
14099cc38: 7331                         jae 0x14099cc6b
14099cc3a: 488d053fed1102               lea rax, [rip + 0x211ed3f]
14099cc41: 41803c00bf                   cmp byte ptr [r8 + rax], 0xbf
14099cc46: 7f25                         jg 0x14099cc6d
14099cc48: 488d0529ee1102               lea rax, [rip + 0x211ee29]
14099cc4f: 4889442420                   mov qword ptr [rsp + 0x20], rax
14099cc54: 488d0d25ed1102               lea rcx, [rip + 0x211ed25]
14099cc5b: ba56000000                   mov edx, 0x56
14099cc60: 41b956000000                 mov r9d, 0x56
14099cc66: e8d5ccf301                   call 0x1428d9940
14099cc6b: 75db                         jne 0x14099cc48
14099cc6d: 41bf54000000                 mov r15d, 0x54
14099cc73: 4929d7                       sub r15, rdx
14099cc76: 4c8d2d03ed1102               lea r13, [rip + 0x211ed03]
14099cc7d: 4d01c5                       add r13, r8
14099cc80: 4c8d05a2ee1102               lea r8, [rip + 0x211eea2]
14099cc87: 488d1d5fee1102               lea rbx, [rip + 0x211ee5f]
14099cc8e: 6690                         nop 
14099cc90: 410fbe40ff                   movsx eax, byte ptr [r8 - 1]
14099cc95: 85c0                         test eax, eax
14099cc97: 7817                         js 0x14099ccb0
14099cc99: 49ffc8                       dec r8
14099cc9c: 83f85c                       cmp eax, 0x5c
14099cc9f: 7563                         jne 0x14099cd04
14099cca1: eb72                         jmp 0x14099cd15
14099cca3: 666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
14099ccb0: 410fb648fe                   movzx ecx, byte ptr [r8 - 2]
14099ccb5: 80f9c0                       cmp cl, 0xc0
14099ccb8: 7d23                         jge 0x14099ccdd
14099ccba: 410fb650fd                   movzx edx, byte ptr [r8 - 3]
14099ccbf: 80fac0                       cmp dl, 0xc0
14099ccc2: 7d22                         jge 0x14099cce6
14099ccc4: 450fb648fc                   movzx r9d, byte ptr [r8 - 4]
14099ccc9: 4983c0fc                     add r8, -4
14099cccd: 4183e107                     and r9d, 7
14099ccd1: 41c1e106                     shl r9d, 6
14099ccd5: 83e23f                       and edx, 0x3f
14099ccd8: 4409ca                       or edx, r9d
14099ccdb: eb10                         jmp 0x14099cced
14099ccdd: 4983c0fe                     add r8, -2
14099cce1: 83e11f                       and ecx, 0x1f
14099cce4: eb0f                         jmp 0x14099ccf5
14099cce6: 4983c0fd                     add r8, -3
14099ccea: 83e20f                       and edx, 0xf
14099cced: c1e206                       shl edx, 6
14099ccf0: 83e13f                       and ecx, 0x3f
14099ccf3: 09d1                         or ecx, edx
14099ccf5: c1e106                       shl ecx, 6
14099ccf8: 243f                         and al, 0x3f
14099ccfa: 0fb6c0                       movzx eax, al
14099ccfd: 09c8                         or eax, ecx
14099ccff: 83f85c                       cmp eax, 0x5c
14099cd02: 7411                         je 0x14099cd15
14099cd04: 83f82f                       cmp eax, 0x2f
14099cd07: 740c                         je 0x14099cd15
14099cd09: 4939d8                       cmp r8, rbx
14099cd0c: 7582                         jne 0x14099cc90
14099cd0e: bf3c000000                   mov edi, 0x3c
14099cd13: eb4c                         jmp 0x14099cd61
14099cd15: 488d1dd1ed1102               lea rbx, [rip + 0x211edd1]
14099cd1c: 4929d8                       sub r8, rbx
14099cd1f: 49ffc0                       inc r8
14099cd22: 7432                         je 0x14099cd56
14099cd24: 4983f83c                     cmp r8, 0x3c
14099cd28: 732a                         jae 0x14099cd54
14099cd2a: 41803c18bf                   cmp byte ptr [r8 + rbx], 0xbf
14099cd2f: 7f25                         jg 0x14099cd56
14099cd31: 488d0508ed1102               lea rax, [rip + 0x211ed08]
14099cd38: 4889442420                   mov qword ptr [rsp + 0x20], rax
14099cd3d: 488d0da9ed1102               lea rcx, [rip + 0x211eda9]
14099cd44: ba3c000000                   mov edx, 0x3c
14099cd49: 41b93c000000                 mov r9d, 0x3c
14099cd4f: e8eccbf301                   call 0x1428d9940
14099cd54: 75db                         jne 0x14099cd31
14099cd56: bf3c000000                   mov edi, 0x3c
14099cd5b: 4c29c7                       sub rdi, r8
14099cd5e: 4c01c3                       add rbx, r8
14099cd61: e8cab32700                   call 0x140c18130
14099cd66: 4889c1                       mov rcx, rax
14099cd69: 488b00                       mov rax, qword ptr [rax]
14099cd6c: 4883f801                     cmp rax, 1
14099cd70: 0f85d90c0000                 jne 0x14099da4f
14099cd76: 4883c108                     add rcx, 8
14099cd7a: 48833900                     cmp qword ptr [rcx], 0
14099cd7e: 0f85fa0f0000                 jne 0x14099dd7e
14099cd84: 48c701ffffffff               mov qword ptr [rcx], 0xffffffffffffffff
14099cd8b: 48898d80020000               mov qword ptr [rbp + 0x280], rcx
14099cd92: 4883c108                     add rcx, 8
14099cd96: 48897c2420                   mov qword ptr [rsp + 0x20], rdi
14099cd9b: 48c744243000000000           mov qword ptr [rsp + 0x30], 0
14099cda4: 48c744242801000000           mov qword ptr [rsp + 0x28], 1
14099cdad: 4c89ea                       mov rdx, r13
14099cdb0: 4d89f8                       mov r8, r15
14099cdb3: 4989d9                       mov r9, rbx
14099cdb6: e8859f2700                   call 0x140c16d40
14099cdbb: 488b8d80020000               mov rcx, qword ptr [rbp + 0x280]
14099cdc2: 48ff01                       inc qword ptr [rcx]
14099cdc5: 48898550010000               mov qword ptr [rbp + 0x150], rax
14099cdcc: b801000000                   mov eax, 1
14099cdd1: 48898548010000               mov qword ptr [rbp + 0x148], rax
14099cdd8: 498b7c2418                   mov rdi, qword ptr [r12 + 0x18]
14099cddd: 4885ff                       test rdi, rdi
14099cde0: 4c89a580020000               mov qword ptr [rbp + 0x280], r12
14099cde7: 0f84db000000                 je 0x14099cec8
14099cded: 498b1c24                     mov rbx, qword ptr [r12]
14099cdf1: 660f6f03                     movdqa xmm0, xmmword ptr [rbx]
14099cdf5: 66440fd7e0                   pmovmskb r12d, xmm0
14099cdfa: 41f7d4                       not r12d
14099cdfd: 4c8d7310                     lea r14, [rbx + 0x10]
14099ce01: 4989df                       mov r15, rbx
14099ce04: eb1a                         jmp 0x14099ce20
14099ce06: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
14099ce10: 418d4424ff                   lea eax, [r12 - 1]
14099ce15: 4421e0                       and eax, r12d
14099ce18: 4189c4                       mov r12d, eax
14099ce1b: 48ffcf                       dec rdi
14099ce1e: 7461                         je 0x14099ce81
14099ce20: 664585e4                     test r12w, r12w
14099ce24: 752b                         jne 0x14099ce51
14099ce26: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
14099ce30: 66410f6f06                   movdqa xmm0, xmmword ptr [r14]
14099ce35: 66440fd7e0                   pmovmskb r12d, xmm0
14099ce3a: 4981c700feffff               add r15, -0x200
14099ce41: 4983c610                     add r14, 0x10
14099ce45: 4181fcffff0000               cmp r12d, 0xffff
14099ce4c: 74e2                         je 0x14099ce30
14099ce4e: 41f7d4                       not r12d
14099ce51: f3410fbcc4                   tzcnt eax, r12d
14099ce56: c1e005                       shl eax, 5
14099ce59: 4c89f9                       mov rcx, r15
14099ce5c: 4829c1                       sub rcx, rax
14099ce5f: 488b41e8                     mov rax, qword ptr [rcx - 0x18]
14099ce63: 4885c0                       test rax, rax
14099ce66: 74a8                         je 0x14099ce10
14099ce68: 488b49f0                     mov rcx, qword ptr [rcx - 0x10]
14099ce6c: 48c1e005                     shl rax, 5
14099ce70: 488d1440                     lea rdx, [rax + rax*2]
14099ce74: 41b810000000                 mov r8d, 0x10
14099ce7a: e8a16dc7ff                   call 0x140613c20
14099ce7f: eb8f                         jmp 0x14099ce10
14099ce81: 4c8ba580020000               mov r12, qword ptr [rbp + 0x280]
14099ce88: 498b7c2408                   mov rdi, qword ptr [r12 + 8]
14099ce8d: 4885ff                       test rdi, rdi
14099ce90: 740e                         je 0x14099cea0
14099ce92: 4c8d4711                     lea r8, [rdi + 0x11]
14099ce96: 4889d9                       mov rcx, rbx
14099ce99: b2ff                         mov dl, 0xff
14099ce9b: e8806df801                   call 0x142923c20
14099cea0: 49c744241800000000           mov qword ptr [r12 + 0x18], 0
14099cea9: 488d4701                     lea rax, [rdi + 1]
14099cead: 4889c1                       mov rcx, rax
14099ceb0: 48c1e903                     shr rcx, 3
14099ceb4: 4883e0f8                     and rax, 0xfffffffffffffff8
14099ceb8: 4829c8                       sub rax, rcx
14099cebb: 4883ff08                     cmp rdi, 8
14099cebf: 480f42c7                     cmovb rax, rdi
14099cec3: 4989442410                   mov qword ptr [r12 + 0x10], rax
14099cec8: 488b8da8020000               mov rcx, qword ptr [rbp + 0x2a8]
14099cecf: 4c8ba9f8000000               mov r13, qword ptr [rcx + 0xf8]
14099ced6: 488b8100010000               mov rax, qword ptr [rcx + 0x100]
14099cedd: 4c8d3c8500000000             lea r15, [rax*4]
14099cee5: 4d01ef                       add r15, r13
14099cee8: 440fb69108010000             movzx r10d, byte ptr [rcx + 0x108]
14099cef0: 4d8d5c2420                   lea r11, [r12 + 0x20]
14099cef5: 41b808000000                 mov r8d, 8
14099cefb: 660f76f6                     pcmpeqd xmm6, xmm6
14099ceff: 4531c9                       xor r9d, r9d
14099cf02: 31c9                         xor ecx, ecx
14099cf04: 31d2                         xor edx, edx
14099cf06: 31ff                         xor edi, edi
14099cf08: 4531f6                       xor r14d, r14d
14099cf0b: 4584d2                       test r10b, r10b
14099cf0e: 753e                         jne 0x14099cf4e
14099cf10: e97b050000                   jmp 0x14099d490
14099cf15: 31d2                         xor edx, edx
14099cf17: 4531f6                       xor r14d, r14d
14099cf1a: 4c89e1                       mov rcx, r12
14099cf1d: 4489f0                       mov eax, r14d
14099cf20: 488d1c40                     lea rbx, [rax + rax*2]
14099cf24: 4d8d24d9                     lea r12, [r9 + rbx*8]
14099cf28: 4889d8                       mov rax, rbx
14099cf2b: 48c1e005                     shl rax, 5
14099cf2f: 4801c8                       add rax, rcx
14099cf32: 48c1e304                     shl rbx, 4
14099cf36: 4801d3                       add rbx, rdx
14099cf39: 49ffc6                       inc r14
14099cf3c: 833802                       cmp dword ptr [rax], 2
14099cf3f: 0f86c2000000                 jbe 0x14099d007
14099cf45: 4584d2                       test r10b, r10b
14099cf48: 0f8442050000                 je 0x14099d490
14099cf4e: 4939fe                       cmp r14, rdi
14099cf51: 75ca                         jne 0x14099cf1d
14099cf53: 666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
14099cf60: 4d39fd                       cmp r13, r15
14099cf63: 0f84dd060000                 je 0x14099d646
14099cf69: 418b4d00                     mov ecx, dword ptr [r13]
14099cf6d: 4983c504                     add r13, 4
14099cf71: 488b86a8010000               mov rax, qword ptr [rsi + 0x1a8]
14099cf78: 488d0cc9                     lea rcx, [rcx + rcx*8]
14099cf7c: 488b7cc810                   mov rdi, qword ptr [rax + rcx*8 + 0x10]
14099cf81: 4885ff                       test rdi, rdi
14099cf84: 74da                         je 0x14099cf60
14099cf86: 488d04c8                     lea rax, [rax + rcx*8]
14099cf8a: 488b8da8020000               mov rcx, qword ptr [rbp + 0x2a8]
14099cf91: 488b9110010000               mov rdx, qword ptr [rcx + 0x110]
14099cf98: 488b4840                     mov rcx, qword ptr [rax + 0x40]
14099cf9c: 4839d1                       cmp rcx, rdx
14099cf9f: 762d                         jbe 0x14099cfce
14099cfa1: 4c89c3                       mov rbx, r8
14099cfa4: 4c8b4038                     mov r8, qword ptr [rax + 0x38]
14099cfa8: 498b14d0                     mov rdx, qword ptr [r8 + rdx*8]
14099cfac: 4885d2                       test rdx, rdx
14099cfaf: 0f84cb010000                 je 0x14099d180
14099cfb5: 4c8b4018                     mov r8, qword ptr [rax + 0x18]
14099cfb9: 48f7d2                       not rdx
14099cfbc: 488d1452                     lea rdx, [rdx + rdx*2]
14099cfc0: 48c1e204                     shl rdx, 4
14099cfc4: 4d8b4c1010                   mov r9, qword ptr [r8 + rdx + 0x10]
14099cfc9: e9b5010000                   jmp 0x14099d183
14099cfce: 4531c9                       xor r9d, r9d
14099cfd1: 488b95a8020000               mov rdx, qword ptr [rbp + 0x2a8]
14099cfd8: 488b9218010000               mov rdx, qword ptr [rdx + 0x118]
14099cfdf: 4839d1                       cmp rcx, rdx
14099cfe2: 0f87b5010000                 ja 0x14099d19d
14099cfe8: 4531e4                       xor r12d, r12d
14099cfeb: 488b95a8020000               mov rdx, qword ptr [rbp + 0x2a8]
14099cff2: 488b9220010000               mov rdx, qword ptr [rdx + 0x120]
14099cff9: 4839d1                       cmp rcx, rdx
14099cffc: 0f8613ffffff                 jbe 0x14099cf15
14099d002: e9d9010000                   jmp 0x14099d1e0
14099d007: 488995b8000000               mov qword ptr [rbp + 0xb8], rdx
14099d00e: 48898db0000000               mov qword ptr [rbp + 0xb0], rcx
14099d015: 4c8985a8000000               mov qword ptr [rbp + 0xa8], r8
14099d01c: 4c898da0000000               mov qword ptr [rbp + 0xa0], r9
14099d023: 4c899d68020000               mov qword ptr [rbp + 0x268], r11
14099d02a: 448895a7020000               mov byte ptr [rbp + 0x2a7], r10b
14099d031: 8b4810                       mov ecx, dword ptr [rax + 0x10]
14099d034: 898d78020000                 mov dword ptr [rbp + 0x278], ecx
14099d03a: 488b4830                     mov rcx, qword ptr [rax + 0x30]
14099d03e: 488b5038                     mov rdx, qword ptr [rax + 0x38]
14099d042: e889b3f1ff                   call 0x1408b83d0
14099d047: 488d8da0010000               lea rcx, [rbp + 0x1a0]
14099d04e: 89c2                         mov edx, eax
14099d050: 4c8b8580000000               mov r8, qword ptr [rbp + 0x80]
14099d057: e834bef1ff                   call 0x1408b8e90
14099d05c: 488b85a0010000               mov rax, qword ptr [rbp + 0x1a0]
14099d063: 0f1000                       movups xmm0, xmmword ptr [rax]
14099d066: 0f104810                     movups xmm1, xmmword ptr [rax + 0x10]
14099d06a: 0f298d00010000               movaps xmmword ptr [rbp + 0x100], xmm1
14099d071: 0f2985f0000000               movaps xmmword ptr [rbp + 0xf0], xmm0
14099d078: 0f2803                       movaps xmm0, xmmword ptr [rbx]
14099d07b: 0f284b10                     movaps xmm1, xmmword ptr [rbx + 0x10]
14099d07f: 0f285320                     movaps xmm2, xmmword ptr [rbx + 0x20]
14099d083: 488d8510010000               lea rax, [rbp + 0x110]
14099d08a: 0f2900                       movaps xmmword ptr [rax], xmm0
14099d08d: 0f294810                     movaps xmmword ptr [rax + 0x10], xmm1
14099d091: 0f295020                     movaps xmmword ptr [rax + 0x20], xmm2
14099d095: f3410f107c240c               movss xmm7, dword ptr [r12 + 0xc]
14099d09c: 8b8578020000                 mov eax, dword ptr [rbp + 0x278]
14099d0a2: 8985a0010000                 mov dword ptr [rbp + 0x1a0], eax
14099d0a8: 488b8d68020000               mov rcx, qword ptr [rbp + 0x268]
14099d0af: 488d95a0010000               lea rdx, [rbp + 0x1a0]
14099d0b6: e8153ec1ff                   call 0x1405b0ed0
14099d0bb: 488b8d80020000               mov rcx, qword ptr [rbp + 0x280]
14099d0c2: 4c8b11                       mov r10, qword ptr [rcx]
14099d0c5: 4c8b4108                     mov r8, qword ptr [rcx + 8]
14099d0c9: 4d89c4                       mov r12, r8
14099d0cc: 4921c4                       and r12, rax
14099d0cf: 4889c2                       mov rdx, rax
14099d0d2: 48c1ea39                     shr rdx, 0x39
14099d0d6: 660f6ec2                     movd xmm0, edx
14099d0da: 660f60c0                     punpcklbw xmm0, xmm0
14099d0de: f20f70c000                   pshuflw xmm0, xmm0, 0
14099d0e3: 660f70c000                   pshufd xmm0, xmm0, 0
14099d0e8: 498d52e0                     lea rdx, [r10 - 0x20]
14099d0ec: 48895568                     mov qword ptr [rbp + 0x68], rdx
14099d0f0: 31d2                         xor edx, edx
14099d0f2: 48898598000000               mov qword ptr [rbp + 0x98], rax
14099d0f9: f3430f6f0c22                 movdqu xmm1, xmmword ptr [r10 + r12]
14099d0ff: 660f6fd1                     movdqa xmm2, xmm1
14099d103: 660f74d0                     pcmpeqb xmm2, xmm0
14099d107: 66440fd7ca                   pmovmskb r9d, xmm2
14099d10c: 4585c9                       test r9d, r9d
14099d10f: 8b8578020000                 mov eax, dword ptr [rbp + 0x278]
14099d115: 7444                         je 0x14099d15b
14099d117: 4c8995e8000000               mov qword ptr [rbp + 0xe8], r10
14099d11e: f3410fbcd9                   tzcnt ebx, r9d
14099d123: 4c01e3                       add rbx, r12
14099d126: 4c21c3                       and rbx, r8
14099d129: 4989da                       mov r10, rbx
14099d12c: 49c1e205                     shl r10, 5
14099d130: 4c8b5d68                     mov r11, qword ptr [rbp + 0x68]
14099d134: 4d29d3                       sub r11, r10
14099d137: 413903                       cmp dword ptr [r11], eax
14099d13a: 0f84ca000000                 je 0x14099d20a
14099d140: 458d51ff                     lea r10d, [r9 - 1]
14099d144: 664521ca                     and r10w, r9w
14099d148: 4589d1                       mov r9d, r10d
14099d14b: 488b8d80020000               mov rcx, qword ptr [rbp + 0x280]
14099d152: 4c8b95e8000000               mov r10, qword ptr [rbp + 0xe8]
14099d159: 75c3                         jne 0x14099d11e
14099d15b: 660f74ce                     pcmpeqb xmm1, xmm6
14099d15f: 66440fd7c9                   pmovmskb r9d, xmm1
14099d164: 4585c9                       test r9d, r9d
14099d167: 0f8592010000                 jne 0x14099d2ff
14099d16d: 4901d4                       add r12, rdx
14099d170: 4983c410                     add r12, 0x10
14099d174: 4883c210                     add rdx, 0x10
14099d178: 4d21c4                       and r12, r8
14099d17b: e979ffffff                   jmp 0x14099d0f9
14099d180: 4531c9                       xor r9d, r9d
14099d183: 4989d8                       mov r8, rbx
14099d186: 488b95a8020000               mov rdx, qword ptr [rbp + 0x2a8]
14099d18d: 488b9218010000               mov rdx, qword ptr [rdx + 0x118]
14099d194: 4839d1                       cmp rcx, rdx
14099d197: 0f864bfeffff                 jbe 0x14099cfe8
14099d19d: 4c89c3                       mov rbx, r8
14099d1a0: 4c8b4038                     mov r8, qword ptr [rax + 0x38]
14099d1a4: 498b14d0                     mov rdx, qword ptr [r8 + rdx*8]
14099d1a8: 4885d2                       test rdx, rdx
14099d1ab: 7416                         je 0x14099d1c3
14099d1ad: 4c8b4018                     mov r8, qword ptr [rax + 0x18]
14099d1b1: 48f7d2                       not rdx
14099d1b4: 488d1452                     lea rdx, [rdx + rdx*2]
14099d1b8: 48c1e204                     shl rdx, 4
14099d1bc: 4d8b641010                   mov r12, qword ptr [r8 + rdx + 0x10]
14099d1c1: eb03                         jmp 0x14099d1c6
14099d1c3: 4531e4                       xor r12d, r12d
14099d1c6: 4989d8                       mov r8, rbx
14099d1c9: 488b95a8020000               mov rdx, qword ptr [rbp + 0x2a8]
14099d1d0: 488b9220010000               mov rdx, qword ptr [rdx + 0x120]
14099d1d7: 4839d1                       cmp rcx, rdx
14099d1da: 0f8635fdffff                 jbe 0x14099cf15
14099d1e0: 488b4838                     mov rcx, qword ptr [rax + 0x38]
14099d1e4: 488b0cd1                     mov rcx, qword ptr [rcx + rdx*8]
14099d1e8: 4885c9                       test rcx, rcx
14099d1eb: 0f8424fdffff                 je 0x14099cf15
14099d1f1: 488b4018                     mov rax, qword ptr [rax + 0x18]
14099d1f5: 48f7d1                       not rcx
14099d1f8: 488d0c49                     lea rcx, [rcx + rcx*2]
14099d1fc: 48c1e104                     shl rcx, 4
14099d200: 488b540810                   mov rdx, qword ptr [rax + rcx + 0x10]
14099d205: e90dfdffff                   jmp 0x14099cf17
14099d20a: 48f7db                       neg rbx
14099d20d: 48c1e305                     shl rbx, 5
14099d211: 488d8510010000               lea rax, [rbp + 0x110]
14099d218: 0f2800                       movaps xmm0, xmmword ptr [rax]
14099d21b: 0f284810                     movaps xmm1, xmmword ptr [rax + 0x10]
14099d21f: 660f6f5020                   movdqa xmm2, xmmword ptr [rax + 0x20]
14099d224: 488d85c0010000               lea rax, [rbp + 0x1c0]
14099d22b: 660f7f5020                   movdqa xmmword ptr [rax + 0x20], xmm2
14099d230: 0f294810                     movaps xmmword ptr [rax + 0x10], xmm1
14099d234: 0f2900                       movaps xmmword ptr [rax], xmm0
14099d237: 660f6f85f0000000             movdqa xmm0, xmmword ptr [rbp + 0xf0]
14099d23f: 660f6f8d00010000             movdqa xmm1, xmmword ptr [rbp + 0x100]
14099d247: 660f7f8db0010000             movdqa xmmword ptr [rbp + 0x1b0], xmm1
14099d24f: 660f7f85a0010000             movdqa xmmword ptr [rbp + 0x1a0], xmm0
14099d257: 488b95e8000000               mov rdx, qword ptr [rbp + 0xe8]
14099d25e: 4c8b641af8                   mov r12, qword ptr [rdx + rbx - 8]
14099d263: 4c3b641ae8                   cmp r12, qword ptr [rdx + rbx - 0x18]
14099d268: 751b                         jne 0x14099d285
14099d26a: 488d0c1a                     lea rcx, [rdx + rbx]
14099d26e: 4883c1e8                     add rcx, -0x18
14099d272: 488d1537ea1102               lea rdx, [rip + 0x211ea37]
14099d279: e832281c00                   call 0x140b5fab0
14099d27e: 488b95e8000000               mov rdx, qword ptr [rbp + 0xe8]
14099d285: 4801da                       add rdx, rbx
14099d288: 488b42f0                     mov rax, qword ptr [rdx - 0x10]
14099d28c: 4b8d0c64                     lea rcx, [r12 + r12*2]
14099d290: 48c1e105                     shl rcx, 5
14099d294: 0f2885e0010000               movaps xmm0, xmmword ptr [rbp + 0x1e0]
14099d29b: 0f29440840                   movaps xmmword ptr [rax + rcx + 0x40], xmm0
14099d2a0: 660f6f85a0010000             movdqa xmm0, xmmword ptr [rbp + 0x1a0]
14099d2a8: 660f6f8db0010000             movdqa xmm1, xmmword ptr [rbp + 0x1b0]
14099d2b0: 660f6f95c0010000             movdqa xmm2, xmmword ptr [rbp + 0x1c0]
14099d2b8: 0f289dd0010000               movaps xmm3, xmmword ptr [rbp + 0x1d0]
14099d2bf: 0f295c0830                   movaps xmmword ptr [rax + rcx + 0x30], xmm3
14099d2c4: 660f7f540820                 movdqa xmmword ptr [rax + rcx + 0x20], xmm2
14099d2ca: 660f7f4c0810                 movdqa xmmword ptr [rax + rcx + 0x10], xmm1
14099d2d0: 660f7f0408                   movdqa xmmword ptr [rax + rcx], xmm0
14099d2d5: f30f117c0850                 movss dword ptr [rax + rcx + 0x50], xmm7
14099d2db: 49ffc4                       inc r12
14099d2de: 4c8962f8                     mov qword ptr [rdx - 8], r12
14099d2e2: b301                         mov bl, 1
14099d2e4: 4c8ba580020000               mov r12, qword ptr [rbp + 0x280]
14099d2eb: 48899598000000               mov qword ptr [rbp + 0x98], rdx
14099d2f2: 48c7857802000000000000       mov qword ptr [rbp + 0x278], 0
14099d2fd: eb2a                         jmp 0x14099d329
14099d2ff: 4883791000                   cmp qword ptr [rcx + 0x10], 0
14099d304: 0f8416030000                 je 0x14099d620
14099d30a: 89c0                         mov eax, eax
14099d30c: 48ba00000000ffffffff         movabs rdx, 0xffffffff00000000
14099d316: 4c8b6570                     mov r12, qword ptr [rbp + 0x70]
14099d31a: 4921d4                       and r12, rdx
14099d31d: 4909c4                       or r12, rax
14099d320: 31db                         xor ebx, ebx
14099d322: 48898d78020000               mov qword ptr [rbp + 0x278], rcx
14099d329: 0fb605d978c902               movzx eax, byte ptr [rip + 0x2c978d9]
14099d330: b960000000                   mov ecx, 0x60
14099d335: ba10000000                   mov edx, 0x10
14099d33a: e8d168c7ff                   call 0x140613c10
14099d33f: 4885c0                       test rax, rax
14099d342: 0f846f140000                 je 0x14099e7b7
14099d348: 0f288530010000               movaps xmm0, xmmword ptr [rbp + 0x130]
14099d34f: 0f294040                     movaps xmmword ptr [rax + 0x40], xmm0
14099d353: 660f6f85f0000000             movdqa xmm0, xmmword ptr [rbp + 0xf0]
14099d35b: 660f6f8d00010000             movdqa xmm1, xmmword ptr [rbp + 0x100]
14099d363: 660f6f9510010000             movdqa xmm2, xmmword ptr [rbp + 0x110]
14099d36b: 0f289d20010000               movaps xmm3, xmmword ptr [rbp + 0x120]
14099d372: 0f295830                     movaps xmmword ptr [rax + 0x30], xmm3
14099d376: 660f7f5020                   movdqa xmmword ptr [rax + 0x20], xmm2
14099d37b: 660f7f4810                   movdqa xmmword ptr [rax + 0x10], xmm1
14099d380: 660f7f00                     movdqa xmmword ptr [rax], xmm0
14099d384: f30f117850                   movss dword ptr [rax + 0x50], xmm7
14099d389: 84db                         test bl, bl
14099d38b: 4c896570                     mov qword ptr [rbp + 0x70], r12
14099d38f: 7427                         je 0x14099d3b8
14099d391: ba60000000                   mov edx, 0x60
14099d396: 41b810000000                 mov r8d, 0x10
14099d39c: 4889c1                       mov rcx, rax
14099d39f: e87c68c7ff                   call 0x140613c20
14099d3a4: 440fb695a7020000             movzx r10d, byte ptr [rbp + 0x2a7]
14099d3ac: 4c8b9d68020000               mov r11, qword ptr [rbp + 0x268]
14099d3b3: e9a8000000                   jmp 0x14099d460
14099d3b8: 488b9578020000               mov rdx, qword ptr [rbp + 0x278]
14099d3bf: 488b0a                       mov rcx, qword ptr [rdx]
14099d3c2: 488b5208                     mov rdx, qword ptr [rdx + 8]
14099d3c6: 4989d0                       mov r8, rdx
14099d3c9: 488b9d98000000               mov rbx, qword ptr [rbp + 0x98]
14099d3d0: 4921d8                       and r8, rbx
14099d3d3: f3420f6f0401                 movdqu xmm0, xmmword ptr [rcx + r8]
14099d3d9: 66440fd7c8                   pmovmskb r9d, xmm0
14099d3de: 4585c9                       test r9d, r9d
14099d3e1: 0f84f4010000                 je 0x14099d5db
14099d3e7: 4c8b9d68020000               mov r11, qword ptr [rbp + 0x268]
14099d3ee: f3450fbcc9                   tzcnt r9d, r9d
14099d3f3: 4d01c1                       add r9, r8
14099d3f6: 4921d1                       and r9, rdx
14099d3f9: 460fb60409                   movzx r8d, byte ptr [rcx + r9]
14099d3fe: 4584c0                       test r8b, r8b
14099d401: 0f8901020000                 jns 0x14099d608
14099d407: 48c1eb39                     shr rbx, 0x39
14099d40b: 4d8d51f0                     lea r10, [r9 - 0x10]
14099d40f: 4921d2                       and r10, rdx
14099d412: 42881c09                     mov byte ptr [rcx + r9], bl
14099d416: 42885c1110                   mov byte ptr [rcx + r10 + 0x10], bl
14099d41b: 49c1e105                     shl r9, 5
14099d41f: 4889ca                       mov rdx, rcx
14099d422: 4c29ca                       sub rdx, r9
14099d425: 4180e001                     and r8b, 1
14099d429: 450fb6c0                     movzx r8d, r8b
14099d42d: 4c8b9578020000               mov r10, qword ptr [rbp + 0x278]
14099d434: 4d294210                     sub qword ptr [r10 + 0x10], r8
14099d438: 49f7d9                       neg r9
14099d43b: 46896409e0                   mov dword ptr [rcx + r9 - 0x20], r12d
14099d440: 48c742e801000000             mov qword ptr [rdx - 0x18], 1
14099d448: 488942f0                     mov qword ptr [rdx - 0x10], rax
14099d44c: 48c742f801000000             mov qword ptr [rdx - 8], 1
14099d454: 49ff4218                     inc qword ptr [r10 + 0x18]
14099d458: 440fb695a7020000             movzx r10d, byte ptr [rbp + 0x2a7]
14099d460: 4c8b8da0000000               mov r9, qword ptr [rbp + 0xa0]
14099d467: 4c8b85a8000000               mov r8, qword ptr [rbp + 0xa8]
14099d46e: 488b8db0000000               mov rcx, qword ptr [rbp + 0xb0]
14099d475: 488b95b8000000               mov rdx, qword ptr [rbp + 0xb8]
14099d47c: 4584d2                       test r10b, r10b
14099d47f: 0f85c9faffff                 jne 0x14099cf4e
14099d485: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
14099d490: 4939fe                       cmp r14, rdi
14099d493: 0f8531010000                 jne 0x14099d5ca
14099d499: 0f1f8000000000               nop dword ptr [rax]
14099d4a0: 4d39fd                       cmp r13, r15
14099d4a3: 0f849d010000                 je 0x14099d646
14099d4a9: 418b4d00                     mov ecx, dword ptr [r13]
14099d4ad: 4983c504                     add r13, 4
14099d4b1: 488b8600010000               mov rax, qword ptr [rsi + 0x100]
14099d4b8: 488d0c89                     lea rcx, [rcx + rcx*4]
14099d4bc: 48c1e105                     shl rcx, 5
14099d4c0: 488b7c0858                   mov rdi, qword ptr [rax + rcx + 0x58]
14099d4c5: 4885ff                       test rdi, rdi
14099d4c8: 74d6                         je 0x14099d4a0
14099d4ca: 4801c8                       add rax, rcx
14099d4cd: 8b8894000000                 mov ecx, dword ptr [rax + 0x94]
14099d4d3: 488b96a8010000               mov rdx, qword ptr [rsi + 0x1a8]
14099d4da: 4c8d0cc9                     lea r9, [rcx + rcx*8]
14099d4de: 4a8d0cca                     lea rcx, [rdx + r9*8]
14099d4e2: 4c8b85a8020000               mov r8, qword ptr [rbp + 0x2a8]
14099d4e9: 4d8b8010010000               mov r8, qword ptr [r8 + 0x110]
14099d4f0: 4a8b54ca40                   mov rdx, qword ptr [rdx + r9*8 + 0x40]
14099d4f5: 4c39c2                       cmp rdx, r8
14099d4f8: 764c                         jbe 0x14099d546
14099d4fa: 4c8b4938                     mov r9, qword ptr [rcx + 0x38]
14099d4fe: 4f8b04c1                     mov r8, qword ptr [r9 + r8*8]
14099d502: 4d85c0                       test r8, r8
14099d505: 743f                         je 0x14099d546
14099d507: 4c8b4918                     mov r9, qword ptr [rcx + 0x18]
14099d50b: 49f7d0                       not r8
14099d50e: 4f8d0440                     lea r8, [r8 + r8*2]
14099d512: 49c1e004                     shl r8, 4
14099d516: 4f8b4c0110                   mov r9, qword ptr [r9 + r8 + 0x10]
14099d51b: 4c8b85a8020000               mov r8, qword ptr [rbp + 0x2a8]
14099d522: 4d8b8018010000               mov r8, qword ptr [r8 + 0x118]
14099d529: 4c39c2                       cmp rdx, r8
14099d52c: 772e                         ja 0x14099d55c
14099d52e: 4531e4                       xor r12d, r12d
14099d531: 4c8b85a8020000               mov r8, qword ptr [rbp + 0x2a8]
14099d538: 4d8b8020010000               mov r8, qword ptr [r8 + 0x120]
14099d53f: 4c39c2                       cmp rdx, r8
14099d542: 7757                         ja 0x14099d59b
14099d544: eb78                         jmp 0x14099d5be
14099d546: 4531c9                       xor r9d, r9d
14099d549: 4c8b85a8020000               mov r8, qword ptr [rbp + 0x2a8]
14099d550: 4d8b8018010000               mov r8, qword ptr [r8 + 0x118]
14099d557: 4c39c2                       cmp rdx, r8
14099d55a: 76d2                         jbe 0x14099d52e
14099d55c: 4c89cb                       mov rbx, r9
14099d55f: 4c8b4938                     mov r9, qword ptr [rcx + 0x38]
14099d563: 4f8b04c1                     mov r8, qword ptr [r9 + r8*8]
14099d567: 4d85c0                       test r8, r8
14099d56a: 7416                         je 0x14099d582
14099d56c: 4c8b4918                     mov r9, qword ptr [rcx + 0x18]
14099d570: 49f7d0                       not r8
14099d573: 4f8d0440                     lea r8, [r8 + r8*2]
14099d577: 49c1e004                     shl r8, 4
14099d57b: 4f8b640110                   mov r12, qword ptr [r9 + r8 + 0x10]
14099d580: eb03                         jmp 0x14099d585
14099d582: 4531e4                       xor r12d, r12d
14099d585: 4989d9                       mov r9, rbx
14099d588: 4c8b85a8020000               mov r8, qword ptr [rbp + 0x2a8]
14099d58f: 4d8b8020010000               mov r8, qword ptr [r8 + 0x120]
14099d596: 4c39c2                       cmp rdx, r8
14099d599: 7623                         jbe 0x14099d5be
14099d59b: 488b5138                     mov rdx, qword ptr [rcx + 0x38]
14099d59f: 4a8b14c2                     mov rdx, qword ptr [rdx + r8*8]
14099d5a3: 4885d2                       test rdx, rdx
14099d5a6: 7416                         je 0x14099d5be
14099d5a8: 488b4918                     mov rcx, qword ptr [rcx + 0x18]
14099d5ac: 48f7d2                       not rdx
14099d5af: 488d1452                     lea rdx, [rdx + rdx*2]
14099d5b3: 48c1e204                     shl rdx, 4
14099d5b7: 488b541110                   mov rdx, qword ptr [rcx + rdx + 0x10]
14099d5bc: eb02                         jmp 0x14099d5c0
14099d5be: 31d2                         xor edx, edx
14099d5c0: 4c8b4050                     mov r8, qword ptr [rax + 0x50]
14099d5c4: 4531f6                       xor r14d, r14d
14099d5c7: 4c89e1                       mov rcx, r12
14099d5ca: 4c89f0                       mov rax, r14
14099d5cd: 48c1e004                     shl rax, 4
14099d5d1: 418b440008                   mov eax, dword ptr [r8 + rax + 8]
14099d5d6: e945f9ffff                   jmp 0x14099cf20
14099d5db: 41ba10000000                 mov r10d, 0x10
14099d5e1: 4c8b9d68020000               mov r11, qword ptr [rbp + 0x268]
14099d5e8: 4d01d0                       add r8, r10
14099d5eb: 4921d0                       and r8, rdx
14099d5ee: f3420f6f0401                 movdqu xmm0, xmmword ptr [rcx + r8]
14099d5f4: 66440fd7c8                   pmovmskb r9d, xmm0
14099d5f9: 4983c210                     add r10, 0x10
14099d5fd: 4585c9                       test r9d, r9d
14099d600: 0f85e8fdffff                 jne 0x14099d3ee
14099d606: ebe0                         jmp 0x14099d5e8
14099d608: 660f6f01                     movdqa xmm0, xmmword ptr [rcx]
14099d60c: 66440fd7c0                   pmovmskb r8d, xmm0
14099d611: f3450fbcc8                   tzcnt r9d, r8d
14099d616: 460fb60409                   movzx r8d, byte ptr [rcx + r9]
14099d61b: e9e7fdffff                   jmp 0x14099d407
14099d620: ba01000000                   mov edx, 1
14099d625: 4c8b8568020000               mov r8, qword ptr [rbp + 0x268]
14099d62c: 41b101                       mov r9b, 1
14099d62f: e80c5bf201                   call 0x1428c3140
14099d634: 8b85a0010000                 mov eax, dword ptr [rbp + 0x1a0]
14099d63a: 488b8d80020000               mov rcx, qword ptr [rbp + 0x280]
14099d641: e9c4fcffff                   jmp 0x14099d30a
14099d646: 80bd9802000000               cmp byte ptr [rbp + 0x298], 0
14099d64d: 488bbd90020000               mov rdi, qword ptr [rbp + 0x290]
14099d654: 744b                         je 0x14099d6a1
14099d656: e8d5aa2700                   call 0x140c18130
14099d65b: 4889c1                       mov rcx, rax
14099d65e: 488b00                       mov rax, qword ptr [rax]
14099d661: 4883f801                     cmp rax, 1
14099d665: 0f85fe030000                 jne 0x14099da69
14099d66b: 4883c108                     add rcx, 8
14099d66f: 48833900                     cmp qword ptr [rcx], 0
14099d673: 0f8505070000                 jne 0x14099dd7e
14099d679: 48c701ffffffff               mov qword ptr [rcx], 0xffffffffffffffff
14099d680: 48898d98020000               mov qword ptr [rbp + 0x298], rcx
14099d687: 4883c108                     add rcx, 8
14099d68b: 488b9550010000               mov rdx, qword ptr [rbp + 0x150]
14099d692: e8599a2700                   call 0x140c170f0
14099d697: 488b8598020000               mov rax, qword ptr [rbp + 0x298]
14099d69e: 48ff00                       inc qword ptr [rax]
14099d6a1: 4889b598020000               mov qword ptr [rbp + 0x298], rsi
14099d6a8: 488b07                       mov rax, qword ptr [rdi]
14099d6ab: 4531d2                       xor r10d, r10d
14099d6ae: 4889c1                       mov rcx, rax
14099d6b1: 4c8bbd88010000               mov r15, qword ptr [rbp + 0x188]
14099d6b8: 492b4f18                     sub rcx, qword ptr [r15 + 0x18]
14099d6bc: 490f42ca                     cmovb rcx, r10
14099d6c0: 492b4738                     sub rax, qword ptr [r15 + 0x38]
14099d6c4: 490f42c2                     cmovb rax, r10
14099d6c8: 4d8b4f08                     mov r9, qword ptr [r15 + 8]
14099d6cc: 4d8b4730                     mov r8, qword ptr [r15 + 0x30]
14099d6d0: 4c89c2                       mov rdx, r8
14099d6d3: 4829c2                       sub rdx, rax
14099d6d6: 490f42d2                     cmovb rdx, r10
14099d6da: 498b5f10                     mov rbx, qword ptr [r15 + 0x10]
14099d6de: 410f96c6                     setbe r14b
14099d6e2: 4889fe                       mov rsi, rdi
14099d6e5: 4889df                       mov rdi, rbx
14099d6e8: 4829cf                       sub rdi, rcx
14099d6eb: 490f42fa                     cmovb rdi, r10
14099d6ef: 4d8b5f28                     mov r11, qword ptr [r15 + 0x28]
14099d6f3: 4d8b5740                     mov r10, qword ptr [r15 + 0x40]
14099d6f7: 410f96c7                     setbe r15b
14099d6fb: 4d89d5                       mov r13, r10
14099d6fe: 4929d5                       sub r13, rdx
14099d701: 4929fd                       sub r13, rdi
14099d704: 4c892e                       mov qword ptr [rsi], r13
14099d707: 4584f7                       test r15b, r14b
14099d70a: 0f85ba010000                 jne 0x14099d8ca
14099d710: 4989c6                       mov r14, rax
14099d713: 49c1e604                     shl r14, 4
14099d717: 4d01f3                       add r11, r14
14099d71a: 48c1e204                     shl rdx, 4
14099d71e: 4989ce                       mov r14, rcx
14099d721: 49c1e604                     shl r14, 4
14099d725: 4d01f1                       add r9, r14
14099d728: 48c1e704                     shl rdi, 4
14099d72c: 4531f6                       xor r14d, r14d
14099d72f: 4839cb                       cmp rbx, rcx
14099d732: b908000000                   mov ecx, 8
14099d737: 4c0f42c9                     cmovb r9, rcx
14099d73b: 4d8d4c39f0                   lea r9, [r9 + rdi - 0x10]
14099d740: 4d0f47f1                     cmova r14, r9
14099d744: 4939c0                       cmp r8, rax
14099d747: 4c0f42d9                     cmovb r11, rcx
14099d74b: 498d4413f0                   lea rax, [r11 + rdx - 0x10]
14099d750: 490f46c6                     cmovbe rax, r14
14099d754: 4c8916                       mov qword ptr [rsi], r10
14099d757: 488b4008                     mov rax, qword ptr [rax + 8]
14099d75b: 89c1                         mov ecx, eax
14099d75d: 488b9598020000               mov rdx, qword ptr [rbp + 0x298]
14099d764: 483b4a10                     cmp rcx, qword ptr [rdx + 0x10]
14099d768: 0f835c010000                 jae 0x14099d8ca
14099d76e: 488b9598020000               mov rdx, qword ptr [rbp + 0x298]
14099d775: 488b5208                     mov rdx, qword ptr [rdx + 8]
14099d779: 48c1e820                     shr rax, 0x20
14099d77d: 488d0c89                     lea rcx, [rcx + rcx*4]
14099d781: 39048a                       cmp dword ptr [rdx + rcx*4], eax
14099d784: 0f8540010000                 jne 0x14099d8ca
14099d78a: 488d048a                     lea rax, [rdx + rcx*4]
14099d78e: 8b4804                       mov ecx, dword ptr [rax + 4]
14099d791: baffffffff                   mov edx, 0xffffffff
14099d796: 4839d1                       cmp rcx, rdx
14099d799: 0f842b010000                 je 0x14099d8ca
14099d79f: 488b95a8020000               mov rdx, qword ptr [rbp + 0x2a8]
14099d7a6: 48394a38                     cmp qword ptr [rdx + 0x38], rcx
14099d7aa: 0f861a010000                 jbe 0x14099d8ca
14099d7b0: 488b95a8020000               mov rdx, qword ptr [rbp + 0x2a8]
14099d7b7: 488b5228                     mov rdx, qword ptr [rdx + 0x28]
14099d7bb: 4189c8                       mov r8d, ecx
14099d7be: 41c1e806                     shr r8d, 6
14099d7c2: 4a8b14c2                     mov rdx, qword ptr [rdx + r8*8]
14099d7c6: 480fa3ca                     bt rdx, rcx
14099d7ca: 0f83fa000000                 jae 0x14099d8ca
14099d7d0: 8b480c                       mov ecx, dword ptr [rax + 0xc]
14099d7d3: 8b5010                       mov edx, dword ptr [rax + 0x10]
14099d7d6: 488b8598020000               mov rax, qword ptr [rbp + 0x298]
14099d7dd: 4c8b80a8010000               mov r8, qword ptr [rax + 0x1a8]
14099d7e4: 488d0cc9                     lea rcx, [rcx + rcx*8]
14099d7e8: 498b44c818                   mov rax, qword ptr [r8 + rcx*8 + 0x18]
14099d7ed: 498b4cc838                   mov rcx, qword ptr [r8 + rcx*8 + 0x38]
14099d7f2: 4c8b95a8020000               mov r10, qword ptr [rbp + 0x2a8]
14099d7f9: 4d8b8210010000               mov r8, qword ptr [r10 + 0x110]
14099d800: 4d8b8a18010000               mov r9, qword ptr [r10 + 0x118]
14099d807: 4e8b04c1                     mov r8, qword ptr [rcx + r8*8]
14099d80b: 49f7d0                       not r8
14099d80e: 4f8d0440                     lea r8, [r8 + r8*2]
14099d812: 49c1e004                     shl r8, 4
14099d816: 4e8b0cc9                     mov r9, qword ptr [rcx + r9*8]
14099d81a: 49f7d1                       not r9
14099d81d: 4f8d0c49                     lea r9, [r9 + r9*2]
14099d821: 49c1e104                     shl r9, 4
14099d825: 4d8b9220010000               mov r10, qword ptr [r10 + 0x120]
14099d82c: 4a8b0cd1                     mov rcx, qword ptr [rcx + r10*8]
14099d830: 48f7d1                       not rcx
14099d833: 4c8d1449                     lea r10, [rcx + rcx*2]
14099d837: 49c1e204                     shl r10, 4
14099d83b: 488d1452                     lea rdx, [rdx + rdx*2]
14099d83f: 4c8d1cd500000000             lea r11, [rdx*8]
14099d847: 4e035c0010                   add r11, qword ptr [rax + r8 + 0x10]
14099d84c: 4889d1                       mov rcx, rdx
14099d84f: 48c1e104                     shl rcx, 4
14099d853: 4a034c1010                   add rcx, qword ptr [rax + r10 + 0x10]
14099d858: 48c1e205                     shl rdx, 5
14099d85c: 4a03540810                   add rdx, qword ptr [rax + r9 + 0x10]
14099d861: 488b8590000000               mov rax, qword ptr [rbp + 0x90]
14099d868: 448b8564020000               mov r8d, dword ptr [rbp + 0x264]
14099d86f: 448900                       mov dword ptr [rax], r8d
14099d872: 488b85d0000000               mov rax, qword ptr [rbp + 0xd0]
14099d879: 448b85d8000000               mov r8d, dword ptr [rbp + 0xd8]
14099d880: 448900                       mov dword ptr [rax], r8d
14099d883: 488b8588020000               mov rax, qword ptr [rbp + 0x288]
14099d88a: 4889442440                   mov qword ptr [rsp + 0x40], rax
14099d88f: 488b85c8000000               mov rax, qword ptr [rbp + 0xc8]
14099d896: 4889442438                   mov qword ptr [rsp + 0x38], rax
14099d89b: 488d4510                     lea rax, [rbp + 0x10]
14099d89f: 4889442430                   mov qword ptr [rsp + 0x30], rax
14099d8a4: 4c895c2428                   mov qword ptr [rsp + 0x28], r11
14099d8a9: 488b8588000000               mov rax, qword ptr [rbp + 0x88]
14099d8b0: 4889442420                   mov qword ptr [rsp + 0x20], rax
14099d8b5: 4c8d4d30                     lea r9, [rbp + 0x30]
14099d8b9: 4c8b85c0000000               mov r8, qword ptr [rbp + 0xc0]
14099d8c0: e8db93fdff                   call 0x140976ca0
14099d8c5: e9700e0000                   jmp 0x14099e73a
14099d8ca: 488b8588020000               mov rax, qword ptr [rbp + 0x288]
14099d8d1: 4c8b20                       mov r12, qword ptr [rax]
14099d8d4: 418b0424                     mov eax, dword ptr [r12]
14099d8d8: 8d50fa                       lea edx, [rax - 6]
14099d8db: 4989c0                       mov r8, rax
14099d8de: 4983c0fb                     add r8, -5
14099d8e2: 31c9                         xor ecx, ecx
14099d8e4: 83fa0f                       cmp edx, 0xf
14099d8e7: 490f42c8                     cmovb rcx, r8
14099d8eb: 4885c9                       test rcx, rcx
14099d8ee: 741e                         je 0x14099d90e
14099d8f0: 4883f904                     cmp rcx, 4
14099d8f4: 488bb598020000               mov rsi, qword ptr [rbp + 0x298]
14099d8fb: 0f85390e0000                 jne 0x14099e73a
14099d901: 41807c240403                 cmp byte ptr [r12 + 4], 3
14099d907: 7414                         je 0x14099d91d
14099d909: e92c0e0000                   jmp 0x14099e73a
14099d90e: 85c0                         test eax, eax
14099d910: 488bb598020000               mov rsi, qword ptr [rbp + 0x298]
14099d917: 0f851d0e0000                 jne 0x14099e73a
14099d91d: 488b8590010000               mov rax, qword ptr [rbp + 0x190]
14099d924: 4c8b28                       mov r13, qword ptr [rax]
14099d927: 488b8560030000               mov rax, qword ptr [rbp + 0x360]
14099d92e: 488b18                       mov rbx, qword ptr [rax]
14099d931: c7442430f3080000             mov dword ptr [rsp + 0x30], 0x8f3
14099d939: c7442428ffff7f7f             mov dword ptr [rsp + 0x28], 0x7f7fffff
14099d941: 488d8da0010000               lea rcx, [rbp + 0x1a0]
14099d948: 4c89ea                       mov rdx, r13
14099d94b: 4989d8                       mov r8, rbx
14099d94e: 4531c9                       xor r9d, r9d
14099d951: e80a03f3ff                   call 0x1408cdc60
14099d956: 83bdd401000002               cmp dword ptr [rbp + 0x1d4], 2
14099d95d: 0f84d70d0000                 je 0x14099e73a
14099d963: 80bda00100000c               cmp byte ptr [rbp + 0x1a0], 0xc
14099d96a: 0f85ca0d0000                 jne 0x14099e73a
14099d970: f3440f1085b8010000           movss xmm8, dword ptr [rbp + 0x1b8]
14099d979: f30f10bdbc010000             movss xmm7, dword ptr [rbp + 0x1bc]
14099d981: f30f10b5c0010000             movss xmm6, dword ptr [rbp + 0x1c0]
14099d989: 488b85a8010000               mov rax, qword ptr [rbp + 0x1a8]
14099d990: 48894578                     mov qword ptr [rbp + 0x78], rax
14099d994: 89c1                         mov ecx, eax
14099d996: 483b4e10                     cmp rcx, qword ptr [rsi + 0x10]
14099d99a: 0f839a0d0000                 jae 0x14099e73a
14099d9a0: 488b5608                     mov rdx, qword ptr [rsi + 8]
14099d9a4: 48c1e820                     shr rax, 0x20
14099d9a8: 488d0c89                     lea rcx, [rcx + rcx*4]
14099d9ac: 39048a                       cmp dword ptr [rdx + rcx*4], eax
14099d9af: 0f85850d0000                 jne 0x14099e73a
14099d9b5: 488d048a                     lea rax, [rdx + rcx*4]
14099d9b9: 8b4804                       mov ecx, dword ptr [rax + 4]
14099d9bc: baffffffff                   mov edx, 0xffffffff
14099d9c1: 4839d1                       cmp rcx, rdx
14099d9c4: 0f84700d0000                 je 0x14099e73a
14099d9ca: 488b95a8020000               mov rdx, qword ptr [rbp + 0x2a8]
14099d9d1: 48394a38                     cmp qword ptr [rdx + 0x38], rcx
14099d9d5: 0f865f0d0000                 jbe 0x14099e73a
14099d9db: 488b95a8020000               mov rdx, qword ptr [rbp + 0x2a8]
14099d9e2: 488b5228                     mov rdx, qword ptr [rdx + 0x28]
14099d9e6: 4189c8                       mov r8d, ecx
14099d9e9: 41c1e806                     shr r8d, 6
14099d9ed: 4a8b14c2                     mov rdx, qword ptr [rdx + r8*8]
14099d9f1: 480fa3ca                     bt rdx, rcx
14099d9f5: 0f833f0d0000                 jae 0x14099e73a
14099d9fb: 8b480c                       mov ecx, dword ptr [rax + 0xc]
14099d9fe: 488b96a8010000               mov rdx, qword ptr [rsi + 0x1a8]
14099da05: 4c8d0cc9                     lea r9, [rcx + rcx*8]
14099da09: 4a8d0cca                     lea rcx, [rdx + r9*8]
14099da0d: 4c8b85a8020000               mov r8, qword ptr [rbp + 0x2a8]
14099da14: 4d8b8010010000               mov r8, qword ptr [r8 + 0x110]
14099da1b: 4a8b54ca40                   mov rdx, qword ptr [rdx + r9*8 + 0x40]
14099da20: 4c39c2                       cmp rdx, r8
14099da23: 7668                         jbe 0x14099da8d
14099da25: 4c8b4938                     mov r9, qword ptr [rcx + 0x38]
14099da29: 4f8b04c1                     mov r8, qword ptr [r9 + r8*8]
14099da2d: 4d85c0                       test r8, r8
14099da30: 745b                         je 0x14099da8d
14099da32: 4c8b4918                     mov r9, qword ptr [rcx + 0x18]
14099da36: 49f7d0                       not r8
14099da39: 4f8d0440                     lea r8, [r8 + r8*2]
14099da3d: 49c1e004                     shl r8, 4
14099da41: 4f8b440110                   mov r8, qword ptr [r9 + r8 + 0x10]
14099da46: 4c898598020000               mov qword ptr [rbp + 0x298], r8
14099da4d: eb49                         jmp 0x14099da98
14099da4f: 4885c0                       test rax, rax
14099da52: 752d                         jne 0x14099da81
14099da54: 31d2                         xor edx, edx
14099da56: e875adf501                   call 0x1428f87d0
14099da5b: 4889c1                       mov rcx, rax
14099da5e: 4885c0                       test rax, rax
14099da61: 0f8513f3ffff                 jne 0x14099cd7a
14099da67: eb18                         jmp 0x14099da81
14099da69: 4885c0                       test rax, rax
14099da6c: 7513                         jne 0x14099da81
14099da6e: 31d2                         xor edx, edx
14099da70: e85badf501                   call 0x1428f87d0
14099da75: 4889c1                       mov rcx, rax
14099da78: 4885c0                       test rax, rax
14099da7b: 0f85eefbffff                 jne 0x14099d66f
14099da81: 488d0d18d02902               lea rcx, [rip + 0x229d018]
14099da88: e843a6f301                   call 0x1428d80d0
14099da8d: 48c7859802000000000000       mov qword ptr [rbp + 0x298], 0
14099da98: 4c8b85a8020000               mov r8, qword ptr [rbp + 0x2a8]
14099da9f: 4d8b8018010000               mov r8, qword ptr [r8 + 0x118]
14099daa6: 4c39c2                       cmp rdx, r8
14099daa9: 7623                         jbe 0x14099dace
14099daab: 4c8b4938                     mov r9, qword ptr [rcx + 0x38]
14099daaf: 4f8b04c1                     mov r8, qword ptr [r9 + r8*8]
14099dab3: 4d85c0                       test r8, r8
14099dab6: 7416                         je 0x14099dace
14099dab8: 4c8b4918                     mov r9, qword ptr [rcx + 0x18]
14099dabc: 49f7d0                       not r8
14099dabf: 4f8d0440                     lea r8, [r8 + r8*2]
14099dac3: 49c1e004                     shl r8, 4
14099dac7: 4b8b7c0110                   mov rdi, qword ptr [r9 + r8 + 0x10]
14099dacc: eb02                         jmp 0x14099dad0
14099dace: 31ff                         xor edi, edi
14099dad0: 4c8b85a8020000               mov r8, qword ptr [rbp + 0x2a8]
14099dad7: 4d8b8020010000               mov r8, qword ptr [r8 + 0x120]
14099dade: 4c39c2                       cmp rdx, r8
14099dae1: 762a                         jbe 0x14099db0d
14099dae3: 488b5138                     mov rdx, qword ptr [rcx + 0x38]
14099dae7: 4a8b14c2                     mov rdx, qword ptr [rdx + r8*8]
14099daeb: 4885d2                       test rdx, rdx
14099daee: 741d                         je 0x14099db0d
14099daf0: 488b4918                     mov rcx, qword ptr [rcx + 0x18]
14099daf4: 48f7d2                       not rdx
14099daf7: 488d1452                     lea rdx, [rdx + rdx*2]
14099dafb: 48c1e204                     shl rdx, 4
14099daff: 488b4c1110                   mov rcx, qword ptr [rcx + rdx + 0x10]
14099db04: 48898da8020000               mov qword ptr [rbp + 0x2a8], rcx
14099db0b: eb0b                         jmp 0x14099db18
14099db0d: 48c785a802000000000000       mov qword ptr [rbp + 0x2a8], 0
14099db18: 8b7010                       mov esi, dword ptr [rax + 0x10]
14099db1b: 488b85e0000000               mov rax, qword ptr [rbp + 0xe0]
14099db22: 4c8b30                       mov r14, qword ptr [rax]
14099db25: 488d4578                     lea rax, [rbp + 0x78]
14099db29: 48898548010000               mov qword ptr [rbp + 0x148], rax
14099db30: 488d05b9d13400               lea rax, [rip + 0x34d1b9]
14099db37: 48898550010000               mov qword ptr [rbp + 0x150], rax
14099db3e: 488d05f3df1102               lea rax, [rip + 0x211dff3]
14099db45: 488985a0010000               mov qword ptr [rbp + 0x1a0], rax
14099db4c: 48c785a801000001000000       mov qword ptr [rbp + 0x1a8], 1
14099db57: 488d8548010000               lea rax, [rbp + 0x148]
14099db5e: 488985b0010000               mov qword ptr [rbp + 0x1b0], rax
14099db65: 48c785b801000001000000       mov qword ptr [rbp + 0x1b8], 1
14099db70: 48c785c001000000000000       mov qword ptr [rbp + 0x1c0], 0
14099db7b: 4c8dbdf0000000               lea r15, [rbp + 0xf0]
14099db82: 488d95a0010000               lea rdx, [rbp + 0x1a0]
14099db89: 4c89f9                       mov rcx, r15
14099db8c: e8ffc33d00                   call 0x140d79f90
14099db91: f30f1003                     movss xmm0, dword ptr [rbx]
14099db95: f3410f5cc0                   subss xmm0, xmm8
14099db9a: f30f104b04                   movss xmm1, dword ptr [rbx + 4]
14099db9f: f30f5ccf                     subss xmm1, xmm7
14099dba3: f30f105308                   movss xmm2, dword ptr [rbx + 8]
14099dba8: f30f5cd6                     subss xmm2, xmm6
14099dbac: f30f59c0                     mulss xmm0, xmm0
14099dbb0: f30f59c9                     mulss xmm1, xmm1
14099dbb4: f30f58c8                     addss xmm1, xmm0
14099dbb8: f30f59d2                     mulss xmm2, xmm2
14099dbbc: f30f58d1                     addss xmm2, xmm1
14099dbc0: 0f57c0                       xorps xmm0, xmm0
14099dbc3: f30f51c2                     sqrtss xmm0, xmm2
14099dbc7: 48c785a001000000000000       mov qword ptr [rbp + 0x1a0], 0
14099dbd2: 48c785a801000008000000       mov qword ptr [rbp + 0x1a8], 8
14099dbdd: 48c785b001000000000000       mov qword ptr [rbp + 0x1b0], 0
14099dbe8: c785b801000001000000         mov dword ptr [rbp + 0x1b8], 1
14099dbf2: 48c785c001000000000000       mov qword ptr [rbp + 0x1c0], 0
14099dbfd: c785d001000000000000         mov dword ptr [rbp + 0x1d0], 0
14099dc07: c785e801000000000000         mov dword ptr [rbp + 0x1e8], 0
14099dc11: c7850002000000000000         mov dword ptr [rbp + 0x200], 0
14099dc1b: c6851802000000               mov byte ptr [rbp + 0x218], 0
14099dc22: c6852802000005               mov byte ptr [rbp + 0x228], 5
14099dc29: f30f11852c020000             movss dword ptr [rbp + 0x22c], xmm0
14099dc31: 66c785480200000103           mov word ptr [rbp + 0x248], 0x301
14099dc3a: 4c8d85a0010000               lea r8, [rbp + 0x1a0]
14099dc41: 4c89f1                       mov rcx, r14
14099dc44: 4c89fa                       mov rdx, r15
14099dc47: e8a455f8ff                   call 0x1409231f0
14099dc4c: 80781800                     cmp byte ptr [rax + 0x18], 0
14099dc50: 0f84e40a0000                 je 0x14099e73a
14099dc56: 4889c3                       mov rbx, rax
14099dc59: 48395038                     cmp qword ptr [rax + 0x38], rdx
14099dc5d: 0f85d70a0000                 jne 0x14099e73a
14099dc63: 837b2802                     cmp dword ptr [rbx + 0x28], 2
14099dc67: 0f84cd0a0000                 je 0x14099e73a
14099dc6d: 488d0476                     lea rax, [rsi + rsi*2]
14099dc71: 48898588010000               mov qword ptr [rbp + 0x188], rax
14099dc78: 48c1e005                     shl rax, 5
14099dc7c: 4801c7                       add rdi, rax
14099dc7f: 4889bd90020000               mov qword ptr [rbp + 0x290], rdi
14099dc86: 488b07                       mov rax, qword ptr [rdi]
14099dc89: 488d48fd                     lea rcx, [rax - 3]
14099dc8d: 4c8d40fe                     lea r8, [rax - 2]
14099dc91: 4531c9                       xor r9d, r9d
14099dc94: 4883f903                     cmp rcx, 3
14099dc98: 4d0f42c8                     cmovb r9, r8
14099dc9c: 488d0d39e01102               lea rcx, [rip + 0x211e039]
14099dca3: 4e630489                     movsxd r8, dword ptr [rcx + r9*4]
14099dca7: 4901c8                       add r8, rcx
14099dcaa: 48899580020000               mov qword ptr [rbp + 0x280], rdx
14099dcb1: 41ffe0                       jmp r8
14099dcb4: 488b8d78030000               mov rcx, qword ptr [rbp + 0x378]
14099dcbb: 488b09                       mov rcx, qword ptr [rcx]
14099dcbe: 488b9590020000               mov rdx, qword ptr [rbp + 0x290]
14099dcc5: 4c8b4208                     mov r8, qword ptr [rdx + 8]
14099dcc9: 448b4a10                     mov r9d, dword ptr [rdx + 0x10]
14099dccd: 4889c2                       mov rdx, rax
14099dcd0: e81be90700                   call 0x140a1c5f0
14099dcd5: 4885c0                       test rax, rax
14099dcd8: 0f84eb030000                 je 0x14099e0c9
14099dcde: 4989c6                       mov r14, rax
14099dce1: 488b8580030000               mov rax, qword ptr [rbp + 0x380]
14099dce8: 488b10                       mov rdx, qword ptr [rax]
14099dceb: 488bb590020000               mov rsi, qword ptr [rbp + 0x290]
14099dcf2: 448b4610                     mov r8d, dword ptr [rsi + 0x10]
14099dcf6: 4c8d0d4bde1102               lea r9, [rip + 0x211de4b]
14099dcfd: 488d8df0000000               lea rcx, [rbp + 0xf0]
14099dd04: e817cd0700                   call 0x140a1aa20
14099dd09: 488b8588030000               mov rax, qword ptr [rbp + 0x388]
14099dd10: 488b08                       mov rcx, qword ptr [rax]
14099dd13: 488b7820                     mov rdi, qword ptr [rax + 0x20]
14099dd17: 8b5610                       mov edx, dword ptr [rsi + 0x10]
14099dd1a: e8417e1800                   call 0x140b25b60
14099dd1f: 410fb65644                   movzx edx, byte ptr [r14 + 0x44]
14099dd24: 4889f9                       mov rcx, rdi
14099dd27: 4189c0                       mov r8d, eax
14099dd2a: e861891800                   call 0x140b26690
14099dd2f: 4989c7                       mov r15, rax
14099dd32: 488dbda0010000               lea rdi, [rbp + 0x1a0]
14099dd39: 4889f9                       mov rcx, rdi
14099dd3c: 488d95f0000000               lea rdx, [rbp + 0xf0]
14099dd43: e86868e0ff                   call 0x1407a45b0
14099dd48: 488b8598010000               mov rax, qword ptr [rbp + 0x198]
14099dd4f: 4c8b00                       mov r8, qword ptr [rax]
14099dd52: 448b4e10                     mov r9d, dword ptr [rsi + 0x10]
14099dd56: 410fb64644                   movzx eax, byte ptr [r14 + 0x44]
14099dd5b: 41807e2202                   cmp byte ptr [r14 + 0x22], 2
14099dd60: 0f92442428                   setb byte ptr [rsp + 0x28]
14099dd65: 88442420                     mov byte ptr [rsp + 0x20], al
14099dd69: c644243001                   mov byte ptr [rsp + 0x30], 1
14099dd6e: 4c89f9                       mov rcx, r15
14099dd71: 4889fa                       mov rdx, rdi
14099dd74: e8875d1900                   call 0x140b33b00
14099dd79: e94b030000                   jmp 0x14099e0c9
14099dd7e: 488d0debcd2902               lea rcx, [rip + 0x229cdeb]
14099dd85: e8c6b4f301                   call 0x1428d9250
14099dd8a: 488b8598030000               mov rax, qword ptr [rbp + 0x398]
14099dd91: 488b08                       mov rcx, qword ptr [rax]
14099dd94: 4c8b4008                     mov r8, qword ptr [rax + 8]
14099dd98: 4c8bb1f8000000               mov r14, qword ptr [rcx + 0xf8]
14099dd9f: 488b8100010000               mov rax, qword ptr [rcx + 0x100]
14099dda6: 498d3c86                     lea rdi, [r14 + rax*4]
14099ddaa: 48898d90010000               mov qword ptr [rbp + 0x190], rcx
14099ddb1: 440fb68908010000             movzx r9d, byte ptr [rcx + 0x108]
14099ddb9: 488b8598010000               mov rax, qword ptr [rbp + 0x198]
14099ddc0: 488b00                       mov rax, qword ptr [rax]
14099ddc3: 488985c0000000               mov qword ptr [rbp + 0xc0], rax
14099ddca: 41ba08000000                 mov r10d, 8
14099ddd0: 4531db                       xor r11d, r11d
14099ddd3: 31f6                         xor esi, esi
14099ddd5: 4531ff                       xor r15d, r15d
14099ddd8: 4c898588020000               mov qword ptr [rbp + 0x288], r8
14099dddf: 4584c9                       test r9b, r9b
14099dde2: 7474                         je 0x14099de58
14099dde4: 4939f7                       cmp r15, rsi
14099dde7: 0f85fa000000                 jne 0x14099dee7
14099dded: 4939fe                       cmp r14, rdi
14099ddf0: 0f84d3020000                 je 0x14099e0c9
14099ddf6: 418b0e                       mov ecx, dword ptr [r14]
14099ddf9: 4983c604                     add r14, 4
14099ddfd: 498b80a8010000               mov rax, qword ptr [r8 + 0x1a8]
14099de04: 488d0cc9                     lea rcx, [rcx + rcx*8]
14099de08: 488b74c810                   mov rsi, qword ptr [rax + rcx*8 + 0x10]
14099de0d: 4885f6                       test rsi, rsi
14099de10: 74db                         je 0x14099dded
14099de12: 488d04c8                     lea rax, [rax + rcx*8]
14099de16: 488b8d90010000               mov rcx, qword ptr [rbp + 0x190]
14099de1d: 488b8910010000               mov rcx, qword ptr [rcx + 0x110]
14099de24: 48394840                     cmp qword ptr [rax + 0x40], rcx
14099de28: 0f86ac000000                 jbe 0x14099deda
14099de2e: 488b5038                     mov rdx, qword ptr [rax + 0x38]
14099de32: 488b0cca                     mov rcx, qword ptr [rdx + rcx*8]
14099de36: 4885c9                       test rcx, rcx
14099de39: 0f849b000000                 je 0x14099deda
14099de3f: 488b4018                     mov rax, qword ptr [rax + 0x18]
14099de43: 48f7d1                       not rcx
14099de46: 488d0c49                     lea rcx, [rcx + rcx*2]
14099de4a: 48c1e104                     shl rcx, 4
14099de4e: 4c8b5c0810                   mov r11, qword ptr [rax + rcx + 0x10]
14099de53: e985000000                   jmp 0x14099dedd
14099de58: 4939f7                       cmp r15, rsi
14099de5b: 0f859c000000                 jne 0x14099defd
14099de61: 4939fe                       cmp r14, rdi
14099de64: 0f845f020000                 je 0x14099e0c9
14099de6a: 418b0e                       mov ecx, dword ptr [r14]
14099de6d: 4983c604                     add r14, 4
14099de71: 498b8000010000               mov rax, qword ptr [r8 + 0x100]
14099de78: 488d0c89                     lea rcx, [rcx + rcx*4]
14099de7c: 48c1e105                     shl rcx, 5
14099de80: 488b740858                   mov rsi, qword ptr [rax + rcx + 0x58]
14099de85: 4885f6                       test rsi, rsi
14099de88: 74d7                         je 0x14099de61
14099de8a: 4801c8                       add rax, rcx
14099de8d: 8b8894000000                 mov ecx, dword ptr [rax + 0x94]
14099de93: 498b90a8010000               mov rdx, qword ptr [r8 + 0x1a8]
14099de9a: 4c8d04c9                     lea r8, [rcx + rcx*8]
14099de9e: 488b8d90010000               mov rcx, qword ptr [rbp + 0x190]
14099dea5: 488b8910010000               mov rcx, qword ptr [rcx + 0x110]
14099deac: 4a394cc240                   cmp qword ptr [rdx + r8*8 + 0x40], rcx
14099deb1: 7639                         jbe 0x14099deec
14099deb3: 4a8d14c2                     lea rdx, [rdx + r8*8]
14099deb7: 4c8b4238                     mov r8, qword ptr [rdx + 0x38]
14099debb: 498b0cc8                     mov rcx, qword ptr [r8 + rcx*8]
14099debf: 4885c9                       test rcx, rcx
14099dec2: 7428                         je 0x14099deec
14099dec4: 488b5218                     mov rdx, qword ptr [rdx + 0x18]
14099dec8: 48f7d1                       not rcx
14099decb: 488d0c49                     lea rcx, [rcx + rcx*2]
14099decf: 48c1e104                     shl rcx, 4
14099ded3: 4c8b5c0a10                   mov r11, qword ptr [rdx + rcx + 0x10]
14099ded8: eb15                         jmp 0x14099deef
14099deda: 4531db                       xor r11d, r11d
14099dedd: 4531ff                       xor r15d, r15d
14099dee0: 4c8b8588020000               mov r8, qword ptr [rbp + 0x288]
14099dee7: 4489f8                       mov eax, r15d
14099deea: eb1d                         jmp 0x14099df09
14099deec: 4531db                       xor r11d, r11d
14099deef: 4c8b5050                     mov r10, qword ptr [rax + 0x50]
14099def3: 4531ff                       xor r15d, r15d
14099def6: 4c8b8588020000               mov r8, qword ptr [rbp + 0x288]
14099defd: 4c89f8                       mov rax, r15
14099df00: 48c1e004                     shl rax, 4
14099df04: 418b440208                   mov eax, dword ptr [r10 + rax + 8]
14099df09: 486bd038                     imul rdx, rax, 0x38
14099df0d: 4c01da                       add rdx, r11
14099df10: 49ffc7                       inc r15
14099df13: 488b4218                     mov rax, qword ptr [rdx + 0x18]
14099df17: 488b8d90020000               mov rcx, qword ptr [rbp + 0x290]
14099df1e: 483b4108                     cmp rax, qword ptr [rcx + 8]
14099df22: 0f85b7feffff                 jne 0x14099dddf
14099df28: 8b4230                       mov eax, dword ptr [rdx + 0x30]
14099df2b: 488b8d90020000               mov rcx, qword ptr [rbp + 0x290]
14099df32: 3b4110                       cmp eax, dword ptr [rcx + 0x10]
14099df35: 0f85a4feffff                 jne 0x14099dddf
14099df3b: 4c89bde0000000               mov qword ptr [rbp + 0xe0], r15
14099df42: 4c899dd8000000               mov qword ptr [rbp + 0xd8], r11
14099df49: 4c8995d0000000               mov qword ptr [rbp + 0xd0], r10
14099df50: 44888dc8000000               mov byte ptr [rbp + 0xc8], r9b
14099df57: 488d8df0000000               lea rcx, [rbp + 0xf0]
14099df5e: 4c8d05fbdb1102               lea r8, [rip + 0x211dbfb]
14099df65: e8869f9500                   call 0x1412f7ef0
14099df6a: 4c8bbd00010000               mov r15, qword ptr [rbp + 0x100]
14099df71: 4d85ff                       test r15, r15
14099df74: 0f844e080000                 je 0x14099e7c8
14099df7a: 488b85f8000000               mov rax, qword ptr [rbp + 0xf8]
14099df81: 488b08                       mov rcx, qword ptr [rax]
14099df84: 48898da0010000               mov qword ptr [rbp + 0x1a0], rcx
14099df8b: 8b4808                       mov ecx, dword ptr [rax + 8]
14099df8e: 898da8010000                 mov dword ptr [rbp + 0x1a8], ecx
14099df94: 4c3bbdf0000000               cmp r15, qword ptr [rbp + 0xf0]
14099df9b: 7521                         jne 0x14099dfbe
14099df9d: c685b702000001               mov byte ptr [rbp + 0x2b7], 1
14099dfa4: 488d8df0000000               lea rcx, [rbp + 0xf0]
14099dfab: 488d15dedb1102               lea rdx, [rip + 0x211dbde]
14099dfb2: e8c9112000                   call 0x140b9f180
14099dfb7: 488b85f8000000               mov rax, qword ptr [rbp + 0xf8]
14099dfbe: 4b8d0c7f                     lea rcx, [r15 + r15*2]
14099dfc2: 8b95a8010000                 mov edx, dword ptr [rbp + 0x1a8]
14099dfc8: 89548808                     mov dword ptr [rax + rcx*4 + 8], edx
14099dfcc: 488b95a0010000               mov rdx, qword ptr [rbp + 0x1a0]
14099dfd3: 48891488                     mov qword ptr [rax + rcx*4], rdx
14099dfd7: 49ffc7                       inc r15
14099dfda: 4c89bd00010000               mov qword ptr [rbp + 0x100], r15
14099dfe1: 4c89bdb0010000               mov qword ptr [rbp + 0x1b0], r15
14099dfe8: 0f1085f0000000               movups xmm0, xmmword ptr [rbp + 0xf0]
14099dfef: 0f2985a0010000               movaps xmmword ptr [rbp + 0x1a0], xmm0
14099dff6: 48b80000000000000080         movabs rax, 0x8000000000000000
14099e000: 488985b8010000               mov qword ptr [rbp + 0x1b8], rax
14099e007: c785d001000000000000         mov dword ptr [rbp + 0x1d0], 0
14099e011: c785e001000000000040         mov dword ptr [rbp + 0x1e0], 0x40000000
14099e01b: 66c785e40100000101           mov word ptr [rbp + 0x1e4], 0x101
14099e024: c685b702000000               mov byte ptr [rbp + 0x2b7], 0
14099e02b: 488d8da0010000               lea rcx, [rbp + 0x1a0]
14099e032: 488b95c0000000               mov rdx, qword ptr [rbp + 0xc0]
14099e039: e8326f1600                   call 0x140b04f70
14099e03e: 4c8b8588020000               mov r8, qword ptr [rbp + 0x288]
14099e045: 440fb68dc8000000             movzx r9d, byte ptr [rbp + 0xc8]
14099e04d: 4c8b95d0000000               mov r10, qword ptr [rbp + 0xd0]
14099e054: 4c8b9dd8000000               mov r11, qword ptr [rbp + 0xd8]
14099e05b: 4c8bbde0000000               mov r15, qword ptr [rbp + 0xe0]
14099e062: e978fdffff                   jmp 0x14099dddf
14099e067: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
14099e06e: 488b5008                     mov rdx, qword ptr [rax + 8]
14099e072: 4c8b4010                     mov r8, qword ptr [rax + 0x10]
14099e076: 4883c210                     add rdx, 0x10
14099e07a: 4c8db5a0010000               lea r14, [rbp + 0x1a0]
14099e081: 4c89f1                       mov rcx, r14
14099e084: e8f702ecff                   call 0x14085e380
14099e089: 48b80000000000000080         movabs rax, 0x8000000000000000
14099e093: 488985b8010000               mov qword ptr [rbp + 0x1b8], rax
14099e09a: c785d001000000000000         mov dword ptr [rbp + 0x1d0], 0
14099e0a4: c785e001000000000040         mov dword ptr [rbp + 0x1e0], 0x40000000
14099e0ae: 66c785e40100000101           mov word ptr [rbp + 0x1e4], 0x101
14099e0b7: 488b8598010000               mov rax, qword ptr [rbp + 0x198]
14099e0be: 488b10                       mov rdx, qword ptr [rax]
14099e0c1: 4c89f1                       mov rcx, r14
14099e0c4: e8a76e1600                   call 0x140b04f70
14099e0c9: 488b8588010000               mov rax, qword ptr [rbp + 0x188]
14099e0d0: 48c1e004                     shl rax, 4
14099e0d4: 488bb5a8020000               mov rsi, qword ptr [rbp + 0x2a8]
14099e0db: 4801c6                       add rsi, rax
14099e0de: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
14099e0e5: 488b4830                     mov rcx, qword ptr [rax + 0x30]
14099e0e9: 488b5038                     mov rdx, qword ptr [rax + 0x38]
14099e0ed: e8dea2f1ff                   call 0x1408b83d0
14099e0f2: 488d8da0010000               lea rcx, [rbp + 0x1a0]
14099e0f9: 89c2                         mov edx, eax
14099e0fb: 4c8b8580000000               mov r8, qword ptr [rbp + 0x80]
14099e102: e889adf1ff                   call 0x1408b8e90
14099e107: 488b85a0010000               mov rax, qword ptr [rbp + 0x1a0]
14099e10e: f30f107010                   movss xmm6, dword ptr [rax + 0x10]
14099e113: f30f587014                   addss xmm6, dword ptr [rax + 0x14]
14099e118: f3440f10054f77f801           movss xmm8, dword ptr [rip + 0x1f8774f]
14099e121: f3410f59f0                   mulss xmm6, xmm8
14099e126: f3410f59f0                   mulss xmm6, xmm8
14099e12b: f30f103d5178f801             movss xmm7, dword ptr [rip + 0x1f87851]
14099e133: f30f58f7                     addss xmm6, xmm7
14099e137: f30f104e10                   movss xmm1, dword ptr [rsi + 0x10]
14099e13c: f30f105618                   movss xmm2, dword ptr [rsi + 0x18]
14099e141: 4c8db548010000               lea r14, [rbp + 0x148]
14099e148: 4c89f1                       mov rcx, r14
14099e14b: 0f28de                       movaps xmm3, xmm6
14099e14e: e8ed932f00                   call 0x140c97540
14099e153: f30f105e14                   movss xmm3, dword ptr [rsi + 0x14]
14099e158: 488dbdf0000000               lea rdi, [rbp + 0xf0]
14099e15f: 4889f9                       mov rcx, rdi
14099e162: 4c89f2                       mov rdx, r14
14099e165: 0f28d7                       movaps xmm2, xmm7
14099e168: e8e39a2f00                   call 0x140c97c50
14099e16d: 4c8d058cd71102               lea r8, [rip + 0x211d78c]
14099e174: 4c8dbda0010000               lea r15, [rbp + 0x1a0]
14099e17b: 4c89f9                       mov rcx, r15
14099e17e: 4889fa                       mov rdx, rdi
14099e181: e83ae99500                   call 0x1412fcac0
14099e186: 48b80000000000000080         movabs rax, 0x8000000000000000
14099e190: 488985b8010000               mov qword ptr [rbp + 0x1b8], rax
14099e197: c785d001000000000000         mov dword ptr [rbp + 0x1d0], 0
14099e1a1: c785e001000000000040         mov dword ptr [rbp + 0x1e0], 0x40000000
14099e1ab: 66c785e40100000100           mov word ptr [rbp + 0x1e4], 1
14099e1b4: 488b8598010000               mov rax, qword ptr [rbp + 0x198]
14099e1bb: 4c8b30                       mov r14, qword ptr [rax]
14099e1be: 4c89f9                       mov rcx, r15
14099e1c1: 4c89f2                       mov rdx, r14
14099e1c4: e8a76d1600                   call 0x140b04f70
14099e1c9: f30f583517fb0302             addss xmm6, dword ptr [rip + 0x203fb17]
14099e1d1: f3410f5df0                   minss xmm6, xmm8
14099e1d6: f30f11b580010000             movss dword ptr [rbp + 0x180], xmm6
14099e1de: 8b4618                       mov eax, dword ptr [rsi + 0x18]
14099e1e1: 898578010000                 mov dword ptr [rbp + 0x178], eax
14099e1e7: 4889b5a8020000               mov qword ptr [rbp + 0x2a8], rsi
14099e1ee: 488b4610                     mov rax, qword ptr [rsi + 0x10]
14099e1f2: 48898570010000               mov qword ptr [rbp + 0x170], rax
14099e1f9: 488d4578                     lea rax, [rbp + 0x78]
14099e1fd: 488985a0010000               mov qword ptr [rbp + 0x1a0], rax
14099e204: 488d05f5c93400               lea rax, [rip + 0x34c9f5]
14099e20b: 488985a8010000               mov qword ptr [rbp + 0x1a8], rax
14099e212: 488d8570010000               lea rax, [rbp + 0x170]
14099e219: 488985b0010000               mov qword ptr [rbp + 0x1b0], rax
14099e220: 488d05a9d53a00               lea rax, [rip + 0x3ad5a9]
14099e227: 488985b8010000               mov qword ptr [rbp + 0x1b8], rax
14099e22e: 488d8580010000               lea rax, [rbp + 0x180]
14099e235: 488985c0010000               mov qword ptr [rbp + 0x1c0], rax
14099e23c: 488d05cde63e00               lea rax, [rip + 0x3ee6cd]
14099e243: 488985c8010000               mov qword ptr [rbp + 0x1c8], rax
14099e24a: 488d056fd91102               lea rax, [rip + 0x211d96f]
14099e251: 488985f0000000               mov qword ptr [rbp + 0xf0], rax
14099e258: 48c785f800000003000000       mov qword ptr [rbp + 0xf8], 3
14099e263: 48c7851001000000000000       mov qword ptr [rbp + 0x110], 0
14099e26e: 4c89bd00010000               mov qword ptr [rbp + 0x100], r15
14099e275: 48c7850801000003000000       mov qword ptr [rbp + 0x108], 3
14099e280: 488d8d48010000               lea rcx, [rbp + 0x148]
14099e287: 488d95f0000000               lea rdx, [rbp + 0xf0]
14099e28e: e86de6ffff                   call 0x14099c900
14099e293: 488b8550010000               mov rax, qword ptr [rbp + 0x150]
14099e29a: 48898588020000               mov qword ptr [rbp + 0x288], rax
14099e2a1: 4c8bbd58010000               mov r15, qword ptr [rbp + 0x158]
14099e2a8: 8b8578010000                 mov eax, dword ptr [rbp + 0x178]
14099e2ae: 8985a8010000                 mov dword ptr [rbp + 0x1a8], eax
14099e2b4: 488b8570010000               mov rax, qword ptr [rbp + 0x170]
14099e2bb: 488985a0010000               mov qword ptr [rbp + 0x1a0], rax
14099e2c2: f30f108580010000             movss xmm0, dword ptr [rbp + 0x180]
14099e2ca: f30f1185ac010000             movss dword ptr [rbp + 0x1ac], xmm0
14099e2d2: 488d8da0010000               lea rcx, [rbp + 0x1a0]
14099e2d9: e802e7ffff                   call 0x14099c9e0
14099e2de: 488d0d0bd91102               lea rcx, [rip + 0x211d90b]
14099e2e5: 48894c2420                   mov qword ptr [rsp + 0x20], rcx
14099e2ea: 488d8da0010000               lea rcx, [rbp + 0x1a0]
14099e2f1: 488b9588020000               mov rdx, qword ptr [rbp + 0x288]
14099e2f8: 4d89f8                       mov r8, r15
14099e2fb: 4989c1                       mov r9, rax
14099e2fe: e8adf5efff                   call 0x14089d8b0
14099e303: 488b85c0010000               mov rax, qword ptr [rbp + 0x1c0]
14099e30a: 48898510010000               mov qword ptr [rbp + 0x110], rax
14099e311: 0f1085a0010000               movups xmm0, xmmword ptr [rbp + 0x1a0]
14099e318: 0f108db0010000               movups xmm1, xmmword ptr [rbp + 0x1b0]
14099e31f: 0f298d00010000               movaps xmmword ptr [rbp + 0x100], xmm1
14099e326: 0f2985f0000000               movaps xmmword ptr [rbp + 0xf0], xmm0
14099e32d: 488d95f0000000               lea rdx, [rbp + 0xf0]
14099e334: 4c89f1                       mov rcx, r14
14099e337: e844e5ac00                   call 0x14146c880
14099e33c: 488b9548010000               mov rdx, qword ptr [rbp + 0x148]
14099e343: 4885d2                       test rdx, rdx
14099e346: 7412                         je 0x14099e35a
14099e348: 41b801000000                 mov r8d, 1
14099e34e: 488b8d88020000               mov rcx, qword ptr [rbp + 0x288]
14099e355: e8c658c7ff                   call 0x140613c20
14099e35a: 807b1802                     cmp byte ptr [rbx + 0x18], 2
14099e35e: 488b8580020000               mov rax, qword ptr [rbp + 0x280]
14099e365: 488b8d90020000               mov rcx, qword ptr [rbp + 0x290]
14099e36c: 0f82c8030000                 jb 0x14099e73a
14099e372: 48394338                     cmp qword ptr [rbx + 0x38], rax
14099e376: 0f85be030000                 jne 0x14099e73a
14099e37c: 807b1903                     cmp byte ptr [rbx + 0x19], 3
14099e380: 0f85b4030000                 jne 0x14099e73a
14099e386: 4889cb                       mov rbx, rcx
14099e389: 488b8598020000               mov rax, qword ptr [rbp + 0x298]
14099e390: 488b8d88010000               mov rcx, qword ptr [rbp + 0x188]
14099e397: 488d3cc8                     lea rdi, [rax + rcx*8]
14099e39b: 488b85a8020000               mov rax, qword ptr [rbp + 0x2a8]
14099e3a2: 4c8d7810                     lea r15, [rax + 0x10]
14099e3a6: 488b8590000000               mov rax, qword ptr [rbp + 0x90]
14099e3ad: 8b8d64020000                 mov ecx, dword ptr [rbp + 0x264]
14099e3b3: 8908                         mov dword ptr [rax], ecx
14099e3b5: 488b17                       mov rdx, qword ptr [rdi]
14099e3b8: 488b8d88000000               mov rcx, qword ptr [rbp + 0x88]
14099e3bf: e82cbf92ff                   call 0x1402ca2f0
14099e3c4: 488b17                       mov rdx, qword ptr [rdi]
14099e3c7: 4c8d0552d81102               lea r8, [rip + 0x211d852]
14099e3ce: 488d4df0                     lea rcx, [rbp - 0x10]
14099e3d2: e8e9939800                   call 0x1413277c0
14099e3d7: 488b4b30                     mov rcx, qword ptr [rbx + 0x30]
14099e3db: 488b5338                     mov rdx, qword ptr [rbx + 0x38]
14099e3df: e8ec9ff1ff                   call 0x1408b83d0
14099e3e4: 41833c2409                   cmp dword ptr [r12], 9
14099e3e9: 754f                         jne 0x14099e43a
14099e3eb: 41807c240403                 cmp byte ptr [r12 + 4], 3
14099e3f1: 7547                         jne 0x14099e43a
14099e3f3: 418b4f08                     mov ecx, dword ptr [r15 + 8]
14099e3f7: 898db8010000                 mov dword ptr [rbp + 0x1b8], ecx
14099e3fd: 498b0f                       mov rcx, qword ptr [r15]
14099e400: 48898db0010000               mov qword ptr [rbp + 0x1b0], rcx
14099e407: c685a001000020               mov byte ptr [rbp + 0x1a0], 0x20
14099e40e: 8885a1010000                 mov byte ptr [rbp + 0x1a1], al
14099e414: c685bc01000000               mov byte ptr [rbp + 0x1bc], 0
14099e41b: 4c8d055ed81102               lea r8, [rip + 0x211d85e]
14099e422: 488d95a0010000               lea rdx, [rbp + 0x1a0]
14099e429: 488b8da0030000               mov rcx, qword ptr [rbp + 0x3a0]
14099e430: e85b8e9800                   call 0x141327290
14099e435: e900030000                   jmp 0x14099e73a
14099e43a: 418b4708                     mov eax, dword ptr [r15 + 8]
14099e43e: 8985b8010000                 mov dword ptr [rbp + 0x1b8], eax
14099e444: 498b07                       mov rax, qword ptr [r15]
14099e447: 488985b0010000               mov qword ptr [rbp + 0x1b0], rax
14099e44e: c685a001000025               mov byte ptr [rbp + 0x1a0], 0x25
14099e455: c685bc01000000               mov byte ptr [rbp + 0x1bc], 0
14099e45c: 4c8d05d5d71102               lea r8, [rip + 0x211d7d5]
14099e463: 488d95a0010000               lea rdx, [rbp + 0x1a0]
14099e46a: 488b8da0030000               mov rcx, qword ptr [rbp + 0x3a0]
14099e471: e81a8e9800                   call 0x141327290
14099e476: 488b85a8020000               mov rax, qword ptr [rbp + 0x2a8]
14099e47d: 0f2830                       movaps xmm6, xmmword ptr [rax]
14099e480: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
14099e487: 8b4018                       mov eax, dword ptr [rax + 0x18]
14099e48a: 85c0                         test eax, eax
14099e48c: 7444                         je 0x14099e4d2
14099e48e: 83f801                       cmp eax, 1
14099e491: 7566                         jne 0x14099e4f9
14099e493: 418b4708                     mov eax, dword ptr [r15 + 8]
14099e497: 488b8d90020000               mov rcx, qword ptr [rbp + 0x290]
14099e49e: f3440f7e4120                 movq xmm8, qword ptr [rcx + 0x20]
14099e4a4: 898560020000                 mov dword ptr [rbp + 0x260], eax
14099e4aa: 498b07                       mov rax, qword ptr [r15]
14099e4ad: 48898558020000               mov qword ptr [rbp + 0x258], rax
14099e4b4: 0f297550                     movaps xmmword ptr [rbp + 0x50], xmm6
14099e4b8: 488d8d60010000               lea rcx, [rbp + 0x160]
14099e4bf: 488d5550                     lea rdx, [rbp + 0x50]
14099e4c3: e858dfffff                   call 0x14099c420
14099e4c8: 31f6                         xor esi, esi
14099e4ca: 4531ff                       xor r15d, r15d
14099e4cd: e925010000                   jmp 0x14099e5f7
14099e4d2: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
14099e4d9: 488b7828                     mov rdi, qword ptr [rax + 0x28]
14099e4dd: 83781c01                     cmp dword ptr [rax + 0x1c], 1
14099e4e1: 754f                         jne 0x14099e532
14099e4e3: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
14099e4ea: f20f107820                   movsd xmm7, qword ptr [rax + 0x20]
14099e4ef: 664c0f6ec7                   movq xmm8, rdi
14099e4f4: e9c1000000                   jmp 0x14099e5ba
14099e4f9: 418b4708                     mov eax, dword ptr [r15 + 8]
14099e4fd: 898560020000                 mov dword ptr [rbp + 0x260], eax
14099e503: 498b07                       mov rax, qword ptr [r15]
14099e506: 48898558020000               mov qword ptr [rbp + 0x258], rax
14099e50d: 48b8000000000000803f         movabs rax, 0x3f80000000000000
14099e517: 48898560010000               mov qword ptr [rbp + 0x160], rax
14099e51e: c7856801000000000000         mov dword ptr [rbp + 0x168], 0
14099e528: 41b70b                       mov r15b, 0xb
14099e52b: 31f6                         xor esi, esi
14099e52d: e9c5000000                   jmp 0x14099e5f7
14099e532: 0f57c0                       xorps xmm0, xmm0
14099e535: 488b85a8020000               mov rax, qword ptr [rbp + 0x2a8]
14099e53c: f30f584018                   addss xmm0, dword ptr [rax + 0x18]
14099e541: f20f104810                   movsd xmm1, qword ptr [rax + 0x10]
14099e546: 0f580db3460e02               addps xmm1, xmmword ptr [rip + 0x20e46b3]
14099e54d: 0f138df0000000               movlps qword ptr [rbp + 0xf0], xmm1
14099e554: f30f1185f8000000             movss dword ptr [rbp + 0xf8], xmm0
14099e55c: f20f10059cf70302             movsd xmm0, qword ptr [rip + 0x203f79c]
14099e564: f20f1185fc000000             movsd qword ptr [rbp + 0xfc], xmm0
14099e56c: c7850401000000000080         mov dword ptr [rbp + 0x104], 0x80000000
14099e576: c744243002000000             mov dword ptr [rsp + 0x30], 2
14099e57e: c7442428ffff7f7f             mov dword ptr [rsp + 0x28], 0x7f7fffff
14099e586: 488d8da0010000               lea rcx, [rbp + 0x1a0]
14099e58d: 4c8d85f0000000               lea r8, [rbp + 0xf0]
14099e594: 4c89ea                       mov rdx, r13
14099e597: 4531c9                       xor r9d, r9d
14099e59a: e8c1f6f2ff                   call 0x1408cdc60
14099e59f: 8b85d4010000                 mov eax, dword ptr [rbp + 0x1d4]
14099e5a5: 83f802                       cmp eax, 2
14099e5a8: 0f85b6010000                 jne 0x14099e764
14099e5ae: 664c0f6ec7                   movq xmm8, rdi
14099e5b3: 0f283d2673f801               movaps xmm7, xmmword ptr [rip + 0x1f87326]
14099e5ba: 40b701                       mov dil, 1
14099e5bd: 4531f6                       xor r14d, r14d
14099e5c0: 418b4708                     mov eax, dword ptr [r15 + 8]
14099e5c4: 898560020000                 mov dword ptr [rbp + 0x260], eax
14099e5ca: 498b07                       mov rax, qword ptr [r15]
14099e5cd: 48898558020000               mov qword ptr [rbp + 0x258], rax
14099e5d4: 48b8000000000000803f         movabs rax, 0x3f80000000000000
14099e5de: 48898560010000               mov qword ptr [rbp + 0x160], rax
14099e5e5: c7856801000000000000         mov dword ptr [rbp + 0x168], 0
14099e5ef: 41b702                       mov r15b, 2
14099e5f2: be01000000                   mov esi, 1
14099e5f7: 488b8590020000               mov rax, qword ptr [rbp + 0x290]
14099e5fe: 488b4830                     mov rcx, qword ptr [rax + 0x30]
14099e602: 488b5038                     mov rdx, qword ptr [rax + 0x38]
14099e606: e8c59df1ff                   call 0x1408b83d0
14099e60b: 89c3                         mov ebx, eax
14099e60d: 488d8da0010000               lea rcx, [rbp + 0x1a0]
14099e614: 488b5560                     mov rdx, qword ptr [rbp + 0x60]
14099e618: e853882f00                   call 0x140c96e70
14099e61d: 0f2885a0010000               movaps xmm0, xmmword ptr [rbp + 0x1a0]
14099e624: 0f5705d5ea0202               xorps xmm0, xmmword ptr [rip + 0x202ead5]
14099e62b: 0f28ce                       movaps xmm1, xmm6
14099e62e: 0fc6ce00                     shufps xmm1, xmm6, 0
14099e632: 0f28d6                       movaps xmm2, xmm6
14099e635: 0fc6d655                     shufps xmm2, xmm6, 0x55
14099e639: 0f28de                       movaps xmm3, xmm6
14099e63c: 0fc6deaa                     shufps xmm3, xmm6, 0xaa
14099e640: 0fc6f6ff                     shufps xmm6, xmm6, 0xff
14099e644: 0f59f0                       mulps xmm6, xmm0
14099e647: 660f70e01b                   pshufd xmm4, xmm0, 0x1b
14099e64c: 0f59e1                       mulps xmm4, xmm1
14099e64f: 660f70c84e                   pshufd xmm1, xmm0, 0x4e
14099e654: 0f59ca                       mulps xmm1, xmm2
14099e657: 0f592512eb0202               mulps xmm4, xmmword ptr [rip + 0x202eb12]
14099e65e: 660f70c0b1                   pshufd xmm0, xmm0, 0xb1
14099e663: 0f59c3                       mulps xmm0, xmm3
14099e666: 0f590d13eb0202               mulps xmm1, xmmword ptr [rip + 0x202eb13]
14099e66d: 0f59051ceb0202               mulps xmm0, xmmword ptr [rip + 0x202eb1c]
14099e674: 0f58e6                       addps xmm4, xmm6
14099e677: 0f58c1                       addps xmm0, xmm1
14099e67a: 0f58c4                       addps xmm0, xmm4
14099e67d: 488b8580020000               mov rax, qword ptr [rbp + 0x280]
14099e684: 488985b0010000               mov qword ptr [rbp + 0x1b0], rax
14099e68b: 4488bdb8010000               mov byte ptr [rbp + 0x1b8], r15b
14099e692: 4088bdb9010000               mov byte ptr [rbp + 0x1b9], dil
14099e699: 4488b5ba010000               mov byte ptr [rbp + 0x1ba], r14b
14099e6a0: 8b8570020000                 mov eax, dword ptr [rbp + 0x270]
14099e6a6: 8985bb010000                 mov dword ptr [rbp + 0x1bb], eax
14099e6ac: 0fb68574020000               movzx eax, byte ptr [rbp + 0x274]
14099e6b3: 8885bf010000                 mov byte ptr [rbp + 0x1bf], al
14099e6b9: 66440f7f85c0010000           movdqa xmmword ptr [rbp + 0x1c0], xmm8
14099e6c2: 488b8558020000               mov rax, qword ptr [rbp + 0x258]
14099e6c9: 488985d0010000               mov qword ptr [rbp + 0x1d0], rax
14099e6d0: 8b8560020000                 mov eax, dword ptr [rbp + 0x260]
14099e6d6: 8985d8010000                 mov dword ptr [rbp + 0x1d8], eax
14099e6dc: 488b8560010000               mov rax, qword ptr [rbp + 0x160]
14099e6e3: 488985dc010000               mov qword ptr [rbp + 0x1dc], rax
14099e6ea: 8b8568010000                 mov eax, dword ptr [rbp + 0x168]
14099e6f0: 8985e4010000                 mov dword ptr [rbp + 0x1e4], eax
14099e6f6: 89b5e8010000                 mov dword ptr [rbp + 0x1e8], esi
14099e6fc: 0f13bdec010000               movlps qword ptr [rbp + 0x1ec], xmm7
14099e703: 889df8010000                 mov byte ptr [rbp + 0x1f8], bl
14099e709: 0f2985a0010000               movaps xmmword ptr [rbp + 0x1a0], xmm0
14099e710: 4c8d0539d51102               lea r8, [rip + 0x211d539]
14099e717: 488d95a0010000               lea rdx, [rbp + 0x1a0]
14099e71e: 488b8d68030000               mov rcx, qword ptr [rbp + 0x368]
14099e725: e8468e9800                   call 0x141327570
14099e72a: 488d1537d51102               lea rdx, [rip + 0x211d537]
14099e731: 488d4dd0                     lea rcx, [rbp - 0x30]
14099e735: e876929800                   call 0x1413279b0
14099e73a: 0f28b5c0020000               movaps xmm6, xmmword ptr [rbp + 0x2c0]
14099e741: 0f28bdd0020000               movaps xmm7, xmmword ptr [rbp + 0x2d0]
14099e748: 440f2885e0020000             movaps xmm8, xmmword ptr [rbp + 0x2e0]
14099e750: 4881c478030000               add rsp, 0x378
14099e757: 5b                           pop rbx
14099e758: 5f                           pop rdi
14099e759: 5e                           pop rsi
14099e75a: 415c                         pop r12
14099e75c: 415d                         pop r13
14099e75e: 415e                         pop r14
14099e760: 415f                         pop r15
14099e762: 5d                           pop rbp
14099e763: c3                           ret 
14099e764: a801                         test al, 1
14099e766: 0f8442feffff                 je 0x14099e5ae
14099e76c: 80bda001000002               cmp byte ptr [rbp + 0x1a0], 2
14099e773: 0f8535feffff                 jne 0x14099e5ae
14099e779: f20f10bdd8010000             movsd xmm7, qword ptr [rbp + 0x1d8]
14099e781: 0fb6bda1010000               movzx edi, byte ptr [rbp + 0x1a1]
14099e788: 440fb6b5a2010000             movzx r14d, byte ptr [rbp + 0x1a2]
14099e790: 8b85a3010000                 mov eax, dword ptr [rbp + 0x1a3]
14099e796: 898570020000                 mov dword ptr [rbp + 0x270], eax
14099e79c: 0fb685a7010000               movzx eax, byte ptr [rbp + 0x1a7]
14099e7a3: 888574020000                 mov byte ptr [rbp + 0x274], al
14099e7a9: f3440f6f85a8010000           movdqu xmm8, xmmword ptr [rbp + 0x1a8]
14099e7b2: e909feffff                   jmp 0x14099e5c0
14099e7b7: b910000000                   mov ecx, 0x10
14099e7bc: ba60000000                   mov edx, 0x60
14099e7c1: e83aa8f301                   call 0x1428d9000
14099e7c6: eb17                         jmp 0x14099e7df
14099e7c8: c685b702000001               mov byte ptr [rbp + 0x2b7], 1
14099e7cf: 4c8d05a2d31102               lea r8, [rip + 0x211d3a2]
14099e7d6: 31c9                         xor ecx, ecx
14099e7d8: 31d2                         xor edx, edx
14099e7da: e839adf301                   call 0x1428d9518
14099e7df: 0f0b                         ud2 
14099e7e1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14099e7f0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
14099e7f5: 55                           push rbp
14099e7f6: 4157                         push r15
14099e7f8: 4156                         push r14
14099e7fa: 4155                         push r13
14099e7fc: 4154                         push r12
14099e7fe: 56                           push rsi
14099e7ff: 57                           push rdi
14099e800: 53                           push rbx
14099e801: 4883ec78                     sub rsp, 0x78
14099e805: 488daa80000000               lea rbp, [rdx + 0x80]
14099e80c: 440f29442440                 movaps xmmword ptr [rsp + 0x40], xmm8
14099e812: 0f297c2450                   movaps xmmword ptr [rsp + 0x50], xmm7
14099e817: 0f29742460                   movaps xmmword ptr [rsp + 0x60], xmm6
14099e81c: 80bdb702000000               cmp byte ptr [rbp + 0x2b7], 0
14099e823: 7426                         je 0x14099e84b
14099e825: 488b85f0000000               mov rax, qword ptr [rbp + 0xf0]
14099e82c: 4885c0                       test rax, rax
14099e82f: 741a                         je 0x14099e84b
14099e831: 488b8df8000000               mov rcx, qword ptr [rbp + 0xf8]
14099e838: 48c1e002                     shl rax, 2
14099e83c: 488d1440                     lea rdx, [rax + rax*2]
14099e840: 41b804000000                 mov r8d, 4
14099e846: e8d553c7ff                   call 0x140613c20
14099e84b: 0f28742460                   movaps xmm6, xmmword ptr [rsp + 0x60]
14099e850: 0f287c2450                   movaps xmm7, xmmword ptr [rsp + 0x50]
14099e855: 440f28442440                 movaps xmm8, xmmword ptr [rsp + 0x40]
14099e85b: 4883c478                     add rsp, 0x78
14099e85f: 5b                           pop rbx
14099e860: 5f                           pop rdi
14099e861: 5e                           pop rsi
14099e862: 415c                         pop r12
14099e864: 415d                         pop r13
14099e866: 415e                         pop r14
14099e868: 415f                         pop r15
14099e86a: 5d                           pop rbp
14099e86b: c3                           ret 
14099e86c: 0f1f4000                     nop dword ptr [rax]
14099e870: 4889542410                   mov qword ptr [rsp + 0x10], rdx
14099e875: 55                           push rbp
14099e876: 4157                         push r15
14099e878: 4156                         push r14
14099e87a: 4155                         push r13
14099e87c: 4154                         push r12
14099e87e: 56                           push rsi
14099e87f: 57                           push rdi
14099e880: 53                           push rbx
14099e881: 4883ec78                     sub rsp, 0x78
14099e885: 488daa80000000               lea rbp, [rdx + 0x80]
14099e88c: 440f29442440                 movaps xmmword ptr [rsp + 0x40], xmm8
14099e892: 0f297c2450                   movaps xmmword ptr [rsp + 0x50], xmm7
14099e897: 0f29742460                   movaps xmmword ptr [rsp + 0x60], xmm6
14099e89c: 488b9548010000               mov rdx, qword ptr [rbp + 0x148]
14099e8a3: 4885d2                       test rdx, rdx
14099e8a6: 7412                         je 0x14099e8ba
14099e8a8: 41b801000000                 mov r8d, 1
14099e8ae: 488b8d88020000               mov rcx, qword ptr [rbp + 0x288]
14099e8b5: e86653c7ff                   call 0x140613c20
14099e8ba: 0f28742460                   movaps xmm6, xmmword ptr [rsp + 0x60]
14099e8bf: 0f287c2450                   movaps xmm7, xmmword ptr [rsp + 0x50]
14099e8c4: 440f28442440                 movaps xmm8, xmmword ptr [rsp + 0x40]
14099e8ca: 4883c478                     add rsp, 0x78
14099e8ce: 5b                           pop rbx
14099e8cf: 5f                           pop rdi
14099e8d0: 5e                           pop rsi
14099e8d1: 415c                         pop r12
14099e8d3: 415d                         pop r13
14099e8d5: 415e                         pop r14
14099e8d7: 415f                         pop r15
14099e8d9: 5d                           pop rbp
14099e8da: c3                           ret 
14099e8db: 0f1f440000                   nop dword ptr [rax + rax]
14099e8e0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
14099e8e5: 55                           push rbp
14099e8e6: 4157                         push r15
14099e8e8: 4156                         push r14
14099e8ea: 4155                         push r13
14099e8ec: 4154                         push r12
14099e8ee: 56                           push rsi
14099e8ef: 57                           push rdi
14099e8f0: 53                           push rbx
14099e8f1: 4883ec78                     sub rsp, 0x78
14099e8f5: 488daa80000000               lea rbp, [rdx + 0x80]
14099e8fc: 440f29442440                 movaps xmmword ptr [rsp + 0x40], xmm8
14099e902: 0f297c2450                   movaps xmmword ptr [rsp + 0x50], xmm7
14099e907: 660f7f742460                 movdqa xmmword ptr [rsp + 0x60], xmm6
14099e90d: 80bd9802000000               cmp byte ptr [rbp + 0x298], 0
14099e914: 742f                         je 0x14099e945
14099e916: 488d8550010000               lea rax, [rbp + 0x150]
14099e91d: 48898580010000               mov qword ptr [rbp + 0x180], rax
14099e924: 488d8580010000               lea rax, [rbp + 0x180]
14099e92b: 48898570010000               mov qword ptr [rbp + 0x170], rax
14099e932: 488d0da7d01102               lea rcx, [rip + 0x211d0a7]
14099e939: 488d9570010000               lea rdx, [rbp + 0x170]
14099e940: e88b099900                   call 0x14132f2d0
14099e945: 0f28742460                   movaps xmm6, xmmword ptr [rsp + 0x60]
14099e94a: 0f287c2450                   movaps xmm7, xmmword ptr [rsp + 0x50]
14099e94f: 440f28442440                 movaps xmm8, xmmword ptr [rsp + 0x40]
14099e955: 4883c478                     add rsp, 0x78
14099e959: 5b                           pop rbx
14099e95a: 5f                           pop rdi
14099e95b: 5e                           pop rsi
14099e95c: 415c                         pop r12
14099e95e: 415d                         pop r13
14099e960: 415e                         pop r14
14099e962: 415f                         pop r15
14099e964: 5d                           pop rbp
14099e965: c3                           ret 
14099e966: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
14099e970: 4889542410                   mov qword ptr [rsp + 0x10], rdx
14099e975: 55                           push rbp
14099e976: 4157                         push r15
14099e978: 4156                         push r14
14099e97a: 4155                         push r13
14099e97c: 4154                         push r12
14099e97e: 56                           push rsi
14099e97f: 57                           push rdi
14099e980: 53                           push rbx
14099e981: 4883ec78                     sub rsp, 0x78
14099e985: 488daa80000000               lea rbp, [rdx + 0x80]
14099e98c: 440f29442440                 movaps xmmword ptr [rsp + 0x40], xmm8
14099e992: 0f297c2450                   movaps xmmword ptr [rsp + 0x50], xmm7
14099e997: 660f7f742460                 movdqa xmmword ptr [rsp + 0x60], xmm6
14099e99d: 488b8598020000               mov rax, qword ptr [rbp + 0x298]
14099e9a4: 48ff00                       inc qword ptr [rax]
14099e9a7: 0f28742460                   movaps xmm6, xmmword ptr [rsp + 0x60]
14099e9ac: 0f287c2450                   movaps xmm7, xmmword ptr [rsp + 0x50]
14099e9b1: 440f28442440                 movaps xmm8, xmmword ptr [rsp + 0x40]
14099e9b7: 4883c478                     add rsp, 0x78
14099e9bb: 5b                           pop rbx
14099e9bc: 5f                           pop rdi
14099e9bd: 5e                           pop rsi
14099e9be: 415c                         pop r12
14099e9c0: 415d                         pop r13
14099e9c2: 415e                         pop r14
14099e9c4: 415f                         pop r15
14099e9c6: 5d                           pop rbp
14099e9c7: c3                           ret 
14099e9c8: 0f1f840000000000             nop dword ptr [rax + rax]
14099e9d0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
14099e9d5: 55                           push rbp
14099e9d6: 4157                         push r15
14099e9d8: 4156                         push r14
14099e9da: 4155                         push r13
14099e9dc: 4154                         push r12
14099e9de: 56                           push rsi
14099e9df: 57                           push rdi
14099e9e0: 53                           push rbx
14099e9e1: 4883ec78                     sub rsp, 0x78
14099e9e5: 488daa80000000               lea rbp, [rdx + 0x80]
14099e9ec: 440f29442440                 movaps xmmword ptr [rsp + 0x40], xmm8
14099e9f2: 0f297c2450                   movaps xmmword ptr [rsp + 0x50], xmm7
14099e9f7: 660f7f742460                 movdqa xmmword ptr [rsp + 0x60], xmm6
14099e9fd: 488b8580020000               mov rax, qword ptr [rbp + 0x280]
14099ea04: 48ff00                       inc qword ptr [rax]
14099ea07: 0f28742460                   movaps xmm6, xmmword ptr [rsp + 0x60]
14099ea0c: 0f287c2450                   movaps xmm7, xmmword ptr [rsp + 0x50]
14099ea11: 440f28442440                 movaps xmm8, xmmword ptr [rsp + 0x40]
14099ea17: 4883c478                     add rsp, 0x78
14099ea1b: 5b                           pop rbx
14099ea1c: 5f                           pop rdi
14099ea1d: 5e                           pop rsi
14099ea1e: 415c                         pop r12
14099ea20: 415d                         pop r13
14099ea22: 415e                         pop r14
14099ea24: 415f                         pop r15
14099ea26: 5d                           pop rbp
14099ea27: c3                           ret 
