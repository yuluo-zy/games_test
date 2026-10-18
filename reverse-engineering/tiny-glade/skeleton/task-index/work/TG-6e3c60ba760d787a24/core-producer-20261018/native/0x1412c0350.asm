1412c0350: 55 push rbp
1412c0351: 56 push rsi
1412c0352: 4881ecd8000000 sub rsp, 0xd8
1412c0359: 488dac2480000000 lea rbp, [rsp + 0x80]
1412c0361: 48c74550feffffff mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c0369: 4889ce mov rsi, rcx
1412c036c: 488d4da8 lea rcx, [rbp - 0x58]
1412c0370: 4531c0 xor r8d, r8d
1412c0373: e8c8743500 call 0x141617840
1412c0378: 48c745a000000000 mov qword ptr [rbp - 0x60], 0
1412c0380: 0fb60582483702 movzx eax, byte ptr [rip + 0x2374882]
1412c0387: b930000000 mov ecx, 0x30
1412c038c: ba08000000 mov edx, 8
1412c0391: e87a3835ff call 0x140613c10
1412c0396: 4885c0 test rax, rax
1412c0399: 0f84b0000000 je 0x1412c044f
1412c039f: 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
1412c03a3: 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
1412c03a7: 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
1412c03ab: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
1412c03af: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
1412c03b3: 0f1100 movups xmmword ptr [rax], xmm0
1412c03b6: 488d0d40db7d01 lea rcx, [rip + 0x17ddb40]
1412c03bd: 48894d00 mov qword ptr [rbp], rcx
1412c03c1: 48c745085e000000 mov qword ptr [rbp + 8], 0x5e
1412c03c9: 48894510 mov qword ptr [rbp + 0x10], rax
1412c03cd: 488d055cc79601 lea rax, [rip + 0x196c75c]
1412c03d4: 48894518 mov qword ptr [rbp + 0x18], rax
1412c03d8: 48c7453000000000 mov qword ptr [rbp + 0x30], 0
1412c03e0: c6454605 mov byte ptr [rbp + 0x46], 5
1412c03e4: 48894d20 mov qword ptr [rbp + 0x20], rcx
1412c03e8: 48c745285e000000 mov qword ptr [rbp + 0x28], 0x5e
1412c03f0: 0fb60512483702 movzx eax, byte ptr [rip + 0x2374812]
1412c03f7: b940000000 mov ecx, 0x40
1412c03fc: ba08000000 mov edx, 8
1412c0401: e80a3835ff call 0x140613c10
1412c0406: 4885c0 test rax, rax
1412c0409: 7455 je 0x1412c0460
1412c040b: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
1412c040f: 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
1412c0413: 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
1412c0417: 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
1412c041b: 0f115830 movups xmmword ptr [rax + 0x30], xmm3
1412c041f: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
1412c0423: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
1412c0427: 0f1100 movups xmmword ptr [rax], xmm0
1412c042a: 48bab19d911118e9531c movabs rdx, 0x1c53e91811919db1
1412c0434: 4889f1 mov rcx, rsi
1412c0437: 4989c0 mov r8, rax
1412c043a: e8b1976aff call 0x140969bf0
1412c043f: 4883f801 cmp rax, 1
1412c0443: 742c je 0x1412c0471
1412c0445: 4881c4d8000000 add rsp, 0xd8
1412c044c: 5e pop rsi
1412c044d: 5d pop rbp
1412c044e: c3 ret 
1412c044f: b908000000 mov ecx, 8
1412c0454: ba30000000 mov edx, 0x30
1412c0459: e8a28b6101 call 0x1428d9000
1412c045e: eb0f jmp 0x1412c046f
1412c0460: b908000000 mov ecx, 8
1412c0465: ba40000000 mov edx, 0x40
1412c046a: e8918b6101 call 0x1428d9000
1412c046f: 0f0b ud2 
1412c0471: 4889d0 mov rax, rdx
1412c0474: 4883c210 add rdx, 0x10
1412c0478: 4889d6 mov rsi, rdx
1412c047b: 488378185e cmp qword ptr [rax + 0x18], 0x5e
1412c0480: 757c jne 0x1412c04fe
1412c0482: 488b16 mov rdx, qword ptr [rsi]
1412c0485: 488d0d71da7d01 lea rcx, [rip + 0x17dda71]
1412c048c: 41b85e000000 mov r8d, 0x5e
1412c0492: e859376601 call 0x142923bf0
1412c0497: 85c0 test eax, eax
1412c0499: 7563 jne 0x1412c04fe
1412c049b: 488d058edb9601 lea rax, [rip + 0x196db8e]
1412c04a2: 488945d0 mov qword ptr [rbp - 0x30], rax
1412c04a6: 488d05f33327ff lea rax, [rip - 0xd8cc0d]
1412c04ad: 488945d8 mov qword ptr [rbp - 0x28], rax
1412c04b1: 4889e8 mov rax, rbp
1412c04b4: 488945e0 mov qword ptr [rbp - 0x20], rax
1412c04b8: 488d05d199e4fe lea rax, [rip - 0x11b662f]
1412c04bf: 488945e8 mov qword ptr [rbp - 0x18], rax
1412c04c3: 488d055ed19601 lea rax, [rip + 0x196d15e]
1412c04ca: 48894510 mov qword ptr [rbp + 0x10], rax
1412c04ce: 48c7451803000000 mov qword ptr [rbp + 0x18], 3
1412c04d6: 48c7453000000000 mov qword ptr [rbp + 0x30], 0
1412c04de: 488d45d0 lea rax, [rbp - 0x30]
1412c04e2: 48894520 mov qword ptr [rbp + 0x20], rax
1412c04e6: 48c7452802000000 mov qword ptr [rbp + 0x28], 2
1412c04ee: 488d1563d19601 lea rdx, [rip + 0x196d163]
1412c04f5: 488d4d10 lea rcx, [rbp + 0x10]
1412c04f9: e8928e6101 call 0x1428d9390
1412c04fe: 488d052bdb9601 lea rax, [rip + 0x196db2b]
1412c0505: 48894510 mov qword ptr [rbp + 0x10], rax
1412c0509: 488d05903327ff lea rax, [rip - 0xd8cc70]
1412c0510: 48894518 mov qword ptr [rbp + 0x18], rax
1412c0514: 48897520 mov qword ptr [rbp + 0x20], rsi
1412c0518: 488d057199e4fe lea rax, [rip - 0x11b668f]
1412c051f: 48894528 mov qword ptr [rbp + 0x28], rax
1412c0523: 4889e9 mov rcx, rbp
1412c0526: 48894d30 mov qword ptr [rbp + 0x30], rcx
1412c052a: 48894538 mov qword ptr [rbp + 0x38], rax
1412c052e: 488d0533d09601 lea rax, [rip + 0x196d033]
1412c0535: 488945d0 mov qword ptr [rbp - 0x30], rax
1412c0539: 48c745d803000000 mov qword ptr [rbp - 0x28], 3
1412c0541: 48c745f000000000 mov qword ptr [rbp - 0x10], 0
1412c0549: 488d4510 lea rax, [rbp + 0x10]
1412c054d: 488945e0 mov qword ptr [rbp - 0x20], rax
1412c0551: 48c745e803000000 mov qword ptr [rbp - 0x18], 3
1412c0559: 488d1570d09601 lea rdx, [rip + 0x196d070]
1412c0560: 488d4dd0 lea rcx, [rbp - 0x30]
1412c0564: e8278e6101 call 0x1428d9390
1412c0569: cc int3 
1412c056a: 660f1f440000 nop word ptr [rax + rax]
1412c0570: 4889542410 mov qword ptr [rsp + 0x10], rdx
1412c0575: 55 push rbp
1412c0576: 56 push rsi
1412c0577: 4883ec28 sub rsp, 0x28
1412c057b: 488daa80000000 lea rbp, [rdx + 0x80]
1412c0582: 488d4da8 lea rcx, [rbp - 0x58]
1412c0586: e8251f92ff call 0x140be24b0
1412c058b: 90 nop 
1412c058c: 4883c428 add rsp, 0x28
1412c0590: 5e pop rsi
1412c0591: 5d pop rbp
1412c0592: c3 ret 
1412c0593: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1412c05a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1412c05a5: 55 push rbp
1412c05a6: 56 push rsi
1412c05a7: 4883ec28 sub rsp, 0x28
1412c05ab: 488daa80000000 lea rbp, [rdx + 0x80]
1412c05b2: 488d4d10 lea rcx, [rbp + 0x10]
1412c05b6: e8f5fedffe call 0x1400c04b0
1412c05bb: 90 nop 
1412c05bc: 4883c428 add rsp, 0x28
1412c05c0: 5e pop rsi
1412c05c1: 5d pop rbp
1412c05c2: c3 ret 
