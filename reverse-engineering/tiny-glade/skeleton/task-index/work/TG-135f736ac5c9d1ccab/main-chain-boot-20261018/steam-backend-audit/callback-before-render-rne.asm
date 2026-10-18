1404de447: movdqa xmmword ptr [rbp + 0x600], xmm0
1404de44f: lea rcx, [rbp + 0xc30]
1404de456: mov rdx, qword ptr [rbp + 0x1418]
1404de45d: lea r8, [rbp + 0x600]
1404de464: call 0x1403c5a10
1404de469: mov rax, qword ptr [rbp + 0xc30]
1404de470: movabs rcx, 0x8000000000000007
1404de47a: cmp rax, rcx
1404de47d: je 0x1404de4e0
1404de47f: movabs rcx, 0x8000000000000000
1404de489: xor rax, rcx
1404de48c: cmp rax, 7
1404de490: mov ecx, 3
1404de495: cmovae rax, rcx
1404de499: cmp rax, 4
1404de49d: je 0x1404de4b3
1404de49f: cmp rax, 3
1404de4a3: jne 0x1404de4e0
1404de4a5: lea rcx, [rbp + 0xc30]
1404de4ac: call 0x1405f0d10
1404de4b1: jmp 0x1404de4e0
1404de4b3: mov rdx, qword ptr [rbp + 0xc38]
1404de4ba: movabs rax, 0x8000000000000002
1404de4c4: cmp rdx, rax
1404de4c7: jl 0x1404de4e0
1404de4c9: test rdx, rdx
1404de4cc: je 0x1404de4e0
1404de4ce: mov rcx, qword ptr [rbp + 0xc40]
1404de4d5: mov r8d, 1
1404de4db: call 0x140613c20
1404de4e0: mov rax, qword ptr [rbp + 0x1388]
1404de4e7: cmp qword ptr [rax], 0
1404de4eb: je 0x1404de4f9
1404de4ed: mov rcx, qword ptr [rbp + 0x1388]
1404de4f4: call 0x1400e6f90
1404de4f9: mov rcx, qword ptr [rbp + 0x1488]
1404de500: movabs rdx, 0xbba451ee6a08a3ff
1404de50a: movabs r8, 0x8d008d15febcf29d
1404de514: call 0x1400dd6f0
1404de519: test rax, rax
1404de51c: je 0x1404de560
1404de51e: cmp qword ptr [r15 + 0x248], rdx
1404de525: jbe 0x1404de560
1404de527: mov rax, qword ptr [r15 + 0x240]
1404de52e: mov rax, qword ptr [rax + rdx*8]
1404de532: test rax, rax
1404de535: je 0x1404de560
1404de537: mov rcx, qword ptr [r15 + 0x210]
1404de53e: not rax
1404de541: lea rax, [rax + rax*2]
1404de545: shl rax, 5
1404de549: cmp qword ptr [rcx + rax + 0x20], 0
1404de54f: setne al
1404de552: add al, al
1404de554: jmp 0x1404de562
1404de556: nop word ptr cs:[rax + rax]
1404de560: xor eax, eax
1404de562: imul rcx, r14, 0x3b9aca00
1404de569: mov edx, dword ptr [rbp + 0x1468]
1404de56f: add rsi, rdx
1404de572: add rsi, rcx
1404de575: movd edx, xmm6
1404de579: shl rdx, 0x20
1404de57d: mov rcx, qword ptr [rbp + 0x1460]
1404de584: or rcx, rdx
1404de587: mov byte ptr [rsp + 0x38], al
1404de58b: mov rax, qword ptr [rbp + 0x1390]
1404de592: mov qword ptr [rsp + 0x30], rax
1404de597: mov rax, qword ptr [rbp + 0x1420]
1404de59e: mov qword ptr [rsp + 0x28], rax
1404de5a3: mov rax, qword ptr [rbp + 0x1338]
1404de5aa: mov qword ptr [rsp + 0x20], rax
1404de5af: mov rdx, r13
1404de5b2: mov r8, rsi
1404de5b5: mov r9, qword ptr [rbp + 0x1330]
1404de5bc: call 0x1404e8010
1404de5c1: mov rcx, qword ptr [rbp + 0x1488]
