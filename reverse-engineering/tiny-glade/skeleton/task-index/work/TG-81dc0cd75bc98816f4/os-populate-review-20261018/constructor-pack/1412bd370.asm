1412bd370: push rbp
1412bd371: push rsi
1412bd372: sub rsp, 0xd8
1412bd379: lea rbp, [rsp + 0x80]
1412bd381: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bd389: mov rsi, rcx
1412bd38c: lea rcx, [rbp - 0x58]
1412bd390: xor r8d, r8d
1412bd393: call 0x141611380
1412bd398: movzx eax, byte ptr [rip + 0x237786a]
1412bd39f: mov ecx, 0x28
1412bd3a4: mov edx, 8
1412bd3a9: call 0x140613c10
1412bd3ae: test rax, rax
1412bd3b1: je 0x1412bd467
1412bd3b7: mov rcx, qword ptr [rbp - 0x38]
1412bd3bb: mov qword ptr [rax + 0x20], rcx
1412bd3bf: movups xmm0, xmmword ptr [rbp - 0x58]
1412bd3c3: movups xmm1, xmmword ptr [rbp - 0x48]
1412bd3c7: movups xmmword ptr [rax + 0x10], xmm1
1412bd3cb: movups xmmword ptr [rax], xmm0
1412bd3ce: lea rcx, [rip + 0x17e05a3]
1412bd3d5: mov qword ptr [rbp], rcx
1412bd3d9: mov qword ptr [rbp + 8], 0x53
1412bd3e1: mov qword ptr [rbp + 0x10], rax
1412bd3e5: lea rax, [rip + 0x196f31c]
1412bd3ec: mov qword ptr [rbp + 0x18], rax
1412bd3f0: mov qword ptr [rbp + 0x30], 0
1412bd3f8: mov byte ptr [rbp + 0x46], 5
1412bd3fc: mov qword ptr [rbp + 0x20], rcx
1412bd400: mov qword ptr [rbp + 0x28], 0x53
1412bd408: movzx eax, byte ptr [rip + 0x23777fa]
1412bd40f: mov ecx, 0x40
1412bd414: mov edx, 8
1412bd419: call 0x140613c10
1412bd41e: test rax, rax
1412bd421: je 0x1412bd478
1412bd423: movups xmm0, xmmword ptr [rbp + 0x10]
1412bd427: movups xmm1, xmmword ptr [rbp + 0x20]
1412bd42b: movups xmm2, xmmword ptr [rbp + 0x30]
1412bd42f: movups xmm3, xmmword ptr [rbp + 0x40]
1412bd433: movups xmmword ptr [rax + 0x30], xmm3
1412bd437: movups xmmword ptr [rax + 0x20], xmm2
1412bd43b: movups xmmword ptr [rax + 0x10], xmm1
1412bd43f: movups xmmword ptr [rax], xmm0
1412bd442: movabs rdx, 0xb5918c6e8c25a37
1412bd44c: mov rcx, rsi
1412bd44f: mov r8, rax
1412bd452: call 0x140969bf0
1412bd457: cmp rax, 1
1412bd45b: je 0x1412bd489
1412bd45d: add rsp, 0xd8
1412bd464: pop rsi
1412bd465: pop rbp
1412bd466: ret 
1412bd467: mov ecx, 8
1412bd46c: mov edx, 0x28
1412bd471: call 0x1428d9000
1412bd476: jmp 0x1412bd487
1412bd478: mov ecx, 8
1412bd47d: mov edx, 0x40
1412bd482: call 0x1428d9000
1412bd487: ud2 
1412bd489: mov rax, rdx
1412bd48c: add rdx, 0x10
1412bd490: mov rsi, rdx
1412bd493: cmp qword ptr [rax + 0x18], 0x53
1412bd498: jne 0x1412bd516
1412bd49a: mov rdx, qword ptr [rsi]
1412bd49d: lea rcx, [rip + 0x17e04d4]
1412bd4a4: mov r8d, 0x53
1412bd4aa: call 0x142923bf0
1412bd4af: test eax, eax
1412bd4b1: jne 0x1412bd516
1412bd4b3: lea rax, [rip + 0x197078e]
1412bd4ba: mov qword ptr [rbp - 0x30], rax
1412bd4be: lea rax, [rip - 0xd89c25]
1412bd4c5: mov qword ptr [rbp - 0x28], rax
1412bd4c9: mov rax, rbp
1412bd4cc: mov qword ptr [rbp - 0x20], rax
1412bd4d0: lea rax, [rip - 0x11b3647]
1412bd4d7: mov qword ptr [rbp - 0x18], rax
1412bd4db: lea rax, [rip + 0x1970146]
1412bd4e2: mov qword ptr [rbp + 0x10], rax
1412bd4e6: mov qword ptr [rbp + 0x18], 3
1412bd4ee: mov qword ptr [rbp + 0x30], 0
1412bd4f6: lea rax, [rbp - 0x30]
1412bd4fa: mov qword ptr [rbp + 0x20], rax
1412bd4fe: mov qword ptr [rbp + 0x28], 2
1412bd506: lea rdx, [rip + 0x197014b]
1412bd50d: lea rcx, [rbp + 0x10]
1412bd511: call 0x1428d9390
1412bd516: lea rax, [rip + 0x197072b]
1412bd51d: mov qword ptr [rbp + 0x10], rax
1412bd521: lea rax, [rip - 0xd89c88]
1412bd528: mov qword ptr [rbp + 0x18], rax
1412bd52c: mov qword ptr [rbp + 0x20], rsi
1412bd530: lea rax, [rip - 0x11b36a7]
1412bd537: mov qword ptr [rbp + 0x28], rax
1412bd53b: mov rcx, rbp
1412bd53e: mov qword ptr [rbp + 0x30], rcx
1412bd542: mov qword ptr [rbp + 0x38], rax
1412bd546: lea rax, [rip + 0x197001b]
1412bd54d: mov qword ptr [rbp - 0x30], rax
1412bd551: mov qword ptr [rbp - 0x28], 3
1412bd559: mov qword ptr [rbp - 0x10], 0
1412bd561: lea rax, [rbp + 0x10]
1412bd565: mov qword ptr [rbp - 0x20], rax
1412bd569: mov qword ptr [rbp - 0x18], 3
1412bd571: lea rdx, [rip + 0x1970058]
1412bd578: lea rcx, [rbp - 0x30]
1412bd57c: call 0x1428d9390
1412bd581: int3 
1412bd582: nop word ptr cs:[rax + rax]
1412bd590: mov qword ptr [rsp + 0x10], rdx
1412bd595: push rbp
1412bd596: push rsi
1412bd597: sub rsp, 0x28
1412bd59b: lea rbp, [rdx + 0x80]
1412bd5a2: lea rcx, [rbp - 0x58]
1412bd5a6: call 0x140be24b0
1412bd5ab: nop 
1412bd5ac: add rsp, 0x28
1412bd5b0: pop rsi
1412bd5b1: pop rbp
1412bd5b2: ret 
1412bd5b3: nop word ptr cs:[rax + rax]
1412bd5c0: mov qword ptr [rsp + 0x10], rdx
1412bd5c5: push rbp
1412bd5c6: push rsi
1412bd5c7: sub rsp, 0x28
1412bd5cb: lea rbp, [rdx + 0x80]
1412bd5d2: lea rcx, [rbp + 0x10]
1412bd5d6: call 0x1400c04b0
1412bd5db: nop 
1412bd5dc: add rsp, 0x28
1412bd5e0: pop rsi
1412bd5e1: pop rbp
1412bd5e2: ret 
