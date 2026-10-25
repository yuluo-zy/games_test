142315050: 4157                     push r15
142315052: 4156                     push r14
142315054: 4155                     push r13
142315056: 4154                     push r12
142315058: 56                       push rsi
142315059: 57                       push rdi
14231505a: 55                       push rbp
14231505b: 53                       push rbx
14231505c: 4881ecc8010000           sub rsp, 0x1c8
142315063: 440f29bc24b0010000       movaps xmmword ptr [rsp + 0x1b0], xmm15
14231506c: 440f29b424a0010000       movaps xmmword ptr [rsp + 0x1a0], xmm14
142315075: 440f29ac2490010000       movaps xmmword ptr [rsp + 0x190], xmm13
14231507e: 440f29a42480010000       movaps xmmword ptr [rsp + 0x180], xmm12
142315087: 440f299c2470010000       movaps xmmword ptr [rsp + 0x170], xmm11
142315090: 440f29942460010000       movaps xmmword ptr [rsp + 0x160], xmm10
142315099: 440f298c2450010000       movaps xmmword ptr [rsp + 0x150], xmm9
1423150a2: 440f29842440010000       movaps xmmword ptr [rsp + 0x140], xmm8
1423150ab: 0f29bc2430010000         movaps xmmword ptr [rsp + 0x130], xmm7
1423150b3: 0f29b42420010000         movaps xmmword ptr [rsp + 0x120], xmm6
1423150bb: 4c89cb                   mov rbx, r9
1423150be: 4d89c6                   mov r14, r8
1423150c1: 4889cf                   mov rdi, rcx
1423150c4: 488bb42448020000         mov rsi, qword ptr [rsp + 0x248]
1423150cc: 4c8bbc2438020000         mov r15, qword ptr [rsp + 0x238]
1423150d4: 488b842440020000         mov rax, qword ptr [rsp + 0x240]
1423150dc: 8b4a1c                   mov ecx, dword ptr [rdx + 0x1c]
1423150df: 4c8b4210                 mov r8, qword ptr [rdx + 0x10]
1423150e3: 418908                   mov dword ptr [r8], ecx
1423150e6: 488b0a                   mov rcx, qword ptr [rdx]
1423150e9: 48894c2478               mov qword ptr [rsp + 0x78], rcx
1423150ee: 488b08                   mov rcx, qword ptr [rax]
1423150f1: e88a8c36fe               call 0x14067dd80
1423150f6: 488d4808                 lea rcx, [rax + 8]
1423150fa: 488d5020                 lea rdx, [rax + 0x20]
1423150fe: 4c8d4010                 lea r8, [rax + 0x10]
142315102: 80b89802000000           cmp byte ptr [rax + 0x298], 0
142315109: 480f45ca                 cmovne rcx, rdx
14231510d: 488d4028                 lea rax, [rax + 0x28]
142315111: 490f44c0                 cmove rax, r8
142315115: 488b00                   mov rax, qword ptr [rax]
142315118: 488b09                   mov rcx, qword ptr [rcx]
14231511b: 48898c24f0000000         mov qword ptr [rsp + 0xf0], rcx
142315123: 48898424f8000000         mov qword ptr [rsp + 0xf8], rax
14231512b: 498b0f                   mov rcx, qword ptr [r15]
14231512e: e84d8c36fe               call 0x14067dd80
142315133: 488d4808                 lea rcx, [rax + 8]
142315137: 488d5020                 lea rdx, [rax + 0x20]
14231513b: 4c8d4010                 lea r8, [rax + 0x10]
14231513f: 80b89802000000           cmp byte ptr [rax + 0x298], 0
142315146: 480f45ca                 cmovne rcx, rdx
14231514a: 488d4028                 lea rax, [rax + 0x28]
14231514e: 490f44c0                 cmove rax, r8
142315152: 488b00                   mov rax, qword ptr [rax]
142315155: 488b09                   mov rcx, qword ptr [rcx]
142315158: 48898c2410010000         mov qword ptr [rsp + 0x110], rcx
142315160: 4889842418010000         mov qword ptr [rsp + 0x118], rax
142315168: 498b16                   mov rdx, qword ptr [r14]
14231516b: f3440f105a28             movss xmm11, dword ptr [rdx + 0x28]
142315171: 488b0f                   mov rcx, qword ptr [rdi]
142315174: 4c8b6708                 mov r12, qword ptr [rdi + 8]
142315178: 448b5f14                 mov r11d, dword ptr [rdi + 0x14]
14231517c: 4c8bb9f8000000           mov r15, qword ptr [rcx + 0xf8]
142315183: 488b8100010000           mov rax, qword ptr [rcx + 0x100]
14231518a: 498d3c87                 lea rdi, [r15 + rax*4]
14231518e: 48898c2480000000         mov qword ptr [rsp + 0x80], rcx
142315196: 0fb68108010000           movzx eax, byte ptr [rcx + 0x108]
14231519d: 8844242f                 mov byte ptr [rsp + 0x2f], al
1423151a1: 488b2e                   mov rbp, qword ptr [rsi]
1423151a4: 488b03                   mov rax, qword ptr [rbx]
1423151a7: 48898424d0000000         mov qword ptr [rsp + 0xd0], rax
1423151af: f3450f12d3               movsldup xmm10, xmm11
1423151b4: b808000000               mov eax, 8
1423151b9: 48898424d8000000         mov qword ptr [rsp + 0xd8], rax
1423151c1: 48c78424a000000000000000 mov qword ptr [rsp + 0xa0], 0
1423151cd: f3440f100d927e6b00       movss xmm9, dword ptr [rip + 0x6b7e92]
1423151d6: 450f57e4                 xorps xmm12, xmm12
1423151da: f30f1035667b7600         movss xmm6, dword ptr [rip + 0x767b66]
1423151e2: f3440f102d19076100       movss xmm13, dword ptr [rip + 0x610719]
1423151eb: f3440f1035e0806b00       movss xmm14, dword ptr [rip + 0x6b80e0]
1423151f4: f3440f103d97f46100       movss xmm15, dword ptr [rip + 0x61f497]
1423151fd: 48c784249800000000000000 mov qword ptr [rsp + 0x98], 0
142315209: 48c744245800000000       mov qword ptr [rsp + 0x58], 0
142315212: 48c744245000000000       mov qword ptr [rsp + 0x50], 0
14231521b: 31db                     xor ebx, ebx
14231521d: 4531c0                   xor r8d, r8d
142315220: 48899424e8000000         mov qword ptr [rsp + 0xe8], rdx
142315228: 44895c2428               mov dword ptr [rsp + 0x28], r11d
14231522d: 4889ac2490000000         mov qword ptr [rsp + 0x90], rbp
142315235: 4c89a424c8000000         mov qword ptr [rsp + 0xc8], r12
14231523d: 4889bc24c0000000         mov qword ptr [rsp + 0xc0], rdi
142315245: eb5e                     jmp 0x1423152a5
142315247: 488b742478               mov rsi, qword ptr [rsp + 0x78]
14231524c: 4889f1                   mov rcx, rsi
14231524f: 4d89c6                   mov r14, r8
142315252: e8890831ff               call 0x141625ae0
142315257: 0f28f8                   movaps xmm7, xmm0
14231525a: f3410f59fe               mulss xmm7, xmm14
14231525f: f3410f58ff               addss xmm7, xmm15
142315264: 4889f1                   mov rcx, rsi
142315267: e8740831ff               call 0x141625ae0
14231526c: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
142315271: 4d89f0                   mov r8, r14
142315274: 448b5c2428               mov r11d, dword ptr [rsp + 0x28]
142315279: 440f28c0                 movaps xmm8, xmm0
14231527d: f3440f5905f27d6b00       mulss xmm8, dword ptr [rip + 0x6b7df2]
142315286: f3450f58c5               addss xmm8, xmm13
14231528b: b201                     mov dl, 1
14231528d: 448919                   mov dword ptr [rcx], r11d
142315290: 41885530                 mov byte ptr [r13 + 0x30], dl
142315294: 41c6453100               mov byte ptr [r13 + 0x31], 0
142315299: f3410f117d34             movss dword ptr [r13 + 0x34], xmm7
14231529f: f3450f114538             movss dword ptr [r13 + 0x38], xmm8
1423152a5: 807c242f00               cmp byte ptr [rsp + 0x2f], 0
1423152aa: 0f8490000000             je 0x142315340
1423152b0: 4939d8                   cmp r8, rbx
1423152b3: 0f85ec010000             jne 0x1423154a5
1423152b9: 0f1f8000000000           nop dword ptr [rax]
1423152c0: 4939ff                   cmp r15, rdi
1423152c3: 0f84010b0000             je 0x142315dca
1423152c9: 418b0f                   mov ecx, dword ptr [r15]
1423152cc: 4983c704                 add r15, 4
1423152d0: 498b8424a8010000         mov rax, qword ptr [r12 + 0x1a8]
1423152d8: 488d0cc9                 lea rcx, [rcx + rcx*8]
1423152dc: 488b5cc810               mov rbx, qword ptr [rax + rcx*8 + 0x10]
1423152e1: 4885db                   test rbx, rbx
1423152e4: 74da                     je 0x1423152c0
1423152e6: 488d0cc8                 lea rcx, [rax + rcx*8]
1423152ea: 488b842480000000         mov rax, qword ptr [rsp + 0x80]
1423152f2: 4c8b8010010000           mov r8, qword ptr [rax + 0x110]
1423152f9: 488b4138                 mov rax, qword ptr [rcx + 0x38]
1423152fd: 488b5140                 mov rdx, qword ptr [rcx + 0x40]
142315301: 4c39c2                   cmp rdx, r8
142315304: 0f86ee000000             jbe 0x1423153f8
14231530a: 4e8b04c0                 mov r8, qword ptr [rax + r8*8]
14231530e: 488b4918                 mov rcx, qword ptr [rcx + 0x18]
142315312: 4d85c0                   test r8, r8
142315315: 0f84e1000000             je 0x1423153fc
14231531b: 49f7d0                   not r8
14231531e: 4f8d0440                 lea r8, [r8 + r8*2]
142315322: 49c1e004                 shl r8, 4
142315326: 4e8b440110               mov r8, qword ptr [rcx + r8 + 0x10]
14231532b: 4c89442458               mov qword ptr [rsp + 0x58], r8
142315330: e9d0000000               jmp 0x142315405
142315335: 66662e0f1f840000000000   nop word ptr cs:[rax + rax]
142315340: 4939d8                   cmp r8, rbx
142315343: 0f8525020000             jne 0x14231556e
142315349: 0f1f8000000000           nop dword ptr [rax]
142315350: 4939ff                   cmp r15, rdi
142315353: 0f84710a0000             je 0x142315dca
142315359: 418b0f                   mov ecx, dword ptr [r15]
14231535c: 4983c704                 add r15, 4
142315360: 498b842400010000         mov rax, qword ptr [r12 + 0x100]
142315368: 488d0c89                 lea rcx, [rcx + rcx*4]
14231536c: 48c1e105                 shl rcx, 5
142315370: 488b5c0858               mov rbx, qword ptr [rax + rcx + 0x58]
142315375: 4885db                   test rbx, rbx
142315378: 74d6                     je 0x142315350
14231537a: 4801c8                   add rax, rcx
14231537d: 8b8894000000             mov ecx, dword ptr [rax + 0x94]
142315383: 4d8b8424a8010000         mov r8, qword ptr [r12 + 0x1a8]
14231538b: 4c8d14c9                 lea r10, [rcx + rcx*8]
14231538f: 4b8d14d0                 lea rdx, [r8 + r10*8]
142315393: 488b8c2480000000         mov rcx, qword ptr [rsp + 0x80]
14231539b: 4c8b8910010000           mov r9, qword ptr [rcx + 0x110]
1423153a2: 4b8b4cd038               mov rcx, qword ptr [r8 + r10*8 + 0x38]
1423153a7: 4f8b44d040               mov r8, qword ptr [r8 + r10*8 + 0x40]
1423153ac: 4d39c8                   cmp r8, r9
1423153af: 0f86f8000000             jbe 0x1423154ad
1423153b5: 4e8b0cc9                 mov r9, qword ptr [rcx + r9*8]
1423153b9: 488b5218                 mov rdx, qword ptr [rdx + 0x18]
1423153bd: 4d85c9                   test r9, r9
1423153c0: 0f84eb000000             je 0x1423154b1
1423153c6: 49f7d1                   not r9
1423153c9: 4f8d0c49                 lea r9, [r9 + r9*2]
1423153cd: 49c1e104                 shl r9, 4
1423153d1: 4e8b4c0a10               mov r9, qword ptr [rdx + r9 + 0x10]
1423153d6: 4c894c2458               mov qword ptr [rsp + 0x58], r9
1423153db: 4c8b8c2480000000         mov r9, qword ptr [rsp + 0x80]
1423153e3: 4d8b8928010000           mov r9, qword ptr [r9 + 0x128]
1423153ea: 4d39c8                   cmp r8, r9
1423153ed: 0f87db000000             ja 0x1423154ce
1423153f3: e9f6000000               jmp 0x1423154ee
1423153f8: 488b4918                 mov rcx, qword ptr [rcx + 0x18]
1423153fc: 48c744245800000000       mov qword ptr [rsp + 0x58], 0
142315405: 4c8b942480000000         mov r10, qword ptr [rsp + 0x80]
14231540d: 4d8b8218010000           mov r8, qword ptr [r10 + 0x118]
142315414: 4d8b8a20010000           mov r9, qword ptr [r10 + 0x120]
14231541b: 4e8b04c0                 mov r8, qword ptr [rax + r8*8]
14231541f: 49f7d0                   not r8
142315422: 4f8d0440                 lea r8, [r8 + r8*2]
142315426: 49c1e004                 shl r8, 4
14231542a: 4a8b740110               mov rsi, qword ptr [rcx + r8 + 0x10]
14231542f: 4889b42498000000         mov qword ptr [rsp + 0x98], rsi
142315437: 4e8b440128               mov r8, qword ptr [rcx + r8 + 0x28]
14231543c: 4c898424b0000000         mov qword ptr [rsp + 0xb0], r8
142315444: 4e8b04c8                 mov r8, qword ptr [rax + r9*8]
142315448: 49f7d0                   not r8
14231544b: 4f8d0440                 lea r8, [r8 + r8*2]
14231544f: 49c1e004                 shl r8, 4
142315453: 4e8b4c0110               mov r9, qword ptr [rcx + r8 + 0x10]
142315458: 4c898c24a0000000         mov qword ptr [rsp + 0xa0], r9
142315460: 4e8b440128               mov r8, qword ptr [rcx + r8 + 0x28]
142315465: 4c898424a8000000         mov qword ptr [rsp + 0xa8], r8
14231546d: 4d8b8228010000           mov r8, qword ptr [r10 + 0x128]
142315474: 4c39c2                   cmp rdx, r8
142315477: 7620                     jbe 0x142315499
142315479: 4a8b04c0                 mov rax, qword ptr [rax + r8*8]
14231547d: 4885c0                   test rax, rax
142315480: 7417                     je 0x142315499
142315482: 48f7d0                   not rax
142315485: 488d0440                 lea rax, [rax + rax*2]
142315489: 48c1e004                 shl rax, 4
14231548d: 488b440110               mov rax, qword ptr [rcx + rax + 0x10]
142315492: 4889442450               mov qword ptr [rsp + 0x50], rax
142315497: eb09                     jmp 0x1423154a2
142315499: 48c744245000000000       mov qword ptr [rsp + 0x50], 0
1423154a2: 4531c0                   xor r8d, r8d
1423154a5: 4589c6                   mov r14d, r8d
1423154a8: e9d5000000               jmp 0x142315582
1423154ad: 488b5218                 mov rdx, qword ptr [rdx + 0x18]
1423154b1: 48c744245800000000       mov qword ptr [rsp + 0x58], 0
1423154ba: 4c8b8c2480000000         mov r9, qword ptr [rsp + 0x80]
1423154c2: 4d8b8928010000           mov r9, qword ptr [r9 + 0x128]
1423154c9: 4d39c8                   cmp r8, r9
1423154cc: 7620                     jbe 0x1423154ee
1423154ce: 4e8b04c9                 mov r8, qword ptr [rcx + r9*8]
1423154d2: 4d85c0                   test r8, r8
1423154d5: 7417                     je 0x1423154ee
1423154d7: 49f7d0                   not r8
1423154da: 4f8d0440                 lea r8, [r8 + r8*2]
1423154de: 49c1e004                 shl r8, 4
1423154e2: 4e8b440210               mov r8, qword ptr [rdx + r8 + 0x10]
1423154e7: 4c89442450               mov qword ptr [rsp + 0x50], r8
1423154ec: eb09                     jmp 0x1423154f7
1423154ee: 48c744245000000000       mov qword ptr [rsp + 0x50], 0
1423154f7: 4c8b8c2480000000         mov r9, qword ptr [rsp + 0x80]
1423154ff: 4d8b8118010000           mov r8, qword ptr [r9 + 0x118]
142315506: 4d8b8920010000           mov r9, qword ptr [r9 + 0x120]
14231550d: 4e8b04c1                 mov r8, qword ptr [rcx + r8*8]
142315511: 49f7d0                   not r8
142315514: 4f8d0440                 lea r8, [r8 + r8*2]
142315518: 49c1e004                 shl r8, 4
14231551c: 4e8b540210               mov r10, qword ptr [rdx + r8 + 0x10]
142315521: 4c89942498000000         mov qword ptr [rsp + 0x98], r10
142315529: 4e8b440228               mov r8, qword ptr [rdx + r8 + 0x28]
14231552e: 4c898424b0000000         mov qword ptr [rsp + 0xb0], r8
142315536: 4a8b0cc9                 mov rcx, qword ptr [rcx + r9*8]
14231553a: 48f7d1                   not rcx
14231553d: 488d0c49                 lea rcx, [rcx + rcx*2]
142315541: 48c1e104                 shl rcx, 4
142315545: 4c8b440a10               mov r8, qword ptr [rdx + rcx + 0x10]
14231554a: 4c898424a0000000         mov qword ptr [rsp + 0xa0], r8
142315552: 488b4c0a28               mov rcx, qword ptr [rdx + rcx + 0x28]
142315557: 48898c24a8000000         mov qword ptr [rsp + 0xa8], rcx
14231555f: 488b4050                 mov rax, qword ptr [rax + 0x50]
142315563: 48898424d8000000         mov qword ptr [rsp + 0xd8], rax
14231556b: 4531c0                   xor r8d, r8d
14231556e: 4c89c0                   mov rax, r8
142315571: 48c1e004                 shl rax, 4
142315575: 488b8c24d8000000         mov rcx, qword ptr [rsp + 0xd8]
14231557d: 448b740108               mov r14d, dword ptr [rcx + rax + 8]
142315582: 4b8d3476                 lea rsi, [r14 + r14*2]
142315586: 48c1e604                 shl rsi, 4
14231558a: 4803742458               add rsi, qword ptr [rsp + 0x58]
14231558f: 4d6bee68                 imul r13, r14, 0x68
142315593: 4c03ac2498000000         add r13, qword ptr [rsp + 0x98]
14231559b: 488b8424b0000000         mov rax, qword ptr [rsp + 0xb0]
1423155a3: 4a8d0cb0                 lea rcx, [rax + r14*4]
1423155a7: 488b8424a8000000         mov rax, qword ptr [rsp + 0xa8]
1423155af: 4a8d04b0                 lea rax, [rax + r14*4]
1423155b3: 4889842488000000         mov qword ptr [rsp + 0x88], rax
1423155bb: 488b442450               mov rax, qword ptr [rsp + 0x50]
1423155c0: 4a8d04f0                 lea rax, [rax + r14*8]
1423155c4: 48898424e0000000         mov qword ptr [rsp + 0xe0], rax
1423155cc: 49c1e604                 shl r14, 4
1423155d0: 4c03b424a0000000         add r14, qword ptr [rsp + 0xa0]
1423155d8: 448919                   mov dword ptr [rcx], r11d
1423155db: 418b4508                 mov eax, dword ptr [r13 + 8]
1423155df: b202                     mov dl, 2
1423155e1: 898424bc000000           mov dword ptr [rsp + 0xbc], eax
1423155e8: 83f801                   cmp eax, 1
1423155eb: 48894c2440               mov qword ptr [rsp + 0x40], rcx
1423155f0: 0f85dc010000             jne 0x1423157d2
1423155f6: 4c89442460               mov qword ptr [rsp + 0x60], r8
1423155fb: 4889e9                   mov rcx, rbp
1423155fe: e82dbc5ffe               call 0x140911230
142315603: 3c03                     cmp al, 3
142315605: 7554                     jne 0x14231565b
142315607: 488d4c2468               lea rcx, [rsp + 0x68]
14231560c: 4889ea                   mov rdx, rbp
14231560f: e8bcc75ffe               call 0x140911dd0
142315614: 4c8d642430               lea r12, [rsp + 0x30]
142315619: 4c89e1                   mov rcx, r12
14231561c: 4889ea                   mov rdx, rbp
14231561f: e8ccc75ffe               call 0x140911df0
142315624: f30f10442438             movss xmm0, dword ptr [rsp + 0x38]
14231562a: f30f58c0                 addss xmm0, xmm0
14231562e: f20f104c2430             movsd xmm1, qword ptr [rsp + 0x30]
142315634: 0f58c9                   addps xmm1, xmm1
142315637: f20f10542468             movsd xmm2, qword ptr [rsp + 0x68]
14231563d: 0f58d1                   addps xmm2, xmm1
142315640: f30f58442470             addss xmm0, dword ptr [rsp + 0x70]
142315646: 0f13542430               movlps qword ptr [rsp + 0x30], xmm2
14231564b: f30f11442438             movss dword ptr [rsp + 0x38], xmm0
142315651: 488bac24d0000000         mov rbp, qword ptr [rsp + 0xd0]
142315659: eb18                     jmp 0x142315673
14231565b: 4c8d642430               lea r12, [rsp + 0x30]
142315660: 4c89e1                   mov rcx, r12
142315663: 488bac24d0000000         mov rbp, qword ptr [rsp + 0xd0]
14231566b: 4889ea                   mov rdx, rbp
14231566e: e8ddcf6ffe               call 0x140a12650
142315673: 8b442438                 mov eax, dword ptr [rsp + 0x38]
142315677: 89442470                 mov dword ptr [rsp + 0x70], eax
14231567b: 488b442430               mov rax, qword ptr [rsp + 0x30]
142315680: 4889442468               mov qword ptr [rsp + 0x68], rax
142315685: 8b442428                 mov eax, dword ptr [rsp + 0x28]
142315689: 488b8c2488000000         mov rcx, qword ptr [rsp + 0x88]
142315691: 8901                     mov dword ptr [rcx], eax
142315693: 41c70601000000           mov dword ptr [r14], 1
14231569a: 488b442468               mov rax, qword ptr [rsp + 0x68]
14231569f: 49894604                 mov qword ptr [r14 + 4], rax
1423156a3: 8b442470                 mov eax, dword ptr [rsp + 0x70]
1423156a7: 4189460c                 mov dword ptr [r14 + 0xc], eax
1423156ab: f3410f10450c             movss xmm0, dword ptr [r13 + 0xc]
1423156b1: 488bbc24e8000000         mov rdi, qword ptr [rsp + 0xe8]
1423156b9: f30f5c4728               subss xmm0, dword ptr [rdi + 0x28]
1423156be: f3410f11450c             movss dword ptr [r13 + 0xc], xmm0
1423156c4: 4c89e1                   mov rcx, r12
1423156c7: 4889ea                   mov rdx, rbp
1423156ca: e881cf6ffe               call 0x140a12650
1423156cf: f30f10442434             movss xmm0, dword ptr [rsp + 0x34]
1423156d5: f30f5c4614               subss xmm0, dword ptr [rsi + 0x14]
1423156da: f30f59c0                 mulss xmm0, xmm0
1423156de: f30f104c2438             movss xmm1, dword ptr [rsp + 0x38]
1423156e4: f30f10542430             movss xmm2, dword ptr [rsp + 0x30]
1423156ea: 0f14d1                   unpcklps xmm2, xmm1
1423156ed: f30f104e10               movss xmm1, dword ptr [rsi + 0x10]
1423156f2: f30f105e18               movss xmm3, dword ptr [rsi + 0x18]
1423156f7: 0f14cb                   unpcklps xmm1, xmm3
1423156fa: 0f5cd1                   subps xmm2, xmm1
1423156fd: 0f59d2                   mulps xmm2, xmm2
142315700: f30f58c2                 addss xmm0, xmm2
142315704: f30f16ca                 movshdup xmm1, xmm2
142315708: f30f58c8                 addss xmm1, xmm0
14231570c: 0f57c0                   xorps xmm0, xmm0
14231570f: f30f51c1                 sqrtss xmm0, xmm1
142315713: 440f2ec8                 ucomiss xmm9, xmm0
142315717: 761b                     jbe 0x142315734
142315719: f30f104728               movss xmm0, dword ptr [rdi + 0x28]
14231571e: f30f5905c2017c00         mulss xmm0, dword ptr [rip + 0x7c01c2]
142315726: f3410f58450c             addss xmm0, dword ptr [r13 + 0xc]
14231572c: f3410f11450c             movss dword ptr [r13 + 0xc], xmm0
142315732: eb06                     jmp 0x14231573a
142315734: f3410f10450c             movss xmm0, dword ptr [r13 + 0xc]
14231573a: 4c8ba424c8000000         mov r12, qword ptr [rsp + 0xc8]
142315742: 488bbc24c0000000         mov rdi, qword ptr [rsp + 0xc0]
14231574a: 488b442440               mov rax, qword ptr [rsp + 0x40]
14231574f: 440f2ee0                 ucomiss xmm12, xmm0
142315753: 7630                     jbe 0x142315785
142315755: 8b6c2428                 mov ebp, dword ptr [rsp + 0x28]
142315759: 8928                     mov dword ptr [rax], ebp
14231575b: 41c7450800000000         mov dword ptr [r13 + 8], 0
142315763: 488b4c2478               mov rcx, qword ptr [rsp + 0x78]
142315768: e8730331ff               call 0x141625ae0
14231576d: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
142315772: 4189eb                   mov r11d, ebp
142315775: 0f28f8                   movaps xmm7, xmm0
142315778: f30f59fe                 mulss xmm7, xmm6
14231577c: f3410f58fd               addss xmm7, xmm13
142315781: 31d2                     xor edx, edx
142315783: eb40                     jmp 0x1423157c5
142315785: 488b6c2478               mov rbp, qword ptr [rsp + 0x78]
14231578a: 4889e9                   mov rcx, rbp
14231578d: e84e0331ff               call 0x141625ae0
142315792: 0f28f8                   movaps xmm7, xmm0
142315795: f3410f59fe               mulss xmm7, xmm14
14231579a: f3410f58ff               addss xmm7, xmm15
14231579f: 4889e9                   mov rcx, rbp
1423157a2: e8390331ff               call 0x141625ae0
1423157a7: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
1423157ac: 440f28c0                 movaps xmm8, xmm0
1423157b0: f3440f5905bf786b00       mulss xmm8, dword ptr [rip + 0x6b78bf]
1423157b9: f3450f58c5               addss xmm8, xmm13
1423157be: b201                     mov dl, 1
1423157c0: 448b5c2428               mov r11d, dword ptr [rsp + 0x28]
1423157c5: 488bac2490000000         mov rbp, qword ptr [rsp + 0x90]
1423157cd: 4c8b442460               mov r8, qword ptr [rsp + 0x60]
1423157d2: 49ffc0                   inc r8
1423157d5: 448919                   mov dword ptr [rcx], r11d
1423157d8: 41807d3000               cmp byte ptr [r13 + 0x30], 0
1423157dd: 7431                     je 0x142315810
1423157df: f2410f104534             movsd xmm0, qword ptr [r13 + 0x34]
1423157e5: 410f5cc2                 subps xmm0, xmm10
1423157e9: 410f134534               movlps qword ptr [r13 + 0x34], xmm0
1423157ee: 440f2ee0                 ucomiss xmm12, xmm0
1423157f2: f30f16c0                 movshdup xmm0, xmm0
1423157f6: 7334                     jae 0x14231582c
1423157f8: 440f2ee0                 ucomiss xmm12, xmm0
1423157fc: 724b                     jb 0x142315849
1423157fe: 410fb64531               movzx eax, byte ptr [r13 + 0x31]
142315803: eb3c                     jmp 0x142315841
142315805: 66662e0f1f840000000000   nop word ptr cs:[rax + rax]
142315810: f3410f104534             movss xmm0, dword ptr [r13 + 0x34]
142315816: f3410f5cc3               subss xmm0, xmm11
14231581b: f3410f114534             movss dword ptr [r13 + 0x34], xmm0
142315821: 440f2ee0                 ucomiss xmm12, xmm0
142315825: 7269                     jb 0x142315890
142315827: e91bfaffff               jmp 0x142315247
14231582c: 410fb64531               movzx eax, byte ptr [r13 + 0x31]
142315831: 440f2ee0                 ucomiss xmm12, xmm0
142315835: 7306                     jae 0x14231583d
142315837: 84c0                     test al, al
142315839: 750e                     jne 0x142315849
14231583b: eb66                     jmp 0x1423158a3
14231583d: 84c0                     test al, al
14231583f: 7462                     je 0x1423158a3
142315841: 84c0                     test al, al
142315843: 0f8487000000             je 0x1423158d0
142315849: 418b06                   mov eax, dword ptr [r14]
14231584c: a801                     test al, 1
14231584e: 0f847f000000             je 0x1423158d3
142315854: f3410f104604             movss xmm0, dword ptr [r14 + 4]
14231585a: f3410f104e0c             movss xmm1, dword ptr [r14 + 0xc]
142315860: 0f14c8                   unpcklps xmm1, xmm0
142315863: f30f104610               movss xmm0, dword ptr [rsi + 0x10]
142315868: f30f105618               movss xmm2, dword ptr [rsi + 0x18]
14231586d: 0f14d0                   unpcklps xmm2, xmm0
142315870: 0f5cca                   subps xmm1, xmm2
142315873: 0f59c9                   mulps xmm1, xmm1
142315876: f30f16c1                 movshdup xmm0, xmm1
14231587a: f30f58c1                 addss xmm0, xmm1
14231587e: f30f100d226f9000         movss xmm1, dword ptr [rip + 0x906f22]
142315886: 0f2ec8                   ucomiss xmm1, xmm0
142315889: 7748                     ja 0x1423158d3
14231588b: 0f1f440000               nop dword ptr [rax + rax]
142315890: 80bc24bc00000000         cmp byte ptr [rsp + 0xbc], 0
142315898: 0f8407faffff             je 0x1423152a5
14231589e: e9eaf9ffff               jmp 0x14231528d
1423158a3: 488b4c2478               mov rcx, qword ptr [rsp + 0x78]
1423158a8: 4c89c6                   mov rsi, r8
1423158ab: e8300231ff               call 0x141625ae0
1423158b0: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
1423158b5: 4989f0                   mov r8, rsi
1423158b8: 448b5c2428               mov r11d, dword ptr [rsp + 0x28]
1423158bd: 0f28f8                   movaps xmm7, xmm0
1423158c0: f30f59fe                 mulss xmm7, xmm6
1423158c4: f3410f58fd               addss xmm7, xmm13
1423158c9: 31d2                     xor edx, edx
1423158cb: e9bdf9ffff               jmp 0x14231528d
1423158d0: 418b06                   mov eax, dword ptr [r14]
1423158d3: 8954244c                 mov dword ptr [rsp + 0x4c], edx
1423158d7: 4c89442460               mov qword ptr [rsp + 0x60], r8
1423158dc: 450f28fb                 movaps xmm15, xmm11
1423158e0: 450f28f2                 movaps xmm14, xmm10
1423158e4: 440f28e6                 movaps xmm12, xmm6
1423158e8: 410f28f1                 movaps xmm6, xmm9
1423158ec: 31c9                     xor ecx, ecx
1423158ee: 85c0                     test eax, eax
1423158f0: 0f94c1                   sete cl
1423158f3: 488d051e54d300           lea rax, [rip + 0xd3541e]
1423158fa: f3440f100c88             movss xmm9, dword ptr [rax + rcx*4]
142315900: 488b6c2478               mov rbp, qword ptr [rsp + 0x78]
142315905: 4889e9                   mov rcx, rbp
142315908: e8d30131ff               call 0x141625ae0
14231590d: f30f590563776b00         mulss xmm0, dword ptr [rip + 0x6b7763]
142315915: f3410f58c5               addss xmm0, xmm13
14231591a: f3410f114538             movss dword ptr [r13 + 0x38], xmm0
142315920: 4889e9                   mov rcx, rbp
142315923: e8b80131ff               call 0x141625ae0
142315928: 440f28d0                 movaps xmm10, xmm0
14231592c: 4889e9                   mov rcx, rbp
14231592f: e8ac0131ff               call 0x141625ae0
142315934: 66440f7ed1               movd ecx, xmm10
142315939: 660f7ec0                 movd eax, xmm0
14231593d: 48c1e020                 shl rax, 0x20
142315941: 4809c1                   or rcx, rax
142315944: e8b7ad97fe               call 0x140c90700
142315949: f30f11442430             movss dword ptr [rsp + 0x30], xmm0
14231594f: f30f114c2434             movss dword ptr [rsp + 0x34], xmm1
142315955: 488d8c2400010000         lea rcx, [rsp + 0x100]
14231595d: 488d6c2430               lea rbp, [rsp + 0x30]
142315962: 4889ea                   mov rdx, rbp
142315965: e8567498fe               call 0x140c9cdc0
14231596a: 0f2806                   movaps xmm0, xmmword ptr [rsi]
14231596d: 0f28d0                   movaps xmm2, xmm0
142315970: 0f59d0                   mulps xmm2, xmm0
142315973: f30f16ca                 movshdup xmm1, xmm2
142315977: f30f58ca                 addss xmm1, xmm2
14231597b: 0f28da                   movaps xmm3, xmm2
14231597e: 660f15da                 unpckhpd xmm3, xmm2
142315982: f30f58d9                 addss xmm3, xmm1
142315986: 0fc6db00                 shufps xmm3, xmm3, 0
14231598a: 0fc6d2ff                 shufps xmm2, xmm2, 0xff
14231598e: 0f5cd3                   subps xmm2, xmm3
142315991: 0f281de8366900           movaps xmm3, xmmword ptr [rip + 0x6936e8]
142315998: 0f59d3                   mulps xmm2, xmm3
14231599b: 0f28c8                   movaps xmm1, xmm0
14231599e: 0f59cb                   mulps xmm1, xmm3
1423159a1: f30f16d9                 movshdup xmm3, xmm1
1423159a5: f30f58d9                 addss xmm3, xmm1
1423159a9: 0f12c9                   movhlps xmm1, xmm1
1423159ac: f30f58cb                 addss xmm1, xmm3
1423159b0: f30f58c9                 addss xmm1, xmm1
1423159b4: 0fc6c900                 shufps xmm1, xmm1, 0
1423159b8: 0f59c8                   mulps xmm1, xmm0
1423159bb: 0f58ca                   addps xmm1, xmm2
1423159be: 0f28d0                   movaps xmm2, xmm0
1423159c1: 0fc6d0d2                 shufps xmm2, xmm0, 0xd2
1423159c5: 0f5915c4366900           mulps xmm2, xmmword ptr [rip + 0x6936c4]
1423159cc: 0f28d8                   movaps xmm3, xmm0
1423159cf: 0fc6d8c9                 shufps xmm3, xmm0, 0xc9
1423159d3: 0f591dc6366900           mulps xmm3, xmmword ptr [rip + 0x6936c6]
1423159da: 0f5cda                   subps xmm3, xmm2
1423159dd: 0f58c0                   addps xmm0, xmm0
1423159e0: 0fc6c0ff                 shufps xmm0, xmm0, 0xff
1423159e4: 0f59c3                   mulps xmm0, xmm3
1423159e7: 0f58c1                   addps xmm0, xmm1
1423159ea: 0f28c8                   movaps xmm1, xmm0
1423159ed: 660f15c8                 unpckhpd xmm1, xmm0
1423159f1: f30f590d77fe6000         mulss xmm1, dword ptr [rip + 0x60fe77]
1423159f9: 0f5905e0fe6000           mulps xmm0, xmmword ptr [rip + 0x60fee0]
142315a00: 0f13442430               movlps qword ptr [rsp + 0x30], xmm0
142315a05: f30f114c2438             movss dword ptr [rsp + 0x38], xmm1
142315a0b: 488d4c2468               lea rcx, [rsp + 0x68]
142315a10: 4889ea                   mov rdx, rbp
142315a13: e818a697fe               call 0x140c90030
142315a18: f3410f12c1               movsldup xmm0, xmm9
142315a1d: f3440f594c2470           mulss xmm9, dword ptr [rsp + 0x70]
142315a24: f20f104c2468             movsd xmm1, qword ptr [rsp + 0x68]
142315a2a: 0f59c8                   mulps xmm1, xmm0
142315a2d: f20f10842400010000       movsd xmm0, qword ptr [rsp + 0x100]
142315a36: f3440f588c2408010000     addss xmm9, dword ptr [rsp + 0x108]
142315a40: 0f58c1                   addps xmm0, xmm1
142315a43: 0f28c8                   movaps xmm1, xmm0
142315a46: 0f59c8                   mulps xmm1, xmm0
142315a49: f30f16d1                 movshdup xmm2, xmm1
142315a4d: f30f58d1                 addss xmm2, xmm1
142315a51: 410f28c9                 movaps xmm1, xmm9
142315a55: f3410f59c9               mulss xmm1, xmm9
142315a5a: f30f58ca                 addss xmm1, xmm2
142315a5e: 0f57d2                   xorps xmm2, xmm2
142315a61: f30f51d1                 sqrtss xmm2, xmm1
142315a65: 410f28cd                 movaps xmm1, xmm13
142315a69: f30f5eca                 divss xmm1, xmm2
142315a6d: 660f7ec8                 movd eax, xmm1
142315a71: 85c0                     test eax, eax
142315a73: 0f99c1                   setns cl
142315a76: 89c2                     mov edx, eax
142315a78: 81e2ffffff7f             and edx, 0x7fffffff
142315a7e: 81c2000080ff             add edx, 0xff800000
142315a84: 81fa0000007f             cmp edx, 0x7f000000
142315a8a: 0f92c2                   setb dl
142315a8d: 20ca                     and dl, cl
142315a8f: ffc8                     dec eax
142315a91: 3dffff7f00               cmp eax, 0x7fffff
142315a96: 0f92c0                   setb al
142315a99: 450f28dd                 movaps xmm11, xmm13
142315a9d: 08d0                     or al, dl
142315a9f: 7506                     jne 0x142315aa7
142315aa1: 450f57c9                 xorps xmm9, xmm9
142315aa5: eb05                     jmp 0x142315aac
142315aa7: f3440f59c9               mulss xmm9, xmm1
142315aac: f30f12c9                 movsldup xmm1, xmm1
142315ab0: 0f59c1                   mulps xmm0, xmm1
142315ab3: 0fb6c0                   movzx eax, al
142315ab6: 660f6ec8                 movd xmm1, eax
142315aba: 660f70d150               pshufd xmm2, xmm1, 0x50
142315abf: 660f72f21f               pslld xmm2, 0x1f
142315ac4: 660f72e21f               psrad xmm2, 0x1f
142315ac9: 660fdbd0                 pand xmm2, xmm0
142315acd: f3450f106d38             movss xmm13, dword ptr [r13 + 0x38]
142315ad3: f20f104610               movsd xmm0, qword ptr [rsi + 0x10]
142315ad8: f30f104e18               movss xmm1, dword ptr [rsi + 0x18]
142315add: 488b05b4653101           mov rax, qword ptr [rip + 0x13165b4]
142315ae4: 803800                   cmp byte ptr [rax], 0
142315ae7: 448b5c2428               mov r11d, dword ptr [rsp + 0x28]
142315aec: 488bac2490000000         mov rbp, qword ptr [rsp + 0x90]
142315af4: 4c8b442460               mov r8, qword ptr [rsp + 0x60]
142315af9: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
142315afe: 8b54244c                 mov edx, dword ptr [rsp + 0x4c]
142315b02: 0f84ed000000             je 0x142315bf5
142315b08: 0f28d8                   movaps xmm3, xmm0
142315b0b: 0f59d8                   mulps xmm3, xmm0
142315b0e: f30f16e3                 movshdup xmm4, xmm3
142315b12: f30f58e3                 addss xmm4, xmm3
142315b16: 0f28d9                   movaps xmm3, xmm1
142315b19: f30f59d9                 mulss xmm3, xmm1
142315b1d: f30f58dc                 addss xmm3, xmm4
142315b21: f30f51db                 sqrtss xmm3, xmm3
142315b25: 0f2e1df451d300           ucomiss xmm3, dword ptr [rip + 0xd351f4]
142315b2c: 0f86c3000000             jbe 0x142315bf5
142315b32: 488b6c2478               mov rbp, qword ptr [rsp + 0x78]
142315b37: 4889e9                   mov rcx, rbp
142315b3a: e8a1ff30ff               call 0x141625ae0
142315b3f: 440f28c8                 movaps xmm9, xmm0
142315b43: 4889e9                   mov rcx, rbp
142315b46: 488bac2490000000         mov rbp, qword ptr [rsp + 0x90]
142315b4e: e88dff30ff               call 0x141625ae0
142315b53: 66440f7ec9               movd ecx, xmm9
142315b58: 660f7ec0                 movd eax, xmm0
142315b5c: 48c1e020                 shl rax, 0x20
142315b60: 4809c1                   or rcx, rax
142315b63: e898ab97fe               call 0x140c90700
142315b68: f30f11442468             movss dword ptr [rsp + 0x68], xmm0
142315b6e: f30f114c246c             movss dword ptr [rsp + 0x6c], xmm1
142315b74: 488d4c2430               lea rcx, [rsp + 0x30]
142315b79: 488d542468               lea rdx, [rsp + 0x68]
142315b7e: e83d7298fe               call 0x140c9cdc0
142315b83: 8b54244c                 mov edx, dword ptr [rsp + 0x4c]
142315b87: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
142315b8c: 4c8b442460               mov r8, qword ptr [rsp + 0x60]
142315b91: 448b5c2428               mov r11d, dword ptr [rsp + 0x28]
142315b96: f30f105c2438             movss xmm3, dword ptr [rsp + 0x38]
142315b9c: f30f591df47f6d00         mulss xmm3, dword ptr [rip + 0x6d7ff4]
142315ba4: f20f10642430             movsd xmm4, qword ptr [rsp + 0x30]
142315baa: 0f59257f51d300           mulps xmm4, xmmword ptr [rip + 0xd3517f]
142315bb1: f20f104610               movsd xmm0, qword ptr [rsi + 0x10]
142315bb6: f30f104e18               movss xmm1, dword ptr [rsi + 0x18]
142315bbb: 0f5ce0                   subps xmm4, xmm0
142315bbe: f30f5cd9                 subss xmm3, xmm1
142315bc2: 0f28d4                   movaps xmm2, xmm4
142315bc5: 0f59d4                   mulps xmm2, xmm4
142315bc8: f30f16ea                 movshdup xmm5, xmm2
142315bcc: f30f58ea                 addss xmm5, xmm2
142315bd0: 0f28d3                   movaps xmm2, xmm3
142315bd3: f30f59d3                 mulss xmm2, xmm3
142315bd7: f30f58d5                 addss xmm2, xmm5
142315bdb: f30f51d2                 sqrtss xmm2, xmm2
142315bdf: 450f28cb                 movaps xmm9, xmm11
142315be3: f3440f5eca               divss xmm9, xmm2
142315be8: f3410f12d1               movsldup xmm2, xmm9
142315bed: 0f59d4                   mulps xmm2, xmm4
142315bf0: f3440f59cb               mulss xmm9, xmm3
142315bf5: f3450f12d5               movsldup xmm10, xmm13
142315bfa: 440f59d2                 mulps xmm10, xmm2
142315bfe: f3450f59cd               mulss xmm9, xmm13
142315c03: 440f58d0                 addps xmm10, xmm0
142315c07: f3440f58c9               addss xmm9, xmm1
142315c0c: 410f28d2                 movaps xmm2, xmm10
142315c10: f3410f59d2               mulss xmm2, xmm10
142315c15: 410f28d9                 movaps xmm3, xmm9
142315c19: f3410f59d9               mulss xmm3, xmm9
142315c1e: f30f58da                 addss xmm3, xmm2
142315c22: 0f57d2                   xorps xmm2, xmm2
142315c25: f30f51d3                 sqrtss xmm2, xmm3
142315c29: f30f101de3db7600         movss xmm3, dword ptr [rip + 0x76dbe3]
142315c31: 0f2eda                   ucomiss xmm3, xmm2
142315c34: 7718                     ja 0x142315c4e
142315c36: f30f59c0                 mulss xmm0, xmm0
142315c3a: f30f59c9                 mulss xmm1, xmm1
142315c3e: f30f58c8                 addss xmm1, xmm0
142315c42: 0f57c0                   xorps xmm0, xmm0
142315c45: f30f51c1                 sqrtss xmm0, xmm1
142315c49: 0f2ec2                   ucomiss xmm0, xmm2
142315c4c: 766d                     jbe 0x142315cbb
142315c4e: 488b8424e0000000         mov rax, qword ptr [rsp + 0xe0]
142315c56: 833802                   cmp dword ptr [rax], 2
142315c59: 450f28eb                 movaps xmm13, xmm11
142315c5d: 0f850c010000             jne 0x142315d6f
142315c63: c744242000000243         mov dword ptr [rsp + 0x20], 0x43020000
142315c6b: 488d8c2410010000         lea rcx, [rsp + 0x110]
142315c73: 410f28ca                 movaps xmm1, xmm10
142315c77: 410f28d1                 movaps xmm2, xmm9
142315c7b: f30f101dd5597600         movss xmm3, dword ptr [rip + 0x7659d5]
142315c83: e868f16ffe               call 0x140a14df0
142315c88: 8b54244c                 mov edx, dword ptr [rsp + 0x4c]
142315c8c: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
142315c91: 4c8b442460               mov r8, qword ptr [rsp + 0x60]
142315c96: 448b5c2428               mov r11d, dword ptr [rsp + 0x28]
142315c9b: 0f2e05eea96400           ucomiss xmm0, dword ptr [rip + 0x64a9ee]
142315ca2: 7643                     jbe 0x142315ce7
142315ca4: 488b842488000000         mov rax, qword ptr [rsp + 0x88]
142315cac: 448918                   mov dword ptr [rax], r11d
142315caf: 41c70600000000           mov dword ptr [r14], 0
142315cb6: e9d1000000               jmp 0x142315d8c
142315cbb: 488b842488000000         mov rax, qword ptr [rsp + 0x88]
142315cc3: 448918                   mov dword ptr [rax], r11d
142315cc6: 41c70600000000           mov dword ptr [r14], 0
142315ccd: 41c6453100               mov byte ptr [r13 + 0x31], 0
142315cd2: 440f28ce                 movaps xmm9, xmm6
142315cd6: 410f28f4                 movaps xmm6, xmm12
142315cda: 450f28d6                 movaps xmm10, xmm14
142315cde: 450f28eb                 movaps xmm13, xmm11
142315ce2: e9b6000000               jmp 0x142315d9d
142315ce7: f30f104e10               movss xmm1, dword ptr [rsi + 0x10]
142315cec: f30f105618               movss xmm2, dword ptr [rsi + 0x18]
142315cf1: c744242000000243         mov dword ptr [rsp + 0x20], 0x43020000
142315cf9: 488d8c24f0000000         lea rcx, [rsp + 0xf0]
142315d01: f30f101d4f597600         movss xmm3, dword ptr [rip + 0x76594f]
142315d09: e8e2f06ffe               call 0x140a14df0
142315d0e: 8b54244c                 mov edx, dword ptr [rsp + 0x4c]
142315d12: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
142315d17: 4c8b442460               mov r8, qword ptr [rsp + 0x60]
142315d1c: 448b5c2428               mov r11d, dword ptr [rsp + 0x28]
142315d21: 0f2e0568a96400           ucomiss xmm0, dword ptr [rip + 0x64a968]
142315d28: 7745                     ja 0x142315d6f
142315d2a: c744242000000243         mov dword ptr [rsp + 0x20], 0x43020000
142315d32: 488d8c24f0000000         lea rcx, [rsp + 0xf0]
142315d3a: 410f28ca                 movaps xmm1, xmm10
142315d3e: 410f28d1                 movaps xmm2, xmm9
142315d42: f30f101d0e597600         movss xmm3, dword ptr [rip + 0x76590e]
142315d4a: e8a1f06ffe               call 0x140a14df0
142315d4f: 8b54244c                 mov edx, dword ptr [rsp + 0x4c]
142315d53: 488b4c2440               mov rcx, qword ptr [rsp + 0x40]
142315d58: 4c8b442460               mov r8, qword ptr [rsp + 0x60]
142315d5d: 448b5c2428               mov r11d, dword ptr [rsp + 0x28]
142315d62: 0f2e0527a96400           ucomiss xmm0, dword ptr [rip + 0x64a927]
142315d69: 0f8735ffffff             ja 0x142315ca4
142315d6f: 488b842488000000         mov rax, qword ptr [rsp + 0x88]
142315d77: 448918                   mov dword ptr [rax], r11d
142315d7a: 41c70601000000           mov dword ptr [r14], 1
142315d81: 450f135604               movlps qword ptr [r14 + 4], xmm10
142315d86: f3450f114e0c             movss dword ptr [r14 + 0xc], xmm9
142315d8c: 41c6453100               mov byte ptr [r13 + 0x31], 0
142315d91: 440f28ce                 movaps xmm9, xmm6
142315d95: 410f28f4                 movaps xmm6, xmm12
142315d99: 450f28d6                 movaps xmm10, xmm14
142315d9d: 450f28df                 movaps xmm11, xmm15
142315da1: 450f57e4                 xorps xmm12, xmm12
142315da5: f3440f103526756b00       movss xmm14, dword ptr [rip + 0x6b7526]
142315dae: f3440f103ddde86100       movss xmm15, dword ptr [rip + 0x61e8dd]
142315db7: 80bc24bc00000000         cmp byte ptr [rsp + 0xbc], 0
142315dbf: 0f84e0f4ffff             je 0x1423152a5
142315dc5: e9c3f4ffff               jmp 0x14231528d
142315dca: 0f28b42420010000         movaps xmm6, xmmword ptr [rsp + 0x120]
142315dd2: 0f28bc2430010000         movaps xmm7, xmmword ptr [rsp + 0x130]
142315dda: 440f28842440010000       movaps xmm8, xmmword ptr [rsp + 0x140]
142315de3: 440f288c2450010000       movaps xmm9, xmmword ptr [rsp + 0x150]
142315dec: 440f28942460010000       movaps xmm10, xmmword ptr [rsp + 0x160]
142315df5: 440f289c2470010000       movaps xmm11, xmmword ptr [rsp + 0x170]
142315dfe: 440f28a42480010000       movaps xmm12, xmmword ptr [rsp + 0x180]
142315e07: 440f28ac2490010000       movaps xmm13, xmmword ptr [rsp + 0x190]
142315e10: 440f28b424a0010000       movaps xmm14, xmmword ptr [rsp + 0x1a0]
142315e19: 440f28bc24b0010000       movaps xmm15, xmmword ptr [rsp + 0x1b0]
142315e22: 4881c4c8010000           add rsp, 0x1c8
142315e29: 5b                       pop rbx
142315e2a: 5d                       pop rbp
142315e2b: 5f                       pop rdi
142315e2c: 5e                       pop rsi
142315e2d: 415c                     pop r12
142315e2f: 415d                     pop r13
142315e31: 415e                     pop r14
142315e33: 415f                     pop r15
142315e35: c3                       ret 
