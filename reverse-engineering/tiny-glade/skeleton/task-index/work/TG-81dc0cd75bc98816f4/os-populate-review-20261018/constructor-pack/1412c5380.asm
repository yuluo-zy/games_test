1412c5380: push rbp
1412c5381: push rsi
1412c5382: sub rsp, 0xd8
1412c5389: lea rbp, [rsp + 0x80]
1412c5391: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c5399: mov rsi, rcx
1412c539c: lea rcx, [rbp - 0x58]
1412c53a0: xor r8d, r8d
1412c53a3: call 0x141613500
1412c53a8: movzx eax, byte ptr [rip + 0x236f85a]
1412c53af: mov ecx, 0x28
1412c53b4: mov edx, 8
1412c53b9: call 0x140613c10
1412c53be: test rax, rax
1412c53c1: je 0x1412c5477
1412c53c7: mov rcx, qword ptr [rbp - 0x38]
1412c53cb: mov qword ptr [rax + 0x20], rcx
1412c53cf: movups xmm0, xmmword ptr [rbp - 0x58]
1412c53d3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c53d7: movups xmmword ptr [rax + 0x10], xmm1
1412c53db: movups xmmword ptr [rax], xmm0
1412c53de: lea rcx, [rip + 0x17d84cb]
1412c53e5: mov qword ptr [rbp], rcx
1412c53e9: mov qword ptr [rbp + 8], 0x49
1412c53f1: mov qword ptr [rbp + 0x10], rax
1412c53f5: lea rax, [rip + 0x1967e34]
1412c53fc: mov qword ptr [rbp + 0x18], rax
1412c5400: mov qword ptr [rbp + 0x30], 0
1412c5408: mov byte ptr [rbp + 0x46], 5
1412c540c: mov qword ptr [rbp + 0x20], rcx
1412c5410: mov qword ptr [rbp + 0x28], 0x49
1412c5418: movzx eax, byte ptr [rip + 0x236f7ea]
1412c541f: mov ecx, 0x40
1412c5424: mov edx, 8
1412c5429: call 0x140613c10
1412c542e: test rax, rax
1412c5431: je 0x1412c5488
1412c5433: movups xmm0, xmmword ptr [rbp + 0x10]
1412c5437: movups xmm1, xmmword ptr [rbp + 0x20]
1412c543b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c543f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c5443: movups xmmword ptr [rax + 0x30], xmm3
1412c5447: movups xmmword ptr [rax + 0x20], xmm2
1412c544b: movups xmmword ptr [rax + 0x10], xmm1
1412c544f: movups xmmword ptr [rax], xmm0
1412c5452: movabs rdx, 0x8d7e0d9b9dc116db
1412c545c: mov rcx, rsi
1412c545f: mov r8, rax
1412c5462: call 0x140969bf0
1412c5467: cmp rax, 1
1412c546b: je 0x1412c5499
1412c546d: add rsp, 0xd8
1412c5474: pop rsi
1412c5475: pop rbp
1412c5476: ret 
1412c5477: mov ecx, 8
1412c547c: mov edx, 0x28
1412c5481: call 0x1428d9000
1412c5486: jmp 0x1412c5497
1412c5488: mov ecx, 8
1412c548d: mov edx, 0x40
1412c5492: call 0x1428d9000
1412c5497: ud2 
1412c5499: mov rax, rdx
1412c549c: add rdx, 0x10
1412c54a0: mov rsi, rdx
1412c54a3: cmp qword ptr [rax + 0x18], 0x49
1412c54a8: jne 0x1412c5526
1412c54aa: mov rdx, qword ptr [rsi]
1412c54ad: lea rcx, [rip + 0x17d83fc]
1412c54b4: mov r8d, 0x49
1412c54ba: call 0x142923bf0
1412c54bf: test eax, eax
1412c54c1: jne 0x1412c5526
1412c54c3: lea rax, [rip + 0x196845e]
1412c54ca: mov qword ptr [rbp - 0x30], rax
1412c54ce: lea rax, [rip - 0xd91c35]
1412c54d5: mov qword ptr [rbp - 0x28], rax
1412c54d9: mov rax, rbp
1412c54dc: mov qword ptr [rbp - 0x20], rax
1412c54e0: lea rax, [rip - 0x11bb657]
1412c54e7: mov qword ptr [rbp - 0x18], rax
1412c54eb: lea rax, [rip + 0x1968136]
1412c54f2: mov qword ptr [rbp + 0x10], rax
1412c54f6: mov qword ptr [rbp + 0x18], 3
1412c54fe: mov qword ptr [rbp + 0x30], 0
1412c5506: lea rax, [rbp - 0x30]
1412c550a: mov qword ptr [rbp + 0x20], rax
1412c550e: mov qword ptr [rbp + 0x28], 2
1412c5516: lea rdx, [rip + 0x196813b]
1412c551d: lea rcx, [rbp + 0x10]
1412c5521: call 0x1428d9390
1412c5526: lea rax, [rip + 0x19683fb]
1412c552d: mov qword ptr [rbp + 0x10], rax
1412c5531: lea rax, [rip - 0xd91c98]
1412c5538: mov qword ptr [rbp + 0x18], rax
1412c553c: mov qword ptr [rbp + 0x20], rsi
1412c5540: lea rax, [rip - 0x11bb6b7]
1412c5547: mov qword ptr [rbp + 0x28], rax
1412c554b: mov rcx, rbp
1412c554e: mov qword ptr [rbp + 0x30], rcx
1412c5552: mov qword ptr [rbp + 0x38], rax
1412c5556: lea rax, [rip + 0x196800b]
1412c555d: mov qword ptr [rbp - 0x30], rax
1412c5561: mov qword ptr [rbp - 0x28], 3
1412c5569: mov qword ptr [rbp - 0x10], 0
1412c5571: lea rax, [rbp + 0x10]
1412c5575: mov qword ptr [rbp - 0x20], rax
1412c5579: mov qword ptr [rbp - 0x18], 3
1412c5581: lea rdx, [rip + 0x1968048]
1412c5588: lea rcx, [rbp - 0x30]
1412c558c: call 0x1428d9390
1412c5591: int3 
1412c5592: nop word ptr cs:[rax + rax]
1412c55a0: mov qword ptr [rsp + 0x10], rdx
1412c55a5: push rbp
1412c55a6: push rsi
1412c55a7: sub rsp, 0x28
1412c55ab: lea rbp, [rdx + 0x80]
1412c55b2: lea rcx, [rbp - 0x58]
1412c55b6: call 0x140be24b0
1412c55bb: nop 
1412c55bc: add rsp, 0x28
1412c55c0: pop rsi
1412c55c1: pop rbp
1412c55c2: ret 
1412c55c3: nop word ptr cs:[rax + rax]
1412c55d0: mov qword ptr [rsp + 0x10], rdx
1412c55d5: push rbp
1412c55d6: push rsi
1412c55d7: sub rsp, 0x28
1412c55db: lea rbp, [rdx + 0x80]
1412c55e2: lea rcx, [rbp + 0x10]
1412c55e6: call 0x1400c04b0
1412c55eb: nop 
1412c55ec: add rsp, 0x28
1412c55f0: pop rsi
1412c55f1: pop rbp
1412c55f2: ret 
