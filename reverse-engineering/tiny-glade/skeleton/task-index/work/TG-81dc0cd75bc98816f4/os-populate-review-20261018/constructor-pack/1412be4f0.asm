1412be4f0: push rbp
1412be4f1: push rsi
1412be4f2: sub rsp, 0xd8
1412be4f9: lea rbp, [rsp + 0x80]
1412be501: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412be509: mov rsi, rcx
1412be50c: lea rcx, [rbp - 0x58]
1412be510: xor r8d, r8d
1412be513: call 0x14160fb70
1412be518: movzx eax, byte ptr [rip + 0x23766ea]
1412be51f: mov ecx, 0x28
1412be524: mov edx, 8
1412be529: call 0x140613c10
1412be52e: test rax, rax
1412be531: je 0x1412be5e7
1412be537: mov rcx, qword ptr [rbp - 0x38]
1412be53b: mov qword ptr [rax + 0x20], rcx
1412be53f: movups xmm0, xmmword ptr [rbp - 0x58]
1412be543: movups xmm1, xmmword ptr [rbp - 0x48]
1412be547: movups xmmword ptr [rax + 0x10], xmm1
1412be54b: movups xmmword ptr [rax], xmm0
1412be54e: lea rcx, [rip + 0x17dfe2c]
1412be555: mov qword ptr [rbp], rcx
1412be559: mov qword ptr [rbp + 8], 0x4c
1412be561: mov qword ptr [rbp + 0x10], rax
1412be565: lea rax, [rip + 0x196e324]
1412be56c: mov qword ptr [rbp + 0x18], rax
1412be570: mov qword ptr [rbp + 0x30], 0
1412be578: mov byte ptr [rbp + 0x46], 5
1412be57c: mov qword ptr [rbp + 0x20], rcx
1412be580: mov qword ptr [rbp + 0x28], 0x4c
1412be588: movzx eax, byte ptr [rip + 0x237667a]
1412be58f: mov ecx, 0x40
1412be594: mov edx, 8
1412be599: call 0x140613c10
1412be59e: test rax, rax
1412be5a1: je 0x1412be5f8
1412be5a3: movups xmm0, xmmword ptr [rbp + 0x10]
1412be5a7: movups xmm1, xmmword ptr [rbp + 0x20]
1412be5ab: movups xmm2, xmmword ptr [rbp + 0x30]
1412be5af: movups xmm3, xmmword ptr [rbp + 0x40]
1412be5b3: movups xmmword ptr [rax + 0x30], xmm3
1412be5b7: movups xmmword ptr [rax + 0x20], xmm2
1412be5bb: movups xmmword ptr [rax + 0x10], xmm1
1412be5bf: movups xmmword ptr [rax], xmm0
1412be5c2: movabs rdx, 0x69f8bc21227db88
1412be5cc: mov rcx, rsi
1412be5cf: mov r8, rax
1412be5d2: call 0x140969bf0
1412be5d7: cmp rax, 1
1412be5db: je 0x1412be609
1412be5dd: add rsp, 0xd8
1412be5e4: pop rsi
1412be5e5: pop rbp
1412be5e6: ret 
1412be5e7: mov ecx, 8
1412be5ec: mov edx, 0x28
1412be5f1: call 0x1428d9000
1412be5f6: jmp 0x1412be607
1412be5f8: mov ecx, 8
1412be5fd: mov edx, 0x40
1412be602: call 0x1428d9000
1412be607: ud2 
1412be609: mov rax, rdx
1412be60c: add rdx, 0x10
1412be610: mov rsi, rdx
1412be613: cmp qword ptr [rax + 0x18], 0x4c
1412be618: jne 0x1412be696
1412be61a: mov rdx, qword ptr [rsi]
1412be61d: lea rcx, [rip + 0x17dfd5d]
1412be624: mov r8d, 0x4c
1412be62a: call 0x142923bf0
1412be62f: test eax, eax
1412be631: jne 0x1412be696
1412be633: lea rax, [rip + 0x196fa7e]
1412be63a: mov qword ptr [rbp - 0x30], rax
1412be63e: lea rax, [rip - 0xd8ada5]
1412be645: mov qword ptr [rbp - 0x28], rax
1412be649: mov rax, rbp
1412be64c: mov qword ptr [rbp - 0x20], rax
1412be650: lea rax, [rip - 0x11b47c7]
1412be657: mov qword ptr [rbp - 0x18], rax
1412be65b: lea rax, [rip + 0x196efc6]
1412be662: mov qword ptr [rbp + 0x10], rax
1412be666: mov qword ptr [rbp + 0x18], 3
1412be66e: mov qword ptr [rbp + 0x30], 0
1412be676: lea rax, [rbp - 0x30]
1412be67a: mov qword ptr [rbp + 0x20], rax
1412be67e: mov qword ptr [rbp + 0x28], 2
1412be686: lea rdx, [rip + 0x196efcb]
1412be68d: lea rcx, [rbp + 0x10]
1412be691: call 0x1428d9390
1412be696: lea rax, [rip + 0x196fa1b]
1412be69d: mov qword ptr [rbp + 0x10], rax
1412be6a1: lea rax, [rip - 0xd8ae08]
1412be6a8: mov qword ptr [rbp + 0x18], rax
1412be6ac: mov qword ptr [rbp + 0x20], rsi
1412be6b0: lea rax, [rip - 0x11b4827]
1412be6b7: mov qword ptr [rbp + 0x28], rax
1412be6bb: mov rcx, rbp
1412be6be: mov qword ptr [rbp + 0x30], rcx
1412be6c2: mov qword ptr [rbp + 0x38], rax
1412be6c6: lea rax, [rip + 0x196ee9b]
1412be6cd: mov qword ptr [rbp - 0x30], rax
1412be6d1: mov qword ptr [rbp - 0x28], 3
1412be6d9: mov qword ptr [rbp - 0x10], 0
1412be6e1: lea rax, [rbp + 0x10]
1412be6e5: mov qword ptr [rbp - 0x20], rax
1412be6e9: mov qword ptr [rbp - 0x18], 3
1412be6f1: lea rdx, [rip + 0x196eed8]
1412be6f8: lea rcx, [rbp - 0x30]
1412be6fc: call 0x1428d9390
1412be701: int3 
1412be702: nop word ptr cs:[rax + rax]
1412be710: mov qword ptr [rsp + 0x10], rdx
1412be715: push rbp
1412be716: push rsi
1412be717: sub rsp, 0x28
1412be71b: lea rbp, [rdx + 0x80]
1412be722: lea rcx, [rbp - 0x58]
1412be726: call 0x140be24b0
1412be72b: nop 
1412be72c: add rsp, 0x28
1412be730: pop rsi
1412be731: pop rbp
1412be732: ret 
1412be733: nop word ptr cs:[rax + rax]
1412be740: mov qword ptr [rsp + 0x10], rdx
1412be745: push rbp
1412be746: push rsi
1412be747: sub rsp, 0x28
1412be74b: lea rbp, [rdx + 0x80]
1412be752: lea rcx, [rbp + 0x10]
1412be756: call 0x1400c04b0
1412be75b: nop 
1412be75c: add rsp, 0x28
1412be760: pop rsi
1412be761: pop rbp
1412be762: ret 
