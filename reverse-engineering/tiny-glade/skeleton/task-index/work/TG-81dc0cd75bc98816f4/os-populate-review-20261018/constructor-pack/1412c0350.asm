1412c0350: push rbp
1412c0351: push rsi
1412c0352: sub rsp, 0xd8
1412c0359: lea rbp, [rsp + 0x80]
1412c0361: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c0369: mov rsi, rcx
1412c036c: lea rcx, [rbp - 0x58]
1412c0370: xor r8d, r8d
1412c0373: call 0x141617840
1412c0378: mov qword ptr [rbp - 0x60], 0
1412c0380: movzx eax, byte ptr [rip + 0x2374882]
1412c0387: mov ecx, 0x30
1412c038c: mov edx, 8
1412c0391: call 0x140613c10
1412c0396: test rax, rax
1412c0399: je 0x1412c044f
1412c039f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c03a3: movups xmm1, xmmword ptr [rbp - 0x50]
1412c03a7: movups xmm2, xmmword ptr [rbp - 0x40]
1412c03ab: movups xmmword ptr [rax + 0x20], xmm2
1412c03af: movups xmmword ptr [rax + 0x10], xmm1
1412c03b3: movups xmmword ptr [rax], xmm0
1412c03b6: lea rcx, [rip + 0x17ddb40]
1412c03bd: mov qword ptr [rbp], rcx
1412c03c1: mov qword ptr [rbp + 8], 0x5e
1412c03c9: mov qword ptr [rbp + 0x10], rax
1412c03cd: lea rax, [rip + 0x196c75c]
1412c03d4: mov qword ptr [rbp + 0x18], rax
1412c03d8: mov qword ptr [rbp + 0x30], 0
1412c03e0: mov byte ptr [rbp + 0x46], 5
1412c03e4: mov qword ptr [rbp + 0x20], rcx
1412c03e8: mov qword ptr [rbp + 0x28], 0x5e
1412c03f0: movzx eax, byte ptr [rip + 0x2374812]
1412c03f7: mov ecx, 0x40
1412c03fc: mov edx, 8
1412c0401: call 0x140613c10
1412c0406: test rax, rax
1412c0409: je 0x1412c0460
1412c040b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c040f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c0413: movups xmm2, xmmword ptr [rbp + 0x30]
1412c0417: movups xmm3, xmmword ptr [rbp + 0x40]
1412c041b: movups xmmword ptr [rax + 0x30], xmm3
1412c041f: movups xmmword ptr [rax + 0x20], xmm2
1412c0423: movups xmmword ptr [rax + 0x10], xmm1
1412c0427: movups xmmword ptr [rax], xmm0
1412c042a: movabs rdx, 0x1c53e91811919db1
1412c0434: mov rcx, rsi
1412c0437: mov r8, rax
1412c043a: call 0x140969bf0
1412c043f: cmp rax, 1
1412c0443: je 0x1412c0471
1412c0445: add rsp, 0xd8
1412c044c: pop rsi
1412c044d: pop rbp
1412c044e: ret 
1412c044f: mov ecx, 8
1412c0454: mov edx, 0x30
1412c0459: call 0x1428d9000
1412c045e: jmp 0x1412c046f
1412c0460: mov ecx, 8
1412c0465: mov edx, 0x40
1412c046a: call 0x1428d9000
1412c046f: ud2 
1412c0471: mov rax, rdx
1412c0474: add rdx, 0x10
1412c0478: mov rsi, rdx
1412c047b: cmp qword ptr [rax + 0x18], 0x5e
1412c0480: jne 0x1412c04fe
1412c0482: mov rdx, qword ptr [rsi]
1412c0485: lea rcx, [rip + 0x17dda71]
1412c048c: mov r8d, 0x5e
1412c0492: call 0x142923bf0
1412c0497: test eax, eax
1412c0499: jne 0x1412c04fe
1412c049b: lea rax, [rip + 0x196db8e]
1412c04a2: mov qword ptr [rbp - 0x30], rax
1412c04a6: lea rax, [rip - 0xd8cc0d]
1412c04ad: mov qword ptr [rbp - 0x28], rax
1412c04b1: mov rax, rbp
1412c04b4: mov qword ptr [rbp - 0x20], rax
1412c04b8: lea rax, [rip - 0x11b662f]
1412c04bf: mov qword ptr [rbp - 0x18], rax
1412c04c3: lea rax, [rip + 0x196d15e]
1412c04ca: mov qword ptr [rbp + 0x10], rax
1412c04ce: mov qword ptr [rbp + 0x18], 3
1412c04d6: mov qword ptr [rbp + 0x30], 0
1412c04de: lea rax, [rbp - 0x30]
1412c04e2: mov qword ptr [rbp + 0x20], rax
1412c04e6: mov qword ptr [rbp + 0x28], 2
1412c04ee: lea rdx, [rip + 0x196d163]
1412c04f5: lea rcx, [rbp + 0x10]
1412c04f9: call 0x1428d9390
1412c04fe: lea rax, [rip + 0x196db2b]
1412c0505: mov qword ptr [rbp + 0x10], rax
1412c0509: lea rax, [rip - 0xd8cc70]
1412c0510: mov qword ptr [rbp + 0x18], rax
1412c0514: mov qword ptr [rbp + 0x20], rsi
1412c0518: lea rax, [rip - 0x11b668f]
1412c051f: mov qword ptr [rbp + 0x28], rax
1412c0523: mov rcx, rbp
1412c0526: mov qword ptr [rbp + 0x30], rcx
1412c052a: mov qword ptr [rbp + 0x38], rax
1412c052e: lea rax, [rip + 0x196d033]
1412c0535: mov qword ptr [rbp - 0x30], rax
1412c0539: mov qword ptr [rbp - 0x28], 3
1412c0541: mov qword ptr [rbp - 0x10], 0
1412c0549: lea rax, [rbp + 0x10]
1412c054d: mov qword ptr [rbp - 0x20], rax
1412c0551: mov qword ptr [rbp - 0x18], 3
1412c0559: lea rdx, [rip + 0x196d070]
1412c0560: lea rcx, [rbp - 0x30]
1412c0564: call 0x1428d9390
1412c0569: int3 
1412c056a: nop word ptr [rax + rax]
1412c0570: mov qword ptr [rsp + 0x10], rdx
1412c0575: push rbp
1412c0576: push rsi
1412c0577: sub rsp, 0x28
1412c057b: lea rbp, [rdx + 0x80]
1412c0582: lea rcx, [rbp - 0x58]
1412c0586: call 0x140be24b0
1412c058b: nop 
1412c058c: add rsp, 0x28
1412c0590: pop rsi
1412c0591: pop rbp
1412c0592: ret 
1412c0593: nop word ptr cs:[rax + rax]
1412c05a0: mov qword ptr [rsp + 0x10], rdx
1412c05a5: push rbp
1412c05a6: push rsi
1412c05a7: sub rsp, 0x28
1412c05ab: lea rbp, [rdx + 0x80]
1412c05b2: lea rcx, [rbp + 0x10]
1412c05b6: call 0x1400c04b0
1412c05bb: nop 
1412c05bc: add rsp, 0x28
1412c05c0: pop rsi
1412c05c1: pop rbp
1412c05c2: ret 
