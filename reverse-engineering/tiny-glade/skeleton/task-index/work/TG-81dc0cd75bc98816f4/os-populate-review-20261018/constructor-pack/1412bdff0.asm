1412bdff0: push rbp
1412bdff1: push rsi
1412bdff2: sub rsp, 0xd8
1412bdff9: lea rbp, [rsp + 0x80]
1412be001: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412be009: mov rsi, rcx
1412be00c: lea rcx, [rbp - 0x60]
1412be010: xor r8d, r8d
1412be013: call 0x1416207e0
1412be018: movzx eax, byte ptr [rip + 0x2376bea]
1412be01f: mov ecx, 0x30
1412be024: mov edx, 8
1412be029: call 0x140613c10
1412be02e: test rax, rax
1412be031: je 0x1412be0e7
1412be037: movups xmm0, xmmword ptr [rbp - 0x60]
1412be03b: movups xmm1, xmmword ptr [rbp - 0x50]
1412be03f: movups xmm2, xmmword ptr [rbp - 0x40]
1412be043: movups xmmword ptr [rax + 0x20], xmm2
1412be047: movups xmmword ptr [rax + 0x10], xmm1
1412be04b: movups xmmword ptr [rax], xmm0
1412be04e: lea rcx, [rip + 0x17def95]
1412be055: mov qword ptr [rbp], rcx
1412be059: mov qword ptr [rbp + 8], 0x3f
1412be061: mov qword ptr [rbp + 0x10], rax
1412be065: lea rax, [rip + 0x196e7b4]
1412be06c: mov qword ptr [rbp + 0x18], rax
1412be070: mov qword ptr [rbp + 0x30], 0
1412be078: mov byte ptr [rbp + 0x46], 5
1412be07c: mov qword ptr [rbp + 0x20], rcx
1412be080: mov qword ptr [rbp + 0x28], 0x3f
1412be088: movzx eax, byte ptr [rip + 0x2376b7a]
1412be08f: mov ecx, 0x40
1412be094: mov edx, 8
1412be099: call 0x140613c10
1412be09e: test rax, rax
1412be0a1: je 0x1412be0f8
1412be0a3: movups xmm0, xmmword ptr [rbp + 0x10]
1412be0a7: movups xmm1, xmmword ptr [rbp + 0x20]
1412be0ab: movups xmm2, xmmword ptr [rbp + 0x30]
1412be0af: movups xmm3, xmmword ptr [rbp + 0x40]
1412be0b3: movups xmmword ptr [rax + 0x30], xmm3
1412be0b7: movups xmmword ptr [rax + 0x20], xmm2
1412be0bb: movups xmmword ptr [rax + 0x10], xmm1
1412be0bf: movups xmmword ptr [rax], xmm0
1412be0c2: movabs rdx, 0x80ee06458ec5fb92
1412be0cc: mov rcx, rsi
1412be0cf: mov r8, rax
1412be0d2: call 0x140969bf0
1412be0d7: cmp rax, 1
1412be0db: je 0x1412be109
1412be0dd: add rsp, 0xd8
1412be0e4: pop rsi
1412be0e5: pop rbp
1412be0e6: ret 
1412be0e7: mov ecx, 8
1412be0ec: mov edx, 0x30
1412be0f1: call 0x1428d9000
1412be0f6: jmp 0x1412be107
1412be0f8: mov ecx, 8
1412be0fd: mov edx, 0x40
1412be102: call 0x1428d9000
1412be107: ud2 
1412be109: mov rax, rdx
1412be10c: add rdx, 0x10
1412be110: mov rsi, rdx
1412be113: cmp qword ptr [rax + 0x18], 0x3f
1412be118: jne 0x1412be196
1412be11a: mov rdx, qword ptr [rsi]
1412be11d: lea rcx, [rip + 0x17deec6]
1412be124: mov r8d, 0x3f
1412be12a: call 0x142923bf0
1412be12f: test eax, eax
1412be131: jne 0x1412be196
1412be133: lea rax, [rip + 0x196f90e]
1412be13a: mov qword ptr [rbp - 0x30], rax
1412be13e: lea rax, [rip - 0xd8a8a5]
1412be145: mov qword ptr [rbp - 0x28], rax
1412be149: mov rax, rbp
1412be14c: mov qword ptr [rbp - 0x20], rax
1412be150: lea rax, [rip - 0x11b42c7]
1412be157: mov qword ptr [rbp - 0x18], rax
1412be15b: lea rax, [rip + 0x196f4c6]
1412be162: mov qword ptr [rbp + 0x10], rax
1412be166: mov qword ptr [rbp + 0x18], 3
1412be16e: mov qword ptr [rbp + 0x30], 0
1412be176: lea rax, [rbp - 0x30]
1412be17a: mov qword ptr [rbp + 0x20], rax
1412be17e: mov qword ptr [rbp + 0x28], 2
1412be186: lea rdx, [rip + 0x196f4cb]
1412be18d: lea rcx, [rbp + 0x10]
1412be191: call 0x1428d9390
1412be196: lea rax, [rip + 0x196f8ab]
1412be19d: mov qword ptr [rbp + 0x10], rax
1412be1a1: lea rax, [rip - 0xd8a908]
1412be1a8: mov qword ptr [rbp + 0x18], rax
1412be1ac: mov qword ptr [rbp + 0x20], rsi
1412be1b0: lea rax, [rip - 0x11b4327]
1412be1b7: mov qword ptr [rbp + 0x28], rax
1412be1bb: mov rcx, rbp
1412be1be: mov qword ptr [rbp + 0x30], rcx
1412be1c2: mov qword ptr [rbp + 0x38], rax
1412be1c6: lea rax, [rip + 0x196f39b]
1412be1cd: mov qword ptr [rbp - 0x30], rax
1412be1d1: mov qword ptr [rbp - 0x28], 3
1412be1d9: mov qword ptr [rbp - 0x10], 0
1412be1e1: lea rax, [rbp + 0x10]
1412be1e5: mov qword ptr [rbp - 0x20], rax
1412be1e9: mov qword ptr [rbp - 0x18], 3
1412be1f1: lea rdx, [rip + 0x196f3d8]
1412be1f8: lea rcx, [rbp - 0x30]
1412be1fc: call 0x1428d9390
1412be201: int3 
1412be202: nop word ptr cs:[rax + rax]
1412be210: mov qword ptr [rsp + 0x10], rdx
1412be215: push rbp
1412be216: push rsi
1412be217: sub rsp, 0x28
1412be21b: lea rbp, [rdx + 0x80]
1412be222: lea rcx, [rbp - 0x58]
1412be226: call 0x140be24b0
1412be22b: nop 
1412be22c: add rsp, 0x28
1412be230: pop rsi
1412be231: pop rbp
1412be232: ret 
1412be233: nop word ptr cs:[rax + rax]
1412be240: mov qword ptr [rsp + 0x10], rdx
1412be245: push rbp
1412be246: push rsi
1412be247: sub rsp, 0x28
1412be24b: lea rbp, [rdx + 0x80]
1412be252: lea rcx, [rbp + 0x10]
1412be256: call 0x1400c04b0
1412be25b: nop 
1412be25c: add rsp, 0x28
1412be260: pop rsi
1412be261: pop rbp
1412be262: ret 
