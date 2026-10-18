1415ca5f0: 55  push rbp
1415ca5f1: 4157  push r15
1415ca5f3: 4156  push r14
1415ca5f5: 4155  push r13
1415ca5f7: 4154  push r12
1415ca5f9: 56  push rsi
1415ca5fa: 57  push rdi
1415ca5fb: 53  push rbx
1415ca5fc: 4881ecc8010000  sub rsp, 0x1c8
1415ca603: 488dac2480000000  lea rbp, [rsp + 0x80]
1415ca60b: 440f298530010000  movaps xmmword ptr [rbp + 0x130], xmm8
1415ca613: 0f29bd20010000  movaps xmmword ptr [rbp + 0x120], xmm7
1415ca61a: 0f29b510010000  movaps xmmword ptr [rbp + 0x110], xmm6
1415ca621: 48c78508010000feffffff  mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
1415ca62c: 48898d00010000  mov qword ptr [rbp + 0x100], rcx
1415ca633: 488b4210  mov rax, qword ptr [rdx + 0x10]
1415ca637: 488945d0  mov qword ptr [rbp - 0x30], rax
1415ca63b: 4c8d75f8  lea r14, [rbp - 8]
1415ca63f: 4c89f1  mov rcx, r14
1415ca642: e889da5bff  call 0x140b880d0
1415ca647: 488dbda1000000  lea rdi, [rbp + 0xa1]
1415ca64e: c685fa00000005  mov byte ptr [rbp + 0xfa], 5
1415ca655: c685fb00000040  mov byte ptr [rbp + 0xfb], 0x40
1415ca65c: c685fd00000004  mov byte ptr [rbp + 0xfd], 4
1415ca663: c685fc00000002  mov byte ptr [rbp + 0xfc], 2
1415ca66a: c7858400000000000000  mov dword ptr [rbp + 0x84], 0
1415ca674: 4c8da5d0000000  lea r12, [rbp + 0xd0]
1415ca67b: 488d3586116b01  lea rsi, [rip + 0x16b1186]
1415ca682: c685fe00000002  mov byte ptr [rbp + 0xfe], 2
1415ca689: c685ff00000002  mov byte ptr [rbp + 0xff], 2
1415ca690: 488d1d55116b01  lea rbx, [rip + 0x16b1155]
1415ca697: c7858800000000000000  mov dword ptr [rbp + 0x88], 0
1415ca6a1: c7859c00000000000000  mov dword ptr [rbp + 0x9c], 0
1415ca6ab: c7859800000000000000  mov dword ptr [rbp + 0x98], 0
1415ca6b5: c7859400000000000000  mov dword ptr [rbp + 0x94], 0
1415ca6bf: c7859000000000000000  mov dword ptr [rbp + 0x90], 0
1415ca6c9: c7858c00000000000000  mov dword ptr [rbp + 0x8c], 0
1415ca6d3: c7457c00000000  mov dword ptr [rbp + 0x7c], 0
1415ca6da: c7858000000000000000  mov dword ptr [rbp + 0x80], 0
1415ca6e4: c7457800000000  mov dword ptr [rbp + 0x78], 0
1415ca6eb: 0f1f440000  nop dword ptr [rax + rax]
1415ca6f0: 4c89e1  mov rcx, r12
1415ca6f3: 4c89f2  mov rdx, r14
1415ca6f6: e8551de7ff  call 0x14143c450
1415ca6fb: 488b85d0000000  mov rax, qword ptr [rbp + 0xd0]
1415ca702: 4885c0  test rax, rax
1415ca705: 0f8402080000  je 0x1415caf0d
1415ca70b: 488b8de0000000  mov rcx, qword ptr [rbp + 0xe0]
1415ca712: 488d1449  lea rdx, [rcx + rcx*2]
1415ca716: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415ca71e: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415ca726: 4c894d60  mov qword ptr [rbp + 0x60], r9
1415ca72a: 4c8bbcd078010000  mov r15, qword ptr [rax + rdx*8 + 0x178]
1415ca732: 48c1e105  shl rcx, 5
1415ca736: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415ca73a: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415ca73f: 0f2985a0000000  movaps xmmword ptr [rbp + 0xa0], xmm0
1415ca746: 0f298db0000000  movaps xmmword ptr [rbp + 0xb0], xmm1
1415ca74d: 4c894568  mov qword ptr [rbp + 0x68], r8
1415ca751: 4c89c0  mov rax, r8
1415ca754: 48f7d8  neg rax
1415ca757: 0f80b0070000  jo 0x1415caf0d
1415ca75d: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415ca761: 4883f806  cmp rax, 6
1415ca765: 0f8745010000  ja 0x1415ca8b0
1415ca76b: 48630483  movsxd rax, dword ptr [rbx + rax*4]
1415ca76f: 4801d8  add rax, rbx
1415ca772: ffe0  jmp rax
1415ca774: 0f2885a0000000  movaps xmm0, xmmword ptr [rbp + 0xa0]
1415ca77b: 0f288db0000000  movaps xmm1, xmmword ptr [rbp + 0xb0]
1415ca782: 488d4540  lea rax, [rbp + 0x40]
1415ca786: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415ca78a: 0f1100  movups xmmword ptr [rax], xmm0
1415ca78d: 4c89e1  mov rcx, r12
1415ca790: 488b5560  mov rdx, qword ptr [rbp + 0x60]
1415ca794: 4d89f8  mov r8, r15
1415ca797: e8d4691f00  call 0x1417c1170
1415ca79c: 488b5568  mov rdx, qword ptr [rbp + 0x68]
1415ca7a0: 4885d2  test rdx, rdx
1415ca7a3: 740f  je 0x1415ca7b4
1415ca7a5: 41b801000000  mov r8d, 1
1415ca7ab: 488b4d60  mov rcx, qword ptr [rbp + 0x60]
1415ca7af: e86c9404ff  call 0x140613c20
1415ca7b4: 80bdd000000000  cmp byte ptr [rbp + 0xd0], 0
1415ca7bb: 0f856f070000  jne 0x1415caf30
1415ca7c1: 0fb685d1000000  movzx eax, byte ptr [rbp + 0xd1]
1415ca7c8: 48630486  movsxd rax, dword ptr [rsi + rax*4]
1415ca7cc: 4801f0  add rax, rsi
1415ca7cf: ffe0  jmp rax
1415ca7d1: 80bdfa00000005  cmp byte ptr [rbp + 0xfa], 5
1415ca7d8: 0f85080b0000  jne 0x1415cb2e6
1415ca7de: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415ca7e2: c6454006  mov byte ptr [rbp + 0x40], 6
1415ca7e6: 3c06  cmp al, 6
1415ca7e8: 0f848d090000  je 0x1415cb17b
1415ca7ee: 8885d0000000  mov byte ptr [rbp + 0xd0], al
1415ca7f4: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415ca7f9: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415ca7fe: 488d85d1000000  lea rax, [rbp + 0xd1]
1415ca805: 0f11480f  movups xmmword ptr [rax + 0xf], xmm1
1415ca809: 0f1100  movups xmmword ptr [rax], xmm0
1415ca80c: 488d8da0000000  lea rcx, [rbp + 0xa0]
1415ca813: 4c89e2  mov rdx, r12
1415ca816: e8850048ff  call 0x140a4a8a0
1415ca81b: 0fb685c2000000  movzx eax, byte ptr [rbp + 0xc2]
1415ca822: 4c8bada0000000  mov r13, qword ptr [rbp + 0xa0]
1415ca829: 8885fa000000  mov byte ptr [rbp + 0xfa], al
1415ca82f: 3c05  cmp al, 5
1415ca831: 0f84380a0000  je 0x1415cb26f
1415ca837: 0f104707  movups xmm0, xmmword ptr [rdi + 7]
1415ca83b: 0f104f11  movups xmm1, xmmword ptr [rdi + 0x11]
1415ca83f: 0f114dba  movups xmmword ptr [rbp - 0x46], xmm1
1415ca843: 0f2945b0  movaps xmmword ptr [rbp - 0x50], xmm0
1415ca847: 8b4722  mov eax, dword ptr [rdi + 0x22]
1415ca84a: 894570  mov dword ptr [rbp + 0x70], eax
1415ca84d: 0fb64726  movzx eax, byte ptr [rdi + 0x26]
1415ca851: 884574  mov byte ptr [rbp + 0x74], al
1415ca854: e997feffff  jmp 0x1415ca6f0
1415ca859: 488b5548  mov rdx, qword ptr [rbp + 0x48]
1415ca85d: 4885d2  test rdx, rdx
1415ca860: 0f840effffff  je 0x1415ca774
1415ca866: 488b4d50  mov rcx, qword ptr [rbp + 0x50]
1415ca86a: 41b801000000  mov r8d, 1
1415ca870: e8ab9304ff  call 0x140613c20
1415ca875: e9fafeffff  jmp 0x1415ca774
1415ca87a: 488b4d50  mov rcx, qword ptr [rbp + 0x50]
1415ca87e: 488b5558  mov rdx, qword ptr [rbp + 0x58]
1415ca882: 48894dd8  mov qword ptr [rbp - 0x28], rcx
1415ca886: e8e5e42bff  call 0x140888d70
1415ca88b: 488b5548  mov rdx, qword ptr [rbp + 0x48]
1415ca88f: 4885d2  test rdx, rdx
1415ca892: 0f84dcfeffff  je 0x1415ca774
1415ca898: 48c1e205  shl rdx, 5
1415ca89c: 41b808000000  mov r8d, 8
1415ca8a2: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415ca8a6: e8759304ff  call 0x140613c20
1415ca8ab: e9c4feffff  jmp 0x1415ca774
1415ca8b0: 488d4d48  lea rcx, [rbp + 0x48]
1415ca8b4: e887e62cff  call 0x140898f40
1415ca8b9: e9b6feffff  jmp 0x1415ca774
1415ca8be: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415ca8c2: c6454006  mov byte ptr [rbp + 0x40], 6
1415ca8c6: 3c06  cmp al, 6
1415ca8c8: 0f843f080000  je 0x1415cb10d
1415ca8ce: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415ca8d4: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415ca8d9: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415ca8de: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415ca8e2: 0f1107  movups xmmword ptr [rdi], xmm0
1415ca8e5: 488d8da0000000  lea rcx, [rbp + 0xa0]
1415ca8ec: e84f90fdff  call 0x1415a3940
1415ca8f1: 4885c0  test rax, rax
1415ca8f4: 0f84f6fdffff  je 0x1415ca6f0
1415ca8fa: e96d090000  jmp 0x1415cb26c
1415ca8ff: 80bdfc00000002  cmp byte ptr [rbp + 0xfc], 2
1415ca906: 0f85060a0000  jne 0x1415cb312
1415ca90c: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415ca910: c6454006  mov byte ptr [rbp + 0x40], 6
1415ca914: 3c06  cmp al, 6
1415ca916: 0f8433080000  je 0x1415cb14f
1415ca91c: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415ca922: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415ca927: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415ca92c: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415ca930: 0f1107  movups xmmword ptr [rdi], xmm0
1415ca933: 4c89e1  mov rcx, r12
1415ca936: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415ca93d: e8deee47ff  call 0x140a49820
1415ca942: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415ca949: 0f85e1050000  jne 0x1415caf30
1415ca94f: 0fb685d1000000  movzx eax, byte ptr [rbp + 0xd1]
1415ca956: 8885fc000000  mov byte ptr [rbp + 0xfc], al
1415ca95c: e98ffdffff  jmp 0x1415ca6f0
1415ca961: 83bd9c00000000  cmp dword ptr [rbp + 0x9c], 0
1415ca968: 0f85fc090000  jne 0x1415cb36a
1415ca96e: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415ca972: c6454006  mov byte ptr [rbp + 0x40], 6
1415ca976: 3c06  cmp al, 6
1415ca978: 0f84dd080000  je 0x1415cb25b
1415ca97e: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415ca984: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415ca989: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415ca98e: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415ca992: 0f1107  movups xmmword ptr [rdi], xmm0
1415ca995: 4c89e1  mov rcx, r12
1415ca998: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415ca99f: e8ccf9fcff  call 0x14159a370
1415ca9a4: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415ca9ab: 0f857f050000  jne 0x1415caf30
1415ca9b1: f30f10b5d4000000  movss xmm6, dword ptr [rbp + 0xd4]
1415ca9b9: c7859c00000001000000  mov dword ptr [rbp + 0x9c], 1
1415ca9c3: e928fdffff  jmp 0x1415ca6f0
1415ca9c8: 837d7c00  cmp dword ptr [rbp + 0x7c], 0
1415ca9cc: 0f85f0090000  jne 0x1415cb3c2
1415ca9d2: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415ca9d6: c6454006  mov byte ptr [rbp + 0x40], 6
1415ca9da: 3c06  cmp al, 6
1415ca9dc: 0f842d080000  je 0x1415cb20f
1415ca9e2: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415ca9e8: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415ca9ed: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415ca9f2: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415ca9f6: 0f1107  movups xmmword ptr [rdi], xmm0
1415ca9f9: 4c89e1  mov rcx, r12
1415ca9fc: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415caa03: e89825fdff  call 0x14159cfa0
1415caa08: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415caa0f: 0f851b050000  jne 0x1415caf30
1415caa15: 8b85d4000000  mov eax, dword ptr [rbp + 0xd4]
1415caa1b: 8945f4  mov dword ptr [rbp - 0xc], eax
1415caa1e: c7457c01000000  mov dword ptr [rbp + 0x7c], 1
1415caa25: e9c6fcffff  jmp 0x1415ca6f0
1415caa2a: 83bd8800000000  cmp dword ptr [rbp + 0x88], 0
1415caa31: 0f8507090000  jne 0x1415cb33e
1415caa37: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415caa3b: c6454006  mov byte ptr [rbp + 0x40], 6
1415caa3f: 3c06  cmp al, 6
1415caa41: 0f8460070000  je 0x1415cb1a7
1415caa47: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415caa4d: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415caa52: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415caa57: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415caa5b: 0f1107  movups xmmword ptr [rdi], xmm0
1415caa5e: 4c89e1  mov rcx, r12
1415caa61: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415caa68: e83325fdff  call 0x14159cfa0
1415caa6d: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415caa74: 0f85b6040000  jne 0x1415caf30
1415caa7a: 8b85d4000000  mov eax, dword ptr [rbp + 0xd4]
1415caa80: 8945ec  mov dword ptr [rbp - 0x14], eax
1415caa83: c7858800000001000000  mov dword ptr [rbp + 0x88], 1
1415caa8d: e95efcffff  jmp 0x1415ca6f0
1415caa92: 83bd9800000000  cmp dword ptr [rbp + 0x98], 0
1415caa99: 0f850d090000  jne 0x1415cb3ac
1415caa9f: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415caaa3: c6454006  mov byte ptr [rbp + 0x40], 6
1415caaa7: 3c06  cmp al, 6
1415caaa9: 0f8474060000  je 0x1415cb123
1415caaaf: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415caab5: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415caaba: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415caabf: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415caac3: 0f1107  movups xmmword ptr [rdi], xmm0
1415caac6: 4c89e1  mov rcx, r12
1415caac9: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415caad0: e89bf8fcff  call 0x14159a370
1415caad5: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415caadc: 0f854e040000  jne 0x1415caf30
1415caae2: f30f10bdd4000000  movss xmm7, dword ptr [rbp + 0xd4]
1415caaea: c7859800000001000000  mov dword ptr [rbp + 0x98], 1
1415caaf4: e9f7fbffff  jmp 0x1415ca6f0
1415caaf9: f6457801  test byte ptr [rbp + 0x78], 1
1415caafd: 0f857d080000  jne 0x1415cb380
1415cab03: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415cab07: c6454006  mov byte ptr [rbp + 0x40], 6
1415cab0b: 3c06  cmp al, 6
1415cab0d: 0f84e9060000  je 0x1415cb1fc
1415cab13: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415cab19: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415cab1e: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415cab23: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415cab27: 0f1107  movups xmmword ptr [rdi], xmm0
1415cab2a: 488d054f435001  lea rax, [rip + 0x150434f]
1415cab31: 4889442420  mov qword ptr [rsp + 0x20], rax
1415cab36: 48c744242801000000  mov qword ptr [rsp + 0x28], 1
1415cab3f: 41b90d000000  mov r9d, 0xd
1415cab45: 4c89e1  mov rcx, r12
1415cab48: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415cab4f: 4c8d0518435001  lea r8, [rip + 0x1504318]
1415cab56: e8e581fdff  call 0x1415a2d40
1415cab5b: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415cab62: 0f85c8030000  jne 0x1415caf30
1415cab68: 0fb685d1000000  movzx eax, byte ptr [rbp + 0xd1]
1415cab6f: 8885f7000000  mov byte ptr [rbp + 0xf7], al
1415cab75: b001  mov al, 1
1415cab77: 894578  mov dword ptr [rbp + 0x78], eax
1415cab7a: e971fbffff  jmp 0x1415ca6f0
1415cab7f: 83bd8400000000  cmp dword ptr [rbp + 0x84], 0
1415cab86: 0f8534070000  jne 0x1415cb2c0
1415cab8c: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415cab90: c6454006  mov byte ptr [rbp + 0x40], 6
1415cab94: 3c06  cmp al, 6
1415cab96: 0f84f5050000  je 0x1415cb191
1415cab9c: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415caba2: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415caba7: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415cabac: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415cabb0: 0f1107  movups xmmword ptr [rdi], xmm0
1415cabb3: 4c89e1  mov rcx, r12
1415cabb6: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415cabbd: e8de23fdff  call 0x14159cfa0
1415cabc2: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415cabc9: 0f8561030000  jne 0x1415caf30
1415cabcf: 8b85d4000000  mov eax, dword ptr [rbp + 0xd4]
1415cabd5: 8945e8  mov dword ptr [rbp - 0x18], eax
1415cabd8: c7858400000001000000  mov dword ptr [rbp + 0x84], 1
1415cabe2: e909fbffff  jmp 0x1415ca6f0
1415cabe7: 83bd8000000000  cmp dword ptr [rbp + 0x80], 0
1415cabee: 0f8534070000  jne 0x1415cb328
1415cabf4: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415cabf8: c6454006  mov byte ptr [rbp + 0x40], 6
1415cabfc: 3c06  cmp al, 6
1415cabfe: 0f84b9050000  je 0x1415cb1bd
1415cac04: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415cac0a: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415cac0f: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415cac14: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415cac18: 0f1107  movups xmmword ptr [rdi], xmm0
1415cac1b: 4c89e1  mov rcx, r12
1415cac1e: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415cac25: e87623fdff  call 0x14159cfa0
1415cac2a: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415cac31: 0f85f9020000  jne 0x1415caf30
1415cac37: 8b85d4000000  mov eax, dword ptr [rbp + 0xd4]
1415cac3d: 8945e4  mov dword ptr [rbp - 0x1c], eax
1415cac40: c7858000000001000000  mov dword ptr [rbp + 0x80], 1
1415cac4a: e9a1faffff  jmp 0x1415ca6f0
1415cac4f: 80bdff00000002  cmp byte ptr [rbp + 0xff], 2
1415cac56: 0f857c070000  jne 0x1415cb3d8
1415cac5c: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415cac60: c6454006  mov byte ptr [rbp + 0x40], 6
1415cac64: 3c06  cmp al, 6
1415cac66: 0f84dc050000  je 0x1415cb248
1415cac6c: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415cac72: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415cac77: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415cac7c: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415cac80: 0f1107  movups xmmword ptr [rdi], xmm0
1415cac83: 4c89e1  mov rcx, r12
1415cac86: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415cac8d: e85e28fdff  call 0x14159d4f0
1415cac92: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415cac99: 0f8591020000  jne 0x1415caf30
1415cac9f: 0fb685d1000000  movzx eax, byte ptr [rbp + 0xd1]
1415caca6: 8885ff000000  mov byte ptr [rbp + 0xff], al
1415cacac: e93ffaffff  jmp 0x1415ca6f0
1415cacb1: 80bdfb00000040  cmp byte ptr [rbp + 0xfb], 0x40
1415cacb8: 0f8546070000  jne 0x1415cb404
1415cacbe: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415cacc2: c6454006  mov byte ptr [rbp + 0x40], 6
1415cacc6: 3c06  cmp al, 6
1415cacc8: 0f841b050000  je 0x1415cb1e9
1415cacce: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415cacd4: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415cacd9: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415cacde: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415cace2: 0f1107  movups xmmword ptr [rdi], xmm0
1415cace5: 4c89e1  mov rcx, r12
1415cace8: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415cacef: e8dcd347ff  call 0x140a480d0
1415cacf4: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415cacfb: 0f852f020000  jne 0x1415caf30
1415cad01: 0fb685d1000000  movzx eax, byte ptr [rbp + 0xd1]
1415cad08: 8885fb000000  mov byte ptr [rbp + 0xfb], al
1415cad0e: e9ddf9ffff  jmp 0x1415ca6f0
1415cad13: f6858c00000001  test byte ptr [rbp + 0x8c], 1
1415cad1a: 0f8534060000  jne 0x1415cb354
1415cad20: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415cad24: c6454006  mov byte ptr [rbp + 0x40], 6
1415cad28: 3c06  cmp al, 6
1415cad2a: 0f84f2040000  je 0x1415cb222
1415cad30: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415cad36: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415cad3b: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415cad40: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415cad44: 0f1107  movups xmmword ptr [rdi], xmm0
1415cad47: 4c89e1  mov rcx, r12
1415cad4a: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415cad51: e82af4fcff  call 0x14159a180
1415cad56: 80bdd000000000  cmp byte ptr [rbp + 0xd0], 0
1415cad5d: 0f85cd010000  jne 0x1415caf30
1415cad63: 0fb685d1000000  movzx eax, byte ptr [rbp + 0xd1]
1415cad6a: 8885f8000000  mov byte ptr [rbp + 0xf8], al
1415cad70: b001  mov al, 1
1415cad72: 89858c000000  mov dword ptr [rbp + 0x8c], eax
1415cad78: e973f9ffff  jmp 0x1415ca6f0
1415cad7d: 80bdfd00000004  cmp byte ptr [rbp + 0xfd], 4
1415cad84: 0f8564060000  jne 0x1415cb3ee
1415cad8a: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415cad8e: c6454006  mov byte ptr [rbp + 0x40], 6
1415cad92: 3c06  cmp al, 6
1415cad94: 0f849f030000  je 0x1415cb139
1415cad9a: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415cada0: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415cada5: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415cadaa: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415cadae: 0f1107  movups xmmword ptr [rdi], xmm0
1415cadb1: 4c89e1  mov rcx, r12
1415cadb4: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415cadbb: e810e347ff  call 0x140a490d0
1415cadc0: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415cadc7: 0f8563010000  jne 0x1415caf30
1415cadcd: 0fb685d1000000  movzx eax, byte ptr [rbp + 0xd1]
1415cadd4: 8885fd000000  mov byte ptr [rbp + 0xfd], al
1415cadda: e911f9ffff  jmp 0x1415ca6f0
1415caddf: 83bd9000000000  cmp dword ptr [rbp + 0x90], 0
1415cade6: 0f85e7040000  jne 0x1415cb2d3
1415cadec: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415cadf0: c6454006  mov byte ptr [rbp + 0x40], 6
1415cadf4: 3c06  cmp al, 6
1415cadf6: 0f84d7030000  je 0x1415cb1d3
1415cadfc: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415cae02: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415cae07: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415cae0c: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415cae10: 0f1107  movups xmmword ptr [rdi], xmm0
1415cae13: 4c89e1  mov rcx, r12
1415cae16: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415cae1d: e84ef5fcff  call 0x14159a370
1415cae22: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415cae29: 0f8501010000  jne 0x1415caf30
1415cae2f: f3440f1085d4000000  movss xmm8, dword ptr [rbp + 0xd4]
1415cae38: c7859000000001000000  mov dword ptr [rbp + 0x90], 1
1415cae42: e9a9f8ffff  jmp 0x1415ca6f0
1415cae47: 80bdfe00000002  cmp byte ptr [rbp + 0xfe], 2
1415cae4e: 0f85a8040000  jne 0x1415cb2fc
1415cae54: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415cae58: c6454006  mov byte ptr [rbp + 0x40], 6
1415cae5c: 3c06  cmp al, 6
1415cae5e: 0f84d1030000  je 0x1415cb235
1415cae64: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415cae6a: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415cae6f: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415cae74: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415cae78: 0f1107  movups xmmword ptr [rdi], xmm0
1415cae7b: 4c89e1  mov rcx, r12
1415cae7e: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415cae85: e86626fdff  call 0x14159d4f0
1415cae8a: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415cae91: 0f8599000000  jne 0x1415caf30
1415cae97: 0fb685d1000000  movzx eax, byte ptr [rbp + 0xd1]
1415cae9e: 8885fe000000  mov byte ptr [rbp + 0xfe], al
1415caea4: e947f8ffff  jmp 0x1415ca6f0
1415caea9: 83bd9400000000  cmp dword ptr [rbp + 0x94], 0
1415caeb0: 0f85e0040000  jne 0x1415cb396
1415caeb6: 0fb64540  movzx eax, byte ptr [rbp + 0x40]
1415caeba: c6454006  mov byte ptr [rbp + 0x40], 6
1415caebe: 3c06  cmp al, 6
1415caec0: 0f849f020000  je 0x1415cb165
1415caec6: 8885a0000000  mov byte ptr [rbp + 0xa0], al
1415caecc: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415caed1: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415caed6: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415caeda: 0f1107  movups xmmword ptr [rdi], xmm0
1415caedd: 4c89e1  mov rcx, r12
1415caee0: 488d95a0000000  lea rdx, [rbp + 0xa0]
1415caee7: e8b420fdff  call 0x14159cfa0
1415caeec: f685d000000001  test byte ptr [rbp + 0xd0], 1
1415caef3: 753b  jne 0x1415caf30
1415caef5: 8b85d4000000  mov eax, dword ptr [rbp + 0xd4]
1415caefb: 8945f0  mov dword ptr [rbp - 0x10], eax
1415caefe: c7859400000001000000  mov dword ptr [rbp + 0x94], 1
1415caf08: e9e3f7ffff  jmp 0x1415ca6f0
1415caf0d: 0fb6bdfa000000  movzx edi, byte ptr [rbp + 0xfa]
1415caf14: 4080ff05  cmp dil, 5
1415caf18: 7522  jne 0x1415caf3c
1415caf1a: 488d0dfff66a01  lea rcx, [rip + 0x16af6ff]
1415caf21: ba03000000  mov edx, 3
1415caf26: e895ec3201  call 0x1428f9bc0
1415caf2b: e93c030000  jmp 0x1415cb26c
1415caf30: 4c8badd8000000  mov r13, qword ptr [rbp + 0xd8]
1415caf37: e933030000  jmp 0x1415cb26f
1415caf3c: 0fb6b5fb000000  movzx esi, byte ptr [rbp + 0xfb]
1415caf43: 4080fe40  cmp sil, 0x40
1415caf47: 7516  jne 0x1415caf5f
1415caf49: 488d0dd3f66a01  lea rcx, [rip + 0x16af6d3]
1415caf50: ba02000000  mov edx, 2
1415caf55: e866ec3201  call 0x1428f9bc0
1415caf5a: e90d030000  jmp 0x1415cb26c
1415caf5f: f6457801  test byte ptr [rbp + 0x78], 1
1415caf63: 0f8478010000  je 0x1415cb0e1
1415caf69: 31c0  xor eax, eax
1415caf6b: f6858000000001  test byte ptr [rbp + 0x80], 1
1415caf72: 0f4545e4  cmovne eax, dword ptr [rbp - 0x1c]
1415caf76: f6457c01  test byte ptr [rbp + 0x7c], 1
1415caf7a: 0f8477010000  je 0x1415cb0f7
1415caf80: 48837d3800  cmp qword ptr [rbp + 0x38], 0
1415caf85: 0f858f040000  jne 0x1415cb41a
1415caf8b: f30f100571a93501  movss xmm0, dword ptr [rip + 0x135a971]
1415caf93: f6859c00000001  test byte ptr [rbp + 0x9c], 1
1415caf9a: 0fb69dfc000000  movzx ebx, byte ptr [rbp + 0xfc]
1415cafa1: 448b65f0  mov r12d, dword ptr [rbp - 0x10]
1415cafa5: 440fb695fe000000  movzx r10d, byte ptr [rbp + 0xfe]
1415cafad: 440fb69dff000000  movzx r11d, byte ptr [rbp + 0xff]
1415cafb5: 7503  jne 0x1415cafba
1415cafb7: 0f28f0  movaps xmm6, xmm0
1415cafba: f6859800000001  test byte ptr [rbp + 0x98], 1
1415cafc1: 7503  jne 0x1415cafc6
1415cafc3: 0f28f8  movaps xmm7, xmm0
1415cafc6: 4489d2  mov edx, r10d
1415cafc9: 80e201  and dl, 1
1415cafcc: 4531c0  xor r8d, r8d
1415cafcf: f6859400000001  test byte ptr [rbp + 0x94], 1
1415cafd6: 450f44e0  cmove r12d, r8d
1415cafda: 4589d9  mov r9d, r11d
1415cafdd: 4180e101  and r9b, 1
1415cafe1: f6859000000001  test byte ptr [rbp + 0x90], 1
1415cafe8: 7504  jne 0x1415cafee
1415cafea: 450f57c0  xorps xmm8, xmm8
1415cafee: f6858c00000001  test byte ptr [rbp + 0x8c], 1
1415caff5: 0fb68df8000000  movzx ecx, byte ptr [rbp + 0xf8]
1415caffc: 410f44c8  cmove ecx, r8d
1415cb000: 4180fa02  cmp r10b, 2
1415cb004: 0fb6d2  movzx edx, dl
1415cb007: 41ba01000000  mov r10d, 1
1415cb00d: 410f44d2  cmove edx, r10d
1415cb011: 4180fb02  cmp r11b, 2
1415cb015: 450fb6c9  movzx r9d, r9b
1415cb019: 450f44ca  cmove r9d, r10d
1415cb01d: f6858400000001  test byte ptr [rbp + 0x84], 1
1415cb024: 448b75e8  mov r14d, dword ptr [rbp - 0x18]
1415cb028: 450f44f0  cmove r14d, r8d
1415cb02c: 80e301  and bl, 1
1415cb02f: 440fb695fd000000  movzx r10d, byte ptr [rbp + 0xfd]
1415cb037: 4180fa04  cmp r10b, 4
1415cb03b: 450fb6d2  movzx r10d, r10b
1415cb03f: 450f44d0  cmove r10d, r8d
1415cb043: f6858800000001  test byte ptr [rbp + 0x88], 1
1415cb04a: 448b7dec  mov r15d, dword ptr [rbp - 0x14]
1415cb04e: 450f44f8  cmove r15d, r8d
1415cb052: 0f1045ba  movups xmm0, xmmword ptr [rbp - 0x46]
1415cb056: 4c8b9d00010000  mov r11, qword ptr [rbp + 0x100]
1415cb05d: 410f114312  movups xmmword ptr [r11 + 0x12], xmm0
1415cb062: 0f2845b0  movaps xmm0, xmmword ptr [rbp - 0x50]
1415cb066: 410f114308  movups xmmword ptr [r11 + 8], xmm0
1415cb06b: 448b4570  mov r8d, dword ptr [rbp + 0x70]
1415cb06f: 45894323  mov dword ptr [r11 + 0x23], r8d
1415cb073: 440fb64574  movzx r8d, byte ptr [rbp + 0x74]
1415cb078: 45884327  mov byte ptr [r11 + 0x27], r8b
1415cb07c: 4d892b  mov qword ptr [r11], r13
1415cb07f: 41887b22  mov byte ptr [r11 + 0x22], dil
1415cb083: 41894328  mov dword ptr [r11 + 0x28], eax
1415cb087: 4c89d8  mov rax, r11
1415cb08a: 448b45f4  mov r8d, dword ptr [rbp - 0xc]
1415cb08e: 4589432c  mov dword ptr [r11 + 0x2c], r8d
1415cb092: f3450f114330  movss dword ptr [r11 + 0x30], xmm8
1415cb098: 45896334  mov dword ptr [r11 + 0x34], r12d
1415cb09c: f3410f117b38  movss dword ptr [r11 + 0x38], xmm7
1415cb0a2: f3410f11733c  movss dword ptr [r11 + 0x3c], xmm6
1415cb0a8: 45897b40  mov dword ptr [r11 + 0x40], r15d
1415cb0ac: 45897344  mov dword ptr [r11 + 0x44], r14d
1415cb0b0: 440fb685f7000000  movzx r8d, byte ptr [rbp + 0xf7]
1415cb0b8: 45884348  mov byte ptr [r11 + 0x48], r8b
1415cb0bc: 41884b49  mov byte ptr [r11 + 0x49], cl
1415cb0c0: 4188734a  mov byte ptr [r11 + 0x4a], sil
1415cb0c4: 4588534b  mov byte ptr [r11 + 0x4b], r10b
1415cb0c8: 45884b4c  mov byte ptr [r11 + 0x4c], r9b
1415cb0cc: 4188534d  mov byte ptr [r11 + 0x4d], dl
1415cb0d0: 88584e  mov byte ptr [rax + 0x4e], bl
1415cb0d3: 488d4df8  lea rcx, [rbp - 8]
1415cb0d7: e814cd2bff  call 0x140887df0
1415cb0dc: e9a5010000  jmp 0x1415cb286
1415cb0e1: 488d0d3df56a01  lea rcx, [rip + 0x16af53d]
1415cb0e8: ba04000000  mov edx, 4
1415cb0ed: e8ceea3201  call 0x1428f9bc0
1415cb0f2: e975010000  jmp 0x1415cb26c
1415cb0f7: 488d0d33f56a01  lea rcx, [rip + 0x16af533]
1415cb0fe: ba04000000  mov edx, 4
1415cb103: e8b8ea3201  call 0x1428f9bc0
1415cb108: e95f010000  jmp 0x1415cb26c
1415cb10d: 488d0d54f16a01  lea rcx, [rip + 0x16af154]
1415cb114: ba10000000  mov edx, 0x10
1415cb119: e8e2ee3201  call 0x1428fa000
1415cb11e: e949010000  jmp 0x1415cb26c
1415cb123: 488d0d3ef16a01  lea rcx, [rip + 0x16af13e]
1415cb12a: ba10000000  mov edx, 0x10
1415cb12f: e8ccee3201  call 0x1428fa000
1415cb134: e933010000  jmp 0x1415cb26c
1415cb139: 488d0d28f16a01  lea rcx, [rip + 0x16af128]
1415cb140: ba10000000  mov edx, 0x10
1415cb145: e8b6ee3201  call 0x1428fa000
1415cb14a: e91d010000  jmp 0x1415cb26c
1415cb14f: 488d0d12f16a01  lea rcx, [rip + 0x16af112]
1415cb156: ba10000000  mov edx, 0x10
1415cb15b: e8a0ee3201  call 0x1428fa000
1415cb160: e907010000  jmp 0x1415cb26c
1415cb165: 488d0dfcf06a01  lea rcx, [rip + 0x16af0fc]
1415cb16c: ba10000000  mov edx, 0x10
1415cb171: e88aee3201  call 0x1428fa000
1415cb176: e9f1000000  jmp 0x1415cb26c
1415cb17b: 488d0de6f06a01  lea rcx, [rip + 0x16af0e6]
1415cb182: ba10000000  mov edx, 0x10
1415cb187: e874ee3201  call 0x1428fa000
1415cb18c: e9db000000  jmp 0x1415cb26c
1415cb191: 488d0dd0f06a01  lea rcx, [rip + 0x16af0d0]
1415cb198: ba10000000  mov edx, 0x10
1415cb19d: e85eee3201  call 0x1428fa000
1415cb1a2: e9c5000000  jmp 0x1415cb26c
1415cb1a7: 488d0dbaf06a01  lea rcx, [rip + 0x16af0ba]
1415cb1ae: ba10000000  mov edx, 0x10
1415cb1b3: e848ee3201  call 0x1428fa000
1415cb1b8: e9af000000  jmp 0x1415cb26c
1415cb1bd: 488d0da4f06a01  lea rcx, [rip + 0x16af0a4]
1415cb1c4: ba10000000  mov edx, 0x10
1415cb1c9: e832ee3201  call 0x1428fa000
1415cb1ce: e999000000  jmp 0x1415cb26c
1415cb1d3: 488d0d8ef06a01  lea rcx, [rip + 0x16af08e]
1415cb1da: ba10000000  mov edx, 0x10
1415cb1df: e81cee3201  call 0x1428fa000
1415cb1e4: e983000000  jmp 0x1415cb26c
1415cb1e9: 488d0d78f06a01  lea rcx, [rip + 0x16af078]
1415cb1f0: ba10000000  mov edx, 0x10
1415cb1f5: e806ee3201  call 0x1428fa000
1415cb1fa: eb70  jmp 0x1415cb26c
1415cb1fc: 488d0d65f06a01  lea rcx, [rip + 0x16af065]
1415cb203: ba10000000  mov edx, 0x10
1415cb208: e8f3ed3201  call 0x1428fa000
1415cb20d: eb5d  jmp 0x1415cb26c
1415cb20f: 488d0d52f06a01  lea rcx, [rip + 0x16af052]
1415cb216: ba10000000  mov edx, 0x10
1415cb21b: e8e0ed3201  call 0x1428fa000
1415cb220: eb4a  jmp 0x1415cb26c
1415cb222: 488d0d3ff06a01  lea rcx, [rip + 0x16af03f]
1415cb229: ba10000000  mov edx, 0x10
1415cb22e: e8cded3201  call 0x1428fa000
1415cb233: eb37  jmp 0x1415cb26c
1415cb235: 488d0d2cf06a01  lea rcx, [rip + 0x16af02c]
1415cb23c: ba10000000  mov edx, 0x10
1415cb241: e8baed3201  call 0x1428fa000
1415cb246: eb24  jmp 0x1415cb26c
1415cb248: 488d0d19f06a01  lea rcx, [rip + 0x16af019]
1415cb24f: ba10000000  mov edx, 0x10
1415cb254: e8a7ed3201  call 0x1428fa000
1415cb259: eb11  jmp 0x1415cb26c
1415cb25b: 488d0d06f06a01  lea rcx, [rip + 0x16af006]
1415cb262: ba10000000  mov edx, 0x10
1415cb267: e894ed3201  call 0x1428fa000
1415cb26c: 4989c5  mov r13, rax
1415cb26f: 488b8500010000  mov rax, qword ptr [rbp + 0x100]
1415cb276: 4c8928  mov qword ptr [rax], r13
1415cb279: c6404e02  mov byte ptr [rax + 0x4e], 2
1415cb27d: 488d4df8  lea rcx, [rbp - 8]
1415cb281: e86acb2bff  call 0x140887df0
1415cb286: 488d4d40  lea rcx, [rbp + 0x40]
1415cb28a: e8a1312cff  call 0x14088e430
1415cb28f: 488b8500010000  mov rax, qword ptr [rbp + 0x100]
1415cb296: 0f28b510010000  movaps xmm6, xmmword ptr [rbp + 0x110]
1415cb29d: 0f28bd20010000  movaps xmm7, xmmword ptr [rbp + 0x120]
1415cb2a4: 440f288530010000  movaps xmm8, xmmword ptr [rbp + 0x130]
1415cb2ac: 4881c4c8010000  add rsp, 0x1c8
1415cb2b3: 5b  pop rbx
1415cb2b4: 5f  pop rdi
1415cb2b5: 5e  pop rsi
1415cb2b6: 415c  pop r12
1415cb2b8: 415d  pop r13
1415cb2ba: 415e  pop r14
1415cb2bc: 415f  pop r15
1415cb2be: 5d  pop rbp
1415cb2bf: c3  ret 
1415cb2c0: 488d0d1bf26a01  lea rcx, [rip + 0x16af21b]
1415cb2c7: ba0b000000  mov edx, 0xb
1415cb2cc: e8dfea3201  call 0x1428f9db0
1415cb2d1: eb99  jmp 0x1415cb26c
1415cb2d3: 488d0d5ef36a01  lea rcx, [rip + 0x16af35e]
1415cb2da: ba08000000  mov edx, 8
1415cb2df: e8ccea3201  call 0x1428f9db0
1415cb2e4: eb86  jmp 0x1415cb26c
1415cb2e6: 488d0d33f36a01  lea rcx, [rip + 0x16af333]
1415cb2ed: ba03000000  mov edx, 3
1415cb2f2: e8b9ea3201  call 0x1428f9db0
1415cb2f7: e970ffffff  jmp 0x1415cb26c
1415cb2fc: 488d0d5ff36a01  lea rcx, [rip + 0x16af35f]
1415cb303: ba0d000000  mov edx, 0xd
1415cb308: e8a3ea3201  call 0x1428f9db0
1415cb30d: e95affffff  jmp 0x1415cb26c
1415cb312: 488d0d68f36a01  lea rcx, [rip + 0x16af368]
1415cb319: ba0f000000  mov edx, 0xf
1415cb31e: e88dea3201  call 0x1428f9db0
1415cb323: e944ffffff  jmp 0x1415cb26c
1415cb328: 488d0dfaf26a01  lea rcx, [rip + 0x16af2fa]
1415cb32f: ba08000000  mov edx, 8
1415cb334: e877ea3201  call 0x1428f9db0
1415cb339: e92effffff  jmp 0x1415cb26c
1415cb33e: 488d0d87f16a01  lea rcx, [rip + 0x16af187]
1415cb345: ba0c000000  mov edx, 0xc
1415cb34a: e861ea3201  call 0x1428f9db0
1415cb34f: e918ffffff  jmp 0x1415cb26c
1415cb354: 488d0ddaf26a01  lea rcx, [rip + 0x16af2da]
1415cb35b: ba03000000  mov edx, 3
1415cb360: e84bea3201  call 0x1428f9db0
1415cb365: e902ffffff  jmp 0x1415cb26c
1415cb36a: 488d0dfef26a01  lea rcx, [rip + 0x16af2fe]
1415cb371: ba0b000000  mov edx, 0xb
1415cb376: e835ea3201  call 0x1428f9db0
1415cb37b: e9ecfeffff  jmp 0x1415cb26c
1415cb380: 488d0d9ef26a01  lea rcx, [rip + 0x16af29e]
1415cb387: ba04000000  mov edx, 4
1415cb38c: e81fea3201  call 0x1428f9db0
1415cb391: e9d6feffff  jmp 0x1415cb26c
1415cb396: 488d0db3f26a01  lea rcx, [rip + 0x16af2b3]
1415cb39d: ba12000000  mov edx, 0x12
1415cb3a2: e809ea3201  call 0x1428f9db0
1415cb3a7: e9c0feffff  jmp 0x1415cb26c
1415cb3ac: 488d0dbbf06a01  lea rcx, [rip + 0x16af0bb]
1415cb3b3: ba12000000  mov edx, 0x12
1415cb3b8: e8f3e93201  call 0x1428f9db0
1415cb3bd: e9aafeffff  jmp 0x1415cb26c
1415cb3c2: 488d0d68f26a01  lea rcx, [rip + 0x16af268]
1415cb3c9: ba04000000  mov edx, 4
1415cb3ce: e8dde93201  call 0x1428f9db0
1415cb3d3: e994feffff  jmp 0x1415cb26c
1415cb3d8: 488d0d61f26a01  lea rcx, [rip + 0x16af261]
1415cb3df: ba10000000  mov edx, 0x10
1415cb3e4: e8c7e93201  call 0x1428f9db0
1415cb3e9: e97efeffff  jmp 0x1415cb26c
1415cb3ee: 488d0d85f26a01  lea rcx, [rip + 0x16af285]
1415cb3f5: ba07000000  mov edx, 7
1415cb3fa: e8b1e93201  call 0x1428f9db0
1415cb3ff: e968feffff  jmp 0x1415cb26c
1415cb404: 488d0d18f26a01  lea rcx, [rip + 0x16af218]
1415cb40b: ba02000000  mov edx, 2
1415cb410: e89be93201  call 0x1428f9db0
1415cb415: e952feffff  jmp 0x1415cb26c
1415cb41a: 488d1577ec6a01  lea rdx, [rip + 0x16aec77]
1415cb421: 4c8d0538ec6a01  lea r8, [rip + 0x16aec38]
1415cb428: 488b4dd0  mov rcx, qword ptr [rbp - 0x30]
1415cb42c: e8efe83201  call 0x1428f9d20
1415cb431: 488b8d00010000  mov rcx, qword ptr [rbp + 0x100]
1415cb438: 488901  mov qword ptr [rcx], rax
1415cb43b: 4889c8  mov rax, rcx
1415cb43e: b302  mov bl, 2
1415cb440: e98bfcffff  jmp 0x1415cb0d0
1415cb445: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415cb450: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415cb455: 55  push rbp
1415cb456: 4157  push r15
1415cb458: 4156  push r14
1415cb45a: 4155  push r13
1415cb45c: 4154  push r12
1415cb45e: 56  push rsi
1415cb45f: 57  push rdi
1415cb460: 53  push rbx
1415cb461: 4883ec68  sub rsp, 0x68
1415cb465: 488daa80000000  lea rbp, [rdx + 0x80]
1415cb46c: 440f29442430  movaps xmmword ptr [rsp + 0x30], xmm8
1415cb472: 0f297c2440  movaps xmmword ptr [rsp + 0x40], xmm7
1415cb477: 0f29742450  movaps xmmword ptr [rsp + 0x50], xmm6
1415cb47c: 488d4d40  lea rcx, [rbp + 0x40]
1415cb480: e8ab2f2cff  call 0x14088e430
1415cb485: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415cb48a: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415cb48f: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415cb495: 4883c468  add rsp, 0x68
1415cb499: 5b  pop rbx
1415cb49a: 5f  pop rdi
1415cb49b: 5e  pop rsi
1415cb49c: 415c  pop r12
1415cb49e: 415d  pop r13
1415cb4a0: 415e  pop r14
1415cb4a2: 415f  pop r15
1415cb4a4: 5d  pop rbp
1415cb4a5: c3  ret 
1415cb4a6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415cb4b0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415cb4b5: 55  push rbp
1415cb4b6: 4157  push r15
1415cb4b8: 4156  push r14
1415cb4ba: 4155  push r13
1415cb4bc: 4154  push r12
1415cb4be: 56  push rsi
1415cb4bf: 57  push rdi
1415cb4c0: 53  push rbx
1415cb4c1: 4883ec68  sub rsp, 0x68
1415cb4c5: 488daa80000000  lea rbp, [rdx + 0x80]
1415cb4cc: 440f29442430  movaps xmmword ptr [rsp + 0x30], xmm8
1415cb4d2: 0f297c2440  movaps xmmword ptr [rsp + 0x40], xmm7
1415cb4d7: 0f29742450  movaps xmmword ptr [rsp + 0x50], xmm6
1415cb4dc: 488b5548  mov rdx, qword ptr [rbp + 0x48]
1415cb4e0: 4885d2  test rdx, rdx
1415cb4e3: 7413  je 0x1415cb4f8
1415cb4e5: 48c1e205  shl rdx, 5
1415cb4e9: 41b808000000  mov r8d, 8
1415cb4ef: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415cb4f3: e8288704ff  call 0x140613c20
1415cb4f8: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415cb4fd: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415cb502: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415cb508: 4883c468  add rsp, 0x68
1415cb50c: 5b  pop rbx
1415cb50d: 5f  pop rdi
1415cb50e: 5e  pop rsi
1415cb50f: 415c  pop r12
1415cb511: 415d  pop r13
1415cb513: 415e  pop r14
1415cb515: 415f  pop r15
1415cb517: 5d  pop rbp
1415cb518: c3  ret 
1415cb519: 0f1f8000000000  nop dword ptr [rax]
1415cb520: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415cb525: 55  push rbp
1415cb526: 4157  push r15
1415cb528: 4156  push r14
1415cb52a: 4155  push r13
1415cb52c: 4154  push r12
1415cb52e: 56  push rsi
1415cb52f: 57  push rdi
1415cb530: 53  push rbx
1415cb531: 4883ec68  sub rsp, 0x68
1415cb535: 488daa80000000  lea rbp, [rdx + 0x80]
1415cb53c: 440f29442430  movaps xmmword ptr [rsp + 0x30], xmm8
1415cb542: 0f297c2440  movaps xmmword ptr [rsp + 0x40], xmm7
1415cb547: 0f29742450  movaps xmmword ptr [rsp + 0x50], xmm6
1415cb54c: 488d4d40  lea rcx, [rbp + 0x40]
1415cb550: e8db2e2cff  call 0x14088e430
1415cb555: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415cb55a: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415cb55f: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415cb565: 4883c468  add rsp, 0x68
1415cb569: 5b  pop rbx
1415cb56a: 5f  pop rdi
1415cb56b: 5e  pop rsi
1415cb56c: 415c  pop r12
1415cb56e: 415d  pop r13
1415cb570: 415e  pop r14
1415cb572: 415f  pop r15
1415cb574: 5d  pop rbp
1415cb575: c3  ret 
1415cb576: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415cb580: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415cb585: 55  push rbp
1415cb586: 4157  push r15
1415cb588: 4156  push r14
1415cb58a: 4155  push r13
1415cb58c: 4154  push r12
1415cb58e: 56  push rsi
1415cb58f: 57  push rdi
1415cb590: 53  push rbx
1415cb591: 4883ec68  sub rsp, 0x68
1415cb595: 488daa80000000  lea rbp, [rdx + 0x80]
1415cb59c: 440f29442430  movaps xmmword ptr [rsp + 0x30], xmm8
1415cb5a2: 0f297c2440  movaps xmmword ptr [rsp + 0x40], xmm7
1415cb5a7: 0f29742450  movaps xmmword ptr [rsp + 0x50], xmm6
1415cb5ac: 0f2885a0000000  movaps xmm0, xmmword ptr [rbp + 0xa0]
1415cb5b3: 0f288db0000000  movaps xmm1, xmmword ptr [rbp + 0xb0]
1415cb5ba: 488d4540  lea rax, [rbp + 0x40]
1415cb5be: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415cb5c2: 0f1100  movups xmmword ptr [rax], xmm0
1415cb5c5: c685f900000001  mov byte ptr [rbp + 0xf9], 1
1415cb5cc: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415cb5d1: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415cb5d6: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415cb5dc: 4883c468  add rsp, 0x68
1415cb5e0: 5b  pop rbx
1415cb5e1: 5f  pop rdi
1415cb5e2: 5e  pop rsi
1415cb5e3: 415c  pop r12
1415cb5e5: 415d  pop r13
1415cb5e7: 415e  pop r14
1415cb5e9: 415f  pop r15
1415cb5eb: 5d  pop rbp
1415cb5ec: c3  ret 
1415cb5ed: 0f1f00  nop dword ptr [rax]
1415cb5f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415cb5f5: 55  push rbp
1415cb5f6: 4157  push r15
1415cb5f8: 4156  push r14
1415cb5fa: 4155  push r13
1415cb5fc: 4154  push r12
1415cb5fe: 56  push rsi
1415cb5ff: 57  push rdi
1415cb600: 53  push rbx
1415cb601: 4883ec68  sub rsp, 0x68
1415cb605: 488daa80000000  lea rbp, [rdx + 0x80]
1415cb60c: 440f29442430  movaps xmmword ptr [rsp + 0x30], xmm8
1415cb612: 0f297c2440  movaps xmmword ptr [rsp + 0x40], xmm7
1415cb617: 0f29742450  movaps xmmword ptr [rsp + 0x50], xmm6
1415cb61c: 48837d6800  cmp qword ptr [rbp + 0x68], 0
1415cb621: 7413  je 0x1415cb636
1415cb623: 41b801000000  mov r8d, 1
1415cb629: 488b4d60  mov rcx, qword ptr [rbp + 0x60]
1415cb62d: 488b5568  mov rdx, qword ptr [rbp + 0x68]
1415cb631: e8ea8504ff  call 0x140613c20
1415cb636: c685f900000000  mov byte ptr [rbp + 0xf9], 0
1415cb63d: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415cb642: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415cb647: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415cb64d: 4883c468  add rsp, 0x68
1415cb651: 5b  pop rbx
1415cb652: 5f  pop rdi
1415cb653: 5e  pop rsi
1415cb654: 415c  pop r12
1415cb656: 415d  pop r13
1415cb658: 415e  pop r14
1415cb65a: 415f  pop r15
1415cb65c: 5d  pop rbp
1415cb65d: c3  ret 
1415cb65e: 6690  nop 
1415cb660: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415cb665: 55  push rbp
1415cb666: 4157  push r15
1415cb668: 4156  push r14
1415cb66a: 4155  push r13
1415cb66c: 4154  push r12
1415cb66e: 56  push rsi
1415cb66f: 57  push rdi
1415cb670: 53  push rbx
1415cb671: 4883ec68  sub rsp, 0x68
1415cb675: 488daa80000000  lea rbp, [rdx + 0x80]
1415cb67c: 440f29442430  movaps xmmword ptr [rsp + 0x30], xmm8
1415cb682: 0f297c2440  movaps xmmword ptr [rsp + 0x40], xmm7
1415cb687: 0f29742450  movaps xmmword ptr [rsp + 0x50], xmm6
1415cb68c: 80bdf900000000  cmp byte ptr [rbp + 0xf9], 0
1415cb693: 743b  je 0x1415cb6d0
1415cb695: 48837d6800  cmp qword ptr [rbp + 0x68], 0
1415cb69a: 7413  je 0x1415cb6af
1415cb69c: 41b801000000  mov r8d, 1
1415cb6a2: 488b4d60  mov rcx, qword ptr [rbp + 0x60]
1415cb6a6: 488b5568  mov rdx, qword ptr [rbp + 0x68]
1415cb6aa: e8718504ff  call 0x140613c20
1415cb6af: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415cb6b4: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415cb6b9: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415cb6bf: 4883c468  add rsp, 0x68
1415cb6c3: 5b  pop rbx
1415cb6c4: 5f  pop rdi
1415cb6c5: 5e  pop rsi
1415cb6c6: 415c  pop r12
1415cb6c8: 415d  pop r13
1415cb6ca: 415e  pop r14
1415cb6cc: 415f  pop r15
1415cb6ce: 5d  pop rbp
1415cb6cf: c3  ret 
1415cb6d0: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415cb6d5: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415cb6da: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415cb6e0: 4883c468  add rsp, 0x68
1415cb6e4: 5b  pop rbx
1415cb6e5: 5f  pop rdi
1415cb6e6: 5e  pop rsi
1415cb6e7: 415c  pop r12
1415cb6e9: 415d  pop r13
1415cb6eb: 415e  pop r14
1415cb6ed: 415f  pop r15
1415cb6ef: 5d  pop rbp
1415cb6f0: c3  ret 
1415cb6f1: 6666666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415cb700: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415cb705: 55  push rbp
1415cb706: 4157  push r15
1415cb708: 4156  push r14
1415cb70a: 4155  push r13
1415cb70c: 4154  push r12
1415cb70e: 56  push rsi
1415cb70f: 57  push rdi
1415cb710: 53  push rbx
1415cb711: 4883ec68  sub rsp, 0x68
1415cb715: 488daa80000000  lea rbp, [rdx + 0x80]
1415cb71c: 440f29442430  movaps xmmword ptr [rsp + 0x30], xmm8
1415cb722: 0f297c2440  movaps xmmword ptr [rsp + 0x40], xmm7
1415cb727: 0f29742450  movaps xmmword ptr [rsp + 0x50], xmm6
1415cb72c: 488d4df8  lea rcx, [rbp - 8]
1415cb730: e8cbed2bff  call 0x14088a500
1415cb735: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415cb73a: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415cb73f: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415cb745: 4883c468  add rsp, 0x68
1415cb749: 5b  pop rbx
1415cb74a: 5f  pop rdi
1415cb74b: 5e  pop rsi
1415cb74c: 415c  pop r12
1415cb74e: 415d  pop r13
1415cb750: 415e  pop r14
1415cb752: 415f  pop r15
1415cb754: 5d  pop rbp
1415cb755: c3  ret 
