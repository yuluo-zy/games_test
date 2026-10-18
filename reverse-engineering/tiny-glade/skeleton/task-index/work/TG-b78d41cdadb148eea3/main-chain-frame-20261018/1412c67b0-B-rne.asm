00000001412c67b0 55                               push rbp
00000001412c67b1 56                               push rsi
00000001412c67b2 4881ecd8000000                   sub rsp, 0xd8
00000001412c67b9 488dac2480000000                 lea rbp, [rsp + 0x80]
00000001412c67c1 48c74550feffffff                 mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
00000001412c67c9 4889ce                           mov rsi, rcx
00000001412c67cc 488d4da8                         lea rcx, [rbp - 0x58]
00000001412c67d0 4531c0                           xor r8d, r8d
00000001412c67d3 e838d93400                       call 0x141614110
00000001412c67d8 0fb6052ae43602                   movzx eax, byte ptr [rip + 0x236e42a]
00000001412c67df b928000000                       mov ecx, 0x28
00000001412c67e4 ba08000000                       mov edx, 8
00000001412c67e9 e822d434ff                       call 0x140613c10
00000001412c67ee 4885c0                           test rax, rax
00000001412c67f1 0f84b0000000                     je 0x1412c68a7
00000001412c67f7 488b4dc8                         mov rcx, qword ptr [rbp - 0x38]
00000001412c67fb 48894820                         mov qword ptr [rax + 0x20], rcx
00000001412c67ff 0f1045a8                         movups xmm0, xmmword ptr [rbp - 0x58]
00000001412c6803 0f104db8                         movups xmm1, xmmword ptr [rbp - 0x48]
00000001412c6807 0f114810                         movups xmmword ptr [rax + 0x10], xmm1
00000001412c680b 0f1100                           movups xmmword ptr [rax], xmm0
00000001412c680e 488d0d88807d01                   lea rcx, [rip + 0x17d8088]
00000001412c6815 48894d00                         mov qword ptr [rbp], rcx
00000001412c6819 48c7450854000000                 mov qword ptr [rbp + 8], 0x54
00000001412c6821 48894510                         mov qword ptr [rbp + 0x10], rax
00000001412c6825 488d05c46b9601                   lea rax, [rip + 0x1966bc4]
00000001412c682c 48894518                         mov qword ptr [rbp + 0x18], rax
00000001412c6830 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
00000001412c6838 c6454605                         mov byte ptr [rbp + 0x46], 5
00000001412c683c 48894d20                         mov qword ptr [rbp + 0x20], rcx
00000001412c6840 48c7452854000000                 mov qword ptr [rbp + 0x28], 0x54
00000001412c6848 0fb605bae33602                   movzx eax, byte ptr [rip + 0x236e3ba]
00000001412c684f b940000000                       mov ecx, 0x40
00000001412c6854 ba08000000                       mov edx, 8
00000001412c6859 e8b2d334ff                       call 0x140613c10
00000001412c685e 4885c0                           test rax, rax
00000001412c6861 7455                             je 0x1412c68b8
00000001412c6863 0f104510                         movups xmm0, xmmword ptr [rbp + 0x10]
00000001412c6867 0f104d20                         movups xmm1, xmmword ptr [rbp + 0x20]
00000001412c686b 0f105530                         movups xmm2, xmmword ptr [rbp + 0x30]
00000001412c686f 0f105d40                         movups xmm3, xmmword ptr [rbp + 0x40]
00000001412c6873 0f115830                         movups xmmword ptr [rax + 0x30], xmm3
00000001412c6877 0f115020                         movups xmmword ptr [rax + 0x20], xmm2
00000001412c687b 0f114810                         movups xmmword ptr [rax + 0x10], xmm1
00000001412c687f 0f1100                           movups xmmword ptr [rax], xmm0
00000001412c6882 48baac5a2530d88623da             movabs rdx, 0xda2386d830255aac
00000001412c688c 4889f1                           mov rcx, rsi
00000001412c688f 4989c0                           mov r8, rax
00000001412c6892 e859336aff                       call 0x140969bf0
00000001412c6897 4883f801                         cmp rax, 1
00000001412c689b 742c                             je 0x1412c68c9
00000001412c689d 4881c4d8000000                   add rsp, 0xd8
00000001412c68a4 5e                               pop rsi
00000001412c68a5 5d                               pop rbp
00000001412c68a6 c3                               ret 
00000001412c68a7 b908000000                       mov ecx, 8
00000001412c68ac ba28000000                       mov edx, 0x28
00000001412c68b1 e84a276101                       call 0x1428d9000
00000001412c68b6 eb0f                             jmp 0x1412c68c7
00000001412c68b8 b908000000                       mov ecx, 8
00000001412c68bd ba40000000                       mov edx, 0x40
00000001412c68c2 e839276101                       call 0x1428d9000
00000001412c68c7 0f0b                             ud2 
00000001412c68c9 4889d0                           mov rax, rdx
00000001412c68cc 4883c210                         add rdx, 0x10
00000001412c68d0 4889d6                           mov rsi, rdx
00000001412c68d3 4883781854                       cmp qword ptr [rax + 0x18], 0x54
00000001412c68d8 757c                             jne 0x1412c6956
00000001412c68da 488b16                           mov rdx, qword ptr [rsi]
00000001412c68dd 488d0db97f7d01                   lea rcx, [rip + 0x17d7fb9]
00000001412c68e4 41b854000000                     mov r8d, 0x54
00000001412c68ea e801d36501                       call 0x142923bf0
00000001412c68ef 85c0                             test eax, eax
00000001412c68f1 7563                             jne 0x1412c6956
00000001412c68f3 488d0576759601                   lea rax, [rip + 0x1967576]
00000001412c68fa 488945d0                         mov qword ptr [rbp - 0x30], rax
00000001412c68fe 488d059bcf26ff                   lea rax, [rip - 0xd93065]
00000001412c6905 488945d8                         mov qword ptr [rbp - 0x28], rax
00000001412c6909 4889e8                           mov rax, rbp
00000001412c690c 488945e0                         mov qword ptr [rbp - 0x20], rax
00000001412c6910 488d057935e4fe                   lea rax, [rip - 0x11bca87]
00000001412c6917 488945e8                         mov qword ptr [rbp - 0x18], rax
00000001412c691b 488d05066d9601                   lea rax, [rip + 0x1966d06]
00000001412c6922 48894510                         mov qword ptr [rbp + 0x10], rax
00000001412c6926 48c7451803000000                 mov qword ptr [rbp + 0x18], 3
00000001412c692e 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
00000001412c6936 488d45d0                         lea rax, [rbp - 0x30]
00000001412c693a 48894520                         mov qword ptr [rbp + 0x20], rax
00000001412c693e 48c7452802000000                 mov qword ptr [rbp + 0x28], 2
00000001412c6946 488d150b6d9601                   lea rdx, [rip + 0x1966d0b]
00000001412c694d 488d4d10                         lea rcx, [rbp + 0x10]
00000001412c6951 e83a2a6101                       call 0x1428d9390
00000001412c6956 488d0513759601                   lea rax, [rip + 0x1967513]
00000001412c695d 48894510                         mov qword ptr [rbp + 0x10], rax
00000001412c6961 488d0538cf26ff                   lea rax, [rip - 0xd930c8]
00000001412c6968 48894518                         mov qword ptr [rbp + 0x18], rax
00000001412c696c 48897520                         mov qword ptr [rbp + 0x20], rsi
00000001412c6970 488d051935e4fe                   lea rax, [rip - 0x11bcae7]
00000001412c6977 48894528                         mov qword ptr [rbp + 0x28], rax
00000001412c697b 4889e9                           mov rcx, rbp
00000001412c697e 48894d30                         mov qword ptr [rbp + 0x30], rcx
00000001412c6982 48894538                         mov qword ptr [rbp + 0x38], rax
00000001412c6986 488d05db6b9601                   lea rax, [rip + 0x1966bdb]
00000001412c698d 488945d0                         mov qword ptr [rbp - 0x30], rax
00000001412c6991 48c745d803000000                 mov qword ptr [rbp - 0x28], 3
00000001412c6999 48c745f000000000                 mov qword ptr [rbp - 0x10], 0
00000001412c69a1 488d4510                         lea rax, [rbp + 0x10]
00000001412c69a5 488945e0                         mov qword ptr [rbp - 0x20], rax
00000001412c69a9 48c745e803000000                 mov qword ptr [rbp - 0x18], 3
00000001412c69b1 488d15186c9601                   lea rdx, [rip + 0x1966c18]
00000001412c69b8 488d4dd0                         lea rcx, [rbp - 0x30]
00000001412c69bc e8cf296101                       call 0x1428d9390
00000001412c69c1 cc                               int3 
00000001412c69c2 66666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
00000001412c69d0 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001412c69d5 55                               push rbp
00000001412c69d6 56                               push rsi
00000001412c69d7 4883ec28                         sub rsp, 0x28
00000001412c69db 488daa80000000                   lea rbp, [rdx + 0x80]
00000001412c69e2 488d4da8                         lea rcx, [rbp - 0x58]
00000001412c69e6 e8c5ba91ff                       call 0x140be24b0
00000001412c69eb 90                               nop 
00000001412c69ec 4883c428                         add rsp, 0x28
00000001412c69f0 5e                               pop rsi
00000001412c69f1 5d                               pop rbp
00000001412c69f2 c3                               ret 
00000001412c69f3 666666662e0f1f840000000000       nop word ptr cs:[rax + rax]
00000001412c6a00 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001412c6a05 55                               push rbp
00000001412c6a06 56                               push rsi
00000001412c6a07 4883ec28                         sub rsp, 0x28
00000001412c6a0b 488daa80000000                   lea rbp, [rdx + 0x80]
00000001412c6a12 488d4d10                         lea rcx, [rbp + 0x10]
00000001412c6a16 e8959adffe                       call 0x1400c04b0
00000001412c6a1b 90                               nop 
00000001412c6a1c 4883c428                         add rsp, 0x28
00000001412c6a20 5e                               pop rsi
00000001412c6a21 5d                               pop rbp
00000001412c6a22 c3                               ret 
