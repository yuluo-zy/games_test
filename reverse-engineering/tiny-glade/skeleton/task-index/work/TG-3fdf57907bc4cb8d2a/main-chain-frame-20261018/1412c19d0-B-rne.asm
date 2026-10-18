00000001412c19d0 55                               push rbp
00000001412c19d1 56                               push rsi
00000001412c19d2 4881ecd8000000                   sub rsp, 0xd8
00000001412c19d9 488dac2480000000                 lea rbp, [rsp + 0x80]
00000001412c19e1 48c74550feffffff                 mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
00000001412c19e9 4889ce                           mov rsi, rcx
00000001412c19ec 488d4da8                         lea rcx, [rbp - 0x58]
00000001412c19f0 4531c0                           xor r8d, r8d
00000001412c19f3 e888ff3400                       call 0x141611980
00000001412c19f8 0fb6050a323702                   movzx eax, byte ptr [rip + 0x237320a]
00000001412c19ff b928000000                       mov ecx, 0x28
00000001412c1a04 ba08000000                       mov edx, 8
00000001412c1a09 e8022235ff                       call 0x140613c10
00000001412c1a0e 4885c0                           test rax, rax
00000001412c1a11 0f84b0000000                     je 0x1412c1ac7
00000001412c1a17 488b4dc8                         mov rcx, qword ptr [rbp - 0x38]
00000001412c1a1b 48894820                         mov qword ptr [rax + 0x20], rcx
00000001412c1a1f 0f1045a8                         movups xmm0, xmmword ptr [rbp - 0x58]
00000001412c1a23 0f104db8                         movups xmm1, xmmword ptr [rbp - 0x48]
00000001412c1a27 0f114810                         movups xmmword ptr [rax + 0x10], xmm1
00000001412c1a2b 0f1100                           movups xmmword ptr [rax], xmm0
00000001412c1a2e 488d0d41b87d01                   lea rcx, [rip + 0x17db841]
00000001412c1a35 48894d00                         mov qword ptr [rbp], rcx
00000001412c1a39 48c745084d000000                 mov qword ptr [rbp + 8], 0x4d
00000001412c1a41 48894510                         mov qword ptr [rbp + 0x10], rax
00000001412c1a45 488d05dcb29601                   lea rax, [rip + 0x196b2dc]
00000001412c1a4c 48894518                         mov qword ptr [rbp + 0x18], rax
00000001412c1a50 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
00000001412c1a58 c6454605                         mov byte ptr [rbp + 0x46], 5
00000001412c1a5c 48894d20                         mov qword ptr [rbp + 0x20], rcx
00000001412c1a60 48c745284d000000                 mov qword ptr [rbp + 0x28], 0x4d
00000001412c1a68 0fb6059a313702                   movzx eax, byte ptr [rip + 0x237319a]
00000001412c1a6f b940000000                       mov ecx, 0x40
00000001412c1a74 ba08000000                       mov edx, 8
00000001412c1a79 e8922135ff                       call 0x140613c10
00000001412c1a7e 4885c0                           test rax, rax
00000001412c1a81 7455                             je 0x1412c1ad8
00000001412c1a83 0f104510                         movups xmm0, xmmword ptr [rbp + 0x10]
00000001412c1a87 0f104d20                         movups xmm1, xmmword ptr [rbp + 0x20]
00000001412c1a8b 0f105530                         movups xmm2, xmmword ptr [rbp + 0x30]
00000001412c1a8f 0f105d40                         movups xmm3, xmmword ptr [rbp + 0x40]
00000001412c1a93 0f115830                         movups xmmword ptr [rax + 0x30], xmm3
00000001412c1a97 0f115020                         movups xmmword ptr [rax + 0x20], xmm2
00000001412c1a9b 0f114810                         movups xmmword ptr [rax + 0x10], xmm1
00000001412c1a9f 0f1100                           movups xmmword ptr [rax], xmm0
00000001412c1aa2 48ba4634c00f72831b82             movabs rdx, 0x821b83720fc03446
00000001412c1aac 4889f1                           mov rcx, rsi
00000001412c1aaf 4989c0                           mov r8, rax
00000001412c1ab2 e839816aff                       call 0x140969bf0
00000001412c1ab7 4883f801                         cmp rax, 1
00000001412c1abb 742c                             je 0x1412c1ae9
00000001412c1abd 4881c4d8000000                   add rsp, 0xd8
00000001412c1ac4 5e                               pop rsi
00000001412c1ac5 5d                               pop rbp
00000001412c1ac6 c3                               ret 
00000001412c1ac7 b908000000                       mov ecx, 8
00000001412c1acc ba28000000                       mov edx, 0x28
00000001412c1ad1 e82a756101                       call 0x1428d9000
00000001412c1ad6 eb0f                             jmp 0x1412c1ae7
00000001412c1ad8 b908000000                       mov ecx, 8
00000001412c1add ba40000000                       mov edx, 0x40
00000001412c1ae2 e819756101                       call 0x1428d9000
00000001412c1ae7 0f0b                             ud2 
00000001412c1ae9 4889d0                           mov rax, rdx
00000001412c1aec 4883c210                         add rdx, 0x10
00000001412c1af0 4889d6                           mov rsi, rdx
00000001412c1af3 488378184d                       cmp qword ptr [rax + 0x18], 0x4d
00000001412c1af8 757c                             jne 0x1412c1b76
00000001412c1afa 488b16                           mov rdx, qword ptr [rsi]
00000001412c1afd 488d0d72b77d01                   lea rcx, [rip + 0x17db772]
00000001412c1b04 41b84d000000                     mov r8d, 0x4d
00000001412c1b0a e8e1206601                       call 0x142923bf0
00000001412c1b0f 85c0                             test eax, eax
00000001412c1b11 7563                             jne 0x1412c1b76
00000001412c1b13 488d0596bc9601                   lea rax, [rip + 0x196bc96]
00000001412c1b1a 488945d0                         mov qword ptr [rbp - 0x30], rax
00000001412c1b1e 488d057b1d27ff                   lea rax, [rip - 0xd8e285]
00000001412c1b25 488945d8                         mov qword ptr [rbp - 0x28], rax
00000001412c1b29 4889e8                           mov rax, rbp
00000001412c1b2c 488945e0                         mov qword ptr [rbp - 0x20], rax
00000001412c1b30 488d055983e4fe                   lea rax, [rip - 0x11b7ca7]
00000001412c1b37 488945e8                         mov qword ptr [rbp - 0x18], rax
00000001412c1b3b 488d05e6ba9601                   lea rax, [rip + 0x196bae6]
00000001412c1b42 48894510                         mov qword ptr [rbp + 0x10], rax
00000001412c1b46 48c7451803000000                 mov qword ptr [rbp + 0x18], 3
00000001412c1b4e 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
00000001412c1b56 488d45d0                         lea rax, [rbp - 0x30]
00000001412c1b5a 48894520                         mov qword ptr [rbp + 0x20], rax
00000001412c1b5e 48c7452802000000                 mov qword ptr [rbp + 0x28], 2
00000001412c1b66 488d15ebba9601                   lea rdx, [rip + 0x196baeb]
00000001412c1b6d 488d4d10                         lea rcx, [rbp + 0x10]
00000001412c1b71 e81a786101                       call 0x1428d9390
00000001412c1b76 488d0533bc9601                   lea rax, [rip + 0x196bc33]
00000001412c1b7d 48894510                         mov qword ptr [rbp + 0x10], rax
00000001412c1b81 488d05181d27ff                   lea rax, [rip - 0xd8e2e8]
00000001412c1b88 48894518                         mov qword ptr [rbp + 0x18], rax
00000001412c1b8c 48897520                         mov qword ptr [rbp + 0x20], rsi
00000001412c1b90 488d05f982e4fe                   lea rax, [rip - 0x11b7d07]
00000001412c1b97 48894528                         mov qword ptr [rbp + 0x28], rax
00000001412c1b9b 4889e9                           mov rcx, rbp
00000001412c1b9e 48894d30                         mov qword ptr [rbp + 0x30], rcx
00000001412c1ba2 48894538                         mov qword ptr [rbp + 0x38], rax
00000001412c1ba6 488d05bbb99601                   lea rax, [rip + 0x196b9bb]
00000001412c1bad 488945d0                         mov qword ptr [rbp - 0x30], rax
00000001412c1bb1 48c745d803000000                 mov qword ptr [rbp - 0x28], 3
00000001412c1bb9 48c745f000000000                 mov qword ptr [rbp - 0x10], 0
00000001412c1bc1 488d4510                         lea rax, [rbp + 0x10]
00000001412c1bc5 488945e0                         mov qword ptr [rbp - 0x20], rax
00000001412c1bc9 48c745e803000000                 mov qword ptr [rbp - 0x18], 3
00000001412c1bd1 488d15f8b99601                   lea rdx, [rip + 0x196b9f8]
00000001412c1bd8 488d4dd0                         lea rcx, [rbp - 0x30]
00000001412c1bdc e8af776101                       call 0x1428d9390
00000001412c1be1 cc                               int3 
00000001412c1be2 66666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
00000001412c1bf0 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001412c1bf5 55                               push rbp
00000001412c1bf6 56                               push rsi
00000001412c1bf7 4883ec28                         sub rsp, 0x28
00000001412c1bfb 488daa80000000                   lea rbp, [rdx + 0x80]
00000001412c1c02 488d4da8                         lea rcx, [rbp - 0x58]
00000001412c1c06 e8a50892ff                       call 0x140be24b0
00000001412c1c0b 90                               nop 
00000001412c1c0c 4883c428                         add rsp, 0x28
00000001412c1c10 5e                               pop rsi
00000001412c1c11 5d                               pop rbp
00000001412c1c12 c3                               ret 
00000001412c1c13 666666662e0f1f840000000000       nop word ptr cs:[rax + rax]
00000001412c1c20 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001412c1c25 55                               push rbp
00000001412c1c26 56                               push rsi
00000001412c1c27 4883ec28                         sub rsp, 0x28
00000001412c1c2b 488daa80000000                   lea rbp, [rdx + 0x80]
00000001412c1c32 488d4d10                         lea rcx, [rbp + 0x10]
00000001412c1c36 e875e8dffe                       call 0x1400c04b0
00000001412c1c3b 90                               nop 
00000001412c1c3c 4883c428                         add rsp, 0x28
00000001412c1c40 5e                               pop rsi
00000001412c1c41 5d                               pop rbp
00000001412c1c42 c3                               ret 
