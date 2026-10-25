140aef1f0: 4157                     push r15
140aef1f2: 4156                     push r14
140aef1f4: 4155                     push r13
140aef1f6: 4154                     push r12
140aef1f8: 56                       push rsi
140aef1f9: 57                       push rdi
140aef1fa: 55                       push rbp
140aef1fb: 53                       push rbx
140aef1fc: 4881ecd8020000           sub rsp, 0x2d8
140aef203: 440f29bc24c0020000       movaps xmmword ptr [rsp + 0x2c0], xmm15
140aef20c: 440f29b424b0020000       movaps xmmword ptr [rsp + 0x2b0], xmm14
140aef215: 440f29ac24a0020000       movaps xmmword ptr [rsp + 0x2a0], xmm13
140aef21e: 440f29a42490020000       movaps xmmword ptr [rsp + 0x290], xmm12
140aef227: 440f299c2480020000       movaps xmmword ptr [rsp + 0x280], xmm11
140aef230: 440f29942470020000       movaps xmmword ptr [rsp + 0x270], xmm10
140aef239: 440f298c2460020000       movaps xmmword ptr [rsp + 0x260], xmm9
140aef242: 440f29842450020000       movaps xmmword ptr [rsp + 0x250], xmm8
140aef24b: 0f29bc2440020000         movaps xmmword ptr [rsp + 0x240], xmm7
140aef253: 0f29b42430020000         movaps xmmword ptr [rsp + 0x230], xmm6
140aef25b: 4c894c2470               mov qword ptr [rsp + 0x70], r9
140aef260: 4c89442478               mov qword ptr [rsp + 0x78], r8
140aef265: 488b01                   mov rax, qword ptr [rcx]
140aef268: 83380a                   cmp dword ptr [rax], 0xa
140aef26b: 0f85a10d0000             jne 0x140af0012
140aef271: 4889542458               mov qword ptr [rsp + 0x58], rdx
140aef276: 488b9c2488030000         mov rbx, qword ptr [rsp + 0x388]
140aef27e: 488bbc2478030000         mov rdi, qword ptr [rsp + 0x378]
140aef286: 488b4720                 mov rax, qword ptr [rdi + 0x20]
140aef28a: 4889442430               mov qword ptr [rsp + 0x30], rax
140aef28f: 488b0b                   mov rcx, qword ptr [rbx]
140aef292: 4c8b6308                 mov r12, qword ptr [rbx + 8]
140aef296: 4c8b11                   mov r10, qword ptr [rcx]
140aef299: 4531ed                   xor r13d, r13d
140aef29c: 4d89d3                   mov r11, r10
140aef29f: 4d2b5c2418               sub r11, qword ptr [r12 + 0x18]
140aef2a4: 4d0f42dd                 cmovb r11, r13
140aef2a8: 4d2b542438               sub r10, qword ptr [r12 + 0x38]
140aef2ad: 4d0f42d5                 cmovb r10, r13
140aef2b1: 4d8b742410               mov r14, qword ptr [r12 + 0x10]
140aef2b6: 498b542430               mov rdx, qword ptr [r12 + 0x30]
140aef2bb: 4889d6                   mov rsi, rdx
140aef2be: 4c29d6                   sub rsi, r10
140aef2c1: 490f42f5                 cmovb rsi, r13
140aef2c5: 0f96c0                   setbe al
140aef2c8: 4d89f7                   mov r15, r14
140aef2cb: 4d29df                   sub r15, r11
140aef2ce: 4d0f42fd                 cmovb r15, r13
140aef2d2: 4c8b4328                 mov r8, qword ptr [rbx + 0x28]
140aef2d6: 4c898424c0000000         mov qword ptr [rsp + 0xc0], r8
140aef2de: 4d8b442440               mov r8, qword ptr [r12 + 0x40]
140aef2e3: 410f96c1                 setbe r9b
140aef2e7: 4c89c3                   mov rbx, r8
140aef2ea: 4829f3                   sub rbx, rsi
140aef2ed: 4c29fb                   sub rbx, r15
140aef2f0: 498b6c2408               mov rbp, qword ptr [r12 + 8]
140aef2f5: 4d8b6c2428               mov r13, qword ptr [r12 + 0x28]
140aef2fa: 4c8b6730                 mov r12, qword ptr [rdi + 0x30]
140aef2fe: 4c89a424b8000000         mov qword ptr [rsp + 0xb8], r12
140aef306: 448b673c                 mov r12d, dword ptr [rdi + 0x3c]
140aef30a: 448964243c               mov dword ptr [rsp + 0x3c], r12d
140aef30f: 4c8b6740                 mov r12, qword ptr [rdi + 0x40]
140aef313: 4c89a424c8000000         mov qword ptr [rsp + 0xc8], r12
140aef31b: 488919                   mov qword ptr [rcx], rbx
140aef31e: 0f57f6                   xorps xmm6, xmm6
140aef321: 4c8ba42490030000         mov r12, qword ptr [rsp + 0x390]
140aef329: 4184c1                   test r9b, al
140aef32c: 754d                     jne 0x140aef37b
140aef32e: 4c89d0                   mov rax, r10
140aef331: 48c1e004                 shl rax, 4
140aef335: 4901c5                   add r13, rax
140aef338: 48c1e604                 shl rsi, 4
140aef33c: 4c89d8                   mov rax, r11
140aef33f: 48c1e004                 shl rax, 4
140aef343: 4801c5                   add rbp, rax
140aef346: 49c1e704                 shl r15, 4
140aef34a: 31c0                     xor eax, eax
140aef34c: 4d39de                   cmp r14, r11
140aef34f: 41b908000000             mov r9d, 8
140aef355: 490f42e9                 cmovb rbp, r9
140aef359: 4e8d5c3df0               lea r11, [rbp + r15 - 0x10]
140aef35e: 490f47c3                 cmova rax, r11
140aef362: 4c39d2                   cmp rdx, r10
140aef365: 4d0f42e9                 cmovb r13, r9
140aef369: 4d8d4c35f0               lea r9, [r13 + rsi - 0x10]
140aef36e: 4c0f46c8                 cmovbe r9, rax
140aef372: 4c8901                   mov qword ptr [rcx], r8
140aef375: f2410f107108             movsd xmm6, qword ptr [r9 + 8]
140aef37b: 498b0c24                 mov rcx, qword ptr [r12]
140aef37f: 803900                   cmp byte ptr [rcx], 0
140aef382: b8ffffffff               mov eax, 0xffffffff
140aef387: 7403                     je 0x140aef38c
140aef389: 8b4108                   mov eax, dword ptr [rcx + 8]
140aef38c: 488bb42480030000         mov rsi, qword ptr [rsp + 0x380]
140aef394: 488b4c2458               mov rcx, qword ptr [rsp + 0x58]
140aef399: 488b09                   mov rcx, qword ptr [rcx]
140aef39c: 48894c2458               mov qword ptr [rsp + 0x58], rcx
140aef3a1: 8944242c                 mov dword ptr [rsp + 0x2c], eax
140aef3a5: 83f8ff                   cmp eax, -1
140aef3a8: 745b                     je 0x140aef405
140aef3aa: 488b442458               mov rax, qword ptr [rsp + 0x58]
140aef3af: 803801                   cmp byte ptr [rax], 1
140aef3b2: 7551                     jne 0x140aef405
140aef3b4: 488b942478030000         mov rdx, qword ptr [rsp + 0x378]
140aef3bc: 8b421c                   mov eax, dword ptr [rdx + 0x1c]
140aef3bf: 488b4a10                 mov rcx, qword ptr [rdx + 0x10]
140aef3c3: 8901                     mov dword ptr [rcx], eax
140aef3c5: 488b3a                   mov rdi, qword ptr [rdx]
140aef3c8: 488b5f30                 mov rbx, qword ptr [rdi + 0x30]
140aef3cc: 4c8b7740                 mov r14, qword ptr [rdi + 0x40]
140aef3d0: 483b5f20                 cmp rbx, qword ptr [rdi + 0x20]
140aef3d4: 7510                     jne 0x140aef3e6
140aef3d6: 488d4f20                 lea rcx, [rdi + 0x20]
140aef3da: 488d1567ad1402           lea rdx, [rip + 0x214ad67]
140aef3e1: e84a32a7ff               call 0x140562630
140aef3e6: 488b4728                 mov rax, qword ptr [rdi + 0x28]
140aef3ea: 4889d9                   mov rcx, rbx
140aef3ed: 48c1e104                 shl rcx, 4
140aef3f1: 4c893408                 mov qword ptr [rax + rcx], r14
140aef3f5: c644080801               mov byte ptr [rax + rcx + 8], 1
140aef3fa: 48ffc3                   inc rbx
140aef3fd: 48895f30                 mov qword ptr [rdi + 0x30], rbx
140aef401: 48ff4740                 inc qword ptr [rdi + 0x40]
140aef405: 488b06                   mov rax, qword ptr [rsi]
140aef408: 488b4e08                 mov rcx, qword ptr [rsi + 8]
140aef40c: 488b10                   mov rdx, qword ptr [rax]
140aef40f: 4531c0                   xor r8d, r8d
140aef412: 4989d1                   mov r9, rdx
140aef415: 4c2b4918                 sub r9, qword ptr [rcx + 0x18]
140aef419: 4d0f42c8                 cmovb r9, r8
140aef41d: 482b5138                 sub rdx, qword ptr [rcx + 0x38]
140aef421: 490f42d0                 cmovb rdx, r8
140aef425: 4c8b5130                 mov r10, qword ptr [rcx + 0x30]
140aef429: 4929d2                   sub r10, rdx
140aef42c: 4d0f42d0                 cmovb r10, r8
140aef430: 0f97c2                   seta dl
140aef433: 4c8b5910                 mov r11, qword ptr [rcx + 0x10]
140aef437: 4d29cb                   sub r11, r9
140aef43a: 4d0f42d8                 cmovb r11, r8
140aef43e: 488b4940                 mov rcx, qword ptr [rcx + 0x40]
140aef442: 410f97c1                 seta r9b
140aef446: 4d01d3                   add r11, r10
140aef449: 4108d1                   or r9b, dl
140aef44c: 44884c242b               mov byte ptr [rsp + 0x2b], r9b
140aef451: 4d0f45d8                 cmovne r11, r8
140aef455: 4c29d9                   sub rcx, r11
140aef458: 488908                   mov qword ptr [rax], rcx
140aef45b: 488b442478               mov rax, qword ptr [rsp + 0x78]
140aef460: 488b00                   mov rax, qword ptr [rax]
140aef463: 4889442460               mov qword ptr [rsp + 0x60], rax
140aef468: 488b7820                 mov rdi, qword ptr [rax + 0x20]
140aef46c: 4885ff                   test rdi, rdi
140aef46f: 0f849d0b0000             je 0x140af0012
140aef475: 488b8c2450030000         mov rcx, qword ptr [rsp + 0x350]
140aef47d: 4c8b8c2448030000         mov r9, qword ptr [rsp + 0x348]
140aef485: 488b942440030000         mov rdx, qword ptr [rsp + 0x340]
140aef48d: 488b842470030000         mov rax, qword ptr [rsp + 0x370]
140aef495: 4c8b442470               mov r8, qword ptr [rsp + 0x70]
140aef49a: 4d8b00                   mov r8, qword ptr [r8]
140aef49d: 4c89442478               mov qword ptr [rsp + 0x78], r8
140aef4a2: 458b411c                 mov r8d, dword ptr [r9 + 0x1c]
140aef4a6: 448984248c000000         mov dword ptr [rsp + 0x8c], r8d
140aef4ae: 4d8b01                   mov r8, qword ptr [r9]
140aef4b1: 4c89842400020000         mov qword ptr [rsp + 0x200], r8
140aef4b9: 4d8b4110                 mov r8, qword ptr [r9 + 0x10]
140aef4bd: 4c898424f8010000         mov qword ptr [rsp + 0x1f8], r8
140aef4c5: 4c8b02                   mov r8, qword ptr [rdx]
140aef4c8: 4c8b4a10                 mov r9, qword ptr [rdx + 0x10]
140aef4cc: 4c898c24e0000000         mov qword ptr [rsp + 0xe0], r9
140aef4d4: 0f28c6                   movaps xmm0, xmm6
140aef4d7: 0f59c6                   mulps xmm0, xmm6
140aef4da: f30f16c8                 movshdup xmm1, xmm0
140aef4de: f30f58c8                 addss xmm1, xmm0
140aef4e2: f30f51f9                 sqrtss xmm7, xmm1
140aef4e6: 440f280592f4e301         movaps xmm8, xmmword ptr [rip + 0x1e3f492]
140aef4ee: 440f28ce                 movaps xmm9, xmm6
140aef4f2: 450f54c8                 andps xmm9, xmm8
140aef4f6: 66450f70d1f5             pshufd xmm10, xmm9, 0xf5
140aef4fc: f3440f16de               movshdup xmm11, xmm6
140aef501: f30f5935af3aff01         mulss xmm6, dword ptr [rip + 0x1ff3aaf]
140aef509: f3440f591dbedded01       mulss xmm11, dword ptr [rip + 0x1edddbe]
140aef512: 440f54c6                 andps xmm8, xmm6
140aef516: f3440f5e1d1105f901       divss xmm11, dword ptr [rip + 0x1f90511]
140aef51f: 8b521c                   mov edx, dword ptr [rdx + 0x1c]
140aef522: 89542444                 mov dword ptr [rsp + 0x44], edx
140aef526: 8b511c                   mov edx, dword ptr [rcx + 0x1c]
140aef529: 89942488000000           mov dword ptr [rsp + 0x88], edx
140aef530: 488b5110                 mov rdx, qword ptr [rcx + 0x10]
140aef534: 48899424f0010000         mov qword ptr [rsp + 0x1f0], rdx
140aef53c: 488b09                   mov rcx, qword ptr [rcx]
140aef53f: 48898c24e8010000         mov qword ptr [rsp + 0x1e8], rcx
140aef547: 488b4c2460               mov rcx, qword ptr [rsp + 0x60]
140aef54c: 488b7108                 mov rsi, qword ptr [rcx + 8]
140aef550: 660f6f06                 movdqa xmm0, xmmword ptr [rsi]
140aef554: 66440fd7e0               pmovmskb r12d, xmm0
140aef559: 41f7d4                   not r12d
140aef55c: 0fb654242b               movzx edx, byte ptr [rsp + 0x2b]
140aef561: 80f201                   xor dl, 1
140aef564: 88542470                 mov byte ptr [rsp + 0x70], dl
140aef568: 8b501c                   mov edx, dword ptr [rax + 0x1c]
140aef56b: 89942480000000           mov dword ptr [rsp + 0x80], edx
140aef572: 488b10                   mov rdx, qword ptr [rax]
140aef575: 488b4010                 mov rax, qword ptr [rax + 0x10]
140aef579: 48898424d0010000         mov qword ptr [rsp + 0x1d0], rax
140aef581: 488b842460030000         mov rax, qword ptr [rsp + 0x360]
140aef589: 448b481c                 mov r9d, dword ptr [rax + 0x1c]
140aef58d: 44894c2440               mov dword ptr [rsp + 0x40], r9d
140aef592: 4c8b08                   mov r9, qword ptr [rax]
140aef595: 488b4010                 mov rax, qword ptr [rax + 0x10]
140aef599: 48898424d8000000         mov qword ptr [rsp + 0xd8], rax
140aef5a1: 488b842468030000         mov rax, qword ptr [rsp + 0x368]
140aef5a9: 448b501c                 mov r10d, dword ptr [rax + 0x1c]
140aef5ad: 4489942484000000         mov dword ptr [rsp + 0x84], r10d
140aef5b5: 4c8b10                   mov r10, qword ptr [rax]
140aef5b8: 488b4010                 mov rax, qword ptr [rax + 0x10]
140aef5bc: 48898424e0010000         mov qword ptr [rsp + 0x1e0], rax
140aef5c4: 488b842458030000         mov rax, qword ptr [rsp + 0x358]
140aef5cc: 448b581c                 mov r11d, dword ptr [rax + 0x1c]
140aef5d0: 44895c2438               mov dword ptr [rsp + 0x38], r11d
140aef5d5: 4c8b18                   mov r11, qword ptr [rax]
140aef5d8: 488b4010                 mov rax, qword ptr [rax + 0x10]
140aef5dc: 4889842498000000         mov qword ptr [rsp + 0x98], rax
140aef5e4: 450f57e4                 xorps xmm12, xmm12
140aef5e8: f3440f10353f04f901       movss xmm14, dword ptr [rip + 0x1f9043f]
140aef5f1: f3440f102dde62e301       movss xmm13, dword ptr [rip + 0x1e362de]
140aef5fa: 486344242c               movsxd rax, dword ptr [rsp + 0x2c]
140aef5ff: 48898424c8010000         mov qword ptr [rsp + 0x1c8], rax
140aef607: 4c898424e8000000         mov qword ptr [rsp + 0xe8], r8
140aef60f: 498d4008                 lea rax, [r8 + 8]
140aef613: 4889442468               mov qword ptr [rsp + 0x68], rax
140aef618: 488d5e10                 lea rbx, [rsi + 0x10]
140aef61c: 48899424b0000000         mov qword ptr [rsp + 0xb0], rdx
140aef624: 488d4220                 lea rax, [rdx + 0x20]
140aef628: 48898424a8010000         mov qword ptr [rsp + 0x1a8], rax
140aef630: 4c894c2450               mov qword ptr [rsp + 0x50], r9
140aef635: 498d4120                 lea rax, [r9 + 0x20]
140aef639: 48898424a8000000         mov qword ptr [rsp + 0xa8], rax
140aef641: 488b442430               mov rax, qword ptr [rsp + 0x30]
140aef646: 488d4020                 lea rax, [rax + 0x20]
140aef64a: 48898424a0000000         mov qword ptr [rsp + 0xa0], rax
140aef652: 4c899424d0000000         mov qword ptr [rsp + 0xd0], r10
140aef65a: 498d4220                 lea rax, [r10 + 0x20]
140aef65e: 48898424c0010000         mov qword ptr [rsp + 0x1c0], rax
140aef666: 4c895c2448               mov qword ptr [rsp + 0x48], r11
140aef66b: 498d4320                 lea rax, [r11 + 0x20]
140aef66f: 4889842490000000         mov qword ptr [rsp + 0x90], rax
140aef677: 488d4128                 lea rax, [rcx + 0x28]
140aef67b: 48898424a0010000         mov qword ptr [rsp + 0x1a0], rax
140aef683: 488b8424c0000000         mov rax, qword ptr [rsp + 0xc0]
140aef68b: 4883c004                 add rax, 4
140aef68f: 48898424b8010000         mov qword ptr [rsp + 0x1b8], rax
140aef697: 4c8db42418020000         lea r14, [rsp + 0x218]
140aef69f: e987000000               jmp 0x140aef72b
140aef6a4: 488b8c24b8010000         mov rcx, qword ptr [rsp + 0x1b8]
140aef6ac: 8b4108                   mov eax, dword ptr [rcx + 8]
140aef6af: 898424f8000000           mov dword ptr [rsp + 0xf8], eax
140aef6b6: 488b01                   mov rax, qword ptr [rcx]
140aef6b9: 48898424f0000000         mov qword ptr [rsp + 0xf0], rax
140aef6c1: 4c89ff                   mov rdi, r15
140aef6c4: 488b8424e0000000         mov rax, qword ptr [rsp + 0xe0]
140aef6cc: 8b4c2444                 mov ecx, dword ptr [rsp + 0x44]
140aef6d0: 8908                     mov dword ptr [rax], ecx
140aef6d2: 8b8424f8000000           mov eax, dword ptr [rsp + 0xf8]
140aef6d9: 488b4c2468               mov rcx, qword ptr [rsp + 0x68]
140aef6de: 894108                   mov dword ptr [rcx + 8], eax
140aef6e1: 488b8424f0000000         mov rax, qword ptr [rsp + 0xf0]
140aef6e9: 488901                   mov qword ptr [rcx], rax
140aef6ec: 8b842488000000           mov eax, dword ptr [rsp + 0x88]
140aef6f3: 488b8c24f0010000         mov rcx, qword ptr [rsp + 0x1f0]
140aef6fb: 8901                     mov dword ptr [rcx], eax
140aef6fd: 488b8424e8010000         mov rax, qword ptr [rsp + 0x1e8]
140aef705: c680e001000001           mov byte ptr [rax + 0x1e0], 1
140aef70c: 488b8424c8000000         mov rax, qword ptr [rsp + 0xc8]
140aef714: c70000000000             mov dword ptr [rax], 0
140aef71a: 48ffcf                   dec rdi
140aef71d: 4c8db42418020000         lea r14, [rsp + 0x218]
140aef725: 0f84e7080000             je 0x140af0012
140aef72b: 664585e4                 test r12w, r12w
140aef72f: 740f                     je 0x140aef740
140aef731: 418d4424ff               lea eax, [r12 - 1]
140aef736: 4421e0                   and eax, r12d
140aef739: eb32                     jmp 0x140aef76d
140aef73b: 0f1f440000               nop dword ptr [rax + rax]
140aef740: 660f6f03                 movdqa xmm0, xmmword ptr [rbx]
140aef744: 66440fd7e0               pmovmskb r12d, xmm0
140aef749: 4881c600fdffff           add rsi, -0x300
140aef750: 4883c310                 add rbx, 0x10
140aef754: 4181fcffff0000           cmp r12d, 0xffff
140aef75b: 74e3                     je 0x140aef740
140aef75d: b9feffffff               mov ecx, 0xfffffffe
140aef762: 4429e1                   sub ecx, r12d
140aef765: 41f7d4                   not r12d
140aef768: 4489e0                   mov eax, r12d
140aef76b: 21c8                     and eax, ecx
140aef76d: 4489e1                   mov ecx, r12d
140aef770: f30fbcc9                 tzcnt ecx, ecx
140aef774: 4189c4                   mov r12d, eax
140aef777: 48f7d9                   neg rcx
140aef77a: 4c8d2c49                 lea r13, [rcx + rcx*2]
140aef77e: 49c1e504                 shl r13, 4
140aef782: 4a8d2c2e                 lea rbp, [rsi + r13]
140aef786: 4883c5d0                 add rbp, -0x30
140aef78a: 4889ac2408020000         mov qword ptr [rsp + 0x208], rbp
140aef792: 488d05c7dd2900           lea rax, [rip + 0x29ddc7]
140aef799: 4889842410020000         mov qword ptr [rsp + 0x210], rax
140aef7a1: 488d056042ff01           lea rax, [rip + 0x1ff4260]
140aef7a8: 48898424f0000000         mov qword ptr [rsp + 0xf0], rax
140aef7b0: 48c78424f800000002000000 mov qword ptr [rsp + 0xf8], 2
140aef7bc: 488d842408020000         lea rax, [rsp + 0x208]
140aef7c4: 4889842400010000         mov qword ptr [rsp + 0x100], rax
140aef7cc: 48c784240801000001000000 mov qword ptr [rsp + 0x108], 1
140aef7d8: 48c784241001000000000000 mov qword ptr [rsp + 0x110], 0
140aef7e4: 4c89f1                   mov rcx, r14
140aef7e7: 488d9424f0000000         lea rdx, [rsp + 0xf0]
140aef7ef: e89ca72800               call 0x140d79f90
140aef7f4: b007                     mov al, 7
140aef7f6: 488b4c2458               mov rcx, qword ptr [rsp + 0x58]
140aef7fb: 803900                   cmp byte ptr [rcx], 0
140aef7fe: 7512                     jne 0x140aef812
140aef800: 31c0                     xor eax, eax
140aef802: 8b4c242c                 mov ecx, dword ptr [rsp + 0x2c]
140aef806: 3b4d00                   cmp ecx, dword ptr [rbp]
140aef809: 0f95c0                   setne al
140aef80c: 8d0440                   lea eax, [rax + rax*2]
140aef80f: 83c004                   add eax, 4
140aef812: 48c78424f000000000000000 mov qword ptr [rsp + 0xf0], 0
140aef81e: 48c78424f800000008000000 mov qword ptr [rsp + 0xf8], 8
140aef82a: 48c784240001000000000000 mov qword ptr [rsp + 0x100], 0
140aef836: c784240801000000000000   mov dword ptr [rsp + 0x108], 0
140aef841: c784242001000000000000   mov dword ptr [rsp + 0x120], 0
140aef84c: c784243801000000000000   mov dword ptr [rsp + 0x138], 0
140aef857: c784245001000000000000   mov dword ptr [rsp + 0x150], 0
140aef862: 0fb64c2470               movzx ecx, byte ptr [rsp + 0x70]
140aef867: 888c2468010000           mov byte ptr [rsp + 0x168], cl
140aef86e: c68424690100000b         mov byte ptr [rsp + 0x169], 0xb
140aef876: 88842478010000           mov byte ptr [rsp + 0x178], al
140aef87d: c684247901000000         mov byte ptr [rsp + 0x179], 0
140aef885: 48b80000024300000243     movabs rax, 0x4302000043020000
140aef88f: 488984247c010000         mov qword ptr [rsp + 0x17c], rax
140aef897: 66c78424980100000001     mov word ptr [rsp + 0x198], 0x100
140aef8a1: 488b4c2478               mov rcx, qword ptr [rsp + 0x78]
140aef8a6: 4c89f2                   mov rdx, r14
140aef8a9: 4c8d8424f0000000         lea r8, [rsp + 0xf0]
140aef8b1: e83a39e3ff               call 0x1409231f0
140aef8b6: 4989c7                   mov r15, rax
140aef8b9: 4989d6                   mov r14, rdx
140aef8bc: 0fb64818                 movzx ecx, byte ptr [rax + 0x18]
140aef8c0: 80f901                   cmp cl, 1
140aef8c3: 0f85b7000000             jne 0x140aef980
140aef8c9: b002                     mov al, 2
140aef8cb: 4d397738                 cmp qword ptr [r15 + 0x38], r14
140aef8cf: 0f85cb000000             jne 0x140aef9a0
140aef8d5: 837c242cff               cmp dword ptr [rsp + 0x2c], -1
140aef8da: 0f94c1                   sete cl
140aef8dd: 410fb65719               movzx edx, byte ptr [r15 + 0x19]
140aef8e2: 80f201                   xor dl, 1
140aef8e5: 08ca                     or dl, cl
140aef8e7: 0a54242b                 or dl, byte ptr [rsp + 0x2b]
140aef8eb: 0f85af000000             jne 0x140aef9a0
140aef8f1: 4889bc24d8010000         mov qword ptr [rsp + 0x1d8], rdi
140aef8f9: 8b842480000000           mov eax, dword ptr [rsp + 0x80]
140aef900: 488b8c24d0010000         mov rcx, qword ptr [rsp + 0x1d0]
140aef908: 8901                     mov dword ptr [rcx], eax
140aef90a: 488b8424b0000000         mov rax, qword ptr [rsp + 0xb0]
140aef912: 488b7830                 mov rdi, qword ptr [rax + 0x30]
140aef916: 4c8b4040                 mov r8, qword ptr [rax + 0x40]
140aef91a: 483b7820                 cmp rdi, qword ptr [rax + 0x20]
140aef91e: 7524                     jne 0x140aef944
140aef920: 488b8c24a8010000         mov rcx, qword ptr [rsp + 0x1a8]
140aef928: 488d1519a81402           lea rdx, [rip + 0x214a819]
140aef92f: 4c898424b0010000         mov qword ptr [rsp + 0x1b0], r8
140aef937: e8f42ca7ff               call 0x140562630
140aef93c: 4c8b8424b0010000         mov r8, qword ptr [rsp + 0x1b0]
140aef944: 488b9424b0000000         mov rdx, qword ptr [rsp + 0xb0]
140aef94c: 488b4228                 mov rax, qword ptr [rdx + 0x28]
140aef950: 4889f9                   mov rcx, rdi
140aef953: 48c1e104                 shl rcx, 4
140aef957: 4c890408                 mov qword ptr [rax + rcx], r8
140aef95b: 4c8b8424c8010000         mov r8, qword ptr [rsp + 0x1c8]
140aef963: 4c89440808               mov qword ptr [rax + rcx + 8], r8
140aef968: 48ffc7                   inc rdi
140aef96b: 48897a30                 mov qword ptr [rdx + 0x30], rdi
140aef96f: 48ff4240                 inc qword ptr [rdx + 0x40]
140aef973: 410fb64f18               movzx ecx, byte ptr [r15 + 0x18]
140aef978: 488bbc24d8010000         mov rdi, qword ptr [rsp + 0x1d8]
140aef980: b002                     mov al, 2
140aef982: 80f902                   cmp cl, 2
140aef985: 7219                     jb 0x140aef9a0
140aef987: 4d397738                 cmp qword ptr [r15 + 0x38], r14
140aef98b: 7513                     jne 0x140aef9a0
140aef98d: 410fb64719               movzx eax, byte ptr [r15 + 0x19]
140aef992: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140aef9a0: 4901f5                   add r13, rsi
140aef9a3: 04fe                     add al, 0xfe
140aef9a5: 0fb6c0                   movzx eax, al
140aef9a8: 3c03                     cmp al, 3
140aef9aa: b903000000               mov ecx, 3
140aef9af: 0f43c1                   cmovae eax, ecx
140aef9b2: 0fb6c0                   movzx eax, al
140aef9b5: 488d0d9046ff01           lea rcx, [rip + 0x1ff4690]
140aef9bc: 48630481                 movsxd rax, dword ptr [rcx + rax*4]
140aef9c0: 4801c8                   add rax, rcx
140aef9c3: ffe0                     jmp rax
140aef9c5: 4989ff                   mov r15, rdi
140aef9c8: 8b442440                 mov eax, dword ptr [rsp + 0x40]
140aef9cc: 488b8c24d8000000         mov rcx, qword ptr [rsp + 0xd8]
140aef9d4: 8901                     mov dword ptr [rcx], eax
140aef9d6: 488b442450               mov rax, qword ptr [rsp + 0x50]
140aef9db: 488b7830                 mov rdi, qword ptr [rax + 0x30]
140aef9df: 4c8b7040                 mov r14, qword ptr [rax + 0x40]
140aef9e3: 483b7820                 cmp rdi, qword ptr [rax + 0x20]
140aef9e7: 7514                     jne 0x140aef9fd
140aef9e9: 488b8c24a8000000         mov rcx, qword ptr [rsp + 0xa8]
140aef9f1: 488d1550a71402           lea rdx, [rip + 0x214a750]
140aef9f8: e8332ca7ff               call 0x140562630
140aef9fd: 488b542450               mov rdx, qword ptr [rsp + 0x50]
140aefa02: 488b4228                 mov rax, qword ptr [rdx + 0x28]
140aefa06: 4889f9                   mov rcx, rdi
140aefa09: 48c1e104                 shl rcx, 4
140aefa0d: 4c893408                 mov qword ptr [rax + rcx], r14
140aefa11: c644080800               mov byte ptr [rax + rcx + 8], 0
140aefa16: 48ffc7                   inc rdi
140aefa19: 48897a30                 mov qword ptr [rdx + 0x30], rdi
140aefa1d: 48ff4240                 inc qword ptr [rdx + 0x40]
140aefa21: f3410f1045f8             movss xmm0, dword ptr [r13 - 8]
140aefa27: 488b8c24e0000000         mov rcx, qword ptr [rsp + 0xe0]
140aefa2f: 8b542444                 mov edx, dword ptr [rsp + 0x44]
140aefa33: 8911                     mov dword ptr [rcx], edx
140aefa35: 488b8424e8000000         mov rax, qword ptr [rsp + 0xe8]
140aefa3d: f30f1100                 movss dword ptr [rax], xmm0
140aefa41: 66410f6e45f0             movd xmm0, dword ptr [r13 - 0x10]
140aefa47: 8911                     mov dword ptr [rcx], edx
140aefa49: 660f7e4004               movd dword ptr [rax + 4], xmm0
140aefa4e: 488b8424c0000000         mov rax, qword ptr [rsp + 0xc0]
140aefa56: f60001                   test byte ptr [rax], 1
140aefa59: 0f8545fcffff             jne 0x140aef6a4
140aefa5f: c78424f800000000000000   mov dword ptr [rsp + 0xf8], 0
140aefa6a: 48c78424f000000000000000 mov qword ptr [rsp + 0xf0], 0
140aefa76: e946fcffff               jmp 0x140aef6c1
140aefa7b: 0f1f440000               nop dword ptr [rax + rax]
140aefa80: 4989fd                   mov r13, rdi
140aefa83: 8b842484000000           mov eax, dword ptr [rsp + 0x84]
140aefa8a: 488b8c24e0010000         mov rcx, qword ptr [rsp + 0x1e0]
140aefa92: 8901                     mov dword ptr [rcx], eax
140aefa94: 488b8424d0000000         mov rax, qword ptr [rsp + 0xd0]
140aefa9c: 488b7830                 mov rdi, qword ptr [rax + 0x30]
140aefaa0: 4c8b7040                 mov r14, qword ptr [rax + 0x40]
140aefaa4: 483b7820                 cmp rdi, qword ptr [rax + 0x20]
140aefaa8: 7514                     jne 0x140aefabe
140aefaaa: 488b8c24c0010000         mov rcx, qword ptr [rsp + 0x1c0]
140aefab2: 488d158fa61402           lea rdx, [rip + 0x214a68f]
140aefab9: e8b22da7ff               call 0x140562870
140aefabe: 488b8c24d0000000         mov rcx, qword ptr [rsp + 0xd0]
140aefac6: 488b4128                 mov rax, qword ptr [rcx + 0x28]
140aefaca: 4c8934f8                 mov qword ptr [rax + rdi*8], r14
140aeface: 48ff4140                 inc qword ptr [rcx + 0x40]
140aefad2: 48ffc7                   inc rdi
140aefad5: 410f2efc                 ucomiss xmm7, xmm12
140aefad9: 48897930                 mov qword ptr [rcx + 0x30], rdi
140aefadd: 4c89ef                   mov rdi, r13
140aefae0: 0f8634fcffff             jbe 0x140aef71a
140aefae6: 450f2ed1                 ucomiss xmm10, xmm9
140aefaea: 0f8626020000             jbe 0x140aefd16
140aefaf0: 448b7500                 mov r14d, dword ptr [rbp]
140aefaf4: 4489b424f0000000         mov dword ptr [rsp + 0xf0], r14d
140aefafc: 488b442460               mov rax, qword ptr [rsp + 0x60]
140aefb01: 4883782000               cmp qword ptr [rax + 0x20], 0
140aefb06: 0f841e030000             je 0x140aefe2a
140aefb0c: 488b8c24a0010000         mov rcx, qword ptr [rsp + 0x1a0]
140aefb14: 488d9424f0000000         lea rdx, [rsp + 0xf0]
140aefb1c: e8af13acff               call 0x1405b0ed0
140aefb21: 488b542460               mov rdx, qword ptr [rsp + 0x60]
140aefb26: 488b4a08                 mov rcx, qword ptr [rdx + 8]
140aefb2a: 488b5210                 mov rdx, qword ptr [rdx + 0x10]
140aefb2e: 4989d0                   mov r8, rdx
140aefb31: 4921c0                   and r8, rax
140aefb34: 48c1e839                 shr rax, 0x39
140aefb38: 660f6ec0                 movd xmm0, eax
140aefb3c: 660f60c0                 punpcklbw xmm0, xmm0
140aefb40: f20f70c000               pshuflw xmm0, xmm0, 0
140aefb45: 660f70c000               pshufd xmm0, xmm0, 0
140aefb4a: 488d41d0                 lea rax, [rcx - 0x30]
140aefb4e: 4531c9                   xor r9d, r9d
140aefb51: f3420f6f0c01             movdqu xmm1, xmmword ptr [rcx + r8]
140aefb57: 660f6fd1                 movdqa xmm2, xmm1
140aefb5b: 660f74d0                 pcmpeqb xmm2, xmm0
140aefb5f: 66440fd7d2               pmovmskb r10d, xmm2
140aefb64: 4585d2                   test r10d, r10d
140aefb67: 742d                     je 0x140aefb96
140aefb69: f3450fbcda               tzcnt r11d, r10d
140aefb6e: 4d01c3                   add r11, r8
140aefb71: 4921d3                   and r11, rdx
140aefb74: 49f7db                   neg r11
140aefb77: 4f8d1c5b                 lea r11, [r11 + r11*2]
140aefb7b: 49c1e304                 shl r11, 4
140aefb7f: 46393418                 cmp dword ptr [rax + r11], r14d
140aefb83: 0f84a9030000             je 0x140aeff32
140aefb89: 458d5aff                 lea r11d, [r10 - 1]
140aefb8d: 664521d3                 and r11w, r10w
140aefb91: 4589da                   mov r10d, r11d
140aefb94: 75d3                     jne 0x140aefb69
140aefb96: 660f740df25de301         pcmpeqb xmm1, xmmword ptr [rip + 0x1e35df2]
140aefb9e: 66440fd7d1               pmovmskb r10d, xmm1
140aefba3: 4585d2                   test r10d, r10d
140aefba6: 0f857e020000             jne 0x140aefe2a
140aefbac: 4d01c8                   add r8, r9
140aefbaf: 4983c010                 add r8, 0x10
140aefbb3: 4983c110                 add r9, 0x10
140aefbb7: 4921d0                   and r8, rdx
140aefbba: eb95                     jmp 0x140aefb51
140aefbbc: 0f1f4000                 nop dword ptr [rax]
140aefbc0: 4989ff                   mov r15, rdi
140aefbc3: 8b442440                 mov eax, dword ptr [rsp + 0x40]
140aefbc7: 488b8c24d8000000         mov rcx, qword ptr [rsp + 0xd8]
140aefbcf: 8901                     mov dword ptr [rcx], eax
140aefbd1: 488b442450               mov rax, qword ptr [rsp + 0x50]
140aefbd6: 488b7830                 mov rdi, qword ptr [rax + 0x30]
140aefbda: 4c8b7040                 mov r14, qword ptr [rax + 0x40]
140aefbde: 483b7820                 cmp rdi, qword ptr [rax + 0x20]
140aefbe2: 7514                     jne 0x140aefbf8
140aefbe4: 488b8c24a8000000         mov rcx, qword ptr [rsp + 0xa8]
140aefbec: 488d1555a51402           lea rdx, [rip + 0x214a555]
140aefbf3: e8382aa7ff               call 0x140562630
140aefbf8: 488b542450               mov rdx, qword ptr [rsp + 0x50]
140aefbfd: 488b4228                 mov rax, qword ptr [rdx + 0x28]
140aefc01: 4889f9                   mov rcx, rdi
140aefc04: 48c1e104                 shl rcx, 4
140aefc08: 4c893408                 mov qword ptr [rax + rcx], r14
140aefc0c: c644080801               mov byte ptr [rax + rcx + 8], 1
140aefc11: 48ffc7                   inc rdi
140aefc14: 48897a30                 mov qword ptr [rdx + 0x30], rdi
140aefc18: 48ff4240                 inc qword ptr [rdx + 0x40]
140aefc1c: 488b8424b8000000         mov rax, qword ptr [rsp + 0xb8]
140aefc24: 8b4c243c                 mov ecx, dword ptr [rsp + 0x3c]
140aefc28: 8908                     mov dword ptr [rax], ecx
140aefc2a: 488b442430               mov rax, qword ptr [rsp + 0x30]
140aefc2f: 488b7830                 mov rdi, qword ptr [rax + 0x30]
140aefc33: 4c8b7040                 mov r14, qword ptr [rax + 0x40]
140aefc37: 483b7820                 cmp rdi, qword ptr [rax + 0x20]
140aefc3b: 7514                     jne 0x140aefc51
140aefc3d: 488b8c24a0000000         mov rcx, qword ptr [rsp + 0xa0]
140aefc45: 488d15fca41402           lea rdx, [rip + 0x214a4fc]
140aefc4c: e8df29a7ff               call 0x140562630
140aefc51: 488b542430               mov rdx, qword ptr [rsp + 0x30]
140aefc56: 488b4228                 mov rax, qword ptr [rdx + 0x28]
140aefc5a: 4889f9                   mov rcx, rdi
140aefc5d: 48c1e104                 shl rcx, 4
140aefc61: 4c893408                 mov qword ptr [rax + rcx], r14
140aefc65: c644080800               mov byte ptr [rax + rcx + 8], 0
140aefc6a: 48ffc7                   inc rdi
140aefc6d: 48897a30                 mov qword ptr [rdx + 0x30], rdi
140aefc71: 48ff4240                 inc qword ptr [rdx + 0x40]
140aefc75: 8b84248c000000           mov eax, dword ptr [rsp + 0x8c]
140aefc7c: 488b8c24f8010000         mov rcx, qword ptr [rsp + 0x1f8]
140aefc84: 8901                     mov dword ptr [rcx], eax
140aefc86: 418b45d0                 mov eax, dword ptr [r13 - 0x30]
140aefc8a: 488b8c24e8000000         mov rcx, qword ptr [rsp + 0xe8]
140aefc92: 660f6e01                 movd xmm0, dword ptr [rcx]
140aefc96: f30f104904               movss xmm1, dword ptr [rcx + 4]
140aefc9b: 66410f6e55f8             movd xmm2, dword ptr [r13 - 8]
140aefca1: f3410f105df0             movss xmm3, dword ptr [r13 - 0x10]
140aefca7: 488b942400020000         mov rdx, qword ptr [rsp + 0x200]
140aefcaf: 488b4a20                 mov rcx, qword ptr [rdx + 0x20]
140aefcb3: 48898c2410010000         mov qword ptr [rsp + 0x110], rcx
140aefcbb: 0f1022                   movups xmm4, xmmword ptr [rdx]
140aefcbe: 0f106a10                 movups xmm5, xmmword ptr [rdx + 0x10]
140aefcc2: 0f29ac2400010000         movaps xmmword ptr [rsp + 0x100], xmm5
140aefcca: 0f29a424f0000000         movaps xmmword ptr [rsp + 0xf0], xmm4
140aefcd2: 48b90100000000000080     movabs rcx, 0x8000000000000001
140aefcdc: 48890a                   mov qword ptr [rdx], rcx
140aefcdf: 894208                   mov dword ptr [rdx + 8], eax
140aefce2: 660f7e420c               movd dword ptr [rdx + 0xc], xmm0
140aefce7: 660f7e5210               movd dword ptr [rdx + 0x10], xmm2
140aefcec: f30f114a14               movss dword ptr [rdx + 0x14], xmm1
140aefcf1: f30f115a18               movss dword ptr [rdx + 0x18], xmm3
140aefcf6: 48b80200000000000080     movabs rax, 0x8000000000000002
140aefd00: 48398424f0000000         cmp qword ptr [rsp + 0xf0], rax
140aefd08: 0f8570030000             jne 0x140af007e
140aefd0e: 4c89ff                   mov rdi, r15
140aefd11: e904faffff               jmp 0x140aef71a
140aefd16: 8b6d00                   mov ebp, dword ptr [rbp]
140aefd19: 488b4c2468               mov rcx, qword ptr [rsp + 0x68]
140aefd1e: 8b4108                   mov eax, dword ptr [rcx + 8]
140aefd21: 488d9424f4000000         lea rdx, [rsp + 0xf4]
140aefd29: 894208                   mov dword ptr [rdx + 8], eax
140aefd2c: 488b01                   mov rax, qword ptr [rcx]
140aefd2f: 488902                   mov qword ptr [rdx], rax
140aefd32: 8b442438                 mov eax, dword ptr [rsp + 0x38]
140aefd36: 488b8c2498000000         mov rcx, qword ptr [rsp + 0x98]
140aefd3e: 8901                     mov dword ptr [rcx], eax
140aefd40: 488b442448               mov rax, qword ptr [rsp + 0x48]
140aefd45: 4c8b7030                 mov r14, qword ptr [rax + 0x30]
140aefd49: 4c8b7840                 mov r15, qword ptr [rax + 0x40]
140aefd4d: 4c3b7020                 cmp r14, qword ptr [rax + 0x20]
140aefd51: 7514                     jne 0x140aefd67
140aefd53: 488b8c2490000000         mov rcx, qword ptr [rsp + 0x90]
140aefd5b: 488d15e6a31402           lea rdx, [rip + 0x214a3e6]
140aefd62: e8492da7ff               call 0x140562ab0
140aefd67: 488b542448               mov rdx, qword ptr [rsp + 0x48]
140aefd6c: 488b4228                 mov rax, qword ptr [rdx + 0x28]
140aefd70: 4b8d0cb6                 lea rcx, [r14 + r14*4]
140aefd74: c704c802000000           mov dword ptr [rax + rcx*8], 2
140aefd7b: f30f1174c804             movss dword ptr [rax + rcx*8 + 4], xmm6
140aefd81: c744c80800000000         mov dword ptr [rax + rcx*8 + 8], 0
140aefd89: 0f108424f0000000         movups xmm0, xmmword ptr [rsp + 0xf0]
140aefd91: 0f1144c80c               movups xmmword ptr [rax + rcx*8 + 0xc], xmm0
140aefd96: 896cc81c                 mov dword ptr [rax + rcx*8 + 0x1c], ebp
140aefd9a: 4c897cc820               mov qword ptr [rax + rcx*8 + 0x20], r15
140aefd9f: 49ffc6                   inc r14
140aefda2: 4c897230                 mov qword ptr [rdx + 0x30], r14
140aefda6: 48ff4240                 inc qword ptr [rdx + 0x40]
140aefdaa: 488b8424c8000000         mov rax, qword ptr [rsp + 0xc8]
140aefdb2: f30f1000                 movss xmm0, dword ptr [rax]
140aefdb6: f3410f58c0               addss xmm0, xmm8
140aefdbb: f30f1100                 movss dword ptr [rax], xmm0
140aefdbf: 410f2ec5                 ucomiss xmm0, xmm13
140aefdc3: 0f8651f9ffff             jbe 0x140aef71a
140aefdc9: 488b8424b8000000         mov rax, qword ptr [rsp + 0xb8]
140aefdd1: 8b4c243c                 mov ecx, dword ptr [rsp + 0x3c]
140aefdd5: 8908                     mov dword ptr [rax], ecx
140aefdd7: 488b442430               mov rax, qword ptr [rsp + 0x30]
140aefddc: 488b7830                 mov rdi, qword ptr [rax + 0x30]
140aefde0: 4c8b7040                 mov r14, qword ptr [rax + 0x40]
140aefde4: 483b7820                 cmp rdi, qword ptr [rax + 0x20]
140aefde8: 7514                     jne 0x140aefdfe
140aefdea: 488b8c24a0000000         mov rcx, qword ptr [rsp + 0xa0]
140aefdf2: 488d154fa31402           lea rdx, [rip + 0x214a34f]
140aefdf9: e83228a7ff               call 0x140562630
140aefdfe: 488b542430               mov rdx, qword ptr [rsp + 0x30]
140aefe03: 488b4228                 mov rax, qword ptr [rdx + 0x28]
140aefe07: 4889f9                   mov rcx, rdi
140aefe0a: 48c1e104                 shl rcx, 4
140aefe0e: 4c893408                 mov qword ptr [rax + rcx], r14
140aefe12: c644080801               mov byte ptr [rax + rcx + 8], 1
140aefe17: 48ffc7                   inc rdi
140aefe1a: 48897a30                 mov qword ptr [rdx + 0x30], rdi
140aefe1e: 48ff4240                 inc qword ptr [rdx + 0x40]
140aefe22: 4c89ef                   mov rdi, r13
140aefe25: e9f0f8ffff               jmp 0x140aef71a
140aefe2a: 488b0577c5b302           mov rax, qword ptr [rip + 0x2b3c577]
140aefe31: 488b00                   mov rax, qword ptr [rax]
140aefe34: 4885c0                   test rax, rax
140aefe37: 0f84ddf8ffff             je 0x140aef71a
140aefe3d: 4889ac2418020000         mov qword ptr [rsp + 0x218], rbp
140aefe45: 488d0594050000           lea rax, [rip + 0x594]
140aefe4c: 4889842420020000         mov qword ptr [rsp + 0x220], rax
140aefe54: 488d0d2d3cff01           lea rcx, [rip + 0x1ff3c2d]
140aefe5b: e820dfb8ff               call 0x14067dd80
140aefe60: f30f6f00                 movdqu xmm0, xmmword ptr [rax]
140aefe64: 8b4010                   mov eax, dword ptr [rax + 0x10]
140aefe67: 48c784242001000001000000 mov qword ptr [rsp + 0x120], 1
140aefe73: 488d15263cff01           lea rdx, [rip + 0x1ff3c26]
140aefe7a: 4889942428010000         mov qword ptr [rsp + 0x128], rdx
140aefe82: 48c78424300100002f000000 mov qword ptr [rsp + 0x130], 0x2f
140aefe8e: 488d0dd33bff01           lea rcx, [rip + 0x1ff3bd3]
140aefe95: 48898c2440010000         mov qword ptr [rsp + 0x140], rcx
140aefe9d: 48c784244801000002000000 mov qword ptr [rsp + 0x148], 2
140aefea9: 488d8c2418020000         lea rcx, [rsp + 0x218]
140aefeb1: 48898c2450010000         mov qword ptr [rsp + 0x150], rcx
140aefeb9: 48c784245801000001000000 mov qword ptr [rsp + 0x158], 1
140aefec5: 48c784246001000000000000 mov qword ptr [rsp + 0x160], 0
140aefed1: 48c78424f000000000000000 mov qword ptr [rsp + 0xf0], 0
140aefedd: 48899424f8000000         mov qword ptr [rsp + 0xf8], rdx
140aefee5: 48c78424000100002f000000 mov qword ptr [rsp + 0x100], 0x2f
140aefef1: 48c784240801000000000000 mov qword ptr [rsp + 0x108], 0
140aefefd: f30f7f842410010000       movdqu xmmword ptr [rsp + 0x110], xmm0
140aeff06: c784243801000001000000   mov dword ptr [rsp + 0x138], 1
140aeff11: 8984243c010000           mov dword ptr [rsp + 0x13c], eax
140aeff18: 488d8c2408020000         lea rcx, [rsp + 0x208]
140aeff20: 488d9424f0000000         lea rdx, [rsp + 0xf0]
140aeff28: e803a42500               call 0x140d4a330
140aeff2d: e9e8f7ffff               jmp 0x140aef71a
140aeff32: f3460f107c19f8           movss xmm15, dword ptr [rcx + r11 - 8]
140aeff39: 8b6d00                   mov ebp, dword ptr [rbp]
140aeff3c: 488b4c2468               mov rcx, qword ptr [rsp + 0x68]
140aeff41: 8b4108                   mov eax, dword ptr [rcx + 8]
140aeff44: 898424f8000000           mov dword ptr [rsp + 0xf8], eax
140aeff4b: 488b01                   mov rax, qword ptr [rcx]
140aeff4e: 48898424f0000000         mov qword ptr [rsp + 0xf0], rax
140aeff56: 8b442438                 mov eax, dword ptr [rsp + 0x38]
140aeff5a: 488b8c2498000000         mov rcx, qword ptr [rsp + 0x98]
140aeff62: 8901                     mov dword ptr [rcx], eax
140aeff64: 488b442448               mov rax, qword ptr [rsp + 0x48]
140aeff69: 4c8b7030                 mov r14, qword ptr [rax + 0x30]
140aeff6d: 4c8b7840                 mov r15, qword ptr [rax + 0x40]
140aeff71: 4c3b7020                 cmp r14, qword ptr [rax + 0x20]
140aeff75: 7514                     jne 0x140aeff8b
140aeff77: 488b8c2490000000         mov rcx, qword ptr [rsp + 0x90]
140aeff7f: 488d15c2a11402           lea rdx, [rip + 0x214a1c2]
140aeff86: e8252ba7ff               call 0x140562ab0
140aeff8b: 410f28c7                 movaps xmm0, xmm15
140aeff8f: f3410f5ec6               divss xmm0, xmm14
140aeff94: f30f59c0                 mulss xmm0, xmm0
140aeff98: f3410f5cc3               subss xmm0, xmm11
140aeff9d: 660fefc9                 pxor xmm1, xmm1
140aeffa1: f30f5fc8                 maxss xmm1, xmm0
140aeffa5: f30f10055759e301         movss xmm0, dword ptr [rip + 0x1e35957]
140aeffad: f30f5dc1                 minss xmm0, xmm1
140aeffb1: f30f51c0                 sqrtss xmm0, xmm0
140aeffb5: f3410f59c6               mulss xmm0, xmm14
140aeffba: f3410f5cc7               subss xmm0, xmm15
140aeffbf: 4c8b442448               mov r8, qword ptr [rsp + 0x48]
140aeffc4: 498b4028                 mov rax, qword ptr [r8 + 0x28]
140aeffc8: 4b8d0cb6                 lea rcx, [r14 + r14*4]
140aeffcc: c704c800000000           mov dword ptr [rax + rcx*8], 0
140aeffd3: c744c80802000000         mov dword ptr [rax + rcx*8 + 8], 2
140aeffdb: f30f1144c80c             movss dword ptr [rax + rcx*8 + 0xc], xmm0
140aeffe1: 488b9424f0000000         mov rdx, qword ptr [rsp + 0xf0]
140aeffe9: 488954c810               mov qword ptr [rax + rcx*8 + 0x10], rdx
140aeffee: 8b9424f8000000           mov edx, dword ptr [rsp + 0xf8]
140aefff5: 8954c818                 mov dword ptr [rax + rcx*8 + 0x18], edx
140aefff9: 896cc81c                 mov dword ptr [rax + rcx*8 + 0x1c], ebp
140aefffd: 4c897cc820               mov qword ptr [rax + rcx*8 + 0x20], r15
140af0002: 49ffc6                   inc r14
140af0005: 4d897030                 mov qword ptr [r8 + 0x30], r14
140af0009: 49ff4040                 inc qword ptr [r8 + 0x40]
140af000d: e908f7ffff               jmp 0x140aef71a
140af0012: 0f28b42430020000         movaps xmm6, xmmword ptr [rsp + 0x230]
140af001a: 0f28bc2440020000         movaps xmm7, xmmword ptr [rsp + 0x240]
140af0022: 440f28842450020000       movaps xmm8, xmmword ptr [rsp + 0x250]
140af002b: 440f288c2460020000       movaps xmm9, xmmword ptr [rsp + 0x260]
140af0034: 440f28942470020000       movaps xmm10, xmmword ptr [rsp + 0x270]
140af003d: 440f289c2480020000       movaps xmm11, xmmword ptr [rsp + 0x280]
140af0046: 440f28a42490020000       movaps xmm12, xmmword ptr [rsp + 0x290]
140af004f: 440f28ac24a0020000       movaps xmm13, xmmword ptr [rsp + 0x2a0]
140af0058: 440f28b424b0020000       movaps xmm14, xmmword ptr [rsp + 0x2b0]
140af0061: 440f28bc24c0020000       movaps xmm15, xmmword ptr [rsp + 0x2c0]
140af006a: 4881c4d8020000           add rsp, 0x2d8
140af0071: 5b                       pop rbx
140af0072: 5d                       pop rbp
140af0073: 5f                       pop rdi
140af0074: 5e                       pop rsi
140af0075: 415c                     pop r12
140af0077: 415d                     pop r13
140af0079: 415e                     pop r14
140af007b: 415f                     pop r15
140af007d: c3                       ret 
140af007e: 488d8c24f0000000         lea rcx, [rsp + 0xf0]
140af0086: e85536ebff               call 0x1409a36e0
140af008b: 488d0d3d3aff01           lea rcx, [rip + 0x1ff3a3d]
140af0092: 4c8d05af3bff01           lea r8, [rip + 0x1ff3baf]
140af0099: ba72010000               mov edx, 0x172
140af009e: e88d93de01               call 0x1428d9430
140af00a3: cc                       int3 
140af00a4: cc                       int3 
140af00a5: cc                       int3 
140af00a6: cc                       int3 
140af00a7: cc                       int3 
140af00a8: cc                       int3 
140af00a9: cc                       int3 
140af00aa: cc                       int3 
140af00ab: cc                       int3 
140af00ac: cc                       int3 
140af00ad: cc                       int3 
140af00ae: cc                       int3 
140af00af: cc                       int3 
140af00b0: 4889d0                   mov rax, rdx
140af00b3: 0fb609                   movzx ecx, byte ptr [rcx]
140af00b6: 488d154b3eff01           lea rdx, [rip + 0x1ff3e4b]
140af00bd: 4c8b04ca                 mov r8, qword ptr [rdx + rcx*8]
140af00c1: 4c8d0d183fff01           lea r9, [rip + 0x1ff3f18]
140af00c8: 49631489                 movsxd rdx, dword ptr [r9 + rcx*4]
140af00cc: 4c01ca                   add rdx, r9
140af00cf: 4889c1                   mov rcx, rax
140af00d2: e9b96a2900               jmp 0x140d86b90
140af00d7: cc                       int3 
140af00d8: cc                       int3 
140af00d9: cc                       int3 
140af00da: cc                       int3 
140af00db: cc                       int3 
140af00dc: cc                       int3 
140af00dd: cc                       int3 
140af00de: cc                       int3 
140af00df: cc                       int3 
140af00e0: 55                       push rbp
140af00e1: 56                       push rsi
140af00e2: 57                       push rdi
140af00e3: 4881ec90000000           sub rsp, 0x90
140af00ea: 488dac2480000000         lea rbp, [rsp + 0x80]
140af00f2: 48c74508feffffff         mov qword ptr [rbp + 8], 0xfffffffffffffffe
140af00fa: 4889d7                   mov rdi, rdx
140af00fd: 4889ce                   mov rsi, rcx
140af0100: 48b80000000000000080     movabs rax, 0x8000000000000000
140af010a: 488945d0                 mov qword ptr [rbp - 0x30], rax
140af010e: 48c745e800000000         mov qword ptr [rbp - 0x18], 0
140af0116: 48c745f800000000         mov qword ptr [rbp - 8], 0
140af011e: 4c8d4a28                 lea r9, [rdx + 0x28]
140af0122: c6450701                 mov byte ptr [rbp + 7], 1
140af0126: 488d15823cff01           lea rdx, [rip + 0x1ff3c82]
140af012d: 488d4dd0                 lea rcx, [rbp - 0x30]
140af0131: 41b80c000000             mov r8d, 0xc
140af0137: e8448faa00               call 0x141599080
140af013c: 4885c0                   test rax, rax
140af013f: 0f85b5000000             jne 0x140af01fa
140af0145: c6450701                 mov byte ptr [rbp + 7], 1
140af0149: 488d156b3cff01           lea rdx, [rip + 0x1ff3c6b]
140af0150: 488d4dd0                 lea rcx, [rbp - 0x30]
140af0154: 41b813000000             mov r8d, 0x13
140af015a: 4989f9                   mov r9, rdi
140af015d: e82e56aa00               call 0x141595790
140af0162: 4885c0                   test rax, rax
140af0165: 0f858f000000             jne 0x140af01fa
140af016b: 4c8d4f18                 lea r9, [rdi + 0x18]
140af016f: c6450701                 mov byte ptr [rbp + 7], 1
140af0173: 488d15543cff01           lea rdx, [rip + 0x1ff3c54]
140af017a: 488d4dd0                 lea rcx, [rbp - 0x30]
140af017e: 41b810000000             mov r8d, 0x10
140af0184: e89758aa00               call 0x141595a20
140af0189: 4885c0                   test rax, rax
140af018c: 756c                     jne 0x140af01fa
140af018e: 4c8d4f1c                 lea r9, [rdi + 0x1c]
140af0192: c6450701                 mov byte ptr [rbp + 7], 1
140af0196: 488d15413cff01           lea rdx, [rip + 0x1ff3c41]
140af019d: 488d4dd0                 lea rcx, [rbp - 0x30]
140af01a1: 41b808000000             mov r8d, 8
140af01a7: e87458aa00               call 0x141595a20
140af01ac: 4885c0                   test rax, rax
140af01af: 7549                     jne 0x140af01fa
140af01b1: 4c8d4f20                 lea r9, [rdi + 0x20]
140af01b5: c6450701                 mov byte ptr [rbp + 7], 1
140af01b9: 488d15263cff01           lea rdx, [rip + 0x1ff3c26]
140af01c0: 488d4dd0                 lea rcx, [rbp - 0x30]
140af01c4: 41b808000000             mov r8d, 8
140af01ca: e85158aa00               call 0x141595a20
140af01cf: 4885c0                   test rax, rax
140af01d2: 7526                     jne 0x140af01fa
140af01d4: 4883c724                 add rdi, 0x24
140af01d8: c6450701                 mov byte ptr [rbp + 7], 1
140af01dc: 488d150b3cff01           lea rdx, [rip + 0x1ff3c0b]
140af01e3: 488d4dd0                 lea rcx, [rbp - 0x30]
140af01e7: 41b80a000000             mov r8d, 0xa
140af01ed: 4989f9                   mov r9, rdi
140af01f0: e82b58aa00               call 0x141595a20
140af01f5: 4885c0                   test rax, rax
140af01f8: 743b                     je 0x140af0235
140af01fa: 488d4de8                 lea rcx, [rbp - 0x18]
140af01fe: 48894608                 mov qword ptr [rsi + 8], rax
140af0202: c60606                   mov byte ptr [rsi], 6
140af0205: e8368ddaff               call 0x140898f40
140af020a: 488b55d0                 mov rdx, qword ptr [rbp - 0x30]
140af020e: 4889d0                   mov rax, rdx
140af0211: 48f7d8                   neg rax
140af0214: 7011                     jo 0x140af0227
140af0216: 730f                     jae 0x140af0227
140af0218: 488b4dd8                 mov rcx, qword ptr [rbp - 0x28]
140af021c: 41b801000000             mov r8d, 1
140af0222: e8f939b2ff               call 0x140613c20
140af0227: 4889f0                   mov rax, rsi
140af022a: 4881c490000000           add rsp, 0x90
140af0231: 5f                       pop rdi
140af0232: 5e                       pop rsi
140af0233: 5d                       pop rbp
140af0234: c3                       ret 
140af0235: 0f1045d0                 movups xmm0, xmmword ptr [rbp - 0x30]
140af0239: 0f104de0                 movups xmm1, xmmword ptr [rbp - 0x20]
140af023d: 0f1055f0                 movups xmm2, xmmword ptr [rbp - 0x10]
140af0241: 0f2955c0                 movaps xmmword ptr [rbp - 0x40], xmm2
140af0245: 0f294db0                 movaps xmmword ptr [rbp - 0x50], xmm1
140af0249: 0f2945a0                 movaps xmmword ptr [rbp - 0x60], xmm0
140af024d: c6450700                 mov byte ptr [rbp + 7], 0
140af0251: 488d55a0                 lea rdx, [rbp - 0x60]
140af0255: 4889f1                   mov rcx, rsi
140af0258: e863620900               call 0x140b864c0
140af025d: ebc8                     jmp 0x140af0227
140af025f: 90                       nop 
140af0260: 4889542410               mov qword ptr [rsp + 0x10], rdx
140af0265: 55                       push rbp
140af0266: 56                       push rsi
140af0267: 57                       push rdi
140af0268: 4883ec20                 sub rsp, 0x20
140af026c: 488daa80000000           lea rbp, [rdx + 0x80]
140af0273: 488b55d0                 mov rdx, qword ptr [rbp - 0x30]
140af0277: 488d045500000000         lea rax, [rdx*2]
140af027f: 4885c0                   test rax, rax
140af0282: 740f                     je 0x140af0293
140af0284: 488b4dd8                 mov rcx, qword ptr [rbp - 0x28]
140af0288: 41b801000000             mov r8d, 1
140af028e: e88d39b2ff               call 0x140613c20
140af0293: 90                       nop 
140af0294: 4883c420                 add rsp, 0x20
140af0298: 5f                       pop rdi
140af0299: 5e                       pop rsi
140af029a: 5d                       pop rbp
140af029b: c3                       ret 
140af029c: 0f1f4000                 nop dword ptr [rax]
140af02a0: 4889542410               mov qword ptr [rsp + 0x10], rdx
140af02a5: 55                       push rbp
140af02a6: 56                       push rsi
140af02a7: 57                       push rdi
140af02a8: 4883ec20                 sub rsp, 0x20
140af02ac: 488daa80000000           lea rbp, [rdx + 0x80]
140af02b3: 807d0700                 cmp byte ptr [rbp + 7], 0
140af02b7: 7508                     jne 0x140af02c1
140af02b9: 4883c420                 add rsp, 0x20
140af02bd: 5f                       pop rdi
140af02be: 5e                       pop rsi
140af02bf: 5d                       pop rbp
140af02c0: c3                       ret 
140af02c1: 488d4dd0                 lea rcx, [rbp - 0x30]
140af02c5: e896d8d6ff               call 0x14085db60
140af02ca: ebed                     jmp 0x140af02b9
140af02cc: cc                       int3 
140af02cd: cc                       int3 
140af02ce: cc                       int3 
140af02cf: cc                       int3 
140af02d0: 4889c8                   mov rax, rcx
140af02d3: 4983c0f8                 add r8, -8
140af02d7: 4983f80b                 cmp r8, 0xb
140af02db: 0f87d4000000             ja 0x140af03b5
140af02e1: 488d0d743dff01           lea rcx, [rip + 0x1ff3d74]
140af02e8: 4e630481                 movsxd r8, dword ptr [rcx + r8*4]
140af02ec: 4901c8                   add r8, rcx
140af02ef: 41ffe0                   jmp r8
140af02f2: 48b965765f7368696674     movabs rcx, 0x74666968735f7665
140af02fc: 48390a                   cmp qword ptr [rdx], rcx
140af02ff: 0f84bb000000             je 0x140af03c0
140af0305: 48b9636f6e7472617374     movabs rcx, 0x74736172746e6f63
140af030f: 48390a                   cmp qword ptr [rdx], rcx
140af0312: 0f859d000000             jne 0x140af03b5
140af0318: c6400104                 mov byte ptr [rax + 1], 4
140af031c: c60000                   mov byte ptr [rax], 0
140af031f: c3                       ret 
140af0320: 48b96279706173735f63     movabs rcx, 0x635f737361707962
140af032a: 48330a                   xor rcx, qword ptr [rdx]
140af032d: 8b5208                   mov edx, dword ptr [rdx + 8]
140af0330: 4881f26f6c6f72           xor rdx, 0x726f6c6f
140af0337: 4809ca                   or rdx, rcx
140af033a: 7579                     jne 0x140af03b5
140af033c: c6400100                 mov byte ptr [rax + 1], 0
140af0340: c60000                   mov byte ptr [rax], 0
140af0343: c3                       ret 
140af0344: 49b87361747572617469     movabs r8, 0x6974617275746173
140af034e: 4c3302                   xor r8, qword ptr [rdx]
140af0351: 0fb75208                 movzx edx, word ptr [rdx + 8]
140af0355: 4881f26f6e0000           xor rdx, 0x6e6f
140af035c: 488d4801                 lea rcx, [rax + 1]
140af0360: 4c09c2                   or rdx, r8
140af0363: 7463                     je 0x140af03c8
140af0365: c60106                   mov byte ptr [rcx], 6
140af0368: c60000                   mov byte ptr [rax], 0
140af036b: c3                       ret 
140af036c: f30f6f02                 movdqu xmm0, xmmword ptr [rdx]
140af0370: 660f740508d6e401         pcmpeqb xmm0, xmmword ptr [rip + 0x1e4d608]
140af0378: 660fd7c8                 pmovmskb ecx, xmm0
140af037c: 81f9ffff0000             cmp ecx, 0xffff
140af0382: 7531                     jne 0x140af03b5
140af0384: c6400102                 mov byte ptr [rax + 1], 2
140af0388: c60000                   mov byte ptr [rax], 0
140af038b: c3                       ret 
140af038c: f30f6f02                 movdqu xmm0, xmmword ptr [rdx]
140af0390: f30f6f4a03               movdqu xmm1, xmmword ptr [rdx + 3]
140af0395: 660f740df3d5e401         pcmpeqb xmm1, xmmword ptr [rip + 0x1e4d5f3]
140af039d: 660f7405fbd5e401         pcmpeqb xmm0, xmmword ptr [rip + 0x1e4d5fb]
140af03a5: 660fdbc1                 pand xmm0, xmm1
140af03a9: 660fd7c8                 pmovmskb ecx, xmm0
140af03ad: 81f9ffff0000             cmp ecx, 0xffff
140af03b3: 741a                     je 0x140af03cf
140af03b5: 488d4801                 lea rcx, [rax + 1]
140af03b9: c60106                   mov byte ptr [rcx], 6
140af03bc: c60000                   mov byte ptr [rax], 0
140af03bf: c3                       ret 
140af03c0: c6400103                 mov byte ptr [rax + 1], 3
140af03c4: c60000                   mov byte ptr [rax], 0
140af03c7: c3                       ret 
140af03c8: c60105                   mov byte ptr [rcx], 5
140af03cb: c60000                   mov byte ptr [rax], 0
140af03ce: c3                       ret 
140af03cf: c6400101                 mov byte ptr [rax + 1], 1
140af03d3: c60000                   mov byte ptr [rax], 0
140af03d6: c3                       ret 
140af03d7: cc                       int3 
140af03d8: cc                       int3 
140af03d9: cc                       int3 
140af03da: cc                       int3 
140af03db: cc                       int3 
140af03dc: cc                       int3 
140af03dd: cc                       int3 
140af03de: cc                       int3 
140af03df: cc                       int3 
140af03e0: 4883ec38                 sub rsp, 0x38
140af03e4: 4889d0                   mov rax, rdx
140af03e7: 48894c2430               mov qword ptr [rsp + 0x30], rcx
140af03ec: 488d0d653aff01           lea rcx, [rip + 0x1ff3a65]
140af03f3: 48894c2420               mov qword ptr [rsp + 0x20], rcx
140af03f8: 488d15793aff01           lea rdx, [rip + 0x1ff3a79]
140af03ff: 4c8d4c2430               lea r9, [rsp + 0x30]
140af0404: 41b80f000000             mov r8d, 0xf
140af040a: 4889c1                   mov rcx, rax
140af040d: e8be6e2900               call 0x140d872d0
140af0412: 90                       nop 
140af0413: 4883c438                 add rsp, 0x38
140af0417: c3                       ret 
140af0418: cc                       int3 
140af0419: cc                       int3 
140af041a: cc                       int3 
140af041b: cc                       int3 
140af041c: cc                       int3 
140af041d: cc                       int3 
140af041e: cc                       int3 
140af041f: cc                       int3 
140af0420: 4883ec38                 sub rsp, 0x38
140af0424: 4889d0                   mov rax, rdx
140af0427: 48894c2430               mov qword ptr [rsp + 0x30], rcx
140af042c: 488d0d553aff01           lea rcx, [rip + 0x1ff3a55]
140af0433: 48894c2420               mov qword ptr [rsp + 0x20], rcx
140af0438: 488d15693aff01           lea rdx, [rip + 0x1ff3a69]
140af043f: 4c8d4c2430               lea r9, [rsp + 0x30]
140af0444: 41b80e000000             mov r8d, 0xe
140af044a: 4889c1                   mov rcx, rax
140af044d: e87e6e2900               call 0x140d872d0
140af0452: 90                       nop 
140af0453: 4883c438                 add rsp, 0x38
140af0457: c3                       ret 
140af0458: cc                       int3 
140af0459: cc                       int3 
140af045a: cc                       int3 
140af045b: cc                       int3 
140af045c: cc                       int3 
140af045d: cc                       int3 
140af045e: cc                       int3 
140af045f: cc                       int3 
140af0460: 56                       push rsi
140af0461: 57                       push rdi
140af0462: 53                       push rbx
140af0463: 4883ec20                 sub rsp, 0x20
140af0467: 4889d6                   mov rsi, rdx
140af046a: 4889cf                   mov rdi, rcx
140af046d: e8ee0d8700               call 0x141361260
140af0472: 4889c3                   mov rbx, rax
140af0475: 4889f9                   mov rcx, rdi
140af0478: 4889f2                   mov rdx, rsi
140af047b: e880d88900               call 0x14138dd00
140af0480: 4889c2                   mov rdx, rax
140af0483: 4889d8                   mov rax, rbx
140af0486: 4883c420                 add rsp, 0x20
140af048a: 5b                       pop rbx
140af048b: 5f                       pop rdi
140af048c: 5e                       pop rsi
140af048d: c3                       ret 
140af048e: cc                       int3 
140af048f: cc                       int3 
140af0490: 55                       push rbp
140af0491: 56                       push rsi
140af0492: 57                       push rdi
140af0493: 53                       push rbx
140af0494: 4881ec98000000           sub rsp, 0x98
140af049b: 488dac2480000000         lea rbp, [rsp + 0x80]
140af04a3: 48c74510feffffff         mov qword ptr [rbp + 0x10], 0xfffffffffffffffe
140af04ab: 4889d6                   mov rsi, rdx
140af04ae: 31c0                     xor eax, eax
140af04b0: 483b8140010000           cmp rax, qword ptr [rcx + 0x140]
140af04b7: 0f818e000000             jno 0x140af054b
140af04bd: 488d8138010000           lea rax, [rcx + 0x138]
140af04c4: 48894508                 mov qword ptr [rbp + 8], rax
140af04c8: 4889cb                   mov rbx, rcx
140af04cb: 4889f1                   mov rcx, rsi
140af04ce: 4889da                   mov rdx, rbx
140af04d1: e8ca6f8500               call 0x1413474a0
140af04d6: 4889c7                   mov rdi, rax
140af04d9: 488d4db8                 lea rcx, [rbp - 0x48]
140af04dd: 4889f2                   mov rdx, rsi
140af04e0: 4989d8                   mov r8, rbx
140af04e3: e848402200               call 0x140d14530
140af04e8: 48897db0                 mov qword ptr [rbp - 0x50], rdi
140af04ec: 488b86a0050000           mov rax, qword ptr [rsi + 0x5a0]
140af04f3: 48894500                 mov qword ptr [rbp], rax
140af04f7: 31c0                     xor eax, eax
140af04f9: 48895df8                 mov qword ptr [rbp - 8], rbx
140af04fd: 483b8340010000           cmp rax, qword ptr [rbx + 0x140]
140af0504: 7009                     jo 0x140af050f
140af0506: 488b4d08                 mov rcx, qword ptr [rbp + 8]
140af050a: e80134afff               call 0x1405e3910
140af050f: 488b45b0                 mov rax, qword ptr [rbp - 0x50]
140af0513: 488b4db8                 mov rcx, qword ptr [rbp - 0x48]
140af0517: 0f1045c0                 movups xmm0, xmmword ptr [rbp - 0x40]
140af051b: 0f104dd0                 movups xmm1, xmmword ptr [rbp - 0x30]
140af051f: 0f1055e0                 movups xmm2, xmmword ptr [rbp - 0x20]
140af0523: 488b5508                 mov rdx, qword ptr [rbp + 8]
140af0527: 0f115230                 movups xmmword ptr [rdx + 0x30], xmm2
140af052b: 0f114a20                 movups xmmword ptr [rdx + 0x20], xmm1
140af052f: 0f114210                 movups xmmword ptr [rdx + 0x10], xmm0
140af0533: 488902                   mov qword ptr [rdx], rax
140af0536: 48894a08                 mov qword ptr [rdx + 8], rcx
140af053a: 488b4df8                 mov rcx, qword ptr [rbp - 8]
140af053e: 488b4500                 mov rax, qword ptr [rbp]
140af0542: 48898178010000           mov qword ptr [rcx + 0x178], rax
140af0549: eb14                     jmp 0x140af055f
140af054b: 488b86a0050000           mov rax, qword ptr [rsi + 0x5a0]
140af0552: 488945f0                 mov qword ptr [rbp - 0x10], rax
140af0556: 48398178010000           cmp qword ptr [rcx + 0x178], rax
140af055d: 751d                     jne 0x140af057c
140af055f: b80050cc3d               mov eax, 0x3dcc5000
140af0564: 0386b0050000             add eax, dword ptr [rsi + 0x5b0]
140af056a: 898130010000             mov dword ptr [rcx + 0x130], eax
140af0570: 4881c498000000           add rsp, 0x98
140af0577: 5b                       pop rbx
140af0578: 5f                       pop rdi
140af0579: 5e                       pop rsi
140af057a: 5d                       pop rbp
140af057b: c3                       ret 
140af057c: 488d059d41ff01           lea rax, [rip + 0x1ff419d]
140af0583: 488945b0                 mov qword ptr [rbp - 0x50], rax
140af0587: 48c745b801000000         mov qword ptr [rbp - 0x48], 1
140af058f: 48c745c008000000         mov qword ptr [rbp - 0x40], 8
140af0597: 0f57c0                   xorps xmm0, xmm0
140af059a: 0f1145c8                 movups xmmword ptr [rbp - 0x38], xmm0
140af059e: 4881c178010000           add rcx, 0x178
140af05a5: 488d05f441ff01           lea rax, [rip + 0x1ff41f4]
140af05ac: 4889442420               mov qword ptr [rsp + 0x20], rax
140af05b1: 4c8d45f0                 lea r8, [rbp - 0x10]
140af05b5: 4c8d4db0                 lea r9, [rbp - 0x50]
140af05b9: 4889ca                   mov rdx, rcx
140af05bc: 31c9                     xor ecx, ecx
140af05be: e81edbdd01               call 0x1428ce0e1
140af05c3: cc                       int3 
140af05c4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140af05d0: 4889542410               mov qword ptr [rsp + 0x10], rdx
140af05d5: 55                       push rbp
140af05d6: 56                       push rsi
140af05d7: 57                       push rdi
140af05d8: 53                       push rbx
140af05d9: 4883ec28                 sub rsp, 0x28
140af05dd: 488daa80000000           lea rbp, [rdx + 0x80]
140af05e4: 0f1045b0                 movups xmm0, xmmword ptr [rbp - 0x50]
140af05e8: 0f104dc0                 movups xmm1, xmmword ptr [rbp - 0x40]
140af05ec: 0f1055d0                 movups xmm2, xmmword ptr [rbp - 0x30]
140af05f0: 0f105de0                 movups xmm3, xmmword ptr [rbp - 0x20]
140af05f4: 488b4508                 mov rax, qword ptr [rbp + 8]
140af05f8: 0f115830                 movups xmmword ptr [rax + 0x30], xmm3
140af05fc: 0f115020                 movups xmmword ptr [rax + 0x20], xmm2
140af0600: 0f114810                 movups xmmword ptr [rax + 0x10], xmm1
140af0604: 0f1100                   movups xmmword ptr [rax], xmm0
140af0607: 488b45f8                 mov rax, qword ptr [rbp - 8]
140af060b: 488b4d00                 mov rcx, qword ptr [rbp]
140af060f: 48898878010000           mov qword ptr [rax + 0x178], rcx
140af0616: 4883c428                 add rsp, 0x28
140af061a: 5b                       pop rbx
140af061b: 5f                       pop rdi
140af061c: 5e                       pop rsi
140af061d: 5d                       pop rbp
140af061e: c3                       ret 
140af061f: cc                       int3 
140af0620: 4157                     push r15
140af0622: 4156                     push r14
140af0624: 4155                     push r13
140af0626: 4154                     push r12
140af0628: 56                       push rsi
140af0629: 57                       push rdi
140af062a: 55                       push rbp
140af062b: 53                       push rbx
140af062c: 4883ec78                 sub rsp, 0x78
140af0630: 4889d7                   mov rdi, rdx
140af0633: 4889ce                   mov rsi, rcx
140af0636: 833901                   cmp dword ptr [rcx], 1
140af0639: 7560                     jne 0x140af069b
140af063b: 488b87a0050000           mov rax, qword ptr [rdi + 0x5a0]
140af0642: 4889442440               mov qword ptr [rsp + 0x40], rax
140af0647: 48394660                 cmp qword ptr [rsi + 0x60], rax
140af064b: 0f8424010000             je 0x140af0775
140af0651: 488d05c840ff01           lea rax, [rip + 0x1ff40c8]
140af0658: 4889442448               mov qword ptr [rsp + 0x48], rax
140af065d: 48c744245001000000       mov qword ptr [rsp + 0x50], 1
140af0666: 48c744245808000000       mov qword ptr [rsp + 0x58], 8
140af066f: 0f57c0                   xorps xmm0, xmm0
140af0672: 0f11442460               movups xmmword ptr [rsp + 0x60], xmm0
140af0677: 4883c660                 add rsi, 0x60
140af067b: 488d051e41ff01           lea rax, [rip + 0x1ff411e]
140af0682: 4889442420               mov qword ptr [rsp + 0x20], rax
140af0687: 4c8d442440               lea r8, [rsp + 0x40]
140af068c: 4c8d4c2448               lea r9, [rsp + 0x48]
140af0691: 31c9                     xor ecx, ecx
140af0693: 4889f2                   mov rdx, rsi
140af0696: e846dadd01               call 0x1428ce0e1
140af069b: 4c8d7668                 lea r14, [rsi + 0x68]
140af069f: 4889f9                   mov rcx, rdi
140af06a2: 4c89f2                   mov rdx, r14
140af06a5: e896688600               call 0x141356f40
140af06aa: 4889442438               mov qword ptr [rsp + 0x38], rax
140af06af: 4889f9                   mov rcx, rdi
140af06b2: 4c89f2                   mov rdx, r14
140af06b5: e8667f8500               call 0x141348620
140af06ba: 4889442430               mov qword ptr [rsp + 0x30], rax
140af06bf: 4889f9                   mov rcx, rdi
140af06c2: 4c89f2                   mov rdx, r14
140af06c5: e8d66d8500               call 0x1413474a0
140af06ca: 4889442428               mov qword ptr [rsp + 0x28], rax
140af06cf: 4889f9                   mov rcx, rdi
140af06d2: 4c89f2                   mov rdx, r14
140af06d5: e896218c00               call 0x1413b2870
140af06da: 4989c5                   mov r13, rax
140af06dd: 4889f9                   mov rcx, rdi
140af06e0: 4c89f2                   mov rdx, r14
140af06e3: e8a86a8600               call 0x141357190
140af06e8: 4889c5                   mov rbp, rax
140af06eb: 4889f9                   mov rcx, rdi
140af06ee: 4c89f2                   mov rdx, r14
140af06f1: e87ab68500               call 0x14134bd70
140af06f6: 4889c3                   mov rbx, rax
140af06f9: 4889f9                   mov rcx, rdi
140af06fc: 4c89f2                   mov rdx, r14
140af06ff: e8ec3a8500               call 0x1413441f0
140af0704: 4989c7                   mov r15, rax
140af0707: 4889f9                   mov rcx, rdi
140af070a: 4c89f2                   mov rdx, r14
140af070d: e85e5c8700               call 0x141366370
140af0712: 4989c4                   mov r12, rax
140af0715: 4889f9                   mov rcx, rdi
140af0718: 4c89f2                   mov rdx, r14
140af071b: e8f07e8800               call 0x141378610
140af0720: 488b8fa0050000           mov rcx, qword ptr [rdi + 0x5a0]
140af0727: 48c70601000000           mov qword ptr [rsi], 1
140af072e: 48c7460800000000         mov qword ptr [rsi + 8], 0
140af0736: 488b542430               mov rdx, qword ptr [rsp + 0x30]
140af073b: 48895610                 mov qword ptr [rsi + 0x10], rdx
