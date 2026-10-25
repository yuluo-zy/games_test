0000000140993750: 4157                             push r15
0000000140993752: 4156                             push r14
0000000140993754: 56                               push rsi
0000000140993755: 57                               push rdi
0000000140993756: 53                               push rbx
0000000140993757: 4881ec90000000                   sub rsp, 0x90
000000014099375e: 0f29bc2480000000                 movaps xmmword ptr [rsp + 0x80], xmm7
0000000140993766: 0f29742470                       movaps xmmword ptr [rsp + 0x70], xmm6
000000014099376b: 0f28f2                           movaps xmm6, xmm2
000000014099376e: 0f28f9                           movaps xmm7, xmm1
0000000140993771: 4c8b4108                         mov r8, qword ptr [rcx + 8]
0000000140993775: 488b7910                         mov rdi, qword ptr [rcx + 0x10]
0000000140993779: 48c1e704                         shl rdi, 4
000000014099377d: 48bbe50f3358d24f8e71             movabs rbx, 0x718e4fd258330fe5
0000000140993787: 49bebb4427dab0f9f907             movabs r14, 0x7f9f9b0da2744bb
0000000140993791: 6666666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
00000001409937a0: 4885ff                           test rdi, rdi
00000001409937a3: 0f8499000000                     je 0x140993842
00000001409937a9: 4d8d7810                         lea r15, [r8 + 0x10]
00000001409937ad: 498b08                           mov rcx, qword ptr [r8]
00000001409937b0: 498b4008                         mov rax, qword ptr [r8 + 8]
00000001409937b4: ff5040                           call qword ptr [rax + 0x40]
00000001409937b7: 4889c6                           mov rsi, rax
00000001409937ba: 4889c1                           mov rcx, rax
00000001409937bd: ff5218                           call qword ptr [rdx + 0x18]
00000001409937c0: 4883c7f0                         add rdi, -0x10
00000001409937c4: 4d89f8                           mov r8, r15
00000001409937c7: 4885f6                           test rsi, rsi
00000001409937ca: 74d4                             je 0x1409937a0
00000001409937cc: 4d89f8                           mov r8, r15
00000001409937cf: 4839d8                           cmp rax, rbx
00000001409937d2: 75cc                             jne 0x1409937a0
00000001409937d4: 4d89f8                           mov r8, r15
00000001409937d7: 4c39f2                           cmp rdx, r14
00000001409937da: 75c4                             jne 0x1409937a0
00000001409937dc: f30f10058c20f901                 movss xmm0, dword ptr [rip + 0x1f9208c]
00000001409937e4: f30f59f8                         mulss xmm7, xmm0
00000001409937e8: f30f59f0                         mulss xmm6, xmm0
00000001409937ec: f30f583e                         addss xmm7, dword ptr [rsi]
00000001409937f0: f30f100db4711202                 movss xmm1, dword ptr [rip + 0x21271b4]
00000001409937f8: 0f28c7                           movaps xmm0, xmm7
00000001409937fb: e81007f901                       call 0x142923f10 ; 
0000000140993800: f30f1106                         movss dword ptr [rsi], xmm0
0000000140993804: f30f587604                       addss xmm6, dword ptr [rsi + 4]
0000000140993809: f30f10059f711202                 movss xmm0, dword ptr [rip + 0x212719f]
0000000140993811: f30f5fc6                         maxss xmm0, xmm6
0000000140993815: f30f100d77a30502                 movss xmm1, dword ptr [rip + 0x205a377]
000000014099381d: f30f5dc8                         minss xmm1, xmm0
0000000140993821: f30f114e04                       movss dword ptr [rsi + 4], xmm1
0000000140993826: 0f28742470                       movaps xmm6, xmmword ptr [rsp + 0x70]
000000014099382b: 0f28bc2480000000                 movaps xmm7, xmmword ptr [rsp + 0x80]
0000000140993833: 4881c490000000                   add rsp, 0x90
000000014099383a: 5b                               pop rbx
000000014099383b: 5f                               pop rdi
000000014099383c: 5e                               pop rsi
000000014099383d: 415e                             pop r14
000000014099383f: 415f                             pop r15
0000000140993841: c3                               ret 
0000000140993842: 488d0577951502                   lea rax, [rip + 0x2159577]
0000000140993849: 4889442430                       mov qword ptr [rsp + 0x30], rax
000000014099384e: 48c744243823000000               mov qword ptr [rsp + 0x38], 0x23
0000000140993857: 488d442430                       lea rax, [rsp + 0x30]
000000014099385c: 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000140993861: 488d05286677ff                   lea rax, [rip - 0x8899d8]
0000000140993868: 4889442428                       mov qword ptr [rsp + 0x28], rax
000000014099386d: 488d05bc941502                   lea rax, [rip + 0x21594bc]
0000000140993874: 4889442440                       mov qword ptr [rsp + 0x40], rax
0000000140993879: 48c744244802000000               mov qword ptr [rsp + 0x48], 2
0000000140993882: 48c744246000000000               mov qword ptr [rsp + 0x60], 0
000000014099388b: 488d442420                       lea rax, [rsp + 0x20]
0000000140993890: 4889442450                       mov qword ptr [rsp + 0x50], rax
0000000140993895: 48c744245801000000               mov qword ptr [rsp + 0x58], 1
000000014099389e: 488d1503951502                   lea rdx, [rip + 0x2159503]
00000001409938a5: 488d4c2440                       lea rcx, [rsp + 0x40]
00000001409938aa: e8e15af401                       call 0x1428d9390 ; core::panicking::panic_fmt
00000001409938af: cc                               int3 
