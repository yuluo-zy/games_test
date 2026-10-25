00000001412c62b0 55                               push rbp
00000001412c62b1 56                               push rsi
00000001412c62b2 4881ecd8000000                   sub rsp, 0xd8
00000001412c62b9 488dac2480000000                 lea rbp, [rsp + 0x80]
00000001412c62c1 48c74550feffffff                 mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
00000001412c62c9 4889ce                           mov rsi, rcx
00000001412c62cc 488d4da8                         lea rcx, [rbp - 0x58]
00000001412c62d0 4531c0                           xor r8d, r8d
00000001412c62d3 e828a33400                       call 0x141610600
00000001412c62d8 0fb6052ae93602                   movzx eax, byte ptr [rip + 0x236e92a]
00000001412c62df b928000000                       mov ecx, 0x28
00000001412c62e4 ba08000000                       mov edx, 8
00000001412c62e9 e822d934ff                       call 0x140613c10
00000001412c62ee 4885c0                           test rax, rax
00000001412c62f1 0f84b0000000                     je 0x1412c63a7
00000001412c62f7 488b4dc8                         mov rcx, qword ptr [rbp - 0x38]
00000001412c62fb 48894820                         mov qword ptr [rax + 0x20], rcx
00000001412c62ff 0f1045a8                         movups xmm0, xmmword ptr [rbp - 0x58]
00000001412c6303 0f104db8                         movups xmm1, xmmword ptr [rbp - 0x48]
00000001412c6307 0f114810                         movups xmmword ptr [rax + 0x10], xmm1
00000001412c630b 0f1100                           movups xmmword ptr [rax], xmm0
00000001412c630e 488d0d147f7d01                   lea rcx, [rip + 0x17d7f14]
00000001412c6315 48894d00                         mov qword ptr [rbp], rcx
00000001412c6319 48c7450857000000                 mov qword ptr [rbp + 8], 0x57
00000001412c6321 48894510                         mov qword ptr [rbp + 0x10], rax
00000001412c6325 488d0554709601                   lea rax, [rip + 0x1967054]
00000001412c632c 48894518                         mov qword ptr [rbp + 0x18], rax
00000001412c6330 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
00000001412c6338 c6454605                         mov byte ptr [rbp + 0x46], 5
00000001412c633c 48894d20                         mov qword ptr [rbp + 0x20], rcx
00000001412c6340 48c7452857000000                 mov qword ptr [rbp + 0x28], 0x57
00000001412c6348 0fb605bae83602                   movzx eax, byte ptr [rip + 0x236e8ba]
00000001412c634f b940000000                       mov ecx, 0x40
00000001412c6354 ba08000000                       mov edx, 8
00000001412c6359 e8b2d834ff                       call 0x140613c10
00000001412c635e 4885c0                           test rax, rax
00000001412c6361 7455                             je 0x1412c63b8
00000001412c6363 0f104510                         movups xmm0, xmmword ptr [rbp + 0x10]
00000001412c6367 0f104d20                         movups xmm1, xmmword ptr [rbp + 0x20]
00000001412c636b 0f105530                         movups xmm2, xmmword ptr [rbp + 0x30]
00000001412c636f 0f105d40                         movups xmm3, xmmword ptr [rbp + 0x40]
00000001412c6373 0f115830                         movups xmmword ptr [rax + 0x30], xmm3
00000001412c6377 0f115020                         movups xmmword ptr [rax + 0x20], xmm2
00000001412c637b 0f114810                         movups xmmword ptr [rax + 0x10], xmm1
00000001412c637f 0f1100                           movups xmmword ptr [rax], xmm0
00000001412c6382 48ba31c97bab185b9e05             movabs rdx, 0x59e5b18ab7bc931
00000001412c638c 4889f1                           mov rcx, rsi
00000001412c638f 4989c0                           mov r8, rax
00000001412c6392 e859386aff                       call 0x140969bf0
00000001412c6397 4883f801                         cmp rax, 1
00000001412c639b 742c                             je 0x1412c63c9
00000001412c639d 4881c4d8000000                   add rsp, 0xd8
00000001412c63a4 5e                               pop rsi
00000001412c63a5 5d                               pop rbp
00000001412c63a6 c3                               ret 
00000001412c63a7 b908000000                       mov ecx, 8
00000001412c63ac ba28000000                       mov edx, 0x28
00000001412c63b1 e84a2c6101                       call 0x1428d9000
00000001412c63b6 eb0f                             jmp 0x1412c63c7
00000001412c63b8 b908000000                       mov ecx, 8
00000001412c63bd ba40000000                       mov edx, 0x40
00000001412c63c2 e8392c6101                       call 0x1428d9000
00000001412c63c7 0f0b                             ud2 
00000001412c63c9 4889d0                           mov rax, rdx
00000001412c63cc 4883c210                         add rdx, 0x10
00000001412c63d0 4889d6                           mov rsi, rdx
00000001412c63d3 4883781857                       cmp qword ptr [rax + 0x18], 0x57
00000001412c63d8 757c                             jne 0x1412c6456
00000001412c63da 488b16                           mov rdx, qword ptr [rsi]
00000001412c63dd 488d0d457e7d01                   lea rcx, [rip + 0x17d7e45]
00000001412c63e4 41b857000000                     mov r8d, 0x57
00000001412c63ea e801d86501                       call 0x142923bf0
00000001412c63ef 85c0                             test eax, eax
00000001412c63f1 7563                             jne 0x1412c6456
00000001412c63f3 488d05de7c9601                   lea rax, [rip + 0x1967cde]
00000001412c63fa 488945d0                         mov qword ptr [rbp - 0x30], rax
00000001412c63fe 488d059bd426ff                   lea rax, [rip - 0xd92b65]
00000001412c6405 488945d8                         mov qword ptr [rbp - 0x28], rax
00000001412c6409 4889e8                           mov rax, rbp
00000001412c640c 488945e0                         mov qword ptr [rbp - 0x20], rax
00000001412c6410 488d05793ae4fe                   lea rax, [rip - 0x11bc587]
00000001412c6417 488945e8                         mov qword ptr [rbp - 0x18], rax
00000001412c641b 488d0506729601                   lea rax, [rip + 0x1967206]
00000001412c6422 48894510                         mov qword ptr [rbp + 0x10], rax
00000001412c6426 48c7451803000000                 mov qword ptr [rbp + 0x18], 3
00000001412c642e 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
00000001412c6436 488d45d0                         lea rax, [rbp - 0x30]
00000001412c643a 48894520                         mov qword ptr [rbp + 0x20], rax
00000001412c643e 48c7452802000000                 mov qword ptr [rbp + 0x28], 2
00000001412c6446 488d150b729601                   lea rdx, [rip + 0x196720b]
00000001412c644d 488d4d10                         lea rcx, [rbp + 0x10]
00000001412c6451 e83a2f6101                       call 0x1428d9390
00000001412c6456 488d057b7c9601                   lea rax, [rip + 0x1967c7b]
00000001412c645d 48894510                         mov qword ptr [rbp + 0x10], rax
00000001412c6461 488d0538d426ff                   lea rax, [rip - 0xd92bc8]
00000001412c6468 48894518                         mov qword ptr [rbp + 0x18], rax
00000001412c646c 48897520                         mov qword ptr [rbp + 0x20], rsi
00000001412c6470 488d05193ae4fe                   lea rax, [rip - 0x11bc5e7]
00000001412c6477 48894528                         mov qword ptr [rbp + 0x28], rax
00000001412c647b 4889e9                           mov rcx, rbp
00000001412c647e 48894d30                         mov qword ptr [rbp + 0x30], rcx
00000001412c6482 48894538                         mov qword ptr [rbp + 0x38], rax
00000001412c6486 488d05db709601                   lea rax, [rip + 0x19670db]
00000001412c648d 488945d0                         mov qword ptr [rbp - 0x30], rax
00000001412c6491 48c745d803000000                 mov qword ptr [rbp - 0x28], 3
00000001412c6499 48c745f000000000                 mov qword ptr [rbp - 0x10], 0
00000001412c64a1 488d4510                         lea rax, [rbp + 0x10]
00000001412c64a5 488945e0                         mov qword ptr [rbp - 0x20], rax
00000001412c64a9 48c745e803000000                 mov qword ptr [rbp - 0x18], 3
00000001412c64b1 488d1518719601                   lea rdx, [rip + 0x1967118]
00000001412c64b8 488d4dd0                         lea rcx, [rbp - 0x30]
00000001412c64bc e8cf2e6101                       call 0x1428d9390
00000001412c64c1 cc                               int3 
00000001412c64c2 66666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
00000001412c64d0 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001412c64d5 55                               push rbp
00000001412c64d6 56                               push rsi
00000001412c64d7 4883ec28                         sub rsp, 0x28
00000001412c64db 488daa80000000                   lea rbp, [rdx + 0x80]
00000001412c64e2 488d4da8                         lea rcx, [rbp - 0x58]
00000001412c64e6 e8c5bf91ff                       call 0x140be24b0
00000001412c64eb 90                               nop 
00000001412c64ec 4883c428                         add rsp, 0x28
00000001412c64f0 5e                               pop rsi
00000001412c64f1 5d                               pop rbp
00000001412c64f2 c3                               ret 
00000001412c64f3 666666662e0f1f840000000000       nop word ptr cs:[rax + rax]
00000001412c6500 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001412c6505 55                               push rbp
00000001412c6506 56                               push rsi
00000001412c6507 4883ec28                         sub rsp, 0x28
00000001412c650b 488daa80000000                   lea rbp, [rdx + 0x80]
00000001412c6512 488d4d10                         lea rcx, [rbp + 0x10]
00000001412c6516 e8959fdffe                       call 0x1400c04b0
00000001412c651b 90                               nop 
00000001412c651c 4883c428                         add rsp, 0x28
00000001412c6520 5e                               pop rsi
00000001412c6521 5d                               pop rbp
00000001412c6522 c3                               ret 
