1416101f0: push rbp
1416101f1: push r15
1416101f3: push r14
1416101f5: push r12
1416101f7: push rsi
1416101f8: push rdi
1416101f9: push rbx
1416101fa: sub rsp, 0x120
141610201: lea rbp, [rsp + 0x80]
141610209: mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
141610214: mov rdi, rdx
141610217: mov rsi, rcx
14161021a: test r8, r8
14161021d: je 0x14161025c
14161021f: mov rbx, r9
141610222: mov rax, r9
141610225: and rax, 0x3f
141610229: mov qword ptr [rbp - 0x40], rax
14161022d: jne 0x14161041d
141610233: mov r15, r8
141610236: mov rax, r8
141610239: and rax, 3
14161023d: mov qword ptr [rbp - 0x40], rax
141610241: jne 0x14161042e
141610247: cmp rbx, 0x240
14161024e: jae 0x1416104b8
141610254: xor r12d, r12d
141610257: jmp 0x141610354
14161025c: movzx eax, byte ptr [rip + 0x20249a6]
141610263: mov ebx, 0x200
141610268: mov ecx, 0x200
14161026d: mov edx, 4
141610272: call 0x140613c10
141610277: test rax, rax
14161027a: je 0x141610522
141610280: mov r15, rax
141610283: movaps xmm0, xmmword ptr [rip + 0x14b7bd6]
14161028a: movups xmmword ptr [rax], xmm0
14161028d: movups xmmword ptr [rax + 0x10], xmm0
141610291: movups xmmword ptr [rax + 0x20], xmm0
141610295: movups xmmword ptr [rax + 0x30], xmm0
141610299: movups xmmword ptr [rax + 0x40], xmm0
14161029d: movups xmmword ptr [rax + 0x50], xmm0
1416102a1: movups xmmword ptr [rax + 0x60], xmm0
1416102a5: movups xmmword ptr [rax + 0x70], xmm0
1416102a9: movups xmmword ptr [rax + 0x80], xmm0
1416102b0: movups xmmword ptr [rax + 0x90], xmm0
1416102b7: movups xmmword ptr [rax + 0xa0], xmm0
1416102be: movups xmmword ptr [rax + 0xb0], xmm0
1416102c5: movups xmmword ptr [rax + 0xc0], xmm0
1416102cc: movups xmmword ptr [rax + 0xd0], xmm0
1416102d3: movups xmmword ptr [rax + 0xe0], xmm0
1416102da: movups xmmword ptr [rax + 0xf0], xmm0
1416102e1: movups xmmword ptr [rax + 0x100], xmm0
1416102e8: movups xmmword ptr [rax + 0x110], xmm0
1416102ef: movups xmmword ptr [rax + 0x120], xmm0
1416102f6: movups xmmword ptr [rax + 0x130], xmm0
1416102fd: movups xmmword ptr [rax + 0x140], xmm0
141610304: movups xmmword ptr [rax + 0x150], xmm0
14161030b: movups xmmword ptr [rax + 0x160], xmm0
141610312: movups xmmword ptr [rax + 0x170], xmm0
141610319: movups xmmword ptr [rax + 0x180], xmm0
141610320: movups xmmword ptr [rax + 0x190], xmm0
141610327: movups xmmword ptr [rax + 0x1a0], xmm0
14161032e: movups xmmword ptr [rax + 0x1b0], xmm0
141610335: movups xmmword ptr [rax + 0x1c0], xmm0
14161033c: movups xmmword ptr [rax + 0x1d0], xmm0
141610343: movups xmmword ptr [rax + 0x1e0], xmm0
14161034a: movups xmmword ptr [rax + 0x1f0], xmm0
141610351: mov r12b, 1
141610354: mov qword ptr [rbp + 0x88], r15
14161035b: mov byte ptr [rbp + 0x97], r12b
141610362: lea rcx, [rbp + 0x10]
141610366: mov edx, 0x200
14161036b: mov r8d, 0x22
141610371: mov r9b, 1
141610374: call 0x140bd4de0
141610379: mov qword ptr [rbp + 0x88], r15
141610380: mov byte ptr [rbp + 0x97], r12b
141610387: lea r14, [rbp + 0x68]
14161038b: lea rdx, [rbp + 0x10]
14161038f: mov r8d, 4
141610395: mov rcx, r14
141610398: call 0x140bd4f20
14161039d: mov qword ptr [rbp + 0x88], r15
1416103a4: mov byte ptr [rbp + 0x97], r12b
1416103ab: mov qword ptr [rsp + 0x30], rbx
1416103b0: mov qword ptr [rsp + 0x28], r15
1416103b5: mov qword ptr [rsp + 0x20], r14
1416103ba: lea r8, [rip + 0x166f791]
1416103c1: lea rcx, [rbp - 0x40]
1416103c5: mov r9d, 0x10
1416103cb: mov rdx, rdi
1416103ce: call 0x1416a2dc0
1416103d3: cmp dword ptr [rbp - 0x40], 0xb
1416103d7: jne 0x141610458
1416103d9: mov rax, qword ptr [rbp - 0x18]
1416103dd: mov qword ptr [rsi + 0x20], rax
1416103e1: movups xmm0, xmmword ptr [rbp - 0x38]
1416103e5: movups xmm1, xmmword ptr [rbp - 0x28]
1416103e9: movups xmmword ptr [rsi + 0x10], xmm1
1416103ed: movups xmmword ptr [rsi], xmm0
1416103f0: test r12b, r12b
1416103f3: je 0x141610408
1416103f5: mov edx, 0x200
1416103fa: mov r8d, 4
141610400: mov rcx, r15
141610403: call 0x140613c20
141610408: mov rax, rsi
14161040b: add rsp, 0x120
141610412: pop rbx
141610413: pop rdi
141610414: pop rsi
141610415: pop r12
141610417: pop r14
141610419: pop r15
14161041b: pop rbp
14161041c: ret 
14161041d: mov qword ptr [rbp + 0x10], 0
141610425: lea rax, [rip + 0x16b9b1c]
14161042c: jmp 0x14161043d
14161042e: mov qword ptr [rbp + 0x10], 0
141610436: lea rax, [rip + 0x16b9b23]
14161043d: mov qword ptr [rsp + 0x20], rax
141610442: lea r8, [rip + 0x16b9af7]
141610449: lea rdx, [rbp - 0x40]
14161044d: lea r9, [rbp + 0x10]
141610451: xor ecx, ecx
141610453: call 0x1428d9596
141610458: mov dword ptr [rbp + 0x84], r12d
14161045f: mov qword ptr [rbp + 0x78], r15
141610463: movups xmm0, xmmword ptr [rbp]
141610467: movaps xmmword ptr [rbp + 0x50], xmm0
14161046b: movups xmm0, xmmword ptr [rbp - 0x40]
14161046f: movups xmm1, xmmword ptr [rbp - 0x30]
141610473: movups xmm2, xmmword ptr [rbp - 0x20]
141610477: movups xmm3, xmmword ptr [rbp - 0x10]
14161047b: movaps xmmword ptr [rbp + 0x40], xmm3
14161047f: movaps xmmword ptr [rbp + 0x30], xmm2
141610483: movaps xmmword ptr [rbp + 0x20], xmm1
141610487: movaps xmmword ptr [rbp + 0x10], xmm0
14161048b: lea rax, [rip + 0x166fd16]
141610492: mov qword ptr [rsp + 0x20], rax
141610497: lea rcx, [rip + 0x166e64a]
14161049e: lea r9, [rip + 0x166e623]
1416104a5: lea r8, [rbp + 0x10]
1416104a9: mov edx, 0x2b
1416104ae: call 0x1428d9760
1416104b3: jmp 0x14161053f
1416104b8: lea rax, [rip + 0x148cf74]
1416104bf: mov qword ptr [rbp - 0x40], rax
1416104c3: mov qword ptr [rbp - 0x38], 0x53
1416104cb: lea rax, [rbp - 0x40]
1416104cf: mov qword ptr [rbp + 0x68], rax
1416104d3: lea rax, [rip - 0x150664a]
1416104da: mov qword ptr [rbp + 0x70], rax
1416104de: lea rax, [rip + 0x166f2e3]
1416104e5: mov qword ptr [rbp + 0x10], rax
1416104e9: mov qword ptr [rbp + 0x18], 1
1416104f1: mov qword ptr [rbp + 0x30], 0
1416104f9: lea rax, [rbp + 0x68]
1416104fd: mov qword ptr [rbp + 0x20], rax
141610501: mov qword ptr [rbp + 0x28], 1
141610509: mov byte ptr [rbp + 0x97], 0
141610510: lea rdx, [rip + 0x166f2c1]
141610517: lea rcx, [rbp + 0x10]
14161051b: call 0x1428d9390
141610520: jmp 0x14161053f
141610522: mov byte ptr [rbp + 0x97], 0
141610529: lea r8, [rip + 0x166e580]
141610530: mov ecx, 4
141610535: mov edx, 0x200
14161053a: call 0x1428d8fe3
14161053f: ud2 
141610541: nop word ptr cs:[rax + rax]
141610550: mov qword ptr [rsp + 0x10], rdx
141610555: push rbp
141610556: push r15
141610558: push r14
14161055a: push r12
14161055c: push rsi
14161055d: push rdi
14161055e: push rbx
14161055f: sub rsp, 0x40
141610563: lea rbp, [rdx + 0x80]
14161056a: mov rsi, qword ptr [rbp + 0x78]
14161056e: mov qword ptr [rbp + 0x88], rsi
141610575: mov edi, dword ptr [rbp + 0x84]
14161057b: mov byte ptr [rbp + 0x97], dil
141610582: lea rcx, [rbp + 0x10]
141610586: call 0x1407cea70
14161058b: mov qword ptr [rbp + 0x88], rsi
141610592: mov byte ptr [rbp + 0x97], dil
141610599: add rsp, 0x40
14161059d: pop rbx
14161059e: pop rdi
14161059f: pop rsi
1416105a0: pop r12
1416105a2: pop r14
1416105a4: pop r15
1416105a6: pop rbp
1416105a7: ret 
1416105a8: nop dword ptr [rax + rax]
1416105b0: mov qword ptr [rsp + 0x10], rdx
1416105b5: push rbp
1416105b6: push r15
1416105b8: push r14
1416105ba: push r12
1416105bc: push rsi
1416105bd: push rdi
1416105be: push rbx
1416105bf: sub rsp, 0x40
1416105c3: lea rbp, [rdx + 0x80]
1416105ca: test byte ptr [rbp + 0x97], 1
1416105d1: je 0x1416105ea
1416105d3: mov rcx, qword ptr [rbp + 0x88]
1416105da: mov edx, 0x200
1416105df: mov r8d, 4
1416105e5: call 0x140613c20
1416105ea: nop 
1416105eb: add rsp, 0x40
1416105ef: pop rbx
1416105f0: pop rdi
1416105f1: pop rsi
1416105f2: pop r12
1416105f4: pop r14
1416105f6: pop r15
1416105f8: pop rbp
1416105f9: ret 
