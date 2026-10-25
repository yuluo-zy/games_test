1423246b0: 4157                     push r15
1423246b2: 4156                     push r14
1423246b4: 4155                     push r13
1423246b6: 4154                     push r12
1423246b8: 56                       push rsi
1423246b9: 57                       push rdi
1423246ba: 55                       push rbp
1423246bb: 53                       push rbx
1423246bc: 4881ecb8010000           sub rsp, 0x1b8
1423246c3: 440f29b424a0010000       movaps xmmword ptr [rsp + 0x1a0], xmm14
1423246cc: 440f29ac2490010000       movaps xmmword ptr [rsp + 0x190], xmm13
1423246d5: 440f29a42480010000       movaps xmmword ptr [rsp + 0x180], xmm12
1423246de: 440f299c2470010000       movaps xmmword ptr [rsp + 0x170], xmm11
1423246e7: 440f29942460010000       movaps xmmword ptr [rsp + 0x160], xmm10
1423246f0: 440f298c2450010000       movaps xmmword ptr [rsp + 0x150], xmm9
1423246f9: 440f29842440010000       movaps xmmword ptr [rsp + 0x140], xmm8
142324702: 0f29bc2430010000         movaps xmmword ptr [rsp + 0x130], xmm7
14232470a: 0f29b42420010000         movaps xmmword ptr [rsp + 0x120], xmm6
142324712: 488b842430020000         mov rax, qword ptr [rsp + 0x230]
14232471a: 4c8b9c2438020000         mov r11, qword ptr [rsp + 0x238]
142324722: 4d8b13                   mov r10, qword ptr [r11]
142324725: 4d8b5b08                 mov r11, qword ptr [r11 + 8]
142324729: 498b32                   mov rsi, qword ptr [r10]
14232472c: 31ff                     xor edi, edi
14232472e: 4889f3                   mov rbx, rsi
142324731: 492b5b18                 sub rbx, qword ptr [r11 + 0x18]
142324735: 480f42df                 cmovb rbx, rdi
142324739: 492b7338                 sub rsi, qword ptr [r11 + 0x38]
14232473d: 480f42f7                 cmovb rsi, rdi
142324741: 4d8b7330                 mov r14, qword ptr [r11 + 0x30]
142324745: 4929f6                   sub r14, rsi
142324748: 4c0f42f7                 cmovb r14, rdi
14232474c: 400f96c6                 setbe sil
142324750: 4d8b7b10                 mov r15, qword ptr [r11 + 0x10]
142324754: 4929df                   sub r15, rbx
142324757: 4c0f42ff                 cmovb r15, rdi
14232475b: 4d8b5b40                 mov r11, qword ptr [r11 + 0x40]
14232475f: 0f96c3                   setbe bl
142324762: 4d01f7                   add r15, r14
142324765: 4c89df                   mov rdi, r11
142324768: 4c29ff                   sub rdi, r15
14232476b: 49893a                   mov qword ptr [r10], rdi
14232476e: 4084f3                   test bl, sil
142324771: 0f840b070000             je 0x142324e82
142324777: 4c8b9c2440020000         mov r11, qword ptr [rsp + 0x240]
14232477f: 4d8b13                   mov r10, qword ptr [r11]
142324782: 4d8b5b08                 mov r11, qword ptr [r11 + 8]
142324786: 498b32                   mov rsi, qword ptr [r10]
142324789: 31ff                     xor edi, edi
14232478b: 4889f3                   mov rbx, rsi
14232478e: 492b5b18                 sub rbx, qword ptr [r11 + 0x18]
142324792: 480f42df                 cmovb rbx, rdi
142324796: 492b7338                 sub rsi, qword ptr [r11 + 0x38]
14232479a: 480f42f7                 cmovb rsi, rdi
14232479e: 4d8b7330                 mov r14, qword ptr [r11 + 0x30]
1423247a2: 4929f6                   sub r14, rsi
1423247a5: 4c0f42f7                 cmovb r14, rdi
1423247a9: 400f96c6                 setbe sil
1423247ad: 4d8b7b10                 mov r15, qword ptr [r11 + 0x10]
1423247b1: 4929df                   sub r15, rbx
1423247b4: 4c0f42ff                 cmovb r15, rdi
1423247b8: 4d8b5b40                 mov r11, qword ptr [r11 + 0x40]
1423247bc: 0f96c3                   setbe bl
1423247bf: 4d01f7                   add r15, r14
1423247c2: 4c89df                   mov rdi, r11
1423247c5: 4c29ff                   sub rdi, r15
1423247c8: 49893a                   mov qword ptr [r10], rdi
1423247cb: 4084f3                   test bl, sil
1423247ce: 0f84ae060000             je 0x142324e82
1423247d4: 803800                   cmp byte ptr [rax], 0
1423247d7: 0f840e070000             je 0x142324eeb
1423247dd: 4d89c6                   mov r14, r8
1423247e0: 4889cf                   mov rdi, rcx
1423247e3: 4c89cb                   mov rbx, r9
1423247e6: 488b0a                   mov rcx, qword ptr [rdx]
1423247e9: 488d15f85ad200           lea rdx, [rip + 0xd25af8]
1423247f0: e88b9ffeff               call 0x14230e780
1423247f5: 4885c0                   test rax, rax
1423247f8: 0f8428070000             je 0x142324f26
1423247fe: 4889c6                   mov rsi, rax
142324801: 488d05c85bd200           lea rax, [rip + 0xd25bc8]
142324808: 4889842490000000         mov qword ptr [rsp + 0x90], rax
142324810: 48c78424980000003f000000 mov qword ptr [rsp + 0x98], 0x3f
14232481c: 48837e183f               cmp qword ptr [rsi + 0x18], 0x3f
142324821: 0f85c9060000             jne 0x142324ef0
142324827: 488b5610                 mov rdx, qword ptr [rsi + 0x10]
14232482b: 488d0d9e5bd200           lea rcx, [rip + 0xd25b9e]
142324832: 41b83f000000             mov r8d, 0x3f
142324838: e8b3f35f00               call 0x142923bf0
14232483d: 85c0                     test eax, eax
14232483f: 0f85ab060000             jne 0x142324ef0
142324845: 488b0e                   mov rcx, qword ptr [rsi]
142324848: 488b4608                 mov rax, qword ptr [rsi + 8]
14232484c: ff5018                   call qword ptr [rax + 0x18]
14232484f: 4885c0                   test rax, rax
142324852: 0f84ce060000             je 0x142324f26
142324858: 4c8bbc2420020000         mov r15, qword ptr [rsp + 0x220]
142324860: 4c8bac2428020000         mov r13, qword ptr [rsp + 0x228]
142324868: 488bac2450020000         mov rbp, qword ptr [rsp + 0x250]
142324870: 488b08                   mov rcx, qword ptr [rax]
142324873: 48898c24c0000000         mov qword ptr [rsp + 0xc0], rcx
14232487b: 488b4008                 mov rax, qword ptr [rax + 8]
14232487f: 48898424b8000000         mov qword ptr [rsp + 0xb8], rax
142324887: 498b0e                   mov rcx, qword ptr [r14]
14232488a: e8f19435fe               call 0x14067dd80
14232488f: 488d4808                 lea rcx, [rax + 8]
142324893: 488d5020                 lea rdx, [rax + 0x20]
142324897: 4c8d4010                 lea r8, [rax + 0x10]
14232489b: 4c8d4828                 lea r9, [rax + 0x28]
14232489f: 80b89802000000           cmp byte ptr [rax + 0x298], 0
1423248a6: 480f45ca                 cmovne rcx, rdx
1423248aa: 4d0f44c8                 cmove r9, r8
1423248ae: 498b01                   mov rax, qword ptr [r9]
1423248b1: 488b09                   mov rcx, qword ptr [rcx]
1423248b4: 48898c24d0000000         mov qword ptr [rsp + 0xd0], rcx
1423248bc: 48898424d8000000         mov qword ptr [rsp + 0xd8], rax
1423248c4: 488b0f                   mov rcx, qword ptr [rdi]
1423248c7: 4c8b6708                 mov r12, qword ptr [rdi + 8]
1423248cb: 8b5714                   mov edx, dword ptr [rdi + 0x14]
1423248ce: 488bb1f8000000           mov rsi, qword ptr [rcx + 0xf8]
1423248d5: 488b8100010000           mov rax, qword ptr [rcx + 0x100]
1423248dc: 488d3c86                 lea rdi, [rsi + rax*4]
1423248e0: 48894c2448               mov qword ptr [rsp + 0x48], rcx
1423248e5: 0fb68108010000           movzx eax, byte ptr [rcx + 0x108]
1423248ec: 88442433                 mov byte ptr [rsp + 0x33], al
1423248f0: 488b03                   mov rax, qword ptr [rbx]
1423248f3: 4889842480000000         mov qword ptr [rsp + 0x80], rax
1423248fb: 488b4500                 mov rax, qword ptr [rbp]
1423248ff: 89d5                     mov ebp, edx
142324901: 4883c058                 add rax, 0x58
142324905: 48898424a8000000         mov qword ptr [rsp + 0xa8], rax
14232490d: 498b07                   mov rax, qword ptr [r15]
142324910: 48898424b0000000         mov qword ptr [rsp + 0xb0], rax
142324918: 498b4500                 mov rax, qword ptr [r13]
14232491c: 48898424c8000000         mov qword ptr [rsp + 0xc8], rax
142324924: b808000000               mov eax, 8
142324929: 4889842488000000         mov qword ptr [rsp + 0x88], rax
142324931: 48c744243800000000       mov qword ptr [rsp + 0x38], 0
14232493a: f3440f1005a5576300       movss xmm8, dword ptr [rip + 0x6357a5]
142324943: f2440f100db4936b00       movsd xmm9, qword ptr [rip + 0x6b93b4]
14232494c: f30f1035b00f6000         movss xmm6, dword ptr [rip + 0x600fb0]
142324954: f30f103dfc6c7500         movss xmm7, dword ptr [rip + 0x756cfc]
14232495c: 450f57d2                 xorps xmm10, xmm10
142324960: f3440f101d070f6000       movss xmm11, dword ptr [rip + 0x600f07]
142324969: 440f28250fa06000         movaps xmm12, xmmword ptr [rip + 0x60a00f]
142324971: f3440f102d2efc6000       movss xmm13, dword ptr [rip + 0x60fc2e]
14232497a: 48c744245800000000       mov qword ptr [rsp + 0x58], 0
142324983: 48c744245000000000       mov qword ptr [rsp + 0x50], 0
14232498c: 4531ff                   xor r15d, r15d
14232498f: 31c0                     xor eax, eax
142324991: 89542434                 mov dword ptr [rsp + 0x34], edx
142324995: 807c243300               cmp byte ptr [rsp + 0x33], 0
14232499a: 0f8480000000             je 0x142324a20
1423249a0: 4c39f8                   cmp rax, r15
1423249a3: 0f8578010000             jne 0x142324b21
1423249a9: 0f1f8000000000           nop dword ptr [rax]
1423249b0: 4839fe                   cmp rsi, rdi
1423249b3: 0f84cf040000             je 0x142324e88
1423249b9: 8b0e                     mov ecx, dword ptr [rsi]
1423249bb: 4883c604                 add rsi, 4
1423249bf: 498b8424a8010000         mov rax, qword ptr [r12 + 0x1a8]
1423249c7: 488d0cc9                 lea rcx, [rcx + rcx*8]
1423249cb: 4c8b7cc810               mov r15, qword ptr [rax + rcx*8 + 0x10]
1423249d0: 4d85ff                   test r15, r15
1423249d3: 74db                     je 0x1423249b0
1423249d5: 488d0cc8                 lea rcx, [rax + rcx*8]
1423249d9: 488b442448               mov rax, qword ptr [rsp + 0x48]
1423249de: 488b9010010000           mov rdx, qword ptr [rax + 0x110]
1423249e5: 488b4138                 mov rax, qword ptr [rcx + 0x38]
1423249e9: 48395140                 cmp qword ptr [rcx + 0x40], rdx
1423249ed: 0f86c6000000             jbe 0x142324ab9
1423249f3: 488b14d0                 mov rdx, qword ptr [rax + rdx*8]
1423249f7: 488b4918                 mov rcx, qword ptr [rcx + 0x18]
1423249fb: 4885d2                   test rdx, rdx
1423249fe: 0f84b9000000             je 0x142324abd
142324a04: 48f7d2                   not rdx
142324a07: 488d1452                 lea rdx, [rdx + rdx*2]
142324a0b: 48c1e204                 shl rdx, 4
142324a0f: 488b541110               mov rdx, qword ptr [rcx + rdx + 0x10]
142324a14: 4889542438               mov qword ptr [rsp + 0x38], rdx
142324a19: e9a8000000               jmp 0x142324ac6
142324a1e: 6690                     nop 
142324a20: 4c39f8                   cmp rax, r15
142324a23: 0f8578010000             jne 0x142324ba1
142324a29: 0f1f8000000000           nop dword ptr [rax]
142324a30: 4839fe                   cmp rsi, rdi
142324a33: 0f844f040000             je 0x142324e88
142324a39: 8b0e                     mov ecx, dword ptr [rsi]
142324a3b: 4883c604                 add rsi, 4
142324a3f: 498b842400010000         mov rax, qword ptr [r12 + 0x100]
142324a47: 488d0c89                 lea rcx, [rcx + rcx*4]
142324a4b: 48c1e105                 shl rcx, 5
142324a4f: 4c8b7c0858               mov r15, qword ptr [rax + rcx + 0x58]
142324a54: 4d85ff                   test r15, r15
142324a57: 74d7                     je 0x142324a30
142324a59: 4801c8                   add rax, rcx
142324a5c: 8b8894000000             mov ecx, dword ptr [rax + 0x94]
142324a62: 4d8b8c24a8010000         mov r9, qword ptr [r12 + 0x1a8]
142324a6a: 4c8d14c9                 lea r10, [rcx + rcx*8]
142324a6e: 4b8d14d1                 lea rdx, [r9 + r10*8]
142324a72: 488b4c2448               mov rcx, qword ptr [rsp + 0x48]
142324a77: 4c8b8110010000           mov r8, qword ptr [rcx + 0x110]
142324a7e: 4b8b4cd138               mov rcx, qword ptr [r9 + r10*8 + 0x38]
142324a83: 4f3944d140               cmp qword ptr [r9 + r10*8 + 0x40], r8
142324a88: 0f869f000000             jbe 0x142324b2d
142324a8e: 4e8b04c1                 mov r8, qword ptr [rcx + r8*8]
142324a92: 488b5218                 mov rdx, qword ptr [rdx + 0x18]
142324a96: 4d85c0                   test r8, r8
142324a99: 0f8492000000             je 0x142324b31
142324a9f: 49f7d0                   not r8
142324aa2: 4f8d0440                 lea r8, [r8 + r8*2]
142324aa6: 49c1e004                 shl r8, 4
142324aaa: 4e8b440210               mov r8, qword ptr [rdx + r8 + 0x10]
142324aaf: 4c89442438               mov qword ptr [rsp + 0x38], r8
142324ab4: e981000000               jmp 0x142324b3a
142324ab9: 488b4918                 mov rcx, qword ptr [rcx + 0x18]
142324abd: 48c744243800000000       mov qword ptr [rsp + 0x38], 0
142324ac6: 4c8b442448               mov r8, qword ptr [rsp + 0x48]
142324acb: 498b9018010000           mov rdx, qword ptr [r8 + 0x118]
142324ad2: 4d8b8020010000           mov r8, qword ptr [r8 + 0x120]
142324ad9: 488b14d0                 mov rdx, qword ptr [rax + rdx*8]
142324add: 48f7d2                   not rdx
142324ae0: 488d1452                 lea rdx, [rdx + rdx*2]
142324ae4: 48c1e204                 shl rdx, 4
142324ae8: 4c8b4c1110               mov r9, qword ptr [rcx + rdx + 0x10]
142324aed: 4c894c2458               mov qword ptr [rsp + 0x58], r9
142324af2: 488b541128               mov rdx, qword ptr [rcx + rdx + 0x28]
142324af7: 4889542468               mov qword ptr [rsp + 0x68], rdx
142324afc: 4a8b04c0                 mov rax, qword ptr [rax + r8*8]
142324b00: 48f7d0                   not rax
142324b03: 488d0440                 lea rax, [rax + rax*2]
142324b07: 48c1e004                 shl rax, 4
142324b0b: 488b540110               mov rdx, qword ptr [rcx + rax + 0x10]
142324b10: 4889542450               mov qword ptr [rsp + 0x50], rdx
142324b15: 488b440128               mov rax, qword ptr [rcx + rax + 0x28]
142324b1a: 4889442460               mov qword ptr [rsp + 0x60], rax
142324b1f: 31c0                     xor eax, eax
142324b21: 4889442478               mov qword ptr [rsp + 0x78], rax
142324b26: 89c0                     mov eax, eax
142324b28: e989000000               jmp 0x142324bb6
142324b2d: 488b5218                 mov rdx, qword ptr [rdx + 0x18]
142324b31: 48c744243800000000       mov qword ptr [rsp + 0x38], 0
142324b3a: 4c8b4c2448               mov r9, qword ptr [rsp + 0x48]
142324b3f: 4d8b8118010000           mov r8, qword ptr [r9 + 0x118]
142324b46: 4d8b8920010000           mov r9, qword ptr [r9 + 0x120]
142324b4d: 4e8b04c1                 mov r8, qword ptr [rcx + r8*8]
142324b51: 49f7d0                   not r8
142324b54: 4f8d0440                 lea r8, [r8 + r8*2]
142324b58: 49c1e004                 shl r8, 4
142324b5c: 4e8b540210               mov r10, qword ptr [rdx + r8 + 0x10]
142324b61: 4c89542458               mov qword ptr [rsp + 0x58], r10
142324b66: 4e8b440228               mov r8, qword ptr [rdx + r8 + 0x28]
142324b6b: 4c89442468               mov qword ptr [rsp + 0x68], r8
142324b70: 4a8b0cc9                 mov rcx, qword ptr [rcx + r9*8]
142324b74: 48f7d1                   not rcx
142324b77: 488d0c49                 lea rcx, [rcx + rcx*2]
142324b7b: 48c1e104                 shl rcx, 4
142324b7f: 4c8b440a10               mov r8, qword ptr [rdx + rcx + 0x10]
142324b84: 4c89442450               mov qword ptr [rsp + 0x50], r8
142324b89: 488b4c0a28               mov rcx, qword ptr [rdx + rcx + 0x28]
142324b8e: 48894c2460               mov qword ptr [rsp + 0x60], rcx
142324b93: 488b4050                 mov rax, qword ptr [rax + 0x50]
142324b97: 4889842488000000         mov qword ptr [rsp + 0x88], rax
142324b9f: 31c0                     xor eax, eax
142324ba1: 4889442478               mov qword ptr [rsp + 0x78], rax
142324ba6: 48c1e004                 shl rax, 4
142324baa: 488b8c2488000000         mov rcx, qword ptr [rsp + 0x88]
142324bb2: 8b440108                 mov eax, dword ptr [rcx + rax + 8]
142324bb6: 488d1c40                 lea rbx, [rax + rax*2]
142324bba: 48c1e304                 shl rbx, 4
142324bbe: 48035c2438               add rbx, qword ptr [rsp + 0x38]
142324bc3: 488b4c2458               mov rcx, qword ptr [rsp + 0x58]
142324bc8: 4c8d2cc1                 lea r13, [rcx + rax*8]
142324bcc: 488b4c2468               mov rcx, qword ptr [rsp + 0x68]
142324bd1: 488d0c81                 lea rcx, [rcx + rax*4]
142324bd5: 48894c2440               mov qword ptr [rsp + 0x40], rcx
142324bda: 4c6bf068                 imul r14, rax, 0x68
142324bde: 4c03742450               add r14, qword ptr [rsp + 0x50]
142324be3: 488b4c2460               mov rcx, qword ptr [rsp + 0x60]
142324be8: 488d0481                 lea rax, [rcx + rax*4]
142324bec: 4889442470               mov qword ptr [rsp + 0x70], rax
142324bf1: f30f104310               movss xmm0, dword ptr [rbx + 0x10]
142324bf6: f30f104b18               movss xmm1, dword ptr [rbx + 0x18]
142324bfb: f30f105314               movss xmm2, dword ptr [rbx + 0x14]
142324c00: f3410f58d0               addss xmm2, xmm8
142324c05: f30f11842490000000       movss dword ptr [rsp + 0x90], xmm0
142324c0e: f30f11942494000000       movss dword ptr [rsp + 0x94], xmm2
142324c17: f30f118c2498000000       movss dword ptr [rsp + 0x98], xmm1
142324c20: f2440f118c249c000000     movsd qword ptr [rsp + 0x9c], xmm9
142324c2a: c78424a400000000000080   mov dword ptr [rsp + 0xa4], 0x80000000
142324c35: 488d8c24e0000000         lea rcx, [rsp + 0xe0]
142324c3d: 488b9424c8000000         mov rdx, qword ptr [rsp + 0xc8]
142324c45: 4c8d842490000000         lea r8, [rsp + 0x90]
142324c4d: 0f28de                   movaps xmm3, xmm6
142324c50: e83b805afe               call 0x1408ccc90
142324c55: 83bc241001000002         cmp dword ptr [rsp + 0x110], 2
142324c5d: 0f857d000000             jne 0x142324ce0
142324c63: f30f105310               movss xmm2, dword ptr [rbx + 0x10]
142324c68: f30f105b18               movss xmm3, dword ptr [rbx + 0x18]
142324c6d: 488b8c24c0000000         mov rcx, qword ptr [rsp + 0xc0]
142324c75: 488b9424b8000000         mov rdx, qword ptr [rsp + 0xb8]
142324c7d: e87e5b6cfe               call 0x1409ea800
142324c82: 89c5                     mov ebp, eax
142324c84: f30f104b10               movss xmm1, dword ptr [rbx + 0x10]
142324c89: f30f105318               movss xmm2, dword ptr [rbx + 0x18]
142324c8e: c744242000000243         mov dword ptr [rsp + 0x20], 0x43020000
142324c96: 488d8c24d0000000         lea rcx, [rsp + 0xd0]
142324c9e: 0f28df                   movaps xmm3, xmm7
142324ca1: e84a016ffe               call 0x140a14df0
142324ca6: 418b4500                 mov eax, dword ptr [r13]
142324caa: 4084ed                   test bpl, bpl
142324cad: 0f8486000000             je 0x142324d39
142324cb3: 83f806                   cmp eax, 6
142324cb6: 0f877d000000             ja 0x142324d39
142324cbc: b954000000               mov ecx, 0x54
142324cc1: 0fa3c1                   bt ecx, eax
142324cc4: 7373                     jae 0x142324d39
142324cc6: 8b6c2434                 mov ebp, dword ptr [rsp + 0x34]
142324cca: 488b442440               mov rax, qword ptr [rsp + 0x40]
142324ccf: 8928                     mov dword ptr [rax], ebp
142324cd1: 41c7450000000000         mov dword ptr [r13], 0
142324cd9: eb23                     jmp 0x142324cfe
142324cdb: 0f1f440000               nop dword ptr [rax + rax]
142324ce0: f30f108424fc000000       movss xmm0, dword ptr [rsp + 0xfc]
142324ce9: 488b442440               mov rax, qword ptr [rsp + 0x40]
142324cee: 8928                     mov dword ptr [rax], ebp
142324cf0: 41c7450001000000         mov dword ptr [r13], 1
142324cf8: f3410f114504             movss dword ptr [r13 + 4], xmm0
142324cfe: 488b4c2470               mov rcx, qword ptr [rsp + 0x70]
142324d03: 8929                     mov dword ptr [rcx], ebp
142324d05: 41c70600000000           mov dword ptr [r14], 0
142324d0c: 488b442478               mov rax, qword ptr [rsp + 0x78]
142324d11: 48ffc0                   inc rax
142324d14: 41837d0002               cmp dword ptr [r13], 2
142324d19: 0f8476fcffff             je 0x142324995
142324d1f: 8929                     mov dword ptr [rcx], ebp
142324d21: 41807e3000               cmp byte ptr [r14 + 0x30], 0
142324d26: 0f8569fcffff             jne 0x142324995
142324d2c: 41c7463400000000         mov dword ptr [r14 + 0x34], 0
142324d34: e95cfcffff               jmp 0x142324995
142324d39: 8d48fe                   lea ecx, [rax - 2]
142324d3c: 83f905                   cmp ecx, 5
142324d3f: ba03000000               mov edx, 3
142324d44: 0f43ca                   cmovae ecx, edx
142324d47: 85c9                     test ecx, ecx
142324d49: 7450                     je 0x142324d9b
142324d4b: 83f903                   cmp ecx, 3
142324d4e: 757e                     jne 0x142324dce
142324d50: 410f2ec2                 ucomiss xmm0, xmm10
142324d54: 0f97c1                   seta cl
142324d57: 4008cd                   or bpl, cl
142324d5a: 8b6c2434                 mov ebp, dword ptr [rsp + 0x34]
142324d5e: 7572                     jne 0x142324dd2
142324d60: f30f104b10               movss xmm1, dword ptr [rbx + 0x10]
142324d65: f3440f107314             movss xmm14, dword ptr [rbx + 0x14]
142324d6b: f30f105318               movss xmm2, dword ptr [rbx + 0x18]
142324d70: 488b8c2480000000         mov rcx, qword ptr [rsp + 0x80]
142324d78: e83322f2fe               call 0x141246fb0
142324d7d: 440f2ef0                 ucomiss xmm14, xmm0
142324d81: 0f86e9000000             jbe 0x142324e70
142324d87: 488b442440               mov rax, qword ptr [rsp + 0x40]
142324d8c: 8928                     mov dword ptr [rax], ebp
142324d8e: 41c7450006000000         mov dword ptr [r13], 6
142324d96: e963ffffff               jmp 0x142324cfe
142324d9b: 410f2ec2                 ucomiss xmm0, xmm10
142324d9f: 8b6c2434                 mov ebp, dword ptr [rsp + 0x34]
142324da3: 762d                     jbe 0x142324dd2
142324da5: 488b8c24a8000000         mov rcx, qword ptr [rsp + 0xa8]
142324dad: e86e357cfe               call 0x140ae8320
142324db2: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
142324db7: 8929                     mov dword ptr [rcx], ebp
142324db9: 84c0                     test al, al
142324dbb: 0f8510ffffff             jne 0x142324cd1
142324dc1: 41c7450004000000         mov dword ptr [r13], 4
142324dc9: e930ffffff               jmp 0x142324cfe
142324dce: 8b6c2434                 mov ebp, dword ptr [rsp + 0x34]
142324dd2: 83f802                   cmp eax, 2
142324dd5: 0f8523ffffff             jne 0x142324cfe
142324ddb: f3450f107604             movss xmm14, dword ptr [r14 + 4]
142324de1: f30f104b10               movss xmm1, dword ptr [rbx + 0x10]
142324de6: f30f105318               movss xmm2, dword ptr [rbx + 0x18]
142324deb: 410fb61e                 movzx ebx, byte ptr [r14]
142324def: 488b8c2480000000         mov rcx, qword ptr [rsp + 0x80]
142324df7: e8b421f2fe               call 0x141246fb0
142324dfc: f3410f5fc3               maxss xmm0, xmm11
142324e01: f6c301                   test bl, 1
142324e04: 743e                     je 0x142324e44
142324e06: 0f28c8                   movaps xmm1, xmm0
142324e09: f3410f5cce               subss xmm1, xmm14
142324e0e: 488b8424b0000000         mov rax, qword ptr [rsp + 0xb0]
142324e16: f30f5e4828               divss xmm1, dword ptr [rax + 0x28]
142324e1b: 660f7ec8                 movd eax, xmm1
142324e1f: 25ffffff7f               and eax, 0x7fffffff
142324e24: 3dffff7f7f               cmp eax, 0x7f7fffff
142324e29: 7f19                     jg 0x142324e44
142324e2b: 410f54cc                 andps xmm1, xmm12
142324e2f: 410f2ecd                 ucomiss xmm1, xmm13
142324e33: 760f                     jbe 0x142324e44
142324e35: 488b442440               mov rax, qword ptr [rsp + 0x40]
142324e3a: 8928                     mov dword ptr [rax], ebp
142324e3c: 41c7450000000000         mov dword ptr [r13], 0
142324e44: 488b4c2470               mov rcx, qword ptr [rsp + 0x70]
142324e49: 8929                     mov dword ptr [rcx], ebp
142324e4b: 41c70601000000           mov dword ptr [r14], 1
142324e52: f3410f114604             movss dword ptr [r14 + 4], xmm0
142324e58: 488b442478               mov rax, qword ptr [rsp + 0x78]
142324e5d: 48ffc0                   inc rax
142324e60: 41837d0002               cmp dword ptr [r13], 2
142324e65: 0f842afbffff             je 0x142324995
142324e6b: e9affeffff               jmp 0x142324d1f
142324e70: 418b4500                 mov eax, dword ptr [r13]
142324e74: 83f802                   cmp eax, 2
142324e77: 0f8581feffff             jne 0x142324cfe
142324e7d: e959ffffff               jmp 0x142324ddb
142324e82: 4d891a                   mov qword ptr [r10], r11
142324e85: c60000                   mov byte ptr [rax], 0
142324e88: 0f28b42420010000         movaps xmm6, xmmword ptr [rsp + 0x120]
142324e90: 0f28bc2430010000         movaps xmm7, xmmword ptr [rsp + 0x130]
142324e98: 440f28842440010000       movaps xmm8, xmmword ptr [rsp + 0x140]
142324ea1: 440f288c2450010000       movaps xmm9, xmmword ptr [rsp + 0x150]
142324eaa: 440f28942460010000       movaps xmm10, xmmword ptr [rsp + 0x160]
142324eb3: 440f289c2470010000       movaps xmm11, xmmword ptr [rsp + 0x170]
142324ebc: 440f28a42480010000       movaps xmm12, xmmword ptr [rsp + 0x180]
142324ec5: 440f28ac2490010000       movaps xmm13, xmmword ptr [rsp + 0x190]
142324ece: 440f28b424a0010000       movaps xmm14, xmmword ptr [rsp + 0x1a0]
142324ed7: 4881c4b8010000           add rsp, 0x1b8
142324ede: 5b                       pop rbx
142324edf: 5d                       pop rbp
142324ee0: 5f                       pop rdi
142324ee1: 5e                       pop rsi
142324ee2: 415c                     pop r12
142324ee4: 415d                     pop r13
142324ee6: 415e                     pop r14
142324ee8: 415f                     pop r15
142324eea: c3                       ret 
142324eeb: c60001                   mov byte ptr [rax], 1
142324eee: eb98                     jmp 0x142324e88
142324ef0: 48c78424e000000000000000 mov qword ptr [rsp + 0xe0], 0
142324efc: 4883c610                 add rsi, 0x10
142324f00: 488d05b953d200           lea rax, [rip + 0xd253b9]
142324f07: 4889442420               mov qword ptr [rsp + 0x20], rax
142324f0c: 488d942490000000         lea rdx, [rsp + 0x90]
142324f14: 4c8d8c24e0000000         lea r9, [rsp + 0xe0]
142324f1c: 31c9                     xor ecx, ecx
142324f1e: 4989f0                   mov r8, rsi
142324f21: e81b5c5a00               call 0x1428cab41
142324f26: 488d0d0b6ed200           lea rcx, [rip + 0xd26e0b]
142324f2d: e8de435b00               call 0x1428d9310
142324f32: cc                       int3 
