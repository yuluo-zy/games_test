1404f7450: 4157 push r15
1404f7452: 4156 push r14
1404f7454: 4155 push r13
1404f7456: 4154 push r12
1404f7458: 56 push rsi
1404f7459: 57 push rdi
1404f745a: 55 push rbp
1404f745b: 53 push rbx
1404f745c: 4881ecf8000000 sub rsp, 0xf8
1404f7463: 4889d7 mov rdi, rdx
1404f7466: 4889ce mov rsi, rcx
1404f7469: 488b9c2460010000 mov rbx, qword ptr [rsp + 0x160]
1404f7471: 4c89c1 mov rcx, r8
1404f7474: 4c89ca mov rdx, r9
1404f7477: e8c4817100 call 0x140c0f640
1404f747c: 4889442428 mov qword ptr [rsp + 0x28], rax
1404f7481: 4c8d642428 lea r12, [rsp + 0x28]
1404f7486: 4c89642468 mov qword ptr [rsp + 0x68], r12
1404f748b: 4c8d2d5e827100 lea r13, [rip + 0x71825e]
1404f7492: 4c896c2470 mov qword ptr [rsp + 0x70], r13
1404f7497: 488d058a5a4f02 lea rax, [rip + 0x24f5a8a]
1404f749e: 4889442430 mov qword ptr [rsp + 0x30], rax
1404f74a3: 48c744243802000000 mov qword ptr [rsp + 0x38], 2
1404f74ac: 4c8d7c2468 lea r15, [rsp + 0x68]
1404f74b1: 4c897c2440 mov qword ptr [rsp + 0x40], r15
1404f74b6: 48c744244801000000 mov qword ptr [rsp + 0x48], 1
1404f74bf: 48c744245000000000 mov qword ptr [rsp + 0x50], 0
1404f74c8: 4c8db424e0000000 lea r14, [rsp + 0xe0]
1404f74d0: 488d542430 lea rdx, [rsp + 0x30]
1404f74d5: 4c89f1 mov rcx, r14
1404f74d8: e8b32a8800 call 0x140d79f90
1404f74dd: 0f1003 movups xmm0, xmmword ptr [rbx]
1404f74e0: 0f104b10 movups xmm1, xmmword ptr [rbx + 0x10]
1404f74e4: 0f294c2440 movaps xmmword ptr [rsp + 0x40], xmm1
1404f74e9: 0f29442430 movaps xmmword ptr [rsp + 0x30], xmm0
1404f74ee: 488d4c2478 lea rcx, [rsp + 0x78]
1404f74f3: 4c8d4c2430 lea r9, [rsp + 0x30]
1404f74f8: 4889fa mov rdx, rdi
1404f74fb: 4d89f0 mov r8, r14
1404f74fe: e84df3ffff call 0x1404f6850
1404f7503: 0fb6ac249c000000 movzx ebp, byte ptr [rsp + 0x9c]
1404f750b: 488b442478 mov rax, qword ptr [rsp + 0x78]
1404f7510: 4080fd02 cmp bpl, 2
1404f7514: 750e jne 0x1404f7524
1404f7516: 48894608 mov qword ptr [rsi + 8], rax
1404f751a: b901000000 mov ecx, 1
1404f751f: e975010000 jmp 0x1404f7699
1404f7524: 4889442420 mov qword ptr [rsp + 0x20], rax
1404f7529: 0f10842480000000 movups xmm0, xmmword ptr [rsp + 0x80]
1404f7531: 0f108c248c000000 movups xmm1, xmmword ptr [rsp + 0x8c]
1404f7539: 0f118c24ac000000 movups xmmword ptr [rsp + 0xac], xmm1
1404f7541: 0f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm0
1404f7549: 4c89642468 mov qword ptr [rsp + 0x68], r12
1404f754e: 4c896c2470 mov qword ptr [rsp + 0x70], r13
1404f7553: 488d05f6594f02 lea rax, [rip + 0x24f59f6]
1404f755a: 4889442430 mov qword ptr [rsp + 0x30], rax
1404f755f: 48c744243802000000 mov qword ptr [rsp + 0x38], 2
1404f7568: 4c897c2440 mov qword ptr [rsp + 0x40], r15
1404f756d: 48c744244801000000 mov qword ptr [rsp + 0x48], 1
1404f7576: 48c744245000000000 mov qword ptr [rsp + 0x50], 0
1404f757f: 4c8db424e0000000 lea r14, [rsp + 0xe0]
1404f7587: 488d542430 lea rdx, [rsp + 0x30]
1404f758c: 4c89f1 mov rcx, r14
1404f758f: e8fc298800 call 0x140d79f90
1404f7594: 488d4c2478 lea rcx, [rsp + 0x78]
1404f7599: 4889fa mov rdx, rdi
1404f759c: 4d89f0 mov r8, r14
1404f759f: 4989d9 mov r9, rbx
1404f75a2: e8a9f2ffff call 0x1404f6850
1404f75a7: 440fb6b4249c000000 movzx r14d, byte ptr [rsp + 0x9c]
1404f75b0: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1404f75b5: 4180fe02 cmp r14b, 2
1404f75b9: 750e jne 0x1404f75c9
1404f75bb: 48895e08 mov qword ptr [rsi + 8], rbx
1404f75bf: b901000000 mov ecx, 1
1404f75c4: e9d0000000 jmp 0x1404f7699
1404f75c9: 0f10842480000000 movups xmm0, xmmword ptr [rsp + 0x80]
1404f75d1: 0f108c248c000000 movups xmm1, xmmword ptr [rsp + 0x8c]
1404f75d9: 0f118c24cc000000 movups xmmword ptr [rsp + 0xcc], xmm1
1404f75e1: 0f298424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm0
1404f75e9: 4881c7b0000000 add rdi, 0xb0
1404f75f0: 488b542428 mov rdx, qword ptr [rsp + 0x28]
1404f75f5: 4889f9 mov rcx, rdi
1404f75f8: e8d3586f00 call 0x140beced0
1404f75fd: 4c8d4e30 lea r9, [rsi + 0x30]
1404f7601: 4c8d460c lea r8, [rsi + 0xc]
1404f7605: 410fb6d6 movzx edx, r14b
1404f7609: 31c9 xor ecx, ecx
1404f760b: 40f6c501 test bpl, 1
1404f760f: 0f44d1 cmove edx, ecx
1404f7612: 85c0 test eax, eax
1404f7614: 7441 je 0x1404f7657
1404f7616: 0f108424ac000000 movups xmm0, xmmword ptr [rsp + 0xac]
1404f761e: 410f11410c movups xmmword ptr [r9 + 0xc], xmm0
1404f7623: 0f288424a0000000 movaps xmm0, xmmword ptr [rsp + 0xa0]
1404f762b: 410f1101 movups xmmword ptr [r9], xmm0
1404f762f: 0f108424cc000000 movups xmm0, xmmword ptr [rsp + 0xcc]
1404f7637: 410f11400c movups xmmword ptr [r8 + 0xc], xmm0
1404f763c: 0f288424c0000000 movaps xmm0, xmmword ptr [rsp + 0xc0]
1404f7644: 410f1100 movups xmmword ptr [r8], xmm0
1404f7648: 48895e04 mov qword ptr [rsi + 4], rbx
1404f764c: 488b442420 mov rax, qword ptr [rsp + 0x20]
1404f7651: 48894628 mov qword ptr [rsi + 0x28], rax
1404f7655: eb3f jmp 0x1404f7696
1404f7657: 0f108424cc000000 movups xmm0, xmmword ptr [rsp + 0xcc]
1404f765f: 410f11410c movups xmmword ptr [r9 + 0xc], xmm0
1404f7664: 0f288424c0000000 movaps xmm0, xmmword ptr [rsp + 0xc0]
1404f766c: 410f1101 movups xmmword ptr [r9], xmm0
1404f7670: 0f108424ac000000 movups xmm0, xmmword ptr [rsp + 0xac]
1404f7678: 410f11400c movups xmmword ptr [r8 + 0xc], xmm0
1404f767d: 0f288424a0000000 movaps xmm0, xmmword ptr [rsp + 0xa0]
1404f7685: 410f1100 movups xmmword ptr [r8], xmm0
1404f7689: 488b442420 mov rax, qword ptr [rsp + 0x20]
1404f768e: 48894604 mov qword ptr [rsi + 4], rax
1404f7692: 48895e28 mov qword ptr [rsi + 0x28], rbx
1404f7696: 88564c mov byte ptr [rsi + 0x4c], dl
1404f7699: 890e mov dword ptr [rsi], ecx
1404f769b: 4889f0 mov rax, rsi
1404f769e: 4881c4f8000000 add rsp, 0xf8
1404f76a5: 5b pop rbx
1404f76a6: 5d pop rbp
1404f76a7: 5f pop rdi
1404f76a8: 5e pop rsi
1404f76a9: 415c pop r12
1404f76ab: 415d pop r13
1404f76ad: 415e pop r14
1404f76af: 415f pop r15
1404f76b1: c3 ret 
