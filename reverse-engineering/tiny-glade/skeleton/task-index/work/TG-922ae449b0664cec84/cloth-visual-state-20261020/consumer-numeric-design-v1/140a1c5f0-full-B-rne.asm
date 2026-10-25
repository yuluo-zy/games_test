140a1c5f0: 56 push rsi
140a1c5f1: 57 push rdi
140a1c5f2: 53 push rbx
140a1c5f3: 4883ec30 sub rsp, 0x30
140a1c5f7: 4489ce mov esi, r9d
140a1c5fa: 44894c2424 mov dword ptr [rsp + 0x24], r9d
140a1c5ff: 4885d2 test rdx, rdx
140a1c602: 0f84d6000000 je 0x140a1c6de
140a1c608: 4883fa01 cmp rdx, 1
140a1c60c: 0f8585010000 jne 0x140a1c797
140a1c612: 4883b99000000000 cmp qword ptr [rcx + 0x90], 0
140a1c61a: 0f8497010000 je 0x140a1c7b7
140a1c620: 4881c190000000 add rcx, 0x90
140a1c627: 4883794800 cmp qword ptr [rcx + 0x48], 0
140a1c62c: 0f8481010000 je 0x140a1c7b3
140a1c632: 488d4150 lea rax, [rcx + 0x50]
140a1c636: 488d542424 lea rdx, [rsp + 0x24]
140a1c63b: 4889cf mov rdi, rcx
140a1c63e: 4889c1 mov rcx, rax
140a1c641: e88a48b9ff call 0x1405b0ed0
140a1c646: 488b4f30 mov rcx, qword ptr [rdi + 0x30]
140a1c64a: 488b5738 mov rdx, qword ptr [rdi + 0x38]
140a1c64e: 4989d0 mov r8, rdx
140a1c651: 4921c0 and r8, rax
140a1c654: 48c1e839 shr rax, 0x39
140a1c658: 660f6ec0 movd xmm0, eax
140a1c65c: 660f60c0 punpcklbw xmm0, xmm0
140a1c660: f20f70c000 pshuflw xmm0, xmm0, 0
140a1c665: 660f70c000 pshufd xmm0, xmm0, 0
140a1c66a: 4c8d49b0 lea r9, [rcx - 0x50]
140a1c66e: 31c0 xor eax, eax
140a1c670: 660f76c9 pcmpeqd xmm1, xmm1
140a1c674: 4531d2 xor r10d, r10d
140a1c677: f3420f6f1401 movdqu xmm2, xmmword ptr [rcx + r8]
140a1c67d: 660f6fda movdqa xmm3, xmm2
140a1c681: 660f74d8 pcmpeqb xmm3, xmm0
140a1c685: 66440fd7db pmovmskb r11d, xmm3
140a1c68a: 4585db test r11d, r11d
140a1c68d: 742d je 0x140a1c6bc
140a1c68f: f3410fbcfb tzcnt edi, r11d
140a1c694: 4c01c7 add rdi, r8
140a1c697: 4821d7 and rdi, rdx
140a1c69a: 48f7df neg rdi
140a1c69d: 488d3cbf lea rdi, [rdi + rdi*4]
140a1c6a1: 48c1e704 shl rdi, 4
140a1c6a5: 41393439 cmp dword ptr [r9 + rdi], esi
140a1c6a9: 0f840c010000 je 0x140a1c7bb
140a1c6af: 418d7bff lea edi, [r11 - 1]
140a1c6b3: 664421df and di, r11w
140a1c6b7: 4189fb mov r11d, edi
140a1c6ba: 75d3 jne 0x140a1c68f
140a1c6bc: 660f74d1 pcmpeqb xmm2, xmm1
140a1c6c0: 66440fd7da pmovmskb r11d, xmm2
140a1c6c5: 4585db test r11d, r11d
140a1c6c8: 0f85f3000000 jne 0x140a1c7c1
140a1c6ce: 4d01d0 add r8, r10
140a1c6d1: 4983c010 add r8, 0x10
140a1c6d5: 4983c210 add r10, 0x10
140a1c6d9: 4921d0 and r8, rdx
140a1c6dc: eb99 jmp 0x140a1c677
140a1c6de: 4c89442428 mov qword ptr [rsp + 0x28], r8
140a1c6e3: 4883791800 cmp qword ptr [rcx + 0x18], 0
140a1c6e8: 0f84c9000000 je 0x140a1c7b7
140a1c6ee: 4c89c7 mov rdi, r8
140a1c6f1: 488d4120 lea rax, [rcx + 0x20]
140a1c6f5: 488d542428 lea rdx, [rsp + 0x28]
140a1c6fa: 4889cb mov rbx, rcx
140a1c6fd: 4889c1 mov rcx, rax
140a1c700: e86b46b9ff call 0x1405b0d70
140a1c705: 488b0b mov rcx, qword ptr [rbx]
140a1c708: 488b5308 mov rdx, qword ptr [rbx + 8]
140a1c70c: 4989d0 mov r8, rdx
140a1c70f: 4921c0 and r8, rax
140a1c712: 48c1e839 shr rax, 0x39
140a1c716: 660f6ec0 movd xmm0, eax
140a1c71a: 660f60c0 punpcklbw xmm0, xmm0
140a1c71e: f20f70c000 pshuflw xmm0, xmm0, 0
140a1c723: 660f70c000 pshufd xmm0, xmm0, 0
140a1c728: 488d8168ffffff lea rax, [rcx - 0x98]
140a1c72f: 4531c9 xor r9d, r9d
140a1c732: 660f76c9 pcmpeqd xmm1, xmm1
140a1c736: f3420f6f1401 movdqu xmm2, xmmword ptr [rcx + r8]
140a1c73c: 660f6fda movdqa xmm3, xmm2
140a1c740: 660f74d8 pcmpeqb xmm3, xmm0
140a1c744: 66440fd7d3 pmovmskb r10d, xmm3
140a1c749: 4585d2 test r10d, r10d
140a1c74c: 742b je 0x140a1c779
140a1c74e: 4989fb mov r11, rdi
140a1c751: f3410fbcda tzcnt ebx, r10d
140a1c756: 4c01c3 add rbx, r8
140a1c759: 4821d3 and rbx, rdx
140a1c75c: 48f7db neg rbx
140a1c75f: 4869db98000000 imul rbx, rbx, 0x98
140a1c766: 4c391c18 cmp qword ptr [rax + rbx], r11
140a1c76a: 7473 je 0x140a1c7df
140a1c76c: 418d5aff lea ebx, [r10 - 1]
140a1c770: 664421d3 and bx, r10w
140a1c774: 4189da mov r10d, ebx
140a1c777: 75d8 jne 0x140a1c751
140a1c779: 660f74d1 pcmpeqb xmm2, xmm1
140a1c77d: 66440fd7d2 pmovmskb r10d, xmm2
140a1c782: 4585d2 test r10d, r10d
140a1c785: 7530 jne 0x140a1c7b7
140a1c787: 4d01c8 add r8, r9
140a1c78a: 4983c010 add r8, 0x10
140a1c78e: 4983c110 add r9, 0x10
140a1c792: 4921d0 and r8, rdx
140a1c795: eb9f jmp 0x140a1c736
140a1c797: 4883b92001000000 cmp qword ptr [rcx + 0x120], 0
140a1c79f: 7416 je 0x140a1c7b7
140a1c7a1: 4881c120010000 add rcx, 0x120
140a1c7a8: 4883794800 cmp qword ptr [rcx + 0x48], 0
140a1c7ad: 0f857ffeffff jne 0x140a1c632
140a1c7b3: 31c9 xor ecx, ecx
140a1c7b5: eb15 jmp 0x140a1c7cc
140a1c7b7: 31c0 xor eax, eax
140a1c7b9: eb1c jmp 0x140a1c7d7
140a1c7bb: 4801f9 add rcx, rdi
140a1c7be: 4889c8 mov rax, rcx
140a1c7c1: 488d48b0 lea rcx, [rax - 0x50]
140a1c7c5: 4885c0 test rax, rax
140a1c7c8: 480f44c8 cmove rcx, rax
140a1c7cc: 488d4108 lea rax, [rcx + 8]
140a1c7d0: 4885c9 test rcx, rcx
140a1c7d3: 480f44c1 cmove rax, rcx
140a1c7d7: 4883c430 add rsp, 0x30
140a1c7db: 5b pop rbx
140a1c7dc: 5f pop rdi
140a1c7dd: 5e pop rsi
140a1c7de: c3 ret 
140a1c7df: 4801d9 add rcx, rbx
140a1c7e2: 4881c170ffffff add rcx, -0x90
140a1c7e9: 4883794800 cmp qword ptr [rcx + 0x48], 0
140a1c7ee: 0f853efeffff jne 0x140a1c632
140a1c7f4: ebbd jmp 0x140a1c7b3
