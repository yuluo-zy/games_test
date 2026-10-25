140a86730: push rbp
140a86731: push r14
140a86733: push rsi
140a86734: push rdi
140a86735: push rbx
140a86736: sub rsp, 0x90
140a8673d: lea rbp, [rsp + 0x80]
140a86745: mov qword ptr [rbp + 8], 0xfffffffffffffffe
140a8674d: mov rdi, rcx
140a86750: mov rax, qword ptr [rcx + 0x30]
140a86754: movabs rbx, 0x8000000000000000
140a8675e: mov qword ptr [rcx + 0x30], rbx
140a86762: mov rcx, rax
140a86765: neg rcx
140a86768: jo 0x140a86909
140a8676e: mov rsi, rdx
140a86771: mov qword ptr [rbp - 0x58], rax
140a86775: movups xmm0, xmmword ptr [rdi + 0x38]
140a86779: movups xmm1, xmmword ptr [rdi + 0x48]
140a8677d: movups xmmword ptr [rbp - 0x50], xmm0
140a86781: movups xmmword ptr [rbp - 0x40], xmm1
140a86785: cmp qword ptr [rbp - 0x48], 0
140a8678a: jne 0x140a867ad
140a8678c: mov rax, qword ptr [rbp - 0x40]
140a86790: cmp qword ptr [rax + 0x28], 0
140a86795: jne 0x140a867ad
140a86797: mov rax, qword ptr [rbp - 0x38]
140a8679b: cmp qword ptr [rax + 0x40], 0
140a867a0: jne 0x140a867ad
140a867a2: cmp qword ptr [rax + 0x58], 0
140a867a7: je 0x140a868be
140a867ad: mov rax, qword ptr [rbp - 0x38]
140a867b1: mov qword ptr [rbp - 0x10], rax
140a867b5: mov rax, qword ptr [rbp - 0x58]
140a867b9: mov rcx, qword ptr [rbp - 0x50]
140a867bd: movups xmm0, xmmword ptr [rbp - 0x48]
140a867c1: movaps xmmword ptr [rbp - 0x20], xmm0
140a867c5: mov qword ptr [rbp - 0x30], rax
140a867c9: mov qword ptr [rbp - 0x28], rcx
140a867cd: mov r14, qword ptr [rdi + 0x10]
140a867d1: cmp r14, qword ptr [rdi]
140a867d4: jne 0x140a867e5
140a867d6: lea rdx, [rip + 0x2066eeb]
140a867dd: mov rcx, rdi
140a867e0: call 0x140562ab0
140a867e5: mov rax, qword ptr [rdi + 8]
140a867e9: lea rcx, [r14 + r14*4]
140a867ed: movaps xmm0, xmmword ptr [rbp - 0x30]
140a867f1: movaps xmm1, xmmword ptr [rbp - 0x20]
140a867f5: mov rdx, qword ptr [rbp - 0x10]
140a867f9: mov qword ptr [rax + rcx*8 + 0x20], rdx
140a867fe: movups xmmword ptr [rax + rcx*8 + 0x10], xmm1
140a86803: movups xmmword ptr [rax + rcx*8], xmm0
140a86807: inc r14
140a8680a: mov qword ptr [rdi + 0x10], r14
140a8680e: mov qword ptr [rbp - 0x30], 0
140a86816: mov qword ptr [rbp - 0x28], 0xffffffffffffffff
140a8681e: mov byte ptr [rbp - 0x20], 0
140a86822: lea rcx, [rip + 0x20510ef]
140a86829: lea rdx, [rbp - 0x30]
140a8682d: call 0x14132ade0
140a86832: mov qword ptr [rdi + 0x60], rax
140a86836: mov eax, dword ptr [rdi + 0x68]
140a86839: cmp eax, -2
140a8683c: jle 0x140a868cb
140a86842: mov rcx, qword ptr [rdi + 0x10]
140a86846: inc eax
140a86848: mov dword ptr [rdi + 0x68], eax
140a8684b: mov dword ptr [rbp + 4], eax
140a8684e: dec ecx
140a86850: mov dword ptr [rbp], ecx
140a86853: cmp eax, ecx
140a86855: jne 0x140a868e3
140a8685b: mov eax, dword ptr [rsi + 0x1c]
140a8685e: mov rcx, qword ptr [rsi + 0x10]
140a86862: mov dword ptr [rcx], eax
140a86864: mov rsi, qword ptr [rsi]
140a86867: mov rdi, qword ptr [rsi + 0x30]
140a8686b: mov r14, qword ptr [rsi + 0x40]
140a8686f: cmp rdi, qword ptr [rsi + 0x20]
140a86873: jne 0x140a86885
140a86875: lea rcx, [rsi + 0x20]
140a86879: lea rdx, [rip + 0x21b38c8]
140a86880: call 0x1405629f0
140a86885: mov rax, qword ptr [rsi + 0x28]
140a86889: lea rcx, [rdi + rdi*2]
140a8688d: shl rcx, 4
140a86891: mov qword ptr [rax + rcx], rbx
140a86895: mov qword ptr [rax + rcx + 8], 1
140a8689e: mov qword ptr [rax + rcx + 0x28], r14
140a868a3: inc rdi
140a868a6: mov qword ptr [rsi + 0x30], rdi
140a868aa: inc qword ptr [rsi + 0x40]
140a868ae: mov al, 1
140a868b0: add rsp, 0x90
140a868b7: pop rbx
140a868b8: pop rdi
140a868b9: pop rsi
140a868ba: pop r14
140a868bc: pop rbp
140a868bd: ret 
140a868be: lea rcx, [rbp - 0x58]
140a868c2: call 0x14088bae0
140a868c7: xor eax, eax
140a868c9: jmp 0x140a868b0
140a868cb: lea rcx, [rip + 0x2066dae]
140a868d2: lea r8, [rip + 0x2066dc7]
140a868d9: mov edx, 0x1e
140a868de: call 0x1428d9430
140a868e3: mov qword ptr [rbp - 0x30], 0
140a868eb: lea rax, [rip + 0x2051b4e]
140a868f2: mov qword ptr [rsp + 0x20], rax
140a868f7: lea rdx, [rbp + 4]
140a868fb: mov r8, rbp
140a868fe: lea r9, [rbp - 0x30]
140a86902: xor ecx, ecx
140a86904: call 0x1428d7d73
140a86909: lea rax, [rip + 0x2051b98]
140a86910: mov qword ptr [rbp - 0x30], rax
140a86914: mov qword ptr [rbp - 0x28], 1
140a8691c: mov qword ptr [rbp - 0x20], 8
140a86924: xorps xmm0, xmm0
140a86927: movups xmmword ptr [rbp - 0x18], xmm0
140a8692b: lea rdx, [rip + 0x2051b86]
140a86932: lea rcx, [rbp - 0x30]
140a86936: call 0x1428d9390
140a8693b: int3 
140a8693c: nop dword ptr [rax]
140a86940: mov qword ptr [rsp + 0x10], rdx
140a86945: push rbp
140a86946: push r14
140a86948: push rsi
140a86949: push rdi
140a8694a: push rbx
140a8694b: sub rsp, 0x30
140a8694f: lea rbp, [rdx + 0x80]
140a86956: lea rcx, [rbp - 0x30]
140a8695a: call 0x14088bae0
140a8695f: nop 
140a86960: add rsp, 0x30
140a86964: pop rbx
140a86965: pop rdi
140a86966: pop rsi
140a86967: pop r14
140a86969: pop rbp
140a8696a: ret 
140a8696b: int3 
140a8696c: int3 
140a8696d: int3 
140a8696e: int3 
140a8696f: int3 
