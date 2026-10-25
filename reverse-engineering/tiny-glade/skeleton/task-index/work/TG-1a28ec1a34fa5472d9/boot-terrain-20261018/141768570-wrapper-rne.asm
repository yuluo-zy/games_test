141768570: push r15
141768572: push r14
141768574: push r13
141768576: push r12
141768578: push rsi
141768579: push rdi
14176857a: push rbp
14176857b: push rbx
14176857c: sub rsp, 0x148
141768583: mov edi, 1
141768588: lock xadd dword ptr [rdx + 0x5b0], edi
141768590: xor eax, eax
141768592: cmp rax, qword ptr [rcx + 0x168]
141768599: jo 0x1417689cb
14176859f: mov rsi, rcx
1417685a2: mov rax, qword ptr [rcx + 0x138]
1417685a9: mov rbx, qword ptr [rdx + 0x218]
1417685b0: cmp rbx, rax
1417685b3: jbe 0x1417688b5
1417685b9: mov r14, qword ptr [rdx + 0x210]
1417685c0: mov rax, qword ptr [r14 + rax*8]
1417685c4: test rax, rax
1417685c7: je 0x1417688b5
1417685cd: mov r8, qword ptr [rdx + 0x1e0]
1417685d4: not rax
1417685d7: lea rax, [rax + rax*2]
1417685db: shl rax, 5
1417685df: cmp qword ptr [r8 + rax + 0x20], 0
1417685e5: je 0x1417688b5
1417685eb: mov rcx, qword ptr [rsi + 0x140]
1417685f2: cmp rbx, rcx
1417685f5: jbe 0x1417688d6
1417685fb: mov rcx, qword ptr [r14 + rcx*8]
1417685ff: test rcx, rcx
141768602: je 0x1417688d6
141768608: not rcx
14176860b: lea rcx, [rcx + rcx*2]
14176860f: shl rcx, 5
141768613: cmp qword ptr [r8 + rcx + 0x20], 0
141768619: je 0x1417688d6
14176861f: mov r9, qword ptr [rsi + 0x148]
141768626: cmp rbx, r9
141768629: jbe 0x1417688e6
14176862f: mov r9, qword ptr [r14 + r9*8]
141768633: test r9, r9
141768636: je 0x1417688e6
14176863c: not r9
14176863f: lea r9, [r9 + r9*2]
141768643: shl r9, 5
141768647: cmp qword ptr [r8 + r9 + 0x20], 0
14176864d: je 0x1417688e6
141768653: mov r10, qword ptr [rsi + 0x150]
14176865a: cmp rbx, r10
14176865d: jbe 0x141768904
141768663: mov r10, qword ptr [r14 + r10*8]
141768667: test r10, r10
14176866a: je 0x141768904
141768670: not r10
141768673: lea r10, [r10 + r10*2]
141768677: shl r10, 5
14176867b: cmp qword ptr [r8 + r10 + 0x20], 0
141768681: je 0x141768904
141768687: mov r11, qword ptr [rsi + 0x158]
14176868e: cmp rbx, r11
141768691: jbe 0x141768922
141768697: mov r11, qword ptr [r14 + r11*8]
14176869b: test r11, r11
14176869e: je 0x141768922
1417686a4: not r11
1417686a7: lea r11, [r11 + r11*2]
1417686ab: shl r11, 5
1417686af: cmp qword ptr [r8 + r11 + 0x20], 0
1417686b5: je 0x141768922
1417686bb: mov r15, qword ptr [rsi + 0x160]
1417686c2: cmp rbx, r15
1417686c5: jbe 0x141768940
1417686cb: mov rbx, qword ptr [r14 + r15*8]
1417686cf: test rbx, rbx
1417686d2: je 0x141768940
1417686d8: not rbx
1417686db: lea rbx, [rbx + rbx*2]
1417686df: shl rbx, 5
1417686e3: cmp qword ptr [r8 + rbx + 0x20], 0
1417686e9: je 0x141768940
1417686ef: add rax, r8
1417686f2: mov rbp, qword ptr [rax + 0x10]
1417686f6: add rcx, r8
1417686f9: mov r13, qword ptr [rcx + 0x10]
1417686fd: add r9, r8
141768700: mov r12, qword ptr [r9 + 0x10]
141768704: add r10, r8
141768707: mov r15, qword ptr [r10 + 0x10]
14176870b: add r11, r8
14176870e: mov r14, qword ptr [r11 + 0x10]
141768712: add rbx, r8
141768715: mov r8, qword ptr [rbx + 0x10]
141768719: mov qword ptr [rsp + 0x68], rbp
14176871e: lea rbp, [rax + 0x5c]
141768722: add rax, 0x58
141768726: mov qword ptr [rsp + 0x70], rax
14176872b: mov qword ptr [rsp + 0x78], rbp
141768730: mov ebp, dword ptr [rsi + 0x130]
141768736: mov dword ptr [rsp + 0x80], ebp
14176873d: mov dword ptr [rsp + 0x84], edi
141768744: mov qword ptr [rsp + 0x88], r13
14176874c: lea rax, [rcx + 0x5c]
141768750: add rcx, 0x58
141768754: mov qword ptr [rsp + 0x90], rcx
14176875c: mov qword ptr [rsp + 0x98], rax
141768764: mov dword ptr [rsp + 0xa0], ebp
14176876b: mov dword ptr [rsp + 0xa4], edi
141768772: mov qword ptr [rsp + 0xa8], r12
14176877a: lea rax, [r9 + 0x5c]
14176877e: add r9, 0x58
141768782: mov qword ptr [rsp + 0xb0], r9
14176878a: mov qword ptr [rsp + 0xb8], rax
141768792: mov dword ptr [rsp + 0xc0], ebp
141768799: mov dword ptr [rsp + 0xc4], edi
1417687a0: mov qword ptr [rsp + 0xc8], r15
1417687a8: lea rax, [r10 + 0x5c]
1417687ac: add r10, 0x58
1417687b0: mov qword ptr [rsp + 0xd0], r10
1417687b8: mov qword ptr [rsp + 0xd8], rax
1417687c0: mov dword ptr [rsp + 0xe0], ebp
1417687c7: mov dword ptr [rsp + 0xe4], edi
1417687ce: mov qword ptr [rsp + 0xe8], r14
1417687d6: lea rax, [r11 + 0x5c]
1417687da: add r11, 0x58
1417687de: mov qword ptr [rsp + 0xf0], r11
1417687e6: lea rcx, [rsi + 0x168]
1417687ed: mov qword ptr [rsp + 0xf8], rax
1417687f5: lea r9, [rbx + 0x58]
1417687f9: add rbx, 0x5c
1417687fd: mov dword ptr [rsp + 0x100], ebp
141768804: mov dword ptr [rsp + 0x104], edi
14176880b: mov qword ptr [rsp + 0x108], r8
141768813: lea rax, [rsp + 0x88]
14176881b: mov qword ptr [rsp + 0x110], r9
141768823: lea r8, [rsp + 0xa8]
14176882b: mov qword ptr [rsp + 0x118], rbx
141768833: lea r9, [rsp + 0xc8]
14176883b: mov dword ptr [rsp + 0x120], ebp
141768842: lea r10, [rsp + 0xe8]
14176884a: mov dword ptr [rsp + 0x124], edi
141768851: mov qword ptr [rsp + 0x128], rdx
141768859: lea rdx, [rsp + 0x108]
141768861: mov qword ptr [rsp + 0x130], 0
14176886d: mov qword ptr [rsp + 0x138], rcx
141768875: lea rcx, [rsp + 0x128]
14176887d: mov qword ptr [rsp + 0x30], rcx
141768882: mov qword ptr [rsp + 0x28], rdx
141768887: mov qword ptr [rsp + 0x20], r10
14176888c: lea rcx, [rsp + 0x68]
141768891: mov rdx, rax
141768894: call 0x140926bf0
141768899: mov dword ptr [rsi + 0x130], edi
14176889f: xor eax, eax
1417688a1: add rsp, 0x148
1417688a8: pop rbx
1417688a9: pop rbp
1417688aa: pop rdi
1417688ab: pop rsi
1417688ac: pop r12
1417688ae: pop r13
1417688b0: pop r14
1417688b2: pop r15
1417688b4: ret 
1417688b5: add rsi, 0x98
1417688bc: lea rax, [rip + 0x156ea32]
1417688c3: mov qword ptr [rsp + 0x38], rax
1417688c8: mov qword ptr [rsp + 0x40], 0x2c
1417688d1: jmp 0x14176895c
1417688d6: add rsi, 0x98
1417688dd: lea rax, [rip + 0x156e9d4]
1417688e4: jmp 0x141768930
1417688e6: add rsi, 0x98
1417688ed: lea rax, [rip + 0x156ead3]
1417688f4: mov qword ptr [rsp + 0x38], rax
1417688f9: mov qword ptr [rsp + 0x40], 0x33
141768902: jmp 0x14176895c
141768904: add rsi, 0x98
14176890b: lea rax, [rip + 0x156ebf0]
141768912: mov qword ptr [rsp + 0x38], rax
141768917: mov qword ptr [rsp + 0x40], 0x41
141768920: jmp 0x14176895c
141768922: add rsi, 0x98
141768929: lea rax, [rip + 0x156eb2a]
141768930: mov qword ptr [rsp + 0x38], rax
141768935: mov qword ptr [rsp + 0x40], 0x3d
14176893e: jmp 0x14176895c
141768940: add rsi, 0x98
141768947: lea rax, [rip + 0x156ea44]
14176894e: mov qword ptr [rsp + 0x38], rax
141768953: mov qword ptr [rsp + 0x40], 0x35
14176895c: mov qword ptr [rsp + 0x48], rsi
141768961: lea rax, [rip - 0x172ce98]
141768968: mov qword ptr [rsp + 0x50], rax
14176896d: lea rax, [rsp + 0x38]
141768972: mov qword ptr [rsp + 0x58], rax
141768977: lea rax, [rip - 0x165eaee]
14176897e: mov qword ptr [rsp + 0x60], rax
141768983: lea rax, [rip + 0x156e85e]
14176898a: mov qword ptr [rsp + 0x68], rax
14176898f: mov qword ptr [rsp + 0x70], 2
141768998: mov qword ptr [rsp + 0x88], 0
1417689a4: lea rax, [rsp + 0x48]
1417689a9: mov qword ptr [rsp + 0x78], rax
1417689ae: mov qword ptr [rsp + 0x80], 2
1417689ba: lea rdx, [rip + 0x156e8af]
1417689c1: lea rcx, [rsp + 0x68]
1417689c6: call 0x1428d9390
1417689cb: lea rcx, [rip + 0x156f09e]
1417689d2: lea r8, [rip + 0x156f0f7]
1417689d9: mov edx, 0x59
1417689de: call 0x1428d9330
1417689e3: int3 
