14129d450: 4156 push r14
14129d452: 56 push rsi
14129d453: 57 push rdi
14129d454: 53 push rbx
14129d455: 4881ec98000000 sub rsp, 0x98
14129d45c: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
14129d464: 4889ce mov rsi, rcx
14129d467: 488b02 mov rax, qword ptr [rdx]
14129d46a: 483b4208 cmp rax, qword ptr [rdx + 8]
14129d46e: 0f835f010000 jae 0x14129d5d3
14129d474: 4889d7 mov rdi, rdx
14129d477: 488d4801 lea rcx, [rax + 1]
14129d47b: 48890a mov qword ptr [rdx], rcx
14129d47e: 4885c0 test rax, rax
14129d481: 7807 js 0x14129d48a
14129d483: f3480f2ac8 cvtsi2ss xmm1, rax
14129d488: eb15 jmp 0x14129d49f
14129d48a: 4889c1 mov rcx, rax
14129d48d: 48d1e9 shr rcx, 1
14129d490: 83e001 and eax, 1
14129d493: 4809c8 or rax, rcx
14129d496: f3480f2ac8 cvtsi2ss xmm1, rax
14129d49b: f30f58c9 addss xmm1, xmm1
14129d49f: f30f594f20 mulss xmm1, dword ptr [rdi + 0x20]
14129d4a4: 0f57f6 xorps xmm6, xmm6
14129d4a7: f30f59f1 mulss xmm6, xmm1
14129d4ab: 8b4710 mov eax, dword ptr [rdi + 0x10]
14129d4ae: f30f104714 movss xmm0, dword ptr [rdi + 0x14]
14129d4b3: f30f584f18 addss xmm1, dword ptr [rdi + 0x18]
14129d4b8: f30f58771c addss xmm6, dword ptr [rdi + 0x1c]
14129d4bd: 89442428 mov dword ptr [rsp + 0x28], eax
14129d4c1: f30f1144242c movss dword ptr [rsp + 0x2c], xmm0
14129d4c7: 488d4c2428 lea rcx, [rsp + 0x28]
14129d4cc: e84f4a9fff call 0x140c91f20
14129d4d1: 488d5c2474 lea rbx, [rsp + 0x74]
14129d4d6: 4c8b4728 mov r8, qword ptr [rdi + 0x28]
14129d4da: f30f11442428 movss dword ptr [rsp + 0x28], xmm0
14129d4e0: f30f1174242c movss dword ptr [rsp + 0x2c], xmm6
14129d4e6: 488d542428 lea rdx, [rsp + 0x28]
14129d4eb: 4889d9 mov rcx, rbx
14129d4ee: e89d4b9fff call 0x140c92090
14129d4f3: f6473001 test byte ptr [rdi + 0x30], 1
14129d4f7: 744a je 0x14129d543
14129d4f9: 8b4748 mov eax, dword ptr [rdi + 0x48]
14129d4fc: 89442450 mov dword ptr [rsp + 0x50], eax
14129d500: 488b4740 mov rax, qword ptr [rdi + 0x40]
14129d504: 4889442448 mov qword ptr [rsp + 0x48], rax
14129d509: 488b03 mov rax, qword ptr [rbx]
14129d50c: 48894740 mov qword ptr [rdi + 0x40], rax
14129d510: 8b4308 mov eax, dword ptr [rbx + 8]
14129d513: 894748 mov dword ptr [rdi + 0x48], eax
14129d516: 488b442448 mov rax, qword ptr [rsp + 0x48]
14129d51b: 48894734 mov qword ptr [rdi + 0x34], rax
14129d51f: 8b442450 mov eax, dword ptr [rsp + 0x50]
14129d523: 89473c mov dword ptr [rdi + 0x3c], eax
14129d526: 8b473c mov eax, dword ptr [rdi + 0x3c]
14129d529: 89442430 mov dword ptr [rsp + 0x30], eax
14129d52d: 488b4734 mov rax, qword ptr [rdi + 0x34]
14129d531: 4889442428 mov qword ptr [rsp + 0x28], rax
14129d536: 488b03 mov rax, qword ptr [rbx]
14129d539: 4889442434 mov qword ptr [rsp + 0x34], rax
14129d53e: 8b4308 mov eax, dword ptr [rbx + 8]
14129d541: eb64 jmp 0x14129d5a7
14129d543: 4c8d7730 lea r14, [rdi + 0x30]
14129d547: 8b4308 mov eax, dword ptr [rbx + 8]
14129d54a: 89442464 mov dword ptr [rsp + 0x64], eax
14129d54e: 488b03 mov rax, qword ptr [rbx]
14129d551: 488944245c mov qword ptr [rsp + 0x5c], rax
14129d556: c744245801000000 mov dword ptr [rsp + 0x58], 1
14129d55e: 48897c2468 mov qword ptr [rsp + 0x68], rdi
14129d563: 488d4c2428 lea rcx, [rsp + 0x28]
14129d568: 488d542458 lea rdx, [rsp + 0x58]
14129d56d: e84e80fdff call 0x1412755c0
14129d572: 0f10442428 movups xmm0, xmmword ptr [rsp + 0x28]
14129d577: 0f104c2434 movups xmm1, xmmword ptr [rsp + 0x34]
14129d57c: 410f114e0c movups xmmword ptr [r14 + 0xc], xmm1
14129d581: 410f1106 movups xmmword ptr [r14], xmm0
14129d585: f6473001 test byte ptr [rdi + 0x30], 1
14129d589: 7448 je 0x14129d5d3
14129d58b: 8b473c mov eax, dword ptr [rdi + 0x3c]
14129d58e: 89442430 mov dword ptr [rsp + 0x30], eax
14129d592: 488b4734 mov rax, qword ptr [rdi + 0x34]
14129d596: 4889442428 mov qword ptr [rsp + 0x28], rax
14129d59b: 488b4740 mov rax, qword ptr [rdi + 0x40]
14129d59f: 4889442434 mov qword ptr [rsp + 0x34], rax
14129d5a4: 8b4748 mov eax, dword ptr [rdi + 0x48]
14129d5a7: 8944243c mov dword ptr [rsp + 0x3c], eax
14129d5ab: 488b442428 mov rax, qword ptr [rsp + 0x28]
14129d5b0: 48894604 mov qword ptr [rsi + 4], rax
14129d5b4: 8b442430 mov eax, dword ptr [rsp + 0x30]
14129d5b8: 89460c mov dword ptr [rsi + 0xc], eax
14129d5bb: 8b442434 mov eax, dword ptr [rsp + 0x34]
14129d5bf: 894610 mov dword ptr [rsi + 0x10], eax
14129d5c2: 488b442438 mov rax, qword ptr [rsp + 0x38]
14129d5c7: 48894614 mov qword ptr [rsi + 0x14], rax
14129d5cb: c70601000000 mov dword ptr [rsi], 1
14129d5d1: eb06 jmp 0x14129d5d9
14129d5d3: c70600000000 mov dword ptr [rsi], 0
14129d5d9: 4889f0 mov rax, rsi
14129d5dc: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14129d5e4: 4881c498000000 add rsp, 0x98
14129d5eb: 5b pop rbx
14129d5ec: 5f pop rdi
14129d5ed: 5e pop rsi
14129d5ee: 415e pop r14
14129d5f0: c3 ret 
