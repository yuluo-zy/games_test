0000000140994320: 4157                             push r15
0000000140994322: 4156                             push r14
0000000140994324: 4155                             push r13
0000000140994326: 4154                             push r12
0000000140994328: 56                               push rsi
0000000140994329: 57                               push rdi
000000014099432a: 55                               push rbp
000000014099432b: 53                               push rbx
000000014099432c: 4881ec08010000                   sub rsp, 0x108
0000000140994333: 440f299c24f0000000               movaps xmmword ptr [rsp + 0xf0], xmm11
000000014099433c: 440f299424e0000000               movaps xmmword ptr [rsp + 0xe0], xmm10
0000000140994345: 440f298c24d0000000               movaps xmmword ptr [rsp + 0xd0], xmm9
000000014099434e: 440f298424c0000000               movaps xmmword ptr [rsp + 0xc0], xmm8
0000000140994357: 0f29bc24b0000000                 movaps xmmword ptr [rsp + 0xb0], xmm7
000000014099435f: 0f29b424a0000000                 movaps xmmword ptr [rsp + 0xa0], xmm6
0000000140994367: 44884c2437                       mov byte ptr [rsp + 0x37], r9b
000000014099436c: 4d89c6                           mov r14, r8
000000014099436f: 4989d7                           mov r15, rdx
0000000140994372: 4889ce                           mov rsi, rcx
0000000140994375: 4c8b4108                         mov r8, qword ptr [rcx + 8]
0000000140994379: 4c8b6110                         mov r12, qword ptr [rcx + 0x10]
000000014099437d: 49c1e404                         shl r12, 4
0000000140994381: 48bb661afc7a5763846b             movabs rbx, 0x6b8463577afc1a66
000000014099438b: 49bde3d76841299f4b0e             movabs r13, 0xe4b9f294168d7e3
0000000140994395: 66662e0f1f840000000000           nop word ptr cs:[rax + rax]
00000001409943a0: 4d85e4                           test r12, r12
00000001409943a3: 0f84e4020000                     je 0x14099468d
00000001409943a9: 498d6810                         lea rbp, [r8 + 0x10]
00000001409943ad: 498b08                           mov rcx, qword ptr [r8]
00000001409943b0: 498b4008                         mov rax, qword ptr [r8 + 8]
00000001409943b4: ff5040                           call qword ptr [rax + 0x40]
00000001409943b7: 4889c7                           mov rdi, rax
00000001409943ba: 4889c1                           mov rcx, rax
00000001409943bd: ff5218                           call qword ptr [rdx + 0x18]
00000001409943c0: 4983c4f0                         add r12, -0x10
00000001409943c4: 4989e8                           mov r8, rbp
00000001409943c7: 4885ff                           test rdi, rdi
00000001409943ca: 74d4                             je 0x1409943a0
00000001409943cc: 4989e8                           mov r8, rbp
00000001409943cf: 4839d8                           cmp rax, rbx
00000001409943d2: 75cc                             jne 0x1409943a0
00000001409943d4: 4989e8                           mov r8, rbp
00000001409943d7: 4c39ea                           cmp rdx, r13
00000001409943da: 75c4                             jne 0x1409943a0
00000001409943dc: f3440f1007                       movss xmm8, dword ptr [rdi]
00000001409943e1: f30f107704                       movss xmm6, dword ptr [rdi + 4]
00000001409943e6: f30f103dfa5cfc01                 movss xmm7, dword ptr [rip + 0x1fc5cfa]
00000001409943ee: f30f58fe                         addss xmm7, xmm6
00000001409943f2: f30f5835f6651202                 addss xmm6, dword ptr [rip + 0x21265f6]
00000001409943fa: f30f5835f2651202                 addss xmm6, dword ptr [rip + 0x21265f2]
0000000140994402: 0fb66e54                         movzx ebp, byte ptr [rsi + 0x54]
0000000140994406: 31c0                             xor eax, eax
0000000140994408: 4084ed                           test bpl, bpl
000000014099440b: 41bdcf300000                     mov r13d, 0x30cf
0000000140994411: 440f45e8                         cmovne r13d, eax
0000000140994415: f3440f104f08                     movss xmm9, dword ptr [rdi + 8]
000000014099441b: 4c8d6638                         lea r12, [rsi + 0x38]
000000014099441f: c6463811                         mov byte ptr [rsi + 0x38], 0x11
0000000140994423: f30f1005cd651202                 movss xmm0, dword ptr [rip + 0x21265cd]
000000014099442b: 0f28c8                           movaps xmm1, xmm0
000000014099442e: f3410f5fc8                       maxss xmm1, xmm8
0000000140994433: f30f1015c1651202                 movss xmm2, dword ptr [rip + 0x21265c1]
000000014099443b: 0f28da                           movaps xmm3, xmm2
000000014099443e: f30f5dd9                         minss xmm3, xmm1
0000000140994442: f3410f5fc1                       maxss xmm0, xmm9
0000000140994447: f30f5dd0                         minss xmm2, xmm0
000000014099444b: f30f115c2438                     movss dword ptr [rsp + 0x38], xmm3
0000000140994451: f30f117c243c                     movss dword ptr [rsp + 0x3c], xmm7
0000000140994457: f30f11542440                     movss dword ptr [rsp + 0x40], xmm2
000000014099445d: f2440f10159a980402               movsd xmm10, qword ptr [rip + 0x204989a]
0000000140994466: f2440f11542444                   movsd qword ptr [rsp + 0x44], xmm10
000000014099446d: c744244c00000080                 mov dword ptr [rsp + 0x4c], 0x80000000
0000000140994475: f3440f101d0215f901               movss xmm11, dword ptr [rip + 0x1f91502]
000000014099447e: 440f135c2450                     movlps qword ptr [rsp + 0x50], xmm11
0000000140994484: 66c74424580000                   mov word ptr [rsp + 0x58], 0
000000014099448b: 488d5c2450                       lea rbx, [rsp + 0x50]
0000000140994490: 48895c2420                       mov qword ptr [rsp + 0x20], rbx
0000000140994495: c744242820000000                 mov dword ptr [rsp + 0x28], 0x20
000000014099449d: 488d4c2460                       lea rcx, [rsp + 0x60]
00000001409944a2: 4c8d442438                       lea r8, [rsp + 0x38]
00000001409944a7: f30f101df5b70e02                 movss xmm3, dword ptr [rip + 0x20eb7f5]
00000001409944af: 4c89fa                           mov rdx, r15
00000001409944b2: e8d97ef3ff                       call 0x1408cc390 ; country_core::systems::collision::world::Raycasting::spherecast
00000001409944b7: 83bc249400000002                 cmp dword ptr [rsp + 0x94], 2
00000001409944bf: 7434                             je 0x1409944f5
00000001409944c1: 0f28c7                           movaps xmm0, xmm7
00000001409944c4: f30f5c842490000000               subss xmm0, dword ptr [rsp + 0x90]
00000001409944cd: f30f5805c7320f02                 addss xmm0, dword ptr [rip + 0x20f32c7]
00000001409944d5: 0f28ce                           movaps xmm1, xmm6
00000001409944d8: f30fc2ce03                       cmpunordss xmm1, xmm6
00000001409944dd: 0f28d1                           movaps xmm2, xmm1
00000001409944e0: 0f54d0                           andps xmm2, xmm0
00000001409944e3: f30f5fc6                         maxss xmm0, xmm6
00000001409944e7: 0f55c8                           andnps xmm1, xmm0
00000001409944ea: 0f56ca                           orps xmm1, xmm2
00000001409944ed: 41c604240b                       mov byte ptr [r12], 0xb
00000001409944f2: 0f28f1                           movaps xmm6, xmm1
00000001409944f5: f3440f11442438                   movss dword ptr [rsp + 0x38], xmm8
00000001409944fc: f30f117c243c                     movss dword ptr [rsp + 0x3c], xmm7
0000000140994502: f3440f114c2440                   movss dword ptr [rsp + 0x40], xmm9
0000000140994509: f2440f11542444                   movsd qword ptr [rsp + 0x44], xmm10
0000000140994510: c744244c00000080                 mov dword ptr [rsp + 0x4c], 0x80000000
0000000140994518: 440f135c2450                     movlps qword ptr [rsp + 0x50], xmm11
000000014099451e: 66c74424580000                   mov word ptr [rsp + 0x58], 0
0000000140994525: 44896c2428                       mov dword ptr [rsp + 0x28], r13d
000000014099452a: 48895c2420                       mov qword ptr [rsp + 0x20], rbx
000000014099452f: 488d4c2460                       lea rcx, [rsp + 0x60]
0000000140994534: 4c8d442438                       lea r8, [rsp + 0x38]
0000000140994539: f30f101d63b70e02                 movss xmm3, dword ptr [rip + 0x20eb763]
0000000140994541: 4c89fa                           mov rdx, r15
0000000140994544: e8477ef3ff                       call 0x1408cc390 ; country_core::systems::collision::world::Raycasting::spherecast
0000000140994549: 83bc249400000002                 cmp dword ptr [rsp + 0x94], 2
0000000140994551: 742d                             je 0x140994580
0000000140994553: f30f5cbc2490000000               subss xmm7, dword ptr [rsp + 0x90]
000000014099455c: f30f583d38320f02                 addss xmm7, dword ptr [rip + 0x20f3238]
0000000140994564: 0f2efe                           ucomiss xmm7, xmm6
0000000140994567: 7617                             jbe 0x140994580
0000000140994569: 488b442470                       mov rax, qword ptr [rsp + 0x70]
000000014099456e: 4989442410                       mov qword ptr [r12 + 0x10], rax
0000000140994573: 0f10442460                       movups xmm0, xmmword ptr [rsp + 0x60]
0000000140994578: 410f110424                       movups xmmword ptr [r12], xmm0
000000014099457d: 0f28f7                           movaps xmm6, xmm7
0000000140994580: 498d4608                         lea rax, [r14 + 8]
0000000140994584: 498d4e20                         lea rcx, [r14 + 0x20]
0000000140994588: 498d5610                         lea rdx, [r14 + 0x10]
000000014099458c: 4180be9802000000                 cmp byte ptr [r14 + 0x298], 0
0000000140994594: 480f45c1                         cmovne rax, rcx
0000000140994598: 498d4e28                         lea rcx, [r14 + 0x28]
000000014099459c: 480f44ca                         cmove rcx, rdx
00000001409945a0: 488b09                           mov rcx, qword ptr [rcx]
00000001409945a3: 488b00                           mov rax, qword ptr [rax]
00000001409945a6: 4889442460                       mov qword ptr [rsp + 0x60], rax
00000001409945ab: 48894c2468                       mov qword ptr [rsp + 0x68], rcx
00000001409945b0: 807c243700                       cmp byte ptr [rsp + 0x37], 0
00000001409945b5: 740a                             je 0x1409945c1
00000001409945b7: f30f1005399f0f02                 movss xmm0, dword ptr [rip + 0x20f9f39]
00000001409945bf: eb42                             jmp 0x140994603
00000001409945c1: f30f100f                         movss xmm1, dword ptr [rdi]
00000001409945c5: f30f105708                       movss xmm2, dword ptr [rdi + 8]
00000001409945ca: c744242000000243                 mov dword ptr [rsp + 0x20], 0x43020000
00000001409945d2: 488d4c2460                       lea rcx, [rsp + 0x60]
00000001409945d7: f30f101d79700e02                 movss xmm3, dword ptr [rip + 0x20e7079]
00000001409945df: e80c080800                       call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
00000001409945e4: f30f100d1813f901                 movss xmm1, dword ptr [rip + 0x1f91318]
00000001409945ec: f30f5cc8                         subss xmm1, xmm0
00000001409945f0: 0f57d2                           xorps xmm2, xmm2
00000001409945f3: f30f59d1                         mulss xmm2, xmm1
00000001409945f7: f30f5905f99e0f02                 mulss xmm0, dword ptr [rip + 0x20f9ef9]
00000001409945ff: f30f58c2                         addss xmm0, xmm2
0000000140994603: f30f5835ed9e0f02                 addss xmm6, dword ptr [rip + 0x20f9eed]
000000014099460b: f30f5f055d13f901                 maxss xmm0, dword ptr [rip + 0x1f9135d]
0000000140994613: f30f5ff0                         maxss xmm6, xmm0
0000000140994617: f30f104704                       movss xmm0, dword ptr [rdi + 4]
000000014099461c: 4084ed                           test bpl, bpl
000000014099461f: 740a                             je 0x14099462b
0000000140994621: 0f28c8                           movaps xmm1, xmm0
0000000140994624: f30f5fce                         maxss xmm1, xmm6
0000000140994628: 0f28f1                           movaps xmm6, xmm1
000000014099462b: f30f105650                       movss xmm2, dword ptr [rsi + 0x50]
0000000140994630: f30f101df8b30e02                 movss xmm3, dword ptr [rip + 0x20eb3f8]
0000000140994638: 0f28ce                           movaps xmm1, xmm6
000000014099463b: e8a0be2f00                       call 0x140c904e0 ; utils::time_continuous_lerp
0000000140994640: f30f114704                       movss dword ptr [rdi + 4], xmm0
0000000140994645: 0f28b424a0000000                 movaps xmm6, xmmword ptr [rsp + 0xa0]
000000014099464d: 0f28bc24b0000000                 movaps xmm7, xmmword ptr [rsp + 0xb0]
0000000140994655: 440f288424c0000000               movaps xmm8, xmmword ptr [rsp + 0xc0]
000000014099465e: 440f288c24d0000000               movaps xmm9, xmmword ptr [rsp + 0xd0]
0000000140994667: 440f289424e0000000               movaps xmm10, xmmword ptr [rsp + 0xe0]
0000000140994670: 440f289c24f0000000               movaps xmm11, xmmword ptr [rsp + 0xf0]
0000000140994679: 4881c408010000                   add rsp, 0x108
0000000140994680: 5b                               pop rbx
0000000140994681: 5d                               pop rbp
0000000140994682: 5f                               pop rdi
0000000140994683: 5e                               pop rsi
0000000140994684: 415c                             pop r12
0000000140994686: 415d                             pop r13
0000000140994688: 415e                             pop r14
000000014099468a: 415f                             pop r15
000000014099468c: c3                               ret 
000000014099468d: 488d0554861502                   lea rax, [rip + 0x2158654]
0000000140994694: 4889442438                       mov qword ptr [rsp + 0x38], rax
0000000140994699: 48c744244022000000               mov qword ptr [rsp + 0x40], 0x22
00000001409946a2: 488d442438                       lea rax, [rsp + 0x38]
00000001409946a7: 4889442450                       mov qword ptr [rsp + 0x50], rax
00000001409946ac: 488d05dd5777ff                   lea rax, [rip - 0x88a823]
00000001409946b3: 4889442458                       mov qword ptr [rsp + 0x58], rax
00000001409946b8: 488d0571861502                   lea rax, [rip + 0x2158671]
00000001409946bf: 4889442460                       mov qword ptr [rsp + 0x60], rax
00000001409946c4: 48c744246802000000               mov qword ptr [rsp + 0x68], 2
00000001409946cd: 48c784248000000000000000         mov qword ptr [rsp + 0x80], 0
00000001409946d9: 488d442450                       lea rax, [rsp + 0x50]
00000001409946de: 4889442470                       mov qword ptr [rsp + 0x70], rax
00000001409946e3: 48c744247801000000               mov qword ptr [rsp + 0x78], 1
00000001409946ec: 488d15b5861502                   lea rdx, [rip + 0x21586b5]
00000001409946f3: 488d4c2460                       lea rcx, [rsp + 0x60]
00000001409946f8: e8934cf401                       call 0x1428d9390 ; core::panicking::panic_fmt
00000001409946fd: cc                               int3 
