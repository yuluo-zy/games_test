140a59430: 55                       push rbp
140a59431: 4156                     push r14
140a59433: 56                       push rsi
140a59434: 57                       push rdi
140a59435: 53                       push rbx
140a59436: 4881ecb0000000           sub rsp, 0xb0
140a5943d: 488dac2480000000         lea rbp, [rsp + 0x80]
140a59445: 48c74528feffffff         mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
140a5944d: 4889ce                   mov rsi, rcx
140a59450: 8b4120                   mov eax, dword ptr [rcx + 0x20]
140a59453: 3d00ca9a3b               cmp eax, 0x3b9aca00
140a59458: 7511                     jne 0x140a5946b
140a5945a: e881a13000               call 0x140d635e0
140a5945f: 48894618                 mov qword ptr [rsi + 0x18], rax
140a59463: 895620                   mov dword ptr [rsi + 0x20], edx
140a59466: e965010000               jmp 0x140a595d0
140a5946b: 488b4e18                 mov rcx, qword ptr [rsi + 0x18]
140a5946f: 48894da8                 mov qword ptr [rbp - 0x58], rcx
140a59473: 8945b0                   mov dword ptr [rbp - 0x50], eax
140a59476: 488d4da8                 lea rcx, [rbp - 0x58]
140a5947a: e891a43000               call 0x140d63910
140a5947f: 69c0e8030000             imul eax, eax, 0x3e8
140a59485: 89d1                     mov ecx, edx
140a59487: 4869c983de1b43           imul rcx, rcx, 0x431bde83
140a5948e: 48c1e932                 shr rcx, 0x32
140a59492: 01c1                     add ecx, eax
140a59494: 394e30                   cmp dword ptr [rsi + 0x30], ecx
140a59497: 0f8333010000             jae 0x140a595d0
140a5949d: 488b7e10                 mov rdi, qword ptr [rsi + 0x10]
140a594a1: 488b5e28                 mov rbx, qword ptr [rsi + 0x28]
140a594a5: 807e3500                 cmp byte ptr [rsi + 0x35], 0
140a594a9: 0f84b5000000             je 0x140a59564
140a594af: 4839fb                   cmp rbx, rdi
140a594b2: 0f8326010000             jae 0x140a595de
140a594b8: 4c8d345b                 lea r14, [rbx + rbx*2]
140a594bc: 49c1e604                 shl r14, 4
140a594c0: 4c037608                 add r14, qword ptr [rsi + 8]
140a594c4: 488d15559a0702           lea rdx, [rip + 0x2079a55]
140a594cb: 41b804000000             mov r8d, 4
140a594d1: 0f57db                   xorps xmm3, xmm3
140a594d4: 4c89f1                   mov rcx, r14
140a594d7: e894e1ffff               call 0x140a57670
140a594dc: 807e3601                 cmp byte ptr [rsi + 0x36], 1
140a594e0: 0f85bc000000             jne 0x140a595a2
140a594e6: 41833e01                 cmp dword ptr [r14], 1
140a594ea: 0f85b2000000             jne 0x140a595a2
140a594f0: 4983c608                 add r14, 8
140a594f4: 488d4db8                 lea rcx, [rbp - 0x48]
140a594f8: 4c89f2                   mov rdx, r14
140a594fb: 4531c0                   xor r8d, r8d
140a594fe: e83ddf9d00               call 0x141437440
140a59503: 48b80400000000000080     movabs rax, 0x8000000000000004
140a5950d: 483945b8                 cmp qword ptr [rbp - 0x48], rax
140a59511: 0f848b000000             je 0x140a595a2
140a59517: 488b45e8                 mov rax, qword ptr [rbp - 0x18]
140a5951b: 48894520                 mov qword ptr [rbp + 0x20], rax
140a5951f: 0f1045b8                 movups xmm0, xmmword ptr [rbp - 0x48]
140a59523: 0f104dc8                 movups xmm1, xmmword ptr [rbp - 0x38]
140a59527: 0f1055d8                 movups xmm2, xmmword ptr [rbp - 0x28]
140a5952b: 0f295510                 movaps xmmword ptr [rbp + 0x10], xmm2
140a5952f: 0f294d00                 movaps xmmword ptr [rbp], xmm1
140a59533: 0f2945f0                 movaps xmmword ptr [rbp - 0x10], xmm0
140a59537: 488d05ca960702           lea rax, [rip + 0x20796ca]
140a5953e: 4889442420               mov qword ptr [rsp + 0x20], rax
140a59543: 488d0db6700702           lea rcx, [rip + 0x20770b6]
140a5954a: 4c8d0d6f700702           lea r9, [rip + 0x207706f]
140a59551: 4c8d45f0                 lea r8, [rbp - 0x10]
140a59555: ba2b000000               mov edx, 0x2b
140a5955a: e80102e801               call 0x1428d9760
140a5955f: e9e6000000               jmp 0x140a5964a
140a59564: 4839fb                   cmp rbx, rdi
140a59567: 0f8383000000             jae 0x140a595f0
140a5956d: 488b5608                 mov rdx, qword ptr [rsi + 8]
140a59571: 488d045b                 lea rax, [rbx + rbx*2]
140a59575: 48c1e004                 shl rax, 4
140a59579: 833c0201                 cmp dword ptr [rdx + rax], 1
140a5957d: 7523                     jne 0x140a595a2
140a5957f: 4801c2                   add rdx, rax
140a59582: 4883c208                 add rdx, 8
140a59586: 488d4db8                 lea rcx, [rbp - 0x48]
140a5958a: 4531c0                   xor r8d, r8d
140a5958d: e8aede9d00               call 0x141437440
140a59592: 48b80400000000000080     movabs rax, 0x8000000000000004
140a5959c: 483945b8                 cmp qword ptr [rbp - 0x48], rax
140a595a0: 7560                     jne 0x140a59602
140a595a2: 48ffc3                   inc rbx
140a595a5: 4889d8                   mov rax, rbx
140a595a8: 4809f8                   or rax, rdi
140a595ab: 48c1e820                 shr rax, 0x20
140a595af: 740a                     je 0x140a595bb
140a595b1: 4889d8                   mov rax, rbx
140a595b4: 31d2                     xor edx, edx
140a595b6: 48f7f7                   div rdi
140a595b9: eb06                     jmp 0x140a595c1
140a595bb: 89d8                     mov eax, ebx
140a595bd: 31d2                     xor edx, edx
140a595bf: f7f7                     div edi
140a595c1: 48895628                 mov qword ptr [rsi + 0x28], rdx
140a595c5: c6463400                 mov byte ptr [rsi + 0x34], 0
140a595c9: c7462000ca9a3b           mov dword ptr [rsi + 0x20], 0x3b9aca00
140a595d0: 4881c4b0000000           add rsp, 0xb0
140a595d7: 5b                       pop rbx
140a595d8: 5f                       pop rdi
140a595d9: 5e                       pop rsi
140a595da: 415e                     pop r14
140a595dc: 5d                       pop rbp
140a595dd: c3                       ret 
140a595de: 4c8d0573990702           lea r8, [rip + 0x2079973]
140a595e5: 4889d9                   mov rcx, rbx
140a595e8: 4889fa                   mov rdx, rdi
140a595eb: e828ffe701               call 0x1428d9518
140a595f0: 4c8d0549990702           lea r8, [rip + 0x2079949]
140a595f7: 4889d9                   mov rcx, rbx
140a595fa: 4889fa                   mov rdx, rdi
140a595fd: e816ffe701               call 0x1428d9518
140a59602: 488b45e8                 mov rax, qword ptr [rbp - 0x18]
140a59606: 48894520                 mov qword ptr [rbp + 0x20], rax
140a5960a: 0f1045b8                 movups xmm0, xmmword ptr [rbp - 0x48]
140a5960e: 0f104dc8                 movups xmm1, xmmword ptr [rbp - 0x38]
140a59612: 0f1055d8                 movups xmm2, xmmword ptr [rbp - 0x28]
140a59616: 0f295510                 movaps xmmword ptr [rbp + 0x10], xmm2
140a5961a: 0f294d00                 movaps xmmword ptr [rbp], xmm1
140a5961e: 0f2945f0                 movaps xmmword ptr [rbp - 0x10], xmm0
140a59622: 488d05df950702           lea rax, [rip + 0x20795df]
140a59629: 4889442420               mov qword ptr [rsp + 0x20], rax
140a5962e: 488d0dcb6f0702           lea rcx, [rip + 0x2076fcb]
140a59635: 4c8d0d846f0702           lea r9, [rip + 0x2076f84]
140a5963c: 4c8d45f0                 lea r8, [rbp - 0x10]
140a59640: ba2b000000               mov edx, 0x2b
140a59645: e81601e801               call 0x1428d9760
140a5964a: 0f0b                     ud2 
140a5964c: 0f1f4000                 nop dword ptr [rax]
